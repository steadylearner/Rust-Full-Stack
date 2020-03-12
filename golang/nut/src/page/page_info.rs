use crate::consts::Flags;

#[derive(Clone, Debug)]
pub struct PageInfo {
	pub id: isize,
	pub ptype: Flags,
	pub count: usize,
	pub overflow_count: usize,
}
