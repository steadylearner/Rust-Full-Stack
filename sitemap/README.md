<!--
    Post{
        subtitle: "Build sitemap automatically with Rust",
        image: "post/sitemap/automate-sitemap-rust.png",
        image_decription: "Image by Steadylearner",
        tags: "Rust How sitemap automate",
    }
-->

<!-- Link -->

[Rust Sitemap Crate]: https://github.com/svmk/rust-sitemap
[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[How to deploy Rust Web App]: https://medium.com/@steadylearner/how-to-deploy-rust-web-application-8c0e81394bd5?source=---------9------------------
[Rust Diesel]: http://diesel.rs/
[Sitemap in React]: https://medium.com/@steadylearner/how-to-build-a-sitemap-for-react-app-7bbc3040dc1f
[Steadylearner Sitemap]: https://github.com/steadylearner/Sitemap
[What is image sitemap]: https://support.google.com/webmasters/answer/178636

[futures-timer]: https://github.com/rustasync/futures-timer
[timers-tokio]: https://tokio.rs/docs/going-deeper/timers/

<!-- / -->

<!-- Steadylearner Post -->

[sitemap]: https://www.steadylearnerc.com/blog/search/sitemap
[sitemap.xml]: https://www.steadylearner.com/sitemap.xml
[txt sitemap]: https://www.steadylearner.com/sitemap.txt
[image sitemap]: https://www.steadylearner.com/image_sitemap.xml

<!-- / -->

In this post, We will make functions to build **sitemap.txt**, **sitemap.xml** and **image_siemap.xml** for images and make them reusable.

You can use them later with thread. Then, include them inside interval to automate the process without affecting the main function. You may use it with CLI also.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [Rust Sitemap Crate]
2. [What is image sitemap]
3. [What is sitemap](https://support.google.com/webmasters/answer/156184?hl=en)
4. [How to build a sitemap](https://www.google.com/search?client=firefox-b-d&q=how+to+build+sitemap)
5. [Futuers in Rust](https://docs.rs/futures/0.2.3-docs-yank.4/futures/)
6. [Thread in Rust](https://doc.rust-lang.org/std/thread/)
7. [futures-timer]
8. [timers-tokio]

---

I want you to visit them and read posts for [sitemap] at [Steadylearner].

I will suppose that you already have experience in Rust and other programming.

<br />

<h2 class="blue">Table of Contents</h2>

1. Separate functions to build sitemaps
2. Use them inside fn main() with thread
3. Interval and automation
4. **Conclusion**

---

<br />

## 1. Separate functions to build sitemaps

We will write **cargo.toml** first. Because we will use thread inside **main.rs**, we don't need other Rust bin files like we do in other post for [sitemap].

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

Then, we will define **image_stiemap_renewal** function in **sitemap_renew.rs** first.

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

Only location and name of the function are different from [the previous posts][sitemap].

Then, we will build **sitemap_txt_renewal** function that will use **sitemap.xml** made from **sitemap_renewal** function later.

```rust
fn sitemap_txt_renewal() -> std::io::Result<()> {
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

    // println!("payload = {:?}", urls[0].loc.get_url().unwrap());

    let mut output = String::new();
    output.push_str("http://www.steadylearner.com/video/search/*
http://www.steadylearner.com/video/watch/*
http://www.steadylearner.com/video/write/*
http://www.steadylearner.com/image/search/*
http://www.steadylearner.com/blog/search/*
http://www.steadylearner.com/blog/read/*
http://www.steadylearner.com/code/search/*
http://www.steadylearner.com/static/images/*
");

    for url in urls {
        let payload = url.loc.get_url().unwrap();
        println!("{}", &payload);
        let payload_with_new_line = format!("{}\n", &payload);
        output.push_str(&payload_with_new_line);
    }

    println!("{:#?}", &output);
    write("sitemap.txt", &output)?;

    println!("errors = {:?}", errors);

    Ok(())
}
```

You may use all selector * and others before you make **sitemap.txt** from **sitemap.xml**.

Lastly, our **sitemap_renewal** function will be similar to

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

        // You may use this if you have many sitemaps.

        // let mut sitemap_index_writer = sitemap_writer
        //     .start_sitemapindex()
        //     .expect("start sitemap index tag");

        // for path_for_other_sitemap in paths_for_other_sitemaps {
        //     let entire_path_for_other_sitemap =
        //         format!("https://www.steadylearner.com/{}", path_for_other_sitemap);

        //     let sitemap_entry = SiteMapEntry::builder()
        //         .loc(entire_path_for_other_sitemap)
        //         .lastmod(date)
        //         .build()
        //         .expect("valid");

        //     sitemap_index_writer
        //         .sitemap(sitemap_entry)
        //         .expect("Can't write the file");
        // }

        // sitemap_index_writer.end().expect("close sitemap block");
    }

    write("sitemap.xml"::, &output)?;
    sitemap_txt_renewal();

    Ok(())
}
```

Its purposes are

1. build **sitemap.xml** from static routes and the datas from the database
2. link another sitemap such as **image_sitemap.xml** we built before
3. and make **sitemap.txt** from **sitemap.xml**

If you read the code snippets from the previous post for [sitemap], we removed almost all **println!** to show results at console.

You may not need them when what you want is automation and already know what the code snippets do here.

We organized all our codes into separate functions. They became reusable and can be used everywhere.

<br />

## 2. Use them inside fn main() with thread

We made functions to build sitemaps. We will call them inside **main.rs** to make them build whenever we run our website.

You won't want it to affect the main process, so you may separate it inside another **thread**.

For that, **main.rs** should be similar to this.

```rust
use std::thread;

mod sitemap_renew;
use crate::sitemap_renew::{sitemap_renewal, image_sitemap_renewal};

fn main() {
     thread::Builder::new()
        .name("Build sitemap automatically with Rust".into())
        .spawn(|| {
            image_sitemap_renewal()?;
            let static_paths = vec![
                "about",
                "video",
                "blog",
                "code",
                "image",
                "slideshow",
            ];
            let other_sitemaps = vec!["image_sitemap.xml"];
            sitemap_renewal(static_paths, other_sitemaps)
        })
        .unwrap();

    // your_website();
}
```

We invested our time to modulize our work to build sitemap and **main.rs** became simpler with thread.

You can test it with **$cargo run bin --main** and you will see files similar to [sitemap.xml], [txt sitemap] and [image sitemap] for [Steadylearner].

<br />

## 3. Interval and automation

If you want to automate the process, you may use **Interval** API from [futures-timer], [timers-tokio] etc.

I had working code with [futures-timer] similar to

```rust
extern crate futures;
extern crate futures_timer;

use std::time::Duration;

use futures::prelude::*;
use futures_timer::Interval;

mod sitemap_renew;
use crate::sitemap_renew::{sitemap_renewal, image_sitemap_renewal};

fn main() {
    thread::Builder::new()
        .name("Build sitemap automatically with Rust".into())
        .spawn(|| {
            // { day: 86400, week: 604800, month: 2592000, }
            Interval::new(Duration::from_secs(1))
                .for_each(|()| {
                    image_sitemap_renewal()?;
                    let static_paths = vec![
                        "about",
                        "video",
                        "blog",
                        "code",
                        "image",
                        "slideshow",
                    ]
                    let other_sitemaps = vec!["image_sitemap.xml"];
                    sitemap_renewal(static_paths, other_sitemaps)
                })
                .wait()
                .unwrap();
        })
        .unwrap();

    // your_website(); // It is arbitary name, don't give it importance.
}
```

Its API was rewritten with the introduction of **async** in **Rust** and seem to be not working anymore.

However, You may refer to the code snippet above and their documentations for your project.

With tokio, your main code would be similar to

```rust
extern crate tokio;
use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};

use std::threa

fn main() {
    thread::Builder::new()
        .name("Build sitemap automatically with Rust".into())
        .spawn(|| {
            // { day: 86400, week: 604800, month: 2592000, }
            let task = Interval::new(Instant::now(), Duration::from_secs(604800))
            .for_each(|instant| {
                println!("fire; instant={:?}", instant);
                image_sitemap_renewal().expect("Error while making sitemap.xml for images");
                let static_paths = vec!["about", "video", "blog", "code", "image", "slideshow"];
                let other_sitemaps = vec!["image_sitemap.xml"];
                sitemap_renewal(static_paths, other_sitemaps);
                Ok(())
            })
            .map_err(|e| panic!("interval errored; err={:?}", e));

            tokio::run(task);
        })
        .unwrap();

    // your_website();
}
```

When you run it with **$cargo run --bin main**, it will show log similar to this

```console
fire; instant=Instant

http://www.steadylearner.com/
http://www.steadylearner.com/about
http://www.steadylearner.com/video
http://www.steadylearner.com/blog
http://www.steadylearner.com/code
http://www.steadylearner.com/image
http://www.steadylearner.com/slideshow
http://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
http://www.steadylearner.com/blog/read/How-to-install-Rust

errors = []
```

or more with interval seconds you define.

The **thread** Rust API here is optional. It will work without it.

<br />

## 4. Conclusion

The latest code for [sitemap] can be found at [Steadylearner Sitemap] repository. It is the end of the posts for [sitemap] with Rust.

**Thanks and please share this post with others.**
