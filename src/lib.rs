#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use io::console::Console;
use rhai::Engine;

pub mod io;

pub struct Inim {
    pub engine: Engine,
    console: Option<Console>,
}

impl Inim {
    pub fn new() -> Self {
        let inim = Inim {
            engine: Engine::new(),
            console: None,
        };

        inim
    }

    pub fn register_console(&mut self, console: Console) -> &mut Self {
        self.engine.on_print(console.print);
        self.engine.on_debug(console.debug);

        self.console = Some(console);

        self
    }

    pub fn run(&mut self, prog: &str) {
        self.engine.run(prog).unwrap(); // TODO: Remove the unwrap()
    }
}
