# Section IV - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

We already have the binary file defined in the manifest file, but there are dependent packages that we will need to include in order to make it a RESTful service. 

> Since [Logs](https://12factor.net/logs) is eleventh factor in a 12 Factor Application, we will enable this attribute by implementing automated logging for this RESTful endpoint. by including the [`log`](https://crates.io/crates/log) and [`env_logger`](https://crates.io/crates/env_logger) creates.

In the **\[dependencies\]** section of the `Cargo.toml` file add the following packages.

```text
log = "0.4"
env_logger = "0.8"
actix-web = "3"
```



