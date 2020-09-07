use crate::command::Command;

pub struct Save {}
impl Save {
    pub fn new() -> Save {
        Save {}
    }
}
impl Command for Save {
    fn exec(&self, args: Vec<&str>) {
        println!("Save!");
        println!("{:?}", args);
    }
}
