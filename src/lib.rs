#![no_std]

extern crate alloc;

use core::marker::PhantomData;

use alloc::vec::Vec;
use io::{console::Console, fs::File};
use rhai::{packages::Package, Engine, Scope};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;

pub mod io;

pub struct Inim<'a, C, F>
where
    C: Console + 'static,
    F: File + Clone + 'static,
{
    pub engine: Engine,
    scopes: Vec<Scope<'a>>,
    current_scope: usize,

    console_phantom: PhantomData<C>,
    fs_phantom: PhantomData<F>,
}

impl<'a, C, F> Inim<'a, C, F>
where
    C: Console + 'static,
    F: File + Clone + 'static,
{
    pub fn new() -> Self {
        let mut inim = Inim {
            engine: Engine::new(),
            scopes: Vec::new(),
            current_scope: 0,
            console_phantom: PhantomData,
            fs_phantom: PhantomData,
        };

        // Setup packages
        inim.engine
            .register_global_module(SciPackage::new().as_shared_module())
            .register_global_module(RandomPackage::new().as_shared_module());

        // Register Console
        inim.engine.on_print(C::print);
        inim.engine.on_debug(C::debug);

        // Registering FS
        inim.engine
            .register_type_with_name::<F>("File")
            .register_fn("open", F::open)
            .register_fn("close", F::close)
            .register_fn("seek", F::seek)
            .register_fn("step", F::step)
            .register_fn("rewind", F::rewind)
            .register_fn("read", F::read_all)
            .register_fn("read", F::read_until)
            .register_fn("read", F::read_amount)
            .register_fn("write", F::write);

        // Add default scope
        inim.scopes.push(Scope::<'a>::new());

        inim
    }

    pub fn set_scope(&mut self, scope_num: usize) -> &mut Self {
        self.current_scope = scope_num;

        self
    }

    pub fn run_file(&mut self, path: &str) -> &mut Self {
        let mut file = F::open(path, "r");

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
