pub mod domain;

use async_trait::async_trait;

pub use domain::circulation::{CheckoutCommand, CheckoutError, CirculationEvent, ItemAggregate};
pub use domain::ids::{ActorId, BibRecordId, BranchId, ItemId, LoanId};
pub use domain::patron_credentials::PatronCredentials;
pub use domain::BibliographicRecord;
pub use domain::{Actor, ActorStatus, ActorType, Item, ItemStatus};

#[async_trait]
pub trait CirculationRepository: Send + Sync {
    async fn append_event(&self, event: &CirculationEvent) -> sqlx::Result<i64>;

    async fn get_events(&self) -> sqlx::Result<Vec<CirculationEvent>>;

    async fn get_events_since(&self, timestamp: i64) -> sqlx::Result<Vec<CirculationEvent>>;
}

#[async_trait]
pub trait PatronCredentialsRepository: Send + Sync {
    async fn create(&self, credentials: &PatronCredentials) -> sqlx::Result<()>;

    async fn find_by_actor_id(&self, actor_id: &ActorId) -> sqlx::Result<Option<PatronCredentials>>;

    async fn update_last_login(&self, actor_id: &ActorId, timestamp: i64) -> sqlx::Result<()>;
}

#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn create(&self, item: &Item) -> sqlx::Result<()>;

    async fn find_by_id(&self, id: &ItemId) -> sqlx::Result<Option<Item>>;

    async fn find_all(&self) -> sqlx::Result<Vec<Item>>;

    async fn find_modified_since(&self, since: i64) -> sqlx::Result<Vec<Item>>;
}

#[async_trait]
pub trait BibliographicRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: &BibRecordId) -> sqlx::Result<Option<BibliographicRecord>>;

    async fn find_all(&self) -> sqlx::Result<Vec<BibliographicRecord>>;
}
