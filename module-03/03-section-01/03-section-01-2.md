# Section I - library

> [lib.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/lib.rs)

We will also declare these dependencies in our share `src/lib.rs` library with macros in use.

```text
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate daas;
extern crate pbd;
```

We also need to provide a share function `get_unix_now()` that the daas module will need. So we declare the `use` of the dependencies and the function itself.

```text
use std::str;
use std::time::{SystemTime};
```

```text
fn get_unix_now() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) =>n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```

To add the _daas_ module to our library, we declare it at the bottom of our `src/lib.rs` file, \(after the hello\_world module\).

```text
pub mod daas;
```

