use crate::card::{Card, Rank, Suit};

pub enum DeckOrder {
	/// New-deck order (NDO) - see https://en.wikipedia.org/wiki/Standard_52-card_deck#New-deck_order_(NDO)
	NDO,
}

pub fn create_deck(order: Option<DeckOrder>) -> Vec<Card> {
	match order.unwrap_or(DeckOrder::NDO) {
		DeckOrder::NDO => create_ndo_deck(),
	}
}

const NDO_ORDERED_SUITS: [(Suit, bool); 4] = [
	(Suit::Spades, false),
	(Suit::Hearts, false),
	(Suit::Clubs, true),
	(Suit::Diamonds, true),
];

const NDO_RANK_ORDER: [Rank; 13] = [
	Rank::Ace,
	Rank::Two,
	Rank::Three,
	Rank::Four,
	Rank::Five,
	Rank::Six,
	Rank::Seven,
	Rank::Eight,
	Rank::Nine,
	Rank::Ten,
	Rank::Jack,
	Rank::Queen,
	Rank::King,
];

fn create_ndo_deck() -> Vec<Card> {
	NDO_ORDERED_SUITS
		.iter()
		.flat_map(|(suit, reverse)| {
			let mut ranks = NDO_RANK_ORDER;

			if *reverse {
				ranks.reverse()
			}

			ranks.map(|rank| Card::new(rank, *suit))
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::{DeckOrder, create_deck};

	#[test]
	fn should_create_ndo_deck() {
		let deck = create_deck(Some(DeckOrder::NDO));

		assert_eq!(deck.len(), 52);
		assert_eq!(format!("{}", deck[0]), "As".to_string());
		assert_eq!(format!("{}", deck[12]), "Ks".to_string());
		assert_eq!(format!("{}", deck[13]), "Ah".to_string());
		assert_eq!(format!("{}", deck[25]), "Kh".to_string());
		assert_eq!(format!("{}", deck[26]), "Kc".to_string());
		assert_eq!(format!("{}", deck[38]), "Ac".to_string());
		assert_eq!(format!("{}", deck[39]), "Kd".to_string());
		assert_eq!(format!("{}", deck[51]), "Ad".to_string());
	}

	#[test]
	fn should_create_ndo_deck_by_default() {
		let default_deck = create_deck(None);
		let specified_deck = create_deck(Some(DeckOrder::NDO));

		for (default_card, specified_card) in
			default_deck.iter().zip(specified_deck.iter())
		{
			assert_eq!(
				format!("{}", default_card),
				format!("{}", specified_card)
			);
		}
	}
}
