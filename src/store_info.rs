use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;
use std::{fs, process};

use serde::{Deserialize, Serialize};

const STORE_FILE: &str = "store.json";

// アトリビュートにてJSONパーサーのシリアライズ機能を持たせる
#[derive(Serialize, Deserialize, Debug)]
pub struct StoreInfo {
    pub kvs: HashMap<String, String>,
}

pub fn init_store() {
    if !Path::new(STORE_FILE).exists() {
        File::create(STORE_FILE).unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            process::exit(-1);
        });
    }
}

pub fn re_write_store(json_str: String) -> Result<usize, Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(STORE_FILE)?;
    Ok(file.write(json_str.as_bytes())?)
}

pub fn read_store_info() -> StoreInfo {
    let previous = fs::read_to_string(STORE_FILE).unwrap_or_else(|e| {
        eprintln!("{:#?}", e);
        process::exit(-1);
    });
    if previous.is_empty() {
        return StoreInfo {
            kvs: HashMap::new(),
        };
    }
    serde_json::from_str(&previous).unwrap_or_else(|e| {
        eprintln!("{:#?}", e);
        process::exit(-1);
    })
}

pub fn get_serialized(si: &StoreInfo) -> String {
    serde_json::to_string(si).unwrap_or_else(|e| {
        eprintln!("{:#?}", e);
        process::exit(-1);
    })
}
