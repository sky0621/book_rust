use crate::command::Command;

pub struct End {}

impl Command for End {
    fn exec(&mut self, _: Vec<&str>) {
        println!("End");
    }
}
