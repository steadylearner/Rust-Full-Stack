use parking_lot::{
	MappedRwLockReadGuard, MappedRwLockWriteGuard, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{copy, Cursor as IOCursor, Read, Seek, SeekFrom, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Weak};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::bucket::{Bucket, Cursor};
use crate::consts::{Flags, IGNORE_NOSYNC, PGID, TXID};
use crate::db::{CheckMode, WeakDB, DB};
use crate::errors::Error;
use crate::meta::Meta;
use crate::page::{OwnedPage, Page, PageInfo};

use super::stats::TxStats;

pub(crate) struct TxInner {
	/// is transaction writable
	pub(crate) writable: bool,

	/// declares that transaction is in use
	pub(crate) managed: AtomicBool,

	/// defines whether transaction will be checked on close
	pub(crate) check: AtomicBool,

	/// ref to DB.
	/// if transaction closed then ref points to null
	pub(crate) db: RwLock<WeakDB>,

	/// transaction meta
	pub(crate) meta: RwLock<Meta>,

	/// root bucket.
	/// this bucket holds another buckets
	pub(crate) root: RwLock<Bucket>,

	/// pages cache
	pub(crate) pages: RwLock<HashMap<PGID, OwnedPage>>,

	/// transactions statistics
	pub(crate) stats: Mutex<TxStats>,

	/// list of callbacks that will be called after commit
	pub(crate) commit_handlers: Mutex<Vec<Box<dyn Fn() -> ()>>>,

	/// WriteFlag specifies the flag for write-related methods like WriteTo().
	/// Tx opens the database file with the specified flag to copy the data.
	///
	/// By default, the flag is unset, which works well for mostly in-memory
	/// workloads. For databases that are much larger than available RAM,
	/// set the flag to syscall.O_DIRECT to avoid trashing the page cache.
	pub(super) write_flag: usize,
}

impl fmt::Debug for TxInner {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let db = self
			.db
			.try_read()
			.unwrap()
			.upgrade()
			.map(|db| &db as *const DB);

		f.debug_struct("TxInner")
			.field("writable", &self.writable)
			.field("managed", &self.managed)
			.field("db", &db)
			.field("meta", &*self.meta.try_read().unwrap())
			.field("root", &*self.root.try_read().unwrap())
			.field("pages", &*self.pages.try_read().unwrap())
			.field("stats", &*self.stats.try_lock().unwrap())
			.field(
				"commit handlers len",
				&self.commit_handlers.try_lock().unwrap().len(),
			)
			.field("write_flag", &self.write_flag)
			.finish()
	}
}

/// Transaction
#[derive(Debug)]
pub struct Tx(pub(crate) Arc<TxInner>);

unsafe impl Sync for Tx {}
unsafe impl Send for Tx {}

impl Tx {
	pub(crate) fn clone(&self) -> Self {
		Self(Arc::clone(&self.0))
	}

	/// Returns whether the transaction can perform write operations.
	pub fn writable(&self) -> bool {
		self.0.writable
	}

	/// Defines whether transaction is fresh and not been used before
	/// commiting transaction twice must resolve in error
	pub(crate) fn opened(&self) -> bool {
		match self.0.db.try_read().unwrap().upgrade() {
			None => false,
			Some(db) => db.opened(),
		}
	}

	/// Returns a reference to the database that created the transaction.
	pub(crate) fn db(&self) -> Result<DB, Error> {
		self
			.0
			.db
			.try_read()
			.unwrap()
			.upgrade()
			.ok_or_else(|| Error::DatabaseGone)
	}

	/// Returns the transaction id.
	pub(crate) fn id(&self) -> TXID {
		self.0.meta.try_read().unwrap().txid
	}

	/// Returns the transaction page id.
	pub(crate) fn pgid(&self) -> PGID {
		self.0.meta.try_read().unwrap().pgid
	}

	/// Returns the transaction page id.
	pub(crate) fn set_pgid(&mut self, id: PGID) -> Result<(), Error> {
		self.0.meta.try_write().ok_or_else(|| "pgid locked")?.pgid = id;
		Ok(())
	}

