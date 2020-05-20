use actix_web::{
    web,
    HttpRequest, HttpResponse,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
    pub number: i32,
}

/// This handler uses json extractor with limit
pub fn index(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}