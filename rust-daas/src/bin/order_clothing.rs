extern crate daas;
extern crate kafka;

use std::io;
use std::thread;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaaSProcessor, DaaSProcessorMessage, DaaSProcessorService};
use std::sync::mpsc::{channel};
use serde_json::value::Value;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;

static WORKSPACE_LOCAL_STORAGE: &str = "./workshop_storage";

fn create_local_storage() {
    match fs::create_dir_all(WORKSPACE_LOCAL_STORAGE) {
        Ok(_) => {},
        Err(err) => {
            panic!("Warning: Could not create the local directory path {} : {:?}", WORKSPACE_LOCAL_STORAGE, err);
        },
    }
}

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

fn save_file(product_name: String, content: String) -> std::io::Result<()>{
    let mut file = File::create(format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name))?;
    file.write_all(content.as_bytes())
}


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
                            
    // Create the local storage directory for the aggregated data
    create_local_storage();

    // start the processor
    let _handler = thread::spawn(move || {
        DaaSProcessor::start_listening(consumer, &rx, None, |msg: DaaSProcessorMessage, _none_var , _t: Option<&i8>|{
            let mut doc = msg.doc;
            let order: Value = serde_json::from_str(&String::from_utf8(doc.data_obj_as_ref().to_vec()).unwrap()).unwrap();

            println!("Order Number {} from the {} has a status of {}...", doc.source_uid, doc.source_name, order.get("status").unwrap());
            
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
        
            match save_file(prd.clone().to_string(), content.to_string()) {
                Ok(_ok) => Ok(1),
                Err(err) => {
                    panic!("Warning: Could not save the clothing-{}.json file! : {:?}", prd, err);
                },
            }
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