use core::fmt;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Handle User Input
        trim_newline(&mut input);
        let output = handle_user_input(& input);
        io::stdout().write(output.as_bytes()).unwrap();

        // Print a newline
        io::stdout().write("\n".as_bytes()).unwrap();
    }
}

fn handle_user_input(input: &str) -> String {
    format!("{input}: command not found")
    // match input {
    //     "exit" => std::process::exit(0),
    //     _ => format!("{input}: command not found"),
    // }
}

// Function to trim the newline at the end of the command entered by the user
fn trim_newline(s: &mut String) {
    while s.ends_with('\n') {
        s.pop();
        while s.ends_with('\r') {
            s.pop();
        }
    }
}