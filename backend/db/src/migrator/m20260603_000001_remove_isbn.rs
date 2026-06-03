use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("books", "isbn").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Books::Table)
                        .drop_column("isbn")
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager.has_column("books", "isbn").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Books::Table)
                        .add_column(string_len(Books::Isbn, 64))
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Isbn,
}
