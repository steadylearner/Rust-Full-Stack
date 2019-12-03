use actix_web::{HttpRequest, HttpResponse, web, web::Json};

use crate::models::product::{ProductList, NewProduct, Product};

// curl http://127.0.0.1:8080/products
pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ProductList::list())
}

// curl http://127.0.0.1:8080/products -H "Content-Type: application/json" -d '{"name": "socks", "stock": 7, "price": 2}'
pub fn create(new_product: Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
    new_product
        .create()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

// curl http://127.0.0.1:8080/products/1
pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

// curl -X DELETE http://127.0.0.1:8080/products/1 -H "Content-Type: application/json"
pub fn destroy(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::destroy(&id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

// curl -X PATCH http://127.0.0.1:8080/products/3 -H "Content-Type: application/json" -d '{"stock": 8}'
pub fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
    Product::update(&id, &new_product)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}


