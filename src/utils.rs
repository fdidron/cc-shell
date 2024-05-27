use std::{env, path::PathBuf};

pub fn parse_path_variable() -> Vec<PathBuf> {
    let mut paths = vec![];
    if let Some(path) = env::var_os("PATH") {
        for path in env::split_paths(&path) {
            paths.push(path)
        }
    }
    paths
}

pub fn find_executable(cmd: &str) -> Option<PathBuf> {
    let paths = parse_path_variable();
    for path in paths {
        let path = path.join(cmd);
        if path.exists() {
            return Some(path);
        }
    }
    None
}
