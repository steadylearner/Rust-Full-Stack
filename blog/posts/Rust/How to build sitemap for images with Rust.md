<!--
    Post{
        subtitle: "Learn how to make image_sitemap.xml with Rust",
        image: "post/sitemap/sitemap-for-images-with-rust.png",
        image_decription: "Image Made with CSS by Steadylearner",
        tags: "Rust How sitemap image",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Sitemap]: https://github.com/steadylearner/Sitemap

[Rust Diesel]: http://diesel.rs/
[Rust Sitemap Crate]: https://github.com/svmk/rust-sitemap

[What is sitemap]: https://support.google.com/webmasters/answer/156184?hl=en
[What is image sitemap]: https://support.google.com/webmasters/answer/178636

[xml sitemap]: https://www.steadylearner.com/sitemap.xml
[image sitemap]: https://www.steadylearner.com/image_sitemap.xml
[sitemap]: https://www.steadylearner.com/blog/search/sitemap

<!-- / -->

<!-- Steadylearner Post -->

[How to make sitemap with dynamic contents in Rust]: https://www.steadylearner.com/blog/read/How-to-make-sitemap-with-dynamic-contents-in-Rust

<!-- / -->

If you read the previous post for [sitemap] with Rust, You could already build sitemap.xml with database in Rust.

