use std::{
    cell::RefCell,
    env::current_dir,
    fs::{create_dir_all, read_dir, remove_dir, remove_file, File},
    io::Read,
    os::unix::prelude::FileExt,
    path::Path,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use inim::{
    io::{console::Console, fs, sys::Sys},
    InimFactory, NativeResult,
};
use rhai::Dynamic;

fn main() {
    let mut inim_fac = InimFactory::<LinuxConsole, LinuxFile, LinuxSys>::new();
    let mut inim = inim_fac.build();

    inim.run_file("test.rhai");
}

#[derive(Clone)]
struct LinuxSys;

impl Sys for LinuxSys {
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
struct LinuxConsole;

impl Console for LinuxConsole {
    fn print(text: &str) {
        println!("{}", text);
    }

    fn debug(text: &str) {
        println!("{}", text);
    }
}

#[derive(Clone)]
struct LinuxFile {
    file: Rc<RefCell<File>>,
    offset: u64,
}

impl fs::File for LinuxFile {
    fn open(path: &str, options: &str) -> NativeResult<Self> {
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

        Ok(LinuxFile {
            file: Rc::new(RefCell::new(file)),
            offset: 0,
        })
    }

    fn close(&mut self) -> NativeResult<()> {
        drop(&self.file);
        OK(())
    }

    fn read_all(&mut self) -> NativeResult<String> {
        let mut buf = String::new();
        self.file.borrow_mut().read_to_string(&mut buf).unwrap();

        Ok(buf)
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

    fn write(&mut self, text: &str) {
        self.file
            .borrow_mut()
            .write_all_at(text.as_bytes(), self.offset)
            .unwrap();
    }
}