	/// Adds a handler function to be executed after the transaction successfully commits.
	pub fn on_commit(&mut self, handler: Box<dyn Fn() -> ()>) {
		self.0.commit_handlers.lock().push(handler);
	}

	/// Returns current database size in bytes as seen by this transaction.
	pub(super) fn size(&self) -> i64 {
		self.pgid() as i64 * self.db().unwrap().page_size() as i64
	}

	/// Creates a cursor associated with the root bucket.
	/// All items in the cursor will return a nil value because all items in root bucket are also buckets.
	/// The cursor is only valid as long as the transaction is open.
	/// Do not use a cursor after the transaction is closed.
	pub fn cursor(&self) -> Cursor<RwLockWriteGuard<Bucket>> {
		self.0.stats.lock().cursor_count += 1;

		Cursor::new(self.0.root.write())
	}

	// Returns a copy of the current transaction statistics.
	pub fn stats(&self) -> TxStats {
		self.0.stats.lock().clone()
	}

	/// Bucket retrieves a bucket by name.
	/// Returns None if the bucket does not exist.
	/// The bucket instance is only valid for the lifetime of the transaction.
	pub fn bucket(&self, key: &[u8]) -> Result<MappedRwLockReadGuard<Bucket>, Error> {
		let bucket = self
			.0
			.root
			.try_read()
			.ok_or_else(|| "Can't acquire bucket")?;

		RwLockReadGuard::try_map(bucket, |b| b.bucket(key)).map_err(|_| "Can't get bucket".into())
	}

	/// Bucket retrieves a mutable bucket by name.
	/// Returns None if the bucket does not exist.
	/// The bucket instance is only valid for the lifetime of the transaction.
	pub fn bucket_mut(&mut self, key: &[u8]) -> Result<MappedRwLockWriteGuard<Bucket>, Error> {
		if !self.0.writable {
			return Err(Error::TxReadonly);
		};

		let bucket = self
			.0
			.root
			.try_write()
			.ok_or_else(|| "Can't acquire bucket")?;

		RwLockWriteGuard::try_map(bucket, |b| b.bucket_mut(key)).map_err(|_| "Can't get bucket".into())
	}

	/// returns bucket keys for db
	pub fn buckets(&self) -> Vec<Vec<u8>> {
		self.0.root.read().buckets()
	}

	/// Creates a new bucket.
	/// Returns an error if the bucket already exists, if the bucket name is blank, or if the bucket name is too long.
	/// The bucket instance is only valid for the lifetime of the transaction.
	pub fn create_bucket(&mut self, key: &[u8]) -> Result<MappedRwLockWriteGuard<Bucket>, Error> {
		if !self.0.writable {
			return Err(Error::TxReadonly);
		};

		let bucket = self
			.0
			.root
			.try_write()
			.ok_or_else(|| "Can't acquire bucket")?;

		RwLockWriteGuard::try_map(bucket, |b| b.create_bucket(key).ok())
			.map_err(|_| "Can't get bucket".into())
	}

	/// Creates a new bucket if it doesn't already exist.
	/// Returns an error if the bucket name is blank, or if the bucket name is too long.
	/// The bucket instance is only valid for the lifetime of the transaction.
	pub fn create_bucket_if_not_exists(
		&mut self,
		key: &[u8],
	) -> Result<MappedRwLockWriteGuard<Bucket>, Error> {
		if !self.0.writable {
			return Err(Error::TxReadonly);
		};

		let bucket = self
			.0
			.root
			.try_write()
			.ok_or_else(|| "Can't acquire bucket")?;

		RwLockWriteGuard::try_map(bucket, |b| b.create_bucket_if_not_exists(key).ok())
			.map_err(|_| "Can't get bucket".into())
	}

	/// Deletes a bucket.
	/// Returns an error if the bucket cannot be found or if the key represents a non-bucket value.
	pub fn delete_bucket(&mut self, key: &[u8]) -> Result<(), Error> {
		if !self.0.writable {
			return Err(Error::TxReadonly);
		};

		self.0.root.try_write().unwrap().delete_bucket(key)
	}

