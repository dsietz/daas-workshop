# Seciton III - executable

> order\_clothing.rs

to writing our executable: `src/bin/order_clothing.rs`.

We start by declaring our dependent external crates

```rust
extern crate daas;
extern crate kafka;
```

We then declare the modules we will be using.

```rust
use std::io;
use std::thread;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaaSProcessor, DaaSProcessorMessage, DaaSProcessorService};
use std::sync::mpsc::{channel};
use serde_json::value::Value;
```

Next, we write the main function that will be called.

> NOTE: Normally we would follow coding discipline and avoud hard coding configurations, \(e.g.: `hosts` and `topic` \). However, for this workshop we will skip this practice for the sake of time. If you'd like to implement command line arguments when starting the service, you can use the [`clap`](https://crates.io/crates/clap) crate to do so.

```rust
fn main() {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();

    // configuration settings
    let hosts = vec!("localhost:9092".to_string());
    let topic = "order.clothing".to_string();

    // parameters
    let (tx, rx) = channel();
    let consumer = Consumer::from_hosts(hosts)
                            .with_topic(topic.clone())
                            .with_fallback_offset(FetchOffset::Earliest)
                            .with_group(format!("{}-consumer", topic.clone()))
                            .with_offset_storage(GroupOffsetStorage::Kafka)
                            .create()
                            .unwrap();

    // start the processor
    let _handler = thread::spawn(move || {
        DaaSProcessor::start_listening(consumer, &rx, None, |msg: DaaSProcessorMessage, _none_var , _t: Option<&i8>|{
            let mut doc = msg.doc;
            let order: Value = serde_json::from_str(&String::from_utf8(doc.data_obj_as_ref().to_vec()).unwrap()).unwrap();

            println!("Order Number {} from the {} has a status of {}...", doc.source_uid, doc.source_name, order.get("status").unwrap());
            Ok(1)
        });
    });

    println!("Clothing Orders processor is running ...");
    println!("Press [Enter] to stop the Clothing Orders processor.");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            DaaSProcessor::stop_listening(&tx);
        }
        Err(error) => println!("error: {}", error),
    }    
}
```

