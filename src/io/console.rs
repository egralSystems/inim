pub trait Console: Clone {
    fn print(text: &str);
    fn debug(text: &str);
}
