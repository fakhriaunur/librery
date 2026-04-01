//! Dead Letter Queue for failed circulation events.
//!
//! Provides in-memory storage with bounded retries (max 3).

use librery_core::CirculationEvent;
use std::collections::VecDeque;

/// Entry in the dead letter queue.
#[derive(Debug, Clone)]
pub struct DeadLetterEntry {
    /// The failed event payload.
    pub event: CirculationEvent,
    /// Reason for failure.
    pub error: String,
    /// Unix timestamp when first added to DLQ.
    pub timestamp: i64,
    /// Number of retry attempts (0-3).
    pub retry_count: u8,
}

/// In-memory dead letter queue with bounded retries.
#[derive(Debug)]
pub struct DeadLetterQueue {
    queue: VecDeque<DeadLetterEntry>,
    max_retries: u8,
}

impl DeadLetterQueue {
    /// Create a new DLQ with default max_retries (3).
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_retries: 3,
        }
    }

    /// Create a DLQ with custom max retries.
    pub fn with_max_retries(max_retries: u8) -> Self {
        Self {
            queue: VecDeque::new(),
            max_retries,
        }
    }

    /// Add a failed event to the queue.
    pub fn push(&mut self, event: CirculationEvent, error: String, timestamp: i64) {
        self.queue.push_back(DeadLetterEntry {
            event,
            error,
            timestamp,
            retry_count: 0,
        });
    }

    /// Check if an entry should be retried.
    pub fn should_retry(&self, entry: &DeadLetterEntry) -> bool {
        entry.retry_count < self.max_retries
    }

    /// Get the maximum number of retries allowed.
    pub fn max_retries(&self) -> u8 {
        self.max_retries
    }

    /// Get current queue length.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get entries that can be retried.
    pub fn get_retryable(&self) -> Vec<&DeadLetterEntry> {
        self.queue.iter().filter(|e| self.should_retry(e)).collect()
    }

    /// Calculate exponential backoff duration in milliseconds.
    /// Base delay: 100ms, doubles with each retry.
    pub fn backoff_duration(&self, retry_count: u8) -> u64 {
        let base = 100u64;
        let multiplier = 2u64.pow(retry_count as u32);
        base * multiplier
    }

    /// Mark an entry as retried (increment retry count).
    pub fn increment_retry(&mut self, index: usize) {
        if let Some(entry) = self.queue.get_mut(index) {
            entry.retry_count += 1;
        }
    }

    /// Remove an entry from the queue.
    pub fn remove(&mut self, index: usize) -> Option<DeadLetterEntry> {
        if index < self.queue.len() {
            self.queue.remove(index)
        } else {
            None
        }
    }

    /// Get all entries.
    pub fn entries(&self) -> impl Iterator<Item = &DeadLetterEntry> {
        self.queue.iter()
    }
}

impl Default for DeadLetterQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dlq_is_empty() {
        let dlq = DeadLetterQueue::new();
        assert!(dlq.is_empty());
        assert_eq!(dlq.len(), 0);
    }

    #[test]
    fn test_push_adds_entry() {
        let mut dlq = DeadLetterQueue::new();
        let event = CirculationEvent::ItemCheckedOut {
            item_id: "item-1".into(),
            patron_id: "patron-1".into(),
            timestamp: 1000,
            correlation_id: None,
        };
        dlq.push(event, "Connection timeout".into(), 1000);

        assert_eq!(dlq.len(), 1);
    }

    #[test]
    fn test_should_retry_within_limit() {
        let dlq = DeadLetterQueue::new();
        let entry = DeadLetterEntry {
            event: CirculationEvent::ItemCheckedOut {
                item_id: "item-1".into(),
                patron_id: "patron-1".into(),
                timestamp: 1000,
                correlation_id: None,
            },
            error: "error".into(),
            timestamp: 1000,
            retry_count: 2,
        };

        assert!(dlq.should_retry(&entry));
    }

    #[test]
    fn test_should_retry_at_limit() {
        let dlq = DeadLetterQueue::new();
        let entry = DeadLetterEntry {
            event: CirculationEvent::ItemCheckedOut {
                item_id: "item-1".into(),
                patron_id: "patron-1".into(),
                timestamp: 1000,
                correlation_id: None,
            },
            error: "error".into(),
            timestamp: 1000,
            retry_count: 3,
        };

        assert!(!dlq.should_retry(&entry));
    }

    #[test]
    fn test_exponential_backoff() {
        let dlq = DeadLetterQueue::new();

        // First retry: 100ms * 2^0 = 100ms
        assert_eq!(dlq.backoff_duration(0), 100);
        // Second retry: 100ms * 2^1 = 200ms
        assert_eq!(dlq.backoff_duration(1), 200);
        // Third retry: 100ms * 2^2 = 400ms
        assert_eq!(dlq.backoff_duration(2), 400);
    }
}
