use crate::command::Command;

pub struct Get {}
impl Get {
    pub fn new() -> Get {
        Get {}
    }
}
impl Command for Get {
    fn exec(&self, _: Vec<&str>) {
        println!("Get");
    }
}
