use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

use crate::bucket::Bucket;
use crate::consts::PGID;

use super::{Node, NodeInner, WeakNode};

#[derive(Default)]
pub(crate) struct NodeBuilder {
	pub(crate) bucket: Option<*const Bucket>,
	pub(crate) is_leaf: bool,
	pub(crate) pgid: PGID,
	pub(crate) parent: WeakNode,
	pub(crate) children: Vec<Node>,
}

impl NodeBuilder {
	pub(crate) fn new(bucket: *const Bucket) -> Self {
		Self {
			bucket: Some(bucket),
			..Default::default()
		}
	}

	#[allow(clippy::wrong_self_convention)]
	pub(crate) fn is_leaf(mut self, value: bool) -> Self {
		self.is_leaf = value;
		self
	}

	pub(crate) fn parent(mut self, value: WeakNode) -> Self {
		self.parent = value;
		self
	}

	pub(crate) fn children(mut self, value: Vec<Node>) -> Self {
		self.children = value;
		self
	}

	#[allow(dead_code)]
	pub(crate) fn bucket(mut self, value: *const Bucket) -> Self {
		self.bucket = Some(value);
		self
	}

	pub(crate) fn build(self) -> Node {
		Node(Rc::new(NodeInner {
			bucket: self.bucket.unwrap(),
			is_leaf: AtomicBool::new(self.is_leaf),
			unbalanced: AtomicBool::new(false),
			spilled: AtomicBool::new(false),
			key: RefCell::new(None),
			pgid: RefCell::new(self.pgid),
			parent: RefCell::new(self.parent),
			children: RefCell::new(self.children),
			inodes: RefCell::new(vec![]),
		}))
	}
}
