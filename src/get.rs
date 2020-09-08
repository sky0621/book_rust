use crate::command::Command;
use crate::store_info::{read_store, StoreInfo};

pub const GET: &str = "get";

pub struct Get {}

impl Command for Get {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }
        // JSONファイルから既存分を取得
        let previous = read_store();
        if previous.is_empty() {
            return;
        }
        let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        let v = saved_si.kvs.get(&*args.get(1).unwrap().to_string());
        println!("{}", v.unwrap_or(&mut String::from("")));
    }
}
