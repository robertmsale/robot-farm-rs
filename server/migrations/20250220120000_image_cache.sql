CREATE TABLE IF NOT EXISTS image_cache (
    tag TEXT PRIMARY KEY,
    hash TEXT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS image_cache_updated_at_idx ON image_cache (updated_at DESC);
