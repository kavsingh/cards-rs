use super::{Hand, HandCandidate, HandRank};
use crate::util::{chunk_by, group_by, without};
use crate::{Card, Rank};

const MAX_KICKER_CARDS: usize = 5;

fn to_hand(rank: HandRank, rank_cards: Vec<Card>, sorted: &[Card]) -> Hand {
	let kicker_cards = without(&rank_cards, sorted)
		.into_iter()
		.take(MAX_KICKER_CARDS.saturating_sub(rank_cards.len()))
		.collect();

	Hand {
		rank,
		rank_cards,
		kicker_cards,
	}
}

type Extractor = fn(&[Card]) -> Option<Hand>;
const ORDERED_EXTRACTORS: [Extractor; 9] = [
	extract_royal_flush,
	extract_straight_flush,
	extract_four_of_a_kind,
	extract_full_house,
	extract_flush,
	extract_straight,
	extract_three_of_a_kind,
	extract_two_pair,
	extract_pair,
];

fn extract_royal_flush(sorted: &[Card]) -> Option<Hand> {
	extract_straight_flush(sorted).and_then(|hand| {
		match hand.rank_cards.first() {
			Some(highest) if highest.rank == Rank::Ace => Some(Hand {
				rank: HandRank::RoyalFlush,
				..hand
			}),
			_ => None,
		}
	})
}

fn extract_straight_flush(sorted: &[Card]) -> Option<Hand> {
	let suit_sorted = group_by(sorted, |c| c.suit)
		.into_iter()
		.find(|(_, cards)| cards.len() >= 5)
		.map(|(_, cards)| cards)?;

	extract_straight(&suit_sorted).map(|hand| Hand {
		rank: HandRank::StraightFlush,
		..hand
	})
}

fn extract_four_of_a_kind(sorted: &[Card]) -> Option<Hand> {
	group_by(sorted, |c| c.rank)
		.iter()
		.find(|(_, cards)| cards.len() == 4)
		.map(|(_, cards)| to_hand(HandRank::FourOfAKind, cards.clone(), sorted))
}

fn extract_full_house(sorted: &[Card]) -> Option<Hand> {
	let three_of_a_kind = extract_three_of_a_kind(sorted)?;
	let pair_cards = without(&three_of_a_kind.rank_cards, sorted);
	let pair = extract_pair(&pair_cards)?;

	Some(to_hand(
		HandRank::FullHouse,
		three_of_a_kind
			.rank_cards
			.into_iter()
			.chain(pair.rank_cards)
			.collect(),
		sorted,
	))
}

fn extract_flush(sorted: &[Card]) -> Option<Hand> {
	let flush = group_by(sorted, |c| c.suit)
		.into_iter()
		.find(|(_, cards)| cards.len() >= 5)
		.map(|(_, cards)| cards.into_iter().take(5).collect())?;

	Some(to_hand(HandRank::Flush, flush, sorted))
}

fn extract_straight(sorted: &[Card]) -> Option<Hand> {
	let candidate: Vec<_> = chunk_by(sorted, |a, b| a.rank_diff(b) == 1)
		.into_iter()
		.find(|chunk| chunk.len() >= 4)
		.map(|chunk| chunk.into_iter().take(5).collect())?;

	if candidate.len() == 5 {
		return Some(to_hand(HandRank::Straight, candidate, sorted));
	}

	// if we have a 4 card candidate, we need to check if we can use an ace as
	// the low card to complete the straight

	let ace = sorted.iter().find(|c| c.rank == Rank::Ace)?;
	let lowest = candidate.last()?;

	if lowest.rank == Rank::Two {
		let straight = candidate
			.into_iter()
			.chain(std::iter::once(*ace))
			.collect::<Vec<_>>();

		return Some(to_hand(HandRank::Straight, straight, sorted));
	}

	None
}

fn extract_three_of_a_kind(sorted: &[Card]) -> Option<Hand> {
	group_by(sorted, |c| c.rank)
		.iter()
		.find(|(_, cards)| cards.len() == 3)
		.map(|(_, cards)| {
			to_hand(HandRank::ThreeOfAKind, cards.clone(), sorted)
		})
}

