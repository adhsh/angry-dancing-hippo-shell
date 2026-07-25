#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdin().read_line(&mut command).unwrap();
    print!(command);
    io::stdout().flush().unwrap();
}
