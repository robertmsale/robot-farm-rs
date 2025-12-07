use std::path::Path;

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::process::Command;

use super::Agent;
use crate::globals::PROJECT_DIR;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    roles_all, schema_for_type,
};

#[derive(Default)]
pub struct GitStatusTool;

#[async_trait]
impl McpTool for GitStatusTool {
    fn name(&self) -> &'static str {
        "git_status"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Git Status")
    }

    fn description(&self) -> &'static str {
        "Show the concise git status for the staging repository."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<GitStatusInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let _: GitStatusInput = parse_params(args)?;
        run_git_command(ctx, ["status", "-sb"]).await
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "No parameters required for git status.",
    schema_with = "empty_object_schema"
)]
struct GitStatusInput {}

fn empty_object_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    serde_json::from_value(json!({
        "type": "object",
        "properties": {},
        "description": "No parameters required for git status."
    }))
    .expect("valid empty object schema")
}

async fn run_git_command<const N: usize>(
    ctx: &ToolContext,
    args: [&str; N],
) -> Result<ToolInvocationResponse, ToolInvocationError> {
    let repo = repo_root_for_agent(&ctx.agent);
    if !repo.exists() {
        return Err(ToolInvocationError::Internal(format!(
            "repository {} not found",
            repo.display()
        )));
    }
    let output = Command::new("git")
        .args(args)
        .current_dir(&repo)
        .output()
        .await
        .map_err(|err| ToolInvocationError::Internal(format!("failed to run git: {err}")))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        return Ok(ToolInvocationResponse::text_error(format!(
            "git command failed (exit {}):\n{}\n{}",
            output.status.code().unwrap_or(-1),
            stdout,
            stderr.trim()
        )));
    }
    Ok(ToolInvocationResponse::text(stdout))
}

fn repo_root_for_agent(agent: &Agent) -> std::path::PathBuf {
    let root = Path::new(PROJECT_DIR.as_str());
    match agent {
        Agent::WorkerWithId(id) => root.join(format!("ws{id}")),
        _ => root.join("staging"),
    }
}
