<!--
  Post{
    subtitle: "Learn to build simple static sitemap.xml with Rust",
    image: "post/sitemap/static-sitemap-example.png",
    image_decription: "Image by Steadylearner",
    tags: "Rust How build xml",
  }
-->

<!-- Link -->

[Rust Sitemap Crate]: https://github.com/svmk/rust-sitemap
[Writing Sitemap with Rust example]: https://github.com/svmk/rust-sitemap/blob/master/tests/test_write_sitemap.rs
[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[Rust Diesel]: http://diesel.rs/
[Steadylearner Sitemap]: https://github.com/steadylearner/Sitemap
[Cargo Edit]: https://github.com/killercup/cargo-edit

[image sitemap]: https://www.steadylearner.com/image_sitemap.xml
[xml sitemap]: https://www.steadylearner.com/sitemap.xml
[txt sitemap]: https://www.steadylearner.com/sitemap.txt

<!-- / -->

<!-- Steadylearner Posts -->

[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[How to build a sitemap for React]: https://www.steadylearner.com/blog/read/How-to-build-a-sitemap-for-React
[sitemap]: https://www.steadylearner.com/blog/search/sitemap

<!-- / -->

In this post, You will learn how to build a simple static sitemap.xml flle with Rust. If you are a React developer, you may refer to [How to build a sitemap for React] at [Steadylearner] first.

Because you wouldn't need [sitemap] without building your complete website, I will suppose that you already have experience in Rust or other programming languages.

We will eventually learn how to build **sitemap.xml, sitemap.txt and image_sitemap.xml**. Then, we will make the functions to make the process reusable.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Rust Sitemap Crate]
3. [What is sitemap](https://support.google.com/webmasters/answer/156184?hl=en)
4. [How to build a sitemap](https://www.google.com/search?client=firefox-b-d&q=how+to+build+sitemap)

---

If you don't have Rust installed yet, please visit [How to install Rust] first. Then, I want you to read the doucumentation at [Rust Sitemap Crate].

If you just want to see the final project, you may refer to [Steadylearner Sitemap]. 

You may visit [xml sitemap], [txt sitemap], [image sitemap] pages to verify the results.

<br />

<h2 class="blue">Table of Contents</h2>

1. The official example to write sitemap.xml
2. Modify it to build sitemap with static routes
3. **Conclusion**

---

<br />

## 1. The official example to write sitemap.xml

I hope you already tested [Rust Sitemap Crate] example in your machine. It has a feature to read sitemaps also. But we won't use it in this post.

So we will just talk about the example code to write **sitemap.xml** here.

```rust
extern crate sitemap;
use sitemap::writer::SiteMapWriter;
use sitemap::structs::UrlEntry;
use std::io::stdout;
fn main() {
    let mut output = stdout();
    let sitemap_writer = SiteMapWriter::new(&mut output);
    let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
    urlwriter.url("http://github.com").expect("Unable to write url");
    urlwriter.url(UrlEntry::builder().loc("http://google.com")).expect("Unable to write url");
    urlwriter.url(UrlEntry::builder().loc("http://yandex.ru").build().unwrap()).expect("Unable to write url");
    urlwriter.end().expect("Unable to write close tags");
}
```

with `$cargo run --bin main` You already have seen some results at your console. But, just showing it is not enough.

So in the next part, we will modify the code above to write static sitemaps. I want you to read source code at [Writing Sitemap with Rust example].

<br />

## 2. Modify it to build sitemap with static routes

We will write more complicated code to write sitemap.xml. But, it is just a variation from [the example][Writing Sitemap with Rust example].

We will first define **what_is_date_today** function to use inside main.rs file later.

```rust
// lib.rs
extern crate chrono;
use chrono::prelude::Utc;

#[derive(Debug)]
pub struct Today {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub fn what_is_date_today() -> Today {
    let date = Utc::now().naive_utc();
    let date_str = format!("{}", &date);
    let split_date = date_str.split(" ");
    let vec_of_split_date: Vec<&str> = split_date.collect();

    let payload = vec_of_split_date[0];

    let split_payload = payload.split("-");
    let vec_of_split_payload: Vec<&str> = split_payload.collect();

    let (y, m, d) = (vec_of_split_payload[0], vec_of_split_payload[1], vec_of_split_payload[2]);

    println!("Today is {}-{}-{}(year-month-day\n)", &y, &m, &d);

    Today {
        year: y.parse::<i32>()
                .expect("Invalid Number"),
        month: m.parse::<u32>()
                .expect("Invalid Number"),
        day: d.parse::<u32>()
                .expect("Invalid Number"),
    }
}
```

Then, **main.rs** will be

```rust
extern crate chrono;
extern crate console;
extern crate sitemap;

use std::fs::{write, File};

use console::Style;

use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap::reader::{SiteMapEntity, SiteMapReader};
use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use sitemap::writer::SiteMapWriter;

fn main() -> std::io::Result<()> {
    let bold = Style::new().bold(); // 1.
    let website: &str = "http://www.steadylearner.com";
    let static_routes: Vec<&str> = vec![
      "about",
      "video",
      "blog",
      "code",
      "image",
      "slideshow",
      "markdown"
    ]; // 2.

    let mut output = Vec::<u8>::new();
    {
        let sitemap_writer = SiteMapWriter::new(&mut output);

        let mut urlwriter = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

        let today = what_is_date_today(); // 3.

        let date = DateTime::from_utc(
            NaiveDate::from_ymd(today.year, today.month, today.day).and_hms(0, 0, 0),
            FixedOffset::east(0),
        );

        // 4.
        let home_entry = UrlEntry::builder()
            .loc(website)
            .changefreq(ChangeFreq::Monthly)
            .lastmod(date)
            .build()
            .expect("valid");
        urlwriter.url(home_entry).expect("Unable to write url");

        // 5.
        for route in static_routes.iter() {
            let static_url = format!("{}/{}", &website, route);
            let url_entry = UrlEntry::builder()
                .loc(static_url)
                .changefreq(ChangeFreq::Monthly)
                .lastmod(date)
                .build()
                .expect("valid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        // 6.
        let sitemap_writer = urlwriter.end().expect("close the urlset block");
    }

    println!("{:#?}", std::str::from_utf8(&output)); // 7.

    write("sitemap.xml", &output)?; // 8.

    Ok(())
}
```

What we do are

1. Include it to decorate output from the console.

2. Make variables for static routes to use them to build **sitemap.xml**.

3. Define custom functions not to manually type current time repeatedly.

4. Priority is removed because some search engines ignore it. You may include it.

5. Use **for** to iter through static routes and build sitemap with them.

6. You don't need it for this to work. But you need it later.

7. Verify result at console.

8. Then, we wirte it to **sitemap.xml**.

You can test it in your machine. Then, you will see **sitemap.xml** in your directory.

It would be similar to

```xml
<?xml version="1.0" encoding="utf-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>http://www.steadylearner.com/</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/about</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/video</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/blog</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/code</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/image</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/slideshow</loc>
    <changefreq>monthly</changefreq>
  </url>
  <url>
    <loc>http://www.steadylearner.com/markdown</loc>
    <changefreq>monthly</changefreq>
  </url>
</urlset>
```

<br />

## 3. Conclusion

I hope it worked. Use your own website name and static routes.

In the next posts for [sitemap] with Rust, you will learn how to make sitemap.xml with datas and make some functions.

**Thanks and please share this post with others.**
