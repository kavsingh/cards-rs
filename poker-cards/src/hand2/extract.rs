use thiserror::Error;

use super::{
	Hand, HandCandidate, HighCard, Pair, Straight, ThreeOfAKind, TwoPair,
};
use crate::hand2::{Flush, FullHouse};
use crate::util::{chunk_by, group_by, without};
use crate::{Card, Rank};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExtractError {
	#[error("not enough cards to extract a hand")]
	NotEnoughCards,
	#[error("pair not found")]
	NoPair,
	#[error("two pair not found")]
	NoTwoPair,
	#[error("three of a kind not found")]
	NoThreeOfAKind,
	#[error("straight not found")]
	NoStraight,
	#[error("flush not found")]
	NoFlush,
	#[error("full house not found")]
	NoFullHouse,
}

fn kickers_from(cards: &[Card], exclude: &[Card]) -> Vec<Card> {
	let max: usize = 5;

	without(exclude, cards)
		.iter()
		.take(max.saturating_sub(exclude.len()))
		.copied()
		.collect()
}

impl TryFrom<&HandCandidate> for HighCard {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let high_card = candidate
			.sorted_cards
			.first()
			.ok_or(Self::Error::NotEnoughCards)?;
		let kicker_cards = candidate
			.sorted_cards
			.get(1..5)
			.unwrap_or_default()
			.to_vec();

		Ok(Self {
			high_card: high_card.to_owned(),
			kickers: kicker_cards,
		})
	}
}

fn try_pair_from(sorted_cards: &[Card]) -> Result<[Card; 2], ExtractError> {
	let grouped = group_by(sorted_cards, |c| c.rank);
	let pairs = grouped
		.iter()
		.filter(|(_, cards)| cards.len() >= 2)
		.collect::<Vec<_>>();

	match pairs.first().map(|(_, p)| &p[..]) {
		Some([a, b, ..]) => Ok([*a, *b]),
		_ => Err(ExtractError::NoPair),
	}
}

impl TryFrom<&HandCandidate> for Pair {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let pair = try_pair_from(&candidate.sorted_cards)?;

		Ok(Self {
			pair,
			kickers: kickers_from(&candidate.sorted_cards, &pair),
		})
	}
}

impl TryFrom<&HandCandidate> for TwoPair {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let grouped = group_by(&candidate.sorted_cards, |c| c.rank);
		let pairs = grouped
			.iter()
			.filter(|(_, cards)| cards.len() == 2)
			.collect::<Vec<_>>();

		let (high_pair, low_pair) = match (
			pairs.first().map(|(_, p)| &p[..]),
			pairs.get(1).map(|(_, p)| &p[..]),
		) {
			(Some([a, b]), Some([c, d])) => ([*a, *b], [*c, *d]),
			_ => return Err(Self::Error::NoTwoPair),
		};

		Ok(Self {
			high_pair,
			low_pair,
			kickers: kickers_from(
				&candidate.sorted_cards,
				&[high_pair, low_pair].concat(),
			),
		})
	}
}

fn try_triplet_from(sorted_cards: &[Card]) -> Result<[Card; 3], ExtractError> {
	let grouped = group_by(sorted_cards, |c| c.rank);
	let triplets = grouped
		.iter()
		.filter(|(_, cards)| cards.len() >= 3)
		.collect::<Vec<_>>();

	match triplets.first().map(|(_, t)| &t[..]) {
		Some([a, b, c, ..]) => Ok([*a, *b, *c]),
		_ => Err(ExtractError::NoThreeOfAKind),
	}
}

impl TryFrom<&HandCandidate> for ThreeOfAKind {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let triplet = try_triplet_from(&candidate.sorted_cards)?;

		Ok(Self {
			triplet,
			kickers: kickers_from(&candidate.sorted_cards, &triplet),
		})
	}
}

#[allow(clippy::many_single_char_names)]
fn try_straight_from(
	candidate: &HandCandidate,
) -> Result<[Card; 5], ExtractError> {
	let candidate_straight: Vec<_> =
		chunk_by(&candidate.sorted_cards, |a, b| a.rank_diff(b) == 1)
			.into_iter()
			.find(|chunk| chunk.len() >= 4)
			.ok_or(ExtractError::NoStraight)?;

	if let [a, b, c, d, e, ..] = &candidate_straight[..] {
		return Ok([*a, *b, *c, *d, *e]);
	}

	// if we have a 4 card candidate, check if we can use an ace as
	// the low card to complete the straight

	if let Some(lowest) = candidate_straight.last()
		&& lowest.rank != Rank::Two
	{
		return Err(ExtractError::NoStraight);
	}

	let ace = candidate
		.sorted_cards
		.iter()
		.find(|c| c.rank == Rank::Ace)
		.ok_or(ExtractError::NoStraight)?;

	let straight = candidate_straight
		.into_iter()
		.chain(std::iter::once(*ace))
		.collect::<Vec<_>>();

	match &straight[..] {
		[a, b, c, d, e] => Ok([*a, *b, *c, *d, *e]),
		_ => Err(ExtractError::NoStraight),
	}
}

