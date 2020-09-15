pub trait Command {
    fn exec(&mut self, args: Vec<&str>);
}
