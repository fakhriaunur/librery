pub mod domain;

use async_trait::async_trait;

pub use domain::circulation::{CheckoutCommand, CheckoutError, CirculationEvent, ItemAggregate};
pub use domain::ids::{ActorId, BranchId, ItemId, LoanId};
pub use domain::{Actor, ActorStatus, ActorType, Item, ItemStatus};

#[async_trait]
pub trait CirculationRepository: Send + Sync {
    async fn append_event(&self, event: &CirculationEvent) -> sqlx::Result<i64>;

    async fn get_events(&self) -> sqlx::Result<Vec<CirculationEvent>>;

    async fn get_events_since(&self, timestamp: i64) -> sqlx::Result<Vec<CirculationEvent>>;
}
