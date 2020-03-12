use std::ops::AddAssign;

#[derive(Default, Debug)]
/// Records statistics about resources used by a bucket.
pub struct BucketStats {
	// Page count statistics.
	pub branch_page_n: usize,     // number of logical branch pages
	pub branch_overflow_n: usize, // number of physical branch overflow pages
	pub leaf_page_n: usize,       // number of logical leaf pages
	pub leaf_overflow_n: usize,   // number of physical leaf overflow pages

	// Tree statistics.
	pub key_n: usize, // number of keys/value pairs
	pub depth: usize, // number of levels in B+tree

	// Page size utilization.
	pub branch_alloc: usize,  // bytes allocated for physical branch pages
	pub branch_in_use: usize, // bytes actually used for branch data
	pub leaf_alloc: usize,    // bytes allocated for physical leaf pages
	pub leaf_in_use: usize,   // bytes actually used for leaf data

	// Bucket statistics
	pub bucket_n: usize,        // total number of buckets including the top bucket
	pub inline_bucket_n: usize, // total number on inlined buckets
	pub inline_bucket_in_use: usize, // bytes used for inlined buckets (also accounted for in LeafInuse)
}

impl AddAssign for BucketStats {
	fn add_assign(&mut self, other: BucketStats) {
		self.branch_page_n += other.branch_page_n;
		self.branch_overflow_n += other.branch_overflow_n;
		self.leaf_page_n += other.leaf_page_n;
		self.leaf_overflow_n += other.leaf_overflow_n;
		self.key_n += other.key_n;
		if self.depth < other.depth {
			self.depth = other.depth;
		};
		self.branch_alloc += other.branch_alloc;
		self.branch_in_use += other.branch_in_use;
		self.leaf_alloc += other.leaf_alloc;
		self.leaf_in_use += other.leaf_in_use;

		self.bucket_n += other.bucket_n;
		self.inline_bucket_n += other.inline_bucket_n;
		self.inline_bucket_in_use += other.inline_bucket_in_use;
	}
}
