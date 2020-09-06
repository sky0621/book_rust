use std::borrow::Borrow;
use std::collections::HashMap;
use std::process;

fn separate<'a>(input: &'a String) -> Vec<&'static str> {
    *input.split_ascii_whitespace().collect()
}

fn main() {
    let mut cmd_store: HashMap<&str, &str> = HashMap::new();
    loop {
        let mut input = String::new();

        // 標準入力から input へ
        std::io::stdin().read_line(&mut input).unwrap();

        input.retain(|c| c != '\n'); // 改行コードの除去

        let seps: Vec<&str> = separate(&input);
        println!("{:?}", seps);

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
                let opt_key = seps.get(1).unwrap().clone();
                println!("{:?}", opt_key);
                // println!("{:?}", opt_key.unwrap());
                &cmd_store.insert(opt_key, "test");
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
