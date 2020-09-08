use crate::command::Command;
use std::process;

pub const END: &str = "end";

pub struct End {}

impl Command for End {
    fn exec(&self, _: Vec<&str>) {
        println!("End");
        process::exit(0);
    }
}
