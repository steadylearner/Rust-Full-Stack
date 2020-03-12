use either::Either;
use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};
use std::fmt;
use std::{isize, usize};

use crate::consts::{Flags, PGID};
use crate::db::DB;
use crate::errors::Error;
use crate::node::{Node, NodeBuilder, WeakNode};
use crate::page::{BranchPageElement, LeafPageElement, OwnedPage, Page};
use crate::tx::{Tx, WeakTx};

use super::consts::*;
use super::BucketStats;
use super::Cursor;
use super::IBucket;
use super::PageNode;

/// Bucket represents a collection of key/value pairs inside the database.
pub struct Bucket {
	/// ref to on-file representation of a bucket
	pub(crate) bucket: IBucket,

	/// the associated transaction
	pub(crate) tx: WeakTx,

	/// subbucket cache
	buckets: RefCell<HashMap<Vec<u8>, Bucket>>,

	/// inline page reference
	page: Option<OwnedPage>,

	/// materialized node for the root page
	root_node: Option<Node>,

	/// node cache
	pub(crate) nodes: RefCell<HashMap<PGID, Node>>,

	/// Sets the threshold for filling nodes when they split. By default,
	/// the bucket will fill to 50% but it can be useful to increase this
	/// amount if you know that your write workloads are mostly append-only.
	///
	/// This is non-persisted across transactions so it must be set in every Tx.
	pub(crate) fill_percent: f64,
}

impl fmt::Debug for Bucket {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let tx = self.tx().ok().map(|tx| &tx as *const Tx);

		f.debug_struct("Bucket")
			.field("bucket", &self.bucket)
			.field("tx", &tx)
			.field("buckets", &self.buckets)
			.field("page", &self.page.as_ref())
			.field("root_node", &self.root_node)
			.field("nodes", &*self.nodes.borrow())
			.field("fill_percent", &self.fill_percent)
			.finish()
	}
}

impl Bucket {
	pub(crate) const MIN_FILL_PERCENT: f64 = 0.1;
	pub(crate) const MAX_FILL_PERCENT: f64 = 1.0;

	pub(crate) const DEFAULT_FILL_PERCENT: f64 = 0.5;

	pub(crate) const FLAG: u32 = 0x01;

	pub(crate) fn new(tx: WeakTx) -> Self {
		Self {
			bucket: IBucket::new(),
			tx,
			buckets: RefCell::new(HashMap::new()),
			page: None,
			root_node: None,
			nodes: RefCell::new(HashMap::new()),
			fill_percent: Self::DEFAULT_FILL_PERCENT,
		}
	}

	// Returns transaction
	pub fn tx(&self) -> Result<Tx, Error> {
		self.tx.upgrade().ok_or_else(|| Error::TxGone)
	}

	// Returns database
	pub fn db(&self) -> Result<DB, Error> {
		self.tx()?.db()
	}

	// Returns the root of the bucket
	pub fn root(&self) -> PGID {
		self.bucket.root
	}

	fn root_node(&self) -> Option<Node> {
		self.root_node.clone()
	}

	/// Creates a cursor associated with the bucket.
	pub fn cursor(&self) -> Result<Cursor<&Bucket>, Error> {
		self.tx()?.0.stats.lock().cursor_count += 1;

		Ok(Cursor::new(self))
	}

	fn __bucket(&self, name: &[u8]) -> Option<*mut Bucket> {
		if let Some(b) = self.buckets.borrow_mut().get_mut(name) {
			return Some(b);
		};
		let (key, value) = {
			let c = self.cursor().unwrap();
			let (key, value, flags) = c.seek_to_item(name).unwrap().unwrap();

			// Return None if the key doesn't exist or it is not a bucket.
			if key != Some(name) || (flags & Self::FLAG) == 0 {
				return None;
			};

			(key.map(|k| k.to_vec()), value.map(|v| v.to_vec()))
		};

		// Otherwise create a bucket and cache it.
		let child = self.open_bucket(value.unwrap());

		let mut buckets = self.buckets.borrow_mut();
		let bucket = match buckets.entry(key.unwrap()) {
			Entry::Vacant(e) => e.insert(child),
			Entry::Occupied(e) => {
				let c = e.into_mut();
				*c = child;
				c
			}
		};
		Some(bucket)
	}

	/// Retrieves a nested bucket by name.
	/// Returns None if the bucket does not exist or found item is not bucket.
	pub fn bucket(&self, key: &[u8]) -> Option<&Bucket> {
		self.__bucket(key).map(|b| unsafe { &*b })
	}

