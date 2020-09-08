use crate::command::Command;
use crate::store_info::{re_write_store, read_store, StoreInfo};

pub const REMOVE: &str = "remove";

pub struct Remove {}

impl Command for Remove {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }
        // JSONファイルから既存分を取得
        let previous = read_store();
        if previous.is_empty() {
            return;
        }
        let mut saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        saved_si.kvs.remove(&*args.get(1).unwrap().to_string());

        // JSONファイル書き込み用に文字列化
        let serialized = serde_json::to_string(&saved_si).unwrap();

        // JSONファイルに書き込み
        re_write_store(serialized);
    }
}
