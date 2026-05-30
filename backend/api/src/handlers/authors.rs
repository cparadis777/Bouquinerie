use axum::extract::State;
use axum::Json;
use db::entities::authors;
use db::state::AppState;
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::error::AppError;

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/authors",
    responses(
        (status = 200, description = "List all authors", body = Vec<authors::Model>)
    )
)]
pub async fn list_authors(
    State(state): State<AppState>,
) -> Result<Json<Vec<authors::Model>>, AppError> {
    let authors = authors::Entity::find()
        .order_by_asc(authors::Column::SortName)
        .all(&state.db)
        .await?;
    Ok(Json(authors))
}
