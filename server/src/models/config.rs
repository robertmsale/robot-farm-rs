use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::fs;
use validator::Validate;

pub const PROJECT_CONFIG_PATH: &str = "./";

#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema, Default)]
pub struct AppendFilesConfig {
    #[validate(length(min = 1))]
    pub orchestrator: Vec<String>,

    #[validate(length(min = 1))]
    pub worker: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema, Default)]
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

pub fn load_config_from_path(path: &str) -> Config {
    let raw = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read config file '{path}': {err}");
    });

    let config: Config = serde_json::from_str(&raw).unwrap_or_else(|err: SerdeError| {
        panic!("Failed to parse config file '{path}': {err}");
    });

    if let Err(err) = config.validate() {
        let schema = schema_for!(Config);
        let schema_json =
            serde_json::to_string_pretty(&schema).unwrap_or_else(|schema_err| {
                format!("(failed to render schema: {schema_err})")
            });

        panic!(
            "Config validation failed for '{path}': {err}\nExpected schema:\n{schema_json}"
        );
    }

    config
}
