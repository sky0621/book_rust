use crate::command::Command;
use crate::store_info::{get_serialized, re_write_store, read_store_info};

pub const REMOVE: &str = "remove";

pub struct Remove {}

impl Command for Remove {
    fn exec(&self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }

        // JSONファイルから既存分を取得
        let mut saved_si = read_store_info();

        // 既存分から指定された要素を削除
        saved_si.kvs.remove(&*args.get(1).unwrap().to_string());

        // JSONファイルに書き込み
        re_write_store(get_serialized(&saved_si));
    }
}
