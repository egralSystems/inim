#[cfg(not(feature = "std"))]
pub use dummy::{
    DummyConsole as DefaultConsole, DummyFile as DefaultFile, DummyNet as DefaultNet,
    DummySys as DefaultSys,
};
#[cfg(feature = "std")]
pub use standard::{
    StdConsole as DefaultConsole, StdFile as DefaultFile, StdNet as DefaultNet,
    StdSys as DefaultSys,
};

#[cfg(feature = "std")]
mod standard {
    extern crate std as stdstd;

    use stdstd::{
        cell::RefCell,
        env::current_dir,
        format,
        fs::{create_dir_all, read_dir, remove_dir, remove_file, File},
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        os::unix::prelude::FileExt,
        path::Path,
        prelude::v1::*,
        println,
        rc::Rc,
        time::{SystemTime, UNIX_EPOCH},
        vec,
    };

    use rhai::Dynamic;

    use crate::prelude::net::Net;

    use super::{super::console::Console, super::fs, super::sys::Sys};

    #[derive(Clone)]
    pub struct StdSys;

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
    pub struct StdConsole;

    impl Console for StdConsole {
        fn print(text: &str) {
            println!("{}", text);
        }

        fn debug(text: &str) {
            println!("{}", text);
        }
    }

    #[derive(Clone)]
    pub struct StdFile {
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
            let mut buffer = Vec::new();

            self.file.borrow_mut().read_to_end(&mut buffer).unwrap();

            buffer
        }

        fn read_blob_amount(&mut self, amount: i64) -> Vec<u8> {
            let mut buffer = Vec::new();
            buffer.resize(amount as usize, 0);

            self.file.borrow_mut().read_exact(&mut buffer).unwrap();

            buffer
        }

        fn read_string_all(&mut self) -> String {
            let mut out = String::new();
            self.file.borrow_mut().read_to_string(&mut out).unwrap();
            out
        }

        fn write_string(&mut self, text: &str) {
            self.file
                .borrow_mut()
                .write_all_at(text.as_bytes(), self.offset)
                .unwrap();
        }

        fn write_blob(&mut self, blob: Vec<u8>) {
            self.file
                .borrow_mut()
                .write_all_at(&blob, self.offset)
                .unwrap();
        }
    }

    #[derive(Clone)]
    pub struct StdNet {
        addr: String,
        listener: Option<Rc<RefCell<TcpListener>>>,
        stream: Option<Rc<RefCell<TcpStream>>>,
        timeout: Option<i64>,
    }

    impl Net for StdNet {
        fn tcp() -> Self {
            StdNet {
                listener: None,
                stream: None,
                timeout: None,
                addr: "".into(),
            }
        }

        fn bind(&mut self, addr: &str) -> String {
            self.addr = addr.into();
            let listener = match TcpListener::bind(self.addr.clone()) {
                Ok(listener) => listener,
                Err(_) => return "Error: Bind failed!".into(),
            };

            self.listener = Some(Rc::new(RefCell::new(listener)));

            "OK".into()
        }

        fn connect(&mut self, addr: &str) -> String {
            self.addr = addr.into();
            let stream = match TcpStream::connect(self.addr.clone()) {
                Ok(stream) => stream,
                Err(err) => return format!("Error: {:?}", err),
            };

            self.stream = Some(Rc::new(RefCell::new(stream)));

            "OK".into()
        }

        fn set_timeout(&mut self, timeout: i64) {
            self.timeout = Some(timeout);
        }

        fn accept(&mut self) -> Self {
            if let Some(listener) = &self.listener {
                let (stream, addr) = listener.borrow_mut().accept().unwrap();

                return StdNet {
                    addr: addr.to_string(),
                    listener: None,
                    stream: Some(Rc::new(RefCell::new(stream))),
                    timeout: None,
                };
            }

            StdNet {
                listener: None,
                stream: None,
                timeout: None,
                addr: "".into(),
            }
        }

        fn send_string(&mut self, msg: &str) -> String {
            if let Some(stream) = &self.stream {
                if stream.borrow_mut().write_all(msg.as_bytes()).is_err() {
                    return "Error: Sending failed!".into();
                }
            }

            "OK".into()
        }

        fn recv_string(&mut self, char_count: i64) -> String {
            if let Some(stream) = &self.stream {
                let mut buf = Vec::with_capacity(char_count as usize);
                stream.borrow_mut().read_exact(&mut buf).unwrap();

                return String::from_utf8(buf).unwrap();
            }

            "".into()
        }

        fn recv_line(&mut self) -> String {
            if let Some(stream) = &self.stream {
                let mut buf = String::new();
                stream.borrow_mut().read_to_string(&mut buf).unwrap();

                return buf;
            }

            "".into()
        }

        fn send_blob(&mut self, msg: Vec<u8>) -> String {
            if let Some(stream) = &self.stream {
                stream.borrow_mut().write_all(&msg).unwrap();
            }

            "".into()
        }

        fn recv_blob_amount(&mut self, byte_count: i64) -> Vec<u8> {
            if let Some(stream) = &self.stream {
                let mut buf = Vec::with_capacity(byte_count as usize);
                stream.borrow_mut().read_exact(&mut buf).unwrap();

                return buf;
            }

            vec![]
        }

        fn recv_blob(&mut self) -> Vec<u8> {
            if let Some(stream) = &self.stream {
                let mut buf = Vec::new();
                stream.borrow_mut().read_to_end(&mut buf).unwrap();

                return buf;
            }

            vec![]
        }

        fn close(&mut self) {}

        fn addr(&mut self) -> String {
            self.addr.clone()
        }
    }
}

#[cfg(not(feature = "std"))]
mod dummy {
    use std::prelude::v1::*;

    use rhai::Dynamic;

    use super::{super::console::Console, super::fs::File, super::net::Net, super::sys::Sys};

    #[derive(Clone)]
    pub struct DummyConsole;

    impl Console for DummyConsole {
        fn print(_text: &str) {}
        fn debug(_text: &str) {}
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

    #[derive(Clone)]
    pub struct DummySys;

    impl Sys for DummySys {
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

    #[derive(Clone)]
    pub struct DummyNet;

    impl Net for DummyNet {
        fn tcp() -> Self {
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

        fn addr(&mut self) -> String {
            "".into()
        }

        fn close(&mut self) {}
    }
}
