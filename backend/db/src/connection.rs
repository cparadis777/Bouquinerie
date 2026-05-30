use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::info;

use crate::migrator::Migrator;

pub async fn init_database(url: &str) -> DatabaseConnection {
    info!("Connecting to database...");
    let db = Database::connect(url)
        .await
        .expect("Failed to connect to database");
    info!("Connected to database");
    db
}

pub async fn run_migrations(db: &DatabaseConnection) {
    info!("Running migrations...");
    Migrator::up(db, None)
        .await
        .expect("Failed to run migrations");
    info!("Migrations complete");
}
