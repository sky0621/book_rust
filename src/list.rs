use crate::command::Command;
use crate::store_info::{StoreInfo, STORE_FILE};
use std::fs;

pub const LIST: &str = "list";

pub struct List {}
impl List {
    pub fn new() -> List {
        List {}
    }
}
impl Command for List {
    fn exec(&self, _: Vec<&str>) {
        // JSONファイルから既存分を取得
        let previous = fs::read_to_string(STORE_FILE).unwrap();
        if previous.is_empty() {
            return;
        }
        let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        for (k, v) in saved_si.kvs {
            println!("{{ {}, {} }}", k, v);
        }
    }
}
