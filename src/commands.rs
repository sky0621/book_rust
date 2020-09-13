use crate::clear::Clear;
use crate::command::Command;
use crate::end::End;
use crate::get::Get;
use crate::help::Help;
use crate::list::List;
use crate::no_ope::NoOpe;
use crate::remove::Remove;
use crate::save::Save;

pub fn exec(args: Vec<&str>) {
    if let Some(command_str) = args.get(0) {
        let command: Box<dyn Command> = match *command_str {
            "end" => Box::new(End {}),
            "help" => Box::new(Help {}),
            "clear" => Box::new(Clear {}),
            "save" => Box::new(Save::new(args)),
            "get" => Box::new(Get::new(args)),
            "remove" => Box::new(Remove::new(args)),
            "list" => Box::new(List {}),
            _ => Box::new(NoOpe {}),
        };
        command.exec();
    }
}
