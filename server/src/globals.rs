use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static PROJECT_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::current_dir().unwrap().display().to_string());

pub static PROJECT_NAME: LazyLock<String> =
    LazyLock::new(|| {
        let p = PathBuf::from(PROJECT_DIR.as_str());
        let dir = fs::canonicalize(p).unwrap_or_else(|_| panic!("project doesn't exist"));
        String::from(dir.file_name().unwrap().to_str().unwrap())
    });