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

We next need to create some supportive functions outside of the `main` function:

```text
fn create_local_storage() {
    match fs::create_dir_all(WORKSPACE_LOCAL_STORAGE) {
        Ok(_) => {},
        Err(err) => {
            panic!("Warning: Could not create the local directory path {} : {:?}", WORKSPACE_LOCAL_STORAGE, err);
        },
    }
}
```

```text
fn read_file(product_name: String) -> Option<Value> {
    let path = format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name);
    let mut file = match File::open(path) {
        Ok(f) => {
            println!("Retreiving {} file", product_name);
            f
        },
        Err(_e) => return None,
    };
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    Some(serde_json::from_str(&contents).unwrap())
}
```

```text
fn save_file(product_name: String, content: String) -> std::io::Result<()>{
    let mut file = File::create(format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name))?;
    file.write_all(content.as_bytes())
}
```

With all the `use` declarations and supportive functions in place, we can now start modfing the `main` function.

We first call the function to create the local storage directory when the service starts. This code can be added after the `parameters` section in the `main` function.

```text
    // Create the local storage directory for the aggregated data
    create_local_storage();
```

To add our business logic, \(inside the `callback` function after the `println` we were using to confirm the service is working correctly\) we add the foloowing lines of code:

```text
let prd = order.get("product").unwrap().as_str().unwrap().replace(" ","_").replace("/","");
            let qnty = order.get("quantity").unwrap().as_u64().unwrap();
            let content = match read_file(prd.clone().to_string()){
                Some(mut obj) => {
                    obj["orders"] = json!(obj["orders"].as_u64().unwrap() + qnty);
                    obj
                },
                None => {
                    let c = &format!("{{\"orders\":{:?}}}", qnty); 
                    println!("{}",c);
                    serde_json::from_str(c).unwrap()
                },
            };
```

