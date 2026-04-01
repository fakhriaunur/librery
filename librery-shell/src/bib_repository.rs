use async_trait::async_trait;
use librery_core::{BibRecordId, BibliographicRecord, BibliographicRecordRepository};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub struct SqliteBibliographicRecordRepository {
    pool: SqlitePool,
}

impl SqliteBibliographicRecordRepository {
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
            CREATE TABLE IF NOT EXISTS bibliographic_records (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                isbn TEXT,
                publisher TEXT,
                publication_year INTEGER,
                marc21 TEXT
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl BibliographicRecordRepository for SqliteBibliographicRecordRepository {
    async fn find_by_id(&self, id: &BibRecordId) -> sqlx::Result<Option<BibliographicRecord>> {
        let row = sqlx::query(
            r#"SELECT id, title, author, isbn, publisher, publication_year, marc21 FROM bibliographic_records WHERE id = ?"#,
        )
        .bind(&id.0)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(BibliographicRecord {
                id: BibRecordId(row.try_get("id")?),
                title: row.try_get("title")?,
                author: row.try_get("author")?,
                isbn: row.try_get("isbn")?,
                publisher: row.try_get("publisher")?,
                publication_year: row.try_get("publication_year")?,
                marc21: row.try_get("marc21")?,
            })),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> sqlx::Result<Vec<BibliographicRecord>> {
        let rows = sqlx::query(
            r#"SELECT id, title, author, isbn, publisher, publication_year, marc21 FROM bibliographic_records"#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(BibliographicRecord {
                    id: BibRecordId(row.try_get("id")?),
                    title: row.try_get("title")?,
                    author: row.try_get("author")?,
                    isbn: row.try_get("isbn")?,
                    publisher: row.try_get("publisher")?,
                    publication_year: row.try_get("publication_year")?,
                    marc21: row.try_get("marc21")?,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteBibliographicRecordRepository;
    use librery_core::{BibRecordId, BibliographicRecord, BibliographicRecordRepository};

    #[tokio::test]
    async fn bib_record_persists_to_sqlite() {
        let repo = SqliteBibliographicRecordRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let record = BibliographicRecord {
            id: BibRecordId("bib-1".into()),
            title: "The Rust Programming Language".into(),
            author: "Steve Klabnik".into(),
            isbn: Some("978-1718500440".into()),
            publisher: Some("No Starch Press".into()),
            publication_year: Some(2019),
            marc21: None,
        };

        sqlx::query(
            r#"INSERT INTO bibliographic_records (id, title, author, isbn, publisher, publication_year, marc21) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&record.id.0)
        .bind(&record.title)
        .bind(&record.author)
        .bind(&record.isbn)
        .bind(&record.publisher)
        .bind(record.publication_year)
        .bind(&record.marc21)
        .execute(&repo.pool)
        .await
        .unwrap();

        let found = repo.find_by_id(&BibRecordId("bib-1".into()))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found.id, record.id);
        assert_eq!(found.title, record.title);
        assert_eq!(found.author, record.author);
        assert_eq!(found.isbn, record.isbn);
        assert_eq!(found.publisher, record.publisher);
        assert_eq!(found.publication_year, record.publication_year);
    }

    #[tokio::test]
    async fn find_nonexistent_returns_none() {
        let repo = SqliteBibliographicRecordRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let found = repo.find_by_id(&BibRecordId("nonexistent".into()))
            .await
            .unwrap();

        assert!(found.is_none());
    }

    #[tokio::test]
    async fn find_all_returns_all_records() {
        let repo = SqliteBibliographicRecordRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let record1 = BibliographicRecord {
            id: BibRecordId("bib-1".into()),
            title: "Book 1".into(),
            author: "Author 1".into(),
            isbn: None,
            publisher: None,
            publication_year: None,
            marc21: None,
        };

        let record2 = BibliographicRecord {
            id: BibRecordId("bib-2".into()),
            title: "Book 2".into(),
            author: "Author 2".into(),
            isbn: None,
            publisher: None,
            publication_year: None,
            marc21: None,
        };

        for record in [&record1, &record2] {
            sqlx::query(
                r#"INSERT INTO bibliographic_records (id, title, author, isbn, publisher, publication_year, marc21) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            )
            .bind(&record.id.0)
            .bind(&record.title)
            .bind(&record.author)
            .bind(&record.isbn)
            .bind(&record.publisher)
            .bind(record.publication_year)
            .bind(&record.marc21)
            .execute(&repo.pool)
            .await
            .unwrap();
        }

        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }
}
