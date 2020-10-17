# Section IV - module

> [hello\_world.rs](https://github.com/dsietz/rust-daas/blob/master/src/hello_world.rs)

To create the module, create a new file named _**hello\_world.rs**_ in the **/src** directory.

To begin, we will follow some basic TDD practices and build our tests first.

> NOTE: This is not a TDD workshop, so we will ignore the complete practices and simply illustrate how it would be done.

At the bottom of the file, create an empty nested _testing_ module. This will be where we write our unit test for the hello\_world module. The use `super::*;` line imports all the functionality and variables form the parent hello\_world module.

```text
#[cfg(test)]
mod tests {
    use super::*;
}
```

Our first test will be to return the service root. Add the following test in tests module below the `use super::;` line so it looks like the following.

```text
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }
}
```

Following TDD practices, we now run our test and confirm that it will fail.

```text
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.1.0 (C:\tmp\rust-daas)
error[E0425]: cannot find function `get_service_root` in this scope
 --> src\hello_world.rs:8:20
  |
8 |         assert_eq!(get_service_root(), format!("/hello/{}", VER));
  |                    ^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `VER` in this scope
 --> src\hello_world.rs:8:61
  |
8 |         assert_eq!(get_service_root(), format!("/hello/{}", VER));
  |                                                             ^^^ not found in this scope
  |
help: consider importing this static
  |
4 |     use crate::VER;
  |

warning: unused import: `super::*`
 --> src\hello_world.rs:4:9
  |
4 |     use super::*;
  |         ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: static is never used: `VER`
 --> src/lib.rs:4:1
  |
4 | static VER: &str = "v1";
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rust-daas`.
```

To make the test pass, we will add the `get_service_root()` function to the module.

```text
use super::*;

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }
}
```

If we rerun our test, it will now pass.

```text
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.1.0 (C:\tmp\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 1.60s
     Running target\debug\deps\daas-dafe2c98359dbcd2.exe

running 1 test
test hello_world::tests::test_get_service_root ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\hello_world-f32c48dd7c679e2e.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests daas

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

We will do the same for the `get_service_path()` function.

```text
use super::*;

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }
}
```

Now that we have an understanding of how to write our tests, and then add the functionality to make them pass, we will move on to provide our service call.

Our test will be the following.

```text
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
```

In order to make it pass, we will need to import the web service modules, and provide a `index()` function.

```text
use actix_web::{HttpRequest, HttpResponse };
use actix_web::http::{StatusCode};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
    .body("Hello World!".to_string())
}
```

The final file should look like the following.

```text
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
```

Rerun the tests to make sure it all passes.

We now have one last step, which is to add a function that will provide the service object. This is will not be covered by unit testing and is instead should be covered by integrated testing.

To create integrated tests, first create a new file named _**web-service-tests.rs**_ in a new directory named _**tests**_ in the root path \(same level as the **src** directory\). Cargo will automatically parse the _**tests**_ directory and run any tests that are located in any files located here.

```text
.
|-- .git
|-- .gitignore
|-- src
     |-- bin
          |-- hello-world.rs
     |-- hello_world.rs
     |-- lib.rs
|-- tests
     | -- web-service-tests.rs
|-- Cargo.toml
```

In order to execute our service test, we will first need to include the `actix-rt` library to our project. We do this by adding the line `actix-rt = "1.1"` in the `[dev-dependencies]` section of the _**Cargo.toml**_ file.

```text
[dependencies]
hyper = "0.13.8"
actix-web = "3"

[dev-dependencies]
actix-rt = "1.1"
```

Once the library has been included in the Manifest, we define which libraries are required in the _**web-service-tests**_ module by adding the following lines at the top of the _**web-service-tests.rs**_ file.

```text
extern crate actix_web;
```

The `extern` declarations specify the dependent crates \(or libraries\) that will be used in the _**web-service-tests**_ module.

We then declare the bindings \(or shortcuts\) to a resources that will be used in the _**web-service-tests**_ module. This is done by adding the following lines below the `extern` crate declarations.

```text
use daas::hello_world;
use actix_web::{test, web, App};
```

Now we can add the code for our Hello World service test, which is added below the `use` declarations.

```text
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
```

At this point the _**web-service-tests.rs**_ file should look like this:

```text
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
```

Try running your test with the `cargo test` command. There should now be a line in the results referencing that the `web_service_tests` has run.

```text
     Running target\debug\deps\web_service_tests-664800ae8a37eeb0.exe

running 1 test
test test_hello_world_ok ... ok
```

