extern crate diesel;
extern crate sl_lib;
extern crate console;
extern crate sitemap;

use diesel::prelude::*;
use console::Style;

use std::fs;

use sitemap::writer::SiteMapWriter;
// use sitemap::structs::UrlEntry;

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

    println!("\nWrite sitemap for {} posts", bold.apply_to(post_results.len()));

    let mut output = Vec::<u8>::new();;
    {
        let sitemap_writer = SiteMapWriter::new(&mut output);
        let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
        for post in post_results {
            let post_url = format!("http://www.steadylearner.com/blog/read/{}", post.title.replace(" ", "-"));
            urlwriter.url(post_url).expect("Unable to write url");
        }

        let _sitemap_xml = urlwriter.end().expect("Unable to write close tags");
    }

    println!("{:#?}", std::str::from_utf8(&output));

    fs::write("sitemap.xml", &output)?;

    Ok(())
}



