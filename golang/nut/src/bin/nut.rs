#![allow(clippy::cast_ptr_alignment, clippy::module_inception)]

use clap::{App, Arg, SubCommand};
use hexdump::{hexdump_iter, sanitize_byte};
use nut::{Bucket, BucketStats, DBBuilder, DB};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn is_numeric(i: String) -> Result<(), String> {
	str::parse::<usize>(&i)
		.map(|_| ())
		.map_err(|_| "Option must be numeric".to_string())
}

fn main() {
	let path_arg = Arg::with_name("path")
		.value_name("FILE")
		.short("p")
		.long("path")
		.help("path to database")
		.required(true)
		.takes_value(true)
		.validator(|ref p| {
			if std::path::Path::new(p).exists() {
				Ok(())
			} else {
				Err("Path not exists".to_string())
			}
		});

	let id_ard = Arg::with_name("id")
		.value_name("NUMBER")
		.short("i")
		.long("id")
		.help("id of the page")
		.takes_value(true)
		.required(true)
		.validator(is_numeric);

	let about = format!(
		"{}

homepage:
{}",
		DESCRIPTION, HOMEPAGE,
	);

	let mut app = App::new("Nut Database")
		.bin_name("nut")
		.version(VERSION)
		.author(AUTHORS)
		.long_about(&*about)
		.subcommands(vec![
		SubCommand::with_name("dump")
			.about("Dumps hex of the page")
			.args(&[
				id_ard.clone(),
				Arg::with_name("length")
					.value_name("NUMBER")
					.short("l")
					.long("length")
					.takes_value(true)
					.validator(is_numeric)
					.help("print no more than provided length"),
				path_arg.clone(),
			]),
		SubCommand::with_name("info")
			.about("Prints database info")
			.args(&[
				Arg::with_name("check")
					.short("c")
					.long("check")
					.possible_values(&["true", "false"])
					.default_value("true")
					.help("run db check"),
				path_arg.clone(),
			]),
		SubCommand::with_name("pages")
			.about("Prints a table of pages with their type (Meta, Leaf, Branch, Freelist)")
			.long_about(
				r#"Prints a table of pages with their type (meta, leaf, branch, freelist).
Leaf and branch pages will show a key count in the "items" column while the
freelist will show the number of free pages in the "items" column.

The "overflow" column shows the number of blocks that the page spills over
into. Normally there is no overflow but large keys and values can cause
a single page to take up multiple blocks."#,
			)
			.args(&[
				id_ard
					.clone()
					.multiple(true)
					.required(false)
					.help("if provided, info will be given only on requested ids"),
				path_arg.clone(),
			]),
		SubCommand::with_name("tree")
			.about("Prints buckets tree")
			.args(&[
				path_arg.clone(),
			]),
		SubCommand::with_name("check")
			.about(
				"Runs an exhaustive check to verify that all pages are accessible or are marked as freed.",
			)
			.args(&[path_arg.clone()]),
	]);

	let matches = app.clone().get_matches();

	let result = match matches.subcommand() {
		("dump", Some(args)) => dump(DumpOptions {
			path: PathBuf::from(args.value_of("path").unwrap()),
			id: args
				.value_of("id")
				.map(str::parse::<usize>)
				.unwrap()
				.unwrap(),
			len: args
				.value_of("length")
				.map(str::parse::<u64>)
				.map(|v| v.unwrap()),
		}),
		("info", Some(args)) => info(InfoOptions {
			path: PathBuf::from(args.value_of("path").unwrap()),
			check: args.value_of("check").unwrap() == "true",
		}),
		("pages", Some(args)) => pages(PagesOptions {
			path: PathBuf::from(args.value_of("path").unwrap()),
			ids: args
				.values_of("id")
				.map(|v| v.map(|v| str::parse::<usize>(v).unwrap()).collect()),
		}),
		("tree", Some(args)) => tree(TreeOptions {
			path: PathBuf::from(args.value_of("path").unwrap()),
		}),
		("check", Some(args)) => check(CheckOptions {
			path: PathBuf::from(args.value_of("path").unwrap()),
		}),
		_ => {
			app.print_long_help().unwrap();
			Ok(())
		}
	};
	if let Err(result) = result {
		eprintln!("Error: {}", result);
		std::process::exit(1);
	};
}

