use either::Either;

use super::{Bucket, ElemRef};

/// Contains references to retrieved bucket's item key, value and flags
#[derive(Debug)]
pub struct CursorItem<'a> {
	pub key: Option<&'a [u8]>,
	pub value: Option<&'a [u8]>,
	pub flags: u32,
}

impl<'a> CursorItem<'a> {
	pub(crate) fn new(key: Option<&'a [u8]>, value: Option<&'a [u8]>, flags: u32) -> Self {
		Self { key, value, flags }
	}

	#[inline]
	pub(crate) fn new_null(key: Option<&'a [u8]>, value: Option<&'a [u8]>) -> Self {
		CursorItem::new(key, value, 0)
	}

	/// returns true if key and value are None
	pub fn is_none(&self) -> bool {
		self.key.is_none() && self.value.is_none()
	}

	/// returns true if key or value are Some
	#[inline]
	pub fn is_some(&self) -> bool {
		!self.is_none()
	}

	#[inline]
	pub fn is_bucket(&self) -> bool {
		self.flags & Bucket::FLAG != 0
	}

	/// unwraps item into key, value and flags
	pub fn unwrap(self) -> (Option<&'a [u8]>, Option<&'a [u8]>, u32) {
		(self.key, self.value, self.flags)
	}
}

impl<'a> From<&ElemRef> for CursorItem<'a> {
	fn from(el_ref: &ElemRef) -> Self {
		if (el_ref.count() == 0) || el_ref.index >= el_ref.count() {
			return Self::new(None, None, 0);
		}
		unsafe {
			match el_ref.upgrade() {
				Either::Left(ref p) => {
					let elem = p.leaf_page_element(el_ref.index);
					Self::new(
						Some(&*(elem.key() as *const [u8])),
						Some(&*(elem.value() as *const [u8])),
						elem.flags,
					)
				}
				Either::Right(ref n) => {
					let inode = &n.0.inodes.borrow()[el_ref.index];
					Self::new(
						Some(&*(inode.key.as_slice() as *const [u8])),
						Some(&*(inode.value.as_slice() as *const [u8])),
						inode.flags,
					)
				}
			}
		}
	}
}
