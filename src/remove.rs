use crate::command::Command;
use crate::store_info::{StoreInfo, STORE_FILE};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub const REMOVE: &str = "remove";

pub struct Remove {}
impl Remove {
    pub fn new() -> Remove {
        Remove {}
    }
}
impl Command for Remove {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }
        // JSONファイルから既存分を取得
        let previous = fs::read_to_string(STORE_FILE).unwrap();
        if previous.is_empty() {
            return;
        }
        let mut saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        saved_si.kvs.remove(&*args.get(1).unwrap().to_string());

        // JSONファイル書き込み用に文字列化
        let serialized = serde_json::to_string(&saved_si).unwrap();

        // JSONファイルに書き込み
        let mut store = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(STORE_FILE)
            .unwrap();
        store.write(serialized.as_bytes()).unwrap();
    }
}
