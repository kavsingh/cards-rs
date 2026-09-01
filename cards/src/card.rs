use std::fmt::{Debug, Display};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CardParseError {
	#[error("invalid rank: {0}")]
	InvalidRank(String),
	#[error("invalid suit: {0}")]
	InvalidSuit(String),
	#[error("invalid card: {0}")]
	InvalidCard(String),
}

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
			Self::Diamonds => "d",
			Self::Clubs => "c",
			Self::Hearts => "h",
			Self::Spades => "s",
		};

		f.write_str(display)
	}
}

#[mutants::skip]
impl FromStr for Suit {
	type Err = CardParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"d" => Ok(Self::Diamonds),
			"c" => Ok(Self::Clubs),
			"h" => Ok(Self::Hearts),
			"s" => Ok(Self::Spades),
			_ => Err(CardParseError::InvalidSuit(s.to_string())),
		}
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
			Self::Two => "2",
			Self::Three => "3",
			Self::Four => "4",
			Self::Five => "5",
			Self::Six => "6",
			Self::Seven => "7",
			Self::Eight => "8",
			Self::Nine => "9",
			Self::Ten => "T",
			Self::Jack => "J",
			Self::Queen => "Q",
			Self::King => "K",
			Self::Ace => "A",
		};

		f.write_str(display)
	}
}

#[mutants::skip]
impl FromStr for Rank {
	type Err = CardParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"2" => Ok(Self::Two),
			"3" => Ok(Self::Three),
			"4" => Ok(Self::Four),
			"5" => Ok(Self::Five),
			"6" => Ok(Self::Six),
			"7" => Ok(Self::Seven),
			"8" => Ok(Self::Eight),
			"9" => Ok(Self::Nine),
			"T" => Ok(Self::Ten),
			"J" => Ok(Self::Jack),
			"Q" => Ok(Self::Queen),
			"K" => Ok(Self::King),
			"A" => Ok(Self::Ace),
			_ => Err(CardParseError::InvalidRank(s.to_string())),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Card {
	pub rank: Rank,
	pub suit: Suit,
}

impl Card {
	#[must_use]
	pub const fn new(rank: Rank, suit: Suit) -> Self {
		Self { rank, suit }
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.rank, self.suit)
	}
}

impl FromStr for Card {
	type Err = CardParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (rank_str, suit_str) = s.split_at(s.len().saturating_sub(1));

		if rank_str.is_empty() || suit_str.is_empty() {
			return Err(CardParseError::InvalidCard(s.to_string()));
		}

		Ok(Self::new(rank_str.parse()?, suit_str.parse()?))
	}
}

#[cfg(test)]
mod tests {
	use super::{Card, CardParseError, Rank, Suit};

	#[test]
	fn test_card_display() {
		let card = Card::new(Rank::Ace, Suit::Spades);

		assert_eq!(format!("{card}"), "As");
	}

	#[test]
	#[allow(clippy::unwrap_used)]
	fn test_card_from_str() {
		let card: Card = "Th".parse().unwrap();

		assert_eq!(card.rank, Rank::Ten);
		assert_eq!(card.suit, Suit::Hearts);

		let card: Card = "Jc".parse().unwrap();

		assert_eq!(card.rank, Rank::Jack);
		assert_eq!(card.suit, Suit::Clubs);

		assert_eq!(
			"Zs".parse::<Card>().unwrap_err(),
			CardParseError::InvalidRank("Z".to_string())
		);
		assert_eq!(
			"Tq".parse::<Card>().unwrap_err(),
			CardParseError::InvalidSuit("q".to_string())
		);
		assert_eq!(
			"r".parse::<Card>().unwrap_err(),
			CardParseError::InvalidCard("r".to_string())
		);
	}
}
