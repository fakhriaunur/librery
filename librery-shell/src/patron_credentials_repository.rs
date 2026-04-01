use async_trait::async_trait;
use librery_core::{ActorId, PatronCredentials, PatronCredentialsRepository};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub struct SqlitePatronCredentialsRepository {
    pool: SqlitePool,
}

impl SqlitePatronCredentialsRepository {
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
            CREATE TABLE IF NOT EXISTS patron_credentials (
                actor_id TEXT PRIMARY KEY,
                password_hash TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                last_login INTEGER
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl PatronCredentialsRepository for SqlitePatronCredentialsRepository {
    async fn create(&self, credentials: &PatronCredentials) -> sqlx::Result<()> {
        sqlx::query(
            r#"INSERT INTO patron_credentials (actor_id, password_hash, created_at, last_login) VALUES (?, ?, ?, ?)"#,
        )
        .bind(&credentials.actor_id.0)
        .bind(&credentials.password_hash)
        .bind(credentials.created_at)
        .bind(credentials.last_login)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_actor_id(&self, actor_id: &ActorId) -> sqlx::Result<Option<PatronCredentials>> {
        let row = sqlx::query(
            r#"SELECT actor_id, password_hash, created_at, last_login FROM patron_credentials WHERE actor_id = ?"#,
        )
        .bind(&actor_id.0)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(PatronCredentials {
                actor_id: ActorId(row.try_get("actor_id")?),
                password_hash: row.try_get("password_hash")?,
                created_at: row.try_get("created_at")?,
                last_login: row.try_get("last_login")?,
            })),
            None => Ok(None),
        }
    }

    async fn update_last_login(&self, actor_id: &ActorId, timestamp: i64) -> sqlx::Result<()> {
        sqlx::query(
            r#"UPDATE patron_credentials SET last_login = ? WHERE actor_id = ?"#,
        )
        .bind(timestamp)
        .bind(&actor_id.0)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SqlitePatronCredentialsRepository;
    use librery_core::{ActorId, PatronCredentials, PatronCredentialsRepository};

    #[tokio::test]
    async fn credentials_persist_to_sqlite() {
        let repo = SqlitePatronCredentialsRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let credentials = PatronCredentials {
            actor_id: ActorId("patron-1".into()),
            password_hash: "$2b$12$hash".into(),
            created_at: 100,
            last_login: None,
        };

        repo.create(&credentials).await.unwrap();

        let found = repo.find_by_actor_id(&ActorId("patron-1".into()))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found.actor_id, credentials.actor_id);
        assert_eq!(found.password_hash, credentials.password_hash);
        assert_eq!(found.created_at, credentials.created_at);
        assert_eq!(found.last_login, None);
    }

    #[tokio::test]
    async fn update_last_login_works() {
        let repo = SqlitePatronCredentialsRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let credentials = PatronCredentials {
            actor_id: ActorId("patron-1".into()),
            password_hash: "$2b$12$hash".into(),
            created_at: 100,
            last_login: None,
        };

        repo.create(&credentials).await.unwrap();
        repo.update_last_login(&ActorId("patron-1".into()), 200).await.unwrap();

        let found = repo.find_by_actor_id(&ActorId("patron-1".into()))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found.last_login, Some(200));
    }

    #[tokio::test]
    async fn find_nonexistent_returns_none() {
        let repo = SqlitePatronCredentialsRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let found = repo.find_by_actor_id(&ActorId("nonexistent".into()))
            .await
            .unwrap();

        assert!(found.is_none());
    }
}
