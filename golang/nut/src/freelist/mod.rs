#[cfg(test)]
pub mod tests;

use std::collections::HashMap;
use std::mem::size_of;

use crate::consts::{Flags, PGID, TXID};
use crate::errors::Error;
use crate::page::{merge_pgids, Page};
use crate::utils::find_contiguous;

/// freelist represents a list of all pages that are available for allocation.
/// It also tracks pages that have been freed but are still in use by open transactions.
#[derive(Debug, Clone)]
pub(crate) struct FreeList {
	/// all free and available free page ids
	ids: Vec<PGID>,
	/// mapping of soon-to-be free page ids by tx
	pending: HashMap<TXID, Vec<PGID>>,
	/// fast lookup of all free and pending page ids.
	cache: HashMap<PGID, bool>,
}

impl Default for FreeList {
	fn default() -> Self {
		Self {
			ids: Vec::new(),
			pending: HashMap::new(),
			cache: HashMap::new(),
		}
	}
}

impl FreeList {
	/// size returns the size of the page after serialization.
	pub fn size(&self) -> usize {
		let mut n = self.count();
		if n >= 0xFFFF {
			n += 1;
		};
		Page::header_size() + (size_of::<PGID>() * n)
	}

	/// count returns count of pages on the freelist
	pub fn count(&self) -> usize {
		self.free_count() + self.pending_count()
	}

	/// free_count returns count of free pages
	pub fn free_count(&self) -> usize {
		self.ids.len()
	}

	/// pending_count returns count of pending pages
	pub fn pending_count(&self) -> usize {
		self.pending.iter().fold(0, |c, x| c + x.1.len())
	}

	/// returns vec with all pgids
	pub fn get_pgids(&self) -> Vec<PGID> {
		let mut m = Vec::with_capacity(self.count());
		for list in self.pending.values() {
			m.extend_from_slice(&list);
		}
		m.extend_from_slice(&self.ids);
		m.sort();
		m
	}

	/// allocate returns the starting page id of a contiguous list of pages of a given size.
	/// If a contiguous block cannot be found then 0 is returned.
	pub fn allocate(&mut self, span: u64) -> PGID {
		if self.ids.is_empty() {
			return 0;
		}

		if let Some(index) = find_contiguous(&self.ids, span as usize) {
			let pgid = self.ids[index];
			if pgid <= 1 {
				panic!(format!("invalid page allocation: {}", pgid));
			};
			self.ids.drain(index..index + span as usize);
			// Remove from the free cache.
			for i in 0..span {
				self.cache.remove(&(pgid as u64 + i));
			}
			return pgid;
		} else {
			return 0;
		}
	}

	/// free releases a page and its overflow for a given transaction id.
	/// If the page is already free then a panic will occur.
	pub(crate) fn free(&mut self, txid: TXID, p: &Page) -> Result<(), Error> {
		if p.id <= 1 {
			return Err(format!("cannot free page 0 or 1: {}", p.id).into());
		}

		// Free page and all its overflow pages.
		let ids = self.pending.entry(txid).or_insert_with(|| vec![]);
		let max = p.id + u64::from(p.overflow);
		for id in p.id..=max {
			// Verify that page is not already free.
			if self.cache.contains_key(&id) {
				return Err(format!("page {} already freed", id).into());
			}

			// Add to the freelist and cache.
			ids.push(id);
			self.cache.insert(id, true);
		}

		Ok(())
	}

	/// release moves all page ids for a transaction id (or older) to the freelist.
	pub fn release(&mut self, txid: TXID) {
		let mut m = Vec::new();
		let mut tx_id_to_remove = Vec::new();
		for (tid, ids) in self.pending.iter() {
			let tid = *tid;
			if tid <= txid {
				m.append(&mut ids.clone());
				tx_id_to_remove.push(tid);
			}
		}

		for tid in &tx_id_to_remove {
			self.pending.remove(tid);
		}

		m.sort();
		self.ids = merge_pgids(self.ids.as_slice(), &m.as_slice());
	}

	/// release moves all page ids for a transaction id (or older) to the freelist.
	pub fn rollback(&mut self, txid: TXID) {
		// Remove page ids from cache.
		if let Some(pending) = self.pending.get(&txid) {
			for id in pending {
				self.cache.remove(&id).unwrap();
			}
		}

		// Remove pages from pending list.
		self.pending.remove(&txid);
	}

	/// freed returns whether a given page is in the free list.
	pub fn freed(&self, pgid: PGID) -> bool {
		self.cache.contains_key(&pgid)
	}

	/// read initializes the freelist from a freelist page.
	pub(crate) fn read(&mut self, p: &Page) {
		assert_eq!(
			p.flags,
			Flags::FREELIST,
			"freelist flags incorrect: {}",
			p.flags.bits()
		);
		let mut idx = 0;
		let mut count = p.count as usize;
		if count == 0xFFFF {
			idx = 1;
			let frl = p.freelist();
			count = frl[0] as usize;
		}

		if count == 0 {
			self.ids.clear();
		} else {
			let ids = p.freelist();
			self.ids = ids[idx..count].to_vec();
			self.ids.sort();
		}

		self.reindex();
	}

	/// write writes the page ids onto a freelist page. All free and pending ids are
	/// saved to disk since in the event of a program crash, all pending ids will
	/// become free.
	pub fn write(&mut self, p: &mut Page) {
		// Combine the old free pgids and pgids waiting on an open transaction.

		// The page.count can only hold up to 64k elements so if we overflow that
		// number then we handle it by putting the size in the first element.
		let lenids = self.count();
		p.flags = Flags::FREELIST;
		if lenids == 0 {
			p.count = lenids as u16;
		} else if lenids < 0xFFFF {
			p.count = lenids as u16;
			let m = p.freelist_mut();
			m.copy_from_slice(&self.get_pgids())
		} else {
			p.count = 0xFFFF;
			let m = p.freelist_mut();
			m[0] = lenids as u64;
			m.copy_from_slice(&self.get_pgids());
		}
	}

	/// reload reads the freelist from a page and filters out pending items.
	pub fn reload(&mut self, p: &Page) {
		self.read(p);

		let mut pcache = Vec::<PGID>::new();
		for (_, pids) in self.pending.iter() {
			for pid in pids.iter() {
				pcache.push(*pid);
			}
		}

		// Check each page in the freelist and build a new available freelist
		// with any pages not in the pending lists.
		let mut a = Vec::<PGID>::new();
		for id in self.ids.iter() {
			if !pcache.contains(id) {
				a.push(*id);
			}
		}
		self.ids = a;

		self.reindex();
	}

	pub fn reindex(&mut self) {
		let mut new_cache: HashMap<PGID, bool> = HashMap::new();
		for id in self.ids.iter() {
			new_cache.insert(*id, true);
		}
		for (_key, ids) in self.pending.iter() {
			for id in ids.iter() {
				new_cache.insert(*id, true);
			}
		}
		self.cache = new_cache;
	}
}
