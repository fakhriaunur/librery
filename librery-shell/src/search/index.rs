use librery_core::domain::BibliographicRecord;
use librery_core::CirculationEvent;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value, INDEXED, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexWriter, TantivyDocument};

/// Embedded Tantivy search index for CQRS read model.
///
/// Provides full-text search over circulation events with eventual consistency.
/// The index rebuilds from all events on startup and accepts new events asynchronously.
pub struct SearchIndex {
    index: Index,
    schema: Schema,
    // Circulation event fields
    item_id_field: tantivy::schema::Field,
    patron_id_field: tantivy::schema::Field,
    event_type_field: tantivy::schema::Field,
    correlation_id_field: tantivy::schema::Field,
    timestamp_field: tantivy::schema::Field,
    // Bibliographic record fields
    title_field: tantivy::schema::Field,
    author_field: tantivy::schema::Field,
    isbn_field: tantivy::schema::Field,
    publisher_field: tantivy::schema::Field,
    barcode_field: tantivy::schema::Field,
}

impl SearchIndex {
    /// Creates a new in-memory Tantivy index with the circulation event schema.
    pub fn new() -> tantivy::Result<Self> {
        let mut schema_builder = Schema::builder();
        let item_id_field = schema_builder.add_text_field("item_id", STRING | STORED);
        let patron_id_field = schema_builder.add_text_field("patron_id", STRING | STORED);
        let event_type_field = schema_builder.add_text_field("event_type", STRING | STORED);
        let correlation_id_field = schema_builder.add_text_field("correlation_id", STRING | STORED);
        let timestamp_field = schema_builder.add_i64_field("timestamp", INDEXED | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let author_field = schema_builder.add_text_field("author", TEXT | STORED);
        let isbn_field = schema_builder.add_text_field("isbn", STRING | STORED);
        let publisher_field = schema_builder.add_text_field("publisher", TEXT | STORED);
        let barcode_field = schema_builder.add_text_field("barcode", STRING | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());

        Ok(Self {
            index,
            schema,
            item_id_field,
            patron_id_field,
            event_type_field,
            correlation_id_field,
            timestamp_field,
            title_field,
            author_field,
            isbn_field,
            publisher_field,
            barcode_field,
        })
    }

    /// Indexes a single circulation event.
    ///
    /// This is eventually consistent - the document becomes searchable after
    /// the next commit. Call `commit()` to flush pending changes.
    pub fn index_event(&self, event: &CirculationEvent) -> tantivy::Result<()> {
        let mut writer: IndexWriter = self.index.writer(50_000_000)?;

        let doc = self.event_to_document(event)?;
        writer.add_document(doc)?;
        writer.commit()?;

        Ok(())
    }

    /// Indexes a bibliographic record for catalog search.
    ///
    /// The document becomes searchable after the next commit.
    pub fn index_document(&self, record: &BibliographicRecord) -> tantivy::Result<()> {
        let mut writer: IndexWriter = self.index.writer(50_000_000)?;

        let mut doc = TantivyDocument::new();
        doc.add_text(self.title_field, &record.title);
        doc.add_text(self.author_field, &record.author);
        if let Some(ref isbn) = record.isbn {
            doc.add_text(self.isbn_field, isbn);
        }
        if let Some(ref publisher) = record.publisher {
            doc.add_text(self.publisher_field, publisher);
        }

        writer.add_document(doc)?;
        writer.commit()?;

        Ok(())
    }

    /// Rebuilds the entire index from a sequence of circulation events.
    ///
    /// This is the startup strategy: replay all events to reconstruct the search index.
    /// The index is cleared before rebuilding to ensure consistency.
    pub fn rebuild_from_events(&self, events: &[CirculationEvent]) -> tantivy::Result<()> {
        let mut writer: IndexWriter = self.index.writer(50_000_000)?;

        writer.delete_all_documents()?;

        for event in events {
            let doc = self.event_to_document(event)?;
            writer.add_document(doc)?;
        }

        writer.commit()?;
        Ok(())
    }

    /// Converts a CirculationEvent to a Tantivy document.
    fn event_to_document(&self, event: &CirculationEvent) -> tantivy::Result<TantivyDocument> {
        let (item_id, patron_id, event_type, timestamp, correlation_id) = match event {
            CirculationEvent::ItemCheckedOut {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                patron_id.0.as_str(),
                "ItemCheckedOut",
                *timestamp,
                correlation_id.as_str(),
            ),
            CirculationEvent::ItemReturned {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                patron_id.0.as_str(),
                "ItemReturned",
                *timestamp,
                correlation_id.as_str(),
            ),
            CirculationEvent::ItemRenewed {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                patron_id.0.as_str(),
                "ItemRenewed",
                *timestamp,
                correlation_id.as_str(),
            ),
            CirculationEvent::HoldPlaced {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                patron_id.0.as_str(),
                "HoldPlaced",
                *timestamp,
                correlation_id.as_str(),
            ),
            CirculationEvent::HoldCancelled {
                item_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                "",
                "HoldCancelled",
                *timestamp,
                correlation_id.as_str(),
            ),
            CirculationEvent::HoldFulfilled {
                item_id,
                patron_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                patron_id.0.as_str(),
                "HoldFulfilled",
                *timestamp,
                correlation_id.as_str(),
            ),
            CirculationEvent::ItemLost {
                item_id,
                timestamp,
                correlation_id,
            } => (
                item_id.0.as_str(),
                "",
                "ItemLost",
                *timestamp,
                correlation_id.as_str(),
            ),
        };

        Ok(doc!(
            self.item_id_field => item_id,
            self.patron_id_field => patron_id,
            self.event_type_field => event_type,
            self.correlation_id_field => correlation_id,
            self.timestamp_field => timestamp,
        ))
    }

    pub fn search(&self, query_str: &str) -> tantivy::Result<Vec<SearchResult>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.item_id_field,
                self.patron_id_field,
                self.event_type_field,
                self.correlation_id_field,
                self.title_field,
                self.author_field,
                self.isbn_field,
                self.publisher_field,
                self.barcode_field,
            ],
        );

        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(100))?;

        let mut results = Vec::new();

        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            let item_id = retrieved_doc
                .get_first(self.item_id_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let patron_id = retrieved_doc
                .get_first(self.patron_id_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let event_type = retrieved_doc
                .get_first(self.event_type_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let timestamp = retrieved_doc
                .get_first(self.timestamp_field)
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            results.push(SearchResult {
                item_id,
                patron_id,
                event_type,
                timestamp,
            });
        }

        Ok(results)
    }

    /// Returns a reference to the underlying Tantivy index.
    pub fn index(&self) -> &Index {
        &self.index
    }

    /// Returns a reference to the schema.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub item_id: String,
    pub patron_id: String,
    pub event_type: String,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use librery_core::{ActorId, ItemId};

    #[test]
    fn new_creates_empty_index() {
        let search_index = SearchIndex::new().unwrap();
        let reader = search_index.index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 0);
    }

    #[test]
    fn index_event_makes_document_searchable() {
        let search_index = SearchIndex::new().unwrap();
        let event = CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-1".into()),
            patron_id: ActorId("patron-1".into()),
            timestamp: 123,
            correlation_id: "corr-1".into(),
        };

        search_index.index_event(&event).unwrap();

        let reader = search_index.index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn rebuild_from_events_clears_and_reindexes() {
        let search_index = SearchIndex::new().unwrap();

        let initial_event = CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-old".into()),
            patron_id: ActorId("patron-old".into()),
            timestamp: 100,
            correlation_id: "corr-old".into(),
        };
        search_index.index_event(&initial_event).unwrap();

        let events = vec![
            CirculationEvent::ItemCheckedOut {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 123,
                correlation_id: "corr-1".into(),
            },
            CirculationEvent::ItemReturned {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 456,
                correlation_id: "corr-2".into(),
            },
        ];

        search_index.rebuild_from_events(&events).unwrap();

        let reader = search_index.index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 2);
    }

    #[test]
    fn rebuild_from_empty_events_clears_index() {
        let search_index = SearchIndex::new().unwrap();

        let event = CirculationEvent::ItemCheckedOut {
            item_id: ItemId("item-1".into()),
            patron_id: ActorId("patron-1".into()),
            timestamp: 123,
            correlation_id: "corr-1".into(),
        };
        search_index.index_event(&event).unwrap();

        search_index.rebuild_from_events(&[]).unwrap();

        let reader = search_index.index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 0);
    }

    #[test]
    fn all_event_types_are_indexed() {
        let search_index = SearchIndex::new().unwrap();

        let events = vec![
            CirculationEvent::ItemCheckedOut {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 1,
                correlation_id: "c1".into(),
            },
            CirculationEvent::ItemReturned {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 2,
                correlation_id: "c2".into(),
            },
            CirculationEvent::ItemRenewed {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 3,
                correlation_id: "c3".into(),
            },
            CirculationEvent::HoldPlaced {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 4,
                correlation_id: "c4".into(),
            },
            CirculationEvent::HoldCancelled {
                item_id: ItemId("item-1".into()),
                timestamp: 5,
                correlation_id: "c5".into(),
            },
            CirculationEvent::HoldFulfilled {
                item_id: ItemId("item-1".into()),
                patron_id: ActorId("patron-1".into()),
                timestamp: 6,
                correlation_id: "c6".into(),
            },
            CirculationEvent::ItemLost {
                item_id: ItemId("item-1".into()),
                timestamp: 7,
                correlation_id: "c7".into(),
            },
        ];

        search_index.rebuild_from_events(&events).unwrap();

        let reader = search_index.index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 7);
    }
}
