use cards::Rank;

use super::{
	Flush, FourOfAKind, FullHouse, Hand, HighCard, Pair, RoyalFlush, Straight,
	StraightFlush, ThreeOfAKind, TwoPair,
};
use crate::Card;

fn describe_rank(card: Card, is_many: bool) -> String {
	let (one, many) = match card.rank {
		Rank::Ace => ("Ace", "Aces"),
		Rank::King => ("King", "Kings"),
		Rank::Queen => ("Queen", "Queens"),
		Rank::Jack => ("Jack", "Jacks"),
		Rank::Ten => ("Ten", "Tens"),
		Rank::Nine => ("Nine", "Nines"),
		Rank::Eight => ("Eight", "Eights"),
		Rank::Seven => ("Seven", "Sevens"),
		Rank::Six => ("Six", "Sixes"),
		Rank::Five => ("Five", "Fives"),
		Rank::Four => ("Four", "Fours"),
		Rank::Three => ("Three", "Threes"),
		Rank::Two => ("Two", "Twos"),
	};

	if is_many { many.to_string() } else { one.to_string() }
}

fn describe_kickers(kickers: &[Card]) -> String {
	if kickers.is_empty() {
		return String::new();
	}

	let mut rank_names = vec![];

	for kicker in kickers {
		let named = describe_rank(*kicker, false);

		if !rank_names.contains(&named) {
			rank_names.push(named);
		}
	}

	let suffix = if rank_names.len() == 1 { "kicker" } else { "kickers" };

	format!(", {} {}", rank_names.join(" "), suffix)
}

pub trait Describe {
	fn describe(&self) -> String;
}

impl Describe for HighCard {
	fn describe(&self) -> String {
		format!(
			"High card, {}{}",
			describe_rank(self.high_card, false),
			describe_kickers(&self.kickers)
		)
	}
}

impl Describe for Pair {
	fn describe(&self) -> String {
		format!(
			"Pair of {}{}",
			describe_rank(self.pair[0], true),
			describe_kickers(&self.kickers)
		)
	}
}

impl Describe for TwoPair {
	fn describe(&self) -> String {
		format!(
			"Two pair, {} over {}{}",
			describe_rank(self.high_pair[0], true),
			describe_rank(self.low_pair[0], true),
			describe_kickers(&self.kickers)
		)
	}
}

impl Describe for ThreeOfAKind {
	fn describe(&self) -> String {
		format!(
			"Three of a kind, {}{}",
			describe_rank(self.triplet[0], true),
			describe_kickers(&self.kickers)
		)
	}
}

impl Describe for Straight {
	fn describe(&self) -> String {
		format!("Straight, {} high", describe_rank(self.straight[0], false))
	}
}

impl Describe for Flush {
	fn describe(&self) -> String {
		format!("Flush, {} high", describe_rank(self.flush[0], false))
	}
}

impl Describe for FullHouse {
	fn describe(&self) -> String {
		format!(
			"Full house, {} over {}",
			describe_rank(self.triplet[0], true),
			describe_rank(self.pair[0], true)
		)
	}
}

impl Describe for FourOfAKind {
	fn describe(&self) -> String {
		format!(
			"Four of a kind, {}{}",
			describe_rank(self.quad[0], true),
			describe_kickers(&self.kickers)
		)
	}
}

impl Describe for StraightFlush {
	fn describe(&self) -> String {
		format!(
			"Straight flush, {} high",
			describe_rank(self.straight_flush[0], false)
		)
	}
}

impl Describe for RoyalFlush {
	fn describe(&self) -> String {
		"Royal flush".to_string()
	}
}

impl Describe for Hand {
	fn describe(&self) -> String {
		match self {
			Self::HighCard(hand) => hand.describe(),
			Self::Pair(hand) => hand.describe(),
			Self::TwoPair(hand) => hand.describe(),
			Self::ThreeOfAKind(hand) => hand.describe(),
			Self::Straight(hand) => hand.describe(),
			Self::Flush(hand) => hand.describe(),
			Self::FullHouse(hand) => hand.describe(),
			Self::FourOfAKind(hand) => hand.describe(),
			Self::StraightFlush(hand) => hand.describe(),
			Self::RoyalFlush(hand) => hand.describe(),
		}
	}
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
	use super::*;

	#[test]
	fn high_card() {
		let no_kickers = Hand::HighCard(HighCard {
			high_card: "As".parse().unwrap(),
			kickers: vec![],
		});

		let with_kickers = Hand::HighCard(HighCard {
			high_card: "As".parse().unwrap(),
			kickers: vec![
				"8s".parse().unwrap(),
				"4d".parse().unwrap(),
				"2h".parse().unwrap(),
			],
		});

		assert_eq!(no_kickers.describe(), "High card, Ace");
		assert_eq!(
			with_kickers.describe(),
			"High card, Ace, Eight Four Two kickers"
		);
	}

