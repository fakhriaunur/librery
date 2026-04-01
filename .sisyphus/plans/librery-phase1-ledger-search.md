# Librery Phase 1: Expanding the Ledger & CQRS Search

> Work Plan | Target: Complete circulation state machine + search index
> Generated: April 2026

---

## TL;DR

> **Quick Summary**: Complete the Circulation state machine (check-in, renew, lost, holds), add MPSC actor for ordered event processing, and integrate Tantivy search index.

> **Deliverables**:
> - Complete circulation: Check-in, Renew, Lost, Holds commands
> - MPSC actor with dead letter queue
> - Tantivy search index with background projection
> - Mutation testing via cargo-mutants

> **Estimated Effort**: 1-2 weeks
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Circulation → MPSC → Search

---

## Context

### Phase 0 Completed
- ✅ Checkout command (ItemAggregate)
- ✅ Event-sourced ledger (CirculationEvent)
- ✅ HTTP/3 + HTTP/2 server
- ✅ OpenAPI schema
- ✅ Ratatui TUI

### Original Request
Build Librery, a next-generation library management system using FCIS architecture in Rust.

### Tracer Bullets Reference
Phase 1 scope defined in `.sisyphus/drafts/research/tracer-bullets-updated.md`

### Metis Review (April 2026)
**Identified Gaps** (addressed in this plan):
- Missing acceptance criteria for specific error types
- Event store backend verification needed before MPSC
- Hold lifecycle business rules undefined
- Tantivy rebuild strategy not specified

**Guardrails Applied**:
- `process()` must remain pure — no I/O, no actor references
- Events are immutable once persisted
- Tantivy is eventually consistent — no synchronous write-then-search
- No cross-context coupling (Financial, Catalog, Integration)
- cargo-mutants runs on `librery-core` only

**Scope Creep Locked Down**:
- No patron validation (use stub/mock)
- No metrics/observability dashboards
- No persistent DLQ (in-memory retry only)
- No concurrency tests for MPSC (property tests only)

---

## Work Objectives

### Core Objective
Complete the circulation state machine and project it into the search index for OPAC discovery.

### Concrete Deliverables
- [ ] 1.1a: Check-in command (return item)
- [ ] 1.1b: Renew command (extend loan)
- [ ] 1.1c: Mark Lost command
- [ ] 1.1d: Hold management (place, cancel, fulfill)
- [ ] 1.1e: cargo-mutants for test strength
- [ ] 1.2a: MPSC actor channel in Axum
- [ ] 1.2b: Dead letter queue for failed events
- [ ] 1.3a: Tantivy integration
- [ ] 1.3b: Background worker for SQLx → Tantivy projection
- [ ] 1.3c: Search endpoint with timestamp handling

### Definition of Done
- [ ] All circulation commands work: checkout, check-in, renew, lost, holds
- [ ] Events processed in strict order via MPSC
- [ ] Failed events go to dead letter queue
- [ ] Tantivy index receives events from SQLx
- [ ] Search results include timestamp for eventual consistency
- [ ] cargo-mutants passes at release gate

### Must Have
- Strict event ordering via MPSC
- Dead letter queue for failure handling (in-memory, max 3 retries)
- Eventual consistency acknowledged in search results
- SQLx adapter pattern maintained
- All 6 command types defined with explicit validation rules

### Must NOT Have
- No authentication in this phase (deferred to Phase 2)
- No PostgreSQL (SQLite only for now)
- No external search services (embedded Tantivy only)
- No cross-context dependencies (Financial, Catalog, Integration)
- No persistent DLQ (in-memory retry with bounded attempts only)

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: YES (TDD + mutation testing)
- **Framework**: Rust built-in `#[test]` + cargo-mutants
- **If TDD**: Each task follows RED → GREEN → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — circulation completion):
├── T1: Check-in command + event
├── T2: Renew command + event
├── T3: Mark Lost command + event
├── T4: Hold management (place, cancel, fulfill)
├── T5: cargo-mutants integration
└── T6: Unit tests for all circulation commands

