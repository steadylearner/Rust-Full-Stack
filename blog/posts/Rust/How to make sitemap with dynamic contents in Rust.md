<!--
    Post{
        subtitle: "Use datas to build sitemap.xml with Rust",
        image: "code/Rust_r.png",
        image_decription: "Image by Steadylearner",
        tags: "Rust How sitemap code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Rust Sitemap Crate]: https://github.com/svmk/rust-sitemap
[Rust Diesel]: http://diesel.rs/

[Steadylearnr Sitemap]: https://github.com/steadylearner/Sitemap

[image sitemap]: https://www.steadylearner.com/image_sitemap.xml
[xml sitemap]: https://www.steadylearner.com/sitemap.xml
[txt sitemap]: https://www.steadylearner.com/sitemap.txt

<!-- / -->

<!-- Steadylearner Post -->

[How to build a sitemap for React]: https://www.steadylearner.com/blog/read/How-to-build-a-sitemap-for-React
[Static sitemap.xml with Rust]: https://www.steadylearner.com/blog/read/Static-sitemap.xml-with-Rust

[sitemap]: https://www.steadylearner.com/blog/search/sitemap

<!-- / -->

Writing a code to build sitemaps with dynamic contents seem to be solved problem already. There were already many automated ways for that also.

However, [It][Steadylearner] is a React single page app with Rust backend and there were not so many examples yet.

So I searched and have found that there is already [a sitemap builder in React development environment][Sitemap in React].

I tried to use it. But it was not that helpful because it just extracts static paths defined by a programmer. It was not for the website with dynamic contents and many routes made from them.

Then, I thought that it would be better to find the solution at server side with **Rust** because I already use [Rust Diesel] to handle database. I thought that I could use datas from it to build **sitemap.xml** for [Steadylearner].

I planned to write Rust code to make it but found that there is already [Rust Sitemap Crate] for sitemaps in **Rust**.

I could make [xml sitemap], [txt sitemap] and [image sitemap] with it. I want to share what I learnt from [the process to build them][sitemap].

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [Rust Sitemap Crate]
2. [Rust Diesel]
3. [Static sitemap.xml with Rust]
4. [What is sitemap](https://support.google.com/webmasters/answer/156184?hl=en)
5. [How to build a sitemap](https://www.google.com/search?client=firefox-b-d&amp;q=how+to+build+sitemap)

---

I want you to visit [Rust Diesel] and build CLI for blog posts. Then, read the documentation for [Rust Sitemap Crate].

You may read previous post [Static sitemap.xml with Rust] if you haven't.

I will suppose that you already have experience in programming.

If you just want to see the final project, you may refer to [Steadylearner Sitemap] repository.

<br />

<h2 class="blue" >Table of Contents</h2>

1. Setup Diesel and Rust files
2. How to write sitemap with database with **Rust Diesel**
3. **Conclusion**

---

<br />

## 1. Setup Diesel and Rust files

We will first prepare code for [Rust Diesel]. Then, we will build **sitemap.xml** with database.

You may skip or use your own project instead.

Following the tutorial given by its author, You should have code snippet similar to this.

```rust
// lib.rs
extern crate dotenv;
use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use chrono::prelude::Utc;

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub fn init_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}
```

It maybe different from your project depending on the database you use.

You should have used this function inside your CLI for blog posts already. It is to call datas from the database such as blog posts.

```rust
// models.rs
use crate::schema::{posts};

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
}
```

You should define at least **title** for **Post** struct. We will use it to write **sitemap.xml** with various titles from the posts you built. It may be differ from your project and you may modify it.

<br />

## 2. How to write sitemap with database with **Rust Diesel**

In this part, we will write some codes to read datas from the database and write **sitemap.xml** from it.

If you read the [previous post][Static sitemap.xml with Rust], you will find that the major difference is "we just set paths **with datas from the database**".

```rust
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

// use yours instead
use sl_lib::models::*;
use sl_lib::*;
use sl_lib::custom::str_from_stdin;

// pub fn str_from_stdin() -> String { // Use it to remove \n at the end of the user input
//     let mut var = String::new();
//     stdin().read_line(&amp;mut var).unwrap();
//     let var = var[..(var.len() - 1)].to_string();

//     var
// }

use crate::schema::posts::dsl::*;

fn main() -> std::io::Result<()> {
    let bold = Style::new().bold();
    let website: &amp;str = "http://www.steadylearner.com";
    let static_routes: Vec<&amp;str> = vec![
        "about",
        "video",
        "blog",
        "code",
        "image",
        "slideshow",
        "markdown",
    ];

    // 1.
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
            .loc(website)
            .changefreq(ChangeFreq::Monthly)
            .lastmod(date)
            .build()
            .expect("valid");
        urlwriter.url(home_entry).expect("Unable to write url");

        for route in static_routes.iter() {
            let static_url = format!("{}/{}", website, route);
            let url_entry = UrlEntry::builder()
                .loc(static_url)
                .changefreq(ChangeFreq::Monthly)
                .lastmod(date)
                .build()
                .expect("valid");

            urlwriter.url(url_entry).expect("Unable to write url");
        }

        // 2.
        for post in post_results {
            let post_url = format!(
                "{}/blog/read/{}",
                website,
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

        let sitemap_writer = urlwriter.end().expect("close the urlset block");
    }

    println!("{:#?}", std::str::from_utf8(&amp;output));

    write("sitemap.xml", &amp;output)?;

    Ok(())
}
```

If you read the code from the previous post, it won't be difficult to find what it does.

The differences are

1. We connected main function to database. We already made code for them in the previous part. You should find that **the data** loaded from the process is saved at **post_results** so we can use it later.

2. We use `for in` loops in Rust again. Then, we are using datas to iterate from the datas from the database instead of hard coded static paths.

What is importnat here is that we define customized url **post_url** to pass for the **loc API** from [Rust Sitemap Crate].

(**post.title.replace(" ", "-")** is used here to make url be readable in React Frontend and Rust Backend also. You should use what is right for your project.)

When you make this project compile, you will find that the blog posts for sitemaps such as

```xml
<loc>https://www.steadylearner.com/blog/read/How-to-build-a-sitemap-for-React</loc>
<loc>https://www.steadylearner.com/blog/read/Static-sitemap.xml-with-Rust</loc>
```

are included for your **sitemap.xml**.

I want to you to test it with your static routes and the datas for your website.

If you hadn't any datas yet, you can verify it work at [xml sitemap] for [Steadylearner].

<br />

## 3. Conclusion

You already have the boiler plate to build sitemaps with static and the dynamic datas from the database with this post. you can build whatever sitemap with your Rust code skill.

You can apply process used here for other programming languages also. What you really need is just use `for in` or other loop with datas and preparation to use them.

In the next post for [sitemap], I will help you to write **sitemap.txt** file from the **sitemap.xml** we built. Some search engine uses it and maybe useful for you.

It won't be difficult.

**Thanks and please share this post with others.**
