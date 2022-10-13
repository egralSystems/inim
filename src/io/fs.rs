use std::prelude::v1::*;

pub trait File: Clone + 'static {
    fn open(path: &str, options: &str) -> Self;
    fn close(&mut self);

    fn seek(&mut self, offset: usize);
    fn step(&mut self, step: i64);

    fn read_all(&mut self) -> String;
    fn read_char(&mut self) -> char;

    fn read_amount(&mut self, amount: usize) -> String {
        let mut output = String::new();

        for _ in 0..amount {
            output.push(self.read_char());
        }

        output
    }

    fn read_until(&mut self, stop: char) -> String {
        let mut output = String::new();

        loop {
            let ch = self.read_char();

            if ch == stop {
                break;
            }

            output.push(ch);
        }

        output
    }

    fn write(&mut self, text: &str);
}

#[derive(Clone)]
pub struct DummyFile;

impl File for DummyFile {
    fn open(_path: &str, _options: &str) -> Self {
        Self
    }

    fn close(&mut self) {}

    fn seek(&mut self, _offset: usize) {}

    fn step(&mut self, _step: i64) {}

    fn read_all(&mut self) -> String {
        "".to_string()
    }

    fn read_char(&mut self) -> char {
        '\0'
    }

    fn write(&mut self, _text: &str) {}
}
