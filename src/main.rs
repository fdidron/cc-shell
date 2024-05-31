#[allow(unused_imports)]
use std::io::{self, Write};

mod utils;
mod builtins;

const PROMPT: &str = "$ ";

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

enum CommandError {
    NotFound,
    WrongArguments,
    Failed,
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
                ["exit"] => builtins::exit(),
                ["exit", code] => match builtins::exit_with_code(code) {
                        Ok(_) => (),
                        Err(CommandError::WrongArguments) => eprintln!("exit: wrong arguments"),
                        Err(_) => eprintln!("exit: failed to exit"),
                },
                ["type", ..] => builtins::type_cmd(tokens[1]),
                ["echo"] => builtins::echo(None),
                ["echo", ..] => builtins::echo(Some(&tokens[1..].join(" "))),
                ["pwd"] => match builtins::pwd() {
                    Ok(_) => (),
                    Err(_) => eprintln!("pwd: failed to get current directory"),
                },
                _ => {
                    let cmd = tokens[0];
                    let args = &tokens[1..];
                    builtins::execute(cmd, args);
                }
            }
        }
    }
}
