use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::env::temp_dir;
use std::iter;
use std::path::PathBuf;

pub(crate) fn random_string(len: usize) -> String {
	let mut rng = thread_rng();
	iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.take(len)
		.collect::<String>()
}

pub(crate) fn temp_file() -> PathBuf {
	temp_dir().join(format!("{}.nut.db", random_string(32)))
}
