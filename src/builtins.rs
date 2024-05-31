use crate::CommandError;


pub fn exit() {
    std::process::exit(0);
}

pub fn exit_with_code(code: &str) -> Result<(), CommandError>{
    match code.parse::<i32>() {
        Ok(code) => std::process::exit(code),
        Err(_) => Err(CommandError::WrongArguments),
    }
} 

pub fn echo (args: Option<&str>) {
    match args {
        Some(args) => println!("{}", args),
        None => println!(""),
    }
}

pub fn type_cmd(cmd: &str) {
    match cmd {
        "exit" | "type" | "echo" | "pwd" => println!("{} is a shell builtin", cmd),
        _ => {
            if let Some(path) = crate::utils::find_executable(cmd) {
                println!("{} is {}", cmd, path.display());
            } else {
                println!("{} not found", cmd);
            }
        }
    }
}

pub fn execute(cmd: &str, args: &[&str]) {
    if let Some(path) = crate::utils::find_executable(cmd) {
        let status = std::process::Command::new(path)
            .args(args)
            .status()
            .expect("failed to execute process");
        if !status.success() {
            eprintln!("{}: command failed with status {}", cmd, status);
        }
    } else {
        eprintln!("{}: command not found", cmd)
    }
}

pub fn pwd() -> Result<(), CommandError> {
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(())
        },
        Err(_) => Err(CommandError::Failed),
    }
}
