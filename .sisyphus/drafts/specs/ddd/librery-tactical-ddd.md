# Librery - Tactical DDD: Aggregates & Entities

> Generated: March 2026
> Purpose: Model aggregates, entities, and value objects within each bounded context

---

## CML Tactical Models

> Using Context Mapper tactical syntax (Sculptor-based)

```cml
// ============================================================
// CIRCULATION CONTEXT (Core)
// ============================================================

BoundedContext Circulation {
    type = APPLICATION
    domainVisionStatement = "Core physical state machine managing holds, transits, loans, and checkouts."
    responsibilities = "Checkout, return, renew, holds, loans, physical item state management"
    knowledgeLevel = CONCRETE

    Aggregate ItemAggregate {
        responsibilities = "Item lifecycle and status"
        knowledgeLevel = CONCRETE

        Entity Item {
            aggregateRoot
            - ItemId id
            - RecordId bibId
            - CallNumber callNumber
            - Barcode barcode
            - ItemStatus status
            - BranchId location

            Repository ItemRepository {
                @Item findById(@ItemId id)
                @Item findByBarcode(@Barcode barcode)
            }
        }

        ValueObject ItemId {
            UUID value key
        }

        ValueObject CallNumber {
            String value
        }

        ValueObject Barcode {
            String value key
        }

        enum ItemStatus {
            AVAILABLE, CHECKED_OUT, ON_HOLD_SHELF, LOST, IN_TRANSIT
        }

        DomainEvent ItemCheckedOut {
            - ItemId itemId
            - ActorId patronId
            - DateTime dueDate
        }

        DomainEvent ItemReturned {
            - ItemId itemId
            - ActorId patronId
        }

        Command Checkout {
            - ItemId itemId
            - ActorId patronId
        }

        Service CirculationService {
            @Item checkout(@Command cmd) : write [ AVAILABLE -> CHECKED_OUT ]
            @Item return(@Command cmd) : write [ CHECKED_OUT -> AVAILABLE ]
            @Item hold(@Command cmd) : write [ AVAILABLE -> ON_HOLD_SHELF ]
        }
    }

    Aggregate LoanAggregate {
        responsibilities = "Loan lifecycle and due dates"
        knowledgeLevel = CONCRETE

        Entity Loan {
            aggregateRoot
            - LoanId id
            - ItemId itemId
            - ActorId patronId
            - DateTime checkoutDate
            - DateTime dueDate
            - DateTime? returnDate
            - int renewalCount
            - LoanStatus status

            Repository LoanRepository {
                @Loan findById(@LoanId id)
                @Loan findActiveByPatron(@ActorId patronId)
            }
        }

        ValueObject LoanId {
            UUID value key
        }

        enum LoanStatus {
            aggregateLifecycle
            ACTIVE, RETURNED, OVERDUE, LOST
        }

        Command Renew {
            - LoanId loanId
        }

        Service LoanService {
            @Loan renew(@Command cmd) : write [ ACTIVE -> ACTIVE ]
        }
    }
}

// ============================================================
// CATALOG CONTEXT (Supporting)
// ============================================================

BoundedContext Catalog {
    type = APPLICATION
    domainVisionStatement = "Bibliographic metadata management supporting MARC21."
    responsibilities = "Bibliographic records, item metadata, MARC21 cataloging"
    knowledgeLevel = CONCRETE

    Aggregate BibliographicRecord {
        responsibilities = "Bibliographic record management"
        knowledgeLevel = CONCRETE

        Entity BibliographicRecord {
            aggregateRoot
            - RecordId id
            - String title
            - String author
            - String? isbn
            - String? isbn13
            - Marc21Record marc21

            Repository RecordRepository {
                @BibliographicRecord findById(@RecordId id)
                @BibliographicRecord findByISBN(@String isbn)
            }
        }

        ValueObject RecordId {
            UUID value key
        }

        ValueObject Marc21Record {
            String leader
            - List<MarcField> fields
        }

        DomainEvent RecordCreated {
            - RecordId recordId
        }
    }
}

// ============================================================
// ACTOR CONTEXT (Supporting)
// ============================================================

BoundedContext Actor {
    type = APPLICATION
    domainVisionStatement = "Unified actor management for Patron, Librarian, Admin, and Machine."
    responsibilities = "Authentication, authorization, capabilities, role-based access control"
    knowledgeLevel = CONCRETE

    Aggregate ActorAggregate {
        responsibilities = "Actor lifecycle and capabilities"
        knowledgeLevel = CONCRETE

        Entity Actor {
            aggregateRoot
            - ActorId id
            - ActorType actorType
            - String name
            - String? email
            - ActorStatus status
            - PatronTier? tier

            Repository ActorRepository {
                @Actor findById(@ActorId id)
                @Actor findByEmail(@String email)
            }
        }

        ValueObject ActorId {
            UUID value key
        }

        enum ActorType {
            PATRON, LIBRARIAN, ADMIN, MACHINE
        }

        enum ActorStatus {
            ACTIVE, SUSPENDED, BLOCKED, EXPIRED
        }

        enum PatronTier {
            GOLD, SILVER, BRONZE
        }

        ValueObject Capability {
            enum CapabilityType {
                SEARCH, VIEW_ACCOUNT, PLACE_HOLD,
                ISSUE_LOANS, MANAGE_CATALOG, VIEW_ANALYTICS,
                OVERRIDE_FINES, MANAGE_USERS, MANAGE_SYSTEM
            }
        }

        Service AuthService {
            boolean can(@ActorId actor, @Capability cap) : read-only
        }
    }
}

// ============================================================
// FINANCIAL CONTEXT (Supporting)
// ============================================================

BoundedContext Financial {
    type = APPLICATION
    domainVisionStatement = "Fine calculation and billing."
    responsibilities = "Fine calculation, lost item billing, payment tracking"
    knowledgeLevel = CONCRETE

    Aggregate FineAggregate {
        responsibilities = "Fine lifecycle"
        knowledgeLevel = CONCRETE

        Entity Fine {
            aggregateRoot
            - FineId id
            - ActorId patronId
            - Money amount
            - FineReason reason
            - FineStatus status

            Repository FineRepository {
                @Fine findById(@FineId id)
                @Fine findPendingByPatron(@ActorId patronId)
            }
        }

        ValueObject FineId {
            UUID value key
        }

        enum FineReason {
            OVERDUE_RETURN, LOST_ITEM, DAMAGED_ITEM, UNPAID_BALANCE
        }

        enum FineStatus {
            PENDING, PAID, WAIVED, SENT_TO_COLLECTION
        }

        DomainEvent FineApplied {
            - FineId fineId
            - ActorId patronId
            - Money amount
        }

        Service FineService {
            @Fine calculate(@ItemId itemId, @ActorId patronId) : read-only
        }
    }
}

// ============================================================
// SEARCH CONTEXT (Generic - Read Model Only)
// ============================================================

BoundedContext Search {
    type = APPLICATION
    domainVisionStatement = "CQRS read model for full-text search."
    responsibilities = "Full-text search, discovery, OPAC search interface"
    knowledgeLevel = CONCRETE

    // Search is a read model - no aggregates, just projection
    // Data projected from Circulation and Catalog contexts
}

// ============================================================
// INTEGRATION CONTEXT (Edge)
// ============================================================

BoundedContext Integration {
    type = APPLICATION
    domainVisionStatement = "Edge context hosting WASM plugins for external integrations."
    responsibilities = "SIP2 protocol, legacy migrations, external system integration"
    knowledgeLevel = CONCRETE

    Aggregate PluginAggregate {
        responsibilities = "Plugin lifecycle"
        knowledgeLevel = CONCRETE

        Entity Plugin {
            aggregateRoot
            - PluginId id
            - String name
            - PluginType pluginType
            - PluginStatus status

            Repository PluginRepository {
                @Plugin findById(@PluginId id)
                @Plugin findEnabled()
            }
        }

        ValueObject PluginId {
            UUID value key
        }

        enum PluginType {
            SIP2_ADAPTER, MARC_CONVERTER, SLIMS_MIGRATOR, CUSTOM
        }

        enum PluginStatus {
            INSTALLED, ENABLED, DISABLED, ERROR
        }
    }
}
```

