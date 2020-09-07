use crate::command::Command;
use crate::save::Save;
use std::collections::HashMap;

pub struct Commands<C: Command> {
    pub commands: HashMap<String, Box<C>>,
}
impl<C> Commands<C>
where
    C: Command,
{
    pub fn new() -> Commands<C> {
        let mut commands = HashMap::new();
        commands.insert(String::from("save"), Box::new(Save::new()));
        Commands { commands }
    }
    pub fn exec(&self, seps: Vec<&str>) {
        println!("Commands.exec");
    }
}