	/// Retrieves a nested mutable bucket by name.
	/// Returns None if the bucket does not exist or found item is not bucket.
	pub fn bucket_mut(&mut self, key: &[u8]) -> Option<&mut Bucket> {
		if !self.tx().unwrap().writable() {
			return None;
		};
		self.__bucket(key).map(|b| unsafe { &mut *b })
	}

	/// Helper method that re-interprets a sub-bucket value
	/// from a parent into a Bucket
	///
	/// value is bytes serialized bucket
	pub(crate) fn open_bucket(&self, value: Vec<u8>) -> Bucket {
		let mut child = Bucket::new(self.tx.clone());
		// let value = unsafe { value.as_ref().unwrap() };

		// TODO: !!!
		// If unaligned load/stores are broken on this arch and value is
		// unaligned simply clone to an aligned byte array.
		// let unaligned = BROKEN_UNALIGNED && &value[0] uintptr(unsafe.Pointer(&value[0]))&3 != 0
		// let unaligned = (|| -> bool { unimplemented!() })();
		// if unaligned {
		// 	// value = cloneBytes(value)
		// 	mutvalue = (|| -> &mut [u8] { unimplemented!() })();
		// }

		// If this is a writable transaction then we need to copy the bucket entry.
		// Read-only transactions can point directly at the mmap entry.
		{
			// TODO:
			// if self.tx().unwrap().writable() {
			let b = unsafe { (&*(value.as_ptr() as *const IBucket)).clone() };
			child.bucket = b;
		}

		// Save a reference to the inline page if the bucket is inline.
		if child.bucket.root == 0 {
			let data = unsafe {
				let slice = &value[IBucket::SIZE..];
				let mut vec = vec![0u8; slice.len()];
				std::ptr::copy_nonoverlapping(slice.as_ptr(), vec.as_mut_ptr(), slice.len());
				vec
			};

			let p = OwnedPage::from_vec(data);
			child.page = Some(p);
		}

		child
	}

	pub(crate) fn clear(&mut self) {
		self.buckets.borrow_mut().clear();
		self.nodes.borrow_mut().clear();
		self.page = None;
		self.root_node = None;
	}

	/// Creates bucket
	pub fn create_bucket(&mut self, key: &[u8]) -> Result<&mut Bucket, Error> {
		{
			let tx = self.tx()?;
			if !tx.opened() {
				return Err(Error::TxClosed);
			}
			if !tx.writable() {
				return Err(Error::DatabaseReadonly);
			}
			if key.is_empty() {
				return Err(Error::NameRequired);
			}
		}

		let tx_clone = self.tx.clone();

		{
			let mut cursor = self.cursor()?;
			let (ckey, flags) = {
				let (key, _, flags) = cursor.seek_to_item(key)?.unwrap();
				(key, flags)
			};

			if ckey == Some(key) {
				if (flags & Self::FLAG) != 0 {
					return Err(Error::BucketExists);
				};
				return Err(Error::IncompatibleValue);
			};

			let mut bucket = Bucket::new(tx_clone);
			bucket.root_node = Some(NodeBuilder::new(&bucket).is_leaf(true).build());
			bucket.fill_percent = Self::DEFAULT_FILL_PERCENT;

			let value = bucket.write();
			cursor.node().unwrap().put(key, key, value, 0, Self::FLAG);
			self.page = None;
		}

		self
			.bucket_mut(key)
			.ok_or_else(|| Error::Unexpected("cannot find bucket".to_string()))
	}

	/// Creates bucket if it not exists
	pub fn create_bucket_if_not_exists(&mut self, key: &[u8]) -> Result<&mut Bucket, Error> {
		match unsafe { &mut *(self as *mut Self) }.create_bucket(key) {
			Ok(b) => Ok(b),
			Err(Error::BucketExists) => self
				.bucket_mut(key)
				.ok_or_else(|| Error::Unexpected("can't find bucket".to_string())),
			v => v,
		}
	}

