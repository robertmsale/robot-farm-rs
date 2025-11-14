-- Drop default so future inserts must provide description explicitly.
CREATE TABLE IF NOT EXISTS task_with_description (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL REFERENCES task_group(id) ON DELETE CASCADE,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    commit_hash TEXT,
    status TEXT NOT NULL,
    owner TEXT NOT NULL,
    description TEXT NOT NULL
);
INSERT INTO task_with_description (id, group_id, slug, title, commit_hash, status, owner, description)
SELECT id, group_id, slug, title, commit_hash, status, owner, description FROM task;
DROP TABLE task;
ALTER TABLE task_with_description RENAME TO task;
CREATE INDEX IF NOT EXISTS task_group_id_idx ON task (group_id);
