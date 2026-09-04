use model::{NewMemory, Source, Kind};

mod model;

fn main() {
    let new_memory = NewMemory {
        source: Source::Manual,
        source_ref: None,
        kind: Kind::Note,
        content: String::from("Yeni content"),
        project_id: 1
    };

    println!("{:#?}", new_memory);
    new_memory.source.as_str();
    println!("{:?}", Source::from_str("git"));
    println!("{:?}", Source::from_str("xyz"));
}
