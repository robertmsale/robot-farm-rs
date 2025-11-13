# Robot Farm

## About

This project is a rust-based AI development system that aims to enable multiple `codex exec` instances, running in non-interactive mode, to collaborate on a software project in parallel via their own git worktree.

## Key Structure

- `server/`: Axum-based API server. Routes live in `server/src/routes`, and each domain (tasks, task groups, workers, message queue, feed, config, git) has its own module. Shared functionality (Git helpers, shell runners, docker command builders, Codex/Docker CLI builders) is placed in `server/src/shared`.
- `server/src/db/`: Database abstraction layer. Each module mirrors the routes (tasks, task_groups, etc.) and is responsible for SQLx CRUD logic. `db::ensure_db()` creates `<workspace>/.robot-farm/db/robotfarm.db` and runs SQLite migrations embedded from `server/migrations`.
- `server/migrations/`: SQLx migration files. They are bundled via `sqlx::migrate!` so the server binary can apply them automatically at startup.
- `openapi/`: Source of truth for API schemas. `openapi/openapi.json` defines the API, and running `scripts/generate_openapi_clients.sh` regenerates both the Rust (`openapi/rust`) and Flutter (`openapi/flutter_client`) clients.
- Workspace layout: the server may receive `--workspace <path>`; it changes CWD to that path. Configuration files live under `<workspace>/.robot-farm/config.json`. The active Git repository/worktrees are expected under `<workspace>/staging/` (non-bare repo) with additional worktrees as needed.
- `server/src/system/` owns in-memory coordination state. `strategy.rs` keeps the active strategy (`Planning` by default) in a global `RwLock`, and routes read/write through it. `queue.rs` exposes the `QueueCoordinator`, which tracks worker assignments, registered workers, pending system events, and can synthesize `OrchestratorHint`s plus validation/merge results. `events.rs` defines the serialized payload for system-generated feed entries so Codex JSONL and system events can share the same feed table. `runner.rs` combines Docker & Codex builders so the process manager can spawn the right persona, and the `session` table (SQLite) stores the current Codex thread id for `orchestrator` or any `ws#`, so session clears/retries survive restarts.
- Flutter state is GetX-based. Global app state (`ConnectionController`) is registered once with `Get.put(... permanent: true)` at bootstrap. Feature screens (tasks, settings, task wizard, worker feed, etc.) define controllers per-route or per-widget using bindings/get-builders so each navigation stack owns its controller lifecycle; worker feeds now instantiate their controller via `GetBuilder(init: …, global: false)` to ensure `TabController`s are disposed safely after hot restarts/logouts.
- Codex session management lives in SQLite rather than `.robot-farm` JSON. The `session` table keeps a single `thread_id` per owner (`orchestrator`, `ws#`). Routes call `db::session::delete_session` to clear state and we can later use `upsert_session`/`get_session` when wiring `codex exec ... resume {id}`. Prompts are passed via stdin (instead of CLI args) to `codex exec` so we never hit OS argument-length limits even when running inside Docker.

## Runtime Coordination

- `system::init_system_state()` runs at startup (after DB init) to seed the strategy state and queue coordinator. The server always boots in `Planning` mode until another strategy is persisted via the `/strategy` endpoints.
- System events are stored as `SystemEvent` structs (level/source/target/category/payload). The queue coordinator can record events for strategy nudges, user-submitted messages, validation failures, etc. Future feed writers can convert these events directly into feed rows without changing the Codex JSONL schema.
- Worker-task assignments live in the queue coordinator. Helpers ensure a worker only has one active task, keep track of idle workers, and compute orchestrator hints based on the current strategy focus. Middleware/process-manager hooks (COMPLETE_TASK interception, merge validation) should call into this coordinator to add events and release assignments.
- User-directed system messages can be funneled through the same coordinator, which records them as `SystemEventCategory::User` entries destined for the requested feed target.

## Git Helpers

- `shared::git` wraps `gix` and shell commands to get commit info, per-file diffs, perform merges (regular and `--ff-only`), and collect merge conflicts. All helpers accept a repo root path (e.g., `<workspace>/staging`), making them work with both the main repo and worktrees.
- Startup calls `shared::git::ensure_non-bare_repo` to panic early if `<workspace>/staging` is missing or bare.
- `/tasks/{id}/commit` and `/tasks/{id}/commit/diff` use the shared git helpers and always read from `<workspace>/staging`.
- `/orchestrator/exec` runs `bash -lc` inside `<workspace>/staging`, while `/workers/{id}/exec` runs commands in `<workspace>/ws{id}`; both endpoints reuse `shared::shell::run_shell_command`.
- `shared::docker::DockerRunBuilder` assembles `docker run` invocations with support for `--rm`, `-a`, `-u`, `-v`, `-e`, `--workdir`, and custom commands. Use it whenever spinoff containers are required.

## Configuration & DB

- `routes::config` contains on-disk config helpers. `ensure_config_exists()` seeds `<workspace>/.robot-farm/config.json` with defaults on first launch.
- `db::ensure_db()` runs after config initialization to guarantee the SQLite file exists and migrations are up to date.
