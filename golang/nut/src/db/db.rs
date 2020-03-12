use fs2::FileExt;
use lock_api::{RawMutex, RawRwLock};
use page_size;
use parking_lot::{MappedRwLockReadGuard, Mutex, RwLock, RwLockReadGuard};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Weak};
use std::time::Duration;
use std::u64;

use crate::bucket::IBucket;
use crate::consts::{Flags, MAGIC, MAX_MAP_SIZE, MAX_MMAP_STEP, PGID, VERSION};
use crate::errors::Error;
use crate::freelist::FreeList;
use crate::meta::Meta;
use crate::page::{OwnedPage, Page};
use crate::tx::{Tx, TxBuilder};

use super::builder::Options;
use super::common::{Batch, Call};
use super::info::Info;
use super::stats::Stats;
use super::{RWTxGuard, TxGuard};

bitflags! {
	/// Defines when db check will occur
	pub struct CheckMode: u8 {
		/// no check
		const NO = 0b0000;
		/// check on database close
		const CLOSE = 0b0001;
		/// check on end of read transaction
		///
		/// If there is parallel write transaction going on
		/// check may fail because of old freelist metadata
		const READ = 0b0010;
		/// check on end of write transaction
		const WRITE = 0b0100;
		/// check on close, and end of every transaction
		const ALL = 0b0111;
		/// defines whether result of the check will result
		/// in error or just be spewed in stdout
		const STRICT = 0b1000;
		/// check on close and writes and panic on error
		const STRONG = 0b1101;
		/// check everything and panic on error
		const PARANOID = 0b1111;
	}
}

pub(crate) struct DBInner {
	pub(crate) check_mode: CheckMode,
	pub(crate) no_sync: bool,
	pub(super) no_grow_sync: bool,
	pub(super) max_batch_size: usize,
	pub(super) max_batch_delay: Duration,
	pub(super) autoremove: bool,
	pub(super) alloc_size: u64,
	pub(super) path: Option<PathBuf>,
	pub(crate) file: RwLock<BufWriter<File>>,
	pub(super) mmap_size: Mutex<usize>,
	pub(super) mmap: RwLock<memmap::Mmap>,
	pub(super) file_size: RwLock<u64>,
	page_size: usize,
	opened: AtomicBool,
	pub(crate) rw_lock: Mutex<()>,
	pub(crate) rw_tx: RwLock<Option<Tx>>,
	pub(crate) txs: RwLock<Vec<Tx>>,
	pub(crate) freelist: RwLock<FreeList>,
	pub(crate) stats: RwLock<Stats>,
	pub(crate) batch: Mutex<Option<Batch>>,
	pub(crate) page_pool: Mutex<Vec<OwnedPage>>,
	read_only: bool,
}

/// Database
#[derive(Clone)]
pub struct DB(pub(crate) Arc<DBInner>);

