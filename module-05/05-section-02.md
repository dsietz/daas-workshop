# Section II - library \(old\)

> [lib.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/lib.rs)

Since we didn't declare any new external dependencies in the `Cargo.toml` file, we don't have any new declarations in the `lib.rs` file. However, we do need to add the _processor_ module to our library, we declare it at the bottom of our `src/lib.rs` file, \(after the sourcing module\).

```text
pub mod processor;
```

The `lib` file should now look like the following:

```text
extern crate log;
extern crate env_logger;
extern crate actix_web;

static VER: &str = "v1";

pub mod hello_world;
pub mod processor;
```