#[derive(Debug)]
struct DumpOptions {
	path: PathBuf,
	id: usize,
	len: Option<u64>,
}

fn dump(o: DumpOptions) -> Result<(), String> {
	let mut file = std::fs::File::open(&o.path).unwrap();
	let page_size = DB::get_meta(&mut file)?.page_size;
	let offset = u64::from(page_size) * o.id as u64;
	let meta = file
		.metadata()
		.map_err(|_| "Can't get file meta info")?
		.len();
	if offset > meta {
		return Err("ID out of bounds".to_string());
	};
	let mut overflowbuf = [0u8; 4];

	file
		.seek(SeekFrom::Start(offset + 12))
		.map_err(|_| "Can't seek file")?;
	file
		.read_exact(&mut overflowbuf)
		.map_err(|_| "Can't read file")?;;
	let overflow = unsafe { *(&overflowbuf[0] as *const u8 as *const u32) };
	let mut take = (u64::from(overflow) + 1) * u64::from(page_size);
	if let Some(len) = o.len {
		take = u64::min(take, len);
	};

	file
		.seek(SeekFrom::Start(offset))
		.map_err(|_| "Can't seek file")?;
	let mut bound = Read::by_ref(&mut file).take(take);

	let stdout = std::io::stdout();
	let mut stdout = stdout.lock();
	let mut obuf = [0u8; 1024];
	let newline = [0x0a];
	let mut written = 0;
	while let Ok(size) = bound.read(&mut obuf[written..]) {
		if size == 0 {
			if written > 0 {
				for line in hexdump_iter(&obuf[..written]) {
					stdout
						.write_all(&line.as_bytes())
						.map_err(|_| "Can't print output")?;
					stdout
						.write_all(&newline)
						.map_err(|_| "Can't print output")?;
				}
			}
			break;
		}
		written += size;
		if written == 1024 {
			for line in hexdump_iter(&obuf) {
				stdout
					.write_all(&line.as_bytes())
					.map_err(|_| "Can't print output")?;
				stdout
					.write_all(&newline)
					.map_err(|_| "Can't print output")?;
			}
		}
	}
	stdout.write(&[0x0a]).map_err(|_| "Can't print output")?;
	Ok(())
}

struct CheckOptions {
	path: PathBuf,
}

fn check(o: CheckOptions) -> Result<(), String> {
	use ansi_term::Color::{Green, Red};

	let db = DBBuilder::new(o.path).read_only(true).build()?;
	let tx = db.begin_tx()?;
	let receiver = tx.check();
	let mut errlen = 0;
	let stdout = std::io::stdout();
	let mut stdout = stdout.lock();
	for err in receiver {
		writeln!(stdout, "    {}", err).map_err(|_| "Can't write output")?;
		errlen += 1;
	}
	if errlen != 0 {
		return Err(Red.paint("Check failed").to_string());
	}
	writeln!(stdout, "{}", Green.paint("Everything ok")).map_err(|_| "Can't write output")?;
	Ok(())
}

struct InfoOptions {
	path: PathBuf,
	check: bool,
}

fn info(o: InfoOptions) -> Result<(), String> {
	{
		let db = DBBuilder::new(&o.path).read_only(true).build()?;
		let stdout = std::io::stdout();
		let mut stdout = stdout.lock();
		writeln!(&mut stdout, "Page size: {}", db.page_size()).map_err(|_| "Can't write output")?;
		writeln!(&mut stdout, "Freelist id: {}", db.meta()?.freelist)
			.map_err(|_| "Can't write output")?;

		{
			let tx = db.begin_tx()?;
			let mut stats = BucketStats {
				..Default::default()
			};
			let mut count = 0;
			let mut bucket_keys = vec![];
			tx.for_each(Box::new(|key, bucket| -> Result<(), String> {
				stats += bucket.unwrap().stats();
				bucket_keys.push(key.to_vec());
				count += 1;
				Ok(())
			}))?;
			writeln!(&mut stdout, "\nBucket keys ({}):", count).map_err(|_| "Can't write output")?;
			for key in bucket_keys {
				let sanitized: String = key.clone().into_iter().map(sanitize_byte).collect();
				writeln!(&mut stdout, "    {:<20} {:02X?}", sanitized, key)
					.map_err(|_| "Can't write output")?;
			}
			writeln!(&mut stdout, "\n{:#?}", stats).map_err(|_| "Can't write output")?;
		}

		if o.check {
			writeln!(&mut stdout, "\nCheck:").map_err(|_| "Can't write output")?;
		}
	}

	if o.check {
		check(CheckOptions { path: o.path })?;
	}

	Ok(())
}

