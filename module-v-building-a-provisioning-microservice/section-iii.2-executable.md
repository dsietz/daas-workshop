# Section III.2 - executable

> [order\_clothing.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/bin/order_clothing.rs)

Now that we have confirmed that the service is capturing and parsing the clothing order data correctly, we can add our business logic to the `main` function.

Let's first being by declaring some new uses

```text
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
```

We will also be using a constant to define where our aggregated data records will be stored. Once again, this could be configured as a command line agrument using the `clap` crate. 

```text
static WORKSPACE_LOCAL_STORAGE: &str = "./workshop_storage";
```

