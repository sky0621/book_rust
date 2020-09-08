use std::collections::HashMap;
// use std::fs::OpenOptions;
use std::io::{stdin, Write};
use std::{fs, process};

// use crate::command::Command;
use crate::commands::Commands;
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
    let commands: Commands = Commands::new();

    println!("Start!");
    loop {
        let mut input = String::new();

        // 標準入力から input へ
        stdin().read_line(&mut input).unwrap();

        input.retain(|c| c != '\n'); // 改行コードの除去

        // 半角スペースで分割
        let seps: Vec<&str> = input.split_ascii_whitespace().collect();

        commands.exec(&seps);

        // match seps[0] {
        //     // 保存
        //     "save" if seps.len() == 3 => {
        //         let mut kvs: HashMap<String, String> = HashMap::new();
        //         // 今回分をKVSに格納
        //         kvs.insert(
        //             seps.get(1).unwrap().to_string(),
        //             seps.get(2).unwrap().to_string(),
        //         );
        //
        //         // JSONファイルから既存分を取得
        //         let previous = fs::read_to_string("store.json").unwrap();
        //         if !previous.is_empty() {
        //             // 既存分を今回分に続けてKVSに格納
        //             let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        //             for (k, v) in saved_si.kvs {
        //                 kvs.insert(k, v);
        //             }
        //         }
        //
        //         // JSONファイル書き込み用に文字列化
        //         let serialized = serde_json::to_string(&StoreInfo::new(kvs)).unwrap();
        //
        //         // JSONファイルに書き込み
        //         let mut store = OpenOptions::new()
        //             .write(true)
        //             .truncate(true)
        //             .open("store.json")
        //             .unwrap();
        //         store.write(serialized.as_bytes()).unwrap();
        //     }
        //
        //     // １件取得
        //     "get" if seps.len() == 2 => {
        //         // JSONファイルから既存分を取得
        //         let previous = fs::read_to_string("store.json").unwrap();
        //         if previous.is_empty() {
        //             println!();
        //             continue;
        //         }
        //         let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        //         let v = saved_si.kvs.get(&*seps.get(1).unwrap().to_string());
        //         println!("{}", v.unwrap_or(&mut String::from("")));
        //     }
        //
        //     // 全件取得
        //     "list" => {
        //         // JSONファイルから既存分を取得
        //         let previous = fs::read_to_string("store.json").unwrap();
        //         if previous.is_empty() {
        //             println!();
        //             continue;
        //         }
        //         let saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        //         for (k, v) in saved_si.kvs {
        //             println!("{{ {}, {} }}", k, v);
        //         }
        //     }
        //
        //     // １件削除
        //     "remove" if seps.len() == 2 => {
        //         // JSONファイルから既存分を取得
        //         let previous = fs::read_to_string("store.json").unwrap();
        //         if previous.is_empty() {
        //             println!();
        //             continue;
        //         }
        //         let mut saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
        //         saved_si.kvs.remove(&*seps.get(1).unwrap().to_string());
        //
        //         // JSONファイル書き込み用に文字列化
        //         let serialized = serde_json::to_string(&saved_si).unwrap();
        //
        //         // JSONファイルに書き込み
        //         let mut store = OpenOptions::new()
        //             .write(true)
        //             .truncate(true)
        //             .open("store.json")
        //             .unwrap();
        //         store.write(serialized.as_bytes()).unwrap();
        //     }
        //
        //     // 全件削除
        //     "clear" => {
        //         // JSONファイルに書き込み
        //         let mut store = OpenOptions::new()
        //             .write(true)
        //             .truncate(true)
        //             .open("store.json")
        //             .unwrap();
        //         store.write("".as_bytes()).unwrap();
        //     }
        //
        //     // その他
        //     _ => usage(),
        // }
    }
}
