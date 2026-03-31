# Librery - FCIS Boundaries by Bounded Context

> Generated: March 2026
> Purpose: Define Pure Functional Core vs Imperative Shell for each BC

---

## Guiding Principles

| Layer | Characteristics |
|-------|-----------------|
| **Functional Core (Pure)** | No side effects, deterministic, testable without mocks |
| **Imperative Shell** | Database I/O, network, external services, time/Uuid generation |

---

## 1. Circulation Context (Core)

### Functional Core (Pure)

```rust
// Commands (Intents) - injected with non-deterministic data
pub struct CheckoutCommand {
    pub item_id: ItemId,
    pub patron_id: ActorId,
    pub timestamp: DateTime<Utc>,    // Injected by Shell
    pub correlation_id: Uuid,        // Injected by Shell
}

pub struct ReturnCommand { /* ... */ }
pub struct RenewCommand { /* ... */ }
pub struct HoldCommand { /* ... */ }

// Events (Facts) - immutable ledger entries
pub enum CirculationEvent {
    ItemCheckedOut { item_id: ItemId, patron_id: ActorId, due_date: DateTime<Utc> },
    ItemReturned { item_id: ItemId, patron_id: ActorId },
    LoanRenewed { loan_id: LoanId, new_due_date: DateTime<Utc> },
    HoldPlaced { item_id: ItemId, patron_id: ActorId },
    FineApplied { patron_id: ActorId, amount: Money, reason: FineReason },
}

// Aggregate - pure state machine
pub struct CirculationAggregate {
    pub item: Item,
    pub loan_history: Vec<Loan>,
    pub holds: Vec<Hold>,
}

impl CirculationAggregate {
    // Pure function - no side effects
    pub fn process(&self, cmd: CheckoutCommand) -> Result<Vec<CirculationEvent>, CheckoutError> {
        // All validation logic here
        if self.item.status == ItemStatus::CheckedOut {
            return Err(CheckoutError::ItemNotAvailable);
        }
        // ... more validation
        Ok(vec![CirculationEvent::ItemCheckedOut { ... }])
    }
    
    // Pure function - state reconstruction
    pub fn rebuild(events: &[CirculationEvent]) -> Self { /* ... */ }
}
```

### Imperative Shell

| Responsibility | Description |
|----------------|-------------|
| **SQLx Ledger** | Append events to PostgreSQL/SQLite |
| **MPSC Actor** | Ensure strict-ordered, lock-free event writing |
| **Timestamp Injection** | Generate `DateTime<Utc>` and inject into commands |
| **UUID Generation** | Generate correlation IDs |
| **Event Publishing** | Emit events to Financial context |

---

## 2. Catalog Context (Supporting)

### Functional Core (Pure)

```rust
// Bibliographic record - immutable
pub struct BibRecord {
    pub id: RecordId,
    pub marc21: Marc21Record,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
}

// Item - physical copy
pub struct Item {
    pub id: ItemId,
    pub bib_id: RecordId,
    pub call_number: CallNumber,
    pub status: ItemStatus,
    pub location: BranchId,
}

// CRDT operations - pure merge logic
pub fn merge_catalog_updates(local: &BibRecord, remote: &BibRecord) -> BibRecord {
    // Pure CRDT merge - no I/O
}
```

### Imperative Shell

| Responsibility | Description |
|----------------|-------------|
| **CRDT Sync** | Automerge for offline collaborative editing |
| **MARC Import** | Parse MARC21 files |
| **Database** | SQLx persistence |
| **WASM Plugin Host** | Load cataloging plugins |

---

## 3. Actor Context (Supporting)

### Functional Core (Pure)

