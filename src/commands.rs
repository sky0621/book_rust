use std::collections::HashMap;

use crate::clear::Clear;
use crate::command::Command;
use crate::end::End;
use crate::get::Get;
use crate::help::Help;
use crate::list::List;
use crate::remove::Remove;
use crate::save::Save;

pub struct Commands {
    pub commands: HashMap<String, Box<Command>>,
}
impl Commands {
    pub fn new() -> Commands {
        let mut commands: HashMap<String, Box<Command>> = HashMap::new();
        commands.insert(String::from("end"), Box::new(End::new()));
        commands.insert(String::from("help"), Box::new(Help::new()));
        commands.insert(String::from("clear"), Box::new(Clear::new()));
        commands.insert(String::from("save"), Box::new(Save::new()));
        commands.insert(String::from("get"), Box::new(Get::new()));
        commands.insert(String::from("remove"), Box::new(Remove::new()));
        commands.insert(String::from("list"), Box::new(List::new()));
        Commands { commands }
    }
    pub fn exec(&self, args: &Vec<&str>) {
        if args.len() == 0 {
            self.commands.get("help").unwrap().exec(vec![]);
            return;
        }
        let cmd = &args.get(0).unwrap().to_string();
        if self.commands.contains_key(cmd) {
            self.commands.get(cmd).unwrap().exec(args.to_vec());
            return;
        }
        self.commands.get("help").unwrap().exec(vec![]);
    }
}
