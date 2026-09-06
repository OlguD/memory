use clap::{Parser, Subcommand};
use crate::store::Db;
use crate::model::{Kind, NewMemory, Source};
use crate::project;

#[derive(Parser)]
#[command(name = "memory")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init {
        #[arg(long)]
        name: Option<String>,
    },
    Add {
        #[arg(long)]
        kind: Kind,
        #[arg(long)]
        project: Option<String>,
        content: String,
    },
    List {
        #[arg(long)]
        project: Option<String>
    },
    Search {
        query: String,
        #[arg(long)]
        project: Option<String>
    },
    Update {
        id: i64,
        content: String
    },
    Delete {
        id: i64,
    },
}

pub fn run () -> rusqlite::Result<()> {
    let cli = Cli::parse();
    let home = dirs::home_dir().expect("home dizini bulunamadi");
    let db_path = home.join(".projectmemory").join("memory.db");

    std::fs::create_dir_all(home.join(".projectmemory")).expect("klasor olusturulamadi");
    let db = Db::open(&db_path)?;


    match cli.command {
        Command::Init {name} => {
            let cwd = std::env::current_dir().expect("cwd okunamadi");
            let name = name.unwrap_or_else(|| {
                cwd.file_name().unwrap().to_string_lossy().to_string()
            });
            let path = cwd.to_string_lossy();
            db.insert_project(&name, &path)?;
            println!("Initialized completed");
        } 
        Command::Add {kind, project, content} => {
            let found = project::resolve(&db, project.as_deref())?;
            let Some(p) = found else {
                println!("Bu dizin bir projeye bagli degil. Once `memory init calistir`");
                return Ok(());
            };
            let new = NewMemory {
                project_id: p.id,
                source: Source::Manual,
                source_ref: None,
                kind,
                content,
            };
            let id = db.insert_memory(new)?;
            println!("Saved (#{id})");
        }
        Command::List {project} => {
            let found = project::resolve(&db, project.as_deref())?;
            let Some(p) = found else {
                println!("Bu dizin bir projeye bagli degil. Once `memory init` calistir");
                return Ok(());
            };
            let memories = db.list_memories(p.id)?;
            for memory in memories {
                println!("#{id} [{kind}] {content}", id = memory.id, kind =memory.kind.as_str(), content = memory.content);
            }
        }
        Command::Search {query, project} => {
            let found = project::resolve(&db, project.as_deref())?;
            let Some(p) = found else {
                println!("Bu dizin bir projeye bagli degil. Once `memory init` calistir");
                return Ok(());
            };
            let memories = db.search_memories(p.id, &query)?;
            for memory in memories {
                println!("${} [{}] {}", memory.id, memory.kind.as_str(), memory.content);
            }
        }
        Command::Update {id, content} => {
            if db.update_memory(id, &content)? {
                println!("Memory updated");
            } else {
                println!("Something went wrong while updating the memory");
            }
        }
        Command::Delete {id} => {
            if db.delete_memory(id)? {
                println!("Memory removed");
            } else {
                println!("Something went wrong while updating the memory");
            }
        }
    }

    Ok(())
}
