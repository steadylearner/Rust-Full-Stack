use super::{leaf_page_element::LeafPageElement, OwnedPage, Page, PageData};
use crate::consts::{Flags, PGID};

#[test]
fn new() {
	let mut buf = vec![0u8; 256];
	let mut page = Page::from_buf_mut(&mut buf);
	assert_eq!(page.overflow, 0);
	assert_eq!(page.count, 0);
	assert_eq!(page.id, 0);

	page.id = 25;
	assert_eq!(page.id, 25);

	page.flags = Flags::META;
	assert_eq!(page.flags, Flags::META);
}

#[test]
fn copy_data_from() {
	let mut page = OwnedPage::new(1024);
	page.flags = Flags::FREELIST;
	let ids: &[PGID] = &[23, 50];
	page.copy_data_from(PageData::Freelist(&ids));

	assert_eq!(page.count, 2);
	match page.data() {
		PageData::Freelist(l) => {
			assert_eq!(&[23, 50], l);
		}
		_ => panic!("wrong page type"),
	}
}

#[test]
fn read_leaf_nodes() {
	let mut page = OwnedPage::new(4096);
	page.flags = Flags::LEAVES;
	page.count = 2;
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
		std::slice::from_raw_parts_mut(&mut nodes[2] as *mut LeafPageElement as *mut u8, 4096)
	};
	data[..7].copy_from_slice("barfooz".as_bytes());
	data[7..7 + 13].copy_from_slice("helloworldbye".as_bytes());

	let el = page.leaf_page_element(0);
	assert_eq!(el.ksize, 3);
	assert_eq!(el.vsize, 4);
	assert_eq!(el.pos, 32);

	let el = page.leaf_page_element(1);
	assert_eq!(el.ksize, 10);
	assert_eq!(el.vsize, 3);
	assert_eq!(el.pos, 23);
}

#[test]
fn to_owned() {
	let mut buf = vec![0u8; 1024];
	let mut page = Page::from_buf_mut(&mut buf);
	page.flags = Flags::LEAVES;
	page.count = 2;
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
		std::slice::from_raw_parts_mut(&mut nodes[2] as *mut LeafPageElement as *mut u8, 4096)
	};
	data[..7].copy_from_slice("barfooz".as_bytes());
	data[7..7 + 13].copy_from_slice("helloworldbye".as_bytes());

	let owned = page.to_owned();
	assert_eq!(
		owned.page.len(),
		Page::header_size() + LeafPageElement::SIZE + 23 + 10 + 3
	);
}
