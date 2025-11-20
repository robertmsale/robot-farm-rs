use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use openapi::models::{AppendFilesConfig, Config as WorkspaceConfig};
use thiserror::Error;
use tracing::debug;

use crate::{
    globals::PROJECT_DIR,
    mcp::project_commands::ProjectCommandRegistry,
    post_turn_checks::PostTurnCheckRegistry,
    routes::config::{self, ConfigError},
    shared::git,
    system::{codex_config, docker_overrides},
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
    docker_overrides::reset();
    remove_agent_overrides()?;
    Ok(())
}

fn apply_config(config: &WorkspaceConfig) -> Result<(), ConfigSyncError> {
    codex_config::validate_preferences(config.models.as_ref(), config.reasoning.as_ref())
        .map_err(ConfigSyncError::InvalidCodexSettings)?;
    codex_config::replace((*config.models).clone(), (*config.reasoning).clone());
    docker_overrides::replace((*config.docker_overrides).clone());
    ProjectCommandRegistry::global().replace(config.commands.clone());
    PostTurnCheckRegistry::global().replace(config.post_turn_checks.clone());
    regenerate_agent_overrides(&config.append_agents_file)?;
    Ok(())
}

fn regenerate_agent_overrides(config: &AppendFilesConfig) -> Result<(), ConfigSyncError> {
    for (worktree, role) in worktree_paths()? {
        write_override_for_worktree(&worktree, role, config)?;
    }
    Ok(())
}

fn write_override_for_worktree(
    worktree: &Path,
    role: WorktreeRole,
    config: &AppendFilesConfig,
) -> Result<(), ConfigSyncError> {
    let mut buffer = String::new();
    match role {
        WorktreeRole::Orchestrator => {
            append_role_directive(WorktreeRole::Orchestrator, &mut buffer)?;
            append_sections(worktree, &config.orchestrator, &mut buffer)?;
        }
        WorktreeRole::Worker => {
            append_role_directive(WorktreeRole::Worker, &mut buffer)?;
            append_sections(worktree, &config.worker, &mut buffer)?;
        }
    }

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

pub const WORKER_DIRECTIVE: &str = include_str!("../../directives/worker.md");
pub const ORCHESTRATOR_DIRECTIVE: &str = include_str!("../../directives/orchestrator.md");
fn append_role_directive(role: WorktreeRole, buffer: &mut String) -> Result<(), ConfigSyncError> {
    let data = match role {
        WorktreeRole::Orchestrator => ORCHESTRATOR_DIRECTIVE,
        WorktreeRole::Worker => WORKER_DIRECTIVE,
    };
    buffer.push_str(&data);
    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }
    buffer.push('\n');
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
    for (worktree, _) in worktree_paths()? {
        let file = worktree.join("AGENTS.override.md");
        if file.exists() {
            let _ = fs::remove_file(file);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WorktreeRole {
    Orchestrator,
    Worker,
}

fn worktree_paths() -> Result<Vec<(PathBuf, WorktreeRole)>, ConfigSyncError> {
    let project_root = Path::new(PROJECT_DIR.as_str());
    let canonical_root = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let staging = project_root.join("staging");
    let mut map: HashMap<PathBuf, WorktreeRole> = HashMap::new();
    if staging.exists() {
        if let Some(role) = classify_worktree(&staging, &canonical_root) {
            map.insert(staging.clone(), role);
        }
        for path in git::list_worktrees(&staging)? {
            if let Some(role) = classify_worktree(&path, &canonical_root) {
                map.insert(path, role);
            } else {
                debug!(
                    ?path,
                    "Skipping non-Robot Farm worktree for agent overrides"
                );
            }
        }
    }
    Ok(map.into_iter().collect())
}

fn classify_worktree(path: &Path, canonical_project_root: &Path) -> Option<WorktreeRole> {
    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let staging = canonical_project_root.join("staging");
    if canonical_path == staging {
        return Some(WorktreeRole::Orchestrator);
    }
    let Some(name) = canonical_path.file_name().and_then(|n| n.to_str()) else {
        return None;
    };
    if !name.starts_with("ws") {
        return None;
    }
    if !name[2..].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let is_child_of_workspace = canonical_path
        .parent()
        .map(|parent| parent == canonical_project_root)
        .unwrap_or(false);
    if is_child_of_workspace {
        Some(WorktreeRole::Worker)
    } else {
        None
    }
}
