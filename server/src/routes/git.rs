use crate::{
    db,
    globals::PROJECT_DIR,
    shared::git::{self as shared_git, GitError, WorktreeStatus},
};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    routing::post,
};
use openapi::models::{
    CommitInfo, GitStatusFileChange, GitStatusHunk, GitStatusSummary, GitWorktreeStatus,
};
use serde::Deserialize;
use std::path::Path as FsPath;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct CommitRequest {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskCommitDiffQuery {
    pub file: String,
}

pub async fn get_task_commit_info(
    Path(task_id): Path<i64>,
) -> Result<Json<Option<CommitInfo>>, StatusCode> {
    let task = db::task::get_task(task_id).await.map_err(|err| {
        error!(?err, task_id, "failed to load task for commit info");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let Some(task) = task else {
        return Ok(Json(None));
    };

    let Some(commit_hash) = task.commit_hash.as_deref() else {
        return Ok(Json(None));
    };

    let repo_root = FsPath::new(PROJECT_DIR.as_str()).join("staging");
    match shared_git::get_commit_info(&repo_root, commit_hash) {
        Ok(info) => Ok(Json(Some(info))),
        Err(err) => {
            error!(?err, "failed to gather commit info");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_task_commit_diff(
    Path(task_id): Path<i64>,
    Query(query): Query<TaskCommitDiffQuery>,
) -> Result<(StatusCode, String), StatusCode> {
    let task = db::task::get_task(task_id).await.map_err(|err| {
        error!(?err, task_id, "failed to load task for commit diff");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let Some(task) = task else {
        return Err(StatusCode::NOT_FOUND);
    };

    let Some(commit_hash) = task.commit_hash.as_deref() else {
        return Err(StatusCode::NOT_FOUND);
    };

    let repo_root = FsPath::new(PROJECT_DIR.as_str()).join("staging");
    match shared_git::get_file_diff(&repo_root, commit_hash, &query.file) {
        Ok(diff) => Ok((StatusCode::OK, diff)),
        Err(err) => {
            error!(?err, "failed to generate file diff");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_git_status_summary() -> Result<Json<GitStatusSummary>, StatusCode> {
    let project_dir = FsPath::new(PROJECT_DIR.as_str());
    match shared_git::collect_all_worktree_statuses(project_dir, false) {
        Ok(worktrees) => {
            let models = worktrees.into_iter().map(worktree_to_model).collect();
            Ok(Json(GitStatusSummary { worktrees: models }))
        }
        Err(err) => {
            error!(?err, "failed to collect git status summary");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_git_status_for_worktree(
    Path(worktree_id): Path<String>,
) -> Result<Json<GitWorktreeStatus>, StatusCode> {
    let project_dir = FsPath::new(PROJECT_DIR.as_str());
    match shared_git::collect_worktree_status(project_dir, &worktree_id, true) {
        Ok(status) => Ok(Json(worktree_to_model(status))),
        Err(GitError::NotFound(_)) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            error!(?err, worktree_id, "failed to collect git status");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn fast_forward_all_worktrees() -> Result<StatusCode, StatusCode> {
    let project_dir = FsPath::new(PROJECT_DIR.as_str());
    match shared_git::fast_forward_all_worktrees(project_dir) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            error!(?err, "failed to fast-forward worktrees");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn commit_worktree(
    Path(worktree_id): Path<String>,
    Json(payload): Json<CommitRequest>,
) -> Result<StatusCode, StatusCode> {
    let project_dir = FsPath::new(PROJECT_DIR.as_str());
    let worktree = project_dir.join(&worktree_id);
    if !worktree.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    let message = payload.message.trim();
    if message.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if let Err(err) = shared_git::stage_all(&worktree) {
        error!(?err, worktree_id, "failed to stage all before commit");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    match shared_git::commit(&worktree, message) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(GitError::CommandFailure { .. }) => Err(StatusCode::BAD_REQUEST),
        Err(err) => {
            error!(?err, worktree_id, "failed to commit worktree");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn worktree_to_model(status: WorktreeStatus) -> GitWorktreeStatus {
    let files = status.files.into_iter().map(file_change_to_model).collect();

    GitWorktreeStatus {
        id: status.id,
        path: status.path.display().to_string(),
        branch: status.branch,
        upstream: status.upstream,
        ahead: status.ahead as i32,
        behind: status.behind as i32,
        is_dirty: status.is_dirty,
        files,
    }
}

fn file_change_to_model(change: shared_git::FileChange) -> GitStatusFileChange {
    GitStatusFileChange {
        path: change.path,
        old_path: change.old_path,
        status_code: change.status_code,
        additions: change.additions as i32,
        deletions: change.deletions as i32,
        hunks: change
            .hunks
            .map(|h| h.into_iter().map(hunk_to_model).collect()),
    }
}

fn hunk_to_model(hunk: shared_git::DiffHunk) -> GitStatusHunk {
    GitStatusHunk {
        header: hunk.header,
        lines: hunk.lines,
    }
}