	#[test]
	fn pair() {
		let no_kickers = Pair {
			pair: ["4s".parse().unwrap(), "4h".parse().unwrap()],
			kickers: vec![],
		};

		let with_kickers = Pair {
			pair: ["6s".parse().unwrap(), "6h".parse().unwrap()],
			kickers: vec!["Ks".parse().unwrap()],
		};

		assert_eq!(no_kickers.describe(), "Pair of Fours");
		assert_eq!(with_kickers.describe(), "Pair of Sixes, King kicker");
	}

	#[test]
	fn two_pair() {
		let no_kickers = TwoPair {
			high_pair: ["As".parse().unwrap(), "Ah".parse().unwrap()],
			low_pair: ["Ks".parse().unwrap(), "Kh".parse().unwrap()],
			kickers: vec![],
		};

		let with_kickers = TwoPair {
			high_pair: ["As".parse().unwrap(), "Ah".parse().unwrap()],
			low_pair: ["Ks".parse().unwrap(), "Kh".parse().unwrap()],
			kickers: vec!["Qs".parse().unwrap()],
		};

		assert_eq!(no_kickers.describe(), "Two pair, Aces over Kings");
		assert_eq!(
			with_kickers.describe(),
			"Two pair, Aces over Kings, Queen kicker"
		);
	}

	#[test]
	fn three_of_a_kind() {
		let no_kickers = ThreeOfAKind {
			triplet: [
				"As".parse().unwrap(),
				"Ah".parse().unwrap(),
				"Ad".parse().unwrap(),
			],
			kickers: vec![],
		};

		let with_kickers = ThreeOfAKind {
			triplet: [
				"As".parse().unwrap(),
				"Ah".parse().unwrap(),
				"Ad".parse().unwrap(),
			],
			kickers: vec!["Ks".parse().unwrap()],
		};

		assert_eq!(no_kickers.describe(), "Three of a kind, Aces");
		assert_eq!(
			with_kickers.describe(),
			"Three of a kind, Aces, King kicker"
		);
	}

	#[test]
	fn straight() {
		let straight = Straight {
			straight: [
				"8s".parse().unwrap(),
				"7h".parse().unwrap(),
				"6d".parse().unwrap(),
				"5s".parse().unwrap(),
				"4h".parse().unwrap(),
			],
		};

		assert_eq!(straight.describe(), "Straight, Eight high");
	}

	#[test]
	fn flush() {
		let flush = Flush {
			flush: [
				"Ks".parse().unwrap(),
				"Qs".parse().unwrap(),
				"Js".parse().unwrap(),
				"9s".parse().unwrap(),
				"4s".parse().unwrap(),
			],
		};

		assert_eq!(flush.describe(), "Flush, King high");
	}

	#[test]
	fn four_of_a_kind() {
		let no_kickers = FourOfAKind {
			quad: [
				"As".parse().unwrap(),
				"Ah".parse().unwrap(),
				"Ad".parse().unwrap(),
				"Ac".parse().unwrap(),
			],
			kickers: vec![],
		};

		let with_kickers = FourOfAKind {
			quad: [
				"As".parse().unwrap(),
				"Ah".parse().unwrap(),
				"Ad".parse().unwrap(),
				"Ac".parse().unwrap(),
			],
			kickers: vec!["Ks".parse().unwrap()],
		};

		assert_eq!(no_kickers.describe(), "Four of a kind, Aces");
		assert_eq!(
			with_kickers.describe(),
			"Four of a kind, Aces, King kicker"
		);
	}

	#[test]
	fn straight_flush() {
		let straight_flush = StraightFlush {
			straight_flush: [
				"8s".parse().unwrap(),
				"7s".parse().unwrap(),
				"6s".parse().unwrap(),
				"5s".parse().unwrap(),
				"4s".parse().unwrap(),
			],
		};

		assert_eq!(straight_flush.describe(), "Straight flush, Eight high");
	}

	#[test]
	fn royal_flush() {
		let royal_flush = RoyalFlush {
			royal_flush: [
				"As".parse().unwrap(),
				"Ks".parse().unwrap(),
				"Qs".parse().unwrap(),
				"Js".parse().unwrap(),
				"Ts".parse().unwrap(),
			],
		};

		assert_eq!(royal_flush.describe(), "Royal flush");
	}
}
