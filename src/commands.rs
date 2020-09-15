use crate::clear::Clear;
use crate::command::Command;
use crate::end::End;
use crate::get::Get;
use crate::help::Help;
use crate::list::List;
use crate::remove::Remove;
use crate::save::Save;
use crate::store_info::StoreInfo;

pub fn run(args: Vec<&str>, si: &mut StoreInfo) {
    if let Some(order) = args.get(0) {
        let mut c: Box<dyn Command> = match *order {
            "end" => Box::new(End::new(si)),
            "clear" => Box::new(Clear::new(si)),
            "save" => Box::new(Save::new(si)),
            "get" => Box::new(Get::new(si)),
            "remove" => Box::new(Remove::new(si)),
            "list" => Box::new(List::new(si)),
            _ => Box::new(Help {}),
        };
        c.exec(args)
    }
}
