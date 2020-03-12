# Nut

#### Port of [Bolt DB](https://github.com/boltdb/bolt) in Rust

* **Compatible with Bolt's database format** and provides similar api.

  *Since Nut supports same format as Bolt there are known issues:*
	* Bolt uses Memmap to operate, there is no LRU or other cache, so if database doesn't fit in memory, then it's end of game, Nut will inevitably crash.
	* Data structures are written directly onto disk. No serialization takes place, so Nut doesn't understand db file from computer with different Endianness, for example.
* **Transaction based**. In case of error transaction will be rolled back.
* **Multiple readers, single writer.**
	Nut works just like Bolt: you can have one writer and multiple readers at a time, just be sure they run on different threads. Making writer and reader transaction in same thread will cause deadlock. Writer can write freely if memory map is have enough free pages, in other case it will be waiting for reading transactions to close.

# Usage

Package available at crates.io: https://crates.io/crates/nut

Documentation available at https://docs.rs/nut/0.1.0/nut/

Usual way is to `cargo build --release` in console. Docs available via `cargo doc --no-deps --open`. Api is very similar to Bolt if you are familiar with it.

# Examples

### Create db and put something

```
use nut::DBBuilder;

let mut db = DBBuilder::new("test.db").build().unwrap();
let mut tx = db.begin_rw_tx().unwrap();
{
	let mut flowers = tx.create_bucket(b"flowers").unwrap();
	// returns mutable reference to bucket,
		// which prevents of reborrowing tx

	flowers.put(
		b"iris",
		b"song by American alternative rock band Goo Goo Dolls".to_vec()
	).unwrap();
	flowers.put(
		b"binary data",
		vec![127, 127, 127, 127]
	).unwrap();

	{
		// you can create subbuckets as well
		let mut annuals = flowers
			.create_bucket_if_not_exists(b"annuals").unwrap();

		// api is basically same as for transaction
		annuals.put(
			b"corn",
			b"American nu metal band from Bakersfield".to_vec()
		).unwrap();
		// releasing subbucket
	}

	// releasing bucket to be able use tx again
}
// due to RAII tx will be automatically closed if no error occured,
// or rolled back if there was some.
// Additionally you can commit or rollback manually
tx.rollback().unwrap();
```

### Getting data back

```
use nut::DBBuilder;

let mut db = DBBuilder::new("test.db").build().unwrap();

// creating read only transaction
// read only ransaction will be automatically rolled back
let mut tx = db.begin_tx().unwrap();

// getting bucket
let flowers = tx.bucket(b"flowers").unwrap();
let data = flowers.get(b"iris").unwrap();
assert_eq!(
	data,
	&b"song by American alternative rock band Goo Goo Dolls"[..]
);
```

### Getting available buckets

```
use nut::DBBuilder;

let mut db = DBBuilder::new("test.db").build().unwrap();
let mut tx = db.begin_tx().unwrap();

{
	// .buckets() available to conveniently retrieve all buckets keys
	let bucket_names = tx.buckets(); // returns Vec<Vec<u8>>

	// bucket key is any binary data, not only string
	assert_eq!(bucket_names, vec!["flowers".as_bytes().to_vec()]);
}

{
	// additionally there is .cursor() method
	// that returns Cursor struct,
	// which is able to iterate through bucket contents
	let cursor = tx.cursor();

	assert_eq!(
		&cursor.first().unwrap().value.unwrap(),
		&"flowers".as_bytes()
	);
}
```

# Nut Bin

Crate also provides `nut` binary which is helpful to inspect database file in various ways. It can be found after `cargo build --release` in `./target/release/nut`.

Excerpt from man:

```
USAGE:
    nut [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check    Runs an exhaustive check to verify that all pages are accessible or are marked as freed.
    dump     Dumps hex of the page
    help     Prints this message or the help of the given subcommand(s)
    info     Prints database info
    pages    Prints a table of pages with their type (Meta, Leaf, Branch, Freelist)
    tree     Prints buckets tree
```

# Disclaimer

I'm not planning to actively mantain this project since it just a hobby project to check out Rust, and there are better alternatives in terms of performance.

-------

MIT License