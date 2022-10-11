use alloc::string::String;

pub trait File {
    fn open(path: &str) -> Self;
    fn close(&mut self);
    fn read_all(&mut self) -> String;
}
