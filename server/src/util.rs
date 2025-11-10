use std::fs;
use std::path::Path;

const APPEND_FAILURE_PREFIX: &str =
    "AgentsAppendFailure: Could not generate AGENTS.override.md.";

pub fn append_files(paths: Vec<&str>) -> String {
    let mut contents = String::new();

    for path_str in paths {
        let path = Path::new(path_str);
        let metadata = fs::metadata(path).unwrap_or_else(|err| {
            panic!(
                "{APPEND_FAILURE_PREFIX} Failed to access '{path_str}': {err}",
            )
        });

        if metadata.is_dir() {
            panic!(
                "{APPEND_FAILURE_PREFIX} '{path_str}' is a directory, expected a file.",
            );
        }

        let data = fs::read_to_string(path).unwrap_or_else(|err| {
            panic!(
                "{APPEND_FAILURE_PREFIX} Failed to read '{path_str}': {err}",
            )
        });

        contents.push_str(&data);
    }

    contents
}