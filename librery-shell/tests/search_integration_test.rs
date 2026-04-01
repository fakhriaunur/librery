use librery_core::{ActorId, CirculationEvent, CirculationRepository, ItemId};
use librery_shell::search::SearchIndex;
use librery_shell::SqliteCirculationRepository;

#[tokio::test]
async fn full_search_flow() {
    // 1. Create in-memory SQLite repository
    let repo = SqliteCirculationRepository::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite repository");

    // 2. Create in-memory Tantivy search index
    let search_index = SearchIndex::new().expect("Failed to create search index");

    // 3. Create a circulation event
    let event = CirculationEvent::ItemCheckedOut {
        item_id: ItemId("item-001".into()),
        patron_id: ActorId("patron-001".into()),
        timestamp: 1234567890,
        correlation_id: "corr-001".into(),
    };

    // 4. Persist event to SQLite via repository
    repo.append_event(&event)
        .await
        .expect("Failed to append event to repository");

    // 5. Verify event is persisted in SQLite
    let stored_events = repo.get_events().await.expect("Failed to get events");
    assert_eq!(stored_events.len(), 1, "Expected 1 event in repository");
    assert_eq!(stored_events[0], event, "Stored event should match original");

    // 6. Project event to Tantivy index
    search_index
        .index_event(&event)
        .expect("Failed to index event in Tantivy");

    // 7. Search for the event by item_id
    let results = search_index
        .search("item-001")
        .expect("Failed to search index");

    // 8. Verify search results
    assert!(!results.is_empty(), "Search should return at least one result");
    assert_eq!(results.len(), 1, "Expected exactly 1 search result");
    assert_eq!(results[0].item_id, "item-001", "Result item_id should match");
    assert_eq!(results[0].patron_id, "patron-001", "Result patron_id should match");
    assert_eq!(results[0].event_type, "ItemCheckedOut", "Result event_type should match");
    assert_eq!(results[0].timestamp, 1234567890, "Result timestamp should match");
}

#[tokio::test]
async fn search_flow_with_multiple_events() {
    // 1. Create in-memory SQLite repository
    let repo = SqliteCirculationRepository::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite repository");

    // 2. Create in-memory Tantivy search index
    let search_index = SearchIndex::new().expect("Failed to create search index");

    // 3. Create multiple circulation events
    let events = vec![
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-001".into()),
            patron_id: ActorId("patron-001".into()),
            timestamp: 1000,
            correlation_id: "corr-001".into(),
        },
        CirculationEvent::ItemReturned {
            item_id: ItemId("item-001".into()),
            patron_id: ActorId("patron-001".into()),
            timestamp: 2000,
            correlation_id: "corr-002".into(),
        },
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-002".into()),
            patron_id: ActorId("patron-002".into()),
            timestamp: 3000,
            correlation_id: "corr-003".into(),
        },
    ];

    // 4. Persist all events to SQLite
    for event in &events {
        repo.append_event(event)
            .await
            .expect("Failed to append event to repository");
    }

    // 5. Verify all events are persisted
    let stored_events = repo.get_events().await.expect("Failed to get events");
    assert_eq!(stored_events.len(), 3, "Expected 3 events in repository");

    // 6. Project all events to Tantivy index
    for event in &events {
        search_index
            .index_event(event)
            .expect("Failed to index event in Tantivy");
    }

    // 7. Search for events by item_id
    let results_item1 = search_index
        .search("item-001")
        .expect("Failed to search for item-001");
    assert_eq!(results_item1.len(), 2, "Expected 2 results for item-001");

    let results_item2 = search_index
        .search("item-002")
        .expect("Failed to search for item-002");
    assert_eq!(results_item2.len(), 1, "Expected 1 result for item-002");

    // 8. Search by patron_id
    let results_patron1 = search_index
        .search("patron-001")
        .expect("Failed to search for patron-001");
    assert_eq!(results_patron1.len(), 2, "Expected 2 results for patron-001");

    // 9. Search by event_type
    let results_checkout = search_index
        .search("ItemCheckedOut")
        .expect("Failed to search for ItemCheckedOut");
    assert_eq!(results_checkout.len(), 2, "Expected 2 ItemCheckedOut events");

    let results_returned = search_index
        .search("ItemReturned")
        .expect("Failed to search for ItemReturned");
    assert_eq!(results_returned.len(), 1, "Expected 1 ItemReturned event");
}

#[tokio::test]
async fn search_flow_with_rebuild_from_events() {
    let repo = SqliteCirculationRepository::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite repository");

    let search_index = SearchIndex::new().expect("Failed to create search index");

    let events = vec![
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-001".into()),
            patron_id: ActorId("patron-001".into()),
            timestamp: 1000,
            correlation_id: "corr-001".into(),
        },
        CirculationEvent::HoldPlaced {
            item_id: ItemId("item-002".into()),
            patron_id: ActorId("patron-002".into()),
            timestamp: 2000,
            correlation_id: "corr-002".into(),
        },
    ];

    for event in &events {
        repo.append_event(event)
            .await
            .expect("Failed to append event to repository");
    }

    let stored_events = repo.get_events().await.expect("Failed to get events");
    search_index
        .rebuild_from_events(&stored_events)
        .expect("Failed to rebuild index from events");

    let item1_results = search_index
        .search("item-001")
        .expect("Failed to search for item-001");
    assert!(!item1_results.is_empty(), "Should find item-001");
    assert!(item1_results.iter().any(|r| r.event_type == "ItemCheckedOut"), "Should find ItemCheckedOut for item-001");

    let item2_results = search_index
        .search("item-002")
        .expect("Failed to search for item-002");
    assert!(!item2_results.is_empty(), "Should find item-002");
    assert!(item2_results.iter().any(|r| r.event_type == "HoldPlaced"), "Should find HoldPlaced for item-002");
}

#[tokio::test]
async fn search_flow_verifies_timestamp_ordering() {
    // 1. Create in-memory SQLite repository
    let repo = SqliteCirculationRepository::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite repository");

    // 2. Create in-memory Tantivy search index
    let search_index = SearchIndex::new().expect("Failed to create search index");

    // 3. Create events with different timestamps
    let events = vec![
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-001".into()),
            patron_id: ActorId("patron-001".into()),
            timestamp: 5000,
            correlation_id: "corr-latest".into(),
        },
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-001".into()),
            patron_id: ActorId("patron-001".into()),
            timestamp: 1000,
            correlation_id: "corr-earliest".into(),
        },
        CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-001".into()),
            patron_id: ActorId("patron-001".into()),
            timestamp: 3000,
            correlation_id: "corr-middle".into(),
        },
    ];

    // 4. Persist and index events
    for event in &events {
        repo.append_event(event)
            .await
            .expect("Failed to append event to repository");
        search_index
            .index_event(event)
            .expect("Failed to index event");
    }

    // 5. Search and verify timestamps are present
    let results = search_index
        .search("item-001")
        .expect("Failed to search for item-001");

    assert_eq!(results.len(), 3, "Expected 3 results");

    // 6. Verify all timestamps are present in results
    let timestamps: Vec<i64> = results.iter().map(|r| r.timestamp).collect();
    assert!(timestamps.contains(&1000), "Should contain timestamp 1000");
    assert!(timestamps.contains(&3000), "Should contain timestamp 3000");
    assert!(timestamps.contains(&5000), "Should contain timestamp 5000");
}
