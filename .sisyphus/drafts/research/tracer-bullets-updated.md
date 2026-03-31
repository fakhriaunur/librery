# Librery: Refined Milestones & Tracer Bullets

> Updated: March 2026
> Based on feedback from Prometheus review

---

## Core Philosophy: Tracer Bullets

Instead of building 100% of the database, then 100% of the backend, and then 100% of the frontend, Librery is developed in vertical slices. We fire a "tracer bullet" through the entire stack to validate the FCIS architecture, CQRS, network protocols, and Monorepo setup early.

Every phase enforces the **Anti-Duplication Mandate**: Domain logic is written once in Rust.

---

## Phase 0: The First Tracer Bullet

**Goal**: Prove the Functional Core, Axum Shell, SQLx Ledger, HTTP/3 transport, and Ratatui TUI can communicate seamlessly.

**The Slice**: A librarian checks out a single, hardcoded book to a single patron via the terminal.

### Milestones

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **0.0** | Hello World | Minimal test passes (verify test infra works) |
| **0.1** | Pure Core | ItemAggregate + CheckOut command. Chicago TDD + proptest for invariants |
| **0.2** | Shell/DB/Network | SQLite via SQLx (adapter pattern for PostgreSQL). Axum POST /api/v1/checkout. HTTP/3 (quinn) + HTTP/2 fallback (hyper). Alt-Svc header injection. Health check endpoint |
| **0.3** | Contract | Utoipa generates OpenAPI 3.1 schema at compile time |
| **0.4** | TUI & Client | Minimal Ratatui screen. Native Rust reqwest (HTTP/3) hits Axum backend |

### Success Criteria

Typing an ID in the terminal sends an HTTP/3 request → Immutable ItemCheckedOut event stored in SQLite ledger → Verified via query.

### Key Decisions

- **Database**: SQLx with adapter pattern — works for both SQLite (edge) and PostgreSQL (consortia)
- **Network**: HTTP/3 UDP + HTTP/2 fallback — universal compatibility
- **Test**: Start with trivial test to verify test framework before domain logic

---

## Phase 1: Expanding the Ledger & CQRS Search

**Goal**: Complete the Circulation state machine and project it into the search index.

### Milestones

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **1.1** | Complete Circulation | ItemAggregate: Check-in, Renew, Lost, Holds. cargo-mutants (release-gate) for test suite strength |
| **1.2** | MPSC Actor | Tokio MPSC Actor channel in Axum for lock-free, strict-ordered event writing. Include dead letter queue for failures |
| **1.3** | Search Index | Tantivy embedded. Background worker projects SQLx inserts → Tantivy. Handle eventual consistency (index lag) |

### Key Decisions

- **Event ordering**: MPSC ensures chronological processing
- **Search lag**: OPAC must handle "item shows available but just returned" — add timestamp to search results
- **Adapter pattern**: Continue SQLx abstraction for PostgreSQL migration later

---

## Phase 2: The Unified Patron App

**Goal**: Deliver a lightning-fast web experience for the public, unifying discovery and account management.

### Milestones

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **2.1** | SvelteKit Setup | SvelteKit workspace. Justfile automates openapi-generator-cli for TypeScript SDK |
| **2.2** | OPAC Search | Search bar → Tantivy. Browser native fetch API. HTTP/3 upgrade via Alt-Svc |
| **2.3** | Auth Flow | Axum issues Biscuit Token in HttpOnly, Secure cookie. Patron Dashboard as protected route. **Note**: Patron OPAC is online-only (browser); offline verification is Staff App only |

### QA Gates

- **Component**: vitest + @testing-library/svelte for UI components
- **E2E**: Playwright script — patron searches Tantivy → logs in → places hold

---

## Phase 3: The Universal Staff App & Offline CRDTs

**Goal**: Empower librarians with rich data visualization and offline-capable cataloging.

### Milestones

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **3.1** | Flutter Init | Flutter workspace. flutter_rust_bridge (FRB) compiles librery-core into Dart |
| **3.2** | Rust-Driven Networking | All Flutter API calls route through embedded Rust core. reqwest with HTTP/3 UDP, fallback HTTP/2 |
| **3.3** | Offline Sync | Local SQLite in Rust core queues Circulation commands. Automerge + autosurgeon for offline metadata editing |
| **3.4** | Dashboard | Executive dashboard in Flutter (fl_chart) visualizes CQRS read models |

### QA Gates

- **Widget**: Flutter Widget Tests — chart rendering with mocked offline state
- **Integration**: Flutter Integration Tests — offline command queue syncs to Axum over HTTP/3 when network restores

---

## Phase 4: Extensibility & Migration

**Goal**: Open the system to legacy migrations without compromising the core.

### Milestones

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **4.1** | Extism Runtime | Embed Extism WASM runtime into Axum shell via PluginRuntime trait |
| **4.2** | CSV Import | Simple CSV importer (deferred from SLiMS migrator). CSV → Librery schema. Add rollback/validation |
| **4.3** | SLiMS Migrator | slims-migrator.wasm (lower priority, after CSV works). Legacy MySQL → Librery events |
| **4.4** | Plugin Security | Capability-based manifests for sideloaded plugins. Zero-trust sandbox |

### Key Decisions

- **CSV over SLiMS**: Simpler — just map CSV columns to Librery schema. Faster time-to-value
- **Rollback**: Add cleanup/validation step for failed migrations

---

## Phase 5: Edge Deployment & Load Testing

**Goal**: Real-world infrastructural stress testing in constrained environments.

### Milestones

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **5.1** | Containerfile | Single static Librery binary (Axum + Extism + SvelteKit static assets). Scratch-based OCI image |
| **5.2** | Infrastructure | OpenTofu provisions MicroShift edge cluster. Podman for daemonless containers |
| **5.3** | Load Testing | k6 tests targeting HTTP/3 UDP multiplexing + MPSC checkout queues. Prove single binary handles school-hour traffic |

### QA Gates

- **Load Test**: No memory panics, no head-of-line blocking under concurrent load

---

## Summary: Phase Timeline

| Phase | Focus | Expected Duration |
|-------|-------|-------------------|
| **Phase 0** | First Tracer Bullet | **Couple of days** (max) |
| Phase 1 | Ledger + Search | 1-2 weeks |
| Phase 2 | Patron OPAC | 1-2 weeks |
| Phase 3 | Staff App + Offline | 2-3 weeks |
| Phase 4 | Extensibility | 1-2 weeks |
| Phase 5 | Edge Deployment | 1 week |

---

## Key Decisions Log

| Decision | Rationale |
|----------|-----------|
| **SQLx adapter pattern** | Works for both SQLite (edge) and PostgreSQL (consortia) in parallel |
| **CSV import before SLiMS migrator** | Simpler — just column mapping. Faster time-to-value |
| **Phase 0 in couple of days** | Minimal viable slice — just checkout via terminal |
| **Biscuit offline for Staff App only** | Patron OPAC is browser-based (online). Staff App has offline requirement |
| **MPSC with dead letter queue** | Ensure no events lost if actor fails |

---

*Document Version: 1.1*
*Last Updated: March 2026*