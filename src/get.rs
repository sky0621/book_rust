use crate::command::Command;
use crate::store_info::read_store_info;

pub const GET: &str = "get";

pub struct Get {}

impl Command for Get {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }

        // JSONファイルから既存分を取得
        let saved_si = read_store_info();

        if let Some(v) = saved_si.kvs.get(*args.get(1).unwrap()) {
            println!("{}", v);
        }
    }
}
