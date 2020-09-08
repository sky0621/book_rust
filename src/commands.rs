use std::collections::HashMap;

use crate::clear::{Clear, CLEAR};
use crate::command::Command;
use crate::end::{End, END};
use crate::get::{Get, GET};
use crate::help::{Help, HELP};
use crate::list::{List, LIST};
use crate::remove::{Remove, REMOVE};
use crate::save::{Save, SAVE};

pub struct Commands {
    pub commands: HashMap<&'static str, Box<dyn Command>>,
}
impl Commands {
    pub fn new() -> Commands {
        let mut commands: HashMap<&'static str, Box<dyn Command>> = HashMap::new();
        commands.insert(END, Box::new(End::new()));
        commands.insert(HELP, Box::new(Help::new()));
        commands.insert(CLEAR, Box::new(Clear::new()));
        commands.insert(SAVE, Box::new(Save::new()));
        commands.insert(GET, Box::new(Get::new()));
        commands.insert(REMOVE, Box::new(Remove::new()));
        commands.insert(LIST, Box::new(List::new()));
        Commands { commands }
    }
    pub fn exec(&self, args: &Vec<&str>) {
        if args.len() == 0 {
            self.commands.get(HELP).unwrap().exec(vec![]);
            return;
        }
        let cmd = args.get(0).unwrap();
        if self.commands.contains_key(cmd) {
            self.commands.get(cmd).unwrap().exec(args.to_vec());
            return;
        }
        self.commands.get(HELP).unwrap().exec(vec![]);
    }
}
