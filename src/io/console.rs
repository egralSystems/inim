use rhai::Engine;

use std::prelude::v1::*;

pub fn register_console<C: Console>(engine: &mut Engine) {
    engine.on_print(C::print);
    engine.on_debug(|text, source, position| {
        C::debug(
            format!(
                "{}:{}:{} {}",
                source.unwrap(),
                position.line().unwrap_or(0),
                position.position().unwrap_or(0),
                text
            )
            .as_str(),
        )
    });
}

pub trait Console: Clone + 'static {
    fn print(text: &str);
    fn debug(text: &str);
}
