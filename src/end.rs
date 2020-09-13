use std::process;

use crate::command::Command;

pub struct End {}

impl Command for End {
    fn exec(&self) {
        println!("End");
        process::exit(0);
    }
}
