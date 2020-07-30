extern crate sitemap;
use sitemap::reader::{SiteMapReader,SiteMapEntity};
use std::fs::{File, write};

fn main() -> std::io::Result<()> {
    let mut urls = Vec::new();
    let mut sitemaps = Vec::new();
    let mut errors = Vec::new();

    let file = File::open("sitemap.xml").expect("Unable to open file.");
    let parser = SiteMapReader::new(file);
    for entity in parser {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                urls.push(url_entry);
            },
            SiteMapEntity::SiteMap(sitemap_entry) => {
                sitemaps.push(sitemap_entry);
            },
            SiteMapEntity::Err(error) => {
                errors.push(error);
            },
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

    write("sitemap.txt", &output)?;

    println!("errors = {:?}", errors);

    Ok(())

}