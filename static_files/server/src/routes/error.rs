use rocket::Request;
use rocket_contrib::templates::Template;

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/not-found", &map)
}

// https://github.com/SergioBenitez/Rocket/tree/master/examples/tera_templates