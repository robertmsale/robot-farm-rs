use std::path::Path;

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;

use crate::globals::PROJECT_DIR;

use super::{
    parse_params,
    roles_all,
    schema_for_type,
    AgentRole,
    McpTool,
    ToolContext,
    ToolInvocationError,
    ToolInvocationResponse,
};

#[derive(Default)]
pub struct GitDiffTool;

#[async_trait]
impl McpTool for GitDiffTool {
    fn name(&self) -> &'static str {
        "git_diff"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Git Diff")
    }

    fn description(&self) -> &'static str {
        "Show git diff output for the staging repository (optionally scoped to a path)."
    }

    fn input_schema(&self) -> schemars::schema::Schema {
        schema_for_type::<GitDiffInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: GitDiffInput = parse_params(args)?;
        run_git_diff(input).await
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GitDiffInput {
    /// Relative path to show diff for. When omitted, shows the full diff.
    pub path: Option<String>,
    /// When true, show the staged diff (git diff --cached).
    #[serde(default)]
    pub staged: bool,
}

async fn run_git_diff(input: GitDiffInput) -> Result<ToolInvocationResponse, ToolInvocationError> {
    let repo = Path::new(PROJECT_DIR.as_str()).join("staging");
    let mut command = Command::new("git");
    command.current_dir(&repo).arg("diff");
    if input.staged {
        command.arg("--cached");
    }
    if let Some(path) = input.path {
        if !path.trim().is_empty() {
            command.arg("--").arg(path);
        }
    }
    let output = command
        .output()
        .await
        .map_err(|err| ToolInvocationError::Internal(format!("failed to run git diff: {err}")))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        return Ok(ToolInvocationResponse::text_error(format!(
            "git diff failed (exit {}):\n{}\n{}",
            output.status.code().unwrap_or(-1),
            stdout,
            stderr.trim()
        )));
    }
    if stdout.trim().is_empty() {
        Ok(ToolInvocationResponse::text("(no diff)"))
    } else {
        Ok(ToolInvocationResponse::text(stdout))
    }
}
