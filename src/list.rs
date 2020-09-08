use crate::command::Command;

pub struct List {}
impl List {
    pub fn new() -> List {
        List {}
    }
}
impl Command for List {
    fn exec(&self, _: Vec<&str>) {
        println!("List");
    }
}
