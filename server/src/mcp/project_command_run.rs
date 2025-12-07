use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::{process::Command, time::timeout};

use crate::{globals::PROJECT_DIR, shared::shell};

use super::{
    Agent, AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse,
    parse_params,
    project_commands::{ProjectCommandRegistry, command_visible_for_role, is_post_turn_check},
    roles_all, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct ProjectCommandRunTool;

#[async_trait]
impl McpTool for ProjectCommandRunTool {
    fn name(&self) -> &'static str {
        "project_command_run"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Run Project Command")
    }

    fn description(&self) -> &'static str {
        "Execute a declared project command (with timeout enforcement)."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<ProjectCommandRunInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: ProjectCommandRunInput = parse_params(args)?;
        run_command(&ctx.agent, input).await
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Run a declared project command by id.")]
struct ProjectCommandRunInput {
    /// Identifier of the project command to execute (from config.json).
    pub command_id: String,
}

async fn run_command(
    agent: &Agent,
    input: ProjectCommandRunInput,
) -> Result<ToolInvocationResponse, ToolInvocationError> {
    let command = ProjectCommandRegistry::global()
        .get(&input.command_id)
        .ok_or_else(|| ToolInvocationError::NotFound(format!("command {}", input.command_id)))?;

    let role = agent.role();
    if !command_visible_for_role(role, &command) {
        return Err(ToolInvocationError::Unauthorized(
            "command is hidden for this agent".to_string(),
        ));
    }

    if command.hidden.unwrap_or(false)
        && matches!(role, AgentRole::Worker | AgentRole::Orchestrator)
        && is_post_turn_check(&command.id)
    {
        return Ok(ToolInvocationResponse::text_error(
            "Finish your turn with COMPLETE_TASK intent to use this tool.",
        ));
    }

    if command.exec.is_empty() {
        return Err(ToolInvocationError::InvalidParams(format!(
            "command {} has no exec definition",
            command.id
        )));
    }

    let workspace_root = Path::new(PROJECT_DIR.as_str());
    let worktree_root = worktree_root_for_agent(agent, workspace_root);
    let cwd = shell::resolve_working_dir(workspace_root, &worktree_root, command.cwd.as_deref())
        .map_err(|err| ToolInvocationError::InvalidParams(err.to_string()))?;

    let mut builder = Command::new(&command.exec[0]);
    for arg in command.exec.iter().skip(1) {
        builder.arg(arg);
    }
    builder.current_dir(&cwd);
    builder.kill_on_drop(true);

    let timeout_secs = command.timeout_seconds.unwrap_or(900).max(1) as u64;
    let duration = Duration::from_secs(timeout_secs);

    let child = builder.spawn().map_err(|err| {
        ToolInvocationError::Internal(format!("failed to spawn {}: {err}", input.command_id))
    })?;

    let output = match timeout(duration, child.wait_with_output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(err)) => {
            return Err(ToolInvocationError::Internal(format!(
                "failed to run {}: {err}",
                input.command_id
            )));
        }
        Err(_) => {
            return Ok(ToolInvocationResponse::text_error(format!(
                "command {} timed out after {} seconds",
                input.command_id, timeout_secs
            )));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit = output.status.code().unwrap_or(-1);
    let payload = json!({
        "id": command.id,
        "command": command.exec,
        "cwd": cwd.display().to_string(),
        "exit_code": exit,
        "stdout": stdout,
        "stderr": stderr,
    });
    let text = serialize_json(&payload)?;
    let mut response = ToolInvocationResponse::text(text);
    if !output.status.success() {
        response.is_error = true;
    }
    Ok(response)
}

fn worktree_root_for_agent(agent: &Agent, workspace_root: &Path) -> PathBuf {
    match agent {
        Agent::WorkerWithId(id) => workspace_root.join(format!("ws{id}")),
        _ => workspace_root.join("staging"),
    }
}
