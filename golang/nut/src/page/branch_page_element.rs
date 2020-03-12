use crate::consts::PGID;
use crate::utils::is_aligned;
use std::mem::size_of;

/// BranchPageElement represents a node on a branch page.
#[derive(Debug)]
#[repr(C)]
pub struct BranchPageElement {
	pub pos: u32,
	pub ksize: u32,
	pub pgid: PGID,
}

impl<'a> BranchPageElement {
	/// binary struct's size (16)
	pub const SIZE: usize = size_of::<BranchPageElement>();

	/// key returns a byte slice of the node key.
	pub fn key(&self) -> &[u8] {
		debug_assert!(is_aligned(self));
		unsafe {
			std::slice::from_raw_parts(
				(self as *const Self as *const u8).add(self.pos as usize),
				self.ksize as usize,
			)
		}
	}
}
