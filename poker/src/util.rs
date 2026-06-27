use std::cmp::Ordering;

pub fn without<TItem: Copy + Eq>(
	items: &[TItem],
	from_items: &[TItem],
) -> Vec<TItem> {
	from_items
		.iter()
		.filter(|item| !items.contains(item))
		.copied()
		.collect()
}

pub fn group_by<TItem: Copy, TKey: Eq>(
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
					last_chunk.push(*item)
				}
				_ => chunks.push(vec![*item]),
			}
		} else {
			chunks.push(vec![*item]);
		}
	}

	chunks
}

pub fn cmp_max<T: Ord>(a_opt: &[T], b_opt: &[T]) -> Ordering {
	match (a_opt.iter().max(), b_opt.iter().max()) {
		(Some(a), Some(b)) => a.cmp(b),
		(None, None) => Ordering::Equal,
		(None, Some(_)) => Ordering::Less,
		(Some(_), None) => Ordering::Greater,
	}
}
