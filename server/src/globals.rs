use std::sync::LazyLock;
use std::path::PathBuf;

pub static PROJECT_DIR: LazyLock<String> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .display()
        .to_string()
});

