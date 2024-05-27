#[allow(unused_imports)]
use std::io::{self, Write};

const PROMPT: &str  = "$ ";

fn main() {
    print!("{}", PROMPT);
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let cmd = input.trim();
        if !cmd.is_empty() {
            match cmd {
                "exit" => break,
                _ => println!("{}: command not found", cmd)
            }
        }
        io::stdout().flush().unwrap();
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
    }
}
