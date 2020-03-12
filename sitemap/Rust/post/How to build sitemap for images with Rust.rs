extern crate chrono;
extern crate console;
extern crate diesel;
extern crate sitemap;
extern crate sl_lib;

use std::fs::{write, File};

use console::Style;
use diesel::prelude::*;

use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap::reader::{SiteMapEntity, SiteMapReader};
use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use sitemap::writer::SiteMapWriter;

use sl_lib::models::*;
use sl_lib::*;

use sl_lib::custom::str_from_stdin;

fn main() -> std::io::Result<()> {
    // To decorate console output
    let bold = Style::new().bold();

    // Use database with Rust diesel to write sitemap.xml first
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
            .expect("invalid");
        urlwriter.url(home_entry).expect("Unable to write url");

        let static_routes: Vec<&str> = vec!["about", "video", "blog", "code", "image", "slideshow"];

        for route in static_routes.iter() {
            let static_url = format!("http://www.steadylearner.com/{}", route);
            let url_entry = UrlEntry::builder()
                .loc(static_url)
                .changefreq(ChangeFreq::Monthly)
                .lastmod(date)
                .build()
                .expect("invalid");

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
                .expect("invalid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        // assigining value to sitemap_writer is important to make the next process work
        let sitemap_writer = urlwriter.end().expect("close the urlset block");

        // To link other sitemap to sitemap.xml(works as a index for other .xml type sitemap)
        println!("You wanna chain other .xml type sitemap here?");
        println!("Type yes for that or no to proceed to the nexts process");

        // Consider only first letter of user input to console
        let choice = str_from_stdin()
            .chars()
            .next() // equals to .nth(0)
            .expect("string is empty");

        match choice {
            'y' => {
                let mut sitemap_index_writer = sitemap_writer
                    .start_sitemapindex()
                    .expect("start sitemap index tag");
                println!("Type path for the other sitemap");

                // Type react_sitemap.xml, image_sitemap.xml etc
                let paths_for_other_sitemaps = str_from_stdin();

                let sitemap_paths_to_string = String::from(paths_for_other_sitemaps);
                let split_paths_for_other_sitemaps: Vec<&str> = sitemap_paths_to_string.split(" ").collect();

                for path_for_other_sitemap in split_paths_for_other_sitemaps {
                    let entire_path_for_other_sitemap =
                        format!("https://www.steadylearner.com/{}", path_for_other_sitemap);

                    let sitemap_entry = SiteMapEntry::builder()
                        .loc(entire_path_for_other_sitemap)
                        .lastmod(date)
                        .build()
                        .expect("invalid");

                    sitemap_index_writer
                        .sitemap(sitemap_entry)
                        .expect("Can't write the file");
                }

                sitemap_index_writer.end().expect("close sitemap block");
            }
            _ => println!("You may not need it. Let's move on to the next phase"),
        }
    }

    println!("{:#?}", std::str::from_utf8(&output));

    write("sitemap.xml", &output)?;

    // sitemap.txt is based on the sitemap.xml and it is already built at the point
    println!("You wanna write sitemap.txt file also?");
    println!("Type yes to write sitemap.txt or no to end the process");

    let choice = str_from_stdin().chars().next().expect("string is empty");

    match choice {
        'y' => {
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
            println!("payload = {:?}", urls[0].loc.get_url().unwrap());

            let mut output = String::new();

            for url in urls {
                let payload = url.loc.get_url().unwrap();
                println!("{}", &payload);
                let payload_with_new_line = format!("{}\n", &payload);
                output.push_str(&payload_with_new_line);
            }

            println!("{:#?}", &output);
            write("sitemap.txt", &output)?;

            println!("errors = {:?}", errors);
        }
        _ => {
            println!("The process ends here");
        }
    }

    Ok(())
}

