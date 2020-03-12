#[cfg(test)]
pub mod page_data_tests;
#[cfg(test)]
pub mod tests;

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

mod branch_page_element;
mod leaf_page_element;
mod page_data;
mod page_info;

use crate::consts::{Flags, PGID};
use crate::meta::Meta;

pub(crate) use branch_page_element::BranchPageElement;
pub(crate) use leaf_page_element::LeafPageElement;
pub(crate) use page_data::PageData;
pub(crate) use page_info::PageInfo;

type PageSize = usize;
type PageSpan = u32;

#[repr(C)]
pub(crate) struct Page {
	pub id: PGID,
	/// page type
	pub flags: Flags,
	/// number of items in page
	pub count: u16,
	/// span of the page.
	/// 0 means that page allocated in one page_size block, 1 — two and so on
	pub overflow: PageSpan,
	/// pointer to data, not used as actual u8
	ptr: PhantomData<u8>,
}

impl fmt::Debug for Page {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Page")
			.field("id", &self.id)
			.field("flags", &self.flags)
			.field("count", &self.count)
			.field("overflow", &self.overflow)
			.field("ptr", &(&self.ptr as *const PhantomData<u8>))
			.finish()
	}
}

impl Page {
	pub(crate) fn header_size() -> usize {
		offset_of!(Page, ptr)
	}

	pub(crate) fn from_buf(buf: &[u8]) -> &Page {
		unsafe { &*(buf.as_ptr() as *const Page) }
	}

	pub(crate) fn from_buf_mut(buf: &mut [u8]) -> &mut Page {
		unsafe { &mut *(buf.as_mut_ptr() as *mut Page) }
	}

	/// Returns pointer to page structure
	#[inline]
	pub(crate) fn get_data_ptr(&self) -> *const u8 {
		&self.ptr as *const PhantomData<u8> as *const u8
	}

	/// Returns pointer to page structure
	#[inline]
	pub(crate) fn get_data_mut_ptr(&mut self) -> *mut u8 {
		&mut self.ptr as *mut PhantomData<u8> as *mut u8
	}

	pub(crate) fn data(&self) -> PageData {
		let count = self.count as usize;
		unsafe {
			match self.flags {
				Flags::BRANCHES => PageData::Branches(std::slice::from_raw_parts(
					self.get_data_ptr() as *const BranchPageElement,
					count,
				)),
				Flags::LEAVES => PageData::Leaves(std::slice::from_raw_parts(
					self.get_data_ptr() as *const LeafPageElement,
					count,
				)),
				Flags::META => PageData::Meta(&*(self.get_data_ptr() as *const Meta)),
				Flags::FREELIST => PageData::Freelist(std::slice::from_raw_parts(
					self.get_data_ptr() as *const PGID,
					count,
				)),
				_ => panic!("unknown flag"),
			}
		}
	}

	#[allow(dead_code)]
	pub(crate) fn data_mut(&mut self) -> PageData {
		let count = self.count as usize;
		unsafe {
			match self.flags {
				Flags::BRANCHES => PageData::Branches(std::slice::from_raw_parts(
					self.get_data_mut_ptr() as *mut BranchPageElement,
					count,
				)),
				Flags::LEAVES => PageData::Leaves(std::slice::from_raw_parts(
					self.get_data_mut_ptr() as *mut LeafPageElement,
					count,
				)),
				Flags::META => PageData::Meta(&*(self.get_data_mut_ptr() as *mut Meta)),
				Flags::FREELIST => PageData::Freelist(std::slice::from_raw_parts_mut(
					self.get_data_mut_ptr() as *mut PGID,
					count,
				)),
				_ => panic!("unknown flag"),
			}
		}
	}

	pub(crate) fn byte_size(&self) -> usize {
		use std::mem::size_of;
		let mut size = Self::header_size();
		match self.data() {
			PageData::Branches(b) => {
				let len = b.len();
				if len != 0 {
					size += (len - 1) * BranchPageElement::SIZE;
					let last = b.last().unwrap();
					size += (last.pos + last.ksize) as usize;
				}
			}
			PageData::Leaves(l) => {
				let len = l.len();
				if len != 0 {
					size += (len - 1) * LeafPageElement::SIZE;
					let last = l.last().unwrap();
					size += (last.pos + last.ksize + last.vsize) as usize;
				}
			}
			PageData::Meta(_) => {
				size += Meta::SIZE;
			}
			PageData::Freelist(f) => {
				size += f.len() * size_of::<PGID>();
			}
		}
		size
	}

	pub(crate) fn copy_data_from(&mut self, source: PageData) {
		let sbytes = source.as_bytes();
		match source {
			PageData::Freelist(ref c) => {
				self.count = c.len() as u16;
				self.flags = Flags::FREELIST;
			}
			PageData::Meta(_) => {
				self.count = 0;
				self.flags = Flags::META;
			}
			PageData::Leaves(ref c) => {
				self.count = c.len() as u16;
				self.flags = Flags::LEAVES;
			}
			PageData::Branches(ref c) => {
				self.count = c.len() as u16;
				self.flags = Flags::BRANCHES;
			}
		}
		unsafe {
			std::ptr::copy_nonoverlapping(sbytes.as_ptr(), self.get_data_mut_ptr(), sbytes.len())
		};
	}

	/// Retrieves a list of branch nodes
	pub(crate) fn branch_page_elements(&self) -> &[BranchPageElement] {
		match self.data() {
			PageData::Branches(l) => &l,
			_ => panic!("Flags are not branches"),
		}
	}

