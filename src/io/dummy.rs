use std::prelude::v1::*;

use rhai::Dynamic;

use super::{console::Console, fs::File, net::Net, sys::Sys};

#[derive(Clone)]
pub struct Dummy;

impl Console for Dummy {
    fn print(_text: &str) {}
    fn debug(_text: &str) {}
}

impl File for Dummy {
    fn open(_path: &str, _options: &str) -> Self {
        Self
    }

    fn close(&mut self) {}

    fn seek(&mut self, _offset: usize) {}

    fn step(&mut self, _step: i64) {}

    fn read_char(&mut self) -> char {
        '\0'
    }

    fn read_blob_all(&mut self) -> Vec<u8> {
        vec![]
    }

    fn read_blob_amount(&mut self, amount: i64) -> Vec<u8> {
        vec![]
    }

    fn read_string_all(&mut self) -> String {
        "".to_string()
    }

    fn write_string(&mut self, text: &str) {}

    fn write_blob(&mut self, blob: Vec<u8>) {}
}

impl Sys for Dummy {
    fn ls(_path: &str) -> Vec<Dynamic> {
        vec![]
    }

    fn mkdir(_path: &str) -> bool {
        false
    }

    fn rm(_path: &str) -> bool {
        false
    }

    fn rmdir(_path: &str) -> bool {
        false
    }

    fn time() -> f64 {
        0.0
    }

    fn path() -> String {
        "".to_string()
    }
}

impl Net for Dummy {
    fn tcp() -> Self {
        Self
    }

    fn udp() -> Self {
        Self
    }

    fn bind(&mut self, addr: &str, port: u16) -> String {
        "Not implemented".into()
    }

    fn connect(&mut self, addr: &str, port: u16) -> String {
        "Not implemented".into()
    }

    fn set_timeout(&mut self, timeout: i64) {}

    fn accept(&mut self) -> Self {
        Self
    }

    fn send_string(&mut self, msg: &str) -> String {
        "Not implemented".into()
    }

    fn recv_string(&mut self, char_count: i64) -> String {
        "Not implemented".into()
    }

    fn recv_line(&mut self) -> String {
        "Not implemented".into()
    }

    fn send_blob(&mut self, msg: Vec<u8>) -> String {
        "Not implemented".into()
    }

    fn recv_blob(&mut self, byte_count: i64) -> Vec<u8> {
        vec![]
    }

    fn close(&mut self) {}
}
