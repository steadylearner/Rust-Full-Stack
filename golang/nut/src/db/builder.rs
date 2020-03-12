use std::path::{Path, PathBuf};
use std::time::Duration;

use super::db::{CheckMode, DB};
use crate::consts::{DEFAULT_MAX_BATCH_DELAY, DEFAULT_MAX_BATCH_SIZE};
use crate::errors::Error;

/// Options that can be set when opening a database.
pub(super) struct Options {
	pub(super) no_grow_sync: bool,
	pub(super) read_only: bool,
	pub(super) initial_mmap_size: usize,
	pub(super) autoremove: bool,
	pub(super) checkmode: CheckMode,
	pub(super) max_batch_delay: Duration,
	pub(super) max_batch_size: usize,
}

/// Struct to construct database
///
/// # Example
///
/// ```no_run
/// use nut::DBBuilder;
///
/// let db = DBBuilder::new("./test.db").read_only(true).build();
/// ```
pub struct DBBuilder {
	path: PathBuf,
	no_grow_sync: bool,
	read_only: bool,
	initial_mmap_size: usize,
	autoremove: bool,
	checkmode: CheckMode,
	max_batch_delay: Duration,
	max_batch_size: usize,
}

impl DBBuilder {
	/// Creates new Builder,
	/// path required.
	pub fn new<P: AsRef<Path>>(path: P) -> Self {
		Self {
			path: path.as_ref().to_owned(),
			no_grow_sync: false,
			read_only: false,
			initial_mmap_size: 0,
			autoremove: false,
			checkmode: CheckMode::NO,
			max_batch_delay: DEFAULT_MAX_BATCH_DELAY,
			max_batch_size: DEFAULT_MAX_BATCH_SIZE,
		}
	}

	/// Path to db file
	pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
		self.path = path.as_ref().to_owned();
		self
	}

	/// Sets the DB.no_grow_sync flag before memory mapping the file.
	///
	/// Default: false
	pub fn no_grow_sync(mut self, v: bool) -> Self {
		self.no_grow_sync = v;
		self
	}

	/// Open database in read-only mode.
	///
	/// If database opened in read only mode file will be locked shared
	/// and exclusively otherwise.
	///
	/// Default: false
	pub fn read_only(mut self, v: bool) -> Self {
		self.read_only = v;
		self
	}

	/// Initial mmap size of the database
	///
	/// in bytes. Read transactions won't block write transaction
	///
	/// if the initial_mmap_size is large enough to hold database mmap
	/// size. (See DB.begin for more information)
	///
	/// If = 0, the initial map size is size of first 4 pages.
	///
	/// If initial_mmap_size is smaller than the previous database size,
	/// it takes no effect.
	///
	/// Default: 0 (mmap will be equal to 4 page sizes)
	pub fn initial_mmap_size(mut self, v: usize) -> Self {
		self.initial_mmap_size = v;
		self
	}

	/// Defines whether db file will be removed after db close
	///
	/// Default: false
	pub fn autoremove(mut self, v: bool) -> Self {
		self.autoremove = v;
		self
	}

	/// Defines database checking mode
	///
	/// Default: CheckMode::No
	pub fn checkmode(mut self, v: CheckMode) -> Self {
		self.checkmode = v;
		self
	}

	/// Defines batch delay time
	///
	/// Default: 10 seconds
	pub fn batch_delay(mut self, v: Duration) -> Self {
		self.max_batch_delay = v;
		self
	}

	/// Defines max batch size.
	/// If size equals 0, size is unlimited
	///
	/// Default: 1000
	pub fn batch_size(mut self, v: usize) -> Self {
		self.max_batch_size = v;
		self
	}

	/// Builds and returns DB instance
	pub fn build(self) -> Result<DB, Error> {
		let options = Options {
			no_grow_sync: self.no_grow_sync,
			read_only: self.read_only,
			initial_mmap_size: self.initial_mmap_size,
			autoremove: self.autoremove,
			checkmode: self.checkmode,
			max_batch_delay: self.max_batch_delay,
			max_batch_size: self.max_batch_size,
		};
		DB::open(self.path, options)
	}
}
