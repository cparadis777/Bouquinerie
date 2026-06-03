use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::str::FromStr;
use time::OffsetDateTime;
use tower_sessions::session::{Id, Record};
use tower_sessions::session_store::{self, ExpiredDeletion, SessionStore};

use crate::session;

#[derive(Clone, Debug)]
pub struct SeaOrmSessionStore {
    db: DatabaseConnection,
}

impl SeaOrmSessionStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

fn time_to_chrono(t: OffsetDateTime) -> DateTime<Utc> {
    DateTime::from_timestamp(t.unix_timestamp(), t.nanosecond()).expect("valid timestamp")
}

fn chrono_to_time(c: DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(c.timestamp())
        .expect("valid timestamp")
        .replace_nanosecond(c.timestamp_subsec_nanos())
        .expect("valid nanosecond")
}

fn record_to_active_model(record: &Record) -> session::ActiveModel {
    session::ActiveModel {
        id: Set(record.id.to_string()),
        data: Set(serde_json::to_value(&record.data).expect("serializable data")),
        expiry_date: Set(time_to_chrono(record.expiry_date)),
    }
}

fn active_model_to_record(model: &session::Model) -> Result<Record, session_store::Error> {
    Ok(Record {
        id: Id::from_str(&model.id).map_err(|e| session_store::Error::Decode(e.to_string()))?,
        data: serde_json::from_value(model.data.clone())
            .map_err(|e| session_store::Error::Decode(e.to_string()))?,
        expiry_date: chrono_to_time(model.expiry_date),
    })
}

#[async_trait]
impl SessionStore for SeaOrmSessionStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let model = record_to_active_model(record);
        session::Entity::insert(model)
            .on_conflict(
                OnConflict::column(session::Column::Id)
                    .update_columns([session::Column::Data, session::Column::ExpiryDate])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let result = session::Entity::find_by_id(session_id.to_string())
            .filter(session::Column::ExpiryDate.gt(Utc::now()))
            .one(&self.db)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        match result {
            Some(model) => Ok(Some(active_model_to_record(&model)?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        session::Entity::delete_by_id(session_id.to_string())
            .exec(&self.db)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for SeaOrmSessionStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        session::Entity::delete_many()
            .filter(session::Column::ExpiryDate.lt(Utc::now()))
            .exec(&self.db)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }
}
