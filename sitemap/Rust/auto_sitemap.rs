// You don't need it when main.rs(the file it imports the function) file have them
// extern crate chrono;
// extern crate sitemap;

extern crate diesel;
extern crate sl_lib;

use std::fs::{write, File};

use diesel::prelude::*;

use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap::reader::{SiteMapEntity, SiteMapReader};
use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use sitemap::writer::SiteMapWriter;

use sl_lib::models::*;
use sl_lib::*;

// use sl_lib::custom::str_from_stdin;

pub fn sitemap_renewal(static_routes: Vec<&str>, paths_for_other_sitemaps: Vec<&str>) -> std::io::Result<()> {
    // Use database with Rust diesel to write sitemap.xml first
    use crate::schema::posts::dsl::*;
    let connection = init_pool().get().unwrap();

    let post_results = posts
        .filter(published.eq(true))
        .order(created_at.desc())
        .load::<Post>(&*connection)
        .expect("Error loading posts");

    println!(
        "\nIt starts to write sitemap.xml for {} posts",
        post_results.len()
    );

    let mut output = Vec::<u8>::new();
    {
        let sitemap_writer = SiteMapWriter::new(&mut output);

        let mut urlwriter = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

        let today = what_is_date_today();

        let date = DateTime::from_utc(
            NaiveDate::from_ymd(today.year, today.month, today.day).and_hms(0, 0, 0),
            FixedOffset::east(0),
        );

        let home_entry = UrlEntry::builder()
            .loc("http://www.steadylearner.com")
            .changefreq(ChangeFreq::Monthly)
            .lastmod(date) // priority is removed for some search engines ignore it and personal choice.
            .build()
            .expect("valid");
        urlwriter.url(home_entry).expect("Unable to write url");

        // let static_routes: Vec<&str> = vec!["about", "video", "blog", "code", "image", "slideshow"];

        for route in static_routes.iter() {
            let static_url = format!("http://www.steadylearner.com/{}", route);
            let url_entry = UrlEntry::builder()
                .loc(static_url)
                .changefreq(ChangeFreq::Monthly)
                .lastmod(date)
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
                .lastmod(date)
                .build()
                .expect("valid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        // assigining value to sitemap_writer is important to make the next process work
        let sitemap_writer = urlwriter.end().expect("close the urlset block");

        let mut sitemap_index_writer = sitemap_writer
            .start_sitemapindex()
            .expect("start sitemap index tag");

        for path_for_other_sitemap in paths_for_other_sitemaps {
            let entire_path_for_other_sitemap =
                format!("https://www.steadylearner.com/{}", path_for_other_sitemap);

            let sitemap_entry = SiteMapEntry::builder()
                .loc(entire_path_for_other_sitemap)
                .lastmod(date)
                .build()
                .expect("valid");

            sitemap_index_writer
                .sitemap(sitemap_entry)
                .expect("Can't write the file");
        }

        sitemap_index_writer.end().expect("close sitemap block");
    }

    write("sitemap.xml", &output)?;

    {
        // sitemap.txt is based on the sitemap.xml and it is already built at the point
        let mut urls = Vec::new();
        let mut sitemaps = Vec::new();
        let mut errors = Vec::new();

        let file = File::open("sitemap.xml").expect("Unable to open file.");
        let parser = SiteMapReader::new(file);
        for entity in parser {
            match entity {
                SiteMapEntity::Url(url_entry) => {
                    urls.push(url_entry);
                }
                SiteMapEntity::SiteMap(sitemap_entry) => {
                    sitemaps.push(sitemap_entry);
                }
                SiteMapEntity::Err(error) => {
                    errors.push(error);
                }
            }
        }

        // get_url from the source code, unwrap to get the payload value inside Some
        // println!("payload = {:?}", urls[0].loc.get_url().unwrap());

        let mut output = String::new();

        for url in urls {
            let payload = url.loc.get_url().unwrap();
            println!("{}", &payload);
            let payload_with_new_line = format!("{}\n", &payload);
            output.push_str(&payload_with_new_line);
        }

        // println!("{:#?}", &output);
        // write("sitemap.txt", &output)?;

        // println!("errors = {:?}", errors);
    }

    Ok(())
}
