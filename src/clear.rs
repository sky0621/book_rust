use crate::command::Command;
use crate::store_info::re_write_store;

pub const CLEAR: &str = "clear";

pub struct Clear {}

impl Command for Clear {
    fn exec(&self, _: Vec<&str>) {
        re_write_store(String::from(""));
    }
}
