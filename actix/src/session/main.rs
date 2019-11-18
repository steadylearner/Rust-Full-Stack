extern crate actix_web;
// https://docs.rs/actix-identity/0.1.0/actix_identity/
extern crate actix_identity;
extern crate actix_rt;
// https://docs.rs/actix-session/0.2.0/actix_session/struct.Session.html
// https://docs.rs/actix-web/0.7.19/actix_web/http/struct.CookieBuilder.html, https://docs.rs/actix-web/0.7.19/actix_web/struct.HttpResponse.html#method
extern crate actix_session;
extern crate console;

use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};

use console::Style;

const PORT: &str = "0.0.0.0:8000";

/// simple index handler with session
fn index(session: Session, req: HttpRequest) -> Result<&'static str> {
    println!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        if counter == 10 {
            session.remove("counter");
        } else {
            session.set("counter", counter)?;
        }
    } else {
        session.set("counter", counter)?;
    }

    Ok("welcome!")
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix_rt::System::new("cookie-session");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(Logger::default())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index))
    })
    .bind(&PORT)?
    .start();

    let blue = Style::new()
        .blue();

    println!("\nServer ready at {}", blue.apply_to(format!("http://{}", PORT)));
    sys.run()
}
