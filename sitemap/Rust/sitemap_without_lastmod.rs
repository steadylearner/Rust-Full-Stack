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
// use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap::structs::{ChangeFreq, UrlEntry};
// use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
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

        // [1] Start Sitemap, You can't advance without it. If you want more settings. Replace the code made by the xml below

        // <?xml version="1.0" encoding="UTF-8"?> <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        // xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"
        // xmlns:xhtml="http://www.w3.org/1999/xhtml"
        // xmlns:mobile="http://www.google.com/schemas/sitemap-mobile/1.0"
        // xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"
        // xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">

        let mut urlwriter = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

        // [2] Utc::now().naive_utc(); doesn't work here.
        //     You may not want to set it manually everytime.

        let date = Utc::now().naive_utc();
        // println!("{}", &date); 2019-03-14

        // 1. split with empty space
        // 2. split with "-"
        // 3. pass variables with conditional statements inside from_ymd

        // let date = DateTime::from_utc(
        //     NaiveDate::from_ymd(2019, 13, 3).and_hms(0, 0, 0),
        //     FixedOffset::east(0),
        // );

        // Setting the entry point manually, It will depend on your project.
        // Just define once.

        let home_entry = UrlEntry::builder()
            .loc("http://www.steadylearner.com")
            .changefreq(ChangeFreq::Monthly)
            // .lastmod(date) // priority is also removed for some search engines ignore it.
            .build()
            .expect("valid");
        urlwriter.url(home_entry).expect("Unable to write url");

        // Iterate over normal paths with vector written manually.

        let static_routes: Vec<&str> = vec!["about", "video", "blog", "code", "image", "slideshow"];

        for route in static_routes.iter() {
            let static_url = format!("http://www.steadylearner.com/{}", route);
            let url_entry = UrlEntry::builder()
                .loc(static_url)
                .changefreq(ChangeFreq::Monthly)
                .build()
                .expect("valid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        for post in post_results {
            let post_url = format!(
                "http://www.steadylearner.com/blog/read/{}",
                post.title.replace(" ", "-")
            );
            // Use Monthly or Yeary
            let url_entry = UrlEntry::builder()
                .loc(post_url)
                .changefreq(ChangeFreq::Yearly)
                .build()
                .expect("valid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        let _sitemap_xml = urlwriter.end().expect("Unable to write close tags");
    }

    println!("{:#?}", std::str::from_utf8(&output));

    fs::write("sitemap.xml", &output)?;

    Ok(())
}
