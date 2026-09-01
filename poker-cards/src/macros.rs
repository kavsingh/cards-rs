#[macro_export]
macro_rules! derive_total_ord {
	( $( $t:ty ),+ $(,)? ) => {
		$(
			const _: () = {
				const fn assert_is_ord<T: std::cmp::Ord>() {}

				assert_is_ord::<$t>();
			};

			impl PartialOrd for $t {
				#[inline]
				fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
					Some(self.cmp(other))
				}
			}

			impl PartialEq for $t {
				#[inline]
				fn eq(&self, other: &Self) -> bool {
					self.cmp(other) == std::cmp::Ordering::Equal
				}
			}

			impl Eq for $t {}
		)+
	};
}