struct PagesOptions {
	path: PathBuf,
	ids: Option<Vec<usize>>,
}

fn pages(o: PagesOptions) -> Result<(), String> {
	use ansi_term::Color::Green;
	let db = DBBuilder::new(&o.path).read_only(true).build()?;
	let tx = db.begin_tx()?;
	let stdout = std::io::stdout();
	let mut stdout = stdout.lock();

	writeln!(
		&mut stdout,
		"{}",
		Green.paint("ID     Type        Count   Overflow")
	)
	.map_err(|_| "Can't write output")?;
	writeln!(&mut stdout, "------ ----------- ------- --------").map_err(|_| "Can't write output")?;

	if let Some(ids) = &o.ids {
		for id in ids {
			match tx.page_info(*id) {
				Err(_) => writeln!(&mut stdout, "{:>6} error", id).map_err(|_| "Can't write output")?,
				Ok(None) => writeln!(&mut stdout, "{:>6} none", id).map_err(|_| "Can't write output")?,
				Ok(Some(p)) => writeln!(
					&mut stdout,
					"{:>6} {:<11} {:>7} {:>8}",
					p.id,
					&format!("{:?}", p.ptype),
					p.count,
					p.overflow_count
				)
				.map_err(|_| "Can't write output")?,
			}
		}
	} else {
		let mut id = 0;
		while let Some(p) = tx.page_info(id)? {
			writeln!(
				&mut stdout,
				"{:>6} {:<11} {:>7} {:>8}",
				p.id,
				&format!("{:?}", p.ptype),
				p.count,
				p.overflow_count
			)
			.map_err(|_| "Can't write output")?;
			id += 1;
		}
	}
	Ok(())
}

struct TreeOptions {
	path: PathBuf,
}

fn tree_writer<'a>(
	indent_level: usize,
	mut out: &mut std::io::StdoutLock<'a>,
	key: &[u8],
	bucket: Option<&Bucket>,
) -> Result<(), String> {
	if bucket.is_some() {
		let sanitized: String = key.to_vec().into_iter().map(sanitize_byte).collect();
		writeln!(
			&mut out,
			"{:<40} {:02X?}",
			format!("{}{}", " ".repeat(indent_level * 2), sanitized),
			key
		)
		.map_err(|_| "Can't write output")?;

		let bucket = bucket.unwrap();
		let buckets = bucket.buckets();

		for b_name in buckets {
			tree_writer(indent_level + 1, &mut out, &b_name, bucket.bucket(&b_name))?;
		}
	};

	Ok(())
}

fn tree(o: TreeOptions) -> Result<(), String> {
	let db = DBBuilder::new(&o.path).read_only(true).build()?;
	let stdout = std::io::stdout();
	let mut stdout = stdout.lock();

	writeln!(&mut stdout, "{:<40} binary", "key").map_err(|_| "Can't write output")?;
	writeln!(&mut stdout, "{} {}", "=".repeat(40), "=".repeat(20))
		.map_err(|_| "Can't write output")?;

	{
		let tx = db.begin_tx()?;
		tx.for_each(Box::new(
			|key: &[u8], bucket: Option<&Bucket>| -> Result<(), String> {
				tree_writer(0, &mut stdout, &key, bucket)?;

				Ok(())
			},
		))?;
	}

	Ok(())
}
