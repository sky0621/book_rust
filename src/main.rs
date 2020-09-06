use std::collections::HashMap;

fn main() {
    let mut cmdStore = HashMap::new();
    cmdStore.insert("k1","v1");
    println!("{:?}", cmdStore);

    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        input.retain(|c| c != '\n');

        match input.as_str() {
            "help" => usage(),
            _ => println!(),
        }
        input.clear();
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
    println!("{}",msg);
}
