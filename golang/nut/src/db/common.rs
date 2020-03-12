use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use crate::errors::Error;
use crate::tx::Tx;

use super::WeakDB;

pub(super) struct Call {
	h: Arc<Box<dyn Fn(&mut Tx) -> Result<(), String> + Send>>,
	err: mpsc::Sender<Result<(), String>>,
}

impl Call {
	pub(super) fn new(
		func: Arc<Box<dyn Fn(&mut Tx) -> Result<(), String> + Send>>,
		err_ch: mpsc::Sender<Result<(), String>>,
	) -> Self {
		Self {
			h: func,
			err: err_ch,
		}
	}
}

struct BatchInner {
	db: WeakDB,
	calls_len: usize,
	ran: std::sync::atomic::AtomicBool,
	pub(super) calls: Mutex<Vec<Call>>,
}

#[derive(Clone)]
pub(crate) struct Batch(Arc<BatchInner>);

unsafe impl Send for Batch {}
unsafe impl Sync for Batch {}

impl Batch {
	pub(super) fn new(db: WeakDB, calls_len: usize, delay: Duration) -> Self {
		let batch = Batch(Arc::new(BatchInner {
			db,
			calls_len,
			ran: AtomicBool::new(false),
			calls: Mutex::new(Vec::new()),
		}));

		let mut bc = batch.clone();
		thread::spawn(move || {
			thread::sleep(delay);
			bc.trigger();
		});

		batch
	}

	pub(super) fn closed(&self) -> bool {
		self.0.ran.load(Ordering::Acquire)
	}

	pub(super) fn push(&mut self, call: Call) -> Result<(), Error> {
		if self.0.ran.load(Ordering::Acquire) {
			return Err("batch already run".into());
		};
		let calls_len = {
			let mut calls = self.0.calls.try_lock_for(Duration::from_secs(10)).unwrap();
			calls.push(call);
			calls.len()
		};
		if self.0.calls_len != 0 && calls_len >= self.0.calls_len {
			let mut bc = self.clone();
			thread::spawn(move || {
				bc.trigger();
			});
		};
		Ok(())
	}

	/// trigger runs the batch if it hasn't already been run.
	pub(super) fn trigger(&mut self) {
		use std::sync::atomic::Ordering::SeqCst;
		if let Ok(false) = self.0.ran.compare_exchange(false, true, SeqCst, SeqCst) {
			self.run();
		}
	}

	/// run performs the transactions in the batch and communicates results
	/// back to DB.Batch.
	fn run(&mut self) {
		let mut db = self.0.db.upgrade().unwrap();
		db.0.batch.lock().take();

		let mut calls = self.0.calls.try_lock().unwrap();

		while calls.len() > 0 {
			let mut last_call_id = 0;
			if let Err(e) = db.update(Box::new(|tx| {
				for (index, call) in calls.iter().enumerate() {
					last_call_id = index;
					(call.h)(tx)?
				}
				Ok(())
			})) {
				let failed_call = calls.remove(last_call_id);
				failed_call.err.send(Err(format!("{}", e))).unwrap();
				continue;
			}
			{
				for call in &*calls {
					call.err.send(Ok(())).unwrap();
				}
				break;
			}
		}
	}
}
