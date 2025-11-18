use openapi::models::ExecResult;
use path_clean::PathClean;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("failed to spawn command: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid working directory: {0}")]
    InvalidCwd(String),
}

pub fn resolve_working_dir(
    workspace_root: &Path,
    base_dir: &Path,
    override_cwd: Option<&str>,
) -> Result<PathBuf, ShellError> {
    let workspace = canonicalize_or_clean(workspace_root);
    let base = canonicalize_or_clean(base_dir);
    if !base.starts_with(&workspace) {
        return Err(ShellError::InvalidCwd(format!(
            "base directory {} escapes workspace root {}",
            base.display(),
            workspace_root.display()
        )));
    }

    let Some(raw) = override_cwd.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(base);
    };

    let override_path = Path::new(raw);
    let candidate = if override_path.is_absolute() {
        override_path.to_path_buf()
    } else {
        base.join(override_path)
    };
    Ok(candidate.clean())
}

fn canonicalize_or_clean(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.clean())
}

pub async fn run_shell_command(cwd: &Path, command: &str) -> Result<ExecResult, ShellError> {
    let output = Command::new("bash")
        .arg("-lc")
        .arg(command)
        .current_dir(cwd)
        .output()
        .await?;

    let exit_code = output.status.code().unwrap_or_default();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(ExecResult {
        command: command.to_string(),
        exit_code,
        stdout,
        stderr,
    })
}
