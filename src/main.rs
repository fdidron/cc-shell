#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let cmd = input.trim();

    if cmd.is_empty() {
        println!("No command entered");
    } else {
        println!("{}: command not found", cmd);
    }
}
