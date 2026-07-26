#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {	

    loop {

    print!("$ ");
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    command = command.trim().to_string();
	
    if command.starts_with("type ") {
    let arg = &command[5..];
    let builtins = ["echo", "exit", "type"];

    if builtins.contain(&arg) {
	println!("{} is a shell builtin", arg);
    } else {
	println!("{}: not found", arg)
    }
    }

}
}