impl TryFrom<&HandCandidate> for Straight {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		Ok(Self {
			straight: try_straight_from(candidate)?,
		})
	}
}

impl TryFrom<&HandCandidate> for Flush {
	type Error = ExtractError;

	#[allow(clippy::many_single_char_names)]
	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let flush: Vec<_> = group_by(&candidate.sorted_cards, |c| c.suit)
			.into_iter()
			.find(|(_, cards)| cards.len() >= 5)
			.map(|(_, cards)| cards.into_iter().take(5).collect())
			.ok_or(ExtractError::NoFlush)?;

		match &flush[..] {
			[a, b, c, d, e] => Ok(Self {
				flush: [*a, *b, *c, *d, *e],
			}),
			_ => Err(ExtractError::NoFlush),
		}
	}
}

impl TryFrom<&HandCandidate> for FullHouse {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let triplet = try_triplet_from(&candidate.sorted_cards)
			.map_err(|_| ExtractError::NoFullHouse)?;
		let remaining_cards = without(&triplet, &candidate.sorted_cards);
		let pair = try_pair_from(&remaining_cards)
			.map_err(|_| ExtractError::NoFullHouse)?;

		Ok(Self { triplet, pair })
	}
}

impl TryFrom<&HandCandidate> for Hand {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		FullHouse::try_from(candidate)
			.map(Self::FullHouse)
			.or_else(|_| Flush::try_from(candidate).map(Self::Flush))
			.or_else(|_| Straight::try_from(candidate).map(Self::Straight))
			.or_else(|_| {
				ThreeOfAKind::try_from(candidate).map(Self::ThreeOfAKind)
			})
			.or_else(|_| TwoPair::try_from(candidate).map(Self::TwoPair))
			.or_else(|_| Pair::try_from(candidate).map(Self::Pair))
			.or_else(|_| HighCard::try_from(candidate).map(Self::HighCard))
	}
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
	use super::{Hand, HandCandidate};
	use crate::{Card, Rank, Suit};

	#[test]
	fn should_extract_high_card() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Ten, Suit::Diamonds),
		];
		let community_cards = vec![
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Eight, Suit::Spades),
			Card::new(Rank::Four, Suit::Hearts),
			Card::new(Rank::Five, Suit::Diamonds),
			Card::new(Rank::Six, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::HighCard(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.high_card.to_string(), "Ah");
			assert_eq!(hand.kickers.len(), 4);
		} else {
			panic!("Expected HighCard hand");
		}
	}

	#[test]
	fn should_extract_pair() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Ten, Suit::Diamonds),
		];
		let community_cards = [
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Eight, Suit::Spades),
			Card::new(Rank::Four, Suit::Hearts),
			Card::new(Rank::Ace, Suit::Diamonds),
			Card::new(Rank::Six, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::Pair(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.pair[0].to_string(), "Ah");
			assert_eq!(hand.pair[1].to_string(), "Ad");
			assert_eq!(hand.kickers.len(), 3);
		} else {
			panic!("Expected Pair hand");
		}
	}

	#[test]
	fn should_extract_two_pair() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Ten, Suit::Diamonds),
		];
		let community_cards = [
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Eight, Suit::Spades),
			Card::new(Rank::Ten, Suit::Hearts),
			Card::new(Rank::Ace, Suit::Diamonds),
			Card::new(Rank::Six, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::TwoPair(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.high_pair[0].to_string(), "Ah");
			assert_eq!(&hand.high_pair[1].to_string(), "Ad");
			assert_eq!(&hand.low_pair[0].to_string(), "10d");
			assert_eq!(&hand.low_pair[1].to_string(), "10h");
			assert_eq!(hand.kickers.len(), 1);
		} else {
			panic!("Expected TwoPair hand");
		}
	}

	#[test]
	fn should_extract_three_of_a_kind() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Ten, Suit::Diamonds),
		];
		let community_cards = [
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Eight, Suit::Spades),
			Card::new(Rank::Five, Suit::Hearts),
			Card::new(Rank::Ace, Suit::Diamonds),
			Card::new(Rank::Ace, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::ThreeOfAKind(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.triplet[0].to_string(), "Ah");
			assert_eq!(&hand.triplet[1].to_string(), "Ad");
			assert_eq!(&hand.triplet[2].to_string(), "Ac");
			assert_eq!(hand.kickers.len(), 2);
		} else {
			panic!("Expected ThreeOfAKind hand");
		}
	}

	#[test]
	fn should_extract_straight() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Ten, Suit::Diamonds),
		];
		let community_cards = [
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Three, Suit::Spades),
			Card::new(Rank::Four, Suit::Hearts),
			Card::new(Rank::Five, Suit::Diamonds),
			Card::new(Rank::Six, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::Straight(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.straight[0].to_string(), "6c");
			assert_eq!(&hand.straight[1].to_string(), "5d");
			assert_eq!(&hand.straight[2].to_string(), "4h");
			assert_eq!(&hand.straight[3].to_string(), "3s");
			assert_eq!(&hand.straight[4].to_string(), "2c");
		} else {
			panic!("Expected Straight hand");
		}
	}

	#[test]
	fn should_extract_straight_with_ace_low() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Ten, Suit::Diamonds),
		];
		let community_cards = [
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Three, Suit::Spades),
			Card::new(Rank::Four, Suit::Hearts),
			Card::new(Rank::Five, Suit::Diamonds),
			Card::new(Rank::Seven, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::Straight(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.straight[0].to_string(), "5d");
			assert_eq!(&hand.straight[1].to_string(), "4h");
			assert_eq!(&hand.straight[2].to_string(), "3s");
			assert_eq!(&hand.straight[3].to_string(), "2c");
			assert_eq!(&hand.straight[4].to_string(), "Ah");
		} else {
			panic!("Expected Straight hand");
		}
	}

	#[test]
	fn should_extract_flush() {
		let pocket_cards = [
			Card::new(Rank::Three, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Diamonds),
		];
		let community_cards = [
			Card::new(Rank::Four, Suit::Diamonds),
			Card::new(Rank::Two, Suit::Diamonds),
			Card::new(Rank::Three, Suit::Spades),
			Card::new(Rank::Five, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Hearts),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::Flush(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.flush[0].to_string(), "Jd");
			assert_eq!(&hand.flush[1].to_string(), "5d");
			assert_eq!(&hand.flush[2].to_string(), "4d");
			assert_eq!(&hand.flush[3].to_string(), "3d");
			assert_eq!(&hand.flush[4].to_string(), "2d");
		} else {
			panic!("Expected Flush hand");
		}
	}

	#[test]
	fn should_extract_full_house() {
		let pocket_cards = [
			Card::new(Rank::Three, Suit::Spades),
			Card::new(Rank::Four, Suit::Diamonds),
		];
		let community_cards = vec![
			Card::new(Rank::Three, Suit::Clubs),
			Card::new(Rank::Three, Suit::Diamonds),
			Card::new(Rank::Five, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Hearts),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::FullHouse(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.triplet[0].to_string(), "3s");
			assert_eq!(&hand.triplet[1].to_string(), "3c");
			assert_eq!(&hand.triplet[2].to_string(), "3d");
			assert_eq!(&hand.pair[0].to_string(), "Jd");
			assert_eq!(&hand.pair[1].to_string(), "Jh");
		} else {
			panic!("Expected FullHouse hand");
		}
	}

	#[test]
	fn should_extract_full_house_with_highest_pair() {
		let pocket_cards = [
			Card::new(Rank::Four, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Hearts),
		];
		let community_cards = vec![
			Card::new(Rank::Four, Suit::Hearts),
			Card::new(Rank::Three, Suit::Clubs),
			Card::new(Rank::Three, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Spades),
			Card::new(Rank::Jack, Suit::Diamonds),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::FullHouse(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.triplet[0].to_string(), "Jh");
			assert_eq!(&hand.triplet[1].to_string(), "Js");
			assert_eq!(&hand.triplet[2].to_string(), "Jd");
			assert_eq!(&hand.pair[0].to_string(), "4d");
			assert_eq!(&hand.pair[1].to_string(), "4h");
		} else {
			panic!("Expected FullHouse hand");
		}
	}

	#[test]
	fn should_extract_full_house_with_highest_triplet() {
		let pocket_cards = [
			Card::new(Rank::Four, Suit::Diamonds),
			Card::new(Rank::Three, Suit::Clubs),
		];
		let community_cards = vec![
			Card::new(Rank::Three, Suit::Hearts),
			Card::new(Rank::Three, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Spades),
			Card::new(Rank::Jack, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Hearts),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::FullHouse(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.triplet[0].to_string(), "Js");
			assert_eq!(&hand.triplet[1].to_string(), "Jd");
			assert_eq!(&hand.triplet[2].to_string(), "Jh");
			assert_eq!(&hand.pair[0].to_string(), "3c");
			assert_eq!(&hand.pair[1].to_string(), "3h");
		} else {
			panic!("Expected FullHouse hand");
		}
	}
}
