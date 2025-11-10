use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct Config {
    #[validate]
    pub append_agents_file: AppendFilesConfig,

    #[validate(length(min = 1))]
    #[serde(default)]
    pub commands: Vec<CommandConfig>,

    #[validate(length(min = 1))]
    #[serde(default)]
    pub post_turn_checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct AppendFilesConfig {
    #[validate(length(min = 1))]
    pub orchestrator: Vec<String>,

    #[validate(length(min = 1))]
    pub worker: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct CommandConfig {
    #[validate(length(min = 1))]
    pub id: String,

    #[validate(length(min = 1))]
    pub exec: Vec<String>,

    #[serde(default)]
    pub stdout_success_message: Option<String>,

    #[serde(default)]
    pub hidden: bool,

    #[serde(default)]
    pub timeout_seconds: Option<u64>,
}
