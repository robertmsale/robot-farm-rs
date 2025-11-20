use std::path::PathBuf;

use crate::shared::docker::ensure_default_mcp_url;

#[derive(Debug, Default, Clone)]
pub struct CodexExecBuilder {
    mode: ExecMode,
    prompt: Option<String>,
    command: Option<String>,
    session_id: Option<String>,
    model: Option<String>,
    sandbox_mode: Option<String>,
    profile: Option<String>,
    working_dir: Option<PathBuf>,
    output_schema: Option<PathBuf>,
    output_last_message: Option<PathBuf>,
    color: Option<String>,
    image_files: Vec<PathBuf>,
    features_enabled: Vec<String>,
    features_disabled: Vec<String>,
    config_overrides: Vec<String>,
    json: bool,
    full_auto: bool,
    skip_git_repo_check: bool,
    oss: bool,
    resume_last: bool,
    change_dir: Option<String>,
}

#[derive(Debug, Clone)]
enum ExecMode {
    Start,
    Resume,
}

impl Default for ExecMode {
    fn default() -> Self {
        ExecMode::Start
    }
}

impl CodexExecBuilder {
    pub fn new() -> Self {
        Self {
            mode: ExecMode::Start,
            ..Default::default()
        }
    }

    pub fn resume() -> Self {
        Self {
            mode: ExecMode::Resume,
            ..Default::default()
        }
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = Some(id.into());
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn sandbox(mut self, sandbox: impl Into<String>) -> Self {
        self.sandbox_mode = Some(sandbox.into());
        self
    }

    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    pub fn working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    pub fn change_dir(mut self, dir: impl Into<String>) -> Self {
        self.change_dir = Some(dir.into());
        self
    }

    pub fn output_schema(mut self, file: impl Into<PathBuf>) -> Self {
        self.output_schema = Some(file.into());
        self
    }

    pub fn output_last_message(mut self, file: impl Into<PathBuf>) -> Self {
        self.output_last_message = Some(file.into());
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn add_image(mut self, file: impl Into<PathBuf>) -> Self {
        self.image_files.push(file.into());
        self
    }

    pub fn enable_feature(mut self, feature: impl Into<String>) -> Self {
        self.features_enabled.push(feature.into());
        self
    }

    pub fn disable_feature(mut self, feature: impl Into<String>) -> Self {
        self.features_disabled.push(feature.into());
        self
    }

    pub fn config_override(mut self, key_value: impl Into<String>) -> Self {
        self.config_overrides.push(key_value.into());
        self
    }

    pub fn json(mut self, enabled: bool) -> Self {
        self.json = enabled;
        self
    }

    pub fn full_auto(mut self, enabled: bool) -> Self {
        self.full_auto = enabled;
        self
    }

    pub fn skip_git_repo_check(mut self, enabled: bool) -> Self {
        self.skip_git_repo_check = enabled;
        self
    }

    pub fn oss(mut self, enabled: bool) -> Self {
        self.oss = enabled;
        self
    }

    pub fn resume_last(mut self, enabled: bool) -> Self {
        self.resume_last = enabled;
        self
    }

    pub fn build(self) -> Vec<String> {
        let mut args = vec!["codex".to_string(), "exec".to_string()];
        let is_resume = matches!(self.mode, ExecMode::Resume);

        if let Some(change_dir) = self.change_dir {
            args.push("-C".to_string());
            args.push(change_dir);
        }

        for kv in self.config_overrides {
            args.push("-c".to_string());
            args.push(Self::quote_toml_if_needed(&kv));
        }

        for feature in self.features_enabled {
            args.push("--enable".to_string());
            args.push(feature);
        }

        for feature in self.features_disabled {
            args.push("--disable".to_string());
            args.push(feature);
        }

        for image in self.image_files {
            args.push("--image".to_string());
            args.push(image.display().to_string());
        }

        if let Some(model) = self.model {
            args.push("--model".to_string());
            args.push(model);
        }

        if let Some(sandbox) = self.sandbox_mode {
            args.push("--sandbox".to_string());
            args.push(sandbox);
        }

        if let Some(profile) = self.profile {
            args.push("--profile".to_string());
            args.push(profile);
        }

        if let Some(dir) = self.working_dir {
            args.push("--cd".to_string());
            args.push(dir.display().to_string());
        }

        if let Some(schema) = self.output_schema {
            args.push("--output-schema".to_string());
            args.push(schema.display().to_string());
        }

        if let Some(file) = self.output_last_message {
            args.push("--output-last-message".to_string());
            args.push(file.display().to_string());
        }

        if let Some(color) = self.color {
            args.push("--color".to_string());
            args.push(color);
        }

        if self.json {
            args.push("--json".to_string());
        }

        if self.full_auto {
            args.push("--full-auto".to_string());
        }

        if self.skip_git_repo_check {
            args.push("--skip-git-repo-check".to_string());
        }

        if self.oss {
            args.push("--oss".to_string());
        }

        if is_resume {
            args.push("resume".to_string());
        }

        match self.mode {
            ExecMode::Start => {
                if let Some(prompt) = self.prompt {
                    args.push(prompt);
                }

                if let Some(command) = self.command {
                    args.push(command);
                }
            }
            ExecMode::Resume => {
                if let Some(session_id) = self.session_id {
                    args.push(session_id);
                } else if self.resume_last {
                    args.push("--last".to_string());
                }
                if let Some(prompt) = self.prompt {
                    args.push(prompt);
                }
            }
        }

        args
    }

    fn quote_toml_if_needed(value: &str) -> String {
        let needs_quotes = value.contains('[') || value.contains('{');
        if needs_quotes && !(value.starts_with('\'') && value.ends_with('\'')) {
            format!("'{value}'")
        } else {
            value.to_string()
        }
    }
}

pub fn build_default_codex_exec_command(
    port: Option<u16>,
    resume_session: Option<&str>,
) -> Vec<String> {
    let port = port.unwrap_or(8080);
    let builder = match resume_session {
        Some(id) => CodexExecBuilder::resume().session_id(id.to_string()),
        None => CodexExecBuilder::new(),
    };

    let mcp_url = ensure_default_mcp_url(port);

    builder
        .change_dir("/workspace")
        .json(true)
        .output_schema("/opt/robot-farm/schema.json")
        .config_override("mcp_servers.robot_farm.enabled=true")
        .config_override("mcp_servers.robot_farm.tool_timeout_sec=900")
        .config_override(format!("mcp_servers.robot_farm.url=\"{mcp_url}\""))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_default_exec_start_command() {
        let args = build_default_codex_exec_command(Some(9000), None);
        assert_eq!(args[0], "codex");
        assert_eq!(args[1], "exec");
        assert!(args.contains(&"-C".to_string()));
        assert!(args.contains(&"/workspace".to_string()));
        assert!(args.contains(&"--json".to_string()));
        assert!(args.contains(&"--output-schema".to_string()));
        assert!(args.contains(&"/opt/robot-farm/schema.json".to_string()));
        assert!(args.contains(&"-c".to_string()));
        assert!(args.contains(&"mcp_servers.robot_farm.enabled=true".to_string()));
        assert!(args.contains(&"mcp_servers.robot_farm.tool_timeout_sec=900".to_string()));
        let expected = format!(
            "mcp_servers.robot_farm.url=\"{}\"",
            ensure_default_mcp_url(9000)
        );
        assert!(args.contains(&expected));
    }

    #[test]
    fn builds_resume_command_with_session() {
        let args = build_default_codex_exec_command(None, Some("session-123"));
        assert_eq!(args[0], "codex");
        assert_eq!(args[1], "exec");
        assert!(args.contains(&"resume".to_string()));
        assert!(args.last().unwrap().ends_with("session-123"));
    }

    #[test]
    fn resume_command_lists_change_dir_before_resume() {
        let args = CodexExecBuilder::resume()
            .change_dir("/workspace")
            .session_id("abc-123".to_string())
            .build();
        assert_eq!(args[0], "codex");
        assert_eq!(args[1], "exec");
        assert_eq!(args[2], "-C");
        assert_eq!(args[3], "/workspace");
        let resume_pos = args
            .iter()
            .position(|arg| arg == "resume")
            .expect("resume argument present");
        assert!(resume_pos > 3, "resume should appear after -C /workspace");
        assert_eq!(args.last().unwrap(), "abc-123");
    }
}
