use crate::consts::PGID;

/// bucket represents the on-file representation of a bucket.
/// This is stored as the "value" of a bucket key. If the bucket is small enough,
/// then its root page can be stored inline in the "value", after the bucket
/// header. In the case of inline buckets, the "root" will be 0.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct IBucket {
	/// page id of the bucket's root-level page
	pub root: PGID,

	/// monotonically incrementing, used by next_sequence()
	pub sequence: u64,
}

impl IBucket {
	pub(crate) const SIZE: usize = std::mem::size_of::<Self>();

	pub(crate) fn new() -> IBucket {
		IBucket {
			root: 0,
			sequence: 0,
		}
	}
}
