use openapi::models::ExecResult;
use std::path::Path;
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("failed to spawn command: {0}")]
    Io(#[from] std::io::Error),
}

pub async fn run_shell_command(root: &Path, command: &str) -> Result<ExecResult, ShellError> {
    let output = Command::new("bash")
        .arg("-lc")
        .arg(command)
        .current_dir(root)
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