```rust
// Actor types
pub enum ActorType {
    Patron,
    Librarian,
    Admin,
    Machine(TerminalId),
}

// Capabilities - authorization model
pub enum Capability {
    IssueLoans,
    ManageCatalog,
    OverrideFines { max_amount: u32 },
    ViewAnalytics,
    // ...
}

// Role = collection of capabilities
pub fn role_capabilities(role: &ActorType) -> Vec<Capability> {
    match role {
        ActorType::Patron => vec![Capability::Search, Capability::ViewAccount],
        ActorType::Librarian => vec![Capability::IssueLoans, Capability::ManageCatalog],
        ActorType::Admin => vec![/* all */],
        ActorType::Machine(_) => vec![/* limited */],
    }
}

// Auth context - pure evaluation
pub fn evaluate_access(auth: &AuthContext, capability: &Capability) -> bool {
    auth.capabilities.contains(capability)
}
```

### Imperative Shell

| Responsibility | Description |
|----------------|-------------|
| **Biscuit Token** | Verify token signature, extract capabilities |
| **Session Management** | HttpOnly cookie handling |
| **Actor Repository** | SQLx persistence |
| **Public Key Cache** | Cache offline verification keys |

---

## 4. Financial Context (Supporting)

### Functional Core (Pure)

```rust
// Fine calculation - pure math
pub fn calculate_overdue_fine(overdue_days: u32, daily_rate: Money) -> Money {
    daily_rate * overdue_days as i64
}

// Fine matrix - pure validation
pub fn apply_fine_rules(patron_tier: &Tier, event: &CirculationEvent) -> Option<Money> {
    match event {
        CirculationEvent::ItemReturnedLate { days_late } => {
            let rate = match patron_tier {
                Tier::Gold => Money::new(50),  // $0.50/day
                Tier::Silver => Money::new(100),
                Tier::Bronze => Money::new(200),
            };
            Some(calculate_overdue_fine(*days_late, rate))
        }
        _ => None,
    }
}
```

### Imperative Shell

| Responsibility | Description |
|----------------|-------------|
| **Event Subscription** | Listen to Circulation events |
| **Payment Gateway** | External payment API calls |
| **Notification Trigger** | Send overdue notices |
| **Ledger Persistence** | SQLx fine records |

---

## 5. Search Context (Generic)

### Functional Core (Pure)

```rust
// Search query - pure transformation
pub struct SearchQuery {
    pub terms: Vec<String>,
    pub filters: Vec<SearchFilter>,
    pub sort: SortOption,
}

pub fn build_tantivy_query(query: &SearchQuery) -> tantivy::Query {
    // Pure query construction
}

// Result ranking - pure
pub fn rank_results(results: &[SearchResult], query: &SearchQuery) -> Vec<SearchResult> {
    // Pure ranking logic
}
```

### Imperative Shell

| Responsibility | Description |
|----------------|-------------|
| **Tantivy Index** | Full-text search index management |
| **Index Projection** | Background worker: SQLx → Tantivy |
| **Query Execution** | Tantivy search execution |
| **Cache** | Moka cache for frequent queries |

---

## 6. Integration Context (Edge)

### Functional Core (Pure)

```rust
// Protocol translation - pure transformation
pub fn translate_sip2_to_command(msg: &SIP2Message) -> Result<Command, TranslationError> {
    // Pure translation logic
}

pub fn translate_command_to_sip2(event: &Event) -> Result<SIP2Message, TranslationError> {
    // Pure translation logic
}
```

### Imperative Shell

| Responsibility | Description |
|----------------|-------------|
| **Extism Runtime** | WASM plugin execution |
| **SIP2 Handler** | TCP/IP socket handling |
| **Plugin Loading** | Load/verify WASM plugins |
| **ACL Enforcement** | Validate plugin capabilities |

---

## Summary: FCIS Split by Context

| Context | Core (Pure) | Shell (Effects) |
|---------|-------------|-----------------|
| **Circulation** | Commands → Events, Aggregate rebuild, Validation | SQLx persistence, MPSC actor, Timestamp injection |
| **Catalog** | BibRecord, Item, CRDT merge | Automerge sync, MARC import, SQLx |
| **Actor** | Capabilities, Role resolution, Access evaluation | Biscuit verification, Session, SQLx |
| **Financial** | Fine calculation, Rule application | Event subscription, Payment API, Notifications |
| **Search** | Query building, Result ranking | Tantivy index, Index projection, Cache |
| **Integration** | Protocol translation | Extism runtime, SIP2 sockets, Plugin loading |

