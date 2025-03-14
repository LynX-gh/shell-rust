mod builtins;
mod external_exec;

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
        // println!("{:?}", inputs);
        match command {
            "pwd" => {
                builtins::directory_builtin::pwd_builtin()
            },
            "type" => {
                builtins::type_builtin::type_builtin(inputs)
            },
            "echo" => {
                builtins::echo_builtin::echo_builtin(inputs)
            },
            "exit" => {
                let exit_code = if let Some(exit_code) = inputs.pop_front() { exit_code.parse::<i32>().unwrap_or(0) } else { 0 };
                std::process::exit(exit_code);
            },
            _ => external_exec::run_external_executable(command, inputs.clone())
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
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.trim_newline();
                let mut inputs = input.splitn(2, ' ').collect::<VecDeque<_>>();

                // Handle User Input
                let output = handle_user_input(&mut inputs);
                io::stdout().write(output.as_bytes()).unwrap();
            },
            Err(error) => println!("error: {}", error),
        }

        // Print a newline
        io::stdout().write("\n".as_bytes()).unwrap();
    }
}
