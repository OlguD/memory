use rusqlite::Connection;
use std::path::Path;
use crate::model::Project;
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
}
