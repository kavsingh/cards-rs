use super::{
	Flush, FourOfAKind, FullHouse, Hand, HighCard, Pair, RoyalFlush, Straight,
	StraightFlush, ThreeOfAKind, TwoPair,
};
use crate::util::cmp_max;
use crate::{Card, Rank, derive_total_ord};

fn cmp_straight(a: &[Card; 5], b: &[Card; 5]) -> std::cmp::Ordering {
	match (a[4].rank == Rank::Ace, b[4].rank == Rank::Ace) {
		(true, false) => std::cmp::Ordering::Less,
		(false, true) => std::cmp::Ordering::Greater,
		_ => cmp_max(a, b),
	}
}

impl Ord for HighCard {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.high_card
			.cmp(&other.high_card)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl Ord for Pair {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.pair, &other.pair)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl Ord for TwoPair {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.high_pair, &other.high_pair)
			.then_with(|| cmp_max(&self.low_pair, &other.low_pair))
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl Ord for ThreeOfAKind {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.triplet, &other.triplet)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl Ord for Straight {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_straight(&self.straight, &other.straight)
	}
}

impl Ord for Flush {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.flush, &other.flush)
	}
}

impl Ord for FullHouse {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.triplet, &other.triplet)
			.then_with(|| cmp_max(&self.pair, &other.pair))
	}
}

impl Ord for FourOfAKind {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.quad, &other.quad)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl Ord for StraightFlush {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_straight(&self.straight_flush, &other.straight_flush)
	}
}

impl Ord for RoyalFlush {
	fn cmp(&self, _: &Self) -> std::cmp::Ordering {
		std::cmp::Ordering::Equal
	}
}

derive_total_ord!(
	HighCard,
	Pair,
	TwoPair,
	ThreeOfAKind,
	Straight,
	Flush,
	FullHouse,
	FourOfAKind,
	StraightFlush,
	RoyalFlush
);

trait RankedHand {
	fn rank(&self) -> u8;
}

