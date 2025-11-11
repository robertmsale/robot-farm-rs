mod git;
mod healthz;
mod message_queue;
mod strategy;
mod task;
mod task_dependency;
mod task_group;
mod worker;
mod ws;

use axum::{
    Router,
    http::Method,
    routing::{delete, get, patch},
};
use tower_http::cors::{Any, CorsLayer};

pub fn build_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/healthz", get(healthz::healthz_handler))
        .route("/ws", get(ws::websocket_handler))
        .route("/tasks", get(task::list_tasks).post(task::create_task))
        .route(
            "/tasks/:taskId",
            get(task::get_task)
                .put(task::update_task)
                .delete(task::delete_task),
        )
        .route("/tasks/:taskId/commit", get(git::get_task_commit_info))
        .route("/tasks/:taskId/commit/diff", get(git::get_task_commit_diff))
        .route(
            "/task-groups",
            get(task_group::list_task_groups).post(task_group::create_task_group),
        )
        .route(
            "/task-groups/:taskGroupId",
            get(task_group::get_task_group)
                .put(task_group::update_task_group)
                .delete(task_group::delete_task_group),
        )
        .route(
            "/task-deps",
            get(task_dependency::list_task_dependencies)
                .post(task_dependency::create_task_dependency),
        )
        .route(
            "/task-deps/:taskId/:dependsOnTaskId",
            delete(task_dependency::delete_task_dependency),
        )
        .route(
            "/message_queue",
            get(message_queue::list_messages).delete(message_queue::delete_all_messages),
        )
        .route(
            "/message_queue/:messageId",
            delete(message_queue::delete_message_by_id),
        )
        .route(
            "/message_queue/:messageId/insert",
            patch(message_queue::insert_message_relative),
        )
        .route(
            "/message_queue/to/:sender",
            delete(message_queue::delete_messages_for_recipient),
        )
        .route(
            "/workers",
            get(worker::list_workers).post(worker::create_worker),
        )
        .route("/workers/:workerId", delete(worker::delete_worker))
        .route(
            "/strategy",
            get(strategy::get_active_strategy).put(strategy::update_active_strategy),
        )
        .layer(cors)
}
