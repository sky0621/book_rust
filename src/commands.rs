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
        commands.insert(END, Box::new(End {}));
        commands.insert(HELP, Box::new(Help {}));
        commands.insert(CLEAR, Box::new(Clear {}));
        commands.insert(SAVE, Box::new(Save {}));
        commands.insert(GET, Box::new(Get {}));
        commands.insert(REMOVE, Box::new(Remove {}));
        commands.insert(LIST, Box::new(List {}));
        Commands { commands }
    }

    pub fn exec(&self, args: Vec<&str>) {
        if let Some(order) = args.get(0) {
            if let Some(cmd) = self.commands.get(order) {
                cmd.exec(args)
            }
        }
    }
}
