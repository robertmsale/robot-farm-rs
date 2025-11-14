use crate::{
    shared::{codex_exec::CodexExecBuilder, docker::DockerRunBuilder},
    system::codex_config::{self, AgentKind as CodexAgentKind},
};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum Persona {
    Orchestrator,
    Worker(i64),
}

#[derive(Debug, Clone, Copy)]
pub struct RunnerConfig {
    pub api_port: Option<u16>,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            api_port: Some(8080),
        }
    }
}

pub struct CommandPlan {
    pub docker_args: Vec<String>,
    pub codex_args: Vec<String>,
}

pub fn plan_codex_run(
    persona: Persona,
    session_id: Option<&str>,
    config: RunnerConfig,
) -> CommandPlan {
    let mut codex = match session_id {
        Some(id) => CodexExecBuilder::resume().session_id(id.to_string()),
        None => CodexExecBuilder::new(),
    };

    let agent_kind = match persona {
        Persona::Orchestrator => CodexAgentKind::Orchestrator,
        Persona::Worker(_) => CodexAgentKind::Worker,
    };
    let launch_settings = codex_config::settings_for(agent_kind);

    codex = codex
        .json(true)
        .output_schema("/opt/robot-farm/schema.json")
        .config_override("mcp_servers.robot_farm.enabled=true")
        .config_override("mcp_servers.robot_farm.tool_timeout_sec=900")
        .config_override(format!(
            "mcp_servers.robot_farm.url=\"http://127.0.0.1:{}/mcp\"",
            config.api_port.unwrap_or(8080)
        ))
        .config_override(format!("model=\"{}\"", launch_settings.model))
        .config_override(format!(
            "model_reasoning_effort=\"{}\"",
            launch_settings.reasoning
        ));

    let (workspace_host, workspace_container) = match persona {
        Persona::Orchestrator => (PathBuf::from("./staging"), PathBuf::from("/workspace")),
        Persona::Worker(id) => (
            PathBuf::from(format!("./ws{id}")),
            PathBuf::from("/workspace"),
        ),
    };

    let docker = DockerRunBuilder::new("codex:latest")
        .remove_container(true)
        .attach("stdout")
        .attach("stderr")
        .workdir(&workspace_container)
        .volume(workspace_host, &workspace_container, Some("rw".into()));

    CommandPlan {
        docker_args: docker.build(),
        codex_args: codex.change_dir("/workspace").build(),
    }
}
