use core::fmt;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    // Handle Invalid input
    trim_newline(&mut input);
    let output = format!("{input}: command not found");
    io::stdout().write(output.as_bytes()).unwrap();
}

// Function to trim the newline at the end of the command entered by the user
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}