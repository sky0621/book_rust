use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, process};

pub const STORE_FILE: &str = "store.json";

// アトリビュートにてJSONパーサーのシリアライズ機能を持たせる
#[derive(Serialize, Deserialize, Debug)]
pub struct StoreInfo {
    pub kvs: HashMap<String, String>,
}

pub fn re_write_store(json_str: String) {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(STORE_FILE)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(-1);
        }
    };
    file.write(json_str.as_bytes()).unwrap();
}

pub fn read_store() -> String {
    fs::read_to_string(STORE_FILE).unwrap()
}

pub fn read_store_info() -> StoreInfo {
    let previous = read_store();
    if previous.is_empty() {
        return StoreInfo {
            kvs: HashMap::new(),
        };
    }
    serde_json::from_str(&previous).unwrap()
}

pub fn get_serialized(si: &StoreInfo) -> String {
    serde_json::to_string(si).unwrap()
}
