use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::bucket::Bucket;
use crate::consts::{Flags, MIN_KEYS_PER_PAGE, PGID};
use crate::errors::Error;
use crate::page::{BranchPageElement, LeafPageElement, Page};
use crate::utils::clamp;

use super::{INode, NodeBuilder};

#[derive(Debug)]
pub(crate) struct NodeInner {
	/// associated bucket
	pub(super) bucket: *const Bucket,

	/// is node is leaf element
	pub(super) is_leaf: AtomicBool,

	/// is node unbalanced
	pub(super) unbalanced: AtomicBool,

	/// is node spilled
	pub(super) spilled: AtomicBool,

	/// first inode's key
	pub(super) key: RefCell<Option<Vec<u8>>>,

	/// page's id
	pub(super) pgid: RefCell<PGID>,

	/// parent node if exists
	pub(super) parent: RefCell<WeakNode>,

	/// list of child nodes
	pub(crate) children: RefCell<Vec<Node>>,

	/// node's data
	pub(crate) inodes: RefCell<Vec<INode>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Node(pub(crate) Rc<NodeInner>);

impl Node {
	pub(crate) fn root(&self) -> Node {
		match self.parent() {
			None => self.clone(),
			Some(ref p) => p.root(),
		}
	}

	#[inline]
	pub(crate) fn is_leaf(&self) -> bool {
		self.0.is_leaf.load(Ordering::Acquire)
	}

	#[inline]
	pub(crate) fn pgid(&self) -> PGID {
		*self.0.pgid.borrow()
	}

	#[inline]
	pub(crate) fn parent(&self) -> Option<Node> {
		self.0.parent.borrow().upgrade()
	}

	/// Returns the number of children.
	#[inline]
	fn num_children(&self) -> usize {
		self.0.inodes.borrow().len()
	}

	/// Returns the child node at a given index.
	pub fn child_at(&mut self, index: usize) -> Result<Node, Error> {
		if self.is_leaf() {
			return Err(format!("invalid childAt({}) on a leaf node", index).into());
		}
		Ok(
			self
				.bucket_mut()
				.unwrap()
				.node(self.0.inodes.borrow()[index].pgid, WeakNode::from(self)),
		)
	}

	/// Returns the next node with the same parent.
	fn next_sibling(&self) -> Option<Node> {
		match self.parent() {
			None => None,
			Some(mut parent) => {
				let index = parent.child_index(self);
				if index >= parent.num_children() as isize - 1 {
					return None;
				}
				parent.child_at((index + 1) as usize).ok()
			}
		}
	}

	/// Returns the previous node with the same parent.
	fn prev_sibling(&self) -> Option<Node> {
		match self.parent() {
			None => None,
			Some(mut parent) => {
				let index = parent.child_index(self);
				if index == 0 {
					return None;
				}
				parent.child_at((index - 1) as usize).ok()
			}
		}
	}

	/// Breaks up a node into multiple smaller nodes, if appropriate.
	/// This should only be called from the spill() function.
	///
	/// returns None if no split occured, or parent Node otherwise
	pub(crate) fn split(&mut self, page_size: usize) -> Result<Option<Node>, Error> {
		// let mut parent = self.parent();
		let mut nodes = vec![self.clone()];
		while let Some(n) = nodes.last().unwrap().clone().split_two(page_size)? {
			nodes.push(n);
		}

		if nodes.len() == 1 {
			return Ok(None);
		}

		let parent = match self.parent() {
			Some(p) => {
				let mut children = p.0.children.borrow_mut();
				let index = children.iter().position(|ch| Rc::ptr_eq(&self.0, &ch.0));
				debug_assert!(index.is_some());
				children.remove(index.unwrap());
				for node in nodes {
					*node.0.parent.borrow_mut() = WeakNode::from(&p);
					children.push(node);
				}
				drop(children);

				p
			}
			None => {
				let parent = NodeBuilder::new(self.0.bucket).children(nodes).build();
				for ch in &mut *parent.0.children.borrow_mut() {
					*ch.0.parent.borrow_mut() = WeakNode::from(&parent);
				}
				parent
			}
		};

		Ok(Some(parent))
	}

