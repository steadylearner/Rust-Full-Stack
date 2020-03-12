use crate::db::tests::db_mock;

#[test]
fn seek_none() {
	let mut db = db_mock().build().unwrap();
	let mut tx = db.begin_rw_tx().unwrap();
	drop(tx.create_bucket(b"blub").unwrap());
	let c = tx.cursor();
	let item = c.seek(b"foo");
	assert!(item.is_ok());
	assert!(item.unwrap().is_none());
}

#[test]
fn seek_some() {
	let mut db = db_mock().build().unwrap();
	let mut tx = db.begin_rw_tx().unwrap();
	drop(tx.create_bucket(b"foo").unwrap());
	let c = tx.cursor();
	let item = c.seek(b"foo");
	assert!(item.is_ok());
	assert!(item.unwrap().is_some());
}

#[test]
fn values_cursor() {
	let mut db = db_mock().build().unwrap();
	let mut tx = db.begin_rw_tx().unwrap();
	{
		let mut bucket = tx.create_bucket(b"bucket").unwrap();
		bucket.put(b"petr", b"rachmaninov".to_vec()).unwrap();
		bucket.put(b"robert", b"plant".to_vec()).unwrap();
		bucket.put(b"ziggy", b"stardust".to_vec()).unwrap();
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.first().unwrap().key.unwrap(), b"petr");
		}
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.first().unwrap().key.unwrap(), b"petr");
			assert_eq!(cursor.next().unwrap().key.unwrap(), b"robert");
		}
		{
			let mut key_names = vec![];
			let cursor = bucket.cursor().unwrap();
			{
				let item = cursor.first().unwrap();
				key_names.push(item.key.unwrap().to_vec());
			}
			loop {
				let item = cursor.next().unwrap();
				if item.is_none() {
					break;
				}
				key_names.push(item.key.unwrap().to_vec());
			}
			assert_eq!(key_names.len(), 3);
			assert!(key_names.contains(&b"petr".to_vec()));
			assert!(key_names.contains(&b"robert".to_vec()));
			assert!(key_names.contains(&b"ziggy".to_vec()));
		}

		// backwards
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.last().unwrap().key.unwrap(), b"ziggy");
		}
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.last().unwrap().key.unwrap(), b"ziggy");
			assert_eq!(cursor.prev().unwrap().key.unwrap(), b"robert");
		}
		{
			let mut key_names = vec![];
			let cursor = bucket.cursor().unwrap();
			{
				let item = cursor.last().unwrap();
				key_names.push(item.key.unwrap().to_vec());
			}
			loop {
				let item = cursor.prev().unwrap();
				if item.is_none() {
					break;
				}
				key_names.push(item.key.unwrap().to_vec());
			}
			assert_eq!(key_names.len(), 3);
			assert!(key_names.contains(&b"petr".to_vec()));
			assert!(key_names.contains(&b"robert".to_vec()));
			assert!(key_names.contains(&b"ziggy".to_vec()));
		}
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.last().unwrap().key.unwrap(), b"ziggy");
			assert_eq!(cursor.prev().unwrap().key.unwrap(), b"robert");
			assert_eq!(cursor.prev().unwrap().key.unwrap(), b"petr");
			assert_eq!(cursor.next().unwrap().key.unwrap(), b"robert");

			assert_eq!(cursor.first().unwrap().key.unwrap(), b"petr");
			assert_eq!(cursor.next().unwrap().key.unwrap(), b"robert");
			assert_eq!(cursor.next().unwrap().key.unwrap(), b"ziggy");
			assert_eq!(cursor.prev().unwrap().key.unwrap(), b"robert");
		}
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.first().unwrap().key.unwrap(), b"petr");
			assert_eq!(cursor.prev().unwrap().key, None);
			assert_eq!(cursor.prev().unwrap().key, None);
		}
		{
			let cursor = bucket.cursor().unwrap();
			assert_eq!(cursor.last().unwrap().key.unwrap(), b"ziggy");
			assert_eq!(cursor.next().unwrap().key, None);
			assert_eq!(cursor.next().unwrap().key, None);
		}
	}
}

#[test]
fn bucket_cursor() {
	let mut db = db_mock().build().unwrap();
	let mut tx = db.begin_rw_tx().unwrap();
	{
		let mut bucket = tx.create_bucket(b"bucket").unwrap();
		bucket.put(b"key", b"value".to_vec()).unwrap();
		bucket.put(b"keys", b"value".to_vec()).unwrap();
	}
	{
		let mut bucket = tx.create_bucket(b"another bucket").unwrap();
		bucket.put(b"key", b"value".to_vec()).unwrap();
		bucket.put(b"keys", b"value".to_vec()).unwrap();
	}
	{
		let mut bucket_names = vec![];
		let cursor = tx.cursor();
		{
			let item = cursor.first().unwrap();
			bucket_names.push(item.key.unwrap().to_vec());
		}
		loop {
			let item = cursor.next().unwrap();
			if item.is_none() {
				break;
			}
			bucket_names.push(item.key.unwrap().to_vec());
		}
		assert_eq!(bucket_names.len(), 2);
		assert!(bucket_names.contains(&b"bucket".to_vec()));
		assert!(bucket_names.contains(&b"another bucket".to_vec()));
	}
}
