mod cli;
mod model;
mod project;
mod store;

fn main() -> rusqlite::Result<()>{
    cli::run()
}
