use crate::{db, globals::PROJECT_DIR, shared::git as shared_git};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::CommitInfo;
use serde::Deserialize;
use std::path::Path as FsPath;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct TaskCommitDiffQuery {
    pub file: String,
}

pub async fn get_task_commit_info(
    Path(task_id): Path<i64>,
) -> Result<Json<Option<CommitInfo>>, StatusCode> {
    let Some(task) = db::task::get_task(task_id).await else {
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
    let Some(task) = db::task::get_task(task_id).await else {
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
