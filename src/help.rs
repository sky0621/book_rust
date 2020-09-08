use crate::command::Command;

pub const HELP: &str = "help";

pub struct Help {}
impl Help {
    pub fn new() -> Help {
        Help {}
    }
}
impl Command for Help {
    fn exec(&self, _: Vec<&str>) {
        let msg = r#"

[usage]
キーバリュー形式で文字列情報を管理するコマンドです。
以下のサブコマンドが利用可能です。

save   ... keyとvalueを渡して保存します。
get    ... keyを渡してvalueを表示します。
remove ... keyを渡してvalueを削除します。
list   ... 保存済みの内容を一覧表示します。
clear  ... 保存内容を全て削除します。
help   ... ヘルプ情報（当内容と同じ）を表示します。
end    ... プログラムを終了します。

    "#;
        println!("{}", msg);
    }
}
