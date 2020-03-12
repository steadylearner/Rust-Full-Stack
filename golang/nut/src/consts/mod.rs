use std::fmt;
use std::time::Duration;

// pub(crate) const MAX_ALLOC_SIZE: usize = 0xFFFFFFF;
pub(crate) const MAX_MAP_SIZE: u64 = 0x0FFF_FFFF;

/// Are unaligned load/stores broken on this arch?
// pub(crate) const BROKEN_UNALIGNED: bool = false;

pub(crate) const MIN_KEYS_PER_PAGE: usize = 2;

pub(crate) type PGID = u64;

pub(crate) type TXID = u64;

pub(crate) const MAX_MMAP_STEP: u64 = 1 << 30;

/// database mime header
pub(crate) const MAGIC: u32 = 0xED0C_DAED;

/// database version
pub(crate) const VERSION: u32 = 2;

// TODO: openbsd
pub(crate) const IGNORE_NOSYNC: bool = true;

pub(crate) const DEFAULT_MAX_BATCH_SIZE: usize = 1000;

pub(crate) const DEFAULT_MAX_BATCH_DELAY: Duration = Duration::from_millis(10000);
// pub(crate) const DEFAULT_ALLOC_SIZE: u64 = 16 * 1024 * 1024;

bitflags! {
	/// Defines type of the page
	pub struct Flags: u16 {
		/// Either branch or bucket page
		const BRANCHES = 0b00001;
		/// Leaf page
		const LEAVES = 0b00010;
		/// Meta page
		const META = 0b00100;
		/// Freelist page
		const FREELIST = 0b10000;
	}
}

impl fmt::Display for Flags {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let d = match *self {
			Flags::BRANCHES => "branches".to_string(),
			Flags::LEAVES => "leaves".to_string(),
			Flags::META => "meta".to_string(),
			Flags::FREELIST => "freelist".to_string(),
			_ => panic!("unknown flag"),
		};
		write!(f, "{}", d)
	}
}