	/// Breaks up a node into two smaller nodes, if appropriate.
	/// This should only be called from the split() function.
	///
	/// Returns next node if splid did occur, or None if not
	pub(super) fn split_two(&mut self, page_size: usize) -> Result<Option<Node>, Error> {
		let split_index = {
			if self.0.inodes.borrow().len() <= (MIN_KEYS_PER_PAGE * 2) || self.size_less_than(page_size) {
				return Ok(None);
			}

			let fp = clamp(
				self.bucket().ok_or_else(|| "bucket empty")?.fill_percent,
				Bucket::MIN_FILL_PERCENT,
				Bucket::MAX_FILL_PERCENT,
			);
			let threshold = (fp * page_size as f64) as usize;
			let (split_index, _) = self.split_index(threshold);
			split_index
		};

		let next = NodeBuilder::new(self.0.bucket)
			.is_leaf(self.is_leaf())
			.build();
		let nodes = self.0.inodes.borrow_mut().drain(split_index..).collect();
		// parent.0.children.borrow_mut().push(next.clone());
		*next.0.inodes.borrow_mut() = nodes;

		self
			.bucket_mut()
			.ok_or_else(|| "bucket empty")?
			.tx()?
			.0
			.stats
			.lock()
			.split += 1;

		Ok(Some(next))
	}

	/// Writes the nodes to dirty pages and splits nodes as it goes.
	/// Returns an error if dirty pages cannot be allocated.
	pub fn spill(&mut self) -> Result<(), Error> {
		if self.0.spilled.load(Ordering::Acquire) {
			return Ok(());
		}

		let page_size = self.bucket().unwrap().tx()?.db()?.page_size();
		{
			let mut children = self.0.children.borrow_mut().clone();
			children.sort_by(Node::cmp_by_key);
			for child in &mut *children {
				child.spill()?;
			}

			self.0.children.borrow_mut().clear();
		}

		let mut node_parent = None;

		{
			let mut nodes = match self.split(page_size)? {
				None => vec![self.clone()],
				Some(p) => {
					node_parent = Some(p.clone());
					p.0.children.borrow().clone()
				}
			};

			let bucket = self.bucket_mut().unwrap();
			let mut tx = bucket.tx()?;
			let db = tx.db()?;
			let txid = tx.id();

			for node in &mut nodes {
				{
					// Add node's page to the freelist if it's not new.
					let node_pgid = *node.0.pgid.borrow_mut();
					if node_pgid > 0 {
						db.0
							.freelist
							.try_write()
							.unwrap()
							.free(txid, unsafe { &*tx.page(node_pgid)? })
							.unwrap();
						*node.0.pgid.borrow_mut() = 0;
					}
				}

				let page = tx.allocate((node.size() / db.page_size()) as u64 + 1)?;
				let mut page = unsafe { &mut *page };

				{
					// Write the node.
					let id = page.id;
					let txpgid = tx.pgid();
					if id >= txpgid {
						panic!(format!("pgid ({}) above high water mark ({})", id, txid))
					}
					*node.0.pgid.borrow_mut() = id;
					node.write(&mut page);
					node.0.spilled.store(true, Ordering::Release);
				}

				// Insert into parent inodes.
				if let Some(mut p) = node.parent() {
					let mut okey = node.0.key.borrow().clone();
					let nkey = node.0.inodes.borrow()[0].key.to_vec();
					if okey.is_none() {
						okey = Some(nkey.clone());
					}
					let pgid = *node.0.pgid.borrow();
					p.put(okey.unwrap().as_slice(), &nkey, vec![], pgid, 0);
					*node.0.key.borrow_mut() = Some(nkey);
					assert!(
						!node.0.key.borrow().as_ref().unwrap().is_empty(),
						"spill: zero-length node key"
					)
				}

				tx.0.stats.lock().spill += 1;
			}
		}

		// If the root node split and created a new root then we need to spill that
		// as well. We'll clear out the children to make sure it doesn't try to respill.
		{
			let spill_parent = match node_parent {
				None => None,
				Some(p) => {
					let pgid_valid = *p.0.pgid.borrow() == 0;
					if pgid_valid {
						Some(p)
					} else {
						None
					}
				}
			};
			if let Some(parent) = spill_parent {
				self.0.children.borrow_mut().clear();
				// setting self as parent to hold strong reference
				*self = parent;
				return self.spill();
			}
		}

		Ok(())
	}

