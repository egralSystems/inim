use core::marker::PhantomData;
use std::prelude::v1::*;

use rhai::plugin::*;

use crate::NativeResult;

pub struct FileModule<F: File + 'static> {
    _file_phantom: PhantomData<F>
}

impl<F: File + 'static> FileModule<F> {
    pub fn new() -> Module {
        let mut fmod = Module::new();

        fmod.set_id("file");

        fmod.set_custom_type::<F>("File");
        fmod.set_native_fn("open", F::open);
        fmod.set_native_fn("close", F::close);
        fmod.set_native_fn("seek", F::seek);
        fmod.set_native_fn("step", F::step);
        fmod.set_native_fn("read", F::read_all);
        fmod.set_native_fn("read", F::read_amount);
        fmod.set_native_fn("read", F::read_until);

        fmod.build_index();
        fmod
    }
}

pub trait File: Clone {
    fn open(path: &str, options: &str) -> NativeResult<Self>;
    fn close(&mut self) -> NativeResult<()>;

    fn seek(&mut self, offset: usize) -> NativeResult<()>;
    fn step(&mut self, step: i64) -> NativeResult<()>;

    fn read_all(&mut self) -> NativeResult<String>;
    fn read_char(&mut self) -> NativeResult<char>;

    fn read_amount(&mut self, amount: usize) -> NativeResult<String> {
        let mut output = String::new();

        for _ in 0..amount {
            output.push(self.read_char()?);
        }

        Ok(output)
    }

    fn read_until(&mut self, stop: char) -> NativeResult<String> {
        let mut output = String::new();

        loop {
            let ch = self.read_char()?;

            if ch == stop {
                break;
            }

            output.push(ch);
        }

        Ok(output)
    }

    fn write(&mut self, text: &str) -> NativeResult<()>;
}