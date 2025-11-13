CREATE TABLE IF NOT EXISTS session (
    owner TEXT PRIMARY KEY,
    thread_id TEXT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS session_updated_at_idx ON session (updated_at DESC);