	/// Executes a function for each bucket in the root.
	/// If the provided function returns an error then the iteration is stopped and
	/// the error is returned to the caller.
	///
	/// first argument of function is bucket's key, second is bucket itself
	pub fn for_each<'a, E: Into<Error>>(
		&self,
		mut handler: Box<dyn FnMut(&[u8], Option<&Bucket>) -> Result<(), E> + 'a>,
	) -> Result<(), Error> {
		let root = self.0.root.try_write().unwrap();
		root.for_each(Box::new(|k: &[u8], _v: Option<&[u8]>| -> Result<(), E> {
			handler(k, root.bucket(k))
		}))
	}

	/// Writes the entire database to a writer.
	/// If err == nil then exactly tx.Size() bytes will be written into the writer.
	pub fn write_to<W: Write>(&self, mut w: W) -> Result<i64, Error> {
		let db = self.db()?;
		let page_size = db.page_size();
		let mut file = db
			.0
			.file
			.try_write()
			.ok_or_else(|| "can't obtain file write access")?;

		let mut written = 0;

		let mut page = OwnedPage::new(page_size);
		page.flags = Flags::META;

		// first page
		{
			*page.meta_mut() = self.0.meta.try_read().unwrap().clone();
			page.meta_mut().checksum = page.meta().sum64();
			w.write_all(&page.buf()).map_err(|e| format!("{}", e))?;
			written += page.size();
		}

		// second page
		{
			page.id = 1;
			page.meta_mut().txid -= 1;
			page.meta_mut().checksum = page.meta().sum64();
			w.write_all(&page.buf()).map_err(|e| format!("{}", e))?;
			written += page.size();
		}

		file
			.seek(SeekFrom::Start(page_size as u64 * 2))
			.map_err(|e| format!("{}", e))?;

		let size = self.size() as u64 - (page_size as u64 * 2);
		written += copy(&mut Read::by_ref(&mut file.get_mut()).take(size), &mut w)
			.map_err(|e| format!("{}", e))? as usize;

		Ok(written as i64)
	}

	/// Copies the entire database to file at the given path.
	/// A reader transaction is maintained during the copy so it is safe to continue
	/// using the database while a copy is in progress.
	pub fn copy_to(&self, path: &str, mode: OpenOptions) -> Result<(), Error> {
		let file = mode.open(path).map_err(|e| e.to_string())?;
		self.write_to(file)?;
		Ok(())
	}

	/// Closes transaction (so subsequent use of it will resolve in error)
	pub(crate) fn close(&self) -> Result<(), Error> {
		let mut db = self.db()?;
		let tx = db.remove_tx(self)?;

		*tx.0.db.write() = WeakDB::new();
		tx.0.root.try_write().unwrap().clear();
		tx.0.pages.try_write().unwrap().clear();
		Ok(())
	}

	/// Writes all changes to disk and updates the meta page.
	/// Returns an error if a disk write error occurs, or if Commit is
	/// called on a read-only transaction.
	pub fn commit(&mut self) -> Result<(), Error> {
		if self.0.managed.load(Ordering::Acquire) {
			return Err(Error::TxManaged);
		} else if !self.writable() {
			return Err(Error::TxReadonly);
		};

		let mut db = self.db()?;

		{
			let start_time = SystemTime::now();
			self.0.root.try_write().unwrap().rebalance();
			let mut stats = self.0.stats.lock();
			if stats.rebalance > 0 {
				stats.rebalance_time += SystemTime::now().duration_since(start_time)?;
			};
		}

		{
			// spill
			let start_time = SystemTime::now();
			{
				let mut root = self.0.root.try_write().unwrap();
				root.spill()?;
			}
			self.0.stats.try_lock().unwrap().spill_time = SystemTime::now().duration_since(start_time)?;
		}

		// Free the old root bucket.
		self.0.meta.try_write().unwrap().root.root = self.0.root.try_read().unwrap().bucket.root;

		let (txid, tx_pgid, freelist_pgid) = {
			let meta = self.0.meta.try_read().unwrap();
			(meta.txid as usize, meta.pgid, meta.freelist)
		};

		let (freelist_size, page_size) = {
			// Free the freelist and allocate new pages for it. This will overestimate
			// the size of the freelist but not underestimate the size (which would be bad).
			let page = db.page(freelist_pgid);
			let mut freelist = db.0.freelist.try_write().unwrap();
			freelist.free(txid as u64, &page)?;

			let freelist_size = freelist.size();
			let page_size = db.page_size() as usize;

			(freelist_size, page_size)
		};

		{
			let page = self.allocate((freelist_size / page_size) as u64 + 1);
			if let Err(e) = page {
				self.rollback()?;
				return Err(e);
			}
			let page = page?;
			let page = unsafe { &mut *page };

			db.0.freelist.try_write().unwrap().write(page);
			self.0.meta.try_write().unwrap().freelist = page.id;

			// If the high water mark has moved up then attempt to grow the database.
			if self.pgid() > tx_pgid as u64 {
				if let Err(e) = db.grow((tx_pgid + 1) * page_size as u64) {
					self.rollback()?;
					return Err(e);
				}
			}

			// Write dirty pages to disk.
			let write_start_time = SystemTime::now();
			if let Err(e) = self.write() {
				self.rollback()?;
				return Err(e);
			}

			// If strict mode is enabled then perform a consistency check.
			// Only the first consistency error is reported in the panic.
			if self.0.check.swap(false, Ordering::AcqRel) {
				let strict = db.0.check_mode.contains(CheckMode::STRICT);
				if let Err(e) = self.check_sync() {
					if strict {
						self.rollback()?;
						return Err(e);
					} else {
						println!("{}", e);
					}
				}
			};

			// Write meta to disk.
			if let Err(e) = self.write_meta() {
				self.0.check.store(false, Ordering::Release);
				self.rollback()?;
				return Err(e);
			};

			self.0.stats.try_lock().unwrap().write_time +=
				SystemTime::now().duration_since(write_start_time)?;
		};

		// Finalize the transaction.
		self.close()?;

		{
			for h in &*self.0.commit_handlers.try_lock().unwrap() {
				h();
			}
		}

		Ok(())
	}

	/// Closes the transaction and ignores all previous updates. Read-only
	/// transactions must be rolled back and not committed.
	pub fn rollback(&self) -> Result<(), Error> {
		if self.0.managed.load(Ordering::Acquire) {
			return Err(Error::TxManaged);
		};
		self.__rollback()?;
		Ok(())
	}

	pub(crate) fn __rollback(&self) -> Result<(), Error> {
		let db = self.db()?;
		if self.0.check.swap(false, Ordering::AcqRel) {
			let strict = db.0.check_mode.contains(CheckMode::STRICT);
			if let Err(e) = self.check_sync() {
				if strict {
					return Err(e);
				} else {
					println!("{}", e);
				}
			}
		};
		if self.writable() {
			let txid = self.id();
			let mut freelist = db.0.freelist.write();
			freelist.rollback(txid);
			let freelist_id = db.meta()?.freelist;
			let freelist_page = db.page(freelist_id);
			freelist.reload(&freelist_page);
		};
		self.close()?;
		Ok(())
	}

	/// Sync version of check()
	///
	/// In case of checking thread panic will also return Error
	pub fn check_sync(&self) -> Result<(), Error> {
		let (sender, ch) = mpsc::channel::<String>();
		let tx = self.clone();
		let handle = thread::spawn(move || tx.__check(sender));

		let mut errs = vec![];
		for err in ch {
			errs.push(err);
		}

		if handle.join().is_err() {
			errs.push("check thread panicked".to_string());
		}

		if !errs.is_empty() {
			return Err(Error::CheckFail(errs));
		};

		Ok(())
	}

	// Performs several consistency checks on the database for this transaction.
	// An error is returned if any inconsistency is found.
	//
	// It can be safely run concurrently on a writable transaction. However, this
	// incurs a high cost for large databases and databases with a lot of subbuckets
	// because of caching. This overhead can be removed if running on a read-only
	// transaction, however, it is not safe to execute other writer transactions at
	// the same time.
	pub fn check(&self) -> mpsc::Receiver<String> {
		let (sender, receiver) = mpsc::channel::<String>();
		let tx = self.clone();
		thread::spawn(move || tx.__check(sender));
		receiver
	}

	pub(super) fn __check(&self, ch: mpsc::Sender<String>) {
		let mut freed = HashMap::<PGID, bool>::new();
		let all_pgids = self
			.db()
			.unwrap()
			.0
			.freelist
			.try_read()
			.unwrap()
			.get_pgids();

		for id in &all_pgids {
			if freed.contains_key(id) {
				ch.send(format!("page {}: already freed", id)).unwrap();
			}
			freed.insert(*id, true);
		}

		let mut reachable = HashMap::new();
		reachable.insert(0, true);
		reachable.insert(1, true);
		let freelist_pgid = self
			.0
			.meta
			.try_read_for(Duration::from_secs(10))
			.unwrap()
			.freelist;
		let freelist_overflow = unsafe { &*self.page(freelist_pgid).unwrap() }.overflow;
		for i in 0..=freelist_overflow {
			reachable.insert(freelist_pgid + u64::from(i), true);
		}

		self.check_bucket(
			&self.0.root.try_read().unwrap(),
			&mut reachable,
			&freed,
			&ch,
		);

		for i in 0..self.0.meta.try_read().unwrap().pgid {
			let is_reachable = reachable.contains_key(&i);
			let is_freed = freed.contains_key(&i);
			if !is_reachable && !is_freed {
				ch.send(format!("page {}: unreachable unfreed", i)).unwrap();
			};
		}
	}

	fn check_bucket(
		&self,
		b: &Bucket,
		reachable: &mut HashMap<PGID, bool>,
		freed: &HashMap<PGID, bool>,
		ch: &mpsc::Sender<String>,
	) {
		if b.bucket.root == 0 {
			return;
		}

		let meta_pgid = self.pgid();

		let handler = Box::new(|p: &Page, _pgid: usize| {
			if p.id > meta_pgid {
				ch.send(format!("page {}: out of bounds: {}", p.id, meta_pgid))
					.unwrap();
			}

			for i in 0..=p.overflow {
				let id = p.id + u64::from(i);
				if reachable.contains_key(&id) {
					ch.send(format!("page {}: multiple references", id))
						.unwrap();
				}
				reachable.insert(id, true);
			}

			let page_type_is_valid = match p.flags {
				Flags::BRANCHES | Flags::LEAVES => true,
				_ => false,
			};

			if freed.contains_key(&p.id) {
				ch.send(format!("page {}: reachable freed", p.id)).unwrap();
			} else if !page_type_is_valid {
				ch.send(format!("page {}: invalid type: {}", p.id, p.flags))
					.unwrap();
			}
		});

		b.tx().unwrap().for_each_page(b.bucket.root, 0, handler);

		b.for_each(Box::new(|k, _v| -> Result<(), String> {
			let child = b.bucket(k);
			if let Some(child) = child {
				self.check_bucket(child, reachable, freed, ch);
			};
			Ok(())
		}))
		.unwrap();
	}

	/// Returns a contiguous block of memory starting at a given page.
	pub(crate) fn allocate(&mut self, count: u64) -> Result<*mut Page, Error> {
		let mut db = match self.db() {
			Err(_) => return Err(Error::TxClosed),
			Ok(db) => db,
		};

		let mut page = db.allocate(count, self)?;
		let page_id = page.id;
		let page_ptr = &mut *page as *mut Page;

		self.0.pages.try_write().unwrap().insert(page_id, page);

		// Update statistics.
		{
			let mut stats = self.0.stats.lock();
			stats.page_count += 1;
			stats.page_alloc += count as usize * self.db()?.page_size();
		}

		Ok(page_ptr)
	}

	/// Writes any dirty pages to disk.
	pub(crate) fn write(&mut self) -> Result<(), Error> {
		let mut pages: Vec<_> = self
			.0
			.pages
			.write()
			.drain()
			.map(|x| {
				let pgid = x.1.id;
				(pgid, x.1)
			})
			.collect();
		pages.sort_by(|a, b| a.0.cmp(&b.0));
		let mut db = self.db()?;

		let page_size = db.page_size();
		for (id, p) in &pages {
			let size = (p.overflow + 1) as usize * page_size;
			let offset = *id as u64 * page_size as u64;

			let buf = unsafe { std::slice::from_raw_parts(p.as_ptr(), size) };
			let cursor = IOCursor::new(buf);

			db.write_at(offset, cursor)?;
		}

		if !db.0.no_sync || IGNORE_NOSYNC {
			db.sync()?;
		}

		{
			let mut page_pool = db.0.page_pool.lock();
			let mut i = 0;
			while i != pages.len() {
				if pages[i].1.size() == page_size {
					let mut page = pages.remove(i).1;
					for i in page.buf_mut() {
						*i = 0;
					}
					page_pool.push(page);
				} else {
					i += 1;
				}
			}
		}

		Ok(())
	}

	/// Writes the meta to the disk.
	pub(crate) fn write_meta(&mut self) -> Result<(), Error> {
		let mut db = self.db()?;

		let mut buf = vec![0u8; db.page_size() as usize];
		let mut page = Page::from_buf_mut(&mut buf);
		self.0.meta.try_write().unwrap().write(&mut page)?;

		db.write_at(0, IOCursor::new(buf))?;

		if !db.0.no_sync || IGNORE_NOSYNC {
			db.sync()?;
		}

		self.0.stats.lock().write += 1;
		Ok(())
	}

	/// Returns a reference to the page with a given id.
	/// If page has been written to then a temporary buffered page is returned.
	pub(crate) fn page(&self, id: PGID) -> Result<*const Page, Error> {
		// Check the dirty pages first.
		{
			let pages = self.0.pages.try_read().unwrap();
			if let Some(p) = pages.get(&id) {
				return Ok(&**p);
			}
		}

		// Otherwise return directly from the mmap.
		Ok(&*self.db()?.page(id))
	}

	/// Iterates over every page within a given page and executes a function.
	pub(crate) fn for_each_page<'a>(
		&self,
		pgid: PGID,
		depth: usize,
		mut func: Box<dyn FnMut(&Page, usize) + 'a>,
	) {
		let p = unsafe { &*self.page(pgid).unwrap() };

		// Execute function.
		func(&p, depth);

		// Recursively loop over children.
		let flags = p.flags;
		if flags != Flags::BRANCHES {
			return;
		}
		let count = p.count as usize;
		for i in 0..count {
			let el = p.branch_page_element(i);
			self.for_each_page(el.pgid, depth + 1, Box::new(|p, d| func(p, d)));
		}
	}

	/// Returns page information for a given page number.
	/// This is only safe for concurrent use when used by a writable transaction.
	pub fn page_info(&self, id: usize) -> Result<Option<PageInfo>, Error> {
		if !self.opened() {
			return Err(Error::TxClosed);
		};
		if id >= self.pgid() as usize {
			return Ok(None);
		};

		let db = self.db()?;

		// Build the page info.
		let p = db.page(id as u64);
		let mut info = PageInfo {
			id: id as isize,
			ptype: Flags::FREELIST,
			count: p.count as usize,
			overflow_count: p.overflow as usize,
		};

		// Determine the type (or if it's free).
		if !db.0.freelist.try_read().unwrap().freed(id as u64) {
			info.ptype = p.flags
		}

		Ok(Some(info))
	}
}

impl Drop for Tx {
	fn drop(&mut self) {
		let count = Arc::strong_count(&self.0);
		// one for the reference, and one that is owned by db
		if count > 2 {
			return;
		};
		if let Ok(_db) = self.db() {
			if self.0.writable {
				self.commit().unwrap();
			} else {
				self.rollback().unwrap();
			}
		}
	}
}

#[derive(Clone)]
pub(crate) struct WeakTx(Weak<TxInner>);

impl WeakTx {
	pub(crate) fn new() -> Self {
		Self(Weak::new())
	}

	pub(crate) fn upgrade(&self) -> Option<Tx> {
		self.0.upgrade().map(Tx)
	}

	pub(crate) fn from(tx: &Tx) -> Self {
		Self(Arc::downgrade(&tx.0))
	}
}
