# 🌾 Robot Farm RS

Robot Farm is a containerized agent stack for building software at light speed. Multiple Codex instances (workers + orchestrator) hack away inside isolated git worktrees while the Axum server keeps CI, git merges, and automation in sync. 🧑‍🌾🤖

## 🧭 Quick Start

1. **Clone** this repo + your target repo inside a workspace (see "System Overview" for file system layout).
2. **Run the server**:
   ```bash
   cargo run -p server -- --workspace /path/to/workspace
   ```
   The server builds the shared Docker image, seeds the SQLite DB, and exposes the Axum API + websocket feed on `:8080`.
3. **Launch the Flutter client** (real-time feed + task management UI):
   ```bash
   cd client
   flutter run -d macos # (or your favorite target)
   ```
4. 📡 **Connect** the client to the server’s host:port, and you’ll see worker feeds, task groups, queue controls, and more.

## 🧱 System Overview

```
┌────────────────────────┐        ┌────────────────────────┐
│ Flutter Client (GetX)  │◄─WS───►│ Axum /ws (feed+workers)│
│  • Orchestrator feed   │        ├────────────────────────┤
│  • Worker feeds        │        │ REST API (tasks, git…) │
└──────────▲─────────────┘        └──────────┬─────────────┘
           │                                 │
           │ feed events                     │ REST
           │                                 ▼
    ┌─────────────┐    intents   ┌───────────────────┐
    │Queue Manager│─────────────►│Database Manager   │
    │(post-turns) │◄─────────────│(serialized SQLx)  │
    └─────┬───────┘              └───────────────────┘
          │
          │process directives    ┌──────────────────┐
          ▼                      │ Process Manager  │→ Docker/Codex
    ┌─────────────┐◄────────────►│ (spawns codex)   │
    │Middleware   │ (batching)   └──────────────────┘
    └─────────────┘
```
```
📁 <WORKSPACE_DIRECTORY>
├── 📁 .robot-farm-rs    <- Config location (auto-generated & seeded w/ defaults on startup)
├── 📁 directives        <- Optional storage area for AGENTS files
├── 📁 scripts           <- Scripts that live outside your project (typically for code validation, condensing command outputs, etc.)
├── 📁 staging           <- Orchestrator (non-bare) repository
├── 📁 ui-testing        <- Example non-participating worktree
├── 📁 ws1               <\
├── 📁 ws2               <-\
├── 📁 ws3               <--\__ Worker worktrees
├── 📁 ws4               <--/
├── 📁 ws5               <-/
└── 📁 ws6               </
```

### 💾 Database Manager
- Single-threaded broker for SQLx actions (message queue, feed entries). Other modules submit typed intents and await structured responses.
- Guarantees order (write → realtime broadcast) and keeps axum handlers clean of SQL boilerplate.

### 🔁 Middleware
- Buffers `ProcessIntent`s and reduces them to launch/kill directives. Maintains inflight state so priorities and cancel signals stick even under bursty loads.

### ⚙️ Process Manager
- Spawns dockerized Codex runs, streams stdio, enforces kills.
- Worker runs are parsed for `WorkerTurn` payloads; on exit the parsed turn (intent + completion data) is emitted as a notification back to the queue manager.

### 🧠 Queue Manager
- Releases worker assignments, runs configured `post_turn_checks`, auto-commits/merges worker worktrees, and records feed/validation events.
- Pipes every persisted feed entry into the realtime broadcaster (see below) so the UI stays up-to-date without polling.

### 📡 Realtime Feed
- `realtime::publish(FeedEntry)` pushes events into a global `broadcast::channel`.
- `/ws` streams worker snapshots + feed events to all connected clients; the Flutter app listens and updates orchestrator + worker tabs live.

### 🛠️ Configuration
Stored in WORKSPACE_DIRECTORY/.robot-farm-rs (editable from Flutter UI, with hot reload!)
```json
{ // Used to generate AGENTS.override.md
  "append_agents_file": {
    "orchestrator": [ "../directives/AGENTS.orchestrator.md"],
    "worker": ["../directives/AGENTS.compact.md"]
  },
  "models": {
    "orchestrator": "gpt-5.1-codex",
    "worker": "gpt-5.1-codex",
    "wizard": "gpt-5.1-codex"
  },
  "reasoning": {
    "orchestrator": "medium",
    "worker": "medium",
    "wizard": "medium"
  }, // Commands available for post_turn_checks and workers/orchestrator
  "commands": [
    {
      "id": "cargo-check",
      "exec": [ // Example command with example output sanitizer (potentially massive input token savings!)
        "bash",
        "-lc",
        "cargo check --workspace -q --color never --message-format json 2>/dev/null > >(../scripts/sanitize_cargo_check.ts)"
      ],
      "stdout_success_message": "Cargo workspace check reported no errors!", // Optional (exit 0) message instead of build warnings/info
      "hidden": true, // Optionally hide this tool from AIs 
      "timeout_seconds": 999 // Specify timeout (required)
    }
  ],
  "post_turn_checks": ["cargo-test"] // Run these at the end of a worker's turn, send results to worker on failure
}
```

## 🧩 Client Highlights

- **ConnectionController** keeps the HTTP + websocket connection alive and exposes a `feedEvents` stream other controllers subscribe to.
- **OrchestratorFeedController / WorkerFeedController** fetch initial feed snapshots (`/feed`) and merge websocket events to show a continuous log.
- **Task UI** supports create/edit/delete with guards (built-in groups like `chores`, `bugs`, `hotfix` can’t be deleted in the UI or API).
- **Task Wizard** a special Codex designed to import massive task groups and edit tasks using natural language

## 🛠️ Development Tips

- `cargo check` / `cargo fmt` before committing. Rust warnings are expected because many modules deliberate leave hooks for future extensions.
- Flutter: `flutter analyze` and `dart fix --apply` keep the client lint-free.
- Need extra tooling in Codex containers? Drop a Dockerfile fragment at `<WORKSPACE>/.robot-farm/Dockerfile`; the server folds it in when building the `robot-farm-orchestrator_*` & `robot-farm-worker_*` images.

## 🤝 Contributing

1. Fork + PR welcome! Please describe the subsystem you touched (queue/middleware/process/db) as they’re tightly coupled.
2. Run the integration smoke tests: `cargo test` (server) and `flutter test` (client) if you touched Dart.
3. Document behavior changes in `AGENTS.md` or this `README` so future Codex runs understand the architecture.

Happy farming! 🌱
