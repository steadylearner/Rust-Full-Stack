#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tera::{Context, Tera};
use warp::{self, filters::BoxedFilter, path, Filter};
use warp::{
    reject::{custom, Reject},
    reply, Rejection, Reply,
};

mod schema;

fn create_template_filter() -> Result<BoxedFilter<(Tera,)>, Box<dyn Error>> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*.html"))?;
    Ok(warp::any().map(move || tera.clone()).boxed())
}

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

fn create_database_filter() -> Result<BoxedFilter<(Pool,)>, Box<dyn Error>> {
    let url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(warp::any().map(move || pool.clone()).boxed())
}

#[derive(Deserialize, Serialize, Queryable)]
struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Debug)]
struct TemplateError;
impl Reject for TemplateError {}

fn render(name: &str, tmpl: &Tera, ctx: &Context) -> Result<String, Rejection> {
    tmpl.render(name, &ctx).or(Err(custom(TemplateError)))
}

async fn hello(name: String, tmpl: Tera) -> Result<Box<dyn Reply>, Rejection> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    let payload = render("hello.html", &tmpl, &ctx)?;
    Ok(Box::new(reply::html(payload)))
}

async fn tasks(pool: Pool, tmpl: Tera) -> Result<impl Reply, Rejection> {
    use crate::schema::tasks::dsl::*;
    let mut ctx = Context::new();
    let conn: &MysqlConnection = &pool.get().unwrap();
    match tasks.load::<Task>(conn) {
        Ok(items) => ctx.insert("tasks", &items),
        _ => (),
    }
    let payload = render("tasks.html", &tmpl, &ctx)?;
    Ok(reply::html(payload))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let tmpl = create_template_filter()?;
    let pool = create_database_filter()?;

    let hello_route = warp::get()
        .and(path!("hello" / String))
        .and(tmpl.clone())
        .and_then(hello);

    let tasks_route = warp::get()
        .and(warp::path::end())
        .and(pool.clone())
        .and(tmpl.clone())
        .and_then(tasks);

    let routes = warp::any().and(hello_route.or(tasks_route));
    Ok(warp::serve(routes).run(([127, 0, 0, 1], 3030)).await)
}