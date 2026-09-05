use model::{NewMemory, Source, Kind};
use store::Db;
use std::path::Path;

mod store;
mod model;

fn main() {
    let new_memory = NewMemory {
        source: Source::Manual,
        source_ref: None,
        kind: Kind::Note,
        content: String::from("Yeni content"),
        project_id: 1
    };
    
    let path = "memory/memory/data";
    let db = Db::open(Path::new("memory.db"));


    println!("{:#?}", new_memory);
    new_memory.source.as_str();
    println!("{:?}", Source::from_str("git"));
    println!("{:?}", Source::from_str("xyz"));

    println!("{:?}", db);
}
