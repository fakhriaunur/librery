use async_trait::async_trait;
use librery_core::{CirculationEvent, CirculationRepository};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct EventDispatcher {
    tx: mpsc::Sender<CirculationEvent>,
}

impl EventDispatcher {
    pub fn new() -> (Self, mpsc::Receiver<CirculationEvent>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub fn sender(&self) -> mpsc::Sender<CirculationEvent> {
        self.tx.clone()
    }

    pub async fn run(
        &self,
        mut rx: mpsc::Receiver<CirculationEvent>,
        repository: Arc<dyn CirculationRepository>,
    ) {
        while let Some(event) = rx.recv().await {
            let _ = repository.append_event(&event).await;
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self {
            tx: mpsc::channel(100).0,
        }
    }
}
