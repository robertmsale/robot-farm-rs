use crate::{
    docker::{DOCKER_IMAGE_ORCHESTRATOR, DOCKER_IMAGE_WORKER},
    globals::PROJECT_DIR,
    shared::{
        codex_exec::CodexExecBuilder,
        docker::{DockerRunBuilder, ensure_default_mcp_url},
    },
    system::codex_config::{self, AgentKind as CodexAgentKind},
};
use std::{env, path::PathBuf};

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
    let agent_label = match persona {
        Persona::Orchestrator => "orchestrator".to_string(),
        Persona::Worker(id) => format!("ws{id}"),
    };
    let launch_settings = codex_config::settings_for(agent_kind);

    let api_port = config.api_port.unwrap_or(8080);
    let mcp_url = ensure_default_mcp_url(api_port);

    codex = codex
        .json(true)
        .output_schema("/opt/robot-farm/schema.json")
        .config_override("mcp_servers.robot_farm.enabled=true")
        .config_override("mcp_servers.robot_farm.tool_timeout_sec=900")
        .config_override(format!("mcp_servers.robot_farm.url=\"{mcp_url}\""))
        .config_override(format!(
            "mcp_servers.robot_farm.http_headers={{\"Agent\"=\"{agent_label}\"}}"
        ))
        .config_override(format!("model=\"{}\"", launch_settings.model))
        .config_override(format!(
            "model_reasoning_effort=\"{}\"",
            launch_settings.reasoning
        ));

    let project_root = PathBuf::from(PROJECT_DIR.as_str());
    let (workspace_host, workspace_container) = match persona {
        Persona::Orchestrator => (project_root.join("staging"), PathBuf::from("/workspace")),
        Persona::Worker(id) => (
            project_root.join(format!("ws{id}")),
            PathBuf::from("/workspace"),
        ),
    };

    let docker_home = PathBuf::from("/home/codex");
    let docker_codex_home = docker_home.join(".codex");
    let host_codex_home = resolve_codex_home();

    let image = match persona {
        Persona::Orchestrator => DOCKER_IMAGE_ORCHESTRATOR.as_str(),
        Persona::Worker(_) => DOCKER_IMAGE_WORKER.as_str(),
    };

    let docker = DockerRunBuilder::new(image)
        .remove_container(true)
        .interactive(true)
        .attach("STDOUT")
        .attach("STDERR")
        .user("1000:1000")
        .workdir(&workspace_container)
        .volume(workspace_host, &workspace_container, Some("rw".into()))
        .volume(&host_codex_home, &docker_codex_home, Some("rw".into()))
        .env("CODEX_NO_COLOR", "1")
        .env("HOME", docker_home.display().to_string())
        .env("CODEX_HOME", docker_codex_home.display().to_string())
        .env("PWD", workspace_container.display().to_string());

    CommandPlan {
        docker_args: docker.build(),
        codex_args: codex.change_dir("/workspace").build(),
    }
}

fn resolve_codex_home() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".codex")
}
