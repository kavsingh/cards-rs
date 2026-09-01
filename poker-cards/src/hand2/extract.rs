use thiserror::Error;

use super::{
	Flush, FourOfAKind, FullHouse, Hand, HandCandidate, HighCard, Pair,
	RoyalFlush, Straight, StraightFlush, ThreeOfAKind, TwoPair,
};
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
	#[error("four of a kind not found")]
	NoFourOfAKind,
	#[error("straight flush not found")]
	NoStraightFlush,
	#[error("royal flush not found")]
	NoRoyalFlush,
}

fn kickers_from(cards: &[Card], exclude: &[Card]) -> Vec<Card> {
	let max: usize = 5;

	without(exclude, cards)
		.iter()
		.take(max.saturating_sub(exclude.len()))
		.copied()
		.collect()
}

fn try_pair_from(sorted_cards: &[Card]) -> Result<[Card; 2], ExtractError> {
	match group_by(sorted_cards, |c| c.rank)
		.iter()
		.find(|(_, cards)| cards.len() >= 2)
		.map(|(_, p)| &p[..])
	{
		Some([a, b, ..]) => Ok([*a, *b]),
		_ => Err(ExtractError::NoPair),
	}
}

fn try_triplet_from(sorted_cards: &[Card]) -> Result<[Card; 3], ExtractError> {
	match group_by(sorted_cards, |c| c.rank)
		.iter()
		.find(|(_, cards)| cards.len() >= 3)
		.map(|(_, t)| &t[..])
	{
		Some([a, b, c, ..]) => Ok([*a, *b, *c]),
		_ => Err(ExtractError::NoThreeOfAKind),
	}
}

#[allow(clippy::many_single_char_names)]
fn try_straight_from(sorted_cards: &[Card]) -> Result<[Card; 5], ExtractError> {
	let candidate_straight: Vec<_> =
		chunk_by(sorted_cards, |a, b| a.rank_diff(b) == 1)
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

	let ace = sorted_cards
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

#[allow(clippy::many_single_char_names)]
fn try_flush_from(sorted_cards: &[Card]) -> Result<[Card; 5], ExtractError> {
	match group_by(sorted_cards, |c| c.suit)
		.iter()
		.find(|(_, cards)| cards.len() >= 5)
		.map(|(_, q)| &q[..])
	{
		Some([a, b, c, d, e, ..]) => Ok([*a, *b, *c, *d, *e]),
		_ => Err(ExtractError::NoFlush),
	}
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
			.filter(|(_, cards)| cards.len() >= 2)
			.collect::<Vec<_>>();

		let (high_pair, low_pair) = match (
			pairs.first().map(|(_, p)| &p[..]),
			pairs.get(1).map(|(_, p)| &p[..]),
		) {
			(Some([a, b, ..]), Some([c, d, ..])) => ([*a, *b], [*c, *d]),
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

impl TryFrom<&HandCandidate> for Straight {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		Ok(Self {
			straight: try_straight_from(&candidate.sorted_cards)?,
		})
	}
}

impl TryFrom<&HandCandidate> for Flush {
	type Error = ExtractError;

	#[allow(clippy::many_single_char_names)]
	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		Ok(Self {
			flush: try_flush_from(&candidate.sorted_cards)?,
		})
	}
}

impl TryFrom<&HandCandidate> for FullHouse {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let triplet = try_triplet_from(&candidate.sorted_cards)
			.map_err(|_| ExtractError::NoFullHouse)?;

		Ok(Self {
			triplet,
			pair: try_pair_from(&without(&triplet, &candidate.sorted_cards))
				.map_err(|_| ExtractError::NoFullHouse)?,
		})
	}
}

impl TryFrom<&HandCandidate> for FourOfAKind {
	type Error = ExtractError;

	#[allow(clippy::many_single_char_names)]
	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let grouped = group_by(&candidate.sorted_cards, |c| c.rank);
		let quadruplets = grouped.iter().find(|(_, cards)| cards.len() >= 4);

