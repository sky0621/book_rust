use crate::command::Command;

pub struct Clear {}
impl Clear {
    pub fn new() -> Clear {
        Clear {}
    }
}
impl Command for Clear {
    fn exec(&self, _: Vec<&str>) {
        println!("Clear");
    }
}
