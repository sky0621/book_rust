use crate::command::Command;
use crate::store_info::StoreInfo;

pub struct Save<'a> {
    si: &'a mut StoreInfo,
}

impl<'a> Save<'a> {
    pub fn new(si: &'a mut StoreInfo) -> Save<'a> {
        Save { si }
    }
}

impl<'a> Command for Save<'a> {
    fn exec(&mut self, args: Vec<&str>) {
        if args.len() != 3 {
            return;
        }
        let key = args.get(1).unwrap_or(&&"").to_string();
        let val = args.get(2).unwrap_or(&&"").to_string();
        self.si.kvs.insert(key, val);
    }
}
