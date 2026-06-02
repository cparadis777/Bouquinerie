use std::collections::HashMap;

use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use domain::entities::{
    authors, authors::Model as Author, authors_books, books, books::Model as Book, identifiers,
    identifiers::Model as Identifier, series, series::Model as Series, series_books,
};

use super::{BookListParams, BookRepository};
use super::{BookListResult, RepositoryError};

pub struct DbBookRepository<'a, C: ConnectionTrait> {
    db: &'a C,
}

impl<'a, C: ConnectionTrait> DbBookRepository<'a, C> {
    pub fn new(db: &'a C) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a, C: ConnectionTrait + Send + Sync> BookRepository for DbBookRepository<'a, C> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Book>, RepositoryError> {
        Ok(books::Entity::find_by_id(id).one(self.db).await?)
    }

    async fn list(&self, params: BookListParams) -> Result<BookListResult, RepositoryError> {
        let mut query = books::Entity::find().order_by_asc(books::Column::SortTitle);

        if let Some(author_id) = params.author_id {
            let book_ids: Vec<Uuid> = authors_books::Entity::find()
                .filter(authors_books::Column::AuthorId.eq(author_id))
                .all(self.db)
                .await?
                .into_iter()
                .map(|ab| ab.book_id)
                .collect();
            query = query.filter(books::Column::Id.is_in(book_ids));
        }

        if let Some(series_id) = params.series_id {
            let book_ids: Vec<Uuid> = series_books::Entity::find()
                .filter(series_books::Column::SeriesId.eq(series_id))
                .all(self.db)
                .await?
                .into_iter()
                .map(|sb| sb.book_id)
                .collect();
            query = query.filter(books::Column::Id.is_in(book_ids));
        }

        let paginator = query.paginate(self.db, params.page_size);
        let items = paginator.fetch_page(params.page - 1).await?;
        let total = paginator.num_items().await?;

        let book_ids: Vec<Uuid> = items.iter().map(|b| b.id).collect();
        let author_names = if book_ids.is_empty() {
            HashMap::new()
        } else {
            authors_books::Entity::find()
                .filter(authors_books::Column::BookId.is_in(book_ids))
                .find_also_related(authors::Entity)
                .all(self.db)
                .await?
                .into_iter()
                .fold(
                    HashMap::new(),
                    |mut map: HashMap<Uuid, Vec<String>>, (ab, author_opt)| {
                        if let Some(author) = author_opt {
                            map.entry(ab.book_id).or_default().push(author.name);
                        }
                        map
                    },
                )
        };

        Ok(BookListResult {
            items,
            author_names,
            total,
        })
    }

    async fn find_authors(&self, book: &Book) -> Result<Vec<Author>, RepositoryError> {
        Ok(book.find_related(authors::Entity).all(self.db).await?)
    }

    async fn find_series(&self, book: &Book) -> Result<Vec<Series>, RepositoryError> {
        Ok(book.find_related(series::Entity).all(self.db).await?)
    }

    async fn find_identifiers(&self, book: &Book) -> Result<Vec<Identifier>, RepositoryError> {
        Ok(book.find_related(identifiers::Entity).all(self.db).await?)
    }
}
