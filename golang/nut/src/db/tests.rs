use super::{CheckMode, DBBuilder};
use crate::consts::MAGIC;
use crate::test_utils::temp_file;

pub(crate) fn db_mock<'a>() -> DBBuilder {
	DBBuilder::new(temp_file())
		.autoremove(true)
		.read_only(false)
		.checkmode(CheckMode::PARANOID)
}

#[test]
fn open() {
	let db = db_mock().build().unwrap();
	assert_eq!(db.page_size(), page_size::get());
	assert_eq!(db.meta().unwrap().version, 2);
	assert_eq!(db.meta().unwrap().magic, MAGIC);
	assert_eq!(db.meta().unwrap().root.root, 3);
}

#[test]
fn open_existing() {
	let db = DBBuilder::new("./test_data/remark.db")
		.read_only(true)
		.build()
		.unwrap();
	assert_eq!(db.page_size(), page_size::get());
	assert_eq!(db.meta().unwrap().version, 2);
	assert_eq!(db.meta().unwrap().magic, MAGIC);
	assert_eq!(db.meta().unwrap().root.root, 9);
	{
		let tx = db.begin_tx().unwrap();
		let buckets = tx.buckets();
		assert_eq!(buckets.len(), 7);
		tx.check_sync().unwrap();
	}
}

#[test]
fn panic_while_update() {
	let mut db = db_mock().build().unwrap();

	// create bucket
	db.update(Box::new(|tx| {
		let _ = tx.create_bucket(b"exists").unwrap();
		Ok(())
	}))
	.unwrap();

	// ensure bucket exists
	db.view(Box::new(|tx| {
		assert!(tx.bucket(b"exists").is_ok());
		Ok(())
	}))
	.unwrap();

	// panicking
	db.update(Box::new(|tx| {
		let _ = tx.create_bucket(b"not exists").unwrap();
		panic!("oh shi!");
	}))
	.unwrap_err();

	// ensure transaction wasn't commited
	db.view(Box::new(|tx| {
		assert!(tx.bucket(b"exists").is_ok());
		assert!(tx.bucket(b"not exists").is_err());
		Ok(())
	}))
	.unwrap();
}

#[test]
fn batch() {
	use std::time::Duration;

	let db = db_mock()
		.batch_delay(Duration::from_secs(2))
		.batch_size(3)
		.build()
		.unwrap();

	let mut handles = vec![];
	for i in 0..10 {
		let mut db = db.clone();
		handles.push(std::thread::spawn(move || {
			db.batch(Box::new(move |tx| {
				let _ = tx.create_bucket(format!("{}bubu", i).as_bytes()).unwrap();
				Ok(())
			}))
			.unwrap();
		}));
	}

	for h in handles {
		h.join().unwrap();
	}

	db.view(Box::new(|tx| {
		for i in 0..10 {
			let _ = tx.bucket(format!("{}bubu", i).as_bytes()).unwrap();
		}
		Ok(())
	}))
	.unwrap();
}
