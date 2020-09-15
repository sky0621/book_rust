// 各種コマンドの処理を統一的に扱うI/Oを規定
pub trait Command {
    // args ... 標準入力から受け取ったコマンド及びパラメータ群
    fn exec(&mut self, args: Vec<&str>);
}
