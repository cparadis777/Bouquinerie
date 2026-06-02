use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20260530_000001_create_core_tables;
mod m20260531_000001_add_fk_indexes;
mod m20260601_000001_add_book_files;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260530_000001_create_core_tables::Migration),
            Box::new(m20260531_000001_add_fk_indexes::Migration),
            Box::new(m20260601_000001_add_book_files::Migration),
        ]
    }
}
