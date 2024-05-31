use crate::CommandError;

pub fn exit() {
    std::process::exit(0);
}

pub fn exit_with_code(code: &str) -> Result<(), CommandError> {
    match code.parse::<i32>() {
        Ok(code) => std::process::exit(code),
        Err(_) => Err(CommandError::WrongArguments),
    }
}

pub fn echo(args: Option<&str>) {
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
        }
        Err(_) => Err(CommandError::Failed),
    }
}
fn set_oldpwd() {
    std::env::set_var(
        "OLDPWD",
        std::env::current_dir().unwrap_or(std::path::PathBuf::new()),
    );
}

pub fn cd(path: &str) -> Result<(), CommandError> {
    match path {
        "-" => {
            let previous = match std::env::var("OLDPWD") {
                Ok(path) => path,
                Err(_) => return Err(CommandError::Failed),
            };
            set_oldpwd();
            match std::env::set_current_dir(previous) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => return Err(CommandError::Failed),
            }
        }
        _ => (),
    }

    set_oldpwd();

    match path {
        "~" => {
            let home = match std::env::var("HOME") {
                Ok(home) => home,
                Err(_) => return Err(CommandError::Failed),
            };
            match std::env::set_current_dir(home) {
                Ok(_) => {
                    // Set the OLDPWD variable to the current directory
                    return Ok(());
                }
                Err(_) => {
                    return Err(CommandError::Failed);
                }
            }
        }
        _ => (),
    };
    // TODO: Handle the - character, this needs to keep track of the previous directory

    match std::env::set_current_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(CommandError::Failed),
    }
}
