use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Welcome to the life of Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(message.as_str(), width, &mut writer).unwrap();
}
