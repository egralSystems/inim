use std::prelude::v1::*;

use rhai::Engine;

pub fn register_file<F: File>(engine: &mut Engine) {
    engine
        .register_type_with_name::<F>("File")
        .register_fn("open", F::open)
        .register_fn("seek", F::seek)
        .register_fn("step", F::step)
        .register_fn("read_blob", F::read_blob_all)
        .register_fn("read_blob", F::read_blob_amount)
        .register_fn("read_string", F::read_string_all)
        .register_fn("read_string", F::read_string_amount)
        .register_fn("read_string", F::read_string_until)
        .register_fn("write", F::write_blob)
        .register_fn("write", F::write_string)
        .register_fn("close", F::close);
}

pub trait File: Clone + 'static {
    fn open(path: &str, options: &str) -> Self;
    fn close(&mut self);

    fn seek(&mut self, offset: usize);
    fn step(&mut self, step: i64);

    fn read_blob_all(&mut self) -> Vec<u8>;
    fn read_blob_amount(&mut self, amount: i64) -> Vec<u8>;

    fn read_string_all(&mut self) -> String;
    fn read_char(&mut self) -> char;

    fn read_string_amount(&mut self, amount: usize) -> String {
        let mut output = String::new();

        for _ in 0..amount {
            output.push(self.read_char());
        }

        output
    }

    fn read_string_until(&mut self, stop: char) -> String {
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

    fn write_string(&mut self, text: &str);
    fn write_blob(&mut self, blob: Vec<u8>);
}
