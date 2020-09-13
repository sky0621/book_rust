use std::collections::HashMap;
use std::process;

use crate::command::Command;
use crate::store_info::{get_serialized, re_write_store, read_store_info, StoreInfo};

pub struct Save {
    key: String,
    val: String,
}

impl Save {
    pub fn new(args: Vec<&str>) -> Save {
        if args.len() != 3 {
            return Save {
                key: "".to_string(),
                val: "".to_string(),
            };
        }
        Save {
            key: args.get(1).unwrap().to_string(),
            val: args.get(2).unwrap().to_string(),
        }
    }
}

impl Command for Save {
    fn exec(&self) {
        let mut kvs: HashMap<String, String> = HashMap::new();
        // 今回分をKVSに格納
        kvs.insert((&self.key).clone(), (&self.val).clone());

        // 既存分を今回分に続けてKVSに格納
        for (k, v) in read_store_info().kvs {
            kvs.insert(k, v);
        }

        // JSONファイルに書き込み
        re_write_store(get_serialized(&StoreInfo { kvs })).unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            process::exit(-1);
        });
    }
}
