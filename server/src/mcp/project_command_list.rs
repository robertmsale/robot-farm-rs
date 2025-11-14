use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    project_commands::ProjectCommandRegistry, roles_all, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct ProjectCommandListTool;

#[async_trait]
impl McpTool for ProjectCommandListTool {
    fn name(&self) -> &'static str {
        "project_command_list"
    }

    fn title(&self) -> Option<&'static str> {
        Some("List Project Commands")
    }

    fn description(&self) -> &'static str {
        "Return the declarative command definitions from config.json."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<ProjectCommandListInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let _: ProjectCommandListInput = parse_params(args)?;
        let commands = ProjectCommandRegistry::global().list();
        let payload = json!({
            "commands": commands,
        });
        let text = serialize_json(&payload)?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ProjectCommandListInput {}
