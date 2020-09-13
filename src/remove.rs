use crate::command::Command;
use crate::store_info::StoreInfo;

pub struct Remove<'a> {
    si: &'a mut StoreInfo,
}

impl<'a> Remove<'a> {
    pub fn new(si: &'a mut StoreInfo) -> Remove<'a> {
        Remove { si }
    }
}

impl<'a> Command for Remove<'a> {
    fn exec(&mut self, args: Vec<&str>) {
        if args.len() != 2 {
            return;
        }
        let key = args.get(1).unwrap_or(&&"").clone();
        self.si.kvs.remove(key);
    }
}
