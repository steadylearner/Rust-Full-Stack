// Reference https://github.com/svmk/rust-sitemap/blob/master/tests/test_write_sitemap.rs

extern crate chrono;
extern crate console;
extern crate diesel;
extern crate sitemap;
extern crate sl_lib;

use console::Style;
use diesel::prelude::*;

use std::fs;

use chrono::prelude::Utc;
use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use sitemap::writer::SiteMapWriter;

use sl_lib::models::*;
use sl_lib::*;

// use match and console?

fn main() -> std::io::Result<()> {
    // let green = Style::new().green();
    // let yellow = Style::new().yellow();
    // let cyan = Style::new().cyan();
    let bold = Style::new().bold();

    // println!("{}", "What is author_id?");

    use crate::schema::posts::dsl::*;
    let connection = init_pool().get().unwrap();

    let post_results = posts
        .filter(published.eq(true))
        .order(created_at.desc())
        .load::<Post>(&*connection)
        .expect("Error loading posts");

    println!(
        "\nWrite sitemap for {} posts",
        bold.apply_to(post_results.len())
    );

    let mut output = Vec::<u8>::new();;
    {
        let sitemap_writer = SiteMapWriter::new(&mut output);
        let mut urlwriter = sitemap_writer.start_urlset().expect("Can't write the file");
        let date = DateTime::from_utc(
            NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            FixedOffset::east(0),
        );
        let url_entry = UrlEntry::builder()
            .loc("http://www.example.com/index.html")
            .changefreq(ChangeFreq::Daily)
            .priority(0.2)
            .lastmod(date)
            .build()
            .expect("valid");
        urlwriter.url(url_entry).expect("Can't write the file");
        let date1 = DateTime::from_utc(
            NaiveDate::from_ymd(2016, 7, 18).and_hms(9, 10, 11),
            FixedOffset::east(0),
        );
        let url_entry = UrlEntry::builder()
            .loc("http://www.example.com/other.html")
            .changefreq(ChangeFreq::Monthly)
            .priority(0.1)
            .lastmod(date1)
            .build()
            .expect("valid");
        urlwriter.url(url_entry).expect("Can't write the file");
        let sitemap_writer = urlwriter.end().expect("close the urlset block");

        let mut sitemap_index_writer = sitemap_writer
            .start_sitemapindex()
            .expect("start sitemap index tag");
        let sitemap_entry = SiteMapEntry::builder()
            .loc("http://www.example.com/other_sitemap.xml")
            .lastmod(date1)
            .build()
            .expect("valid");
        sitemap_index_writer
            .sitemap(sitemap_entry)
            .expect("Can't write the file");
        sitemap_index_writer.end().expect("close sitemap block");
    }

    println!("{:#?}", std::str::from_utf8(&output));

    fs::write("sitemap.xml", &output)?;

    Ok(())
}
