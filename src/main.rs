use model::{NewMemory, Source, Kind};
use store::Db;
use std::path::Path;

mod store;
mod model;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let new_memory = NewMemory {
        source: Source::Manual,
        source_ref: None,
        kind: Kind::Note,
        content: String::from("Yeni content"),
        project_id: 1
    };
    
    let path = "memory/memory/data";
    let db = Db::open(Path::new("memory.db"))?;


    println!("{:#?}", new_memory);
    new_memory.source.as_str();
    println!("{:?}", Source::from_str("git"));
    println!("{:?}", Source::from_str("xyz"));

    println!("{:?}", db);

    let name = String::from("memory_project_name"); 
    let insert_result = db.insert_project(&name, &path);
    println!("{:?}", insert_result);


    let project = db.find_project_by_path(&path);
    println!("{:?}", project);

    let wrong_path = String::from("wrong_path");
    let wrong_project = db.find_project_by_path(&wrong_path);
    println!("{:?}", wrong_project);

    Ok(())
}
