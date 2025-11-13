use std::{path::Path, time::Duration};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::{process::Command, time::timeout};

use crate::{
    globals::PROJECT_DIR,
    routes::config::{ConfigError, load_config_from_disk},
    shared::shell,
};

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
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
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: ProjectCommandRunInput = parse_params(args)?;
        run_command(input).await
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ProjectCommandRunInput {
    pub command_id: String,
}

async fn run_command(
    input: ProjectCommandRunInput,
) -> Result<ToolInvocationResponse, ToolInvocationError> {
    let config = load_config_from_disk().map_err(map_config_error)?;
    let command = config
        .commands
        .into_iter()
        .find(|cmd| cmd.id == input.command_id)
        .ok_or_else(|| ToolInvocationError::NotFound(format!("command {}", input.command_id)))?;

    if command.exec.is_empty() {
        return Err(ToolInvocationError::InvalidParams(format!(
            "command {} has no exec definition",
            command.id
        )));
    }

    let workspace = Path::new(PROJECT_DIR.as_str());
    let cwd = shell::resolve_working_dir(workspace, command.cwd.as_deref())
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

fn map_config_error(err: ConfigError) -> ToolInvocationError {
    match err {
        ConfigError::NotFound(_) => ToolInvocationError::NotFound("config.json".to_string()),
        _ => ToolInvocationError::Internal(err.to_string()),
    }
}
