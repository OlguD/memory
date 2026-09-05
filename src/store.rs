use rusqlite::Connection;
use std::path::Path;
use crate::model::Project;
use crate::model::NewMemory;
use crate::model::Memory;
use crate::model::Kind;
use crate::model::Source;
use rusqlite::OptionalExtension;

#[derive(Debug)]
pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open(path: &Path) -> rusqlite::Result<Db> {
        let schema = include_str!("schema.sql");
        let conn = Connection::open(path)?;
        conn.execute_batch(schema)?;
        Ok(Db {conn}) 
    }

    pub fn insert_project(&self, name: &str, path: &str) -> rusqlite::Result<i64> {
        self.conn.execute("INSERT INTO projects (name, path) VALUES (?1, ?2)", (name, path))?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn find_project_by_path(&self, path: &str) -> rusqlite::Result<Option<Project>> {
        let project = self.conn.query_row(
            "SELECT id, name, path, created_at FROM projects WHERE path = ?1",
            [path],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    created_at: row.get(3)?,
                })
            }
        ).optional()?;
        Ok(project)
    }

    pub fn insert_memory(&self, new: NewMemory) -> rusqlite::Result<i64> { 
        self.conn.execute("INSERT INTO memories (project_id, source, source_ref, kind, content) VALUES (?1, ?2, ?3, ?4, ?5)", (new.project_id, new.source.as_str(), new.source_ref, new.kind.as_str(), new.content))?;
        Ok(self.conn.last_insert_rowid())
    }

    
    pub fn list_memories(&self, project_id: i64) -> rusqlite::Result<Vec<Memory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, source, source_ref, kind, content, created_at, updated_at
             FROM memories WHERE project_id = ?1 ORDER BY created_at DESC"
            )?;

        let rows = stmt.query_map([project_id], |row| {
            let kind_str: String = row.get(4)?;
            let kind = Kind::from_str(&kind_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;

            let source_str: String = row.get(2)?;
            let source = Source::from_str(&source_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;

            Ok(Memory {
                id: row.get(0)?,
                project_id: row.get(1)?,
                source,
                source_ref: row.get(3)?,
                kind,
                content: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        rows.collect()
    }

    pub fn search_memories(&self, project_id: i64, query: &str) -> rusqlite::Result<Vec<Memory>> {
        let mut stmt = self.conn.prepare(
            "SELECT m.id, m.project_id, m.source, m.source_ref, m.kind,
             m.content, m.created_at, m.updated_at
             FROM memories m
             JOIN memories_fts f ON f.rowid = m.id
             WHERE f.memories_fts MATCH ?1 AND m.project_id = ?2
             ORDER BY f.rank"
            )?;

        let rows = stmt.query_map((query, project_id), |row| {
            let kind_str: String = row.get(4)?;
            let kind = Kind::from_str(&kind_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;

            let source_str: String = row.get(2)?;
            let source = Source::from_str(&source_str)
                .ok_or(rusqlite::Error::InvalidQuery)?;

            Ok(Memory {
                id: row.get(0)?,
                project_id: row.get(1)?,
                source,
                source_ref: row.get(3)?,
                kind,
                content: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        rows.collect()
    }
}


