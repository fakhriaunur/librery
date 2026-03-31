# Librery Phase 0: First Tracer Bullet

> Work Plan | Target: Production-ready checkout via terminal
> Generated: March 2026

---

## TL;DR

> **Quick Summary**: Prove the Functional Core, Axum Shell, SQLx Ledger, HTTP/3 transport, and Ratatui TUI can communicate seamlessly.
> 
> **Deliverables**: 
> - Working checkout command via terminal
> - Immutable event stored in SQLite
> - HTTP/3 + HTTP/2 fallback working
> - OpenAPI schema auto-generated
> 
> **Estimated Effort**: Short (couple of days)
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Core → Shell → TUI

---

## Context

### Original Request
Build Librery, a next-generation library management system using FCIS architecture in Rust.

### Interview Summary
**Key Discussions**:
- FCIS pattern: Pure functional core + imperative shell
- Event-sourced circulation for audit trail
- HTTP/3 with fallback for unreliable networks
- SvelteKit for OPAC, Flutter+FRB for Staff App, Ratatui for terminal

### Research Findings
- LMS/LIS market dominated by legacy systems (Koha, SLiMS, Evergreen, Alma)
- Gap: deployment complexity + offline operation
- Librery solves with single binary + CRDT offline sync

### Metis Review
**Identified Gaps** (addressed):
- SQLite vs PostgreSQL: Use adapter pattern
- Mutation testing: Add as release-gate only

---

## Work Objectives

### Core Objective
A librarian can check out a single item to a single patron via the terminal, with the event persisted in SQLite.

### Concrete Deliverables
- [ ] `librery-core` crate with ItemAggregate and Checkout command
- [ ] `librery-shell` crate with Axum HTTP/3 server
- [ ] SQLite database with event ledger
- [ ] Ratatui terminal screen for checkout
- [ ] OpenAPI 3.1 schema via Utoipa

### Definition of Done
- [ ] Terminal input → HTTP/3 request → Immutable ItemCheckedOut event in SQLite
- [ ] Verified via database query

### Must Have
- Pure functional core (no time/Uuid generation in core)
- SQLx with adapter pattern for future PostgreSQL
- HTTP/3 (quinn) + HTTP/2 fallback (hyper)
- Health check endpoint

### Must NOT Have
- No authentication in this phase
- No PostgreSQL (use SQLite only for now)
- No complex migration logic

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: NO (new project)
- **Automated tests**: YES (TDD)
- **Framework**: Rust built-in `#[test]`, proptest for properties
- **If TDD**: Each task follows RED (failing test) → GREEN (minimal impl) → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.

**QA Scenarios for this plan:**
- Terminal checkout flow via tmux
- HTTP/3 request verification
- SQLite event persistence check

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — core foundation):
├── T1: Project scaffolding + Cargo workspace
├── T2: Domain types (ItemId, ActorId, ItemStatus, etc.)
├── T3: Checkout command + event definitions
├── T4: proptest properties for checkout invariants
└── T5: Unit tests for core logic

Wave 2 (After Wave 1 — shell + integration):
├── T6: SQLx setup + event persistence
├── T7: Axum HTTP server + HTTP/3 + fallback
├── T8: Utoipa OpenAPI schema generation
├── T9: Ratatui terminal screen
└── T10: Integration test (full flow)
```

### Dependency Matrix

| Task | Blocks | Blocked By |
|------|--------|------------|
| T1 | T2-T5 | — |
| T2 | T3 | T1 |
| T3 | T4 | T2 |
| T4 | T5 | T3 |
| T5 | T6 | T4 |
| T6 | T7 | T5 |
| T7 | T8 | T6 |
| T8 | T9 | T7 |
| T9 | T10 | T8 |
| T10 | — | T9 |

---

## TODOs

- [ ] 1. Project scaffolding + Cargo workspace

  **What to do**:
  - Create `librery` workspace with `librery-core` and `librery-shell` crates
  - Set up `justfile` with commands: `build`, `test`, `run-shell`, `generate-schema`
  - Add dependencies: axum, sqlx, utoipa, proptest, ratatui

  **Must NOT do**:
  - Don't add Flutter dependencies yet (Phase 3)
  - Don't add Tantivy (Phase 1)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`, `cargo`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T2-T5)
  - **Blocks**: T2-T5
  - **Blocked By**: None

  **References**:
  - Rust workspace: https://doc.rust-lang.org/cargo/reference/manifest.html

  **Acceptance Criteria**:
  - [ ] Workspace builds: `cargo build --workspace`
  - [ ] Justfile runs: `just build`

  **QA Scenarios**:
  - Scenario: Workspace builds successfully
    - Tool: Bash
    - Steps: `cargo build --workspace`
    - Expected Result: No compilation errors

