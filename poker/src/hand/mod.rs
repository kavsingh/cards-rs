use std::cmp::Ordering;

use crate::Card;
use crate::hand::compare::{highest_rank_then_kicker, straight, two_pair};

mod compare;
mod extract;

pub struct HandCandidate<'a> {
	pocket_cards: &'a [Card; 2],
	community_cards: &'a Vec<Card>,
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
pub enum HandRank {
	HighCard,
	Pair,
	TwoPair,
	ThreeOfAKind,
	Straight,
	Flush,
	FullHouse,
	FourOfAKind,
	StraightFlush,
	RoyalFlush,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
	pub rank: HandRank,
	pub rank_cards: Vec<Card>,
	pub kicker_cards: Vec<Card>,
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		self.rank.cmp(&other.rank).then_with(|| match self.rank {
			HandRank::RoyalFlush => Ordering::Equal,
			HandRank::TwoPair => two_pair(self, other),
			HandRank::Straight | HandRank::StraightFlush => {
				straight(self, other)
			}
			_ => highest_rank_then_kicker(self, other),
		})
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl From<HandCandidate<'_>> for Hand {
	fn from(value: HandCandidate) -> Self {
		let mut community = value.community_cards.to_owned();
		let mut sorted = value.pocket_cards.to_vec();

		sorted.append(&mut community);
		sorted.sort_by(|a, b| b.cmp(a));

		if let Some(hand) = extract::ORDERED_EXTRACTORS
			.iter()
			.find_map(|extractor| extractor(&sorted))
		{
			return hand;
		}

		// since pocket cards are required to be a 2 card array, we can
		// be sure that sorted has at least 2 cards
		let highest_card = sorted[0];

		Hand {
			rank: HandRank::HighCard,
			rank_cards: vec![highest_card],
			kicker_cards: sorted[1..5].to_vec(),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::vec;

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

	#[test]
	fn should_compare_hands() {
		let two_pair_jack_sixes = Hand {
			rank: HandRank::TwoPair,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Six, Suit::Diamonds),
				Card::new(Rank::Six, Suit::Clubs),
			],
			kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
		};

		let two_pair_jack_fives = Hand {
			rank: HandRank::TwoPair,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Five, Suit::Diamonds),
				Card::new(Rank::Five, Suit::Clubs),
			],
			kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
		};

		let high_card = Hand {
			rank: HandRank::HighCard,
			rank_cards: vec![Card::new(Rank::King, Suit::Clubs)],
			kicker_cards: vec![
				Card::new(Rank::Queen, Suit::Clubs),
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Nine, Suit::Diamonds),
				Card::new(Rank::Seven, Suit::Diamonds),
			],
		};

		let flush = Hand {
			rank: HandRank::Flush,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Queen, Suit::Diamonds),
				Card::new(Rank::Nine, Suit::Diamonds),
				Card::new(Rank::Eight, Suit::Diamonds),
				Card::new(Rank::Seven, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		let straight_no_ace = Hand {
			rank: HandRank::Straight,
			rank_cards: vec![
				Card::new(Rank::Seven, Suit::Diamonds),
				Card::new(Rank::Six, Suit::Diamonds),
				Card::new(Rank::Five, Suit::Diamonds),
				Card::new(Rank::Four, Suit::Diamonds),
				Card::new(Rank::Three, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		let straight_ace_low = Hand {
			rank: HandRank::Straight,
			rank_cards: vec![
				Card::new(Rank::Five, Suit::Diamonds),
				Card::new(Rank::Four, Suit::Diamonds),
				Card::new(Rank::Three, Suit::Diamonds),
				Card::new(Rank::Two, Suit::Diamonds),
				Card::new(Rank::Ace, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		let straight_ace_high = Hand {
			rank: HandRank::Straight,
			rank_cards: vec![
				Card::new(Rank::Ace, Suit::Diamonds),
				Card::new(Rank::King, Suit::Diamonds),
				Card::new(Rank::Queen, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Ten, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		let mut hands = [
			&flush,
			&straight_ace_high,
			&two_pair_jack_sixes,
			&straight_ace_low,
			&high_card,
			&two_pair_jack_fives,
			&straight_no_ace,
		];

		hands.sort();

		assert_eq!(
			[
				&high_card,
				&two_pair_jack_fives,
				&two_pair_jack_sixes,
				&straight_ace_low,
				&straight_no_ace,
				&straight_ace_high,
				&flush
			],
			hands,
		)
	}
}
