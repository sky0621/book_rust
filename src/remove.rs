use std::process;

use crate::command::Command;
use crate::store_info::{get_serialized, re_write_store, read_store_info};

pub struct Remove {
    key: String,
}

impl Remove {
    pub fn new(args: Vec<&str>) -> Remove {
        if args.len() != 2 {
            return Remove {
                key: "".to_string(),
            };
        }
        Remove {
            key: args.get(1).unwrap().to_string(),
        }
    }
}

impl Command for Remove {
    fn exec(&self) {
        // JSONファイルから既存分を取得
        let mut saved_si = read_store_info();

        // 既存分から指定された要素を削除
        saved_si.kvs.remove(&self.key);

        // JSONファイルに書き込み
        re_write_store(get_serialized(&saved_si)).unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            process::exit(-1);
        });
    }
}
