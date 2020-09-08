use crate::command::Command;
use crate::store_info::read_store_info;

pub const LIST: &str = "list";

pub struct List {}

impl Command for List {
    fn exec(&self, _: Vec<&str>) {
        // JSONファイルから既存分を取得
        let saved_si = read_store_info();

        for (k, v) in saved_si.kvs {
            println!("{{ {}, {} }}", k, v);
        }
    }
}
