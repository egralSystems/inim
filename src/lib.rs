#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use io::{console::Console, fs::File};
use rhai::{packages::Package, Engine, Scope};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;

pub mod io;

pub struct Inim<'a> {
    pub engine: Engine,
    console: Option<Console>,
    scopes: Vec<Scope<'a>>,
    current_scope: usize,
}

impl<'a> Inim<'a> {
    pub fn new() -> Self {
        let mut inim = Inim {
            engine: Engine::new(),
            console: None,
            scopes: Vec::new(),
            current_scope: 0,
        };

        // Setup packages
        inim.engine
            .register_global_module(SciPackage::new().as_shared_module())
            .register_global_module(RandomPackage::new().as_shared_module());

        // Add default scope
        inim.scopes.push(Scope::<'a>::new());

        inim
    }

    pub fn register_console(&mut self, console: Console) -> &mut Self {
        self.engine.on_print(console.print);
        self.engine.on_debug(console.debug);

        self.console = Some(console);

        self
    }

    pub fn register_fs<FS: File + Clone + 'static>(&mut self) -> &mut Self {
        self.engine
            .register_type_with_name::<FS>("File")
            .register_fn("open", FS::open)
            .register_fn("close", FS::close)
            .register_fn("seek", FS::seek)
            .register_fn("step", FS::step)
            .register_fn("rewind", FS::rewind)
            .register_fn("read", FS::read_all)
            .register_fn("read", FS::read_until)
            .register_fn("read", FS::read_amount)
            .register_fn("write", FS::write);

        self
    }

    pub fn set_scope(&mut self, scope_num: usize) -> &mut Self {
        self.current_scope = scope_num;

        self
    }

    pub fn run_file<FS: File + Clone + 'static>(&mut self, path: &str) -> &mut Self {
        let mut file = FS::open(path, "r");

        let prog = file.read_all();

        self.run(prog.as_str());

        file.close();

        self
    }

    pub fn run(&mut self, prog: &str) -> &mut Self {
        self.engine
            .run_with_scope(&mut self.scopes[self.current_scope], prog)
            .unwrap(); // TODO: Remove the unwrap()

        self
    }
}
