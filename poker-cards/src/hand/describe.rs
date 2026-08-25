use std::fmt::Display;

use super::{Hand, HandRank};
use crate::{Card, Rank};

fn describe_rank(card: Card) -> String {
	match card.rank {
		Rank::Ace => "Ace".to_string(),
		Rank::King => "King".to_string(),
		Rank::Queen => "Queen".to_string(),
		Rank::Jack => "Jack".to_string(),
		_ => card.rank.to_string(),
	}
}

fn describe_full_house(cards: &[Card]) -> String {
	let triplet_rank =
		cards.first().map(|c| describe_rank(*c)).unwrap_or_default();
	let pair_rank = cards.get(3).map(|c| describe_rank(*c)).unwrap_or_default();

	format!("{triplet_rank}s over {pair_rank}s")
}

fn describe_two_pair(cards: &[Card]) -> String {
	let high_rank =
		cards.first().map(|c| describe_rank(*c)).unwrap_or_default();
	let low_rank = cards.get(2).map(|c| describe_rank(*c)).unwrap_or_default();

	format!("{high_rank}s over {low_rank}s")
}

fn describe_straight(cards: &[Card]) -> String {
	let high_rank =
		cards.first().map(|c| describe_rank(*c)).unwrap_or_default();

	format!("{high_rank} high")
}

fn describe_hand(hand: &Hand) -> String {
	let first_rank = hand
		.rank_cards
		.first()
		.map(|c| describe_rank(*c))
		.unwrap_or_default();
	let first_kicker = hand
		.kicker_cards
		.first()
		.map(|c| describe_rank(*c))
		.unwrap_or_default();

	match hand.rank {
		HandRank::RoyalFlush => "Royal Flush".to_string(),

		HandRank::StraightFlush => {
			format!("Straight Flush, {}", describe_straight(&hand.rank_cards))
		}

		HandRank::FourOfAKind => {
			format!("Four of a Kind, {first_rank}s, {first_kicker} kicker")
		}

		HandRank::FullHouse => {
			format!(
				"Full House, {}, {first_kicker} kicker",
				describe_full_house(&hand.rank_cards)
			)
		}

		HandRank::Flush => format!("Flush, {first_rank} high"),

		HandRank::Straight => {
			format!("Straight, {}", describe_straight(&hand.rank_cards))
		}

		HandRank::ThreeOfAKind => {
			format!("Three of a Kind, {first_rank}s, {first_kicker} kicker")
		}

		HandRank::TwoPair => {
			format!(
				"Two Pair, {}, {first_kicker} kicker",
				describe_two_pair(&hand.rank_cards)
			)
		}

		HandRank::Pair => {
			format!("Pair of {first_rank}s, {first_kicker} kicker")
		}

		HandRank::HighCard => {
			format!("High Card, {first_rank}, {first_kicker} kicker")
		}
	}
}

impl Display for Hand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(describe_hand(self).as_str())
	}
}

#[cfg(test)]
mod tests {
	use super::{Hand, HandRank};
	use crate::{Card, Rank, Suit};

	#[test]
	fn should_describe_high_card() {
		let high_card = Hand {
			rank: HandRank::HighCard,
			rank_cards: vec![Card::new(Rank::King, Suit::Clubs)],
			kicker_cards: vec![
				Card::new(Rank::Queen, Suit::Clubs),
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Nine, Suit::Diamonds),
				Card::new(Rank::Seven, Suit::Diamonds),
			],
		};

		assert_eq!(
			high_card.to_string(),
			"High Card, King, Queen kicker".to_string()
		);
	}

	#[test]
	fn should_describe_pairs() {
		let pair_jacks = Hand {
			rank: HandRank::Pair,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Six, Suit::Diamonds),
				Card::new(Rank::Six, Suit::Clubs),
			],
			kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
		};

		assert_eq!(
			pair_jacks.to_string(),
			"Pair of Jacks, 8 kicker".to_string()
		);

		let two_pair_jack_fives = Hand {
			rank: HandRank::TwoPair,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Five, Suit::Diamonds),
				Card::new(Rank::Five, Suit::Clubs),
			],
			kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
		};

		assert_eq!(
			two_pair_jack_fives.to_string(),
			"Two Pair, Jacks over 5s, 8 kicker".to_string()
		);
	}

	#[test]
	fn should_describe_flush() {
		let flush = Hand {
			rank: HandRank::Flush,
			rank_cards: vec![
				Card::new(Rank::Queen, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Nine, Suit::Diamonds),
				Card::new(Rank::Eight, Suit::Diamonds),
				Card::new(Rank::Seven, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		assert_eq!(flush.to_string(), "Flush, Queen high".to_string());
	}

	#[test]
	fn should_describe_straights() {
		let straight_no_ace = Hand {
			rank: HandRank::Straight,
			rank_cards: vec![
				Card::new(Rank::Six, Suit::Hearts),
				Card::new(Rank::Five, Suit::Diamonds),
				Card::new(Rank::Four, Suit::Clubs),
				Card::new(Rank::Three, Suit::Diamonds),
				Card::new(Rank::Two, Suit::Spades),
			],
			kicker_cards: vec![],
		};

		assert_eq!(straight_no_ace.to_string(), "Straight, 6 high");

		let straight_ace_low = Hand {
			rank: HandRank::Straight,
			rank_cards: vec![
				Card::new(Rank::Five, Suit::Diamonds),
				Card::new(Rank::Four, Suit::Clubs),
				Card::new(Rank::Three, Suit::Diamonds),
				Card::new(Rank::Two, Suit::Hearts),
				Card::new(Rank::Ace, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		assert_eq!(straight_ace_low.to_string(), "Straight, 5 high");

		let straight_ace_high = Hand {
			rank: HandRank::Straight,
			rank_cards: vec![
				Card::new(Rank::Ace, Suit::Diamonds),
				Card::new(Rank::King, Suit::Clubs),
				Card::new(Rank::Queen, Suit::Hearts),
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Ten, Suit::Spades),
			],
			kicker_cards: vec![],
		};

		assert_eq!(straight_ace_high.to_string(), "Straight, Ace high");

		let royal_flush = Hand {
			rank: HandRank::RoyalFlush,
			rank_cards: vec![
				Card::new(Rank::Ace, Suit::Clubs),
				Card::new(Rank::King, Suit::Clubs),
				Card::new(Rank::Queen, Suit::Clubs),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Ten, Suit::Clubs),
			],
			kicker_cards: vec![],
		};

		assert_eq!(royal_flush.to_string(), "Royal Flush");
	}
}
