use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub struct SqliteActorRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct ActorRow {
    pub id: String,
    pub actor_type: String,
    pub name: String,
    pub email: Option<String>,
    pub status: String,
}

impl SqliteActorRepository {
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
            CREATE TABLE IF NOT EXISTS actors (
                id TEXT PRIMARY KEY,
                actor_type TEXT NOT NULL,
                name TEXT NOT NULL,
                email TEXT,
                status TEXT NOT NULL DEFAULT 'Active'
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> sqlx::Result<Option<ActorRow>> {
        let row = sqlx::query(
            r#"SELECT id, actor_type, name, email, status FROM actors WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(ActorRow {
                id: row.try_get("id")?,
                actor_type: row.try_get("actor_type")?,
                name: row.try_get("name")?,
                email: row.try_get("email")?,
                status: row.try_get("status")?,
            })),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteActorRepository;

    #[tokio::test]
    async fn actor_persists_to_sqlite() {
        let repo = SqliteActorRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"INSERT INTO actors (id, actor_type, name, email, status) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind("patron-1")
        .bind("Patron")
        .bind("Test Patron")
        .bind("test@example.com")
        .bind("Active")
        .execute(&repo.pool)
        .await
        .unwrap();

        let found = repo.find_by_id("patron-1").await.unwrap().unwrap();
        assert_eq!(found.id, "patron-1");
        assert_eq!(found.name, "Test Patron");
        assert_eq!(found.email, Some("test@example.com".to_string()));
        assert_eq!(found.status, "Active");
    }

    #[tokio::test]
    async fn find_nonexistent_returns_none() {
        let repo = SqliteActorRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let found = repo.find_by_id("nonexistent").await.unwrap();
        assert!(found.is_none());
    }
}