---

## Legacy Reference: Rust Types (kept for reference)

> The above CML is the canonical tactical model. Below is the original Rust implementation reference.

---

## 1. Circulation Context (Core)

### Aggregates

#### ItemAggregate (Root)

```rust
// Value Objects
pub struct ItemId(pub Uuid);
pub struct BranchId(pub Uuid);

pub enum ItemStatus {
    Available { at_branch: BranchId },
    CheckedOut { to_patron: ActorId, due_date: DateTime<Utc> },
    OnHoldShelf { for_patron: ActorId, expires_at: DateTime<Utc> },
    Lost,
    InTransit { from: BranchId, to: BranchId },
}

pub struct Item {
    pub id: ItemId,
    pub bib_id: RecordId,
    pub call_number: CallNumber,
    pub barcode: Barcode,
    pub status: ItemStatus,
    pub location: BranchId,
}

// Commands
pub enum CirculationCommand {
    Checkout { item_id: ItemId, patron_id: ActorId },
    Return { item_id: ItemId },
    Renew { loan_id: LoanId },
    PlaceHold { item_id: ItemId, patron_id: ActorId },
    CancelHold { hold_id: HoldId },
    MarkLost { item_id: ItemId },
}

// Events
pub enum CirculationEvent {
    ItemCheckedOut { item_id: ItemId, patron_id: ActorId, due_date: DateTime<Utc> },
    ItemReturned { item_id: ItemId, patron_id: ActorId },
    LoanRenewed { loan_id: LoanId, new_due_date: DateTime<Utc> },
    HoldPlaced { item_id: ItemId, patron_id: ActorId, expires_at: DateTime<Utc> },
    HoldCancelled { hold_id: HoldId },
    ItemLost { item_id: ItemId },
    FineApplied { patron_id: ActorId, amount: Money, reason: FineReason },
}
```

