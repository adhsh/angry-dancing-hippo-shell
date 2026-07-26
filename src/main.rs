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

        if command == "exit 0" {
            std::process::exit(0);
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
       } else if command.starts_with("type ") {
            let arg = &command[5..];
            let builtins = ["echo", "exit", "type"];

            if builtins.contains(&arg) {
                println!("{} is a shell builtin", arg);
            } else {
                println!("{}: not found", arg);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
