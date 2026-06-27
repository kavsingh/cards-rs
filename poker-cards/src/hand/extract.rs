use crate::util::{chunk_by, group_by, without};
use crate::{Card, Hand, HandRank, Rank};

fn to_hand(rank: HandRank, rank_cards: Vec<Card>, sorted: &[Card]) -> Hand {
	let kicker_cards = without(&rank_cards, sorted)
		.into_iter()
		.take(5 - rank_cards.len())
		.collect();

	Hand {
		rank,
		rank_cards,
		kicker_cards,
	}
}

pub type Extractor = fn(&[Card]) -> Option<Hand>;
pub const ORDERED_EXTRACTORS: [Extractor; 9] = [
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
		.map(|(_, cards)| {
			to_hand(HandRank::FourOfAKind, cards.to_vec(), sorted)
		})
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
	};

	None
}

fn extract_three_of_a_kind(sorted: &[Card]) -> Option<Hand> {
	group_by(sorted, |c| c.rank)
		.iter()
		.find(|(_, cards)| cards.len() == 3)
		.map(|(_, cards)| {
			to_hand(HandRank::ThreeOfAKind, cards.to_vec(), sorted)
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
