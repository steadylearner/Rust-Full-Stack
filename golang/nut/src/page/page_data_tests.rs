use super::PageData;
use crate::consts::PGID;
use crate::meta::Meta;

#[test]
fn new() {
	let ids: &[PGID] = &[23];
	let pd = PageData::Freelist(ids);
	assert_eq!(std::mem::size_of::<usize>(), pd.as_bytes().len());

	let ids: &[PGID] = &[23, 50, 77, 90, 12, 23];
	let pd = PageData::Freelist(ids);
	assert_eq!(std::mem::size_of::<usize>() * 6, pd.as_bytes().len());

	let m = Meta {
		..Default::default()
	};
	let pd = PageData::Meta(&m);
	assert_eq!(std::mem::size_of::<Meta>(), pd.as_bytes().len());
}
