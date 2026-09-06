use std::env;
use crate::model::Project;
use crate::store::Db;


pub fn resolve(db: &Db, override_name: Option<&str>) -> rusqlite::Result<Option<Project>> {
    if let Some(name) = override_name {
        return db.find_project_by_name(name);
    }

    let cwd = env::current_dir().expect("cwd okunamadi");
    for dir in cwd.ancestors() {
        if let Some(s) = dir.to_str() {
            if let Some(p) = db.find_project_by_path(s)? {
                return Ok(Some(p));        
            }
        }
    }

    Ok(None)
}
