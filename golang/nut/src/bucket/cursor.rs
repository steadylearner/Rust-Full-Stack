use either::Either;
use std::cell::RefCell;
use std::ops::Deref;

use crate::bucket::Bucket;
use crate::consts::{Flags, PGID};
use crate::errors::Error;
use crate::node::{Node, WeakNode};
use crate::page::Page;

use super::CursorItem;
use super::ElemRef;

/// Cursor represents an iterator that can traverse over all key/value pairs in a bucket in sorted order.
/// Cursors see nested buckets with value == None.
/// Cursors can be obtained from a transaction and are valid as long as the transaction is open.
///
/// Keys and values returned from the cursor are only valid for the life of the transaction.
///
/// Changing data while traversing with a cursor may cause it to be invalidated
/// and return unexpected keys and/or values. You must reposition your cursor
/// after mutating data.
pub struct Cursor<'a, B: Deref<Target = Bucket> + 'a> {
	pub(crate) bucket: B,
	pub(crate) stack: RefCell<Vec<ElemRef>>,
	_m: std::marker::PhantomData<CursorItem<'a>>,
}

impl<'a, B: Deref<Target = Bucket> + 'a> Cursor<'a, B> {
	pub(crate) fn new(bucket: B) -> Self {
		Self {
			bucket,
			stack: RefCell::new(Vec::new()),
			_m: std::marker::PhantomData,
		}
	}

	/// Returns reference to bucket which is cursor created from
	pub(crate) fn bucket(&self) -> &Bucket {
		&*self.bucket
	}

	/// Returns mutable reference to bucket which is cursor created from
	pub(crate) fn bucket_mut(&mut self) -> &mut Bucket {
		unsafe { &mut *((&*self.bucket) as *const Bucket as *mut Bucket) }
	}

	/// Recursively performs a binary search against a given page/node until it finds a given key.
	fn search(&self, key: &[u8], pgid: PGID) -> Result<(), Error> {
		let page_node = self.bucket().page_node(pgid)?;
		if let Either::Left(ref p) = page_node.upgrade() {
			match p.flags {
				Flags::BRANCHES => (),
				Flags::LEAVES => (),
				_ => {
					panic!(format!("invalid page type: {}: {}", p.id, p.flags));
				}
			};
		}

		let elem_ref = ElemRef {
			el: page_node,
			index: 0,
		};
		self.stack.borrow_mut().push(elem_ref.clone());
		{
			if elem_ref.is_leaf() {
				self.nsearch(key)?;
				return Ok(());
			};

			match elem_ref.upgrade() {
				Either::Left(p) => self.search_page(key, p)?,
				Either::Right(n) => self.search_node(key, n)?,
			}
		}

		Ok(())
	}

	fn search_node(&self, key: &[u8], n: &Node) -> Result<(), Error> {
		let (exact, mut index) = match n
			.0
			.inodes
			.borrow()
			.binary_search_by(|inode| inode.key.as_slice().cmp(key))
		{
			Ok(mut v) => {
				let inodes = n.0.inodes.borrow();
				for (i, inode) in inodes.iter().enumerate().skip(v) {
					match inode.key.as_slice().cmp(key) {
						std::cmp::Ordering::Greater => break,
						std::cmp::Ordering::Less => break,
						std::cmp::Ordering::Equal => v = i,
					};
				}
				(true, v)
			}
			Err(v) => (false, v),
		};

		if !exact && index > 0 {
			index -= 1;
		};
		self
			.stack
			.borrow_mut()
			.last_mut()
			.ok_or_else(|| "stack empty")?
			.index = index;

		// Recursively search to the next page.
		let pgid = n.0.inodes.borrow()[index].pgid;
		self.search(key, pgid)?;

		Ok(())
	}

	fn search_page(&self, key: &[u8], p: &Page) -> Result<(), Error> {
		let inodes = p.branch_page_elements();
		let (exact, mut index) = match inodes.binary_search_by(|inode| inode.key().cmp(key)) {
			Ok(mut v) => {
				for (i, inode) in inodes.iter().enumerate().skip(v) {
					match inode.key().cmp(key) {
						std::cmp::Ordering::Greater => break,
						std::cmp::Ordering::Less => break,
						std::cmp::Ordering::Equal => v = i,
					};
				}
				(true, v)
			}
			Err(v) => (false, v),
		};
		if !exact && index > 0 {
			index -= 1;
		}
		self
			.stack
			.borrow_mut()
			.last_mut()
			.ok_or_else(|| "stack empty")?
			.index = index;

		// Recursively search to the next page.
		self.search(key, inodes[index].pgid)?;

		Ok(())
	}

	/// Searches the leaf node on the top of the stack for a key.
	fn nsearch(&self, key: &[u8]) -> Result<(), Error> {
		let mut stack = self.stack.borrow_mut();
		let el_ref = stack.last_mut().unwrap();
		if let Either::Right(ref n) = el_ref.upgrade() {
			let index = match n
				.0
				.inodes
				.borrow()
				.binary_search_by(|inode| inode.key.as_slice().cmp(key))
			{
				Ok(v) => v,
				Err(v) => v,
			};
			el_ref.index = index;
			return Ok(());
		}

		// If we have a page then search its leaf elements.
		let page = el_ref.el.upgrade().left().ok_or_else(|| "left empty")?;
		let inodes = page.leaf_page_elements();
		let index = match inodes.binary_search_by(|inode| inode.key().cmp(key)) {
			Ok(v) => v,
			Err(v) => v,
		};
		el_ref.index = index;

		Ok(())
	}

	/// Returns the key and value of the current leaf element.
	fn key_value(&self) -> Result<CursorItem<'a>, Error> {
		let stack = self.stack.borrow();
		let el_ref = stack.last().ok_or_else(|| "stack empty")?;
		Ok(CursorItem::from(el_ref))
	}

	/// Returns the node that the cursor is currently positioned on.
	pub(crate) fn node(&mut self) -> Result<Node, Error> {
		if self.stack.borrow().len() < 1 {
			return Err(Error::StackEmpty);
		};

		// If the top of the stack is a leaf node then just return it.
		{
			let stack = self.stack.borrow();
			let el = &stack.last().ok_or_else(|| "stack empty")?;
			if el.is_leaf() && el.el.is_right() {
				return Ok(el.el.as_ref().right().unwrap().clone());
			}
		}

		// Start from root and traverse down the hierarchy.
		let mut n = {
			let el_ref = self.stack.borrow()[0].clone();
			match el_ref.upgrade() {
				Either::Left(p) => {
					let id = p.id;
					self.bucket_mut().node(id, WeakNode::new())
				}
				Either::Right(n) => n.clone(),
			}
		};
		let stacklen = self.stack.borrow().len();
		for refi in &self.stack.borrow()[..stacklen - 1] {
			assert!(!n.is_leaf(), "expected branch");
			let child = n.child_at(refi.index).map_err(|_| Error::TraverseFailed)?;
			n = child;
		}
		assert!(n.is_leaf(), "expected leaf");
		Ok(n)
	}

	/// Moves the cursor to the first item in the bucket and returns its key and value.
	/// If the bucket is empty then a nil key and value are returned.
	/// The returned key and value are only valid for the life of the transaction.
	pub fn first(&self) -> Result<CursorItem<'a>, Error> {
		if self.bucket.tx()?.db().is_err() {
			return Err(Error::TxClosed);
		};
		{
			let mut stack = self.stack.borrow_mut();
			stack.clear();
			let el_ref = self.bucket.page_node(self.bucket.bucket.root)?;
			stack.push(ElemRef {
				el: el_ref,
				index: 0,
			});
		}

		self.first_leaf()?;

		let is_empty = {
			self
				.stack
				.borrow()
				.last()
				.ok_or_else(|| "stack empty")?
				.count()
				== 0
		};

		if is_empty {
			self.next_leaf()?;
		};

		let mut item = self.key_value()?;
		if (item.flags & Bucket::FLAG) != 0 {
			item.value = None;
		}
		Ok(item)
	}

	/// first moves the cursor to the first leaf element under the last page in the stack.
	pub(crate) fn first_leaf(&self) -> Result<(), Error> {
		let mut stack = self.stack.borrow_mut();
		loop {
			let el_ref = &stack.last().ok_or_else(|| "stack empty")?;
			if el_ref.is_leaf() {
				break;
			};

			let pgid = match el_ref.upgrade() {
				Either::Left(p) => p.branch_page_element(el_ref.index).pgid,
				Either::Right(n) => n.0.inodes.borrow()[el_ref.index].pgid,
			};
			let el_ref = self.bucket.page_node(pgid)?;
			stack.push(ElemRef {
				el: el_ref,
				index: 0,
			});
		}
		Ok(())
	}

	/// Moves the cursor to the last item in the bucket and returns its key and value.
	/// If the bucket is empty then a nil key and value are returned.
	/// The returned key and value are only valid for the life of the transaction.
	pub fn last(&self) -> Result<CursorItem<'a>, Error> {
		if !self.bucket.tx()?.opened() {
			return Err(Error::TxClosed);
		};
		{
			let mut stack = self.stack.borrow_mut();
			stack.clear();
			let el_ref = self.bucket.page_node(self.bucket.bucket.root)?;
			let mut el_ref = ElemRef {
				el: el_ref,
				index: 0,
			};
			el_ref.index = el_ref.count() - 1;
			stack.push(el_ref);
		}

		self.last_leaf()?;

		let mut item = self.key_value()?;
		if (item.flags & Bucket::FLAG) != 0 {
			item.value = None;
		}
		Ok(item)
	}

	/// Moves the cursor to the last leaf element under the last page in the stack.
	pub(crate) fn last_leaf(&self) -> Result<(), Error> {
		let mut stack = self.stack.borrow_mut();
		loop {
			let el_ref = stack.last().ok_or_else(|| "stack empty")?;
			if el_ref.is_leaf() {
				break;
			}

			let pgid = match el_ref.upgrade() {
				Either::Left(p) => p.branch_page_element(el_ref.index).pgid,
				Either::Right(n) => n.0.inodes.borrow()[el_ref.index].pgid,
			};

			let page_node = self.bucket.page_node(pgid)?;
			let mut next_ref = ElemRef {
				el: page_node,
				index: 0,
			};
			next_ref.index = next_ref.count() - 1;
			stack.push(next_ref)
		}

		Ok(())
	}

	/// Moves the cursor to the next item in the bucket and returns its key and value.
	/// If the cursor is at the end of the bucket then a nil key and value are returned.
	///
	/// The returned key and value are only valid for the life of the transaction.
	pub fn next(&self) -> Result<CursorItem<'a>, Error> {
		if !self.bucket.tx()?.opened() {
			return Err(Error::TxClosed);
		};
		let mut item = self.next_leaf()?;
		if (item.flags & Bucket::FLAG) != 0 {
			item.value = None;
		};
		Ok(item)
	}

	/// Moves to the next leaf element and returns the key and value.
	/// If the cursor is at the last leaf element then it stays there and returns nil.
	pub(crate) fn next_leaf(&self) -> Result<CursorItem<'a>, Error> {
		loop {
			let i = {
				let mut stack = self.stack.borrow_mut();
				let mut i = stack.len() as isize - 1;
				while i >= 0 {
					let elem = &mut stack[i as usize];
					if elem.index + 1 < elem.count() {
						elem.index += 1;
						break;
					}
					i -= 1;
				}
				i
			};

			if i == -1 {
				return Ok(CursorItem::new_null(None, None));
			}

			self.stack.borrow_mut().truncate(i as usize + 1);

			self.first_leaf()?;

			if self
				.stack
				.borrow()
				.last()
				.ok_or_else(|| "stack empty")?
				.count()
				== 0
			{
				continue;
			}

			return self.key_value();
		}
	}

	/// Moves the cursor to the previous item in the bucket and returns its key and value.
	/// If the cursor is at the beginning of the bucket then a nil key and value are returned.
	/// The returned key and value are only valid for the life of the transaction.
	pub fn prev(&self) -> Result<CursorItem<'a>, Error> {
		if !self.bucket.tx()?.opened() {
			return Err(Error::TxClosed);
		};
		{
			let mut stack = self.stack.borrow_mut();

			// If we've hit the end then return nil.
			if stack.len() == 0 {
				return Ok(CursorItem::new_null(None, None));
			}

			// Attempt to move back one element until we're successful.
			// Move up the stack as we hit the beginning of each page in our stack.
			for i in stack.len() - 1..=0 {
				let elem = &mut stack[i];
				if elem.index > 0 {
					elem.index -= 1;
					break;
				}
				stack.truncate(i);
			}

			// If we've hit the end then return nil.
			if stack.len() == 0 {
				return Ok(CursorItem::new_null(None, None));
			}
		}

		// Move down the stack to find the last element of the last leaf under this branch.
		self.last_leaf()?;
		let mut item = self.key_value()?;
		if (item.flags & Bucket::FLAG) != 0 {
			item.value = None;
		}
		Ok(item)
	}

	/// Moves the cursor to a given key and returns it.
	/// If the key does not exist then the next key is used. If no keys
	/// follow, a nil key is returned.
	///
	/// The returned key and value are only valid for the life of the transaction.
	pub fn seek(&self, seek: &[u8]) -> Result<CursorItem<'a>, Error> {
		let mut item = self.seek_to_item(seek)?;
		let el_ref = {
			let stack = self.stack.borrow();
			stack.last().ok_or_else(|| "stack empty")?.clone()
		};

		if el_ref.index >= el_ref.count() {
			item = self.next()?;
		}

		let mut item = item;

		if item.key.is_none() {
			return Ok(CursorItem::new_null(None, None));
		}
		if (item.flags & Bucket::FLAG) != 0 {
			item.value = None;
		}
		Ok(item)
	}

	/// Moves the cursor to a given key and returns it.
	/// If the key does not exist then the next key is used.
	pub(crate) fn seek_to_item(&self, seek: &[u8]) -> Result<CursorItem<'a>, Error> {
		if !self.bucket.tx()?.opened() {
			return Err(Error::TxClosed);
		};

		self.stack.borrow_mut().clear();
		self.search(seek, self.bucket.bucket.root)?;
		{
			let stack = self.stack.borrow();
			let el_ref = stack.last().ok_or_else(|| "stack empty")?;
			if el_ref.index >= el_ref.count() {
				return Ok(CursorItem::new_null(None, None));
			}
		}
		self.key_value()
	}

	/// Removes the current key/value under the cursor from the bucket.
	/// Delete fails if current key/value is a bucket or if the transaction is not writable.
	pub fn delete(&mut self) -> Result<(), Error> {
		if !self.bucket.tx()?.opened() {
			return Err(Error::TxClosed);
		};
		if !self.bucket.tx()?.writable() {
			return Err(Error::TxReadonly);
		};

		let key = {
			let item = self.key_value()?;
			// Return an error if current value is a bucket.
			if (item.flags & Bucket::FLAG) != 0 {
				return Err(Error::IncompatibleValue);
			}
			item.key.ok_or_else(|| "key empty")?.to_vec()
		};

		self.node()?.del(&key);

		Ok(())
	}
}
