use async_trait::async_trait;
use librery_core::{BranchId, Item, ItemId, ItemRepository, ItemStatus};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub struct SqliteItemRepository {
    pool: SqlitePool,
}

impl SqliteItemRepository {
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
            CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                bib_id TEXT NOT NULL,
                barcode TEXT NOT NULL,
                status TEXT NOT NULL,
                location TEXT NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl ItemRepository for SqliteItemRepository {
    async fn create(&self, item: &Item) -> sqlx::Result<()> {
        sqlx::query(
            r#"INSERT INTO items (id, bib_id, barcode, status, location, updated_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&item.id.0)
        .bind(&item.bib_id)
        .bind(&item.barcode)
        .bind(item_status_to_str(&item.status))
        .bind(&item.location.0)
        .bind(0i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ItemId) -> sqlx::Result<Option<Item>> {
        let row = sqlx::query(
            r#"SELECT id, bib_id, barcode, status, location FROM items WHERE id = ?"#,
        )
        .bind(&id.0)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Item {
                id: ItemId(row.try_get("id")?),
                bib_id: row.try_get("bib_id")?,
                barcode: row.try_get("barcode")?,
                status: str_to_item_status(row.try_get("status")?),
                location: BranchId(row.try_get("location")?),
            })),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> sqlx::Result<Vec<Item>> {
        let rows = sqlx::query(
            r#"SELECT id, bib_id, barcode, status, location FROM items"#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(Item {
                    id: ItemId(row.try_get("id")?),
                    bib_id: row.try_get("bib_id")?,
                    barcode: row.try_get("barcode")?,
                    status: str_to_item_status(row.try_get("status")?),
                    location: BranchId(row.try_get("location")?),
                })
            })
            .collect()
    }

    async fn find_modified_since(&self, since: i64) -> sqlx::Result<Vec<Item>> {
        let rows = sqlx::query(
            r#"SELECT id, bib_id, barcode, status, location FROM items WHERE updated_at > ?"#,
        )
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(Item {
                    id: ItemId(row.try_get("id")?),
                    bib_id: row.try_get("bib_id")?,
                    barcode: row.try_get("barcode")?,
                    status: str_to_item_status(row.try_get("status")?),
                    location: BranchId(row.try_get("location")?),
                })
            })
            .collect()
    }
}

fn item_status_to_str(status: &ItemStatus) -> &'static str {
    match status {
        ItemStatus::Available => "Available",
        ItemStatus::CheckedOut => "CheckedOut",
        ItemStatus::OnHoldShelf => "OnHoldShelf",
        ItemStatus::Lost => "Lost",
        ItemStatus::InTransit => "InTransit",
    }
}

fn str_to_item_status(s: &str) -> ItemStatus {
    match s {
        "Available" => ItemStatus::Available,
        "CheckedOut" => ItemStatus::CheckedOut,
        "OnHoldShelf" => ItemStatus::OnHoldShelf,
        "Lost" => ItemStatus::Lost,
        "InTransit" => ItemStatus::InTransit,
        _ => ItemStatus::Available,
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteItemRepository;
    use librery_core::{BranchId, Item, ItemId, ItemRepository, ItemStatus};

    #[tokio::test]
    async fn item_persists_to_sqlite() {
        let repo = SqliteItemRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let item = Item {
            id: ItemId("item-1".into()),
            bib_id: "bib-1".into(),
            barcode: "BC001".into(),
            status: ItemStatus::Available,
            location: BranchId("main".into()),
        };

        repo.create(&item).await.unwrap();

        let found = repo.find_by_id(&ItemId("item-1".into()))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found.id, item.id);
        assert_eq!(found.bib_id, item.bib_id);
        assert_eq!(found.barcode, item.barcode);
        assert_eq!(found.status, item.status);
        assert_eq!(found.location, item.location);
    }

    #[tokio::test]
    async fn find_nonexistent_returns_none() {
        let repo = SqliteItemRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let found = repo.find_by_id(&ItemId("nonexistent".into()))
            .await
            .unwrap();

        assert!(found.is_none());
    }

    #[tokio::test]
    async fn find_all_returns_all_items() {
        let repo = SqliteItemRepository::connect("sqlite::memory:")
            .await
            .unwrap();

        let item1 = Item {
            id: ItemId("item-1".into()),
            bib_id: "bib-1".into(),
            barcode: "BC001".into(),
            status: ItemStatus::Available,
            location: BranchId("main".into()),
        };

        let item2 = Item {
            id: ItemId("item-2".into()),
            bib_id: "bib-2".into(),
            barcode: "BC002".into(),
            status: ItemStatus::CheckedOut,
            location: BranchId("branch-2".into()),
        };

        repo.create(&item1).await.unwrap();
        repo.create(&item2).await.unwrap();

        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }
}