	/// Attempts to combine the node with sibling nodes if the node fill
	/// size is below a threshold or if there are not enough keys.
	pub fn rebalance(&mut self) {
		{
			let selfsize = self.size();
			if !self.0.unbalanced.load(Ordering::Acquire) {
				return;
			}

			self.0.unbalanced.store(false, Ordering::Release);

			// update stats and get threshold;
			let threshold = {
				let bucket = self.bucket_mut().unwrap();
				let tx = bucket.tx().unwrap();

				tx.0.stats.lock().rebalance += 1;
				tx.db().unwrap().page_size() / 4
			};

			if selfsize > threshold && self.0.inodes.borrow().len() > self.min_keys() as usize {
				return;
			}

			if self.parent().is_none() {
				let mut inodes = self.0.inodes.borrow_mut();
				if !self.is_leaf() && inodes.len() == 1 {
					let mut child = self
						.bucket_mut()
						.unwrap()
						.node(inodes[0].pgid, WeakNode::from(self));

					self.0.is_leaf.store(child.is_leaf(), Ordering::Release);
					*inodes = child.0.inodes.borrow_mut().drain(..).collect();
					*self.0.children.borrow_mut() = child.0.children.borrow_mut().drain(..).collect();

					// Reparent all child nodes being moved.
					{
						let inode_pgids = inodes.iter().map(|i| i.pgid);
						let bucket = self.bucket_mut().unwrap();
						for pgid in inode_pgids {
							if let Some(child) = bucket.nodes.borrow_mut().get_mut(&pgid) {
								*child.0.parent.borrow_mut() = WeakNode::from(self);
							}
						}
					}

					*child.0.parent.borrow_mut() = WeakNode::new();
					self
						.bucket_mut()
						.unwrap()
						.nodes
						.borrow_mut()
						.remove(&child.0.pgid.borrow());
					child.free();
				}

				return;
			}
		}

		// If node has no keys then just remove it.
		if self.num_children() == 0 {
			let key = self.0.key.borrow().clone().unwrap();
			let pgid = *self.0.pgid.borrow();
			let mut parent = self.parent().unwrap();
			parent.del(&key);
			parent.remove_child(self);
			self.bucket_mut().unwrap().nodes.borrow_mut().remove(&pgid);
			self.free();
			parent.rebalance();
			return;
		}

		assert!(
			self.parent().unwrap().num_children() > 1,
			"parent must have at least 2 children"
		);

		let (use_next_sibling, mut target) = {
			let use_next_sibling = self.parent().unwrap().child_index(self) == 0;
			let target = if use_next_sibling {
				self.next_sibling().unwrap()
			} else {
				self.prev_sibling().unwrap()
			};
			(use_next_sibling, target)
		};

		if use_next_sibling {
			let bucket = self.bucket_mut().unwrap();
			for pgid in target.0.inodes.borrow().iter().map(|i| i.pgid) {
				if let Some(child) = bucket.nodes.borrow_mut().get_mut(&pgid) {
					child.parent().unwrap().remove_child(child);
					*child.0.parent.borrow_mut() = WeakNode::from(self);
					child
						.parent()
						.unwrap()
						.0
						.children
						.borrow_mut()
						.push(child.clone());
				}
			}

			self
				.0
				.inodes
				.borrow_mut()
				.append(&mut *target.0.inodes.borrow_mut());
			{
				let mut parent = self.parent().unwrap();
				parent.del(&target.0.key.borrow().as_ref().unwrap());
				parent.remove_child(&target);
			}
			self
				.bucket_mut()
				.unwrap()
				.nodes
				.borrow_mut()
				.remove(&target.pgid());
			target.free();
		} else {
			for pgid in target.0.inodes.borrow().iter().map(|i| i.pgid) {
				if let Some(child) = self.bucket_mut().unwrap().nodes.borrow_mut().get_mut(&pgid) {
					let mut parent = child.parent().unwrap();
					parent.remove_child(&child);
					*child.0.parent.borrow_mut() = WeakNode::from(&target);
					parent.0.children.borrow_mut().push(child.clone());
				}
			}

			target
				.0
				.inodes
				.borrow_mut()
				.append(&mut *self.0.inodes.borrow_mut());
			{
				let mut parent = self.parent().unwrap();
				parent.del(&self.0.key.borrow().as_ref().unwrap());
				parent.remove_child(&self);
			}
			self
				.bucket_mut()
				.unwrap()
				.nodes
				.borrow_mut()
				.remove(&self.pgid());
			self.free();
		}

		self.parent().unwrap().rebalance();
	}

