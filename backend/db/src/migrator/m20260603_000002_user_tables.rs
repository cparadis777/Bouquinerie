use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    .col(string(Users::Username).not_null().unique_key())
                    .col(string(Users::Name).null())
                    .col(string(Users::Email).not_null().unique_key())
                    .col(string_len(Users::PasswordHash, 256).not_null())
                    .col(boolean(Users::IsAdmin).default(false).not_null())
                    .col(
                        timestamp(Users::CreatedAt)
                            .extra("DEFAULT NOW()")
                            .not_null(),
                    )
                    .col(
                        timestamp(Users::UpdatedAt)
                            .extra("DEFAULT NOW()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Name,
    Email,
    PasswordHash,
    IsAdmin,
    CreatedAt,
    UpdatedAt,
}
