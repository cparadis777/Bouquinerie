use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "book_files")]
#[schema(as = BookFile)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub book_id: Uuid,
    pub medium: String,
    pub file_path: String,
    pub format: Option<String>,
    pub file_hash: Option<String>,
    pub size_bytes: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Books,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
