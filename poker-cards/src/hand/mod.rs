mod candidate;
mod compare;
mod describe;
mod extract;

use std::str::FromStr;

pub use candidate::HandCandidate;
pub use compare::RankedHand;
pub use describe::Describe;

use crate::Card;

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

impl FromStr for Hand {
	type Err = &'static str;

	fn from_str(value: &'_ str) -> Result<Self, Self::Err> {
		let candidate = value.parse::<HandCandidate>()?;

		Self::try_from(&candidate).map_err(|_| "invalid")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_str() {
		match "Ts As Qs Ks Js".parse::<Hand>() {
			#[allow(clippy::assertions_on_constants)]
			Ok(_) => assert!(true),
			_ => panic!("Failed to create hand"),
		}
	}
}
