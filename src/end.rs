use std::process;

use crate::command::Command;
use crate::store_info::{write_store, StoreInfo};

pub struct End<'a> {
    si: &'a StoreInfo,
}

impl<'a> End<'a> {
    pub fn new(si: &'a StoreInfo) -> End<'a> {
        End { si }
    }
}

impl<'a> Command for End<'a> {
    fn exec(&mut self, _: Vec<&str>) {
        // キーバリュー読み書き結果をJSONファイルに保存
        write_store(&self.si);

        // アプリ終了
        process::exit(0);
    }
}