fn extract_two_pair(sorted: &[Card]) -> Option<Hand> {
	let grouped = group_by(sorted, |c| c.rank);
	let pairs_only = grouped
		.iter()
		.filter(|(_, cards)| cards.len() >= 2)
		.collect::<Vec<_>>();

	let (_, first_cards) = pairs_only.first()?;
	let (_, second_cards) = pairs_only.get(1)?;

	Some(to_hand(
		HandRank::TwoPair,
		first_cards
			.iter()
			.copied()
			.chain(second_cards.iter().copied())
			.collect(),
		sorted,
	))
}

fn extract_pair(sorted: &[Card]) -> Option<Hand> {
	group_by(sorted, |c| c.rank)
		.iter()
		.find(|(_, cards)| cards.len() >= 2)
		.map(|(_, cards)| {
			to_hand(
				HandRank::Pair,
				cards.iter().copied().take(2).collect(),
				sorted,
			)
		})
}

impl From<HandCandidate<'_>> for Hand {
	fn from(value: HandCandidate) -> Self {
		let mut community = value.community_cards.to_owned();
		let mut sorted = value.pocket_cards.to_vec();

		sorted.append(&mut community);
		sorted.sort_by(|a, b| b.cmp(a));

		if let Some(hand) = ORDERED_EXTRACTORS
			.iter()
			.find_map(|extractor| extractor(&sorted))
		{
			return hand;
		}

		Self {
			rank: HandRank::HighCard,
			rank_cards: sorted.first().map(|c| vec![*c]).unwrap_or_default(),
			kicker_cards: sorted
				.get(1..5)
				.map(std::borrow::ToOwned::to_owned)
				.unwrap_or_default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Hand, HandCandidate, HandRank};
	use crate::{Card, Rank, Suit};

	#[test]
	fn should_extract_high_card() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Jack, Suit::Clubs),
					Card::new(Rank::Eight, Suit::Spades),
				],
				community_cards: &vec![
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Four, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::HighCard,
				rank_cards: vec![Card::new(Rank::Queen, Suit::Diamonds)],
				kicker_cards: vec![
					Card::new(Rank::Jack, Suit::Clubs),
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Four, Suit::Clubs),
				],
			}
		);
	}

	#[test]
	fn should_extract_pair() {
		let hand = Hand::from(HandCandidate {
			pocket_cards: &[
				Card::new(Rank::Six, Suit::Clubs),
				Card::new(Rank::Two, Suit::Diamonds),
			],
			community_cards: &vec![
				Card::new(Rank::Six, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Eight, Suit::Spades),
				Card::new(Rank::Four, Suit::Clubs),
				Card::new(Rank::Queen, Suit::Diamonds),
			],
		});

		assert_eq!(hand.rank, HandRank::Pair);
		assert_eq!(
			hand.rank_cards,
			vec![
				Card::new(Rank::Six, Suit::Clubs),
				Card::new(Rank::Six, Suit::Diamonds)
			]
		);
		assert_eq!(
			hand.kicker_cards,
			vec![
				Card::new(Rank::Queen, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Eight, Suit::Spades),
			]
		);
	}

	#[test]
	fn should_extract_two_pair() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Four, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
				community_cards: &vec![
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Diamonds),
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Six, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Clubs),
				],
			}),
			Hand {
				rank: HandRank::TwoPair,
				rank_cards: vec![
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Clubs),
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Six, Suit::Clubs),
				],
				kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
			}
		);
	}

	#[test]
	fn should_extract_two_pair_ignoring_extra_pairs() {
		assert_eq!(
			// contains extra pair sixes
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Queen, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Clubs),
				],
				community_cards: &vec![
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Six, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Spades),
				],
			}),
			Hand {
				rank: HandRank::TwoPair,
				rank_cards: vec![
					Card::new(Rank::Queen, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Spades),
					Card::new(Rank::Jack, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
				kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
			}
		);
	}

	#[test]
	fn should_extract_three_of_a_kind() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Six, Suit::Clubs),
				],
				community_cards: &vec![
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Six, Suit::Hearts),
					Card::new(Rank::Queen, Suit::Clubs),
				],
			}),
			Hand {
				rank: HandRank::ThreeOfAKind,
				rank_cards: vec![
					Card::new(Rank::Six, Suit::Clubs),
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Six, Suit::Hearts),
				],
				kicker_cards: vec![
					Card::new(Rank::Queen, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
			}
		);
	}

	#[test]
	fn should_extract_straight() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Six, Suit::Hearts),
				],
				community_cards: &vec![
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Clubs),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Clubs),
					Card::new(Rank::Six, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::Straight,
				rank_cards: vec![
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Clubs),
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Clubs),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_straight_ace_low() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Ace, Suit::Spades),
					Card::new(Rank::Eight, Suit::Hearts),
				],
				community_cards: &vec![
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Clubs),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Clubs),
					Card::new(Rank::Eight, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::Straight,
				rank_cards: vec![
					Card::new(Rank::Five, Suit::Clubs),
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Clubs),
					Card::new(Rank::Ace, Suit::Spades),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_not_extract_straight_ace_ambiguous() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Ace, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Diamonds),
				],
				community_cards: &vec![
					Card::new(Rank::Nine, Suit::Diamonds),
					Card::new(Rank::Queen, Suit::Clubs),
					Card::new(Rank::Seven, Suit::Hearts),
					Card::new(Rank::King, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::HighCard,
				rank_cards: vec![Card::new(Rank::Ace, Suit::Diamonds)],
				kicker_cards: vec![
					Card::new(Rank::King, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Clubs),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Nine, Suit::Diamonds),
				],
			}
		);
	}

	#[test]
	fn should_extract_straight_queen_ace_high() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Clubs),
				],
				community_cards: &vec![
					Card::new(Rank::Queen, Suit::Diamonds),
					Card::new(Rank::King, Suit::Clubs),
					Card::new(Rank::Eight, Suit::Diamonds),
					Card::new(Rank::Ten, Suit::Hearts),
					Card::new(Rank::Ace, Suit::Hearts),
				],
			}),
			Hand {
				rank: HandRank::Straight,
				rank_cards: vec![
					Card::new(Rank::Ace, Suit::Hearts),
					Card::new(Rank::King, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Ten, Suit::Hearts),
				],
				kicker_cards: vec![],
			}
		);

		// invert community

		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Clubs),
				],
				community_cards: &vec![
					Card::new(Rank::Ace, Suit::Hearts),
					Card::new(Rank::Ten, Suit::Hearts),
					Card::new(Rank::Eight, Suit::Diamonds),
					Card::new(Rank::King, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::Straight,
				rank_cards: vec![
					Card::new(Rank::Ace, Suit::Hearts),
					Card::new(Rank::King, Suit::Clubs),
					Card::new(Rank::Queen, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Ten, Suit::Hearts),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_flush() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
				community_cards: &vec![
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Five, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Hearts),
				],
			}),
			Hand {
				rank: HandRank::Flush,
				rank_cards: vec![
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Diamonds),
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Two, Suit::Diamonds),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_full_house() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Four, Suit::Diamonds),
				],
				community_cards: &vec![
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Hearts),
				],
			}),
			Hand {
				rank: HandRank::FullHouse,
				rank_cards: vec![
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Hearts),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_full_house_with_highest_pair() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Hearts),
				],
				community_cards: &vec![
					Card::new(Rank::Four, Suit::Hearts),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Spades),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::FullHouse,
				rank_cards: vec![
					Card::new(Rank::Jack, Suit::Hearts),
					Card::new(Rank::Jack, Suit::Spades),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Four, Suit::Hearts),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_full_house_with_highest_triplet() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Four, Suit::Diamonds),
					Card::new(Rank::Three, Suit::Clubs),
				],
				community_cards: &vec![
					Card::new(Rank::Three, Suit::Hearts),
					Card::new(Rank::Three, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Spades),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Hearts),
				],
			}),
			Hand {
				rank: HandRank::FullHouse,
				rank_cards: vec![
					Card::new(Rank::Jack, Suit::Spades),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Hearts),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Three, Suit::Hearts),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_four_of_a_kind() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Six, Suit::Diamonds),
				],
				community_cards: &vec![
					Card::new(Rank::Six, Suit::Hearts),
					Card::new(Rank::Six, Suit::Spades),
					Card::new(Rank::Queen, Suit::Clubs),
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Six, Suit::Clubs),
				],
			}),
			Hand {
				rank: HandRank::FourOfAKind,
				rank_cards: vec![
					Card::new(Rank::Six, Suit::Diamonds),
					Card::new(Rank::Six, Suit::Hearts),
					Card::new(Rank::Six, Suit::Spades),
					Card::new(Rank::Six, Suit::Clubs),
				],
				kicker_cards: vec![Card::new(Rank::Queen, Suit::Clubs)],
			}
		);
	}

	#[test]
	fn should_extract_straight_flush() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Two, Suit::Clubs),
					Card::new(Rank::Six, Suit::Clubs),
				],
				community_cards: &vec![
					Card::new(Rank::Four, Suit::Clubs),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Five, Suit::Clubs),
					Card::new(Rank::Six, Suit::Spades),
				],
			}),
			Hand {
				rank: HandRank::StraightFlush,
				rank_cards: vec![
					Card::new(Rank::Six, Suit::Clubs),
					Card::new(Rank::Five, Suit::Clubs),
					Card::new(Rank::Four, Suit::Clubs),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Two, Suit::Clubs),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_straight_flush_ace_low() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Ace, Suit::Spades),
					Card::new(Rank::Eight, Suit::Hearts),
				],
				community_cards: &vec![
					Card::new(Rank::Four, Suit::Spades),
					Card::new(Rank::Two, Suit::Spades),
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Five, Suit::Spades),
					Card::new(Rank::Eight, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::StraightFlush,
				rank_cards: vec![
					Card::new(Rank::Five, Suit::Spades),
					Card::new(Rank::Four, Suit::Spades),
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Two, Suit::Spades),
					Card::new(Rank::Ace, Suit::Spades),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_not_extract_straight_flush_offsuit_ace() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Ace, Suit::Clubs),
					Card::new(Rank::Two, Suit::Spades),
				],
				community_cards: &vec![
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Four, Suit::Spades),
					Card::new(Rank::Five, Suit::Spades),
					Card::new(Rank::Eight, Suit::Spades),
				],
			}),
			Hand {
				rank: HandRank::Flush,
				rank_cards: vec![
					Card::new(Rank::Eight, Suit::Spades),
					Card::new(Rank::Five, Suit::Spades),
					Card::new(Rank::Four, Suit::Spades),
					Card::new(Rank::Three, Suit::Spades),
					Card::new(Rank::Two, Suit::Spades),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_not_extract_straight_flush_ambiguous_ace() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Ace, Suit::Diamonds),
					Card::new(Rank::Five, Suit::Diamonds),
				],
				community_cards: &vec![
					Card::new(Rank::Nine, Suit::Diamonds),
					Card::new(Rank::Queen, Suit::Diamonds),
					Card::new(Rank::Seven, Suit::Diamonds),
					Card::new(Rank::King, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::Flush,
				rank_cards: vec![
					Card::new(Rank::Ace, Suit::Diamonds),
					Card::new(Rank::King, Suit::Diamonds),
					Card::new(Rank::Queen, Suit::Diamonds),
					Card::new(Rank::Jack, Suit::Diamonds),
					Card::new(Rank::Nine, Suit::Diamonds),
				],
				kicker_cards: vec![],
			}
		);
	}

	#[test]
	fn should_extract_royal_flush() {
		assert_eq!(
			Hand::from(HandCandidate {
				pocket_cards: &[
					Card::new(Rank::Ace, Suit::Hearts),
					Card::new(Rank::Jack, Suit::Hearts),
				],
				community_cards: &vec![
					Card::new(Rank::Ten, Suit::Hearts),
					Card::new(Rank::King, Suit::Hearts),
					Card::new(Rank::Queen, Suit::Hearts),
					Card::new(Rank::Three, Suit::Clubs),
					Card::new(Rank::Four, Suit::Diamonds),
				],
			}),
			Hand {
				rank: HandRank::RoyalFlush,
				rank_cards: vec![
					Card::new(Rank::Ace, Suit::Hearts),
					Card::new(Rank::King, Suit::Hearts),
					Card::new(Rank::Queen, Suit::Hearts),
					Card::new(Rank::Jack, Suit::Hearts),
					Card::new(Rank::Ten, Suit::Hearts),
				],
				kicker_cards: vec![],
			}
		);
	}
}
