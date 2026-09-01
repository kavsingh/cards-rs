use std::cmp::Ordering;

use crate::{Card, Rank};

pub fn without<TItem: Copy + PartialEq>(
	items: &[TItem],
	from_items: &[TItem],
) -> Vec<TItem> {
	from_items
		.iter()
		.filter(|item| !items.contains(item))
		.copied()
		.collect()
}

pub fn group_by<TItem: Copy, TKey: PartialEq>(
	items: &[TItem],
	key_fn: impl Fn(&TItem) -> TKey,
) -> Vec<(TKey, Vec<TItem>)> {
	let mut groups: Vec<(TKey, Vec<TItem>)> = vec![];

	for item in items {
		let key = key_fn(item);

		match groups.iter_mut().find(|(group_key, _)| *group_key == key) {
			Some((_, group_items)) => group_items.push(*item),
			None => groups.push((key, vec![*item])),
		}
	}

	groups
}

pub fn chunk_by<TItem: Copy>(
	items: &[TItem],
	predicate_fn: impl Fn(&TItem, &TItem) -> bool,
) -> Vec<Vec<TItem>> {
	let mut chunks: Vec<Vec<TItem>> = vec![];

	for item in items {
		if let Some(last_chunk) = chunks.last_mut() {
			match last_chunk.last() {
				Some(last_item) if predicate_fn(last_item, item) => {
					last_chunk.push(*item);
				}
				_ => chunks.push(vec![*item]),
			}
		} else {
			chunks.push(vec![*item]);
		}
	}

	chunks
}

pub fn get_n_by<TItem: Copy, V, const N: usize>(
	items: &[TItem],
	selector: impl Fn(&TItem) -> V,
) -> Option<[TItem; N]>
where
	V: PartialEq,
{
	group_by(items, selector)
		.into_iter()
		.find(|(_, ts)| ts.len() >= N)
		.and_then(|(_, ts)| ts.get(..N).and_then(|slice| slice.try_into().ok()))
}

pub fn cmp_max<T: Ord>(a_opt: &[T], b_opt: &[T]) -> Ordering {
	match (a_opt.iter().max(), b_opt.iter().max()) {
		(Some(a), Some(b)) => a.cmp(b),
		(None, None) => Ordering::Equal,
		(None, Some(_)) => Ordering::Less,
		(Some(_), None) => Ordering::Greater,
	}
}

pub fn is_ace_low_straight(cards: &[Card]) -> bool {
	let mut has_ace = false;
	let mut has_two = false;

	for card in cards {
		if card.rank == Rank::Two {
			has_two = true;
		}

		if card.rank == Rank::Ace {
			has_ace = true;
		}

		if has_ace && has_two {
			return true;
		}
	}

	false
}
