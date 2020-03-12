extern crate diesel;
extern crate sl_lib;
extern crate console;
extern crate sitemap;

use diesel::prelude::*;
// use console::Style;

use sitemap::writer::SiteMapWriter;
// use sitemap::structs::UrlEntry;
use std::io::stdout;

use sl_lib::models::*;
use sl_lib::*;

// use sl_lib::custom::str_from_stdin;

// read post titles from the database with diesel
// verify it work
// stdout -> sitemap.xml file
// use match and console?

fn main() -> std::io::Result<()> {
    use crate::schema::posts::dsl::*;
    let connection = init_pool().get().unwrap();

    let results = posts
        .filter(published.eq(true))
        .order(created_at.desc())
        .load::<Post>(&*connection)
        .expect("Error loading posts");

    println!("\nWrite sitemap for {} posts", results.len());

    let mut output = stdout();
    let sitemap_writer = SiteMapWriter::new(&mut output);
    let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
    for post in results {
        // use this not to make site map with %20 and also searchbot can visit the path with -
        let post_url = format!("http://www.steadylearner.com/blog/read/{}", post.title.replace(" ", "-"));
        urlwriter.url(post_url).expect("Unable to write url");
    }

    urlwriter.end().expect("Unable to write close tags");

    Ok(())
}

// with node.js a.replace(/%20/g, '-');



