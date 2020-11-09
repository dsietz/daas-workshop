# Section V - adding the business logic

> [reporting.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/bin/reporting.rs)

Now that we know the service is reading the path parameter `product` we can add our business logic to retrieve the JSON payload.

Add additional `use` declarations:

```rust
use std::{fs, io};
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
use serde_json::value::Value;
```

Add a new constant as a global variable

```rust
static WORKSPACE_LOCAL_STORAGE: &str = "./workshop_storage";
```

Include a supportive function after the outside of the `main` function.

```rust
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
```

Modify the `index` function with the new business logic.

