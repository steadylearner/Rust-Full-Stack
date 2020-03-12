<!--
    Post{
        subtitle: "Learn how to automate process for sitemap",
        image: "post/sitemap/sitemap_for_images_with_rust.png",
        image_decription: "Made with CSS by Steadylearner",
        tags: "Rust sitemap automate code",
    }
-->

<!-- Link -->

[Rust Sitemap Crate]: https://github.com/svmk/rust-sitemap
[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[How to deploy Rust Web App]: https://medium.com/@steadylearner/how-to-deploy-rust-web-application-8c0e81394bd5?source=---------9------------------
[Rust Diesel]: http://diesel.rs/
[Sitemap in React]: https://medium.com/@steadylearner/how-to-build-a-sitemap-for-react-app-7bbc3040dc1f
[Sitemap GitHub]: https://github.com/steadylearner/Sitemap
[What is image sitemap]: https://support.google.com/webmasters/answer/178636

<!-- / -->

<!-- Post -->

[Your first sitemap with Rust]: https://www.steadylearner.com/blog/read/Your-first-sitemap-with-Rust
[How to use datas to build sitemap with Rust Diesel]: https://www.steadylearner.com/blog/read/How-to-use-datas-to-build-sitemap-with-Rust-Diesel
[How to build a sitemap.txt from sitemap.xml with Rust]: https://www.steadylearner.com/blog/read/How-to-build-a-sitemap.txt-from-sitemap.xml-with-Rust
[How to build sitemap for images with Rust]: https://www.steadylearner.com/blog/read/How-to-build-sitemap-for-images-with-Rust
[How to build a sitemap for React App]: https://medium.com/@steadylearner/how-to-build-a-sitemap-for-react-app-7bbc3040dc1f

<!-- / -->

In this post, we will learn how to automate all the process we learnt before. I hope you read the previous posts if you haven't.

1. [Your first sitemap with Rust]
2. [How to use datas to build sitemap with Rust Diesel]
3. [How to build a sitemap.txt from sitemap.xml with Rust]
4. [How to build sitemap for images with Rust]

What we will do is just to remove some codes and organize all.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [Rust Sitemap Crate]
2. [What is image sitemap]
3. [What is sitemap](https://support.google.com/webmasters/answer/156184?hl=en)
4. [How to build a sitemap](https://www.google.com/search?client=firefox-b-d&q=how+to+build+sitemap)
5. [Futuers in Rust](https://docs.rs/futures/0.2.3-docs-yank.4/futures/)
6. [Interval in Rust](https://crates.io/crates/futures-timer)
7. [Thread in Rust](https://doc.rust-lang.org/std/thread/)

---

We will use **Interval** API from [futuers-timer](https://crates.io/crates/futures-timer) to automatically renew sitemap.xml, sitemap.txt and image_sitemap.xml we built before. We will execute it in another thread inside **fn main()** function not to affect other more important processes. So you need to understand how to use [it](https://doc.rust-lang.org/std/thread/) also.

<br />

<h2 class="blue">Table of Contents</h2>

1. Separate **auto_sitemap.rs** from the previous codes.
2. Use functions from it with thread inside **fn main()** 
3. **Conclusion**

---

<br />

## 1. Separate auto_sitemap.rs from the previous codes.

You don't need to execute sitemap.rs and image_sitemap.rs we built before after this post.
Therefore, we will first remove codes relevant to them from our **cargo.toml**.

(You don't have to care for it if you haven't read the previous post.)

<br />

```toml
# cargo.toml, write the code similar to this

# $cargo run --bin <name> will point to the path we define here
# (Replace <name> to image-sitemap for this post)
[[bin]]
name = "main"
path = "src/bin/main.rs"

# lib is used like crate and can import and export other files in the same directory level.

[lib]
name = "your_lib"
path = "src/lib.rs"
```

<br />

Then we will build **auto_sitemap.rs** and include code from image_sitemap.rs first.
It will be similar to

```rust
extern crate diesel;
extern crate sl_lib;

use std::fs;
use std::fs::{write, File};
use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap::reader::{SiteMapEntity, SiteMapReader};
use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use sitemap::writer::SiteMapWriter;

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
```

Only location and name of the function are different from the previous post. You can use it separately also for it always do the same thing to build sitemap for images.

Then we will include code snippet below it to 

1. build **sitemap.xml** from static routes and the datas from the database
2. link another sitemap such as image_sitemap built before
3. and **sitemap.txt** from sitemap.xml.

```rust
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
            .lastmod(date)
            .build()
            .expect("valid");
        urlwriter.url(home_entry).expect("Unable to write url");

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

        let mut output = String::new();

        for url in urls {
            let payload = url.loc.get_url().unwrap();
            println!("{}", &payload);
            let payload_with_new_line = format!("{}\n", &payload);
            output.push_str(&payload_with_new_line);
        }
    }

    Ok(())
}
```

It is a little bit long. But is is accepatable for we don't have to care for them after we put them inside **main()** function.

If you read the code snippet from the previous post, you will find that there are no more **println!** because you don't need them for automated process.

For we organized our codes into functions, they can be used wherever we want.

<br />

## 2. Use functions from it with thread inside **fn main()**

We are almost there to end all the process to build sitemap automatically with Rust.
What we need to do is just call those functions we made before inside **main.rs**.

For we won't want to this to affect our website, we will separate it inside the other **thread**.

Your main.rs should be similar to this.

```rust
extern crate futures;
extern crate futures_timer;

use std::time::Duration;

use futures::prelude::*;
use futures_timer::Interval;

mod auto_sitemap;
use crate::auto_sitemap::{sitemap_renewal, image_sitemap_renewal};

fn main() {
    thread::Builder::new()
        .name("Build sitemap automatically with Rust".into()) // 1.
        .spawn(|| {
            Interval::new(Duration::from_secs(1)) // 2. { day: 86400, week: 604800, month: 2592000, }
                .for_each(|()| {
                    image_sitemap_renewal()?; // 3.
                    let static_paths = vec!["about", "video", "blog", "code", "image", "slideshow"]; // 4.
                    let other_sitemaps = vec!["image_sitemap.xml"]; // 5.
                    sitemap_renewal(static_paths, other_sitemaps)
                })
                .wait()
                .unwrap();
        })
        .unwrap();

    your_website(); // It is arbitary name, don't give it importance.
}
```

We invested our time to modulize our work to build sitemap before. So your main function became very simple with thread and functions we defined before. I hope you read the posts given above to understand it better.

The point here is

1. Use your name to debug
2. Substitue it for your need
3. This should be located before **sitemap_renewal**
4. Include **static paths** for your website
5. Put .xml for **images, news, videos** etc

and that was all you need to build your **sitemap.xml, sitemap.txt and image_sitemap.xml** etc with **Rust**.

If you execute it with **$cargo run bin --main**, you will see message similar to this on your CLI

```xml
It starts to write image_sitemap.xml for images
image_sitemap.xml was built. Include it to main sitemap.xml.

It starts to write sitemap.xml for posts

http://www.steadylearner.com/
http://www.steadylearner.com/about
http://www.steadylearner.com/video
http://www.steadylearner.com/blog
http://www.steadylearner.com/code
http://www.steadylearner.com/image
http://www.steadylearner.com/slideshow
http://www.steadylearner.com/blog/read/How-to-use-Python-with-JavaScript-with-python-bridge
```

<br />

## 3. Conclusion

This is the end of series for sitemap with Rust. The code for each post can be found at [Sitemap GitHub].

I hope it worked well for your computer. 
(Even though it take some time to compile at first in my battery dead, half-ten year laptop)

The next posts will be simple chat app with Rust Backend and how to write blog with Rust like [Steadylearner].

**Thanks and please share this post with others.**
