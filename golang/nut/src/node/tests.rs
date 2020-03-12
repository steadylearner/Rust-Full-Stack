use std::sync::atomic::Ordering;

use crate::bucket::{tests::bucket_mock, Bucket};
use crate::consts::Flags;
use crate::page::{LeafPageElement, OwnedPage};
use crate::tx::tests::tx_mock;
use crate::tx::WeakTx;

use super::{Node, NodeBuilder};

pub(crate) fn node_mock(bucket: *const Bucket) -> Node {
	NodeBuilder::new(bucket).build()
}

#[test]
fn create() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.put(b"baz", b"baz", [1, 22, 3].to_vec(), 0, 0);
	n.put(b"foo", b"foo", b"barack".to_vec(), 0, 0);
	n.put(b"bar", b"bar", b"bill".to_vec(), 0, 0);
	n.put(b"foo", b"fold", b"donald".to_vec(), 0, 1);

	let inodes = n.0.inodes.borrow();
	assert_eq!(inodes.len(), 3);

	assert_eq!(inodes[0].key, b"bar");
	assert_eq!(inodes[0].value, b"bill".to_vec());

	assert_eq!(inodes[1].key, b"baz");
	assert_eq!(inodes[1].value, [1, 22, 3].to_vec());

	assert_eq!(inodes[2].key, b"fold");
	assert_eq!(inodes[2].value, b"donald".to_vec());
}

#[test]
fn read_page() {
	let mut page = {
		let mut page = OwnedPage::new(1024);
		page.id = 1;
		page.overflow = 0;
		page.flags = Flags::LEAVES;
		page.count = 2;
		page
	};
	{
		let nodes =
			unsafe { std::slice::from_raw_parts_mut(page.get_data_mut_ptr() as *mut LeafPageElement, 3) };
		nodes[0] = LeafPageElement {
			flags: 0,
			pos: 32,
			ksize: 3,
			vsize: 4,
		};
		nodes[1] = LeafPageElement {
			flags: 0,
			pos: 23,
			ksize: 10,
			vsize: 3,
		};

		let data = unsafe {
			std::slice::from_raw_parts_mut(&mut nodes[2] as *mut LeafPageElement as *mut u8, 1024)
		};
		data[..7].copy_from_slice(b"barfooz");
		data[7..7 + 13].copy_from_slice(b"helloworldbye");
	}

	assert!(
		std::mem::size_of::<LeafPageElement>() == 16,
		format!("{}", std::mem::size_of::<LeafPageElement>())
	);

	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.read(&page);
	let inodes = n.0.inodes.borrow();

	assert!(n.is_leaf());
	assert_eq!(inodes.len(), 2);
	assert_eq!(inodes[0].key, b"bar");
	assert_eq!(inodes[0].value, b"fooz");
	assert_eq!(inodes[1].key, b"helloworld");
	assert_eq!(inodes[1].value, b"bye");
}

#[test]
fn write_page() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	n.put(
		b"susy",
		b"susy",
		b"que".to_vec(),
		0,
		Flags::LEAVES.bits() as u32,
	);
	n.put(
		b"ricki",
		b"ricki",
		b"lake".to_vec(),
		0,
		Flags::LEAVES.bits() as u32,
	);
	n.put(
		b"john",
		b"john",
		b"johnson".to_vec(),
		0,
		Flags::LEAVES.bits() as u32,
	);

	let mut page = {
		let mut page = OwnedPage::new(4096);
		page.id = 1;
		page.overflow = 0;
		page.flags = Flags::LEAVES;
		page
	};
	n.write(&mut page);

	let nodes =
		unsafe { std::slice::from_raw_parts_mut(page.get_data_mut_ptr() as *mut LeafPageElement, 3) };
	assert_eq!(nodes[0].ksize, 4);
	assert_eq!(nodes[0].vsize, 7);
	assert_eq!(nodes[0].pos, 48);
	assert_eq!(nodes[1].ksize, 5);
	assert_eq!(nodes[1].vsize, 4);
	assert_eq!(nodes[1].pos, 43);
	assert_eq!(nodes[2].ksize, 4);
	assert_eq!(nodes[2].vsize, 3);
	assert_eq!(nodes[2].pos, 36);

	let mut n2 = node_mock(&bucket);
	n2.read(&page);
	let inodes = n2.0.inodes.borrow();
	assert!(n2.is_leaf());
	assert_eq!(inodes.len(), 3);
	assert_eq!(inodes[0].key, b"john",);
	assert_eq!(inodes[0].value, b"johnson");
	assert_eq!(inodes[1].key, b"ricki");
	assert_eq!(inodes[1].value, b"lake");
	assert_eq!(inodes[2].key, b"susy");
	assert_eq!(inodes[2].value, b"que");
}

#[test]
fn split_two() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	n.put(b"00000001", b"00000001", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000003", b"00000003", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000004", b"00000004", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000005", b"00000005", b"0123456701234567".to_vec(), 0, 0);

	// Split between 2 & 3.
	let next = n.split_two(100).unwrap();

	assert!(next.is_some());
}

#[test]
fn split_two_fail() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	n.put(b"00000001", b"00000001", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000003", b"00000003", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000004", b"00000004", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000005", b"00000005", b"0123456701234567".to_vec(), 0, 0);

	// Split between 2 & 3.
	let next = n.split_two(4096).unwrap();

	assert!(next.is_none());
}

#[test]
fn split() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	n.put(b"00000001", b"00000001", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000003", b"00000003", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000004", b"00000004", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000005", b"00000005", b"0123456701234567".to_vec(), 0, 0);

	// Split between 2 & 3.
	let parent = n.split(100).unwrap();

	assert!(parent.is_some());
	let parent = parent.unwrap();
	let pchildren = parent.0.children.borrow();
	assert_eq!(pchildren.len(), 2);
	assert_eq!(pchildren[1].0.inodes.borrow().len(), 3);
	assert_eq!(pchildren[0].0.inodes.borrow().len(), 2);
}

#[test]
fn split_big() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	for i in 1..1000 {
		let key = format!("{:08}", i);
		n.put(
			&key.as_bytes(),
			&key.as_bytes(),
			b"0123456701234567".to_vec(),
			0,
			0,
		);
	}

	let parent = n.split(4096).unwrap().unwrap();

	assert_eq!(parent.0.children.borrow().len(), 19);
}

#[test]
fn split_min_keys() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	n.put(b"00000001", b"00000001", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);

	let parent = n.split(20).unwrap();
	assert!(parent.is_none());
}

#[test]
fn split_single_page() {
	let tx = tx_mock();
	let bucket = bucket_mock(WeakTx::from(&tx));
	let mut n = node_mock(&bucket);
	n.0.is_leaf.store(true, Ordering::Release);
	n.put(b"00000001", b"00000001", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);
	n.put(b"00000002", b"00000002", b"0123456701234567".to_vec(), 0, 0);

	let parent = n.split(4096).unwrap();
	assert!(parent.is_none());
}
