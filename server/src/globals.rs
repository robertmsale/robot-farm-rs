use std::sync::LazyLock;
use std::env;
use std::path::PathBuf;

static PROJECT_DIR: LazyLock<String> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .display()
        .to_string()
});