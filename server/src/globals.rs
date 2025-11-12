use std::sync::LazyLock;

pub static PROJECT_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::current_dir().unwrap().display().to_string());