- [ ] 2. Domain types (ItemId, ActorId, ItemStatus, etc.)

  **What to do**:
  - Define value objects: `ItemId`, `ActorId`, `BranchId`, `LoanId`
  - Define entities: `Item`, `Actor`
  - Define enums: `ItemStatus`, `ActorType`, `ActorStatus`
  - Place in `librery-core/src/domain/`

  **Must NOT do**:
  - No database types in domain (keep pure)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T1, T3-T5)
  - **Blocks**: T3
  - **Blocked By**: T1

  **References**:
  - Tactical DDD: `.sisyphus/drafts/specs/ddd/librery-tactical-ddd.md`

  **Acceptance Criteria**:
  - [ ] Types compile: `cargo check -p librery-core`

  **QA Scenarios**:
  - Scenario: Domain types compile
    - Tool: Bash
    - Steps: `cargo check -p librery-core`
    - Expected Result: No type errors

- [ ] 3. Checkout command + event definitions

  **What to do**:
  - Define `CheckoutCommand` with injected timestamp/correlation_id
  - Define `CirculationEvent` enum (ItemCheckedOut, ItemReturned, etc.)
  - Implement `ItemAggregate::process()` pure function
  - Implement `ItemAggregate::rebuild()` for state reconstruction

  **Must NOT do**:
  - No time generation in core (inject from shell)
  - No UUID generation in core (inject from shell)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T1-T2, T4-T5)
  - **Blocks**: T4
  - **Blocked By**: T2

  **References**:
  - FCIS boundaries: `.sisyphus/drafts/specs/ddd/librery-fcis-boundaries.md`
  - Deterministic core: `.sisyphus/drafts/research/gemini/more-ddd-context.md`

  **Acceptance Criteria**:
  - [ ] Checkout command compiles
  - [ ] `process()` returns events, not side effects

  **QA Scenarios**:
  - Scenario: Available item checkout succeeds
    - Tool: Rust test
    - Steps: `ItemAggregate::process(available_item, valid_patron)`
    - Expected Result: `Ok(ItemCheckedOut event)`

  - Scenario: CheckedOut item checkout fails
    - Tool: Rust test
    - Steps: `ItemAggregate::process(checked_out_item, any_patron)`
    - Expected Result: `Err(ItemNotAvailable)`

- [ ] 4. proptest properties for checkout invariants

  **What to do**:
  - Add proptest dependency
  - Define `Arbitrary` for domain types
  - Write property tests:
    - `available_item_checkout_succeeds`
    - `blocked_patron_checkout_fails`
    - `fine_never_exceeds_max`

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`, `proptest`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T1-T3, T5)
  - **Blocks**: T5
  - **Blocked By**: T3

  **References**:
  - Testing strategy: `.sisyphus/drafts/research/05-testing-strategy.md`

  **Acceptance Criteria**:
  - [ ] 1000 iterations pass without failure

  **QA Scenarios**:
  - Scenario: Property tests run
    - Tool: Bash
    - Steps: `cargo test --lib -- --test-threads=4`
    - Expected Result: All properties pass

- [ ] 5. Unit tests for core logic

  **What to do**:
  - Write unit tests for all domain functions
  - Use Chicago TDD: test first, then implementation
  - Test all validation paths

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T1-T4)
  - **Blocks**: T6
  - **Blocked By**: T4

  **Acceptance Criteria**:
  - [ ] `cargo test --lib` passes

  **QA Scenarios**:
  - Scenario: All unit tests pass
    - Tool: Bash
    - Steps: `cargo test --lib`
    - Expected Result: All tests pass

- [ ] 6. SQLx setup + event persistence

  **What to do**:
  - Set up SQLx with SQLite
  - Create `circulation_events` table
  - Implement `CirculationRepository` trait
  - Implement `append_event()` and `get_events()`

  **Must NOT do**:
  - Don't add PostgreSQL support yet (adapter pattern ready, not implemented)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`, `sqlx`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: T7
  - **Blocked By**: T5

  **References**:
  - SQLx: https://docs.rs/sqlx/latest/sqlx/

  **Acceptance Criteria**:
  - [ ] Events persist to SQLite

  **QA Scenarios**:
  - Scenario: Event persists to database
    - Tool: Bash
    - Steps: Run checkout, query DB
    - Expected Result: Event in `circulation_events` table

- [ ] 7. Axum HTTP server + HTTP/3 + fallback

  **What to do**:
  - Set up Axum server
  - Add quinn for HTTP/3 (UDP)
  - Add hyper for HTTP/2 fallback (TCP)
  - Inject `Alt-Svc: h3="..."` header
  - Create POST `/api/v1/checkout` route

  **Must NOT do**:
  - No authentication in this phase

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`, `axum`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: T8
  - **Blocked By**: T6

  **References**:
  - FCIS boundaries: `.sisyphus/drafts/specs/ddd/librery-fcis-boundaries.md`

  **Acceptance Criteria**:
  - [ ] Server starts on port 8080
  - [ ] HTTP/3 works (or fallback to HTTP/2)

  **QA Scenarios**:
  - Scenario: Server responds to HTTP request
    - Tool: Bash
    - Steps: `curl -X POST http://localhost:8080/api/v1/checkout -d '{}'`
    - Expected Result: Valid response (even if error)

