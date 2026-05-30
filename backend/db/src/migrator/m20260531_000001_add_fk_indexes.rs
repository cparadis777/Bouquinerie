use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_authors_books_author_id")
                    .table(AuthorsBooks::Table)
                    .col(AuthorsBooks::AuthorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_series_books_series_id")
                    .table(SeriesBooks::Table)
                    .col(SeriesBooks::SeriesId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_identifiers_book_id")
                    .table(Identifiers::Table)
                    .col(Identifiers::BookId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_authors_books_author_id")
                    .table(AuthorsBooks::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_series_books_series_id")
                    .table(SeriesBooks::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_identifiers_book_id")
                    .table(Identifiers::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum AuthorsBooks {
    Table,
    AuthorId,
}

#[derive(DeriveIden)]
enum SeriesBooks {
    Table,
    SeriesId,
}

#[derive(DeriveIden)]
enum Identifiers {
    Table,
    BookId,
}
