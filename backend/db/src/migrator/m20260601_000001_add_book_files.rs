use sea_orm_migration::{prelude::*, schema::*, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add columns to books table
        if !manager.has_column("books", "book_path").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Books::Table)
                        .add_column(string(Books::BookPath))
                        .to_owned(),
                )
                .await?;
        }

        if !manager.has_column("books", "file_hash").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Books::Table)
                        .add_column(string_len(Books::FileHash, 64))
                        .to_owned(),
                )
                .await?;
        }

        if !manager.has_column("books", "size_bytes").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Books::Table)
                        .add_column(big_integer(Books::SizeBytes))
                        .to_owned(),
                )
                .await?;
        }

        // Add is_primary to authors_books
        if !manager.has_column("authors_books", "is_primary").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(AuthorsBooks::Table)
                        .add_column(
                            ColumnDef::new(AuthorsBooks::IsPrimary)
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .to_owned(),
                )
                .await?;
        }

        // Create book_files table
        manager
            .create_table(
                Table::create()
                    .table(BookFiles::Table)
                    .if_not_exists()
                    .col(pk_uuid(BookFiles::Id))
                    .col(uuid(BookFiles::BookId).not_null())
                    .col(string_len(BookFiles::Medium, 20).not_null())
                    .col(string(BookFiles::FilePath).not_null())
                    .col(string_len(BookFiles::Format, 10))
                    .col(string_len(BookFiles::FileHash, 64))
                    .col(big_integer(BookFiles::SizeBytes))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_files_book")
                            .from(BookFiles::Table, BookFiles::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookFiles::Table).to_owned())
            .await?;

        if manager.has_column("authors_books", "is_primary").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(AuthorsBooks::Table)
                        .drop_column(AuthorsBooks::IsPrimary)
                        .to_owned(),
                )
                .await?;
        }

        for col in [Books::SizeBytes, Books::FileHash, Books::BookPath] {
            if manager.has_column("books", &col.to_string()).await? {
                manager
                    .alter_table(
                        Table::alter()
                            .table(Books::Table)
                            .drop_column(col)
                            .to_owned(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    BookPath,
    FileHash,
    SizeBytes,
}

#[derive(DeriveIden)]
enum AuthorsBooks {
    Table,
    IsPrimary,
}

#[derive(DeriveIden)]
enum BookFiles {
    Table,
    Id,
    BookId,
    Medium,
    FilePath,
    Format,
    FileHash,
    SizeBytes,
}
