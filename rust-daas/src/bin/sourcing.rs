extern crate daas;
extern crate actix_web;

use daas::service::listener::{DaaSListener, DaaSListenerService};
use daas::service::extractor::{Base64Author};
use pbd::dua::middleware::actix::*;
use pbd::dtc::middleware::actix::*;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("DAAS_LOCAL_STORAGE", "../local_storage");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| App::new()        
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DUAEnforcer::default())
            .wrap(DTCEnforcer::default())
            .service(
                web::resource(&DaaSListener::get_service_health_path()).route(web::get().to(DaaSListener::health))
            )
            .service(
                web::resource(&DaaSListener::get_service_path()).route(web::post().to(DaaSListener::index::<Base64Author>))
            )
        )
    .bind("localhost:8000")
    .unwrap()
    .run()
    .await
}