#### LoanAggregate

```rust
pub struct LoanId(pub Uuid);

pub struct Loan {
    pub id: LoanId,
    pub item_id: ItemId,
    pub patron_id: ActorId,
    pub checkout_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub return_date: Option<DateTime<Utc>>,
    pub renewal_count: u32,
    pub status: LoanStatus,
}

pub enum LoanStatus {
    Active,
    Returned,
    Overdue,
    Lost,
}
```

---

## 2. Catalog Context (Supporting)

### Aggregates

#### BibliographicRecordAggregate (Root)

```rust
pub struct RecordId(pub Uuid);

pub struct BibliographicRecord {
    pub id: RecordId,
    pub marc21: Marc21Record,        // Full MARC21 XML
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub isbn13: Option<String>,
    pub publisher: Option<String>,
    pub pub_year: Option<i32>,
    pub subjects: Vec<String>,
    pub copies: Vec<ItemId>,         // Linked items
}

pub struct Marc21Record {
    pub leader: String,
    pub fields: Vec<MarcField>,
}
```

#### ItemAggregate

```rust
pub struct ItemId(pub Uuid);
pub struct CallNumber(pub String);
pub struct Barcode(pub String);

pub struct Item {
    pub id: ItemId,
    pub record_id: RecordId,
    pub call_number: CallNumber,
    pub barcode: Barcode,
    pub status: ItemStatus,
    pub location: BranchId,
    pub price: Option<Money>,
    pub acquisition_date: Option<DateTime<Utc>>,
}
```

---

## 3. Actor Context (Supporting)

### Aggregates

#### ActorAggregate (Root)

```rust
pub struct ActorId(pub Uuid);

pub enum ActorType {
    Patron,
    Librarian,
    Admin,
    Machine(TerminalId),
}

pub enum ActorStatus {
    Active,
    Suspended,
    Blocked,
    Expired,
}

pub struct Actor {
    pub id: ActorId,
    pub actor_type: ActorType,
    pub name: String,
    pub email: Option<String>,
    pub status: ActorStatus,
    pub tier: Option<PatronTier>,        // For patrons
    pub privileges: Vec<Privilege>,      // For staff
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub enum PatronTier {
    Gold,    // 21 day loan, 20 items
    Silver,  // 14 day loan, 10 items
    Bronze,  // 7 day loan, 5 items
}
```

#### Capability & Role

```rust
pub enum Capability {
    // Patron
    Search,
    ViewAccount,
    PlaceHold,
    
    // Librarian
    IssueLoans,
    ManageCatalog,
    ViewAnalytics,
    OverrideFines { max_amount: u32 },
    
    // Admin
    ManageUsers,
    ManageSystem,
    ManageFederation,
}

pub struct AuthContext {
    pub actor_id: ActorId,
    pub actor_type: ActorType,
    pub capabilities: Vec<Capability>,
}
```

