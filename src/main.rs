#[allow(unused_imports)]
use std::io::{self, Write};

const PROMPT: &str = "$ ";

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn main() {
    let stdin = io::stdin();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
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
                }
                ["type", ..] => {
                    let cmd = tokens[1];
                    match cmd {
                        "exit" | "type" | "echo" => print!("{} is a shell built-in", cmd),
                        _ => print!("{} not found", cmd),
                    }
                }
                ["echo"] => {
                    print!("");
                }
                ["echo", ..] => {
                    print!("{}", tokens[1..].join(" "));
                }
                _ => print!("{}: command not found", raw),
            }
            println!();
            io::stdout().flush().unwrap();
        }
    }
}
