use super::{
	Flush, FullHouse, Hand, HighCard, Pair, Straight, ThreeOfAKind, TwoPair,
};
use crate::util::cmp_max;
use crate::{Card, Rank};

impl Ord for HighCard {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.high_card
			.cmp(&other.high_card)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl PartialOrd for HighCard {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for HighCard {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for HighCard {}

//

impl Ord for Pair {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.pair, &other.pair)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl PartialOrd for Pair {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Pair {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for Pair {}

//

impl Ord for TwoPair {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.high_pair, &other.high_pair)
			.then_with(|| cmp_max(&self.low_pair, &other.low_pair))
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl PartialOrd for TwoPair {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for TwoPair {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for TwoPair {}

//

impl Ord for ThreeOfAKind {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.triplet, &other.triplet)
			.then_with(|| cmp_max(&self.kickers, &other.kickers))
	}
}

impl PartialOrd for ThreeOfAKind {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for ThreeOfAKind {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for ThreeOfAKind {}

//

fn cmp_straight(a: &[Card; 5], b: &[Card; 5]) -> std::cmp::Ordering {
	match (a[4].rank == Rank::Ace, b[4].rank == Rank::Ace) {
		(true, false) => std::cmp::Ordering::Less,
		(false, true) => std::cmp::Ordering::Greater,
		_ => cmp_max(a, b),
	}
}

//

impl Ord for Straight {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_straight(&self.straight, &other.straight)
	}
}

impl PartialOrd for Straight {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Straight {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for Straight {}

//

impl Ord for Flush {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.flush, &other.flush)
	}
}

impl PartialOrd for Flush {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Flush {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for Flush {}

//

impl Ord for FullHouse {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		cmp_max(&self.triplet, &other.triplet)
			.then_with(|| cmp_max(&self.pair, &other.pair))
	}
}

impl PartialOrd for FullHouse {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for FullHouse {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for FullHouse {}

//

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
				_ => std::cmp::Ordering::Equal,
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
	use super::{Hand, HighCard, Straight, TwoPair};

	fn c(s: &str) -> super::Card {
		s.parse().unwrap()
	}

	#[test]
	#[allow(clippy::too_many_lines)]
	fn should_compare_hands() {
		let high_king_10_kicker = Hand::HighCard(HighCard {
			high_card: c("Kd"),
			kickers: vec![c("Tc")],
		});

		let high_king_6_kicker = Hand::HighCard(HighCard {
			high_card: c("Kc"),
			kickers: vec![c("6d")],
		});

		let two_pair_jack_sixes = Hand::TwoPair(TwoPair {
			high_pair: [c("Jd"), c("Jc")],
			low_pair: [c("6d"), c("6c")],
			kickers: vec![],
		});

		let two_pair_jack_fives = Hand::TwoPair(TwoPair {
			high_pair: [c("Jd"), c("Jc")],
			low_pair: [c("5d"), c("5c")],
			kickers: vec![],
		});

		let straight_no_ace = Hand::Straight(Straight {
			straight: [c("6h"), c("5d"), c("4c"), c("3d"), c("2s")],
		});

		let straight_ace_low = Hand::Straight(Straight {
			straight: [c("5d"), c("4c"), c("3d"), c("2h"), c("Ad")],
		});

		let straight_ace_high = Hand::Straight(Straight {
			straight: [c("Ad"), c("Kc"), c("Qh"), c("Jd"), c("Ts")],
		});

		let flush_king_high = Hand::Flush(super::Flush {
			flush: [c("Kd"), c("Qd"), c("8d"), c("7d"), c("4d")],
		});

		let flush_seven_high = Hand::Flush(super::Flush {
			flush: [c("7s"), c("6s"), c("5s"), c("3s"), c("2s")],
		});

		let full_house_ace_king = Hand::FullHouse(super::FullHouse {
			triplet: [c("Ad"), c("Ac"), c("Ah")],
			pair: [c("Kd"), c("Kc")],
		});

		let full_house_ace_6 = Hand::FullHouse(super::FullHouse {
			triplet: [c("Ad"), c("Ac"), c("Ah")],
			pair: [c("6d"), c("6c")],
		});

		let full_house_10_3 = Hand::FullHouse(super::FullHouse {
			triplet: [c("Td"), c("Tc"), c("Th")],
			pair: [c("3d"), c("3c")],
		});

		let mut hands = [
			&straight_ace_high,
			&two_pair_jack_sixes,
			&flush_seven_high,
			&full_house_ace_king,
			&straight_ace_low,
			&full_house_10_3,
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
			],
			hands,
		);
	}
}