impl RankedHand for Hand {
	fn rank(&self) -> u8 {
		match self {
			Self::HighCard(_) => 0,
			Self::Pair(_) => 1,
			Self::TwoPair(_) => 2,
			Self::ThreeOfAKind(_) => 3,
			Self::Straight(_) => 4,
			Self::Flush(_) => 5,
			Self::FullHouse(_) => 6,
			Self::FourOfAKind(_) => 7,
			Self::StraightFlush(_) => 8,
			Self::RoyalFlush(_) => 9,
		}
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.rank()
			.cmp(&other.rank())
			.then_with(|| match (self, other) {
				(Self::HighCard(a), Self::HighCard(b)) => a.cmp(b),
				(Self::Pair(a), Self::Pair(b)) => a.cmp(b),
				(Self::TwoPair(a), Self::TwoPair(b)) => a.cmp(b),
				(Self::ThreeOfAKind(a), Self::ThreeOfAKind(b)) => a.cmp(b),
				(Self::Straight(a), Self::Straight(b)) => a.cmp(b),
				(Self::Flush(a), Self::Flush(b)) => a.cmp(b),
				(Self::FullHouse(a), Self::FullHouse(b)) => a.cmp(b),
				(Self::FourOfAKind(a), Self::FourOfAKind(b)) => a.cmp(b),
				(Self::StraightFlush(a), Self::StraightFlush(b)) => a.cmp(b),
				(Self::RoyalFlush(a), Self::RoyalFlush(b)) => a.cmp(b),
				_ => std::cmp::Ordering::Less,
			})
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for Hand {}

//

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
	fn c(s: &str) -> super::Card {
		s.parse().unwrap()
	}

	#[test]
	#[allow(clippy::too_many_lines)]
	fn should_compare_hands() {
		let high_king_10_kicker = super::Hand::HighCard(super::HighCard {
			high_card: c("Kd"),
			kickers: vec![c("Tc")],
		});

		let high_king_6_kicker = super::Hand::HighCard(super::HighCard {
			high_card: c("Kc"),
			kickers: vec![c("6d")],
		});

		let two_pair_jack_sixes = super::Hand::TwoPair(super::TwoPair {
			high_pair: [c("Jd"), c("Jc")],
			low_pair: [c("6d"), c("6c")],
			kickers: vec![],
		});

		let two_pair_jack_fives = super::Hand::TwoPair(super::TwoPair {
			high_pair: [c("Jd"), c("Jc")],
			low_pair: [c("5d"), c("5c")],
			kickers: vec![],
		});

		let straight_no_ace = super::Hand::Straight(super::Straight {
			straight: [c("6h"), c("5d"), c("4c"), c("3d"), c("2s")],
		});

		let straight_ace_low = super::Hand::Straight(super::Straight {
			straight: [c("5d"), c("4c"), c("3d"), c("2h"), c("Ad")],
		});

		let straight_ace_high = super::Hand::Straight(super::Straight {
			straight: [c("Ad"), c("Kc"), c("Qh"), c("Jd"), c("Ts")],
		});

		let flush_king_high = super::Hand::Flush(super::Flush {
			flush: [c("Kd"), c("Qd"), c("8d"), c("7d"), c("4d")],
		});

		let flush_seven_high = super::Hand::Flush(super::Flush {
			flush: [c("7s"), c("6s"), c("5s"), c("3s"), c("2s")],
		});

		let full_house_ace_king = super::Hand::FullHouse(super::FullHouse {
			triplet: [c("Ad"), c("Ac"), c("Ah")],
			pair: [c("Kd"), c("Kc")],
		});

		let full_house_ace_6 = super::Hand::FullHouse(super::FullHouse {
			triplet: [c("Ad"), c("Ac"), c("Ah")],
			pair: [c("6d"), c("6c")],
		});

		let full_house_10_3 = super::Hand::FullHouse(super::FullHouse {
			triplet: [c("Td"), c("Tc"), c("Th")],
			pair: [c("3d"), c("3c")],
		});

		let four_of_a_kind_9_2 = super::Hand::FourOfAKind(super::FourOfAKind {
			quad: [c("9d"), c("9c"), c("9h"), c("9s")],
			kickers: vec![c("2d")],
		});

		let four_of_a_kind_9_6 = super::Hand::FourOfAKind(super::FourOfAKind {
			quad: [c("9d"), c("9c"), c("9h"), c("9s")],
			kickers: vec![c("6d")],
		});

		let four_of_a_kind_j = super::Hand::FourOfAKind(super::FourOfAKind {
			quad: [c("Jd"), c("Jc"), c("Jh"), c("Js")],
			kickers: vec![c("8d")],
		});

		let straight_flush_9_high =
			super::Hand::StraightFlush(super::StraightFlush {
				straight_flush: [c("9d"), c("8d"), c("7d"), c("6d"), c("5d")],
			});

		let straight_flush_10_high =
			super::Hand::StraightFlush(super::StraightFlush {
				straight_flush: [c("Td"), c("9d"), c("8d"), c("7d"), c("6d")],
			});

		let straight_flush_ace_low =
			super::Hand::StraightFlush(super::StraightFlush {
				straight_flush: [c("5d"), c("4d"), c("3d"), c("2d"), c("Ad")],
			});

		let royal_flush_diamonds = super::Hand::RoyalFlush(super::RoyalFlush {
			royal_flush: [c("Ad"), c("Kd"), c("Qd"), c("Jd"), c("Td")],
		});

		let royal_flush_clubs = super::Hand::RoyalFlush(super::RoyalFlush {
			royal_flush: [c("Ac"), c("Kc"), c("Qc"), c("Jc"), c("Tc")],
		});

		let mut hands = [
			&straight_ace_high,
			&four_of_a_kind_9_6,
			&two_pair_jack_sixes,
			&straight_flush_10_high,
			&royal_flush_clubs,
			&flush_seven_high,
			&full_house_ace_king,
			&four_of_a_kind_j,
			&straight_flush_ace_low,
			&straight_ace_low,
			&straight_flush_9_high,
			&full_house_10_3,
			&four_of_a_kind_9_2,
			&royal_flush_diamonds,
			&high_king_10_kicker,
			&full_house_ace_6,
			&two_pair_jack_fives,
			&flush_king_high,
			&straight_no_ace,
			&high_king_6_kicker,
		];

		hands.sort();

		assert_eq!(
			[
				&high_king_6_kicker,
				&high_king_10_kicker,
				&two_pair_jack_fives,
				&two_pair_jack_sixes,
				&straight_ace_low,
				&straight_no_ace,
				&straight_ace_high,
				&flush_seven_high,
				&flush_king_high,
				&full_house_10_3,
				&full_house_ace_6,
				&full_house_ace_king,
				&four_of_a_kind_9_2,
				&four_of_a_kind_9_6,
				&four_of_a_kind_j,
				&straight_flush_ace_low,
				&straight_flush_9_high,
				&straight_flush_10_high,
				&royal_flush_clubs,
				&royal_flush_diamonds,
			],
			hands,
		);
	}
}
