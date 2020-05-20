use std::error::Error;
use tera::{Context, Tera};
use warp::{self, filters::BoxedFilter, path, Filter};
use warp::{
    reject::{custom, Reject},
    reply, Rejection, Reply,
};

fn create_template_filter() -> Result<BoxedFilter<(Tera,)>, Box<dyn Error>> {
    let tera = Tera::new("src/views/**/*.tera")?;
    // let tera = Tera::new("views/*.tera")?;
    Ok(warp::any().map(move || tera.clone()).boxed())
}

#[derive(Debug)]
struct TemplateError;
impl Reject for TemplateError {}

fn render(name: &str, tmpl: &Tera, ctx: &Context) -> Result<String, Rejection> {
    tmpl.render(name, &ctx).or(Err(custom(TemplateError)))
}

async fn hi(name: String, tmpl: Tera) -> Result<Box<dyn Reply>, Rejection> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    let payload = render("hi.tera", &tmpl, &ctx)?;
    Ok(Box::new(reply::html(payload)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tmpl = create_template_filter()?;
    // let pool = create_database_filter()?;

    let hi_route = warp::get()
        .and(path!("hi" / String))
        .and(tmpl.clone())
        .and_then(hi);

    let routes = warp::any().and(hi_route);
    Ok(warp::serve(routes).run(([0, 0, 0, 0], 8000)).await)
}