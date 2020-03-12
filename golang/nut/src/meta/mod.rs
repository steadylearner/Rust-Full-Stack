#[cfg(test)]
mod tests;

use fnv::FnvHasher;
use std::hash::Hasher;

use crate::bucket::IBucket;
use crate::consts::{MAGIC, PGID, TXID, VERSION};
use crate::errors::Error;
use crate::page::{Page, PageData};

/// Meta type of the page
///
/// Primary usage is the first two pages of db which declare
/// magic header, db version and other common fields
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Meta {
	/// database mime header
	pub magic: u32,

	/// database version
	pub version: u32,

	/// defined page size.
	/// u32 to be platform independent
	pub page_size: u32,

	/// haven't seen it's usage
	pub flags: u32,

	/// bucket that has root property changed
	/// during commits and transactions
	pub root: IBucket,

	/// free list page id
	pub freelist: PGID,

	/// pgid high watermark
	pub pgid: PGID,

	/// transaction id
	pub txid: TXID,

	/// meta checksum
	pub checksum: u64,
}

impl Default for Meta {
	fn default() -> Self {
		Self {
			magic: MAGIC,
			version: VERSION,
			page_size: page_size::get() as u32,
			flags: 0,
			root: IBucket::new(),
			freelist: 0,
			pgid: 0,
			txid: 0,
			checksum: 0,
		}
	}
}

impl Meta {
	pub(crate) const SIZE: usize = std::mem::size_of::<Self>();

	pub fn validate(&self) -> Result<(), Error> {
		if self.magic != MAGIC {
			return Err(Error::Invalid);
		} else if self.version != VERSION {
			return Err(Error::VersionMismatch);
		} else if self.checksum != 0 && self.checksum != self.sum64() {
			return Err(Error::Checksum);
		};
		Ok(())
	}

	pub(crate) fn write(&mut self, p: &mut Page) -> Result<(), Error> {
		if self.root.root >= self.pgid {
			return Err(
				format!(
					"root bucket pgid ({}) above high water mark ({})",
					self.root.root, self.pgid
				)
				.into(),
			);
		} else if self.freelist >= self.pgid {
			return Err(
				format!(
					"freelist pgid ({}) above high water mark ({})",
					self.freelist, self.pgid
				)
				.into(),
			);
		}

		// Page id is either going to be 0 or 1 which we can determine by the transaction ID.
		p.id = self.txid % 2;
		// Calculate the checksum.
		self.checksum = self.sum64();
		p.copy_data_from(PageData::Meta(self));

		Ok(())
	}

	// generates the checksum for the meta.
	pub fn sum64(&self) -> u64 {
		let mut h = FnvHasher::default();
		let bytes = unsafe {
			std::slice::from_raw_parts(self as *const Self as *const u8, offset_of!(Meta, checksum))
		};
		h.write(bytes);
		h.finish()
	}
}
