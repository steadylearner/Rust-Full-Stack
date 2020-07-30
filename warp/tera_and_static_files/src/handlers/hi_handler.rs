use warp::{reply, Reply, Rejection};
use tera::{Context};

use crate::{
    template_setup::tera::{render},
};

pub async fn hi(name: String) -> Result<Box<dyn Reply>, Rejection> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    let payload = render("hi.tera", &ctx)?;
    Ok(Box::new(reply::html(payload)))
}