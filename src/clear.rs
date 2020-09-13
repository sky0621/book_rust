use std::process;

use crate::command::Command;
use crate::store_info::re_write_store;

pub struct Clear {}

impl Command for Clear {
    fn exec(&self) {
        re_write_store(String::from("")).unwrap_or_else(|e| {
            eprintln!("{:#?}", e);
            process::exit(-1);
        });
    }
}
