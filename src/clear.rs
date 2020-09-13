use std::collections::HashMap;

use crate::command::Command;
use crate::store_info::StoreInfo;

pub struct Clear<'a> {
    si: &'a mut StoreInfo,
}

impl<'a> Clear<'a> {
    pub fn new(si: &'a mut StoreInfo) -> Clear<'a> {
        Clear { si }
    }
}

impl<'a> Command for Clear<'a> {
    fn exec(&mut self, _: Vec<&str>) {
        self.si.kvs = HashMap::new();
    }
}
