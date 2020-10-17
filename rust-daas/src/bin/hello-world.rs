use daas::hello_world;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new( || App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .route(
            &hello_world::get_service_path(), 
            web::get().to(hello_world::index)))
    .bind("127.0.0.1:7999")
    .expect("Can not bind to port 7999")
    .run();
}