		let quad = match quadruplets.map(|(_, q)| &q[..]) {
			Some([a, b, c, d, ..]) => [*a, *b, *c, *d],
			_ => Err(ExtractError::NoFourOfAKind)?,
		};

		Ok(Self {
			quad,
			kickers: kickers_from(&candidate.sorted_cards, &quad),
		})
	}
}

impl TryFrom<&HandCandidate> for StraightFlush {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		let straight_flush = try_flush_from(&candidate.sorted_cards)
			.and_then(|cs| try_straight_from(&cs))
			.map_err(|_| ExtractError::NoStraightFlush)?;

		Ok(Self { straight_flush })
	}
}

impl TryFrom<&HandCandidate> for RoyalFlush {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		match StraightFlush::try_from(candidate) {
			Ok(StraightFlush { straight_flush })
				if straight_flush[0].rank == Rank::Ace =>
			{
				Ok(Self {
					royal_flush: straight_flush,
				})
			}
			_ => Err(ExtractError::NoRoyalFlush),
		}
	}
}

impl TryFrom<&HandCandidate> for Hand {
	type Error = ExtractError;

	fn try_from(candidate: &HandCandidate) -> Result<Self, Self::Error> {
		RoyalFlush::try_from(candidate)
			.map(Self::RoyalFlush)
			.or_else(|_| {
				StraightFlush::try_from(candidate).map(Self::StraightFlush)
			})
			.or_else(|_| {
				FourOfAKind::try_from(candidate).map(Self::FourOfAKind)
			})
			.or_else(|_| FullHouse::try_from(candidate).map(Self::FullHouse))
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
			assert_eq!(&hand.low_pair[0].to_string(), "Td");
			assert_eq!(&hand.low_pair[1].to_string(), "Th");
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

	#[test]
	fn should_extract_four_of_a_kind() {
		let pocket_cards = [
			Card::new(Rank::Jack, Suit::Diamonds),
			Card::new(Rank::Six, Suit::Diamonds),
		];
		let community_cards = vec![
			Card::new(Rank::Six, Suit::Hearts),
			Card::new(Rank::Six, Suit::Spades),
			Card::new(Rank::Queen, Suit::Clubs),
			Card::new(Rank::Eight, Suit::Spades),
			Card::new(Rank::Six, Suit::Clubs),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::FourOfAKind(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.quad[0].to_string(), "6d");
			assert_eq!(&hand.quad[1].to_string(), "6h");
			assert_eq!(&hand.quad[2].to_string(), "6s");
			assert_eq!(&hand.quad[3].to_string(), "6c");
			assert_eq!(&hand.kickers.first().unwrap().to_string(), "Qc");
		} else {
			panic!("Expected FourOfAKind hand");
		}
	}

	#[test]
	fn should_extract_straight_flush() {
		let pocket_cards = [
			Card::new(Rank::Two, Suit::Clubs),
			Card::new(Rank::Six, Suit::Clubs),
		];
		let community_cards = vec![
			Card::new(Rank::Four, Suit::Clubs),
			Card::new(Rank::Three, Suit::Clubs),
			Card::new(Rank::Eight, Suit::Spades),
			Card::new(Rank::Five, Suit::Clubs),
			Card::new(Rank::Six, Suit::Spades),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::StraightFlush(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.straight_flush[0].to_string(), "6c");
			assert_eq!(&hand.straight_flush[1].to_string(), "5c");
			assert_eq!(&hand.straight_flush[2].to_string(), "4c");
			assert_eq!(&hand.straight_flush[3].to_string(), "3c");
			assert_eq!(&hand.straight_flush[4].to_string(), "2c");
		} else {
			panic!("Expected StraightFlush hand");
		}
	}

	#[test]
	fn should_extract_straight_flush_ace_low() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Spades),
			Card::new(Rank::Eight, Suit::Hearts),
		];
		let community_cards = vec![
			Card::new(Rank::Four, Suit::Spades),
			Card::new(Rank::Two, Suit::Spades),
			Card::new(Rank::Three, Suit::Spades),
			Card::new(Rank::Five, Suit::Spades),
			Card::new(Rank::Eight, Suit::Diamonds),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::StraightFlush(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.straight_flush[0].to_string(), "5s");
			assert_eq!(&hand.straight_flush[1].to_string(), "4s");
			assert_eq!(&hand.straight_flush[2].to_string(), "3s");
			assert_eq!(&hand.straight_flush[3].to_string(), "2s");
			assert_eq!(&hand.straight_flush[4].to_string(), "As");
		} else {
			panic!("Expected StraightFlush hand");
		}
	}

	#[test]
	fn should_not_extract_straight_flush_offsuit_ace() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Clubs),
			Card::new(Rank::Two, Suit::Spades),
		];
		let community_cards = vec![
			Card::new(Rank::Three, Suit::Spades),
			Card::new(Rank::Four, Suit::Spades),
			Card::new(Rank::Five, Suit::Spades),
			Card::new(Rank::Eight, Suit::Spades),
		];

		if let Ok(Hand::Flush(hand)) =
			Hand::try_from(&HandCandidate::new(&pocket_cards, &community_cards))
		{
			assert_eq!(&hand.flush[0].to_string(), "8s");
			assert_eq!(&hand.flush[1].to_string(), "5s");
			assert_eq!(&hand.flush[2].to_string(), "4s");
			assert_eq!(&hand.flush[3].to_string(), "3s");
			assert_eq!(&hand.flush[4].to_string(), "2s");
		} else {
			panic!("Expected Flush hand");
		}
	}

	#[test]
	fn should_not_extract_straight_flush_ambiguous_ace() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Diamonds),
			Card::new(Rank::Five, Suit::Diamonds),
		];
		let community_cards = vec![
			Card::new(Rank::Nine, Suit::Diamonds),
			Card::new(Rank::Queen, Suit::Diamonds),
			Card::new(Rank::Seven, Suit::Diamonds),
			Card::new(Rank::King, Suit::Diamonds),
			Card::new(Rank::Jack, Suit::Diamonds),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::Flush(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.flush[0].to_string(), "Ad");
			assert_eq!(&hand.flush[1].to_string(), "Kd");
			assert_eq!(&hand.flush[2].to_string(), "Qd");
			assert_eq!(&hand.flush[3].to_string(), "Jd");
			assert_eq!(&hand.flush[4].to_string(), "9d");
		} else {
			panic!("Expected Flush hand");
		}
	}

	#[test]
	fn should_extract_royal_flush() {
		let pocket_cards = [
			Card::new(Rank::Ace, Suit::Hearts),
			Card::new(Rank::Jack, Suit::Hearts),
		];
		let community_cards = vec![
			Card::new(Rank::Ten, Suit::Hearts),
			Card::new(Rank::King, Suit::Hearts),
			Card::new(Rank::Queen, Suit::Hearts),
			Card::new(Rank::Three, Suit::Clubs),
			Card::new(Rank::Four, Suit::Diamonds),
		];

		let candidate = HandCandidate::new(&pocket_cards, &community_cards);

		if let Ok(Hand::RoyalFlush(hand)) = Hand::try_from(&candidate) {
			assert_eq!(&hand.royal_flush[0].to_string(), "Ah");
			assert_eq!(&hand.royal_flush[1].to_string(), "Kh");
			assert_eq!(&hand.royal_flush[2].to_string(), "Qh");
			assert_eq!(&hand.royal_flush[3].to_string(), "Jh");
			assert_eq!(&hand.royal_flush[4].to_string(), "Th");
		} else {
			panic!("Expected RoyalFlush hand");
		}
	}
}
