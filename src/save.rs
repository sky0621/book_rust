use std::collections::HashMap;

use crate::command::Command;
use crate::store_info::{re_write_store, read_store, StoreInfo};

pub const SAVE: &str = "save";

pub struct Save {}

impl Command for Save {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 3 {
            return;
        }
        let mut kvs: HashMap<String, String> = HashMap::new();
        // 今回分をKVSに格納
        kvs.insert(
            args.get(1).unwrap().to_string(),
            args.get(2).unwrap().to_string(),
        );

        // JSONファイルから既存分を取得
        let previous = read_store();
        if !previous.is_empty() {
            // 既存分を今回分に続けてKVSに格納
            let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
            for (k, v) in saved_si.kvs {
                kvs.insert(k, v);
            }
        }

        // JSONファイル書き込み用に文字列化
        let serialized = serde_json::to_string(&StoreInfo { kvs }).unwrap();

        // JSONファイルに書き込み
        re_write_store(serialized);
    }
}
