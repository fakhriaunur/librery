# Librery Phase 1 Planning

> Draft | Target: Expanding the Ledger & CQRS Search

---

## Phase 0 Completed

- ✅ Checkout command (ItemAggregate)
- ✅ Event-sourced ledger (CirculationEvent)
- ✅ HTTP/3 + HTTP/2 server
- ✅ OpenAPI schema
- ✅ Ratatui TUI

---

## Phase 1 Scope (from tracer-bullets-updated.md)

### Milestone 1.1: Complete Circulation
- ItemAggregate: Check-in, Renew, Lost, Holds
- cargo-mutants (release-gate) for test suite strength

### Milestone 1.2: MPSC Actor
- Tokio MPSC Actor channel in Axum for lock-free, strict-ordered event writing
- Dead letter queue for failures

### Milestone 1.3: Search Index
- Tantivy embedded
- Background worker projects SQLx inserts → Tantivy
- Handle eventual consistency (index lag)

---

## Key Decisions from Tracer Bullets

- **Event ordering**: MPSC ensures chronological processing
- **Search lag**: OPAC must handle "item shows available but just returned" — add timestamp to search results
- **Adapter pattern**: Continue SQLx abstraction for PostgreSQL migration later

---

## Questions for User

1. Should we follow the tracer bullets exactly (1.1 → 1.2 → 1.3)?
2. Any priorities or modifications to the milestones?
3. Test strategy: cargo-mutants for mutation testing - is this acceptable?