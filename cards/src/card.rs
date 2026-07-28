use std::fmt::{Debug, Display};
use std::str::FromStr;

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
			Suit::Diamonds => "d",
			Suit::Clubs => "c",
			Suit::Hearts => "h",
			Suit::Spades => "s",
		};

		f.write_str(display)
	}
}

impl FromStr for Suit {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"d" => Ok(Suit::Diamonds),
			"c" => Ok(Suit::Clubs),
			"h" => Ok(Suit::Hearts),
			"s" => Ok(Suit::Spades),
			_ => Err(format!("invalid suit: {}", s)),
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

impl FromStr for Rank {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"2" => Ok(Rank::Two),
			"3" => Ok(Rank::Three),
			"4" => Ok(Rank::Four),
			"5" => Ok(Rank::Five),
			"6" => Ok(Rank::Six),
			"7" => Ok(Rank::Seven),
			"8" => Ok(Rank::Eight),
			"9" => Ok(Rank::Nine),
			"10" => Ok(Rank::Ten),
			"J" => Ok(Rank::Jack),
			"Q" => Ok(Rank::Queen),
			"K" => Ok(Rank::King),
			"A" => Ok(Rank::Ace),
			_ => Err(format!("invalid rank: {}", s)),
		}
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

impl FromStr for Card {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() < 2 {
			return Err(format!("invalid card: {}", s));
		}

		let (rank_str, suit_str) = s.split_at(s.len() - 1);
		let rank = Rank::from_str(rank_str)?;
		let suit = Suit::from_str(suit_str)?;

		Ok(Card { rank, suit })
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::{Card, Rank, Suit};

	#[test]
	fn test_card_display() {
		let card = Card::new(Rank::Ace, Suit::Spades);

		assert_eq!(format!("{}", card), "As");
	}

	#[test]
	fn test_card_from_str() {
		let card = Card::from_str("10h").unwrap();

		assert_eq!(card.rank, Rank::Ten);
		assert_eq!(card.suit, Suit::Hearts);

		let card = Card::from_str("Jc").unwrap();

		assert_eq!(card.rank, Rank::Jack);
		assert_eq!(card.suit, Suit::Clubs);

		assert_eq!(Card::from_str("Zs").unwrap_err(), "invalid rank: Z");
		assert_eq!(Card::from_str("10q").unwrap_err(), "invalid suit: q");
		assert_eq!(Card::from_str("r").unwrap_err(), "invalid card: r");
	}
}