This post will apply the process to build **sitemap.xml** for images.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [Rust Sitemap Crate]
2. [How to make sitemap with dynamic contents in Rust]
3. [What is sitemap]
4. [How to build a sitemap](https://www.google.com/search?client=firefox-b-d&amp;q=how+to+build+sitemap)

---

Code used here is very similar to [How to make sitemap with dynamic contents in Rust]. I hope you read it first.

Then, visit [What is image sitemap] because we will replicate the example in it.

You need database ready for images with [Rust Diesel] or others. I will give you an example so you may refer to it.

If you want to see the final project first, you can refer to [Steadylearner Sitemap] Repository.

<br />

<h2 class="blue">Table of Contents</h2>

1. Write **image_sitemap.rs** to build sitemap.xml for images
2. **How to include it** inside your main sitemap.xml
3. **Conclusion**

---

<br />

## 1. Write **image_sitemap.rs** to build sitemap.xml for images

We will prepare files to help **image_sitemap.rs** later first.

You can define **Image** struct similar to this.

```rust
// models.rs Rust Diesel example
#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub title: String,
    pub media_url: String,
    pub content: String,
}
```

Then, organize your cargo.toml similar to this.

Our Rust code to write sitemap is becoming complicated. So it will be better to separate them.

```toml
# cargo.toml

# $cargo run --bin <name> will point to the path we define and execute the file.
# (Use <name> to image-sitemap for this post)
[[bin]]
name = "main"
path = "src/bin/main.rs"
[[bin]]
name = "sitemap"
path = "src/bin/sitemap.rs"
[[bin]]
name = "image-sitemap"
path = "src/bin/image_sitemap.rs"

# lib is used like crate and can import and export other files in the same directory level.

[lib]
name = "your_lib"
path = "src/lib.rs"
```

Then, write image_sitemap.rs file to write **image_sitemap.xml** file for images similar to this.

```rust
// image_sitemap.rs
extern crate sl_lib;
extern crate console;
extern crate diesel;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use sl_lib::models::*;
use sl_lib::*;

use console::Style;

use diesel::prelude::*;

fn main() -> std::io::Result<()> {
    use crate::schema::images::dsl::*;
    let connection = init_pool().get().unwrap();

    let bold = Style::new().bold();

    let image_results = images
        .load::<Image>(&amp;*connection)
        .expect("Error loading images");

    println!(
        "\nIt starts to write image_sitemap.xml for {} images",
        bold.apply_to(image_results.len())
    );

    // https://support.google.com/webmasters/answer/178636?hl=en

    let start_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
  <url>
    <loc>https://www.steadylearner.com</loc>
"#;

    fs::write("image_sitemap.xml", &amp;start_xml)?;
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
        );
        image_xml.push_str(&amp;image_url);
    }

    if let Err(e) = writeln!(result, "{}{}", image_xml , r#"  </url>
</urlset> "#) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("image_sitemap.xml was built. Include it to main sitemap.xml.");

    Ok(())
}
```

The example is not so different from the ones your used in other post for [sitemap].

We just need to modify some parts to make them work for images.

What we do are

1. **Data setup for images** instead of posts.

2. **Manually start to write boilerplate** for image_sitemap.xml.

3. Define mutable variable **image_xml** and **pass and process the payload data** inside **for in**.

4. Manually end writing wrapper for **image_sitemap.xml**.

and that was all.

You can test it with `cargo c` or `cargo run --bin <name>` inside your console.

It will create image_sitemap.xml similar to this [image sitemap].
I want you to test it with your datas for images.

What we need to do next is to include it inside your main **sitemap.xml** file.

If you haven't any datas for images yet, you may visit [image sitemap] for [Steadylearner]

<br />

## 2. How to include it inside your main sitemap.xml

In the previous part, we built **image_sitemap.xml**.

It is separtated from your main **sitemap.xml** and we will use API from [Rust Sitemap Crate] to link it.

By doing this, we don't have to submit it to search engines everytime we create new sitemaps.

You can refer to the code for the [Steadylearner] and modify it for your project.

(**The code snippet is long** and we will organize it in the next post.)

```rust
// sitemap.rs
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
    let bold = Style::new().bold();

    use crate::schema::posts::dsl::*;
    let connection = init_pool().get().unwrap();

    let post_results = posts
        .filter(published.eq(true))
        .order(created_at.desc())
        .load::<Post>(&amp;*connection)
        .expect("Error loading posts");

    println!(
        "\nWrite sitemap for {} posts",
        bold.apply_to(post_results.len())
    );

    let mut output = Vec::<u8>::new();
    {
        let sitemap_writer = SiteMapWriter::new(&amp;mut output);

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
            .lastmod(date)
            .build()
            .expect("invalid");
        urlwriter.url(home_entry).expect("Unable to write url");

        let static_routes: Vec<&amp;str> = vec!["about", "video", "blog", "code", "image", "slideshow"];

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
            let url_entry = UrlEntry::builder()
                .loc(post_url)
                .changefreq(ChangeFreq::Yearly)
                .lastmod(date)
                .build()
                .expect("invalid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        // 1.
        let sitemap_writer = urlwriter.end().expect("close the urlset block");

        println!("You wanna chain other .xml type sitemap here?");
        println!("Type yes for that or no to proceed to the nexts process");

        // 2.
        let choice = str_from_stdin()
            .chars()
            .next() // equals to .nth(0)
            .expect("string is empty");

        match choice {
            'y' => {
                // 3.
                let mut sitemap_index_writer = sitemap_writer
                    .start_sitemapindex()
                    .expect("start sitemap index tag");
                println!("Type path for the other sitemap");

                // 4.
                // Use react_sitemap.xml, image_sitemap.xml etc or use variable instead
                let paths_for_other_sitemaps = str_from_stdin();

                let sitemap_paths_to_string = String::from(paths_for_other_sitemaps);
                let split_paths_for_other_sitemaps: Vec<&amp;str> = sitemap_paths_to_string.split(" ").collect();

                // 5.
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

    println!("{:#?}", std::str::from_utf8(&amp;output));

    write("sitemap.xml", &amp;output)?;

    // 6.
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

            println!("payload = {:?}", urls[0].loc.get_url().unwrap());

            let mut output = String::new();

            for url in urls {
                let payload = url.loc.get_url().unwrap();
                println!("{}", &amp;payload);
                let payload_with_new_line = format!("{}\n", &amp;payload);
                output.push_str(&amp;payload_with_new_line);
            }

            println!("{:#?}", &amp;output);
            write("sitemap.txt", &amp;output)?;

            println!("errors = {:?}", errors);
        }
        _ => {
            println!("The process ends here");
        }
    }

    Ok(())
}
```

The main points for the code snippet above are

1. **Giving ownership of the process** to write sitemap to variable **sitemap_writer**.(It is important to include this code to the rest of the code to work)

2. **Consider only first letter** of user input.

3. Note that **sitemap_write** variable is used again for this process. Because normal process to write sitemap from **static path** and **the datas from the data** were ended before, We start to write **sitemap index** to chain other sitemaps before the end of the entire process to write **sitemap.xml**.

4. The code used here is to easily chain other **.xml** sitemap format.

5. Use `for in` again to **use .xml files typed by user** before.

6. **End the entire process to write sitemap.xml** and pass the main control to question users **whether they want to write sitemap.txt** file also or not.

and that was all.

You can use it to build **sitemap.xml**, **sitemap.txt** from it. Then, chain other .xml files such as image_sitemap.xml you built before.

<br />

## 3. Conclusion

By following posts for [sitemap], we could make various sitemaps such as **sitemap.xml**, **sitemap.txt**, **image_sitemap.xml**.

With the help from the Rust Sitemap Crate, we could write sitemaps easily with Rust.

If you spent time with the crate, you could find its main API is

```rust
let url_entry = UrlEntry::builder()
                .loc(post_url)
                .changefreq(ChangeFreq::Yearly)
                .lastmod(date)
                .build()
                .expect("invalid");
```

and what we do is just to use them with other Rust codes.

In the next post, we will organize the entire process for [sitemap] with Rust.

**Thanks and please share this post with others.**
