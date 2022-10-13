// use inim::prelude::*;
use rhai::plugin::*;
use rhai::Dynamic;

#[export_module]
mod greeter {
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

fn main() {
    let greeter_mod = exported_module!(greeter);

    // let mut inim = <Inim>>::new();
    // inim.register_module("greeter", greeter_mod.into())
    // .update_modules();

    // inim.run_file("test.rhai");
}
