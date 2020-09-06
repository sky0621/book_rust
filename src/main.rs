use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdin, Write};
use std::process;

#[derive(Serialize, Deserialize, Debug)]
struct StoreInfo {
    kvs: Vec<KV>,
}

#[derive(Serialize, Deserialize, Debug)]
struct KV {
    key: String,
    val: String,
}

fn main() {
    let mut store = File::create("store.json").expect("file not found");
    loop {
        let mut input = String::new();

        // 標準入力から input へ
        stdin().read_line(&mut input).unwrap();

        input.retain(|c| c != '\n'); // 改行コードの除去

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
                let si = StoreInfo {
                    kvs: vec![KV {
                        key: seps.get(1).unwrap().to_string(),
                        val: seps.get(2).unwrap().to_string(),
                    }],
                };
                let serialized = serde_json::to_string(&si).unwrap();
                store.write_all(serialized.as_bytes());
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
