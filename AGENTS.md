# Robot Farm

## About

This project is a rust-based AI development system that aims to enable multiple `codex exec` instances, running in non-interactive mode, to collaborate on a software project in parallel via their own git worktree.

## Key Structure

- `server/`: Axum-based API server. Routes live in `server/src/routes`, and each domain (tasks, task groups, workers, message queue, feed, config, git) has its own module. Shared functionality (Git helpers, shell runners, docker command builders, Codex/Docker CLI builders) is placed in `server/src/shared`.
- `server/src/db/`: Database abstraction layer. Each module mirrors the routes (tasks, task_groups, etc.) and is responsible for SQLx CRUD logic. `db::ensure_db()` creates `<workspace>/.robot-farm/db/robotfarm.db` and runs SQLite migrations embedded from `server/migrations`.
- `server/migrations/`: SQLx migration files. They are bundled via `sqlx::migrate!` so the server binary can apply them automatically at startup.
- `openapi/`: Source of truth for API schemas. `openapi/openapi.json` defines the API, and running `scripts/generate_openapi_clients.sh` regenerates both the Rust (`openapi/rust`) and Flutter (`openapi/flutter_client`) clients.
- Workspace layout: the server may receive `--workspace <path>`; it changes CWD to that path. Configuration files live under `<workspace>/.robot-farm/config.json`. The active Git repository/worktrees are expected under `<workspace>/staging/` (non-bare repo) with additional worktrees as needed.

## Git Helpers

- `shared::git` wraps `gix` and shell commands to get commit info, per-file diffs, perform merges (regular and `--ff-only`), and collect merge conflicts. All helpers accept a repo root path (e.g., `<workspace>/staging`), making them work with both the main repo and worktrees.
- Startup calls `shared::git::ensure_non-bare_repo` to panic early if `<workspace>/staging` is missing or bare.
- `/tasks/{id}/commit` and `/tasks/{id}/commit/diff` use the shared git helpers and always read from `<workspace>/staging`.
- `/orchestrator/exec` runs `bash -lc` inside `<workspace>/staging`, while `/workers/{id}/exec` runs commands in `<workspace>/ws{id}`; both endpoints reuse `shared::shell::run_shell_command`.
- `shared::docker::DockerRunBuilder` assembles `docker run` invocations with support for `--rm`, `-a`, `-u`, `-v`, `-e`, `--workdir`, and custom commands. Use it whenever spinoff containers are required.

## Configuration & DB

- `routes::config` contains on-disk config helpers. `ensure_config_exists()` seeds `<workspace>/.robot-farm/config.json` with defaults on first launch.
- `db::ensure_db()` runs after config initialization to guarantee the SQLite file exists and migrations are up to date.