	/// Retrieves a list of branch nodes
	#[allow(dead_code)]
	pub(crate) fn branch_page_elements_mut(&mut self) -> &mut [BranchPageElement] {
		unsafe {
			&mut *(self.branch_page_elements() as *const [BranchPageElement]
				as *mut [BranchPageElement])
		}
	}

	/// Retrieves the branch node by index
	pub(crate) fn branch_page_element(&self, index: usize) -> &BranchPageElement {
		match self.data() {
			PageData::Branches(l) => &l[index],
			_ => panic!("Flags are not branches"),
		}
	}

	/// Retrieves the branch node by index
	pub(crate) fn branch_page_element_mut(&mut self, index: usize) -> &mut BranchPageElement {
		unsafe {
			&mut *(self.branch_page_element(index) as *const BranchPageElement as *mut BranchPageElement)
		}
	}

	/// Retrieves a list of leaf nodes
	pub(crate) fn leaf_page_elements(&self) -> &[LeafPageElement] {
		match self.data() {
			PageData::Leaves(l) => &l,
			_ => panic!("Flags are not leaves"),
		}
	}

	/// Retrieves a list of leaf nodes
	#[allow(dead_code)]
	pub(crate) fn leaf_page_elements_mut(&mut self) -> &mut [LeafPageElement] {
		unsafe {
			&mut *(self.leaf_page_elements() as *const [LeafPageElement] as *mut [LeafPageElement])
		}
	}

	/// Retrieves the leaf node by index
	pub(crate) fn leaf_page_element(&self, index: usize) -> &LeafPageElement {
		match self.data() {
			PageData::Leaves(l) => &l[index],
			_ => panic!("Flags are not leaves"),
		}
	}

	/// Retrieves the leaf node by index
	pub(crate) fn leaf_page_element_mut(&mut self, index: usize) -> &mut LeafPageElement {
		unsafe {
			&mut *(self.leaf_page_element(index) as *const LeafPageElement as *mut LeafPageElement)
		}
	}

	/// returns a pointer to the metadata section of the page.
	pub(crate) fn meta(&self) -> &Meta {
		match self.data() {
			PageData::Meta(meta) => meta,
			_ => panic!("Flags are not meta"),
		}
	}

	/// returns a pointer to the metadata section of the page.
	pub(crate) fn meta_mut(&mut self) -> &mut Meta {
		unsafe { &mut *(self.meta() as *const Meta as *mut Meta) }
	}

	/// meta returns a pointer to the freelist section of the page.
	pub(crate) fn freelist(&self) -> &[PGID] {
		match self.data() {
			PageData::Freelist(list) => list,
			_ => panic!("Flags are not freelist"),
		}
	}

	/// meta returns a pointer to the freelist section of the page.
	pub(crate) fn freelist_mut(&mut self) -> &mut [PGID] {
		unsafe { &mut *(self.freelist() as *const [PGID] as *mut [PGID]) }
	}
}

impl ToOwned for Page {
	type Owned = OwnedPage;

	fn to_owned(&self) -> OwnedPage {
		let ptr = self as *const Page as *const u8;
		unsafe {
			let slice = std::slice::from_raw_parts(ptr, self.byte_size()).to_owned();
			OwnedPage::from_vec(slice)
		}
	}
}

#[derive(Clone)]
#[repr(align(64))]
pub(crate) struct OwnedPage {
	page: Vec<u8>,
}

// unsafe impl Send for Page {}
// unsafe impl Sync for Page {}

impl OwnedPage {
	pub(crate) fn new(size: PageSize) -> Self {
		Self {
			page: vec![0u8; size],
		}
	}

	pub(crate) fn from_vec(buf: Vec<u8>) -> Self {
		Self { page: buf }
	}

	/// returns page size
	#[inline]
	pub(crate) fn size(&self) -> PageSize {
		self.page.len()
	}

	/// reservec capacity of underlying vector to size
	#[allow(dead_code)]
	pub(crate) fn reserve(&mut self, size: usize) {
		self.page.reserve(size)
	}

	/// Returns pointer to page structure
	#[inline]
	pub(crate) fn as_ptr(&self) -> *const u8 {
		self.page.as_ptr()
	}

	/// Returns pointer to page structure
	#[allow(dead_code)]
	#[inline]
	pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
		self.page.as_mut_ptr()
	}

	/// Returns binary serialized buffer of a page
	#[inline]
	pub(crate) fn buf(&self) -> &[u8] {
		&self.page
	}

	/// Returns binary serialized muttable buffer of a page
	#[inline]
	pub(crate) fn buf_mut(&mut self) -> &mut [u8] {
		&mut self.page
	}
}

impl Borrow<Page> for OwnedPage {
	#[inline]
	fn borrow(&self) -> &Page {
		unsafe { &*(self.page.as_ptr() as *const Page) }
	}
}

impl BorrowMut<Page> for OwnedPage {
	#[inline]
	fn borrow_mut(&mut self) -> &mut Page {
		unsafe { &mut *(self.page.as_mut_ptr() as *mut Page) }
	}
}

impl Deref for OwnedPage {
	type Target = Page;

	#[inline]
	fn deref(&self) -> &Page {
		self.borrow()
	}
}

impl DerefMut for OwnedPage {
	#[inline]
	fn deref_mut(&mut self) -> &mut Page {
		self.borrow_mut()
	}
}

impl fmt::Debug for OwnedPage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("OwnedPage")
			.field("size", &self.page.len())
			.field("page", &self as &Page)
			.finish()
	}
}

pub(crate) fn merge_pgids(a: &[PGID], b: &[PGID]) -> Vec<PGID> {
	let mut dst = Vec::with_capacity(a.len() + b.len());
	dst.extend(a);
	dst.extend(b);
	dst.sort();
	dst
}
