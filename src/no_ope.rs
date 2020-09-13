use crate::command::Command;

pub struct NoOpe {}

impl Command for NoOpe {
    fn exec(&self) {
        unimplemented!()
    }
}
