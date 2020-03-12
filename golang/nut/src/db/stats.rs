use crate::tx::TxStats;
use std::ops::{AddAssign, Sub};

/// Stats represents statistics about the database.
#[derive(Clone, Debug)]
pub struct Stats {
	// Freelist stats
	/// total number of free pages on the freelist
	pub free_page_n: usize,
	/// total number of pending pages on the freelist
	pub pending_page_n: usize,
	/// total bytes allocated in free pages
	pub free_alloc: usize,
	/// total bytes used by the freelist
	pub freelist_in_use: usize,

	// Transaction stats
	/// total number of started read transactions
	pub tx_n: usize,
	/// number of currently open read transactions
	pub open_tx_n: usize,

	/// global, ongoing stats.
	pub tx_stats: TxStats,
}

impl Default for Stats {
	fn default() -> Self {
		Self {
			free_page_n: 0,
			pending_page_n: 0,
			free_alloc: 0,
			freelist_in_use: 0,
			tx_n: 0,
			open_tx_n: 0,
			tx_stats: Default::default(),
		}
	}
}

impl Sub for Stats {
	type Output = Stats;

	fn sub(self, other: Stats) -> Stats {
		Stats {
			free_page_n: self.free_page_n,
			pending_page_n: self.pending_page_n,
			free_alloc: self.free_alloc,
			freelist_in_use: self.freelist_in_use,
			tx_n: self.tx_n - other.tx_n,
			open_tx_n: self.open_tx_n - other.open_tx_n,
			tx_stats: self.tx_stats.sub(&other.tx_stats),
		}
	}
}

impl AddAssign for Stats {
	fn add_assign(&mut self, other: Stats) {
		self.tx_stats += other.tx_stats;
	}
}
