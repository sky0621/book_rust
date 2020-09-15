use crate::command::Command;
use crate::store_info::StoreInfo;

pub struct Get<'a> {
    si: &'a mut StoreInfo,
}

impl<'a> Get<'a> {
    pub fn new(si: &'a mut StoreInfo) -> Get<'a> {
        Get { si }
    }
}

impl<'a> Command for Get<'a> {
    fn exec(&mut self, args: Vec<&str>) {
        if let Some(key) = args.get(1) {
            // clone() しないと以下のようにRustコンパイラに怒られるが、理屈がわかっていない。
            // 頻繁に出会うコンパイルエラーの内容はそのうちまとめてみよう。
            // 「[E0277]: the trait bound `std::string::String: std::borrow::Borrow<&str>` is not satisfied」
            if let Some(v) = self.si.kvs.get(key.clone()) {
                println!("{}", v);
            }
        }
    }
}
