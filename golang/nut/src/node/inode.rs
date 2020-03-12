use crate::consts::PGID;

/// Node's data container
#[derive(Debug, Default)]
pub struct INode {
	/// bit flags
	pub flags: u32,

	/// page if
	pub pgid: PGID,

	/// data key
	pub key: Vec<u8>,

	/// data key
	pub value: Vec<u8>,
}
