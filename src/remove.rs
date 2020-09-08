use crate::command::Command;

pub struct Remove {}
impl Remove {
    pub fn new() -> Remove {
        Remove {}
    }
}
impl Command for Remove {
    fn exec(&self, _: Vec<&str>) {
        println!("Remove");
    }
}
