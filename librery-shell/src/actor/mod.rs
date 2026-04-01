//! Actor module for librery-shell
//! 
//! Contains infrastructure actors for event processing.

pub mod dead_letter_queue;
pub mod event_dispatcher;

pub use dead_letter_queue::{DeadLetterEntry, DeadLetterQueue};
pub use event_dispatcher::EventDispatcher;
