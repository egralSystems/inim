use inim::{io::console::Console, Inim};

fn main() {
    let mut inim = Inim::new();

    let mut console = Console::new();
    console
        .set_print(|text| println!("{}", text))
        .set_debug(|text, _, _| println!("Debug: {}", text));

    inim.register_console(console);

    inim.run("debug(35+34)");
}
