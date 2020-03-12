use crate::utils::is_aligned;
use std::mem::size_of;

/// LeafPageElement represents a node on a leaf page.
#[derive(Debug)]
#[repr(C)]
pub struct LeafPageElement {
	pub flags: u32,
	pub pos: u32,
	pub ksize: u32,
	pub vsize: u32,
}

impl LeafPageElement {
	/// binary struct's size (16)
	pub const SIZE: usize = size_of::<LeafPageElement>();

	/// key returns a byte slice of the node key.
	pub fn key(&self) -> &[u8] {
		debug_assert!(is_aligned(self));
		unsafe {
			let optr = self as *const Self as *const u8;
			let ptr = optr.add(self.pos as usize);
			std::slice::from_raw_parts(ptr, self.ksize as usize)
		}
	}

	/// value returns a byte slice of the node value.
	pub fn value(&self) -> &[u8] {
		debug_assert!(is_aligned(self));
		unsafe {
			std::slice::from_raw_parts(
				(self as *const Self as *const u8).add((self.pos + self.ksize) as usize),
				self.vsize as usize,
			)
		}
	}
}