- [ ] 8. Utoipa OpenAPI schema generation

  **What to do**:
  - Add Utoipa dependency
  - Generate OpenAPI 3.1 schema at compile time
  - Add `/api/docs` endpoint for swagger UI

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`, `utoipa`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: T9
  - **Blocked By**: T7

  **References**:
  - Utoipa: https://docs.rs/utoipa/latest/utoipa/

  **Acceptance Criteria**:
  - [ ] OpenAPI JSON accessible at `/api/openapi.json`

  **QA Scenarios**:
  - Scenario: OpenAPI schema generated
    - Tool: Bash
    - Steps: `curl http://localhost:8080/api/openapi.json`
    - Expected Result: Valid OpenAPI 3.1 JSON

- [ ] 9. Ratatui terminal screen

  **What to do**:
  - Create `librery-tui` crate
  - Build minimal checkout screen
  - Use reqwest with HTTP/3 to call Axum backend

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`, `ratatui`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: T10
  - **Blocked By**: T8

  **References**:
  - Ratatui: https://docs.rs/ratatui/latest/ratatui/

  **Acceptance Criteria**:
  - [ ] Terminal renders checkout prompt

  **QA Scenarios**:
  - Scenario: Terminal screen renders
    - Tool: interactive_bash (tmux)
    - Steps: Run TUI, capture output
    - Expected Result: Screen visible

- [ ] 10. Integration test (full flow)

  **What to do**:
  - Write end-to-end test: terminal → HTTP → DB
  - Verify event persisted correctly

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`rust`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: F1-F4
  - **Blocked By**: T9

  **Acceptance Criteria**:
  - [ ] Full flow works: input → event in DB

  **QA Scenarios**:
  - Scenario: End-to-end checkout
    - Tool: interactive_bash (tmux)
    - Steps: Enter item ID, patron ID → submit → verify DB
    - Expected Result: ItemCheckedOut event in SQLite

---

- [ ] F1. Plan Compliance Audit — Verify all deliverables present
- [ ] F2. Code Quality Review — `cargo check`, `cargo test`
- [ ] F3. Real Manual QA — Run full terminal checkout flow
- [ ] F4. Scope Fidelity Check — No feature creep beyond checkout

---

## Commit Strategy

- **1**: `feat(core): add item aggregate and checkout command`
- **2**: `feat(core): add proptest properties for checkout`
- **3**: `feat(shell): add sqlx persistence layer`
- **4**: `feat(shell): add axum http server with http3`
- **5**: `feat(shell): add utoipa openapi generation`
- **6**: `feat(tui): add ratatui checkout screen`
- **7**: `test(integration): end-to-end checkout flow`

---

## Final Verification Wave

- [ ] F1. Plan Compliance Audit — `oracle`
  Verify all deliverables present: core, shell, TUI, schema.

- [ ] F2. Code Quality Review — `unspecified-high`
  Run `cargo check`, `cargo test --lib`, verify no warnings.

- [ ] F3. Real Manual QA — `unspecified-high`
  Execute full terminal checkout flow via tmux.

- [ ] F4. Scope Fidelity Check — `deep`
  Verify no feature creep: only checkout, no auth, no PostgreSQL.

---

## Commit Strategy

| Wave | Commit Message | Files |
|------|----------------|-------|
| 1 | `feat(core): add item aggregate and checkout command` | `librery-core/src/domain/*` |
| 1 | `feat(core): add proptest properties for checkout` | `librery-core/src/domain/*_tests.rs` |
| 2 | `feat(shell): add sqlx persistence layer` | `librery-shell/src/persistence/*` |
| 2 | `feat(shell): add axum http server with http3` | `librery-shell/src/api/*` |
| 2 | `feat(shell): add utoipa openapi generation` | `librery-shell/src/api/*` |
| 2 | `feat(tui): add ratatui checkout screen` | `librery-tui/src/*` |
| 2 | `test(integration): end-to-end checkout flow` | `tests/*` |

---

## Success Criteria

### Verification Commands
```bash
cargo test --lib
cargo check
# Terminal flow produces event in SQLite
sqlite3 librery.db "SELECT * FROM circulation_events"
```

### Final Checklist
- [ ] All Must Have items present
- [ ] All Must NOT Have items absent
- [ ] All tests pass
- [ ] Full flow works end-to-end