impl<'a> DB {
	pub(super) fn open_file<P: Into<PathBuf>>(
		mut file: File,
		path: Option<P>,
		options: Options,
	) -> Result<Self, Error> {
		let path = path.map(|v| v.into());
		let needs_initialization = (!std::path::Path::new(path.as_ref().unwrap()).exists())
			|| file.metadata().map_err(|_| "Can't read metadata")?.len() == 0;

		if options.read_only {
			file.lock_exclusive().map_err(|_e| "Cannot lock db file")?;
		} else {
			file.lock_shared().map_err(|_e| "Cannot lock db file")?;
		};

		let page_size = if needs_initialization {
			page_size::get()
		} else {
			let mut buf = vec![0u8; 1000];
			file.read_exact(&mut buf)?;
			let page = Page::from_buf(&buf);
			if page.flags != Flags::META {
				return Err("Database format unknown".into());
			}
			page.meta().page_size as usize
		};

		file
			.allocate(page_size as u64 * 4)
			.map_err(|_e| "Cannot allocate 4 required pages")?;

		let mmap = unsafe {
			memmap::MmapOptions::new()
				.offset(0)
				.len(page_size)
				.map(&file)
				.map_err(|e| format!("mmap failed: {}", e))?
		};

		let mut db = Self(Arc::new(DBInner {
			check_mode: options.checkmode,
			no_sync: false,
			no_grow_sync: options.no_grow_sync,
			max_batch_size: options.max_batch_size,
			max_batch_delay: options.max_batch_delay,
			autoremove: options.autoremove,
			alloc_size: 0,
			path,
			file: RwLock::new(BufWriter::new(file)),
			mmap_size: Mutex::new(0),
			mmap: RwLock::new(mmap),
			file_size: RwLock::new(0),
			page_size,
			opened: AtomicBool::new(true),
			rw_lock: Mutex::new(()),
			rw_tx: RwLock::new(None),
			txs: RwLock::new(Vec::new()),
			freelist: RwLock::new(FreeList::default()),
			stats: RwLock::from(Stats::default()),
			page_pool: Mutex::new(Vec::new()),
			batch: Mutex::new(None),
			read_only: options.read_only,
		}));

		if needs_initialization {
			db.init()?;
		}

		db.mmap(options.initial_mmap_size as u64)?;

		{
			let freelist_id = db.meta()?.freelist;
			db.mmap((freelist_id + 1) * db.0.page_size as u64)?;
			let freelist_page = db.page(freelist_id);
			db.0.freelist.try_write().unwrap().read(&freelist_page);
		}

		Ok(db)
	}

	pub(super) fn open<P: AsRef<Path>>(path: P, options: Options) -> Result<Self, Error> {
		let path = path.as_ref().to_owned();
		let file = {
			let mut open_opts = OpenOptions::new();
			open_opts.read(true);
			if !options.read_only {
				open_opts.write(true).create(true);
			};
			open_opts.open(&path).map_err(|e| format!("{}", e))?
		};

		DB::open_file(file, Some(path), options)
	}

	/// Returns meta info for given path
	pub fn get_meta(file: &mut File) -> Result<Meta, String> {
		let mut buf = [0u8; 12];
		file
			.seek(SeekFrom::Start(Page::header_size() as u64))
			.map_err(|_| "Can't seek file".to_string())?;
		file
			.read_exact(&mut buf)
			.map_err(|_| "Can't read file to buffer".to_string())?;
		let magic: u32 = unsafe { *(&buf[0] as *const u8 as *const u32) };
		if magic != MAGIC {
			return Err("Incorrect header".to_string());
		};
		let page_size: u32 = unsafe { *(&buf[8] as *const u8 as *const u32) };
		let page_size = page_size as usize;
		file
			.seek(SeekFrom::Start(0))
			.map_err(|_| "Can't seek file".to_string())?;
		let mut metabuf = vec![0u8; page_size * 2];
		file
			.read_exact(&mut metabuf)
			.map_err(|_| "Can't read file to buffer".to_string())?;

		let metap0 = unsafe { &*(&metabuf[0] as *const u8 as *const Page) };
		let metap1 = unsafe { &*(&metabuf[page_size] as *const u8 as *const Page) };

		let (mut meta0, mut meta1) = (metap0.meta(), metap1.meta());
		if meta1.txid > meta0.txid {
			std::mem::swap(&mut meta0, &mut meta1);
		}

		if meta0.validate().is_ok() {
			return Ok(meta0.clone());
		}

		if meta1.validate().is_ok() {
			return Ok(meta1.clone());
		}

		Err("Invalid meta".to_string())
	}

	#[inline(always)]
	/// Returns database's defined page size
	pub fn page_size(&self) -> usize {
		self.0.page_size
	}

	pub(crate) fn remove_tx(&mut self, tx: &Tx) -> Result<Tx, Error> {
		if tx.writable() {
			let (freelist_free_n, freelist_pending_n, freelist_alloc) = {
				let freelist = self.0.freelist.try_read().unwrap();
				let freelist_free_n = freelist.free_count();
				let freelist_pending_n = freelist.pending_count();
				let freelist_alloc = freelist.size();
				(freelist_free_n, freelist_pending_n, freelist_alloc)
			};

			let tx = {
				let mut db_tx = self.0.rw_tx.write();
				if db_tx.is_none() {
					return Err("No write transaction exists".into());
				};
				if !Arc::ptr_eq(&tx.0, &db_tx.as_ref().unwrap().0) {
					return Err("trying to remove unowned transaction".into());
				};
				db_tx.take().unwrap()
			};
			unsafe {
				self.0.rw_lock.raw().unlock();
			}

			let mut stats = self.0.stats.write();
			stats.free_page_n = freelist_free_n;
			stats.pending_page_n = freelist_pending_n;
			stats.free_alloc = (freelist_free_n + freelist_pending_n) * self.0.page_size;
			stats.freelist_in_use = freelist_alloc;
			stats.tx_stats += tx.stats();

			Ok(tx)
		} else {
			let mut txs = self.0.txs.try_write_for(Duration::from_secs(10)).unwrap();

			let index = txs.iter().position(|dbtx| Arc::ptr_eq(&tx.0, &dbtx.0));
			debug_assert!(index.is_some(), "trying to remove nonexistent tx");
			let index = index.unwrap();

			let tx = txs.remove(index);
			unsafe {
				self.0.mmap.raw().unlock_shared();
			}

			let mut stats = self.0.stats.write();
			stats.open_tx_n = txs.len();
			stats.tx_stats += tx.0.stats.lock().clone();

			Ok(tx)
		}
	}

	pub(super) fn init(&mut self) -> Result<(), Error> {
		let mut buf = vec![0u8; self.0.page_size * 4];
		for i in 0..=1 {
			let mut p = self.page_in_buffer(&mut buf, i);
			p.id = i as u64;
			p.flags = Flags::META;
			let m = p.meta_mut();
			m.magic = MAGIC;
			m.version = VERSION;
			m.page_size = self.0.page_size as u32;
			m.freelist = 2;
			m.root = {
				let mut b = IBucket::new();
				b.root = 3;
				b
			};
			m.pgid = 4;
			m.txid = i as u64;
			m.checksum = m.sum64();
		}

		let mut p = self.page_in_buffer(&mut buf, 2);
		p.id = 2;
		p.flags = Flags::FREELIST;

		let mut p = self.page_in_buffer(&mut buf, 3);
		p.id = 3;
		p.flags = Flags::LEAVES;

		self.write_at(0, std::io::Cursor::new(&mut buf))?;
		self.sync()?;

		Ok(())
	}

	/// Closes database and removes it's data file
	pub fn remove(mut self) -> Result<(), Error> {
		if !self.0.opened.load(Ordering::Acquire) {
			return Ok(());
		}
		self.cleanup()?;
		if let Some(path) = &self.0.path {
			if path.exists() {
				std::fs::remove_file(path).map_err(|_| "Can't remove file")?;
			}
		}

		Ok(())
	}

	fn cleanup(&mut self) -> Result<(), Error> {
		if !self.0.opened.load(Ordering::Acquire) {
			return Ok(());
		}
		if self.0.check_mode.contains(CheckMode::CLOSE) {
			let strict = self.0.check_mode.contains(CheckMode::STRICT);
			let tx = self.begin_tx()?;
			if let Err(e) = tx.check_sync() {
				if strict {
					return Err(e);
				} else {
					println!("{}", e);
				}
			}
		};
		// self.munmap()?;
		self.0.opened.store(false, Ordering::Release);

		self
			.0
			.file
			.try_read()
			.ok_or_else(|| "Can't acquire file lock")?
			.get_ref()
			.unlock()
			.map_err(|_| "Can't unlock db file")?;
		if self.0.autoremove {
			if let Some(path) = &self.0.path {
				if path.exists() {
					std::fs::remove_file(path).map_err(|_| "Can't remove file")?;
				}
			}
		}
		Ok(())
	}

	#[inline(always)]
	pub fn path(&self) -> Option<PathBuf> {
		self.0.path.clone()
	}

	#[inline(always)]
	pub fn read_only(&self) -> bool {
		self.0.read_only
	}

	#[inline(always)]
	pub fn opened(&self) -> bool {
		self.0.opened.load(Ordering::Acquire)
	}

	/// Starts a new transaction.
	/// Multiple read-only transactions can be used concurrently but only one
	/// write transaction can be used at a time.
	///
	/// Transactions should not be dependent on one another.
	///
	/// If a long running read transaction (for example, a snapshot transaction) is
	/// needed, you might want to set DBBuilder.initial_mmap_size to a large enough value
	/// to avoid potential blocking of write transaction.
	pub fn begin_tx(&'a self) -> Result<TxGuard<'a>, Error> {
		if !self.opened() {
			return Err(Error::DatabaseClosed);
		};

		unsafe {
			self.0.mmap.raw().lock_shared();
		}

		let tx = TxBuilder::new()
			.db(WeakDB::from(self))
			.writable(false)
			.check(self.0.check_mode.contains(CheckMode::READ))
			.build();

		let mut txs = self.0.txs.write();
		txs.push(tx.clone());
		let txs_len = txs.len();
		drop(txs);

		{
			let mut stats = self.0.stats.write();
			stats.tx_n += 1;
			stats.open_tx_n = txs_len;
		}

		Ok(TxGuard {
			tx,
			db: std::marker::PhantomData,
		})
	}

	/// Starts a new writable transaction.
	/// Multiple read-only transactions can be used concurrently but only one
	/// write transaction can be used at a time.
	///
	/// Transactions should not be dependent on one another.
	///
	/// If a long running read transaction (for example, a snapshot transaction) is
	/// needed, you might want to set DBBuilder.initial_mmap_size to a large enough value
	/// to avoid potential blocking of write transaction.
	pub fn begin_rw_tx(&'a mut self) -> Result<RWTxGuard<'a>, Error> {
		if self.read_only() {
			return Err(Error::DatabaseReadonly);
		};
		if !self.opened() {
			return Err(Error::DatabaseClosed);
		};

		unsafe {
			self.0.rw_lock.raw().lock();
		};
		let mut rw_tx = self.0.rw_tx.write();

		let txs = self.0.txs.read();
		let minid = txs
			.iter()
			.map(|tx| tx.id())
			.min()
			.unwrap_or_else(|| 0xFFFF_FFFF_FFFF_FFFF);
		drop(txs);

		let tx = TxBuilder::new()
			.db(WeakDB::from(self))
			.writable(true)
			.check(self.0.check_mode.contains(CheckMode::WRITE))
			.build();
		*rw_tx = Some(tx.clone());
		drop(rw_tx);

		// Free any pages associated with closed read-only transactions.
		if minid > 0 {
			self.0.freelist.try_write().unwrap().release(minid - 1);
		}

		Ok(RWTxGuard {
			tx,
			db: std::marker::PhantomData,
		})
	}

	/// shorthand for db.begin_rw_tx with additional guarantee for panic safery
	pub fn update<'b>(
		&mut self,
		mut handler: Box<dyn FnMut(&mut Tx) -> Result<(), String> + 'b>,
	) -> Result<(), Error> {
		use std::panic::{catch_unwind, AssertUnwindSafe};

		let mut tx = scopeguard::guard(self.begin_rw_tx()?, |tx| {
			let db_exists = tx.db().is_ok();
			if db_exists {
				tx.__rollback().unwrap();
			};
		});

		let result = catch_unwind(AssertUnwindSafe(|| {
			tx.0.managed.store(true, Ordering::Release);
			let result = handler(&mut tx);
			tx.0.managed.store(false, Ordering::Release);
			result
		}));

		if result.is_err() {
			tx.__rollback()?;
			return Err("Panic while update".into());
		}

		let result = result.unwrap();
		if let Err(e) = result {
			tx.rollback()?;
			return Err(e.into());
		};

		tx.commit()
	}

	/// shorthand for db.begin_tx with additional guarantee for panic safery
	pub fn view<'b>(
		&self,
		handler: Box<dyn Fn(&Tx) -> Result<(), String> + 'b>,
	) -> Result<(), Error> {
		use std::panic::{catch_unwind, AssertUnwindSafe};

		let tx = scopeguard::guard(self.begin_tx()?, |tx| {
			let db_exists = tx.db().is_ok();
			if db_exists {
				tx.__rollback().unwrap();
			};
		});

		let result = catch_unwind(AssertUnwindSafe(|| {
			tx.0.managed.store(true, Ordering::Release);
			let result = handler(&tx);
			tx.0.managed.store(false, Ordering::Release);
			result
		}));

		if result.is_err() {
			tx.__rollback()?;
			return Err("Panic while update".into());
		}

		let result = result.unwrap();
		if let Err(e) = result {
			tx.rollback()?;
			return Err(e.into());
		};

		tx.rollback()
	}

	/// Calls fn as part of a batch. It behaves similar to Update,
	/// except:
	///
	/// 1. concurrent batch calls can be combined into a single
	/// transaction.
	///
	/// 2. the function passed to batch may be called multiple times,
	/// regardless of whether it returns error or not.
	///
	/// This means that Batch function side effects must be idempotent and
	/// take permanent effect only after a successful return is seen in
	/// caller.
	///
	/// The maximum batch size and delay can be adjusted with DBBuilder.batch_size
	/// and DBBuilder.batch_delay, respectively.
	///
	/// Batch is only useful when there are multiple threads calling it.
	/// While calling it multiple times from single thread just blocks
	/// thread for each single batch call
	pub fn batch(
		&mut self,
		handler: Box<dyn Fn(&mut Tx) -> Result<(), String> + Send>,
	) -> Result<(), Error> {
		let weak_db = WeakDB::from(self);
		let handler = Arc::new(handler);
		let handler_clone = handler.clone();

		let err_receiver = {
			let mut batch = self.0.batch.lock();
			if batch.is_none() || batch.as_ref().unwrap().closed() {
				*batch = Some(Batch::new(
					weak_db,
					self.0.max_batch_size,
					self.0.max_batch_delay,
				));
			};
			let batch = batch.as_mut().unwrap();

			let (err_sender, err_receiver) = mpsc::channel();
			batch.push(Call::new(handler_clone, err_sender))?;
			err_receiver
		};

		if err_receiver.recv().is_err() {
			return self.update(Box::new(|tx| handler(tx)));
		}

		Ok(())
	}

	pub(crate) fn write_at<T: Read>(&mut self, pos: u64, mut buf: T) -> Result<(), Error> {
		let mut file = self.0.file.write();
		file
			.seek(SeekFrom::Start(pos))
			.map_err(|_e| "can't seek to position")?;
		std::io::copy(&mut buf, &mut *file).map_err(|_e| "can't write buffer")?;
		Ok(())
	}

	pub(super) fn mmap(&mut self, mut min_size: u64) -> Result<(), Error> {
		let file = self
			.0
			.file
			.try_read_for(Duration::from_secs(60))
			.ok_or_else(|| "can't acquire file lock")?;
		let mut mmap = self
			.0
			.mmap
			.try_write_for(Duration::from_secs(600))
			.ok_or_else(|| "can't acquire mmap lock")?;

		let init_min_size = self.0.page_size as u64 * 4;

		min_size = u64::max(min_size, init_min_size);

		let mut size = self.mmap_size(min_size)?;

		if mmap.len() >= size as usize {
			return Ok(());
		}

		if self.0.read_only {
			let file_len = file
				.get_ref()
				.metadata()
				.map_err(|_| "can't get file metadata")?
				.len();
			size = u64::min(size, file_len);
		}

		let mut mmap_size = self.0.mmap_size.lock();

		file
			.get_ref()
			.allocate(size)
			.map_err(|_| "can't allocate space")?;

		// TODO: madvise
		let mut mmap_opts = memmap::MmapOptions::new();
		let nmmap = unsafe {
			mmap_opts
				.offset(0)
				.len(size as usize)
				.map(&*file.get_ref())
				.map_err(|e| format!("mmap failed: {}", e))?
		};
		*mmap_size = nmmap.len();
		*mmap = nmmap;

		drop(file);
		drop(mmap);
		drop(mmap_size);

		let check0 = self.page(0).meta().validate();
		let check1 = self.page(1).meta().validate();

		// Validate the meta pages. We only return an error if both meta pages fail
		// validation, since meta0 failing validation means that it wasn't saved
		// properly -- but we can recover using meta1. And vice-versa.
		if check0.is_err() && check1.is_err() {
			return Err(format!("mmap fail: {}", check0.unwrap_err()).into());
		}

		Ok(())
	}

	fn mmap_size(&self, mut size: u64) -> Result<u64, Error> {
		// Double the size from 32KB until 1GB.
		for i in 15..=30 {
			if size <= 1 << i {
				return Ok(1 << i);
			}
		}

		// Verify the requested size is not above the maximum allowed.
		if size > MAX_MAP_SIZE {
			return Err("mmap too large".into());
		};

		// If larger than 1GB then grow by 1GB at a time.
		let remainder = size % MAX_MMAP_STEP;
		if remainder > 0 {
			size += MAX_MAP_SIZE - remainder;
		};

		// Ensure that the mmap size is a multiple of the page size.
		// This should always be true since we're incrementing in MBs.
		let page_size = self.0.page_size as u64;
		if (size % page_size) != 0 {
			size = ((size / page_size) + 1) * page_size;
		};

		// If we've exceeded the max size then only grow up to the max size.
		if size > MAX_MAP_SIZE {
			size = MAX_MAP_SIZE
		};

		Ok(size)
	}

	pub(crate) fn sync(&mut self) -> Result<(), Error> {
		self
			.0
			.file
			.write()
			.flush()
			.map_err(|_e| "can't sync data".into())
	}

	pub fn stats(&self) -> Stats {
		self.0.stats.try_read().unwrap().clone()
	}

	pub fn info(&self) -> Info {
		let ptr = self.0.mmap.try_read().unwrap().as_ref()[0] as *const u8;
		Info {
			data: ptr,
			page_size: self.0.page_size as i64,
		}
	}

	/// Retrieves page from mmap
	pub(crate) fn page(&self, id: PGID) -> MappedRwLockReadGuard<Page> {
		let page_size = self.0.page_size;
		let pos = id as usize * page_size as usize;
		let mmap = self.0.mmap.read_recursive();
		RwLockReadGuard::map(mmap, |mmap| Page::from_buf(&mmap.as_ref()[pos..]))
	}

	fn page_in_buffer<'b>(&'a self, buf: &'b mut [u8], id: PGID) -> &'b mut Page {
		let page_size = self.0.page_size as usize;
		let pos = id as usize * page_size;
		let endpos = pos + page_size;
		Page::from_buf_mut(&mut buf[pos..endpos])
	}

	pub fn meta(&self) -> Result<Meta, Error> {
		let (page0, page1) = (self.page(0), self.page(1));
		let (mut meta0, mut meta1) = (page0.meta(), page1.meta());

		if meta1.txid > meta0.txid {
			std::mem::swap(&mut meta0, &mut meta1);
		}

		if meta0.validate().is_ok() {
			return Ok(meta0.clone());
		};

		if meta1.validate().is_ok() {
			return Ok(meta1.clone());
		};

		Err("nut::DB.meta(): invalid meta pages".into())
	}

	pub(crate) fn allocate(&mut self, count: u64, tx: &mut Tx) -> Result<OwnedPage, Error> {
		let mut p = if count == 1 {
			let mut pagepool = self.0.page_pool.lock();
			if let Some(p) = pagepool.pop() {
				p
			} else {
				OwnedPage::new(self.0.page_size)
			}
		} else {
			OwnedPage::new(self.0.page_size * count as usize)
		};
		p.overflow = count as u32 - 1;

		{
			let freelist_pid = self.0.freelist.write().allocate(count);
			if freelist_pid != 0 {
				p.id = freelist_pid;
				return Ok(p);
			};
		}

		p.id = tx.pgid();

		// Resize mmap() if we're at the end.
		let minsz = (p.id + 1 + count) * self.0.page_size as u64;
		let mmap_size = *self.0.mmap_size.lock() as u64;
		if minsz >= mmap_size {
			if let Err(e) = self.mmap(minsz) {
				return Err(e);
			};
		};

		// Move the page id high water mark.
		tx.set_pgid(tx.pgid() + count as u64)?;

		Ok(p)
	}

	pub(crate) fn grow(&mut self, mut size: u64) -> Result<(), Error> {
		if self.0.read_only {
			return Err(Error::DatabaseReadonly);
		};

		let file = self.0.file.try_write().unwrap();

		if file.get_ref().metadata().unwrap().len() >= size {
			return Ok(());
		}

		// If the data is smaller than the alloc size then only allocate what's needed.
		// Once it goes over the allocation size then allocate in chunks.
		{
			let mmapsize = self.0.mmap.try_read().unwrap().as_ref().len() as u64;
			if mmapsize < self.0.alloc_size {
				size = mmapsize;
			} else {
				size += self.0.alloc_size;
			};
		}

		file
			.get_ref()
			.allocate(size)
			.map_err(|_e| Error::ResizeFail)?;

		if !self.0.no_grow_sync {
			file.get_ref().sync_all().map_err(|_| "can't flush file")?;
		}

		*self.0.file_size.write() = file
			.get_ref()
			.metadata()
			.map_err(|_| "can't get metadata file")?
			.len();
		Ok(())
	}
}

impl Drop for DB {
	fn drop(&mut self) {
		if Arc::strong_count(&self.0) > 1 {
			return;
		};
		self.cleanup().unwrap();
	}
}

#[derive(Clone)]
pub(crate) struct WeakDB(Weak<DBInner>);

impl WeakDB {
	pub(crate) fn new() -> Self {
		Self(Weak::new())
	}

	pub(crate) fn upgrade(&self) -> Option<DB> {
		self.0.upgrade().map(DB)
	}

	pub(crate) fn from(db: &DB) -> Self {
		Self(Arc::downgrade(&db.0))
	}
}
