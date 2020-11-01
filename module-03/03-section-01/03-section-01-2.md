# Section I - library

> [lib.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/lib.rs)

We will also declare these dependencies in our share `src/lib.rs` library with macros in use.

```text
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate daas;
extern crate pbd;
```

