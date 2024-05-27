#[allow(unused_imports)]
use std::io::{self, Write};

mod utils;

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
                        "exit" | "type" | "echo" => println!("{} is a shell builtin", cmd),
                        _ => {
                            if let Some(path) = utils::find_executable(cmd) {
                                println!("{} is {}", cmd, path.display());
                            } else {
                                println!("{} not found", cmd);
                            }
                        }
                    }
                }
                ["echo"] => {
                    println!("");
                }
                ["echo", ..] => {
                    println!("{}", tokens[1..].join(" "));
                }
                _ => {
                    let cmd = tokens[0];
                    let args = &tokens[1..];
                    if let Some(path) = utils::find_executable(cmd) {
                        let status = std::process::Command::new(path)
                            .args(args)
                            .status()
                            .expect("failed to execute process");
                        if !status.success() {
                            eprintln!("{}: command failed with status {}", raw, status);
                        }
                    } else {
                        eprintln!("{}: command not found", raw)
                    }
                }
            }
        }
    }
}
