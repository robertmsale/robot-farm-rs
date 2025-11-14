-- Feed entries capture system-wide activity.
CREATE TABLE IF NOT EXISTS feed (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source TEXT NOT NULL,
    target TEXT NOT NULL,
    ts BIGINT NOT NULL,
    level TEXT NOT NULL,
    text TEXT NOT NULL,
    raw TEXT NOT NULL,
    category TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS feed_ts_idx ON feed (ts DESC);

-- Message queue stores agent-to-agent communications enqueued for delivery.
CREATE TABLE IF NOT EXISTS message_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_actor TEXT NOT NULL,
    to_actor TEXT NOT NULL,
    message TEXT NOT NULL,
    inserted_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS message_queue_inserted_at_idx ON message_queue (inserted_at DESC);

-- Task groups provide higher-level organization.
CREATE TABLE IF NOT EXISTS task_group (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL
);

-- Tasks track individual units of work.
CREATE TABLE IF NOT EXISTS task (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL REFERENCES task_group(id) ON DELETE CASCADE,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    commit_hash TEXT,
    status TEXT NOT NULL,
    owner TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS task_group_id_idx ON task (group_id);

-- Task dependencies capture prerequisite relationships between tasks.
CREATE TABLE IF NOT EXISTS task_deps (
    task_id INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    depends_on_task_id INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id),
    CHECK (task_id <> depends_on_task_id)
);
