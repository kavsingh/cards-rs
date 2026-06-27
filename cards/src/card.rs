use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
	Diamonds,
	Clubs,
	Hearts,
	Spades,
}

impl Display for Suit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let display = match self {
			Suit::Diamonds => "♦",
			Suit::Clubs => "♣",
			Suit::Hearts => "♥",
			Suit::Spades => "♠",
		};

		f.write_str(display)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rank {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

impl Display for Rank {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let display = match self {
			Rank::Two => "2",
			Rank::Three => "3",
			Rank::Four => "4",
			Rank::Five => "5",
			Rank::Six => "6",
			Rank::Seven => "7",
			Rank::Eight => "8",
			Rank::Nine => "9",
			Rank::Ten => "10",
			Rank::Jack => "J",
			Rank::Queen => "Q",
			Rank::King => "K",
			Rank::Ace => "A",
		};

		f.write_str(display)
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Card {
	pub rank: Rank,
	pub suit: Suit,
}

impl Card {
	pub fn new(rank: Rank, suit: Suit) -> Self {
		Card { rank, suit }
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.rank, self.suit)
	}
}

impl Debug for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}
