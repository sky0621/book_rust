use std::collections::HashMap;

use crate::command::Command;
use crate::store_info::{get_serialized, re_write_store, read_store_info, StoreInfo};

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

        // 既存分を今回分に続けてKVSに格納
        for (k, v) in read_store_info().kvs {
            kvs.insert(k, v);
        }

        // JSONファイルに書き込み
        re_write_store(get_serialized(&StoreInfo { kvs }));
    }
}
