use crate::clear::Clear;
use crate::end::End;
use crate::get::Get;
use crate::help::Help;
use crate::list::List;
use crate::remove::Remove;
use crate::save::Save;
use crate::store_info::StoreInfo;

pub trait Command {
    fn exec(&mut self, args: Vec<&str>);
}

pub fn exec(args: Vec<&str>, si: &mut StoreInfo) -> bool {
    if let Some(command_str) = args.get(0) {
        let mut command: Box<dyn Command> = match *command_str {
            "end" => Box::new(End {}),
            "clear" => Box::new(Clear::new(si)),
            "save" => Box::new(Save::new(si)),
            "get" => Box::new(Get::new(si)),
            "remove" => Box::new(Remove::new(si)),
            "list" => Box::new(List::new(si)),
            _ => Box::new(Help {}),
        };
        command.exec(args.clone());
        if *command_str == "end" {
            return false;
        }
    }
    true
}
