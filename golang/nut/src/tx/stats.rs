use std::ops::AddAssign;
use std::time::Duration;

/// Transaction statistics
#[derive(Clone, Debug, Default)]
pub struct TxStats {
	// Page statistics.
	/// number of page allocations
	pub page_count: usize,
	/// total bytes allocated
	pub page_alloc: usize,

	// Cursor statistics.
	/// number of cursors created
	pub cursor_count: usize,

	// Node statistics
	/// number of node allocations
	pub node_count: usize,
	/// number of node dereferences
	pub node_deref: usize,

	// Rebalance statistics.
	/// number of node rebalances
	pub rebalance: usize,
	/// total time spent rebalancing
	pub rebalance_time: Duration,

	// Split/Spill statistics.
	/// number of nodes split
	pub split: usize,
	/// number of nodes spilled
	pub spill: usize,
	/// total time spent spilling
	pub spill_time: Duration,

	// Write statistics.
	/// number of writes performed
	pub write: usize,
	/// total time spent writing to disk
	pub write_time: Duration,
}

impl TxStats {
	/// returns diff stats
	pub fn sub(&self, other: &TxStats) -> TxStats {
		TxStats {
			page_count: self.page_count - other.page_count,
			page_alloc: self.page_alloc - other.page_alloc,
			cursor_count: self.cursor_count - other.cursor_count,
			node_count: self.node_count - other.node_count,
			node_deref: self.node_deref - other.node_deref,
			rebalance: self.rebalance - other.rebalance,
			rebalance_time: self.rebalance_time - other.rebalance_time,
			split: self.split - other.split,
			spill: self.spill - other.spill,
			spill_time: self.spill_time - other.spill_time,
			write: self.write - other.write,
			write_time: self.write_time - other.write_time,
		}
	}
}

impl AddAssign for TxStats {
	fn add_assign(&mut self, other: TxStats) {
		self.page_count += other.page_count;
		self.page_alloc += other.page_alloc;
		self.cursor_count += other.cursor_count;
		self.node_count += other.node_count;
		self.node_deref += other.node_deref;
		self.rebalance += other.rebalance;
		self.rebalance_time += other.rebalance_time;
		self.split += other.split;
		self.spill += other.spill;
		self.spill_time += other.spill_time;
		self.write += other.write;
		self.write_time += other.write_time;
	}
}
