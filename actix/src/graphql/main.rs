//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use std::sync::Arc;

#[macro_use]
extern crate juniper;
extern crate console;

use console::Style;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod type_defs;

mod resolvers;
use crate::resolvers::user::{create_schema, Schema};

const PORT: &str = "127.0.0.1:8088";

fn graphiql() -> HttpResponse {
    let html = graphiql_source(&format!("http://{}/graphql", &PORT));

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let blue = Style::new()
        .blue();

    println!("\nGraphiql ready at {}", blue.apply_to(format!("http://{}/graphiql\n", PORT)));

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(&PORT)?
    .run()
}

