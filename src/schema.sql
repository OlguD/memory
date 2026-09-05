CREATE TABLE IF NOT EXISTS projects (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	path TEXT UNIQUE,
	created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS memories (
	id INTEGER PRIMARY KEY,
	project_id INTEGER NOT NULL REFERENCES projects(id),
	source TEXT NOT NULL CHECK (source IN ('manual', 'git')),
       	source_ref TEXT,
	kind TEXT NOT NULL CHECK (kind IN ('decision', 'pattern', 'bug', 'note')),
	content TEXT NOT NULL,
	created_at TEXT DEFAULT (datetime('now')),
	updated_at TEXT DEFAULT (datetime('now'))
);

CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5 (
	content,
	content='memories',
	content_rowid='id'
);

CREATE TRIGGER IF NOT EXISTS memories_ai
AFTER INSERT ON memories
BEGIN
	INSERT INTO memories_fts(rowid, content)
	VALUES (new.id, new.content);
END;

CREATE TRIGGER IF NOT EXISTS memories_ad
AFTER DELETE ON memories
BEGIN 
	INSERT INTO memories_fts(memories_fts, rowid, content)
	VALUES ('delete', old.id, old.content);
END;

CREATE TRIGGER IF NOT EXISTS memories_au
AFTER UPDATE ON memories
BEGIN 
	INSERT INTO memories_fts(memories_fts, rowid, content)
	VALUES ('delete', old.id, old.content);

	INSERT INTO memories_fts(rowid, content)
	VALUES (new.id, new.content);
END;
