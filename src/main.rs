use std::io::stdin;

use crate::commands::run;
use crate::store_info::StoreInfo;

mod clear;
mod command;
mod commands;
mod end;
mod get;
mod help;
mod list;
mod remove;
mod save;
mod store_info;

fn main() {
    // オンメモリ用のキーバリューストア
    // 前回アプリ起動時の読み書き情報をJSONファイルから読み込む。
    let mut si = StoreInfo::new();

    loop {
        // 標準入力の受け皿
        let mut input = String::new();

        // 標準入力から input へ
        stdin().read_line(&mut input).unwrap();

        input.retain(|c| c != '\n'); // 改行コードの除去

        // 半角スペースで分割
        let seps: Vec<&str> = input.split_ascii_whitespace().collect();

        run(seps, &mut si);
    }
}
