use std::sync::Arc;
use std::time::Duration;

use librery_core::{CirculationRepository, ItemRepository};
use tokio::sync::broadcast;

use super::SearchIndex;

/// Background worker that polls for new circulation events and items, projecting them to Tantivy.
///
/// Provides eventual consistency between the event store and search index.
/// The worker polls at a configurable interval and indexes any events/items newer
/// than the last processed timestamp.
pub struct ProjectionWorker {
    event_repository: Arc<dyn CirculationRepository>,
    item_repository: Arc<dyn ItemRepository>,
    search_index: Arc<SearchIndex>,
    last_event_timestamp: i64,
    last_item_timestamp: i64,
    poll_interval: Duration,
}

impl ProjectionWorker {
    pub fn new(
        event_repository: Arc<dyn CirculationRepository>,
        item_repository: Arc<dyn ItemRepository>,
        search_index: Arc<SearchIndex>,
    ) -> Self {
        Self {
            event_repository,
            item_repository,
            search_index,
            last_event_timestamp: 0,
            last_item_timestamp: 0,
            poll_interval: Duration::from_secs(1),
        }
    }

    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    pub async fn run(&mut self, shutdown: &mut broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = shutdown.recv() => break,
                _ = tokio::time::sleep(self.poll_interval) => {
                    self.poll_and_index_events().await;
                    self.poll_and_index_items().await;
                }
            }
        }
    }

    async fn poll_and_index_events(&mut self) {
        let events = match self.event_repository.get_events_since(self.last_event_timestamp).await {
            Ok(events) => events,
            Err(_) => return,
        };

        for event in events {
            if self.search_index.index_event(&event).is_ok() {
                self.last_event_timestamp = event.timestamp();
            }
        }
    }

    async fn poll_and_index_items(&mut self) {
        let items = match self.item_repository.find_modified_since(self.last_item_timestamp).await {
            Ok(items) => items,
            Err(_) => return,
        };

        for item in items {
            if self.search_index.index_item(&item).is_ok() {
                self.last_item_timestamp = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use librery_core::{ActorId, CirculationEvent, Item, ItemId, ItemRepository};
    use tokio::sync::broadcast;

    fn test_event(timestamp: i64) -> CirculationEvent {
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-1".into()),
            patron_id: ActorId("patron-1".into()),
            timestamp,
            correlation_id: "corr-1".into(),
        }
    }

    struct MockItemRepo;

    #[async_trait]
    impl ItemRepository for MockItemRepo {
        async fn create(&self, _item: &Item) -> sqlx::Result<()> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &ItemId) -> sqlx::Result<Option<Item>> {
            Ok(None)
        }

        async fn find_all(&self) -> sqlx::Result<Vec<Item>> {
            Ok(vec![])
        }

        async fn find_modified_since(&self, _since: i64) -> sqlx::Result<Vec<Item>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn worker_indexes_events_on_poll() {
        use std::sync::Mutex;

        struct MockRepo {
            events: Mutex<Vec<CirculationEvent>>,
        }

        #[async_trait]
        impl CirculationRepository for MockRepo {
            async fn append_event(&self, _event: &CirculationEvent) -> sqlx::Result<i64> {
                Ok(0)
            }

            async fn get_events(&self) -> sqlx::Result<Vec<CirculationEvent>> {
                Ok(self.events.lock().unwrap().clone())
            }

            async fn get_events_since(&self, since: i64) -> sqlx::Result<Vec<CirculationEvent>> {
                Ok(self
                    .events
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|e| e.timestamp() > since)
                    .cloned()
                    .collect())
            }
        }

        let event_repo = Arc::new(MockRepo {
            events: Mutex::new(vec![test_event(100), test_event(200)]),
        });
        let item_repo = Arc::new(MockItemRepo);
        let search_index = Arc::new(SearchIndex::new().unwrap());
        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

        let mut worker = ProjectionWorker::new(event_repo, item_repo, search_index.clone())
            .with_poll_interval(Duration::from_millis(10));

        let handle = tokio::spawn(async move {
            worker.run(&mut shutdown_rx).await;
        });

        tokio::time::sleep(Duration::from_millis(50)).await;
        shutdown_tx.send(()).unwrap();
        handle.await.unwrap();

        let reader = search_index.index().reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 2);
    }

    #[tokio::test]
    async fn worker_handles_empty_poll() {
        struct EmptyRepo;

        #[async_trait]
        impl CirculationRepository for EmptyRepo {
            async fn append_event(&self, _event: &CirculationEvent) -> sqlx::Result<i64> {
                Ok(0)
            }

            async fn get_events(&self) -> sqlx::Result<Vec<CirculationEvent>> {
                Ok(vec![])
            }

            async fn get_events_since(&self, _since: i64) -> sqlx::Result<Vec<CirculationEvent>> {
                Ok(vec![])
            }
        }

        let event_repo = Arc::new(EmptyRepo);
        let item_repo = Arc::new(MockItemRepo);
        let search_index = Arc::new(SearchIndex::new().unwrap());
        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

        let mut worker = ProjectionWorker::new(event_repo, item_repo, search_index.clone())
            .with_poll_interval(Duration::from_millis(10));

        let handle = tokio::spawn(async move {
            worker.run(&mut shutdown_rx).await;
        });

        tokio::time::sleep(Duration::from_millis(50)).await;
        shutdown_tx.send(()).unwrap();
        handle.await.unwrap();

        let reader = search_index.index().reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 0);
    }
}
