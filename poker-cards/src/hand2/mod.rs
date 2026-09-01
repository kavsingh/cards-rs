mod compare;
mod extract;

use crate::Card;

pub struct HandCandidate {
	sorted_cards: Vec<Card>,
}

impl HandCandidate {
	#[must_use]
	pub fn new(pocket_cards: &[Card; 2], community_cards: &[Card]) -> Self {
		let mut sorted_cards = pocket_cards.to_vec();
		let mut community = community_cards.to_owned();

		sorted_cards.append(&mut community);
		sorted_cards.sort_by(|a, b| b.cmp(a));

		Self { sorted_cards }
	}
}

#[non_exhaustive]
#[derive(Debug)]
pub struct HighCard {
	pub high_card: Card,
	pub kickers: Vec<Card>,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Pair {
	pub pair: [Card; 2],
	pub kickers: Vec<Card>,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct TwoPair {
	pub high_pair: [Card; 2],
	pub low_pair: [Card; 2],
	pub kickers: Vec<Card>,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct ThreeOfAKind {
	pub triplet: [Card; 3],
	pub kickers: Vec<Card>,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Straight {
	pub straight: [Card; 5],
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Flush {
	pub flush: [Card; 5],
}

#[non_exhaustive]
#[derive(Debug)]
pub struct FullHouse {
	pub triplet: [Card; 3],
	pub pair: [Card; 2],
}

#[non_exhaustive]
#[derive(Debug)]
pub struct FourOfAKind {
	pub quad: [Card; 4],
	pub kickers: Vec<Card>,
}

#[non_exhaustive]
#[derive(Debug)]
pub struct StraightFlush {
	pub straight_flush: [Card; 5],
}

#[non_exhaustive]
#[derive(Debug)]
pub struct RoyalFlush {
	pub royal_flush: [Card; 5],
}

#[derive(Debug)]
pub enum Hand {
	HighCard(HighCard),
	Pair(Pair),
	TwoPair(TwoPair),
	ThreeOfAKind(ThreeOfAKind),
	Straight(Straight),
	Flush(Flush),
	FullHouse(FullHouse),
	FourOfAKind(FourOfAKind),
	StraightFlush(StraightFlush),
	RoyalFlush(RoyalFlush),
}
