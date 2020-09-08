use crate::command::Command;
use crate::store_info::STORE_FILE;
use std::fs::OpenOptions;
use std::io::Write;

pub const CLEAR: &str = "clear";

pub struct Clear {}
impl Clear {
    pub fn new() -> Clear {
        Clear {}
    }
}
impl Command for Clear {
    fn exec(&self, _: Vec<&str>) {
        // JSONファイルに書き込み
        let mut store = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(STORE_FILE)
            .unwrap();
        store.write("".as_bytes()).unwrap();
    }
}
