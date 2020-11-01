# Section IV - module

> [hello\_world.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/hello_world.rs)

To create the module, create a new file named `hello_world.rs` in the **/src** directory.

To begin, we will follow some basic TDD practices and build our tests first.

> NOTE: This is not a TDD workshop, so we will ignore the complete practices and simply illustrate how it would be done.

At the bottom of the file, create an empty nested _testing_ module. This will be where we write our unit test for the hello\_world module. The use `super::*;` line imports all the functionality and variables from the parent `hello_world` module.

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

