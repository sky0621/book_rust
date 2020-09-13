use crate::command::Command;
use crate::store_info::StoreInfo;

pub struct List<'a> {
    si: &'a mut StoreInfo,
}

impl<'a> List<'a> {
    pub fn new(si: &'a mut StoreInfo) -> List<'a> {
        List { si }
    }
}

impl<'a> Command for List<'a> {
    fn exec(&mut self, _: Vec<&str>) {
        for (k, v) in &self.si.kvs {
            println!("{{ {}, {} }}", k, v);
        }
    }
}
