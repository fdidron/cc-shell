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
        let cmd = input.trim();
        if !cmd.is_empty() {
            let tokens = tokenize(cmd);

            match tokens[..] {
                ["exit", code] => {
                    let code = code.parse::<i32>().unwrap_or(1);
                    std::process::exit(code);
                }
                _ => println!("{}: command not found", cmd)
            }
        }
        io::stdout().flush().unwrap();
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
    }
}
