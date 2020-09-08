use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

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
