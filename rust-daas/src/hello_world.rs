use super::*;
use actix_web::{HttpRequest, HttpResponse };
use actix_web::http::{StatusCode};

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
    .body("Hello World!".to_string())
}

#[cfg(test)]
mod tests {
   use super::*;
   #[allow(unused_imports)]
   use actix_web::{test};

   #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }

   #[test]
    fn hello_response() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        let resp = index(req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}