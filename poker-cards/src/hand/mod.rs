use crate::Card;

mod compare;
mod describe;
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