	/// Removes bucket and its contents
	///
	/// Returns error if bucket not found or value is not bucket
	pub fn delete_bucket(&mut self, key: &[u8]) -> Result<(), Error> {
		{
			let tx = self.tx()?;
			if !tx.opened() {
				return Err(Error::DatabaseClosed);
			}
			if !tx.writable() {
				return Err(Error::DatabaseReadonly);
			}
			if key.is_empty() {
				return Err(Error::NameRequired);
			}
		};

		let mut c = self.cursor()?;
		{
			let item = c.seek(key)?;
			if item.key.as_ref().map(|v| &**v).unwrap() != key {
				return Err(Error::BucketNotFound);
			}
			if !item.is_bucket() {
				return Err(Error::IncompatibleValue);
			}
		}
		let mut node = c.node()?;
		{
			let child = self.bucket_mut(key).ok_or_else(|| "Can't get bucket")?;
			let child_buckets = child.buckets();

			for bucket in &child_buckets {
				child.delete_bucket(bucket)?;
			}

			// Release all bucket pages to freelist.
			child.nodes.borrow_mut().clear();
			child.root_node = None;
			child.free();
		}

		self.buckets.borrow_mut().remove(key);
		node.del(key);

		Ok(())
	}

	/// Returns list of subbuckets' keys
	pub fn buckets(&self) -> Vec<Vec<u8>> {
		let mut names = vec![];
		self
			.for_each(Box::new(|k, v| -> Result<(), Error> {
				if v.is_none() {
					names.push(k.to_vec());
				}
				Ok(())
			}))
			.unwrap();
		names
	}

	/// Retrieves the value for a key in the bucket.
	/// Returns a None value if the key does not exist or if the key is a nested bucket.
	///
	/// # Example
	///
	/// ```no_run
	/// use nut::DBBuilder;
	///
	/// let db = DBBuilder::new("./test.db").build().unwrap();
	/// let tx = db.begin_tx().unwrap();
	/// let flowers = tx.bucket(b"flowers").unwrap();
	///
	/// assert_eq!(flowers.get(b"irises").unwrap(), &b"Iris is a genus of species of flowering plants"[..]);
	/// ```
	pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
		let (ckey, value, flags) = self.cursor().unwrap().seek(key).unwrap().unwrap();
		// let value = value.map(|v| v.to_vec());

		// Return nil if this is a bucket.
		if (flags & Self::FLAG) != 0 {
			return None;
		}

		// If our target node isn't the same key as what's passed in then return nil.
		if ckey != Some(key) {
			return None;
		}

