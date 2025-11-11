use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use openapi::models::CommitInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskCommitDiffQuery {
    pub file: String,
}

pub async fn get_task_commit_info(Path(task_id): Path<i64>) -> Json<Option<CommitInfo>> {
    // TODO: hydrate commit info from VCS.
    let info = CommitInfo {
        hash: format!("hash-{task_id:0>7}"),
        message: format!("Placeholder commit for task {task_id}"),
        diff: "1 file changed, 1 insertion(+), 0 deletions(-)".to_string(),
    };
    Json(Some(info))
}

pub async fn get_task_commit_diff(
    Path(task_id): Path<i64>,
    Query(query): Query<TaskCommitDiffQuery>,
) -> impl IntoResponse {
    // TODO: stream real diff output.
    let diff = format!(
        "diff --git a/{file} b/{file}\n+ // Diff for task {task_id}",
        file = query.file
    );
    (StatusCode::OK, diff)
}
