extern crate std as stdstd;

use stdstd::{
    cell::RefCell,
    env::current_dir,
    fs::{create_dir_all, read_dir, remove_dir, remove_file, File},
    os::unix::prelude::FileExt,
    path::Path,
    prelude::v1::*,
    println,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use rhai::Dynamic;

use super::{console::Console, fs, sys::Sys};

#[derive(Clone)]
struct StdSys;

impl Sys for StdSys {
    fn ls(path: &str) -> Vec<Dynamic> {
        let mut table: Vec<Dynamic> = Vec::new();

        for file in read_dir(path).unwrap() {
            table.push(Dynamic::from(file.unwrap().path().display().to_string()));
        }

        table
    }

    fn mkdir(path: &str) -> bool {
        create_dir_all(path).is_ok()
    }

    fn rm(path: &str) -> bool {
        remove_file(path).is_ok()
    }

    fn rmdir(path: &str) -> bool {
        remove_dir(path).is_ok()
    }

    fn time() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    fn path() -> String {
        current_dir().unwrap().display().to_string()
    }
}

#[derive(Clone)]
struct StdConsole;

impl Console for StdConsole {
    fn print(text: &str) {
        println!("{}", text);
    }

    fn debug(text: &str) {
        println!("{}", text);
    }
}

#[derive(Clone)]
struct StdFile {
    file: Rc<RefCell<File>>,
    offset: u64,
}

impl fs::File for StdFile {
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

        StdFile {
            file: Rc::new(RefCell::new(file)),
            offset: 0,
        }
    }

    fn close(&mut self) {
        drop(&self.file);
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

    fn read_blob_all(&mut self) -> Vec<u8> {
        todo!()
    }

    fn read_blob_amount(&mut self, amount: i64) -> Vec<u8> {
        todo!()
    }

    fn read_string_all(&mut self) -> String {
        todo!()
    }

    fn write_string(&mut self, text: &str) {
        todo!()
    }

    fn write_blob(&mut self, blob: Vec<u8>) {
        todo!()
    }
}
