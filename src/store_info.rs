use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::{fs, process};

use serde::{Deserialize, Serialize};

// アプリ起動・終了時にオンメモリのキーバリュー情報を読み書きするためのJSONファイル
const STORE_FILE: &str = "store.json";

// アトリビュートにてJSONパーサーのシリアライズ機能を持たせる
#[derive(Serialize, Deserialize, Debug)]
pub struct StoreInfo {
    pub kvs: HashMap<String, String>,
}

impl StoreInfo {
    pub fn new() -> StoreInfo {
        // 未作成ならJSONファイルを作っておく。
        if !Path::new(STORE_FILE).exists() {
            File::create(STORE_FILE).unwrap_or_else(|e| {
                eprintln!("{:#?}", e);
                process::exit(-1);
            });
        }

        // 前回アプリ起動時のキーバリュー情報をオンメモリ保持
        let previous = fs::read_to_string(STORE_FILE).unwrap();
        if previous.is_empty() {
            return StoreInfo {
                kvs: HashMap::new(),
            };
        }
        serde_json::from_str(&previous).unwrap()
    }
}

pub fn re_write_store(si: &StoreInfo) {
    let json_str = serde_json::to_string(si).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(STORE_FILE)
        .unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            process::exit(-1);
        });
    file.write(json_str.as_bytes()).unwrap_or_else(|e| {
        eprintln!("{:#?}", e);
        process::exit(-1);
    });
}
