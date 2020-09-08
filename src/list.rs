use crate::command::Command;
use crate::store_info::{read_store, StoreInfo};

pub const LIST: &str = "list";

pub struct List {}

impl Command for List {
    fn exec(&self, _: Vec<&str>) {
        // JSONファイルから既存分を取得
        let previous = read_store();
        if previous.is_empty() {
            return;
        }
        let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        for (k, v) in saved_si.kvs {
            println!("{{ {}, {} }}", k, v);
        }
    }
}
