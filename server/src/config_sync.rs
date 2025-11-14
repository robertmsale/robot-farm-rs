use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use openapi::models::{AppendFilesConfig, Config as WorkspaceConfig};
use thiserror::Error;

use crate::{
    globals::PROJECT_DIR,
    mcp::project_commands::ProjectCommandRegistry,
    post_turn_checks::PostTurnCheckRegistry,
    routes::config::{self, ConfigError},
    shared::git,
    system::codex_config,
};

#[derive(Debug, Error)]
pub enum ConfigSyncError {
    #[error("config access failed: {0}")]
    Config(#[from] ConfigError),
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("git error: {0}")]
    Git(#[from] git::GitError),
    #[error("codex settings invalid: {0}")]
    InvalidCodexSettings(String),
}

pub fn reload_from_disk() -> Result<(), ConfigSyncError> {
    match config::load_config_from_disk() {
        Ok(config) => apply_config(&config),
        Err(ConfigError::NotFound(_)) => clear_state(),
        Err(err) => Err(ConfigSyncError::Config(err)),
    }
}

pub fn clear_state() -> Result<(), ConfigSyncError> {
    ProjectCommandRegistry::global().replace(Vec::new());
    PostTurnCheckRegistry::global().replace(Vec::new());
    codex_config::reset();
    remove_agent_overrides()?;
    Ok(())
}

fn apply_config(config: &WorkspaceConfig) -> Result<(), ConfigSyncError> {
    codex_config::validate_preferences(config.models.as_ref(), config.reasoning.as_ref())
        .map_err(ConfigSyncError::InvalidCodexSettings)?;
    codex_config::replace((*config.models).clone(), (*config.reasoning).clone());
    ProjectCommandRegistry::global().replace(config.commands.clone());
    PostTurnCheckRegistry::global().replace(config.post_turn_checks.clone());
    regenerate_agent_overrides(&config.append_agents_file)?;
    Ok(())
}

fn regenerate_agent_overrides(config: &AppendFilesConfig) -> Result<(), ConfigSyncError> {
    for worktree in worktree_paths()? {
        write_override_for_worktree(&worktree, config)?;
    }
    Ok(())
}

fn write_override_for_worktree(
    worktree: &Path,
    config: &AppendFilesConfig,
) -> Result<(), ConfigSyncError> {
    let mut buffer = String::new();
    append_sections(worktree, &config.orchestrator, &mut buffer)?;
    append_sections(worktree, &config.worker, &mut buffer)?;

    let file = worktree.join("AGENTS.override.md");
    if buffer.trim().is_empty() {
        if file.exists() {
            let _ = fs::remove_file(&file);
        }
        return Ok(());
    }

    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(file, buffer)?;
    Ok(())
}

fn append_sections(
    worktree: &Path,
    files: &[String],
    buffer: &mut String,
) -> Result<(), ConfigSyncError> {
    for relative in files {
        let path = resolve_path(worktree, relative);
        let data = fs::read_to_string(&path)?;
        buffer.push_str(&data);
        if !buffer.ends_with('\n') {
            buffer.push('\n');
        }
        buffer.push('\n');
    }
    Ok(())
}

fn resolve_path(root: &Path, raw: &str) -> PathBuf {
    let candidate = Path::new(raw);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    }
}

fn remove_agent_overrides() -> Result<(), ConfigSyncError> {
    for worktree in worktree_paths()? {
        let file = worktree.join("AGENTS.override.md");
        if file.exists() {
            let _ = fs::remove_file(file);
        }
    }
    Ok(())
}

fn worktree_paths() -> Result<Vec<PathBuf>, ConfigSyncError> {
    let project_root = Path::new(PROJECT_DIR.as_str());
    let staging = project_root.join("staging");
    let mut set: HashSet<PathBuf> = HashSet::new();
    if staging.exists() {
        set.insert(staging.clone());
        for path in git::list_worktrees(&staging)? {
            set.insert(path);
        }
    }
    Ok(set.into_iter().collect())
}
