use crate::{db, globals::PROJECT_DIR, shared::git};
use openapi::models::{Worker, WorkerState};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::warn;

pub async fn list_workers() -> Vec<Worker> {
    match discover_workers() {
        Ok(mut workers) => {
            for worker in &mut workers {
                let owner = format!("ws{}", worker.id);
                match db::session::get_session(&owner).await {
                    Ok(value) => {
                        worker.thread_id = value;
                    }
                    Err(err) => {
                        warn!(
                            ?err,
                            worker_id = worker.id,
                            "failed to load worker thread id"
                        );
                    }
                }
            }
            workers
        }
        Err(err) => {
            warn!(?err, "failed to discover worker worktrees");
            Vec::new()
        }
    }
}

pub async fn create_worker() -> Worker {
    warn!("worker creation via API is not supported; returning placeholder");
    Worker {
        id: -1,
        last_seen: 0,
        state: WorkerState::Ready,
        thread_id: None,
    }
}

pub async fn delete_worker(_worker_id: i64) -> bool {
    warn!("worker deletion via API is not supported; manual git cleanup required");
    false
}

fn discover_workers() -> Result<Vec<Worker>, WorkerDiscoveryError> {
    let project_dir = Path::new(PROJECT_DIR.as_str());
    let staging_dir = project_dir.join("staging");
    let worktree_paths = git::list_worktrees(&staging_dir)?;
    let canonical_worktrees: HashSet<PathBuf> = worktree_paths
        .into_iter()
        .filter_map(|path| fs::canonicalize(path).ok())
        .collect();

    let mut workers = Vec::new();
    for entry in fs::read_dir(project_dir)? {
        let entry = entry?;
        let name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        if !is_worker_dir(&name) {
            continue;
        }

        let worker_id = match parse_worker_id(&name) {
            Some(id) => id,
            None => continue,
        };

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        match fs::canonicalize(&path) {
            Ok(resolved) => {
                if canonical_worktrees.contains(&resolved) {
                    workers.push(Worker {
                        id: worker_id,
                        last_seen: 0,
                        state: WorkerState::Ready,
                        thread_id: None,
                    });
                }
            }
            Err(err) => {
                warn!(
                    ?err,
                    path = %path.display(),
                    "failed to canonicalize worker directory"
                );
            }
        }
    }

    workers.sort_by_key(|worker| worker.id);
    Ok(workers)
}

fn is_worker_dir(name: &str) -> bool {
    if !name.starts_with("ws") {
        return false;
    }
    let digits = &name[2..];
    !digits.is_empty() && digits.chars().all(|ch| ch.is_ascii_digit())
}

fn parse_worker_id(name: &str) -> Option<i64> {
    name.strip_prefix("ws")
        .and_then(|digits| digits.parse::<i64>().ok())
}

#[derive(Debug, Error)]
enum WorkerDiscoveryError {
    #[error("failed to read project directory: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Git(#[from] git::GitError),
}
