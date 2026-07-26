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

        if command == "exit" {
            std::process::exit(0);
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
        } else if command.starts_with("type ") {
            let arg = &command[5..];
            let builtins = ["echo", "exit", "type"];

            if builtins.contains(&arg) {
                println!("{} is a shell builtin", arg);
            } else {
                let path = std::env::var("PATH").unwrap_or_default();
                let mut found = false;
                for dir in path.split(':') {
                    let full_path = format!("{}/{}", dir, arg);
                    let path_obj = std::path::Path::new(&full_path);
                    if path_obj.exists() {
                        use std::os::unix::fs::PermissionsExt;
                        if let Ok(metadata) = std::fs::metadata(&full_path) {
                            if metadata.permissions().mode() & 0o111 != 0 {
                                println!("{} is {}", arg, full_path);
                                found = true;
                                break;
                      }
                  }
              }
          }
          if !found {
              println!("{}: not found", arg);
          }
      }
  }