	/// Removes a node from the list of in-memory children.
	/// This does not affect the inodes.
	fn remove_child(&mut self, target: &Node) {
		let index = self
			.0
			.children
			.borrow()
			.iter()
			.position(|c| Rc::ptr_eq(&target.0, &c.0));
		if let Some(i) = index {
			self.0.children.borrow_mut().remove(i);
		}
	}

	/// Adds the node's underlying page to the freelist.
	pub fn free(&mut self) {
		if *self.0.pgid.borrow() == 0 {
			return;
		};
		{
			let bucketmut = self.bucket_mut().unwrap();
			let tx = bucketmut.tx().unwrap();
			let txid = tx.id();
			let page = unsafe { &*tx.page(*self.0.pgid.borrow()).unwrap() };
			let db = tx.db().unwrap();
			db.0.freelist.write().free(txid, &page).unwrap();
		}
		*self.0.pgid.borrow_mut() = 0;
	}

	pub(super) fn bucket<'a, 'b: 'a>(&'a self) -> Option<&'b Bucket> {
		if self.0.bucket.is_null() {
			return None;
		}
		Some(unsafe { &*self.0.bucket })
	}

	pub(super) fn bucket_mut<'a, 'b: 'a>(&'a self) -> Option<&'b mut Bucket> {
		if self.0.bucket.is_null() {
			return None;
		}
		Some(unsafe { &mut *(self.0.bucket as *mut Bucket) })
	}

	pub fn min_keys(&self) -> u8 {
		if self.is_leaf() {
			1
		} else {
			2
		}
	}

	/// Returns the size of each page element based on the type of node.
	fn page_element_size(&self) -> usize {
		if self.is_leaf() {
			return LeafPageElement::SIZE;
		}
		BranchPageElement::SIZE
	}

	/// Returns the size of the node after serialization.
	pub(crate) fn size(&self) -> usize {
		let mut sz = Page::header_size();
		let elsz = self.page_element_size();
		for ind in &*self.0.inodes.borrow() {
			sz += elsz + ind.key.len() as usize + ind.value.len();
		}
		sz
	}

	/// Returns true if the node is less than a given size.
	/// This is an optimization to avoid calculating a large node when we only need
	/// to know if it fits inside a certain page size.
	pub(super) fn size_less_than(&self, v: usize) -> bool {
		let mut sz = Page::header_size();
		let elsz = self.page_element_size();
		for ind in &*self.0.inodes.borrow() {
			sz += elsz + ind.key.len() + ind.value.len();
			if sz >= v {
				return false;
			}
		}
		true
	}

	/// Inserts a key/value.
	pub fn put(&mut self, old_key: &[u8], new_key: &[u8], value: Vec<u8>, pgid: PGID, flags: u32) {
		let meta_pgid = self.bucket().unwrap().tx().unwrap().pgid();

		if pgid >= meta_pgid {
			panic!(format!(
				"pgid ({}) above high water mark ({})",
				pgid, meta_pgid
			));
		} else if old_key.is_empty() {
			panic!("put: zero-length old key")
		} else if new_key.is_empty() {
			panic!("put: zero-length new key")
		}

		let mut inodes = self.0.inodes.borrow_mut();
		let (exact, index) = match inodes.binary_search_by(|i| i.key.as_slice().cmp(old_key)) {
			Ok(n) => (true, n),
			Err(n) => (false, n),
		};

		if !exact {
			inodes.insert(index, INode::default());
		}

		let mut inode = &mut inodes[index];
		inode.key = new_key.to_vec();
		inode.value = value;
		inode.flags = flags;
		inode.pgid = pgid;
	}

	// Removes a key from the node.
	pub fn del(&mut self, key: &[u8]) {
		let mut inodes = self.0.inodes.borrow_mut();

		let (exact, index) = match inodes.binary_search_by(|i| i.key.as_slice().cmp(key)) {
			Ok(n) => (true, n),
			Err(n) => (false, n),
		};

		if !exact {
			return;
		}

		inodes.remove(index);

		self.0.unbalanced.store(true, Ordering::Release);
	}

	/// Writes the items onto one or more pages.
	pub(crate) fn write(&self, p: &mut Page) {
		if self.is_leaf() {
			p.flags |= Flags::LEAVES;
		} else {
			p.flags |= Flags::BRANCHES;
		}

		let inodes = self.0.inodes.borrow_mut();

		if inodes.len() >= 0xFFFF {
			panic!(format!("inode overflow: {} (pgid={})", inodes.len(), p.id));
		}

		p.count = inodes.len() as u16;

		if inodes.is_empty() {
			return;
		}

		let mut bptr = unsafe {
			let offset = self.page_element_size() * inodes.len();
			p.get_data_mut_ptr().add(offset)
		};
		let is_leaf = self.is_leaf();
		let pgid = p.id;
		for (i, item) in inodes.iter().enumerate() {
			assert!(!item.key.is_empty(), "write: zero-length inode key");

			if is_leaf {
				let mut elem = p.leaf_page_element_mut(i);
				let el_ptr = elem as *const LeafPageElement as *const u8;
				elem.pos = unsafe { bptr.sub(el_ptr as usize) } as u32;
				elem.flags = item.flags as u32;
				elem.ksize = item.key.len() as u32;
				elem.vsize = item.value.len() as u32;
			} else {
				let mut elem = p.branch_page_element_mut(i);
				let el_ptr = elem as *const BranchPageElement as *const u8;
				elem.pos = unsafe { bptr.sub(el_ptr as usize) } as u32;
				elem.ksize = item.key.len() as u32;
				elem.pgid = item.pgid;
				assert!(elem.pgid != pgid, "write: circular dependency occurred");
			}

			let (klen, vlen) = (item.key.len(), item.value.len());

			unsafe {
				std::ptr::copy_nonoverlapping(item.key.as_ptr(), bptr, klen);
				bptr = bptr.add(klen);
				std::ptr::copy_nonoverlapping(item.value.as_ptr(), bptr, vlen);
				bptr = bptr.add(vlen);
			}
		}
	}

	/// Returns the index of a given child node.
	pub(super) fn child_index(&self, child: &Node) -> isize {
		for (index, node) in self.0.inodes.borrow().iter().enumerate() {
			if Some(&node.key) == child.0.key.borrow().as_ref() {
				return index as isize;
			}
		}
		-1
	}

	/// Initializes the node from a page.
	pub(crate) fn read(&mut self, page: &Page) {
		*self.0.pgid.borrow_mut() = page.id;
		self.0.is_leaf.store(
			match page.flags {
				Flags::LEAVES => true,
				_ => false,
			},
			Ordering::Release,
		);
		let mut inodes = Vec::<INode>::with_capacity(page.count as usize);
		let is_leaf = self.is_leaf();

		for i in 0..page.count as usize {
			if is_leaf {
				let elem = page.leaf_page_element(i);
				let inode = INode {
					flags: elem.flags,
					key: elem.key().to_vec(),
					value: elem.value().to_vec(),
					pgid: 0,
				};
				inodes.push(inode);
			} else {
				let elem = page.branch_page_element(i);
				let inode = INode {
					flags: 0,
					key: elem.key().to_vec(),
					value: Vec::new(),
					pgid: elem.pgid,
				};
				inodes.push(inode);
			}
		}

		*self.0.inodes.borrow_mut() = inodes;

		{
			let inodes = self.0.inodes.borrow();
			*self.0.key.borrow_mut() = if inodes.is_empty() {
				None
			} else {
				Some(inodes[0].key.clone())
			};
		}
	}

	/// Finds the position where a page will fill a given threshold.
	/// It returns the index as well as the size of the first page.
	/// This is only be called from split().
	pub(super) fn split_index(&self, threshold: usize) -> (usize, usize) {
		let mut rindex = 0;
		let mut pgsize = Page::header_size();

		let inodes = self.0.inodes.borrow();
		let pelsize = self.page_element_size();
		let max = inodes.len() - MIN_KEYS_PER_PAGE;

		for (index, inode) in inodes.iter().enumerate().take(max) {
			rindex = index;
			let elsize = pelsize + inode.key.len() + inode.value.len();
			if index >= MIN_KEYS_PER_PAGE && (pgsize + elsize) > threshold {
				break;
			}

			pgsize += elsize;
		}

		(rindex, pgsize)
	}

	/// Causes the node to copy all its inode key/value references to heap memory.
	/// This is required when the mmap is reallocated so inodes are not pointing to stale data.
	// pub(crate) fn dereference(&mut self) {
	// TODO: in current implementation key,value are already copied
	// from page so function is pointless

	// if self.key.is_some() {
	// 	self.key = self.key.clone();
	// 	assert!(
	// 		self.pgid == 0 || self.key.as_ref().unwrap().len() > 0,
	// 		"dereference: zero-length node key on existing node"
	// 	)
	// }

	// for inode in &mut self.inodes {
	// 	inode.key = inode.key.clone();
	// 	assert!(inode.key.len() > 0, "dereference: zero-length inode key");

	// 	inode.value = inode.value.clone();
	// }

	// // Recursively dereference children.
	// for child in &mut self.children {
	// 	child.borrow_mut().dereference()
	// }

	// // Update statistics.
	// {
	// 	let txref = self.bucket().unwrap().tx.upgrade().unwrap();
	// 	let mut tx = txref.0.write().unwrap();
	// 	tx.stats.node_deref += 1;
	// }
	// }

	fn cmp_by_key(a: &Node, b: &Node) -> std::cmp::Ordering {
		a.0.inodes.borrow()[0].key.cmp(&b.0.inodes.borrow()[0].key)
	}
}

#[derive(Clone, Default, Debug)]
pub(crate) struct WeakNode(Weak<NodeInner>);

impl WeakNode {
	pub(crate) fn new() -> Self {
		Self(Weak::new())
	}

	pub(crate) fn upgrade(&self) -> Option<Node> {
		self.0.upgrade().map(Node)
	}

	pub(crate) fn from(tx: &Node) -> Self {
		Self(Rc::downgrade(&tx.0))
	}
}
