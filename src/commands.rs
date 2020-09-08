use crate::command::Command;
use crate::save::Save;
use std::collections::HashMap;

pub struct Commands {
    pub commands: HashMap<String, Box<Command>>,
}
impl Commands {
    pub fn new() -> Commands {
        let mut commands: HashMap<String, Box<Command>> = HashMap::new();
        commands.insert(String::from("save"), Box::new(Save::new()));
        Commands { commands }
    }
    pub fn exec(&self, seps: Vec<&str>) {
        println!("Commands.exec");
    }
}
