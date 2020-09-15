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
        if let (Some(key), Some(val)) = (args.get(1), args.get(2)) {
            self.si.kvs.insert(key.to_string(), val.to_string());
        }
    }
}
