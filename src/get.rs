use crate::command::Command;
use crate::store_info::{StoreInfo, STORE_FILE};
use std::fs;

pub const GET: &str = "get";

pub struct Get {}
impl Get {
    pub fn new() -> Get {
        Get {}
    }
}
impl Command for Get {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }
        // JSONファイルから既存分を取得
        let previous = fs::read_to_string(STORE_FILE).unwrap();
        if previous.is_empty() {
            return;
        }
        let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        let v = saved_si.kvs.get(&*args.get(1).unwrap().to_string());
        println!("{}", v.unwrap_or(&mut String::from("")));
    }
}
