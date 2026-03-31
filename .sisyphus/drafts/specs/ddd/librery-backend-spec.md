# Librery - Backend Architecture Specification

> Generated: March 2026
> Purpose: Axum shell, SQLx schema, HTTP/3, Utoipa contracts

---

## 1. Project Structure

```
librery/
├── librery-core/          # Pure functional core (no dependencies)
│   └── src/
│       ├── domain/        # Entities, Value Objects, Commands, Events
│       │   ├── circulation/
│       │   ├── catalog/
│       │   ├── actor/
│       │   └── financial/
│       └── lib.rs
│
├── librery-shell/         # Imperative shell (Axum, SQLx)
│   └── src/
│       ├── api/           # HTTP handlers
│       ├── persistence/   # SQLx repositories
│       ├── http3/         # Quinn + Hyper setup
│       └── main.rs
│
├── librery-tui/           # Ratatui terminal
├── librery-opac/          # SvelteKit (future)
└── librery-staff/         # Flutter (future)
```

---

## 2. SQLx Schema

### Tables

```sql
-- Circulation Events (Immutable Ledger)
CREATE TABLE circulation_events (
    id              UUID PRIMARY KEY,
    aggregate_id    UUID NOT NULL,
    aggregate_type  TEXT NOT NULL,  -- 'Item', 'Loan'
    event_type      TEXT NOT NULL,  -- 'ItemCheckedOut', 'ItemReturned'
    payload         JSONB NOT NULL, -- Event data
    timestamp       TIMESTAMPTZ NOT NULL,
    correlation_id  UUID NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_aggregate_id ON circulation_events(aggregate_id);

-- Actors
CREATE TABLE actors (
    id              UUID PRIMARY KEY,
    actor_type      TEXT NOT NULL,  -- 'Patron', 'Librarian', 'Admin', 'Machine'
    name            TEXT NOT NULL,
    email           TEXT UNIQUE,
    status          TEXT NOT NULL,  -- 'Active', 'Suspended', 'Blocked'
    tier            TEXT,           -- 'Gold', 'Silver', 'Bronze' (for Patrons)
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

-- Bibliographic Records
CREATE TABLE bib_records (
    id              UUID PRIMARY KEY,
    title           TEXT NOT NULL,
    author          TEXT,
    isbn            TEXT,
    marc21          JSONB,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

-- Items
CREATE TABLE items (
    id              UUID PRIMARY KEY,
    bib_record_id   UUID REFERENCES bib_records(id),
    call_number     TEXT NOT NULL,
    barcode         TEXT UNIQUE NOT NULL,
    status          TEXT NOT NULL,  -- 'Available', 'CheckedOut', 'OnHold', 'Lost'
    location        TEXT NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

-- Fines
CREATE TABLE fines (
    id              UUID PRIMARY KEY,
    patron_id       UUID REFERENCES actors(id),
    amount          INTEGER NOT NULL,  -- cents
    reason          TEXT NOT NULL,
    status          TEXT NOT NULL,  -- 'Pending', 'Paid', 'Waived'
    created_at      TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 3. HTTP API Routes

### Base Path: `/api/v1`

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| **Checkout** | | | |
| POST | `/circulation/checkout` | `checkout_item` | Check out item to patron |
| POST | `/circulation/return` | `return_item` | Return item |
| POST | `/circulation/renew` | `renew_loan` | Renew loan |
| **Catalog** | | | |
| GET | `/catalog/items` | `list_items` | List items |
| GET | `/catalog/items/:id` | `get_item` | Get item by ID |
| GET | `/catalog/search` | `search` | Search catalog |
| **Actor** | | | |
| GET | `/actor/me` | `get_current_actor` | Get authenticated actor |
| POST | `/actor/login` | `login` | Login (Biscuit token) |
| **Financial** | | | |
| GET | `/financial/fines` | `list_fines` | List patron fines |
| POST | `/financial/pay` | `pay_fine` | Pay fine |

---

## 4. HTTP/3 Configuration

### Server Setup (Quinn + Hyper)

```rust
use axum::Server;
use hyper::service::service_fn;
use quinn::{Endpoint, Runtime};

async fn run_server() {
    // HTTP/3 (UDP)
    let mut endpoint = Endpoint::client("0.0.0.0:8080".parse().unwrap()).unwrap();
    let connection = endpoint.connect("0.0.0.0:8080".parse().unwrap(), "librery".unwrap()).await.unwrap();
    
    // HTTP/1.1 + HTTP/2 (TCP fallback)
    let tcp_server = Server::bind(([0, 0, 0, 0], 8080)).await;
    
    // Alt-Svc header for protocol upgrade
    let response = Response::builder()
        .header("alt-svc", "h3=\":8080\"")
        .body(Body::empty())
        .unwrap();
}
```

---

## 5. Utoipa OpenAPI Schema

### Generation

```rust
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::circulation::checkout_item,
        api::circulation::return_item,
        api::catalog::search,
        api::actor::login,
    ),
    components(
        schemas(
            CheckoutRequest,
            CheckoutResponse,
            Item,
            Actor,
            Fine,
        )
    )
)]
pub struct LibreryApi;
```

### Contract Example

```rust
#[derive(ToSchema)]
pub struct CheckoutRequest {
    pub item_id: Uuid,
    pub patron_id: Uuid,
}

#[derive(ToSchema)]
pub struct CheckoutResponse {
    pub success: bool,
    pub event_id: Uuid,
    pub due_date: DateTime<Utc>,
}
```

---

## 6. Shell → Core Communication

### Ports (Traits)

```rust
// Database port
pub trait CirculationRepository {
    async fn append_event(&self, event: CirculationEvent) -> Result<(), DbError>;
    async fn get_events(&self, aggregate_id: Uuid) -> Result<Vec<CirculationEvent>, DbError>;
}

// Event bus port
pub trait EventPublisher {
    async fn publish(&self, event: CirculationEvent) -> Result<(), EventError>;
}

// Search index port
pub trait SearchIndex {
    async fn index(&self, doc: SearchDocument) -> Result<(), IndexError>;
}
```

### Command Flow

```
Shell receives HTTP request
    ↓
Injects timestamp/correlation_id into Command
    ↓
Calls Core.process(Command) → Result<Vec<Event>, Error>
    ↓
Shell persists Events via Repository
    ↓
Shell publishes Events via EventPublisher
    ↓
Returns response to client
```

---

## 7. Health Check

```rust
#[utoipa::get("/health")]
async fn health() -> Json<HealthStatus> {
    Json(HealthStatus {
        database: "healthy",
        http3: "healthy",
    })
}
```

---

*Document Version: 1.0*
*Generated: March 2026*