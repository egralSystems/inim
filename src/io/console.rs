pub trait Console: Clone + 'static {
    fn print(text: &str);
    fn debug(text: &str);
}
