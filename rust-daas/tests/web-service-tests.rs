extern crate actix_web;

use daas::hello_world;
use actix_web::{test, web, App};

#[actix_rt::test]
async fn test_hello_world_ok() {

    let mut app = test::init_service(App::new().route("/", web::get().to(hello_world::index))).await;
    // create the request
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    // call the service
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    // read response body
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello World!".to_string());
}