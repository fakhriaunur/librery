use async_trait::async_trait;
use librery_core::{CirculationEvent, CirculationRepository, ActorId, ItemId};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub mod api;
pub mod middleware;
pub mod search;
pub mod patron_credentials_repository;

use std::sync::Arc;

use biscuit_auth::KeyPair;
use librery_core::PatronCredentialsRepository;

use crate::search::SearchIndex;

#[derive(Clone)]
pub struct AppState {
    pub search_index: Arc<SearchIndex>,
    pub credentials_repo: Arc<dyn PatronCredentialsRepository>,
    pub biscuit_keypair: Arc<KeyPair>,
}

pub struct SqliteCirculationRepository {
    pool: SqlitePool,
}

impl SqliteCirculationRepository {
    pub async fn connect(database_url: &str) -> sqlx::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(database_url)
            .await?;

        Self::ensure_schema(&pool).await?;

        Ok(Self { pool })
    }

    async fn ensure_schema(pool: &SqlitePool) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS circulation_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT NOT NULL,
                item_id TEXT NOT NULL,
                patron_id TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                correlation_id TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl CirculationRepository for SqliteCirculationRepository {
    async fn append_event(&self, event: &CirculationEvent) -> sqlx::Result<i64> {
        let (event_type, item_id, patron_id, timestamp, correlation_id): (&str, String, String, i64, String) = match event {
            CirculationEvent::ItemCheckedOut {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                "ItemCheckedOut",
                item_id.0.clone(),
                patron_id.0.clone(),
                *timestamp,
                correlation_id.clone(),
            ),
            CirculationEvent::ItemReturned {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                "ItemReturned",
                item_id.0.clone(),
                patron_id.0.clone(),
                *timestamp,
                correlation_id.clone(),
            ),
            CirculationEvent::ItemRenewed {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                "ItemRenewed",
                item_id.0.clone(),
                patron_id.0.clone(),
                *timestamp,
                correlation_id.clone(),
            ),
            CirculationEvent::HoldPlaced {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                "HoldPlaced",
                item_id.0.clone(),
                patron_id.0.clone(),
                *timestamp,
                correlation_id.clone(),
            ),
            CirculationEvent::HoldCancelled {
                item_id,
                timestamp,
                correlation_id,
            } => (
                "HoldCancelled",
                item_id.0.clone(),
                String::new(),
                *timestamp,
                correlation_id.clone(),
            ),
            CirculationEvent::HoldFulfilled {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                "HoldFulfilled",
                item_id.0.clone(),
                patron_id.0.clone(),
                *timestamp,
                correlation_id.clone(),
            ),
            CirculationEvent::ItemLost {
                item_id,
                timestamp,
                correlation_id,
            } => (
                "ItemLost",
                item_id.0.clone(),
                String::new(),
                *timestamp,
                correlation_id.clone(),
            ),
        };

        let result = sqlx::query(
            r#"INSERT INTO circulation_events (event_type, item_id, patron_id, timestamp, correlation_id) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(event_type)
        .bind(item_id)
        .bind(patron_id)
        .bind(timestamp)
        .bind(correlation_id)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    async fn get_events(&self) -> sqlx::Result<Vec<CirculationEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT event_type, item_id, patron_id, timestamp, correlation_id
            FROM circulation_events
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                let item_id = ItemId(row.try_get::<String, _>("item_id")?);
                let patron_id = ActorId(row.try_get::<String, _>("patron_id")?);
                let timestamp = row.try_get("timestamp")?;
                let correlation_id = row.try_get("correlation_id")?;

                Ok(match row.try_get::<String, _>("event_type")?.as_str() {
                    "ItemReturned" => CirculationEvent::ItemReturned {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    "ItemRenewed" => CirculationEvent::ItemRenewed {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    "HoldPlaced" => CirculationEvent::HoldPlaced {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    _ => CirculationEvent::ItemCheckedOut {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                })
            })
            .collect()
    }

    async fn get_events_since(&self, since: i64) -> sqlx::Result<Vec<CirculationEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT event_type, item_id, patron_id, timestamp, correlation_id
            FROM circulation_events
            WHERE timestamp > ?
            ORDER BY timestamp ASC
            "#,
        )
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                let item_id = ItemId(row.try_get::<String, _>("item_id")?);
                let patron_id = ActorId(row.try_get::<String, _>("patron_id")?);
                let timestamp = row.try_get("timestamp")?;
                let correlation_id = row.try_get("correlation_id")?;

                Ok(match row.try_get::<String, _>("event_type")?.as_str() {
                    "ItemReturned" => CirculationEvent::ItemReturned {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    "ItemRenewed" => CirculationEvent::ItemRenewed {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    "HoldPlaced" => CirculationEvent::HoldPlaced {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    "HoldCancelled" => CirculationEvent::HoldCancelled {
                        item_id,
                        timestamp,
                        correlation_id,
                    },
                    "HoldFulfilled" => CirculationEvent::HoldFulfilled {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                    "ItemLost" => CirculationEvent::ItemLost {
                        item_id,
                        timestamp,
                        correlation_id,
                    },
                    _ => CirculationEvent::ItemCheckedOut {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteCirculationRepository;
    use librery_core::CirculationRepository;
    use librery_core::{ActorId, CirculationEvent, ItemId};

    #[tokio::test]
    async fn event_persists_to_sqlite() {
        let repo = SqliteCirculationRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let event = CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-1".into()),
            patron_id: ActorId("patron-1".into()),
            timestamp: 123,
            correlation_id: "corr-1".into(),
        };

        repo.append_event(&event)
            .await
            .unwrap();

        let events = repo.get_events().await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], event);
    }
}
