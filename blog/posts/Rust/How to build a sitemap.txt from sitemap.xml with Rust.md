<!--
    Post{
        subtitle: "Learn how to make sitemap.txt easily with Rust.",
        image: "post/sitemap/sitemap-txt-with-rust.png",
        image_decription: "Image by Steadylearner",
        tags: "Rust How sitemap code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Sitemap]: https://github.com/steadylearner/Sitemap

[Rust Diesel]: http://diesel.rs/
[Rust Sitemap Crate]: https://github.com/svmk/rust-sitemap

[xml sitemap]: https://www.steadylearner.com/sitemap.xml
[sitemap]: https://www.steadylearner.com/blog/search/sitemap

<!-- / -->

<!-- Steadylearner Post -->

<!-- / -->

This is a folllowing post from the previous ones to build **sitemap.xml** files. I hope you already read [them][sitemap].

The code used here can be used separately or include it in your project to build **sitemap.xml**.

<br />

<h2 class="red-white" >[Prerequisite]</h2>

1. [Rust Sitemap Crate]
2. [What is sitemap](https://support.google.com/webmasters/answer/156184?hl=en)
3. [How to build a sitemap](https://www.google.com/search?client=firefox-b-d&amp;q=how+to+build+sitemap)
4. sitemap.xml files to build sitemap.txt

---

The process will be simple. I just want you to read and test **Reading sitemap documents** part at [Rust Sitemap Crate].

If you are a React developer and you didn't have any sitemap.xml or sitemap.txt file yet, you may use [xml sitemap] for [Steadylearner].

If you prefer to see the final project first, you can refer to [Steadylearner Sitemap] Repository.

<br />

<h2 class="blue" >Table of Contents</h2>

1. Test official example to read sitemap.xml
2. How to modify it to write **sitemap.txt**
3. **Conclusion**

---

<br />

## 1. Test official example to read sitemap.xml

We will first test the official example to read **sitemap.xml**. Then, we will find the payload we want to use from it. I hope you already executed it in your local machine.

The code snippet below is from the [Rust Sitemap Crate].

```rust
extern crate sitemap;
use sitemap::reader::{SiteMapReader,SiteMapEntity};
use std::fs::File;
fn main() {
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
    println!("urls = {:?}",urls);
    println!("sitemaps = {:?}",sitemaps);
    println!("errors = {:?}",errors);
}
```

If you test this code snippet, you should see some results at your console. It reads the **sitemap.xml**. Then, show important parts such as urls, other sitemaps and errors if they exist.

<br />

## 2. How to modify it to write sitemap.txt

In the previous part, we tested the API from [Rust Sitemap Crate] to read **sitemap.xml**.

For **sitemap.txt** just needs **urls** separated by new lines(\n), we will only use the `urls` variable to build **sitemap.txt**.

```rust
extern crate sitemap;
use sitemap::reader::{SiteMapReader,SiteMapEntity};
use std::fs::{write, File};
fn main() {
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

    println!("payload = {:?}", urls[0].loc.get_url().unwrap()); // 1.

    let mut output = String::new(); // 2.

    for url in urls {
        let payload = url.loc.get_url().unwrap();
        println!("{}", &amp;payload);
        let payload_with_new_line = format!("{}\n", &amp;payload);
        output.push_str(&amp;payload_with_new_line);
    }

    // 3.
    println!("{:#?}", &amp;output);
    write("sitemap.txt", &amp;output)?;

    println!("errors = {:?}", errors);
}
```

The entire code isn't that different from the official example. What we should do were

1. Get urls from **sitemap.xml** with API from [Rust Sitemap Crate].

2. Then we create mutable variable **output** with **let mut output = String::new()**; to save data. Then, we use `for in` loop again to use data inside **urls** variable processed with API from [Rust Sitemap Crate].

3. Finally, we verfiy everything worked well with **println!("{:#?}", &amp;output);**. Then, write sitemap.txt with **write("sitemap.txt", &amp;output)?;**

In the process, we didn't remove **println!("errors = {:?}", errors);** to verify errors if there are any.

You can run them with **$cargo run --bin main** and it will be similar to

```txt
http://www.steadylearner.com/video/search/*
http://www.steadylearner.com/video/watch/*
http://www.steadylearner.com/video/write/*
http://www.steadylearner.com/image/search/*
http://www.steadylearner.com/blog/search/*
http://www.steadylearner.com/blog/read/*
http://www.steadylearner.com/code/search/*
http://www.steadylearner.com/static/images/*
http://www.steadylearner.com/
http://www.steadylearner.com/about
http://www.steadylearner.com/video
http://www.steadylearner.com/blog
http://www.steadylearner.com/code
http://www.steadylearner.com/image
http://www.steadylearner.com/slideshow
http://www.steadylearner.com/blog/read/How-to-build-a-sitemap-for-React
http://www.steadylearner.com/blog/read/How-to-extract-audio-from-the-video-with-Python
http://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
http://www.steadylearner.com/blog/read/From-React-class-to-function
http://www.steadylearner.com/blog/read/How-to-make-a-video-with-Python
http://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
http://www.steadylearner.com/blog/read/How-to-install-Rust
```

or visit [txt sitemap](https://www.steadylearner.com/sitemap.txt) for [Steadylearner].

(**sitemap.txt allows you to use all selector * to define urls also**. If you want, you can use them also)


If you want to include it inside your former projects, You may refer to the code snippet below.

```rust
// main.rs or sitemap.rs

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
            .expect("valid");
        urlwriter.url(home_entry).expect("Unable to write url");

        let static_routes: Vec<&amp;str> = vec![
            "about",
            "video",
            "blog",
            "code",
            "image",
            "slideshow",
            "markdown",
        ];

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

        // assigining value to sitemap_writer is important to make the other processes work
        let sitemap_writer = urlwriter.end().expect("close the urlset block");
    }

    println!("{:#?}", std::str::from_utf8(&amp;output));

    write("sitemap.xml", &amp;output)?;

    // sitemap.txt is based on the sitemap.xml and it is already built at the point
    println!("You wanna write sitemap.txt file also?");
    println!("Type [yes] to write sitemap.txt or [no] to end the process");

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

and that is all. You can test it with `Cargo c or cargo run` and verify the result. Then, modify it for your project.

<br />

## 3. Conclusion

In other posts, we built **sitemap.xml** with [static routes and the datas from the database][sitemap].

With this post, you should be able to build **sitemap.txt** easily with those **sitemap.xml** files.

In the next post, we will learn how to build **sitemap.xml** file for images.

The entire process will be similar.

**Thanks and please share this post with others.**
