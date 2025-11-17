use std::path::Path;

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;

use super::Agent;
use crate::globals::PROJECT_DIR;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    roles_all, schema_for_type,
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

    fn input_schema(&self) -> Value {
        schema_for_type::<GitDiffInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: GitDiffInput = parse_params(args)?;
        run_git_diff(ctx, input).await
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

async fn run_git_diff(
    ctx: &ToolContext,
    input: GitDiffInput,
) -> Result<ToolInvocationResponse, ToolInvocationError> {
    let repo = repo_root_for_agent(&ctx.agent);
    if !repo.exists() {
        return Err(ToolInvocationError::Internal(format!(
            "repository {} not found",
            repo.display()
        )));
    }
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

fn repo_root_for_agent(agent: &Agent) -> std::path::PathBuf {
    let root = Path::new(PROJECT_DIR.as_str());
    match agent {
        Agent::WorkerWithId(id) => root.join(format!("ws{id}")),
        _ => root.join("staging"),
    }
}
