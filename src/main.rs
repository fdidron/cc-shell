#[allow(unused_imports)]
use std::io::{self, Write};

const PROMPT: &str  = "$ ";

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn main() {
    print!("{}", PROMPT);
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let raw = input.trim();
        if !raw.is_empty() {
            let tokens = tokenize(raw);
            match tokens[..] {
                ["exit"] => std::process::exit(0),
                ["exit", code] => {
                    let code = code.parse::<i32>().unwrap_or(1);
                    std::process::exit(code);
                },
                ["echo"] => {
                    println!();
                },
                ["echo",  ..] => {
                    println!("{}", tokens[1..].join(" "));
                }
                _ => println!("{}: command not found", raw)
            }
        }
        io::stdout().flush().unwrap();
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
    }
}
