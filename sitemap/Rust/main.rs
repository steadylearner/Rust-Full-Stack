#![feature(
    proc_macro_hygiene,
    decl_macro,
    custom_attribute,
    rustc_private,
    type_ascription
)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

// native moduels, use rayon later instead?
use std::thread;

// custom
extern crate sl_lib;
use sl_lib::*;

extern crate tera;

extern crate serde;
extern crate serde_json;
// #[macro_use]
extern crate serde_derive;

// environment variable and api etc
extern crate dotenv;

// HTTP
extern crate reqwest;

// for sitemap and iamge_sitemap
extern crate chrono;
extern crate sitemap;
// don't include it here because it colides
// extern crate diesel

// ws
extern crate ws;

// automation with time
// https://github.com/alexcrichton/futures-timer
// https://github.com/rust-lang-nursery/futures-rs

extern crate futures;
extern crate futures_timer;

use std::time::Duration;

use futures::prelude::*;
use futures_timer::Interval;

//

mod route;
use crate::route::{error, get, static_files};

mod web_service;
use crate::web_service::{blog, code, image, video, vlog};

use rocket_contrib::databases::diesel;
use rocket_contrib::templates::Template;

mod chat;
use crate::chat::ws_rs;

// test
mod auto_sitemap;
use crate::auto_sitemap::sitemap_renewal;

mod auto_image_sitemap;
use crate::auto_image_sitemap::image_sitemap_renewal;

// https://rocket.rs/v0.4/guide/state/#databases
#[database("postgres_logs")]
pub struct LogsDbConn(diesel::PgConnection);

// #[cfg(test)]
// mod tests; // Write your own module

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        // StaticFiles::from("/static")

        // Path for static single page app
        get::index,
        //
        get::about,
        get::abouts,
        //
        get::video,
        get::videos,
        // Redirect video/watch and video/write
        get::vr,
        get::vw,
        //
        get::image,
        get::slideshow,
        //
        get::blog,
        get::blogs,
        //
        get::code,
        get::markdown,
        get::jsx,
        //

        // get::javascript_1,
        get::javascript_2,
        get::javascript_3,
        get::javascript_4,
        get::javascript_5,
        get::javascript_6,
        get::javascript_7,
        // Just to html to work as websocket client
        get::chat,
        // CRA or separate single page app to reduce entire webpage javascript size
        get::react_easy_md,
        // sitemaps
        get::xml_sitemap,
        get::txt_sitemap,
        // JSON web service
        video::search_video_by_id::by_id,
        video::search_videos_by_title::by_title,
        video::search_video_counts::count,
        vlog::read_vlog::read,
        image::read_image::read,
        image::search_images::by_title,
        image::popular_and_latest_images::popular,
        image::popular_and_latest_images::latest,
        blog::read_post::read,
        blog::search_posts::by_title,
        blog::popular_and_latest_posts::popular,
        blog::popular_and_latest_posts::latest,
        code::read_code::read,
        code::search_codes::by_title,
        code::popular_and_latest_codes::popular,
        code::popular_and_latest_codes::latest,
    ];

    rocket::ignite()
        .manage(sl_lib::init_pool())
        .attach(Template::fairing())
        .attach(LogsDbConn::fairing())
        .mount("/", rocket_routes)
        .register(catchers![error::not_found])
}

// https://rust-lang.github.io/async-book/getting_started/why_async.html
fn main() {
    // $rustapi and search thread
    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        // .stack_size(83886 * 1024) // 80mib in killobytes
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    // https://api.rocket.rs/v0.4/rocket/config/enum.Environment.html

    // https://docs.rs/futures/0.2.3-docs-yank.4/futures/
    // https://crates.io/crates/futures-timer
    // to automate sitemap renewal with interval
    thread::Builder::new()
        .name("Test automation with futures-timer".into())
        .spawn(|| {
            Interval::new(Duration::from_secs(4))
                .for_each(|()| {
                    image_sitemap_renewal()?;
                    let static_paths = vec!["about", "video", "blog", "code", "image", "slideshow"];
                    let other_sitemaps = vec!["image_sitemap.xml"];
                    sitemap_renewal(static_paths, other_sitemaps)
                })
                .wait()
                .unwrap();
        })
        .unwrap();

    // .for_each(|()| Ok(println!("printed after four seconds")))

    rocket().launch();
}
