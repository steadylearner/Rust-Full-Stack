//! # Nut
//!
//! Key-value embed database, which is basically port of Bolt DB
//!
//! # Examples
//! ## Create db and put something
//! ```no_run
//! use nut::DBBuilder;
//!
//! let mut db = DBBuilder::new("test.db").build().unwrap();
//! let mut tx = db.begin_rw_tx().unwrap();
//! {
//! 	let mut flowers = tx.create_bucket(b"flowers").unwrap();
//! 	// returns mutable reference to bucket,
//!		// which prevents of reborrowing tx
//!
//! 	flowers.put(
//! 		b"iris",
//! 		b"song by American alternative rock band Goo Goo Dolls".to_vec()
//! 	).unwrap();
//! 	flowers.put(
//! 		b"binary data",
//! 		vec![127, 127, 127, 127]
//! 	).unwrap();
//!
//! 	{
//! 		// you can create subbuckets as well
//! 		let mut annuals = flowers
//! 			.create_bucket_if_not_exists(b"annuals").unwrap();
//!
//! 		// api is basically same as for transaction
//! 		annuals.put(
//! 			b"corn",
//! 			b"American nu metal band from Bakersfield".to_vec()
//! 		).unwrap();
//! 		// releasing subbucket
//! 	}
//!
//! 	// releasing bucket to be able use tx again
//! }
//! // due to RAII tx will be automatically closed if no error occured,
//! // or rolled back if there was some.
//! // Additionally you can commit or rollback manually
//! tx.rollback().unwrap();
//! ```
//!
//! ## Getting data back
//! ```no_run
//! use nut::DBBuilder;
//!
//! let mut db = DBBuilder::new("test.db").build().unwrap();
//!
//! // creating read only transaction
//! // read only ransaction will be automatically rolled back
//! let mut tx = db.begin_tx().unwrap();
//!
//! // getting bucket
//! let flowers = tx.bucket(b"flowers").unwrap();
//! let data = flowers.get(b"iris").unwrap();
//! assert_eq!(
//! 	data,
//! 	&b"song by American alternative rock band Goo Goo Dolls"[..]
//! );
//! ```
//!
//! ## Getting available buckets
//! ```no_run
//! use nut::DBBuilder;
//!
//! let mut db = DBBuilder::new("test.db").build().unwrap();
//! let mut tx = db.begin_tx().unwrap();
//!
//! {
//! 	// .buckets() available to conveniently retrieve all buckets keys
//! 	let bucket_names = tx.buckets(); // returns Vec<Vec<u8>>
//!
//! 	// bucket key is any binary data, not only string
//! 	assert_eq!(bucket_names, vec!["flowers".as_bytes().to_vec()]);
//! }
//!
//! {
//! 	// additionally there is .cursor() method
//! 	// that returns Cursor struct,
//! 	// which is able to iterate through bucket contents
//! 	let cursor = tx.cursor();
//!
//! 	assert_eq!(
//! 		&cursor.first().unwrap().value.unwrap(),
//! 		&"flowers".as_bytes()
//! 	);
//! }
//! ```

#![allow(clippy::cast_ptr_alignment, clippy::module_inception)]

#[macro_use]
extern crate memoffset;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod concurrency_tests;

mod bucket;
mod consts;
mod db;
mod errors;
mod freelist;
mod meta;
mod node;
mod page;
mod tx;
mod utils;

pub use bucket::{Bucket, BucketStats, Cursor, CursorItem};
pub use consts::Flags;
pub use db::{CheckMode, DBBuilder, RWTxGuard, Stats as DBStats, TxGuard, DB};
pub use errors::Error;
pub use tx::{Tx, TxStats};