---

## Key Ports (Interfaces)

```rust
// Database port
pub trait CirculationRepository {
    async fn append_event(&self, event: CirculationEvent) -> Result<(), DbError>;
    async fn get_events(&self, item_id: ItemId) -> Result<Vec<CirculationEvent>, DbError>;
}

// Event bus port
pub trait EventPublisher {
    async fn publish(&self, event: CirculationEvent) -> Result<(), EventError>;
}

// Search index port
pub trait SearchIndex {
    async fn index(&self, doc: SearchDocument) -> Result<(), IndexError>;
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, IndexError>;
}
```

---

## Appendix: Boundary Classification Checklist

> Source: fcis-orchestrating-workflows/reference/boundary-classification-checklist.md

Use this checklist to classify components into Functional Core or Imperative Shell.

### Classification Rules

| Classification | Description |
|----------------|-------------|
| **core** | Deterministic decision logic with no direct side effects |
| **shell** | Orchestration, I/O, external interaction, runtime policy handling |
| **shared contract** | Minimal data contracts used to connect shell and core |

### Checklist

| Question | Answer |
|----------|--------|
| Does this component make business decisions? | → Core |
| Does it call database, network, file system, or process APIs? | → Shell |
| Does it read time, randomness, environment, or global mutable state? | → Shell |
| Can it run in memory with deterministic inputs only? | → Core |
| Does it depend on framework transport types? | → Shell |
| Is retry/timeout/circuit-breaker logic present? | → Shell |
| Is transaction coordination present? | → Shell |
| Could this component be tested without mocks or infrastructure? | → Core |

### Decision Cues

- **Mostly "Yes" on decision + deterministic** → `core` candidate
- **Mostly "Yes" on effect and runtime policy** → `shell` candidate
- **Mixed signals** → Split by seam and reassess

### Common Misclassifications

- Validation plus DB lookup in one function
- Domain rules embedded in API middleware
- Transaction logic hidden in repositories
- Core functions accepting framework request/response objects

### Definition of Done

- [ ] Each target component has a declared role
- [ ] Dependency direction is explicit (`shell → core`)
- [ ] One incremental extraction slice is identified

---

## Appendix B: FCIS Workflow & Heuristics

> Source: fcis-separating-core-and-shell skill

### Workflow

1. **Inventory** current responsibilities (domain decisions, orchestration, I/O, framework coupling)
2. **Classify** each component using effect exposure and decision density
3. **Mark** boundary seams where pure decision logic can be extracted
4. **Define** target package/module split:
   - `core` modules with no runtime infrastructure dependencies
   - `shell` modules for adapters, transport, persistence, clock, randomness
5. **Introduce** explicit contracts between shell and core
6. **Verify** dependency direction (shell → core, never core → shell)
7. **Propose** one incremental extraction slice

### Boundary Heuristics

| Pattern | Classification |
|---------|---------------|
| High decision complexity + low I/O needs | → Functional Core |
| Heavy coordination, retries, transactions, API calls | → Imperative Shell |
| Shared schema/transport types | → Keep minimal and stable |

### Guardrails

| Rule | Description |
|------|-------------|
| No domain in adapters | Don't move domain decisions into adapters for convenience |
| No framework in core | Don't leak framework or transport types into core signatures |
| Thin shell | Avoid giant shell orchestrators; keep coordination explicit |
| Capture unknowns | Record unresolved classifications as assumptions |

### First Extraction Slice (for Librery)

Given the FCIS architecture, the recommended first extraction slice is:

1. **Extract Circulation Core First**
   - Pure command handlers (`process_checkout`, `process_return`)
   - Event generation logic
   - No database or time dependencies in core functions

2. **Rationale**
   - Lowest blast radius
   - Most testable (Chicago TDD)
   - Foundation for other contexts

---

*Document Version: 1.2*
*Generated: March 2026*