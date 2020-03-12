use crate::db::tests::db_mock;
use std::thread;
use std::time::Duration;

fn perform_read(number_of_threads: usize, number_of_values: usize) {
	let mut db = db_mock().build().unwrap();
	{
		let mut tx = db.begin_rw_tx().unwrap();
		let mut bucket = tx.create_bucket(b"bucket").unwrap();
		for i in 10..10 + number_of_values {
			bucket
				.put(
					format!("{}", i).as_bytes(),
					format!("{}", i * 100000).into(),
				)
				.unwrap();
		}
	}

	let mut handles = vec![];
	for _ in 0..number_of_threads {
		let db = db.clone();
		handles.push(thread::spawn(move || {
			db.view(Box::new(|tx| {
				let bucket = tx.bucket(b"bucket").unwrap();
				for i in 10..10 + number_of_values {
					let expected = format!("{}", i * 100000);
					let key = format!("{}", i);
					let value = bucket.get(&key.as_bytes()).unwrap();
					assert_eq!(expected.as_bytes(), value);
				}
				Ok(())
			}))
			.unwrap();
		}));
	}

	for handle in handles {
		handle.join().unwrap();
	}
}

#[test]
fn same_bucket_read_2_2() {
	perform_read(2, 2);
}

#[test]
fn same_bucket_read_2_5() {
	perform_read(2, 5);
}

#[test]
fn same_bucket_read_5_10() {
	perform_read(5, 10);
}

#[test]
fn same_bucket_read_10_10() {
	perform_read(10, 10);
}

#[test]
fn same_bucket_read_10_50() {
	perform_read(10, 50);
}

#[test]
fn same_bucket_read_20_100() {
	perform_read(20, 100);
}

#[test]
fn same_bucket_read_10_500() {
	perform_read(10, 500);
}

#[test]
#[ignore]
fn same_bucket_read_10_1500() {
	perform_read(10, 1500);
}

#[test]
#[ignore]
fn same_bucket_read_20_1500() {
	perform_read(10, 1500);
}

#[test]
#[ignore]
fn same_bucket_read_10_5500() {
	perform_read(10, 5500);
}

#[test]
#[ignore]
fn same_bucket_read_20_5500() {
	perform_read(10, 5500);
}

#[test]
fn multiple_writers_multiple_readers() {
	use crate::CheckMode;
	let n_runs = 5;
	let n_values = 1400;
	let n_threads = 10;
	let n_wvalues = 200;
	let n_wthreads = 5;

	let mut db = db_mock()
		// Strong because Paranoid check can fail in case
		// of parallel read/write transaction on after read check
		.checkmode(CheckMode::STRONG)
		// big initial mmap size to prevent read transaction block of re-mmap
		.initial_mmap_size(4096 * 40)
		.build()
		.unwrap();

	{
		let mut tx = db.begin_rw_tx().unwrap();
		let _ = tx.create_bucket(b"bucket").unwrap();
	}

	for _ in 1..=n_runs {
		let mut handles = vec![];

		for n in 0..n_wthreads {
			// writer
			let mut db = db.clone();
			handles.push(thread::spawn(move || {
				let mut tx = db.begin_rw_tx().unwrap();
				let mut bucket = tx.bucket_mut(b"bucket").unwrap();
				for i in 0..n_wvalues {
					bucket
						.put(
							format!("{}", i).as_bytes(),
							format!("{}{v}{v}{v}{v}{v}{v}{v}", n, v = i).into(),
						)
						.unwrap();
					thread::sleep(Duration::from_nanos(10));
				}
			}));
		}

		for _ in 0..n_threads {
			// reader
			let db = db.clone();
			handles.push(thread::spawn(move || {
				let tx = db.begin_tx().unwrap();
				let bucket = tx.bucket(b"bucket").unwrap();
				for i in 0..n_values {
					let _ = bucket.get(format!("{}", i).as_bytes());
					thread::sleep(Duration::from_nanos(15));
				}
			}));
		}

		for handle in handles {
			handle.join().unwrap();
		}
	}
}
