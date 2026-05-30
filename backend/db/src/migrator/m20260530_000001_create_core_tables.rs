use sea_orm_migration::schema::*;
use sea_orm_migration::{prelude::*, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(pk_uuid(Books::Id))
                    .col(string_len(Books::Title, 512).not_null())
                    .col(string(Books::Subtitle))
                    .col(text(Books::Description))
                    .col(string_len(Books::Language, 10).not_null().default("en"))
                    .col(string(Books::Publisher))
                    .col(string(Books::Isbn))
                    .col(integer(Books::PageCount))
                    .col(string(Books::CoverPath))
                    .col(date(Books::PublishedDate))
                    .col(timestamp(Books::CreatedAt))
                    .col(timestamp(Books::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // sort_title generated column (strips leading articles for natural sorting)
        manager
            .alter_table(
                Table::alter()
                    .table(Books::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("sort_title"))
                            .text()
                            .not_null()
                            .extra(
                                "GENERATED ALWAYS AS \
                                 (regexp_replace(title, '^(The |A |An )', '', 'i')) STORED",
                            ),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Authors::Table)
                    .if_not_exists()
                    .col(pk_uuid(Authors::Id))
                    .col(string(Authors::Name).not_null())
                    .col(string(Authors::SortName).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthorsBooks::Table)
                    .if_not_exists()
                    .col(uuid(AuthorsBooks::BookId).not_null())
                    .col(uuid(AuthorsBooks::AuthorId).not_null())
                    .col(integer(AuthorsBooks::SortOrder).not_null().default(0))
                    .primary_key(
                        Index::create()
                            .col(AuthorsBooks::BookId)
                            .col(AuthorsBooks::AuthorId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_authors_books_book")
                            .from(AuthorsBooks::Table, AuthorsBooks::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_authors_books_author")
                            .from(AuthorsBooks::Table, AuthorsBooks::AuthorId)
                            .to(Authors::Table, Authors::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Series::Table)
                    .if_not_exists()
                    .col(pk_uuid(Series::Id))
                    .col(string(Series::Name).not_null().unique_key())
                    .col(text(Series::Description))
                    .col(timestamp(Series::CreatedAt))
                    .col(timestamp(Series::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SeriesBooks::Table)
                    .if_not_exists()
                    .col(uuid(SeriesBooks::BookId).not_null())
                    .col(uuid(SeriesBooks::SeriesId).not_null())
                    .col(string(SeriesBooks::Position))
                    .col(integer(SeriesBooks::SortOrder).not_null().default(0))
                    .primary_key(
                        Index::create()
                            .col(SeriesBooks::BookId)
                            .col(SeriesBooks::SeriesId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_series_books_book")
                            .from(SeriesBooks::Table, SeriesBooks::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_series_books_series")
                            .from(SeriesBooks::Table, SeriesBooks::SeriesId)
                            .to(Series::Table, Series::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Identifiers::Table)
                    .if_not_exists()
                    .col(pk_uuid(Identifiers::Id))
                    .col(uuid(Identifiers::BookId).not_null())
                    .col(string(Identifiers::Source).not_null())
                    .col(string(Identifiers::Value).not_null())
                    .index(
                        Index::create()
                            .unique()
                            .col(Identifiers::Source)
                            .col(Identifiers::Value),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_identifiers_book")
                            .from(Identifiers::Table, Identifiers::BookId)
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
            .drop_table(Table::drop().table(Identifiers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SeriesBooks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Series::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AuthorsBooks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Authors::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    Title,
    Subtitle,
    Description,
    Language,
    Publisher,
    Isbn,
    PageCount,
    CoverPath,
    PublishedDate,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Authors {
    Table,
    Id,
    Name,
    SortName,
}

#[derive(DeriveIden)]
enum Series {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AuthorsBooks {
    Table,
    BookId,
    AuthorId,
    SortOrder,
}

#[derive(DeriveIden)]
enum SeriesBooks {
    Table,
    BookId,
    SeriesId,
    Position,
    SortOrder,
}

#[derive(DeriveIden)]
enum Identifiers {
    Table,
    Id,
    BookId,
    Source,
    Value,
}
