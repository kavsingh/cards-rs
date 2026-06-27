use std::cmp::Ordering;

use cards::Rank;

use crate::util::cmp_max;
use crate::{Card, Hand};

pub fn highest_rank_then_kicker(a: &Hand, b: &Hand) -> Ordering {
	cmp_max(&a.rank_cards, &b.rank_cards)
		.then_with(|| cmp_max(&a.kicker_cards, &b.kicker_cards))
}

fn is_ace_low_straight(cards: &[Card]) -> bool {
	cards.iter().any(|c| c.rank == Rank::Ace)
		&& cards.iter().any(|c| c.rank == Rank::Two)
}

// for straights, we need to take into account ace low - ace is naturally high
pub fn straight(a: &Hand, b: &Hand) -> Ordering {
	match (
		is_ace_low_straight(&a.rank_cards),
		is_ace_low_straight(&b.rank_cards),
	) {
		(true, true) => Ordering::Equal,
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
pub fn two_pair(a: &Hand, b: &Hand) -> Ordering {
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