Wave 2 (After Wave 1 — MPSC + Search):
├── T7: MPSC actor channel in Axum
├── T8: Dead letter queue implementation
├── T9: Tantivy integration
├── T10: Background worker (SQLx → Tantivy)
├── T11: Search endpoint with timestamps
└── T12: Integration test (full search flow)
```

### Dependency Matrix

| Task | Blocks | Blocked By |
|------|--------|------------|
| T1 | T2 | — |
| T2 | T3 | T1 |
| T3 | T4 | T2 |
| T4 | T5 | T3 |
| T5 | T6 | T4 |
| T6 | T7 | T5 |
| T7 | T8 | T6 |
| T8 | T9 | T7 |
| T9 | T10 | T8 |
| T10 | T11 | T9 |
| T11 | T12 | T10 |
| T12 | — | T11 |

### Critical Path Note
**Verify event store exists BEFORE T7 (MPSC Actor)**. The MPSC actor needs a working SQLx repository to persist events. If `CirculationRepository` is stub-only, implement real persistence first.

---

## TODOs

- [x] 1. Check-in command + event

  **What to do**:
  - Add `CheckInCommand` with injected timestamp/correlation_id
  - Add `ItemReturned` event to CirculationEvent enum
  - Implement `ItemAggregate::process_checkin()` pure function
  - State transition: CHECKED_OUT → AVAILABLE

  **Must NOT do**:
  - No time generation in core (inject from shell)

  **Acceptance Criteria**:
  - [ ] Check-in command compiles
  - [ ] `process_checkin()` returns `ItemReturned` event
  - [ ] Check-in of available item returns specific error (`Err(ItemNotAvailable)`)
  - [ ] Check-in of item with active holds is rejected (business rule)

  **QA Scenarios**:
  - Scenario: Check-in succeeds for checked out item
    - Tool: Rust test
    - Steps: `ItemAggregate::process_checkin(checked_out_item)`
    - Expected Result: `Ok(ItemReturned { item_id: "item-001", patron_id: "patron-42", timestamp: 123, correlation_id: "corr-1" })`

  - Scenario: Check-in fails for available item
    - Tool: Rust test
    - Steps: `ItemAggregate::process_checkin(available_item)`
    - Expected Result: `Err(CheckoutError::ItemNotAvailable)`

- [x] 2. Renew command + event

  **What to do**:
  - Add `RenewCommand` with loan_id
  - Add `LoanRenewed` event
  - Implement renew logic with renewal count check

  **Acceptance Criteria**:
  - [ ] Renew command compiles
  - [ ] Renewal count increments

  **QA Scenarios**:
  - Scenario: Renew succeeds within limit
    - Tool: Rust test
    - Steps: `ItemAggregate::process_renew(active_loan)`
    - Expected Result: `Ok(LoanRenewed event)`

- [x] 3. Mark Lost command + event

  **What to do**:
  - Add `MarkLostCommand`
  - Add `ItemLost` event
  - State transition: any → LOST

  **Acceptance Criteria**:
  - [ ] Mark Lost compiles

- [x] 4. Hold management

  **What to do**:
  - Add `HoldCommand` (place, cancel, fulfill)
  - Add `HoldPlaced`, `HoldCancelled`, `HoldFulfilled` events
  - State transitions: AVAILABLE → ON_HOLD_SHELF → CHECKED_OUT

  **Acceptance Criteria**:
  - [ ] Hold commands compile

- [x] 5. cargo-mutants integration

  **What to do**:
  - Add cargo-mutants to dependencies
  - Configure for release-gate testing
  - Run mutation tests on test suite

  **Acceptance Criteria**:
  - [ ] cargo-mutants runs without errors

- [x] 6. Unit tests for all circulation commands

  **What to do**:
  - Write unit tests for all new commands
  - Test all validation paths

  **Acceptance Criteria**:
  - [ ] All tests pass

- [x] 7. MPSC actor channel in Axum

  **What to do**:
  - Create Tokio MPSC channel in Axum shell
  - Route all circulation commands through channel
  - Ensure strict-ordered event processing

  **Must NOT do**:
  - No blocking in the actor
  - No side effects in domain (actor is infrastructure only)

  **Acceptance Criteria**:
  - [ ] Events processed in strict order (FIFO)
  - [ ] Actor shuts down gracefully (drains remaining events)
  - [ ] Events are immutable once emitted

  **QA Scenarios**:
  - Scenario: Events processed in order
    - Tool: tokio::test
    - Steps: Send 3 events out-of-order, verify output order
    - Expected Result: Events emitted in same order as input

- [x] 8. Dead letter queue (in-memory)

  **What to do**:
  - Implement in-memory DLQ with bounded retries (max 3)
  - Store failed events with error reason
  - Add exponential backoff

  **Must NOT do**:
  - No persistent DLQ (in-memory only for Phase 1)

  **Acceptance Criteria**:
  - [ ] Failed events retry up to 3 times
  - [ ] After 3 failures, event goes to DLQ
  - [ ] DLQ stores: event payload, error reason, timestamp

  **QA Scenarios**:
  - Scenario: Event fails 3 times and goes to DLQ
    - Tool: Rust test
    - Steps: Inject failing event, verify DLQ after 3 retries
    - Expected Result: Event in DLQ with error reason

- [x] 9. Tantivy integration

  **What to do**:
  - Add tantivy dependency
  - Create search index schema (ItemId, title, author, status, location, timestamp)
  - Index ItemAggregate events
  - Define rebuild strategy: replay all events on startup

  **Must NOT do**:
  - No synchronous write-then-search (eventual consistency acceptable)

  **Acceptance Criteria**:
  - [ ] Tantivy compiles
  - [ ] Index rebuilds from events on startup
  - [ ] Search results include timestamp for consistency window

  **QA Scenarios**:
  - Scenario: Index rebuilds from events
    - Tool: Rust test
    - Steps: Create events, rebuild index, search
    - Expected Result: Search returns indexed documents

- [x] 10. Background worker

  **What to do**:
  - Create worker that polls SQLx for new events
  - Project events to Tantivy index
  - Handle eventual consistency

  **Acceptance Criteria**:
  - [ ] Events appear in search index

- [x] 11. Search endpoint

  **What to do**:
  - Add GET /api/v1/search endpoint
  - Include timestamp in results
  - Handle index lag gracefully

  **Acceptance Criteria**:
  - [ ] Search returns results with timestamps

- [x] 12. Integration test

  **What to do**:
  - Full flow: command → MPSC → SQLx → Tantivy → search

  **Acceptance Criteria**:
  - [ ] End-to-end search works

---

## Final Verification Wave

- [x] F1. Plan Compliance Audit
- [x] F2. Code Quality Review
- [x] F3. Real Manual QA
- [x] F4. Scope Fidelity Check

---

## Commit Strategy

| Wave | Commit Message | Files |
|------|----------------|-------|
| 1 | `feat(circulation): add CheckInCommand with validation` | `librery-core/src/domain/circulation.rs` |
| 1 | `feat(circulation): add RenewCommand with renewal count` | `librery-core/src/domain/circulation.rs` |
| 1 | `feat(circulation): add MarkLostCommand` | `librery-core/src/domain/circulation.rs` |
| 1 | `feat(circulation): add HoldCommand (place, cancel, fulfill)` | `librery-core/src/domain/circulation.rs` |
| 1 | `test(circulation): add cargo-mutants for mutation testing` | `.cargo/mutants.toml`, `librery-core/src/domain/*_tests.rs` |
| 2 | `feat(shell): implement MPSC event dispatcher` | `librery-shell/src/actor/*` |
| 2 | `feat(shell): add dead letter queue with retry` | `librery-shell/src/actor/*` |
| 2 | `feat(search): add Tantivy CQRS read model` | `librery-shell/src/search/*` |
| 2 | `feat(search): add background worker for projection` | `librery-shell/src/search/*` |
| 2 | `test(integration): end-to-end search flow` | `tests/*` |

**Commit Requirements**:
- Each commit must pass: `cargo test && cargo clippy`
- cargo-mutants runs on `librery-core` only

---

## Success Criteria

### Verification Commands
```bash
cargo test --lib
cargo mutants
cargo check
```

### Final Checklist
- [ ] All Must Have items present
- [ ] All Must NOT Have items absent
- [ ] All tests pass
- [ ] Mutation tests pass
- [ ] Full search flow works