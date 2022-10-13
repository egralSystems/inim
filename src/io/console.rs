pub trait Console: Clone + 'static {
    fn print(text: &str);
    fn debug(text: &str);
}

#[derive(Clone)]
pub struct DummyConsole;

impl Console for DummyConsole {
    fn print(_text: &str) {}
    fn debug(_text: &str) {}
}
