// https://tera.netlify.com/docs
// You can also refer to this.
// https://github.com/seanmonstar/warp/blob/master/examples/handlebars_template.rs

use tera::{Tera, Context};
use warp::{
    self,
    reject::{custom, Reject},
    Rejection,
};

lazy_static! {
    pub static ref TERA: Tera = {
        let target = "src/views/**/*";
        let mut tera = match Tera::new(&target) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // https://www.google.com/search?client=firefox-b-d&q=what+is+HTML+escaping
        tera.autoescape_on(vec!["html", ".sql"]); // Should be .tera later?
        // https://docs.rs/tera/0.5.0/tera/struct.Tera.html#method.register_filter
        // tera.register_filter("do_nothing", do_nothing_filter); // What this do?
        tera
    };
}

// You should write more code here following the documenation of Warp.
#[derive(Debug)]
struct TemplateError;
impl Reject for TemplateError {}

pub fn render(name: &str, ctx: &Context) -> Result<String, Rejection> {
    TERA.render(name, &ctx).or(Err(custom(TemplateError)))
}