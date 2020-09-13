use std::io::stdin;

use crate::store_info::{re_write_store, StoreInfo};

mod clear;
mod command;
mod end;
mod get;
mod help;
mod list;
mod remove;
mod save;
mod store_info;

fn main() {
    let mut si = StoreInfo::new();
    loop {
        let mut input = String::new();

        // 標準入力から input へ
        stdin().read_line(&mut input).unwrap();

        input.retain(|c| c != '\n'); // 改行コードの除去

        // 半角スペースで分割
        let seps: Vec<&str> = input.split_ascii_whitespace().collect();

        if !command::exec(seps, &mut si) {
            break;
        }
    }
    re_write_store(&si);
}
