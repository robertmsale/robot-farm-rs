PRAGMA foreign_keys=OFF;

-- Drop any legacy views that reference task/task_deps so schema rebuild can proceed.
DROP VIEW IF EXISTS task_assignment_summary;
DROP VIEW IF EXISTS assignment;

ALTER TABLE task_deps RENAME TO task_deps_old;
ALTER TABLE task RENAME TO task_old;

CREATE TABLE task (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL REFERENCES task_group(id) ON DELETE CASCADE,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    commit_hash TEXT,
    status TEXT NOT NULL,
    owner TEXT NOT NULL COLLATE NOCASE,
    description TEXT NOT NULL
);

INSERT INTO task (id, group_id, slug, title, commit_hash, status, owner, description)
SELECT id, group_id, slug, title, commit_hash, status, owner, description
FROM task_old;

CREATE INDEX IF NOT EXISTS task_group_id_idx ON task (group_id);

CREATE TABLE task_deps (
    task_id INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    depends_on_task_id INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id),
    CHECK (task_id <> depends_on_task_id)
);

INSERT INTO task_deps (task_id, depends_on_task_id)
SELECT task_id, depends_on_task_id FROM task_deps_old;

DROP TABLE task_old;
DROP TABLE task_deps_old;

PRAGMA foreign_keys=ON;
