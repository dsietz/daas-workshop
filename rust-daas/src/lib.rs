extern crate log;
extern crate env_logger;
extern crate actix_web;

use actix_web::middleware::Logger;

static VER: &str = "v1";

pub mod hello_world;
