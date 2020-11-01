# Section III - executable

> sourcing.rs

Since the SDKs contain all the modules we will need for our web service, we can go right to writing our executable: `src/bin/sourcing.rs`.

We start by declaring our dependent external crates

```rust
extern crate daas;
extern crate actix_web;
```

We then declare the modules we will be using.

```rust
use daas::service::listener::{DaaSListener, DaaSListenerService};
use daas::service::extractor::{Base64Author};
use pbd::dua::middleware::actix::*;
use pbd::dtc::middleware::actix::*;
use actix_web::{web, App, HttpServer};
```

Finally, we write the main function that will be called.

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("DAAS_LOCAL_STORAGE", "C:\\tmp");
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();
    
    HttpServer::new(|| App::new()
            .wrap(DUAEnforcer::default())
            .wrap(DTCEnforcer::default())
            .service(
                web::resource(&DaaSListener::get_service_health_path()).route(web::get().to(DaaSListener::health))
            )
            .service(
                web::resource(&DaaSListener::get_service_path()).route(web::post().to(DaaSListener::index::<Base64Author>))
            )
        )
    .bind("localhost:8088")
    .unwrap()
    .run()
    .await
}
```

Since the DaaS Listener that consumes the source data is loosely coupled to the broker, it is important that we keep a local copy of the DaaSDocument in case conneciton to the broker is lost. We configure the directory path the local storage using the environment variable `DAAS_LOCAL_STORAGE`. If this is not set, the DaaSListener module will use the system's default temporary directory.

```rust
std::env::set_var("DAAS_LOCAL_STORAGE", "C:\\tmp");
```

When you are finished, the `sourcing.rs` file should look like this:

```rust
extern crate daas;
extern crate actix_web;

use daas::service::listener::{DaaSListener, DaaSListenerService};
use daas::service::extractor::{Base64Author};
use pbd::dua::middleware::actix::*;
use pbd::dtc::middleware::actix::*;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("DAAS_LOCAL_STORAGE", "C:\\tmp");
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();
    
    HttpServer::new(|| App::new()
            .wrap(DUAEnforcer::default())
            .wrap(DTCEnforcer::default())
            .service(
                web::resource(&DaaSListener::get_service_health_path()).route(web::get().to(DaaSListener::health))
            )
            .service(
                web::resource(&DaaSListener::get_service_path()).route(web::post().to(DaaSListener::index::<Base64Author>))
            )
        )
    .bind("localhost:8088")
    .unwrap()
    .run()
    .await
}
```

