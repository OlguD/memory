use rusqlite::Connection;
use std::path::Path;

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
}
