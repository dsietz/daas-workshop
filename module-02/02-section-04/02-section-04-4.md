# Section IV - executable

[hello-world.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/bin/hello-world.rs)

Once we have created our service and all the tests have passed, we are ready to build out the executable and run our service.

#### Add Log functionality

> Since [Logs](https://12factor.net/logs) is eleventh factor in a 12 Factor Application, we will enable this attribute by implementing automated logging for this RESTful endpoint. by including the [`log`](https://crates.io/crates/log) and [`env_logger`](https://crates.io/crates/env_logger) creates.

We start by first adding the `log` and `env_logger` crates to the `Cargo.toml` manifest.

```text
[dependencies]
log = "0.4"
env_logger = "0.8"
actix-web = "3"
```

Next, we update the `lib.rs` file to include the logging creates and modules.

We place the `extern crate` declarations for these crates at the top \(so that they are shared in the project\).

```text
extern crate log;
extern crate env_logger;
extern crate actix_web;
```

The final `lib.rs` file should look like this:

```text
extern crate log;
extern crate env_logger;
extern crate actix_web;

static VER: &str = "v1";

pub mod hello_world;
```

#### Writing the executable

Now that we have the dependent crates and modules declared in our library, we can call the service in an executable.

In the `hello-world.rs` file in the `bin` directory, rewrite the file so it looks like the following:

```text
use daas::hello_world;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .route(
            &hello_world::get_service_path(), 
            web::get().to(hello_world::index)))
        .bind("127.0.0.1:7999")?
        .run()
        .await
}
```

> Noteworthy: we call the module and its functionality by using the following code snippets:
>
> * `use daas::hello_world;`
> * `&hello_world::get_service_path()`
> * `hello_world::index`

Make sure all your tests are still passing by using the `cargo test` command.

#### Starting the service

We are now ready to start the RESTful service. There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(local service testing\)

```text
PS C:\workspace\rust-daas> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target\debug\hello_world.exe`
```

Open your browser and navigate to the URL: [http://localhost:7999/hello/v1/](http://localhost:7999/hello/v1/). You should see the message `Hello World!` On the command line, you will notice that the calls are being logged and printed to the console.

```text
[2020-11-01T16:49:19Z INFO  actix_web::middleware::logger] 127.0.0.1:53702 Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:82.0) Gecko/20100101 Firefox/82.0
[2020-11-01T16:49:19Z INFO  actix_web::middleware::logger] 127.0.0.1:53702 "GET /hello/v1/ HTTP/1.1" 200 12 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:82.0) Gecko/20100101 Firefox/82.0" 0.001153
```

To stop the service, use `ctrl` + `c`.

   2. Running using the executable.

```text
PS C:\workspace\rust-daas> cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the Cargo.toml manifest.

Since it is an executable, simple run the executable from the command terminal, and make the same URL call from the browser.

> NOTE: Example below is for Windows.

```text
C:\workspace\demo\rust-daas\target\debug>hello_world.exe
[2019-10-23T14:49:19Z INFO  actix_web::middleware::logger] 127.0.0.1:65360 Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36
[2019-10-23T14:49:19Z INFO  actix_web::middleware::logger] 127.0.0.1:65360 "GET /hello/v1/ HTTP/1.1" 200 12 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36" 0.000948
```

