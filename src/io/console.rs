use rhai::Position;

// pub type PrintFn = fn(&str);
// pub type DebugFn = fn(&str, Option<&str>, Position);

// pub struct Console {
//     pub print: PrintFn,
//     pub debug: DebugFn,
// }

// impl Console {
//     pub fn new() -> Self {
//         Self {
//             print: |_| {},
//             debug: |_, _, _| {},
//         }
//     }

//     pub fn set_print(&mut self, print: PrintFn) -> &mut Self {
//         self.print = print;
//         self
//     }

//     pub fn set_debug(&mut self, debug: DebugFn) -> &mut Self {
//         self.debug = debug;
//         self
//     }
// }

pub trait Console {
    fn print(text: &str);
    fn debug(text: &str, source: Option<&str>, position: Position);
}
