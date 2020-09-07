pub trait Command {
    fn exec(&self, args: Vec<&str>);
}
