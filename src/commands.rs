use crate::clear::Clear;
use crate::command::Command;
use crate::end::End;
use crate::get::Get;
use crate::help::Help;
use crate::list::List;
use crate::remove::Remove;
use crate::save::Save;
use crate::store_info::StoreInfo;

// 各コマンド生成時に重い処理をしているわけではないので、
// いったん構造体にコマンド群を保持するのは止めて、標準入力を受け取るたびに
// 都度、実行コマンド判定、及びコマンド実行させる。
pub fn run(args: Vec<&str>, si: &mut StoreInfo) {
    // 標準入力から何かしらの指示があれば発動
    if let Some(order) = args.get(0) {
        // 指示に応じたコマンドのピックアップ
        let mut c: Box<dyn Command> = match *order {
            "end" => Box::new(End::new(si)),
            "clear" => Box::new(Clear::new(si)),
            "save" => Box::new(Save::new(si)),
            "get" => Box::new(Get::new(si)),
            "remove" => Box::new(Remove::new(si)),
            "list" => Box::new(List::new(si)),
            // 規定外の指示にはヘルプ表示
            _ => Box::new(Help {}), // ヘルプ表示にはキーバリューストアは不要
        };
        c.exec(args)
    }
}
