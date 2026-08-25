use std::cmp::Ordering;

use super::{Hand, HandRank};
use crate::util::{cmp_max, is_ace_low_straight};

fn highest_rank_then_kicker(a: &Hand, b: &Hand) -> Ordering {
	cmp_max(&a.rank_cards, &b.rank_cards)
		.then_with(|| cmp_max(&a.kicker_cards, &b.kicker_cards))
}

// for straights, we need to take into account ace low - ace is naturally high
fn straight(a: &Hand, b: &Hand) -> Ordering {
	match (
		is_ace_low_straight(&a.rank_cards),
		is_ace_low_straight(&b.rank_cards),
	) {
		(true, false) => Ordering::Less,
		(false, true) => Ordering::Greater,
		_ => highest_rank_then_kicker(a, b),
	}
}

// for two pair, we need to compare each of the ranking pairs, then consider
// kickers, i.e.
// J J 6 6 10 > J J 5 5 10
// J J 6 6 10 > J J 6 6 8
// J J 6 6 8 = J J 6 6 8
fn two_pair(a: &Hand, b: &Hand) -> Ordering {
	let mut a_pairs = a.rank_cards.clone();
	let mut b_pairs = b.rank_cards.clone();

	a_pairs.sort();
	b_pairs.sort();

	let a_pairs = a_pairs.chunks(2).collect::<Vec<_>>();
	let b_pairs = b_pairs.chunks(2).collect::<Vec<_>>();

	for (a_pair, b_pair) in a_pairs.iter().zip(b_pairs.iter()) {
		let cmp = cmp_max(a_pair, b_pair);

		if cmp != Ordering::Equal {
			return cmp;
		}
	}

	cmp_max(&a.kicker_cards, &b.kicker_cards)
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		self.rank.cmp(&other.rank).then_with(|| match self.rank {
			HandRank::TwoPair => two_pair(self, other),
			HandRank::Straight | HandRank::StraightFlush => {
				straight(self, other)
			}
			_ => highest_rank_then_kicker(self, other),
		})
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod tests {
	use super::{Hand, HandRank};
	use crate::{Card, Rank, Suit};

	#[test]
	#[allow(clippy::too_many_lines)]
	fn should_compare_hands() {
		let two_pair_jack_sixes = Hand {
			rank: HandRank::TwoPair,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Clubs),
				Card::new(Rank::Six, Suit::Diamonds),
				Card::new(Rank::Six, Suit::Clubs),
			],
			kicker_cards: vec![Card::new(Rank::Eight, Suit::Spades)],
		};

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

		let flush = Hand {
			rank: HandRank::Flush,
			rank_cards: vec![
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Queen, Suit::Diamonds),
				Card::new(Rank::Nine, Suit::Diamonds),
				Card::new(Rank::Eight, Suit::Diamonds),
				Card::new(Rank::Seven, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

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

		let royal_flush_clubs = Hand {
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

		let royal_flush_diamonds = Hand {
			rank: HandRank::RoyalFlush,
			rank_cards: vec![
				Card::new(Rank::Ace, Suit::Diamonds),
				Card::new(Rank::King, Suit::Diamonds),
				Card::new(Rank::Queen, Suit::Diamonds),
				Card::new(Rank::Jack, Suit::Diamonds),
				Card::new(Rank::Ten, Suit::Diamonds),
			],
			kicker_cards: vec![],
		};

		let mut hands = [
			&royal_flush_diamonds,
			&flush,
			&straight_ace_high,
			&two_pair_jack_sixes,
			&royal_flush_clubs,
			&straight_ace_low,
			&high_card,
			&two_pair_jack_fives,
			&straight_no_ace,
		];

		hands.sort();

		assert_eq!(
			[
				&high_card,
				&two_pair_jack_fives,
				&two_pair_jack_sixes,
				&straight_ace_low,
				&straight_no_ace,
				&straight_ace_high,
				&flush,
				&royal_flush_diamonds,
				&royal_flush_clubs,
			],
			hands,
		);
	}
}
