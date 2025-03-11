#[allow(unused_imports)]
use core::fmt;
use std::env;
use std::io::{self, Write};
use std::collections::VecDeque;

trait StringExt {
    fn trim_newline(self: &mut Self);
}

impl StringExt for String {
    // Function to trim the newline at the end of the command entered by the user
    fn trim_newline(&mut self) {
        while self.ends_with('\n') {
            self.pop();
            while self.ends_with('\r') {
                self.pop();
            }
        }
    }
}

fn handle_user_input(inputs: &mut VecDeque<&str>) -> String {
    if let Some(command) = inputs.pop_front() {
        // println!("{:?}", inputs);
        match command {
            "type" => {
                let command = if let Some(command) = inputs.pop_front() { command } else { "" };
                match command {
                    "echo" => "echo is a shell builtin".to_string(),
                    "exit" => "exit is a shell builtin".to_string(),
                    "type" => "type is a shell builtin".to_string(),
                    _ => format!("{command}: not found"),
                }
            },
            "echo" => {
                let output = if let Some(output) = inputs.pop_front() { output } else { "" };
                format!("{}", output)
            },
            "exit" => {
                let exit_code = if let Some(exit_code) = inputs.pop_front() { exit_code.parse::<i32>().unwrap_or(0) } else { 0 };
                std::process::exit(exit_code);
            },
            _ => format!("{command}: command not found"),
        }
    } else {
        "".to_string()
    }
}

fn main() {
    // Get the PATH environment variable
    let path = match env::var("PATH") {
        Ok(val) => val,
        Err(e) => panic!("could not interpret PATH: {}", e),
    };

    // Split PATH using the appropriate separator for the OS
    let path_separator = if cfg!(windows) { ';' } else { ':' };
    let path_entries: Vec<&str> = path.split(path_separator).collect();
    println!("PATH entries: {:?}", path_entries);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_newline();
        let mut inputs = input.splitn(2, ' ').collect::<VecDeque<_>>();

        // Handle User Input
        let output = handle_user_input(&mut inputs);
        io::stdout().write(output.as_bytes()).unwrap();

        // Print a newline
        io::stdout().write("\n".as_bytes()).unwrap();
    }
}
