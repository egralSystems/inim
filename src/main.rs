use std::{cell::RefCell, fs::File, io::Read, os::unix::prelude::FileExt, path::Path, rc::Rc};

use inim::{
    io::{console::Console, fs},
    Inim,
};

fn main() {
    let mut inim = Inim::<LinuxConsole, LinuxFile>::new();

    inim.run_file("test.rhai");
}

struct LinuxConsole;

impl Console for LinuxConsole {
    fn print(text: &str) {
        println!("{}", text);
    }

    fn debug(text: &str, _source: Option<&str>, _position: rhai::Position) {
        println!("Debug: {}", text);
    }
}

#[derive(Clone)]
struct LinuxFile {
    file: Rc<RefCell<File>>,
    offset: u64,
}

impl fs::File for LinuxFile {
    fn open(path: &str, options: &str) -> Self {
        let path = Path::new(path);

        let readable = options.contains('r');
        let writable = options.contains('w');
        let appendable = options.contains('a');

        let file = File::options()
            .read(readable)
            .write(writable)
            .append(appendable)
            .create(writable || appendable)
            .open(path)
            .unwrap();

        LinuxFile {
            file: Rc::new(RefCell::new(file)),
            offset: 0,
        }
    }

    fn close(&mut self) {
        drop(&self.file);
    }

    fn read_all(&mut self) -> String {
        let mut buf = String::new();
        self.file.borrow_mut().read_to_string(&mut buf).unwrap();

        buf
    }

    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        self.file
            .borrow_mut()
            .read_at(&mut buf, self.offset)
            .unwrap();
        self.offset += 1;

        buf[0] as char
    }

    fn rewind(&mut self) {
        self.offset = 0;
    }

    fn seek(&mut self, offset: usize) {
        self.offset = offset as u64;
    }

    fn step(&mut self, step: i64) {
        if step.is_negative() {
            self.offset -= step.wrapping_abs() as u64;
        } else {
            self.offset += step as u64;
        }
    }

    fn write(&mut self, text: &str) {
        self.file
            .borrow_mut()
            .write_all_at(text.as_bytes(), self.offset)
            .unwrap();
    }
}