---

## 4. Financial Context (Supporting)

### Aggregates

#### FineAggregate (Root)

```rust
pub struct FineId(pub Uuid);

pub enum FineReason {
    OverdueReturn,
    LostItem,
    DamagedItem,
    UnpaidBalance,
}

pub enum FineStatus {
    Pending,
    Paid,
    Waived,
    SentToCollection,
}

pub struct Fine {
    pub id: FineId,
    pub patron_id: ActorId,
    pub item_id: Option<ItemId>,
    pub loan_id: Option<LoanId>,
    pub amount: Money,
    pub reason: FineReason,
    pub status: FineStatus,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
}
```

#### PaymentAggregate

```rust
pub struct PaymentId(pub Uuid);

pub enum PaymentMethod {
    Cash,
    Card,
    QRIS,           // Indonesian mobile payment
    BankTransfer,
}

pub struct Payment {
    pub id: PaymentId,
    pub fine_id: FineId,
    pub amount: Money,
    pub method: PaymentMethod,
    pub status: PaymentStatus,
    pub transaction_ref: Option<String>,
    pub paid_at: DateTime<Utc>,
}
```

---

## 5. Search Context (Generic)

### Read Model (Not an Aggregate)

```rust
// This is a CQRS read model - not an aggregate
// Data is projected from Circulation and Catalog

pub struct SearchDocument {
    pub item_id: ItemId,
    pub record_id: RecordId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub call_number: String,
    pub status: String,           // "Available", "Checked Out"
    pub location: String,
    pub barcode: String,
    pub score: f32,               // Search relevance
}

pub struct SearchResult {
    pub document: SearchDocument,
    pub highlights: Vec<String>,
}
```

---

## 6. Integration Context (Edge)

### Aggregates

#### PluginAggregate (Root)

```rust
pub struct PluginId(pub Uuid);

pub enum PluginType {
    Sip2Adapter,
    MarcConverter,
    SlimsMigrator,
    Custom(String),
}

pub enum PluginStatus {
    Installed,
    Enabled,
    Disabled,
    Error,
}

pub struct Plugin {
    pub id: PluginId,
    pub name: String,
    pub plugin_type: PluginType,
    pub version: String,
    pub wasm_path: String,
    pub status: PluginStatus,
    pub capabilities: Vec<PluginCapability>,
    pub loaded_at: Option<DateTime<Utc>>,
}
```

#### SIP2Session

```rust
pub struct TerminalId(pub String);

pub struct SIP2Session {
    pub terminal_id: TerminalId,
    pub institution_id: Option<String>,
    pub patron_id: Option<ActorId>,
    pub status: SessionStatus,
    pub connected_at: DateTime<Utc>,
}
```

---

## Entity Relationship Summary

```
Circulation
├── Item (Aggregate Root)
│   └── ItemStatus (Value Object)
├── Loan (Aggregate Root)
│   └── LoanStatus (Value Object)
└── Hold (Entity)

Catalog
├── BibliographicRecord (Aggregate Root)
│   └── Marc21Record (Value Object)
└── Item (Aggregate Root)

Actor
├── Actor (Aggregate Root)
│   ├── ActorType (Value Object)
│   └── PatronTier (Value Object)
└── Capability (Value Object)

Financial
├── Fine (Aggregate Root)
│   └── FineReason (Value Object)
└── Payment (Aggregate Root)

Search
└── SearchDocument (Read Model - no aggregate)

Integration
├── Plugin (Aggregate Root)
└── SIP2Session (Entity)
```

---

## Value Objects Summary

| Context | Value Objects |
|---------|---------------|
| **Circulation** | ItemId, LoanId, HoldId, BranchId, Money, DateTime |
| **Catalog** | RecordId, CallNumber, Barcode, MarcField |
| **Actor** | ActorId, TerminalId, PatronTier, Capability |
| **Financial** | FineId, PaymentId, FineReason, PaymentMethod |
| **Search** | SearchQuery, SearchResult, SearchDocument |

---

*Document Version: 1.2*
*Generated: March 2026*
*Note: CML tactical model above is canonical. Rust types are reference implementation.*