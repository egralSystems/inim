use std::{fs::File, io::Read, path::Path, rc::Rc};

use inim::{
    io::{console::Console, fs},
    Inim,
};

fn main() {
    let mut inim = Inim::new();

    let mut console = Console::new();
    console
        .set_print(|text| println!("{}", text))
        .set_debug(|text, _, _| println!("Debug: {}", text));

    inim.register_console(console);
    inim.register_fs::<Fl>();

    inim.run("debug(35+34);");
    inim.run("let file = open(\"LICENSE\"); print(file.read()); file.close()");
}

#[derive(Clone)]
struct Fl {
    file: Rc<File>,
}

impl fs::File for Fl {
    fn open(path: &str) -> Self {
        let path = Path::new(path);

        Fl {
            file: Rc::new(File::open(path).unwrap()),
        }
    }

    fn close(&mut self) {
        drop(&self.file);
    }

    fn read_all(&mut self) -> String {
        let mut buf = String::new();
        self.file.as_ref().read_to_string(&mut buf).unwrap();

        buf
    }
}
