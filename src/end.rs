use crate::command::Command;
use std::process;

pub struct End {}
impl End {
    pub fn new() -> End {
        End {}
    }
}
impl Command for End {
    fn exec(&self, _: Vec<&str>) {
        println!("End");
        process::exit(0);
    }
}
