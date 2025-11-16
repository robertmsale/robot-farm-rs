pub mod config;
mod feed;
mod git;
mod healthz;
mod mcp;
mod message_queue;
mod orchestrator;
mod queue;
mod strategy;
mod task;
mod task_dependency;
mod task_group;
mod task_wizard;
mod worker;
mod ws;

use axum::{
    Router,
    http::Method,
    routing::{delete, get, patch, post},
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
        .route("/task-wizard/ws", get(task_wizard::websocket_handler))
        .route(
            "/config",
            get(config::get_config)
                .post(config::create_config)
                .put(config::update_config)
                .delete(config::delete_config),
        )
        .route("/tasks", get(task::list_tasks).post(task::create_task))
        .route(
            "/tasks/{taskId}",
            get(task::get_task)
                .put(task::update_task)
                .delete(task::delete_task),
        )
        .route("/tasks/{taskId}/commit", get(git::get_task_commit_info))
        .route(
            "/tasks/{taskId}/commit/diff",
            get(git::get_task_commit_diff),
        )
        .route("/git/status", get(git::get_git_status_summary))
        .route(
            "/git/status/{worktreeId}",
            get(git::get_git_status_for_worktree),
        )
        .route(
            "/task-groups",
            get(task_group::list_task_groups).post(task_group::create_task_group),
        )
        .route(
            "/task-groups/{taskGroupId}",
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
            "/task-deps/{taskId}/{dependsOnTaskId}",
            delete(task_dependency::delete_task_dependency),
        )
        .route(
            "/message_queue",
            get(message_queue::list_messages)
                .post(message_queue::enqueue_message)
                .delete(message_queue::delete_all_messages),
        )
        .route(
            "/message_queue/{messageId}",
            delete(message_queue::delete_message_by_id),
        )
        .route(
            "/message_queue/{messageId}/insert",
            patch(message_queue::insert_message_relative),
        )
        .route(
            "/message_queue/to/{sender}",
            delete(message_queue::delete_messages_for_recipient),
        )
        .route(
            "/queue",
            get(queue::get_queue_state).put(queue::update_queue_state),
        )
        .route("/feed", get(feed::list_feed).delete(feed::delete_feed))
        .route("/feed/{feedId}", get(feed::get_feed_entry))
        .route(
            "/workers",
            get(worker::list_workers).post(worker::create_worker),
        )
        .route("/workers/{workerId}", delete(worker::delete_worker))
        .route(
            "/workers/{workerId}/session",
            delete(worker::delete_worker_session),
        )
        .route(
            "/workers/{workerId}/exec",
            post(worker::exec_worker_command),
        )
        .route(
            "/strategy",
            get(strategy::get_active_strategy).put(strategy::update_active_strategy),
        )
        .route(
            "/orchestrator/session",
            delete(orchestrator::delete_orchestrator_session),
        )
        .route(
            "/orchestrator/exec",
            post(orchestrator::exec_orchestrator_command),
        )
        .route("/mcp", get(mcp::stream_mcp).post(mcp::handle_mcp_request))
        .layer(cors)
}