		value
	}

	/// Sets the value for a key in the bucket.
	/// If the key exist then its previous value will be overwritten.
	/// Returns an error if the bucket was created from a read-only transaction, if the key is blank, if the key is too large, or if the value is too large.
	///
	/// # Example
	///
	/// ```no_run
	/// use nut::DBBuilder;
	///
	/// let mut db = DBBuilder::new("./test.db").build().unwrap();
	/// let mut tx = db.begin_rw_tx().unwrap();
	/// let mut flowers = tx.create_bucket(b"flowers").unwrap();
	///
	/// flowers.put(b"irises", b"Iris is a genus of species of flowering plants".to_vec()).unwrap()
	/// ```
	pub fn put(&mut self, key: &[u8], value: Vec<u8>) -> Result<(), Error> {
		if !self.tx()?.opened() {
			return Err(Error::TxClosed);
		}
		if !self.tx()?.writable() {
			return Err(Error::TxReadonly);
		}
		if key.is_empty() {
			return Err(Error::KeyRequired);
		}
		if key.len() > MAX_KEY_SIZE {
			return Err(Error::KeyTooLarge);
		}
		if value.len() > MAX_VALUE_SIZE {
			return Err(Error::ValueTooLarge);
		}

		// Move cursor to correct position.
		let mut c = self.cursor()?;
		let item = c.seek(key)?;

		// Return an error if there is an existing key with a bucket value.
		if (Some(key) == item.key) && item.is_bucket() {
			return Err(Error::IncompatibleValue);
		}

		// Insert into node.
		c.node().unwrap().put(key, key, value, 0, 0);

		Ok(())
	}

	/// Removes a key from the bucket.
	/// If the key does not exist then nothing is done.
	/// Returns an error if the bucket was created from a read-only transaction.
	///
	/// # Example
	///
	/// ```no_run
	/// use nut::DBBuilder;
	///
	/// let mut db = DBBuilder::new("./test.db").build().unwrap();
	/// let mut tx = db.begin_rw_tx().unwrap();
	/// let mut flowers = tx.bucket_mut(b"flowers").unwrap();
	///
	/// flowers.delete(b"irises").unwrap()
	/// ```
	pub fn delete(&mut self, key: &[u8]) -> Result<(), Error> {
		if self.tx()?.db().is_err() {
			return Err(Error::TxClosed);
		}
		if !self.tx()?.writable() {
			return Err(Error::TxReadonly);
		}

		// Move cursor to correct position.
		let mut c = self.cursor()?;
		let item = c.seek(key)?;

		// Return an error if there is already existing bucket value.
		if item.is_bucket() {
			return Err(Error::IncompatibleValue);
		};

		// Delete the node if we have a matching key.
		c.node().unwrap().del(key);

		Ok(())
	}

	/// Returns the current integer for the bucket without incrementing it.
	pub fn sequence(&self) -> u64 {
		self.bucket.sequence
	}

	/// Returns an autoincrementing integer for the bucket.
	pub fn next_sequence(&mut self) -> Result<u64, Error> {
		if !self.tx()?.writable() {
			return Err(Error::TxReadonly);
		}

		if self.root_node.is_none() {
			self.node(self.root(), WeakNode::new());
		}

		self.bucket.sequence += 1;

		Ok(self.bucket.sequence)
	}

	/// Updates the sequence number for the bucket.
	pub fn set_sequence(&mut self, value: u64) -> Result<(), Error> {
		if !self.tx()?.writable() {
			return Err(Error::TxReadonly);
		};

		if self.root_node.is_none() {
			let pgid = self.root();
			self.node(pgid, WeakNode::new());
		};

		self.bucket.sequence = value;
		Ok(())
	}

	/// Executes a function for each key/value pair in a bucket.
	/// If the provided function returns an error then the iteration is stopped and
	/// the error is returned to the caller.
	pub fn for_each<'a, E: Into<Error>>(
		&self,
		mut handler: Box<dyn FnMut(&[u8], Option<&[u8]>) -> Result<(), E> + 'a>,
	) -> Result<(), Error> {
		if !self.tx()?.opened() {
			return Err(Error::TxClosed);
		}
		let c = self.cursor()?;
		let mut item = c.first()?;
		loop {
			if item.is_none() {
				break;
			};
			handler(item.key.unwrap(), item.value).map_err(|e| e.into())?;
			item = c.next()?;
		}
		Ok(())
	}

	/// Returns stats on a bucket.
	pub fn stats(&self) -> BucketStats {
		let mut stats = BucketStats::default();
		let mut sub_stats = BucketStats::default();
		let page_size = self.tx().unwrap().db().unwrap().page_size();
		stats.bucket_n += 1;
		if self.bucket.root == 0 {
			stats.inline_bucket_n += 1;
		};
		self.for_each_page(Box::new(|p, depth| {
			let page_count = p.count as usize;
			if p.flags == Flags::LEAVES {
				stats.key_n += page_count;
				let mut used = Page::header_size();
				if page_count != 0 {
					// If page has any elements, add all element headers.
					used += LeafPageElement::SIZE * (page_count - 1);
					// Add all element key, value sizes.
					// The computation takes advantage of the fact that the position
					// of the last element's key/value equals to the total of the sizes
					// of all previous elements' keys and values.
					// It also includes the last element's header.
					let last_element = p.leaf_page_element(page_count - 1);
					used += (last_element.pos + last_element.ksize + last_element.vsize) as usize;

					if self.bucket.root == 0 {
						// For inlined bucket just update the inline stats
						stats.inline_bucket_in_use += used;
					} else {
						// For non-inlined bucket update all the leaf stats
						stats.leaf_page_n += 1;
						stats.leaf_in_use += used;
						stats.leaf_overflow_n += p.overflow as usize;

						// Collect stats from sub-buckets.
						// Do that by iterating over all element headers
						// looking for the ones with the bucketLeafFlag.
						for i in 0..page_count {
							let e = p.leaf_page_element(i);
							if (e.flags & Self::FLAG) != 0 {
								// For any bucket element, open the element value
								// and recursively call Stats on the contained bucket.
								sub_stats += self.open_bucket(e.value().to_vec()).stats();
							};
						}
					}
				} else if p.flags == Flags::BRANCHES {
					stats.branch_page_n += 1;
					let last_element = p.branch_page_element(page_count - 1);

					// used totals the used bytes for the page
					// Add header and all element headers.
					let mut used = Page::header_size() + (BranchPageElement::SIZE * (page_count - 1));

					// Add size of all keys and values.
					// Again, use the fact that last element's position equals to
					// the total of key, value sizes of all previous elements.
					used += (last_element.pos + last_element.ksize) as usize;
					stats.branch_in_use += used;
					stats.branch_overflow_n += p.overflow as usize;
				};
			};

			// Keep track of maximum page depth.
			if depth + 1 > stats.depth {
				stats.depth = depth + 1;
			}
		}));

		// Alloc stats can be computed from page counts and pageSize.
		stats.branch_alloc = (stats.branch_page_n + stats.branch_overflow_n) * page_size;
		stats.leaf_alloc = (stats.leaf_page_n + stats.leaf_overflow_n) * page_size;

		// Add the max depth of sub-buckets to get total nested depth.
		stats.depth += sub_stats.depth;

		// Add the stats for all sub-buckets
		stats += sub_stats;
		stats
	}

	/// Iterates over every page in a bucket, including inline pages.
	fn for_each_page<'a>(&self, mut handler: Box<dyn FnMut(&Page, usize) + 'a>) {
		// If we have an inline page then just use that.
		if let Some(ref page) = self.page {
			handler(&page, 0);
			return;
		}

		// Otherwise traverse the page hierarchy.
		self
			.tx()
			.unwrap()
			.for_each_page(self.bucket.root, 0, handler);
	}

	/// Iterates over every page (or node) in a bucket.
	/// This also includes inline pages.
	fn for_each_page_node<F>(&self, mut handler: F)
	where
		F: FnMut(Either<&Page, &Node>, isize),
	{
		// If we have an inline page then just use that.
		if let Some(ref page) = self.page {
			handler(PageNode::from(&**page as *const Page).upgrade(), 0);
			return;
		}
		self.__for_each_page_node(self.bucket.root, 0, &mut handler)
	}

	fn __for_each_page_node<F>(&self, pgid: PGID, depth: isize, handler: &mut F)
	where
		F: FnMut(Either<&Page, &Node>, isize),
	{
		let item = self.page_node(pgid).unwrap();

		// Execute function.
		handler(item.upgrade(), depth);

		// Recursively loop over children.
		match item.upgrade() {
			Either::Left(p) => {
				let is_branch = match p.flags {
					Flags::BRANCHES => true,
					_ => false,
				};
				if is_branch {
					for i in 0..p.count as usize {
						let elem = p.branch_page_element(i);
						self.__for_each_page_node(elem.pgid, depth + 1, handler);
					}
				}
			}
			Either::Right(n) => {
				if !n.is_leaf() {
					for inode in &*n.0.inodes.borrow() {
						self.__for_each_page_node(inode.pgid, depth + 1, handler)
					}
				}
			}
		}
	}

	/// Writes all the nodes for this bucket to dirty pages.
	pub(crate) fn spill(&mut self) -> Result<(), Error> {
		let mutself = unsafe { &mut *(self as *mut Self) };

		// Spill all child buckets first.
		for (name, child) in &mut *self.buckets.borrow_mut() {
			// If the child bucket is small enough and it has no child buckets then
			// write it inline into the parent bucket's page. Otherwise spill it
			// like a normal bucket and make the parent value a pointer to the page.
			let value = if child.inlineable() {
				child.free();
				child.write()
			} else {
				child.spill()?;

				// Update the child bucket header in this bucket.
				let mut vec = vec![0u8; IBucket::SIZE];
				let bucket_ptr = vec.as_mut_ptr() as *mut IBucket;
				unsafe { std::ptr::copy_nonoverlapping(&child.bucket, bucket_ptr, 1) };
				vec
			};

			// Skip writing the bucket if there are no materialized nodes.
			if child.root_node.is_none() {
				continue;
			};

			// Update parent node.
			let mut c = mutself.cursor()?;
			let item = c.seek(&name)?;
			if item.key != Some(name.as_slice()) {
				return Err(
					format!(
						"misplaced bucket header: {:?} -> {:?}",
						name,
						item.key.as_ref().unwrap()
					)
					.into(),
				);
			}
			if !item.is_bucket() {
				return Err(format!("unexpected bucket header flag: {}", item.flags).into());
			}
			c.node()?.put(&name, &name, value, 0, Self::FLAG);
		}

		// Ignore if there's not a materialized root node.
		if self.root_node.is_none() {
			return Ok(());
		}

		{
			// Spill nodes.
			let mut root_node = self
				.root_node
				.clone()
				.ok_or_else(|| "root node empty")?
				.root();
			root_node.spill()?;
			self.root_node = Some(root_node);

			let pgid = self.root_node.as_ref().unwrap().pgid();
			let txpgid = self.tx()?.pgid();

			// Update the root node for this bucket.
			if pgid >= txpgid as u64 {
				panic!(format!(
					"pgid ({}) above high water mark ({})",
					pgid, txpgid
				));
			}

			self.bucket.root = pgid;
		}

		Ok(())
	}

	/// Returns true if a bucket is small enough to be written inline
	/// and if it contains no subbuckets. Otherwise returns false.
	fn inlineable(&self) -> bool {
		if self.root_node().is_none() || !self.root_node().unwrap().is_leaf() {
			return false;
		}

		let mut size = Page::header_size();
		let node = self.root_node.clone().unwrap();

		for inode in &*node.0.inodes.borrow() {
			if inode.flags & Self::FLAG != 0 {
				return false;
			}

			size += LeafPageElement::SIZE + inode.key.len() + inode.value.len();
			if size > self.max_inline_bucket_size() {
				return false;
			}
		}

		true
	}

	/// Returns the maximum total size of a bucket to make it a candidate for inlining.
	fn max_inline_bucket_size(&self) -> usize {
		self.tx().unwrap().db().unwrap().page_size() / 4
	}

	/// Allocates and writes a bucket to a byte slice.
	fn write(&mut self) -> Vec<u8> {
		// TODO: optimize this page alloc things
		// Allocate the appropriate size.
		let n = self.root_node.as_ref().unwrap();
		let node_size = n.size();
		let mut value = vec![0u8; IBucket::SIZE + node_size];

		// Write a bucket header.
		let bucket_ptr = value.as_mut_ptr() as *mut IBucket;
		unsafe { std::ptr::copy_nonoverlapping(&self.bucket, bucket_ptr, 1) };

		// Convert byte slice to a fake page and write the root node.
		{
			let mut page_buf = &mut value[IBucket::SIZE..];
			let mut page = Page::from_buf_mut(&mut page_buf);
			n.write(&mut page);
		}

		value
	}

	/// Attempts to balance all nodes.
	pub(crate) fn rebalance(&mut self) {
		for node in self.nodes.borrow_mut().values_mut() {
			node.rebalance()
		}
		for child in self.buckets.borrow_mut().values_mut() {
			child.rebalance()
		}
	}

	/// Creates a node from a page and associates it with a given parent.
	pub(crate) fn node(&mut self, pgid: PGID, parent: WeakNode) -> Node {
		if !self.tx().unwrap().writable() {
			panic!("tx is read-only");
		}

		if let Some(n) = self.nodes.borrow().get(&pgid) {
			return n.clone();
		};

		let mut node = NodeBuilder::new(self).parent(parent.clone()).build();
		match parent.upgrade() {
			None => {
				self.root_node.replace(node.clone());
			}
			Some(ref mut p) => {
				p.0.children.borrow_mut().push(node.clone());
			}
		}

		if let Some(ref page) = self.page {
			node.read(&page);
		} else {
			unsafe {
				node.read(&*self.tx().unwrap().page(pgid).unwrap());
			}
		}

		self.nodes.borrow_mut().insert(pgid, node.clone());

		// Update statistics.
		self.tx().unwrap().0.stats.lock().node_count += 1;

		node
	}

	/// Recursively frees all pages in the bucket.
	fn free(&mut self) {
		if self.bucket.root == 0 {
			return;
		};

		let tx = self.tx().unwrap();
		let db = tx.db().unwrap();
		self.for_each_page_node(|p, _| match p {
			Either::Left(p) => {
				let txid = tx.id();
				db.0.freelist.write().free(txid, &p).unwrap()
			}
			Either::Right(n) => n.clone().free(),
		});
		self.bucket.root = 0;
	}

	/// Returns the in-memory node, if it exists.
	/// Otherwise returns the underlying page.
	pub(crate) fn page_node(&self, id: PGID) -> Result<PageNode, Error> {
		// Inline buckets have a fake page embedded in their value so treat them
		// differently. We'll return the rootNode (if available) or the fake page.
		if self.bucket.root == 0 {
			if id != 0 {
				return Err(format!("inline bucket non-zero page access: {} != 0", id).into());
			}
			if let Some(ref node) = self.root_node {
				return Ok(PageNode::from(node.clone()));
			}
			return Ok(PageNode::from(
				&**self.page.as_ref().ok_or_else(|| "page empty")? as *const Page,
			));
		}

		// Check the node cache for non-inline buckets.
		if let Some(node) = self.nodes.borrow().get(&id) {
			return Ok(PageNode::from(node.clone()));
		};

		// Finally lookup the page from the transaction if no node is materialized.
		Ok(PageNode::from(self.tx()?.page(id)?))
	}
}
