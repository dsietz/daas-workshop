extern crate actix_web;

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::middleware::Logger;

static ALL_PRODUCTS: &str = "all";

async fn index(req: HttpRequest) -> HttpResponse {
    let product = req.match_info().get("product").unwrap_or(ALL_PRODUCTS);
    //let name = req.match_info().get("name").unwrap_or("World");
    
    match &product {
        &"all" => {
            HttpResponse::build(StatusCode::OK)
                .body(ALL_PRODUCTS.to_string())
        },
        _ => {
            HttpResponse::build(StatusCode::BAD_REQUEST)
                .body(&product.to_string())
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/{product}", web::get().to(index))
    })
    .bind("localhost:8001")?
    .run()
    .await
}