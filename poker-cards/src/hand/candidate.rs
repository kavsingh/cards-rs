use std::str::FromStr;

use crate::Card;

pub struct HandCandidate {
	pub(crate) sorted_cards: Vec<Card>,
}

impl HandCandidate {
	#[must_use]
	pub fn new(pocket_cards: &[Card; 2], community_cards: &[Card]) -> Self {
		let mut sorted_cards = pocket_cards.to_vec();
		let mut community = community_cards.to_owned();

		sorted_cards.append(&mut community);
		sorted_cards.sort_by(|a, b| b.cmp(a));

		Self { sorted_cards }
	}
}

impl FromStr for HandCandidate {
	type Err = &'static str;

	fn from_str(value: &'_ str) -> Result<Self, Self::Err> {
		let cards: Vec<&'_ str> = value.split_whitespace().collect();

		match &cards[..] {
			[p1, p2, community @ ..] => {
				let pocket_cards = [
					p1.parse().map_err(|_| "invalid card")?,
					p2.parse().map_err(|_| "invalid card")?,
				];

				let community_cards = community
					.iter()
					.map(|s| s.parse().map_err(|_| "invalid card"))
					.collect::<Result<Vec<_>, _>>()?;

				Ok(Self::new(&pocket_cards, &community_cards))
			}
			_ => Err("not enough cards"),
		}
	}
}

#[cfg(test)]
mod tests {
	use cards::Rank;

	use super::*;

	#[test]
	fn from_str() {
		let candidate: HandCandidate = "Ts As Qs Ks Js".parse().unwrap();

		assert_eq!(candidate.sorted_cards.len(), 5);
		assert_eq!(candidate.sorted_cards.first().unwrap().rank, Rank::Ace);
	}
}
