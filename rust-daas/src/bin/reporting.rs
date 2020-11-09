extern crate actix_web;

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::middleware::Logger;

use std::{fs, io};
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
use serde_json::value::Value;

static ALL_PRODUCTS: &str = "all";
static WORKSPACE_LOCAL_STORAGE: &str = "./workshop_storage";

fn read_file(file: String) -> Option<Value> {
    let mut file = match File::open(file) {
        Ok(f) => {
            f
        },
        Err(_e) => return None,
    };
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    Some(serde_json::from_str(&contents).unwrap())
}

fn extract_product_name(file_path: String) -> String {
    file_path
        .replace(&format!("{}/clothing-", WORKSPACE_LOCAL_STORAGE),"")
        .replace(".json","")
        .replace("_"," ")
}

async fn index(req: HttpRequest) -> HttpResponse {
    let product = req.match_info().get("product").unwrap_or(ALL_PRODUCTS);
    
   let content = match &product {
        &"all" => {
            let mut products = Vec::new();
        
            let entries = fs::read_dir(WORKSPACE_LOCAL_STORAGE).unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>().unwrap();
                
            for entry in entries.iter() {
                let file = entry.to_str().unwrap().to_string();
                let mut obj = read_file(file.clone()).unwrap();
                obj.as_object_mut()
                    .unwrap()
                    .insert("product".to_string(), Value::String(extract_product_name(file)));
                
                products.push(obj);
            }
            
            json!(products).to_string()
        },
        _ => {
            match read_file(product.to_string()) {
                Some(cntnt) => {
                    cntnt.to_string()
                },
                None => {
                    "{\"orders\":6}".to_string()
                },
            }
        },
    };
    
    HttpResponse::build(StatusCode::OK)
        .body(&content)
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