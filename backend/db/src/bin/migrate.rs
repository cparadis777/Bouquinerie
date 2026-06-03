use std::process::exit;

use clap::{Parser, Subcommand};
use db::{init_database, migrator::Migrator};
use sea_orm_migration::MigratorTrait;

#[derive(Parser)]
#[command(name = "migrate", about = "Database migration CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Apply pending migrations
    Up,
    /// Rollback last migration
    Down,
    /// Rollback all then re-apply all
    Reset,
    /// Drop all tables then re-apply all
    Fresh,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = init_database(&database_url).await;

    let result = match cli.command {
        Command::Up => Migrator::up(&db, None).await,
        Command::Down => Migrator::down(&db, None).await,
        Command::Reset => Migrator::reset(&db).await,
        Command::Fresh => Migrator::fresh(&db).await,
    };

    if let Err(e) = result {
        eprintln!("Migration failed: {e}");
        exit(1);
    }
}
