use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

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

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Card(cards::Card);

impl Card {
	pub fn new(rank: cards::Rank, suit: cards::Suit) -> Self {
		Self(cards::Card::new(rank, suit))
	}

	pub fn rank_value(&self) -> usize {
		ORDERED_RANKS
			.iter()
			.position(|r| *r == self.rank)
			.unwrap_or_default()
	}

	pub fn rank_diff(&self, other: &Self) -> isize {
		self.rank_value() as isize - other.rank_value() as isize
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

impl Debug for Card {
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

#[cfg(test)]
mod tests {
	use cards::{Rank, Suit};

	use super::Card;

	#[test]
	fn should_order_cards_on_rank() {
		assert_eq!(
			Card::new(Rank::Two, Suit::Hearts)
				.cmp(&Card::new(Rank::Two, Suit::Diamonds)),
			std::cmp::Ordering::Equal
		);

		let mut stack = [
			Card::new(Rank::Three, Suit::Hearts),
			Card::new(Rank::Queen, Suit::Diamonds),
			Card::new(Rank::Ace, Suit::Spades),
			Card::new(Rank::Queen, Suit::Clubs),
		];

		stack.sort();

		assert_eq!(stack.map(|c| c.to_string()), ["3♥", "Q♦", "Q♣", "A♠"]);
	}
}
