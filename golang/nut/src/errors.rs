#![allow(clippy::zero_prefixed_literal)]

use std::error::Error as RError;
use std::fmt;
use std::io::Error as IOError;
use std::sync::PoisonError;
use std::time::SystemTimeError;
// use parking_lot;

/// These error can be returned when opening or calling methods on a DB, Transaction etc...
#[derive(Debug, Clone)]
pub enum Error {
	// database related
	DatabaseGone,
	DatabaseClosed,
	DatabaseOpen,
	DatabaseReadonly,
	FileTooSmall,
	NoMeta,
	Invalid,
	VersionMismatch,
	Checksum,
	Timeout,
	ResizeFail,
	SyncFail,

	// transaction related
	TxReadonly,
	TxGone,
	TxClosed,
	TxManaged,
	TxUnmanaged,

	// bucket related
	IncompatibleValue,
	AllocationFailed,
	BucketNotFound,
	BucketExists,
	NameRequired,
	KeyRequired,
	KeyTooLarge,
	ValueTooLarge,

	ReadInProgress,
	WriteInProgress,

	// cursor related errors
	StackEmpty,
	ExpectedLeaf,
	ExpectedBranch,
	TraverseFailed,

	CheckFail(Vec<String>),
	NoneError(String),
	LockError(String),
	Unexpected(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let desc = match self {
			Error::DatabaseGone => "database gone".to_string(),
			Error::DatabaseClosed => "database closed".to_string(),
			Error::DatabaseOpen => "database already opened".to_string(),
			Error::DatabaseReadonly => "database is read-only".to_string(),
			Error::FileTooSmall => "file is too small".to_string(),
			Error::NoMeta => "no meta page found".to_string(),
			Error::Invalid => "invalid database".to_string(),
			Error::VersionMismatch => "version mismatch".to_string(),
			Error::Checksum => "checksum error".to_string(),
			Error::Timeout => "timeout".to_string(),
			Error::ResizeFail => "resize failed".to_string(),
			Error::SyncFail => "sync failed".to_string(),

			Error::TxReadonly => "tx is read-only".to_string(),
			Error::TxGone => "tx gone".to_string(),
			Error::TxClosed => "tx closed".to_string(),
			Error::TxUnmanaged => "tx is unmanaged".to_string(),
			Error::TxManaged => "tx in use".to_string(),

			Error::IncompatibleValue => "incompatible value".to_string(),
			Error::AllocationFailed => "allocation failed".to_string(),
			Error::BucketNotFound => "bucket not found".to_string(),
			Error::BucketExists => "bucket already exists".to_string(),
			Error::NameRequired => "bucket name required".to_string(),
			Error::KeyRequired => "key required".to_string(),
			Error::KeyTooLarge => "key too large".to_string(),
			Error::ValueTooLarge => "value too large".to_string(),

			Error::ReadInProgress => "database locked on read".to_string(),
			Error::WriteInProgress => "database locked on write".to_string(),

			Error::StackEmpty => "stack empty".to_string(),
			Error::ExpectedLeaf => "expected leaf node".to_string(),
			Error::ExpectedBranch => "expected branch node".to_string(),
			Error::TraverseFailed => "traversion failed".to_string(),

			Error::CheckFail(s) => format!("check failure:\n  {}", s.join("\n  ")),
			Error::NoneError(s) => format!("value is None: {}", s),
			Error::LockError(s) => format!("lock failed: {}", s),
			Error::Unexpected(s) => format!("unexpected: {}", s),
		};
		write!(f, "{}", desc)
	}
}

impl Error {
	fn code(&self) -> u32 {
		match self {
			Error::DatabaseGone => 01_000_000,
			Error::DatabaseClosed => 01_000_001,
			Error::DatabaseOpen => 01_000_002,
			Error::DatabaseReadonly => 01_000_003,
			Error::FileTooSmall => 01_000_004,
			Error::NoMeta => 01_000_005,
			Error::Invalid => 01_000_006,
			Error::VersionMismatch => 01_000_007,
			Error::Checksum => 01_000_008,
			Error::Timeout => 01_000_009,
			Error::ResizeFail => 01_000_010,
			Error::SyncFail => 01_000_011,

			Error::TxReadonly => 02_000_000,
			Error::TxGone => 02_000_001,
			Error::TxClosed => 02_000_002,
			Error::TxUnmanaged => 02_000_003,
			Error::TxManaged => 02_000_004,

			Error::IncompatibleValue => 03_000_000,
			Error::AllocationFailed => 03_000_001,
			Error::BucketNotFound => 03_000_002,
			Error::BucketExists => 03_000_003,
			Error::NameRequired => 03_000_004,
			Error::KeyRequired => 03_000_005,
			Error::KeyTooLarge => 03_000_006,
			Error::ValueTooLarge => 03_000_007,

			Error::ReadInProgress => 04_000_000,
			Error::WriteInProgress => 04_000_001,

			Error::StackEmpty => 05_000_000,
			Error::ExpectedLeaf => 05_000_001,
			Error::ExpectedBranch => 05_000_002,
			Error::TraverseFailed => 06_000_003,

			Error::CheckFail(_) => 07_000_000,
			Error::NoneError(_) => 08_000_000,
			Error::LockError(_) => 09_000_000,
			Error::Unexpected(_) => 10_000_000,
		}
	}
}

impl PartialEq for Error {
	fn eq(&self, o: &Self) -> bool {
		match self {
			Error::CheckFail(_) | Error::NoneError(_) | Error::LockError(_) | Error::Unexpected(_) => {
				format!("{}", self) == format!("{}", o)
			}
			_ => self.code() == o.code(),
		}
	}
}

impl<T> From<PoisonError<T>> for Error {
	fn from(e: PoisonError<T>) -> Self {
		Error::LockError(e.to_string())
	}
}

impl From<SystemTimeError> for Error {
	fn from(e: SystemTimeError) -> Self {
		Error::Unexpected(e.to_string())
	}
}

impl From<String> for Error {
	fn from(e: String) -> Self {
		Error::Unexpected(e.to_string())
	}
}

impl From<&str> for Error {
	fn from(e: &str) -> Self {
		Error::Unexpected(e.to_string())
	}
}

impl From<IOError> for Error {
	fn from(e: IOError) -> Self {
		Error::Unexpected(e.to_string())
	}
}

impl From<Error> for String {
	fn from(e: Error) -> Self {
		format!("{}", e)
	}
}

impl RError for Error {}
