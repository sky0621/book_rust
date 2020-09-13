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
        if args.len() != 2 {
            return;
        }
        let key = args.get(1).unwrap_or(&&"").clone();
        if let Some(v) = self.si.kvs.get(key) {
            println!("{}", v);
        }
    }
}
