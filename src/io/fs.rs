use alloc::string::String;

pub trait File {
    fn open(path: &str, options: &str) -> Self;
    fn close(&mut self);

    fn seek(&mut self, offset: usize);
    fn step(&mut self, step: i64);
    fn rewind(&mut self);

    fn read_all(&mut self) -> String;
    fn read_char(&mut self) -> char;

    fn read_amount(&mut self, amount: usize) -> String {
        let mut output = String::new();

        for _ in 0..amount {
            output.push(self.read_char());
        }

        output
    }

    fn read_until(&mut self, stop: char) -> String {
        let mut output = String::new();

        loop {
            let ch = self.read_char();

            if ch == stop {
                break;
            }

            output.push(ch);
        }

        output
    }

    fn write(&mut self, text: &str);
}
