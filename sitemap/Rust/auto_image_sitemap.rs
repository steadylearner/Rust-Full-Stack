extern crate sl_lib;
extern crate diesel;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use sl_lib::models::*;
use sl_lib::*;

use diesel::prelude::*;

pub fn image_sitemap_renewal() -> std::io::Result<()> {
    use crate::schema::images::dsl::*;
    let connection = init_pool().get().unwrap();

    let image_results = images
        .load::<Image>(&*connection)
        .expect("Error loading images");

    println!(
        "\nIt starts to write image_sitemap.xml for {} images",
        image_results.len()
    );

    // https://support.google.com/webmasters/answer/178636?hl=en

    let start_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
  <url>
    <loc>https://www.steadylearner.com</loc>
"#;

    fs::write("image_sitemap.xml", &start_xml)?;
    let mut result = OpenOptions::new().append(true).open("image_sitemap.xml").unwrap();

    let mut image_xml = String::new();
    for image in image_results {
        let image_url = format!(
            "    <image:image>
      <image:title>{}</image:title>
      <image:caption>{}</image:caption>
      <image:loc>https://www.steadylearner.com{}</image:loc>
      <image:license>https://www.steadylearner.com</image:license>
    </image:image>
",
            image.title,
            image.content,
            image.media_url,
            // image.title.replace(" ", "-")
        );
        image_xml.push_str(&image_url);
    }

    if let Err(e) = writeln!(result, "{}{}", image_xml , r#"  </url>
</urlset> "#) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("image_sitemap.xml was built. Include it to main sitemap.xml.");

    Ok(())
}