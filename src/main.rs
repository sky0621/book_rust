mod store_info;

use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};
use std::{fs, process};

use crate::store_info::{StoreInfo, KV};

fn main() {
    loop {
        let mut input = String::new();

        // 標準入力から input へ
        stdin().read_line(&mut input).unwrap();

        input.retain(|c| c != '\n'); // 改行コードの除去

        // 半角スペースで分割
        let seps: Vec<&str> = input.split_ascii_whitespace().collect();
        if seps.len() == 0 {
            usage();
            continue;
        }

        match seps[0] {
            // アプリ終了判定
            "end" => {
                println!("End");
                process::exit(0);
            }

            // ヘルプ
            "help" => usage(),

            // 保存
            "save" if seps.len() == 3 => {
                let mut kvs: Vec<KV> = vec![];
                // 今回分をベクターに格納
                kvs.push(KV::new(
                    seps.get(1).unwrap().to_string(),
                    seps.get(2).unwrap().to_string(),
                ));

                // JSONファイルから既存分を取得
                let previous = fs::read_to_string("store.json").unwrap();
                if !previous.is_empty() {
                    // 既存分を今回分に続けてベクターに格納
                    let mut saved_si: StoreInfo = serde_json::from_str(&previous).unwrap();
                    kvs.append(saved_si.kvs.as_mut());
                }

                // JSONファイル書き込み用に文字列化
                let serialized = serde_json::to_string(&StoreInfo::new(kvs)).unwrap();

                // JSONファイルに書き込み
                let mut store = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open("store.json")
                    .unwrap();
                store.write(serialized.as_bytes());
            }
            // "save" if seps.len() == 3 => cmd_store.insert(seps[1], seps[2]),
            // // 取得
            // "get" if seps.len() == 2 => cmd_store.get(seps[1]),
            // // 削除
            // "remove" if seps.len() == 2 => cmd_store.remove(seps[1]),
            // その他
            _ => usage(),
        }
    }
}

fn usage() {
    let msg = r#"

[usage]
キーバリュー形式で文字列情報を管理するコマンドです。
以下のサブコマンドが利用可能です。

list   ... 保存済みの内容を一覧表示します。
save   ... keyとvalueを渡して保存します。
get    ... keyを渡してvalueを表示します。
remove ... keyを渡してvalueを削除します。
help   ... ヘルプ情報（当内容と同じ）を表示します。

    "#;
    println!("{}", msg);
}
