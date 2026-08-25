use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use thiserror::Error;

const ORDERED_RANKS: [cards::Rank; 13] = [
	cards::Rank::Two,
	cards::Rank::Three,
	cards::Rank::Four,
	cards::Rank::Five,
	cards::Rank::Six,
	cards::Rank::Seven,
	cards::Rank::Eight,
	cards::Rank::Nine,
	cards::Rank::Ten,
	cards::Rank::Jack,
	cards::Rank::Queen,
	cards::Rank::King,
	cards::Rank::Ace,
];

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CardError {
	#[error("parse error")]
	ParseError(#[from] cards::CardParseError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card(cards::Card);

impl Card {
	#[must_use]
	pub const fn new(rank: cards::Rank, suit: cards::Suit) -> Self {
		Self(cards::Card::new(rank, suit))
	}

	#[must_use]
	pub fn rank_value(&self) -> usize {
		ORDERED_RANKS
			.iter()
			.position(|r| *r == self.rank)
			.unwrap_or_default()
	}

	#[must_use]
	pub fn rank_diff(&self, other: &Self) -> isize {
		let self_rank = isize::try_from(self.rank_value()).unwrap_or_default();
		let other_rank =
			isize::try_from(other.rank_value()).unwrap_or_default();

		self_rank.saturating_sub(other_rank)
	}
}

impl Deref for Card {
	type Target = cards::Card;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Card {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Ord for Card {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.rank == other.rank {
			Ordering::Equal
		} else {
			self.rank_value().cmp(&other.rank_value())
		}
	}
}

impl PartialOrd for Card {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl FromStr for Card {
	type Err = CardError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let card = cards::Card::from_str(s)?;

		Ok(Self(card))
	}
}

#[cfg(test)]
mod tests {
	use cards::{Rank, Suit};

	use super::Card;

	#[test]
	#[allow(clippy::unwrap_used)]
	fn should_order_cards_on_rank() {
		assert_eq!(
			Card::new(Rank::Two, Suit::Hearts)
				.cmp(&Card::new(Rank::Two, Suit::Diamonds)),
			std::cmp::Ordering::Equal
		);

		let unsorted = ["Qd", "3h", "As", "Qc"];
		let sorted = ["3h", "Qd", "Qc", "As"];

		let mut stack = unsorted
			.iter()
			.map(|s| s.parse().unwrap())
			.collect::<Vec<Card>>();

		stack.sort();

		assert_eq!(
			stack.iter().map(Card::to_string).collect::<Vec<_>>(),
			sorted
		);
	}
}
