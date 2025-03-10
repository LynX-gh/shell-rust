#[allow(unused_imports)]
use core::fmt;
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
        match command {
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
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_newline();
        let mut inputs = input.split_whitespace().collect::<VecDeque<_>>();

        // Handle User Input
        let output = handle_user_input(&mut inputs);
        io::stdout().write(output.as_bytes()).unwrap();

        // Print a newline
        io::stdout().write("\n".as_bytes()).unwrap();
    }
}
