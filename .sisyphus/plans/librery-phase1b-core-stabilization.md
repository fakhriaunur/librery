# Librery Phase 1b.1: Core & Backend Stabilization

> Work Plan | Target: Stabilize core + backend before frontend
> Generated: April 2026
> **Pre-requisite**: Phase 0 + Phase 1 complete

---

## TL;DR

> **Quick Summary**: Complete the backend foundation needed for Phase 2 frontend. Add patron management, login endpoint with Biscuit tokens, bibliographic data, and extend search index.

> **Deliverables**:
> - Patron entity + `patrons` table in SQLite
> - Login/Logout endpoints with Biscuit token issuance (Axum)
> - Item/BibliographicRecord entities + `items` table
> - Extended Tantivy index with bibliographic fields
> - Patron loans/holds API endpoints
> - Updated OpenAPI spec

> **Estimated Effort**: 1-2 weeks
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Patron → Auth → Items → Search → API → OpenAPI

---

## Context

### Phase 0 Completed
- ✅ Checkout command (ItemAggregate)
- ✅ Event-sourced ledger (CirculationEvent)
- ✅ HTTP/3 + HTTP/2 server
- ✅ OpenAPI schema
- ✅ Ratatui TUI

### Phase 1 Completed
- ✅ Full circulation: check-in, renew, lost, holds
- ✅ MPSC actor with dead letter queue
- ✅ Tantivy search index (events only)

### What's Missing (Blocking Phase 2)
- ❌ No patron data → no authentication possible
- ❌ No login endpoint → frontend can't authenticate
- ❌ No item/bib data → OPAC displays nothing
- ❌ Tantivy only indexes events, not bibliographic fields

---

## br Skill Invocation

> **CRITICAL**: When `/start-work librery-phase1b-core-stabilization` is invoked, the executor MUST:
> 1. **INITIALIZE** br: `br init` if not already done
> 2. **CREATE** beads for each task: Use `br create` with appropriate priority
> 3. **UPDATE** status as work progresses: `br update <id> --status in_progress`
> 4. **CLOSE** when complete: `br close <id> --reason "Done"`
> 5. **SYNC** before commits: `br sync --flush-only`
>
> Reference: `/br` skill for command reference

---

## Work Objectives

### Core Objective
Stabilize the backend so Phase 2 frontend can actually function.

### Concrete Deliverables
- [ ] 1b.1a: Patron entity + repository (SQLite)
- [ ] 1b.1b: Login endpoint (POST /auth/login) → returns Biscuit token
- [ ] 1b.1c: Logout endpoint (POST /auth/logout)
- [ ] 1b.2a: Item entity + repository
- [ ] 1b.2b: BibliographicRecord entity (title, author, ISBN, etc.)
- [ ] 1b.3a: Extend Tantivy index with bib fields
- [ ] 1b.3b: Background worker projects items to Tantivy
- [ ] 1b.4a: GET /api/v1/patrons/{id}/loans
- [ ] 1b.4b: GET /api/v1/patrons/{id}/holds
- [ ] 1b.5: Update OpenAPI spec

### OPAC Considerations (for Phase 2)
- OPAC is **PUBLIC** (anonymous search/browse)
- Auth required only for: bookmark, borrow, dashboard
- Token issuance: **Axum** (not SvelteKit) —biscuit-auth 6.0.0
- Auth flow: Axum issues token → httpOnly cookie → SvelteKit validates

### Definition of Done
- [ ] Patron can login and receive Biscuit token
- [ ] Items table has bibliographic data (title, author, ISBN)
- [ ] Search returns bibliographic results
- [ ] Patron can query their loans/holds via API
- [ ] OpenAPI spec reflects all new endpoints
- [ ] All tests pass

### Must Have
- Patron entity with credentials
- Login endpoint issues Biscuit token
- Items table with bib fields
- Tantivy indexes bib data
- OpenAPI spec updated

### Must NOT Have
- No patron self-registration (admin-created only)
- No admin/librarian roles (Phase 3)
- No payment/fine handling (Phase 3+)
- No external integrations (Phase 3+)

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: YES (Phase 0/1)
- **Automated tests**: YES (TDD)
- **Framework**: Rust built-in `#[test]` + proptest

### QA Policy
Every task MUST include agent-executed QA scenarios.

---

## Execution Strategy

### Parallel Execution Waves

> **CRITICAL**: True parallelism - auth chain and bib track run CONCURRENTLY

```
Wave 0 ──────────► T0: Repo setup + README
    │
    ├────────────────────────────┐
    ▼                            ▼
Wave 1 ──────────► T1: Patron creds  ║  T5: BibliographicRecord entity
    │                        ║        (INDEPENDENT - NO DEPENDENCY!)
    │                        ║
    ▼                        ║
Wave 2 ──────────► T2: Login     ║
    │                        ║
    ▼                        ║
Wave 3 ──────────► T3: Logout    ║  
    │                        ║
    ▼                        ║
Wave 4 ──────────► T4: Auth      ║
    │                        ║
    ├────────────────────────┴──────────┐
    ▼                                    ▼
Wave 5 ──────────► T6: Tantivy ext    ║   T7: Item repo + worker
    │                        ║        T8: Patron loans/holds API
    │                        ║
    ▼                        ║
Wave 6 ──────────► T9: Update OpenAPI spec
```

### Dependency Matrix (TRULY Parallel)

| Task | Description | Blocked By | Parallel With |
|------|-------------|------------|---------------|
| T0 | Repo setup | — | — |
| T1 | Patron creds | T0 | T5 |
| T5 | BibRecord entity | T0 | T1 ← INDEPENDENT! |
| T2 | Login endpoint | T1 | T5, T6, T7, T8 (after T4) |
| T3 | Logout endpoint | T2 | T5, T6, T7, T8 (after T4) |
| T4 | Auth middleware | T3 | T5, T6, T7, T8 (after T4) |
| T6 | Tantivy extend | T5 | T7, T8 (after T4) |
| T7 | Item repo + worker | T5 | T6, T8 (after T4) |
| T8 | Patron API | T4, T5 | T6, T7 |
| T9 | OpenAPI update | T6, T7, T8 | — |

---

## TODOs

- [ ] 1b.0: README.md

  **What to do**:
  - Create root README.md (git already initialized, no repo creation needed)
  - Use content from project overview

  **Acceptance Criteria**:
  - [ ] README.md exists and describes project

  **QA Scenarios**:
  - Scenario: README exists
    - Tool: Bash
    - Steps: `ls -la README.md`
    - Expected Result: File exists

- [ ] 1b.1a: Actor + PatronCredentials (Composition)

  **What to do**:
  - **COMPOSITION over inheritance** - favor has-a relationship
  - Actor remains general: id, actor_type, name, email, status
  - Add `PatronCredentials` struct (has_a Actor via actor_id):
    - actor_id: ActorId (FK)
    - password_hash: String
    - created_at, last_login
  - Create `patron_credentials` table in SQLite
  - Implement `PatronCredentialsRepository` trait

  **Data Model** (Pragmatic Programmer - DRY + Orthogonality):
  ```rust
  // Actor - general form (unchanged!)
  pub struct Actor {
      pub id: ActorId,
      pub actor_type: ActorType,  // Patron, Librarian, Admin, Machine
      pub name, pub email, pub status
  }
  
  // PatronCredentials - composition (has-a)
  pub struct PatronCredentials {
      pub actor_id: ActorId,      // FK to Actor
      pub password_hash: String,   // bcrypt result
      pub created_at, last_login
  }
  ```

  **FCIS Purity** (CRITICAL):
  - **Core stores only**: `PatronCredentials` struct with password_hash (String)
  - **Shell does hashing**: bcrypt in `librery-shell`, not core
  - `librery-core` has ZERO side effects - pure domain

  **Dependencies** (WHERE each goes):
  - `librery-shell/Cargo.toml`: bcrypt (or argon2) — for hashing
  - `librery-core/Cargo.toml`: NO crypto libs

  **Acceptance Criteria**:
  - [ ] Patron entity compiles
  - [ ] Repository persists to SQLite
  - [ ] No side effects in core (verify with no crypto deps in librery-core)

  **QA Scenarios**:
  - Scenario: Create patron
    - Tool: Rust test
    - Steps: `PatronRepository::create(patron)`
    - Expected Result: `Ok(patron_id)`

- [ ] 1b.1b: Login endpoint with Biscuit token

  **What to do**:
  - Addbiscuit-auth 6.0.0 to dependencies
  - Implement POST /auth/login → issues Biscuit token
  - Token in httpOnly, Secure, SameSite=Strict cookie
  - Token stored in database for validation

  **Must**: Token issued from **Axum** (not SvelteKit)

  **Acceptance Criteria**:
  - [ ] Login with valid credentials returns token
  - [ ] Token in cookie header

  **QA Scenarios**:
  - Scenario: Login returns token
    - Tool: Bash (curl)
    - Steps: `curl -X POST http://localhost:8080/auth/login -d '{"email":"patron@test.com","password":"test"}' -c cookies.txt`
    - Expected Result: Set-Cookie with token

- [ ] 1b.1c: Logout endpoint

  **What to do**:
  - POST /auth/logout clears cookie
  - Invalidates token server-side

  **Acceptance Criteria**:
  - [ ] Logout clears cookie

  **QA Scenarios**:
  - Scenario: Logout clears cookie and returns 200
    - Tool: Bash (curl)
    - Steps: 
      1. Login: `curl -X POST http://localhost:8080/auth/login -H 'Content-Type: application/json' -d '{"email":"patron@test.com","password":"test123"}' -c cookies.txt`
      2. Logout: `curl -X POST http://localhost:8080/auth/logout -b cookies.txt -c cookies.txt`
    - Expected Result: `Set-Cookie: .*=; Max-Age=0`, HTTP 200

  - Scenario: Logout invalidates token server-side
    - Tool: Rust test
    - Steps: `cargo test --lib -p librery-shell test_logout_invalidates_token`
    - Expected Result: test passes

- [ ] 1b.1d: Auth middleware

  **What to do**:
  - Extract token from cookie
  - Validate token, reject invalid/expired
  - Attach patron_id to request context

  **Acceptance Criteria**:
  - [ ] Protected routes reject invalid tokens
  - [ ] Valid token allows access

  **QA Scenarios**:
  - Scenario: Protected route rejects no token
    - Tool: Bash (curl)
    - Steps: `curl -w '%{http_code}' -o /dev/null http://localhost:8080/api/v1/patrons/1/loans`
    - Expected Result: HTTP 401

  - Scenario: Protected route rejects invalid token
    - Tool: Bash (curl)
    - Steps: `curl -H 'Cookie: token=invalid' -w '%{http_code}' -o /dev/null http://localhost:8080/api/v1/patrons/1/loans`
    - Expected Result: HTTP 401

  - Scenario: Protected route accepts valid token
    - Tool: Bash (curl)
    - Steps: 
      1. Login to get token
      2. `curl -b cookies.txt -w '%{http_code}' http://localhost:8080/api/v1/patrons/1/loans`
    - Expected Result: HTTP 200

- [ ] 1b.2a: Item entity + repository

  **What to do**:
  - **EXTEND existing** `Item` entity in `librery-core/src/domain/item.rs`
  - Existing fields: id, bib_id, barcode, status, location
  - **ADD**: bib_record_id: Option<BibRecordId> (backward compatible)
  - Create `items` table + `bibliographic_records` table
  - Implement `ItemRepository` trait

  **Acceptance Criteria**:
  - [ ] Item entity compiles with new field

  **QA Scenarios**:
  - Scenario: Item with bib_record_id compiles
    - Tool: Rust test
    - Steps: `cargo test --lib -p librery-core test_item_extended_with_bib`
    - Expected Result: test passes

- [ ] 1b.2b: BibliographicRecord entity

  **What to do**:
  - Add `BibliographicRecord` entity
  - Fields: title, author, ISBN, publisher, publication_year, marc21
  - Create `bibliographic_records` table

  **Acceptance Criteria**:
  - [ ] BibRecord entity compiles

  **QA Scenarios**:
  - Scenario: BibliographicRecord struct compiles with all fields
    - Tool: Rust test
    - Steps: `cargo test --lib -p librery-core test_bib_record_fields`
    - Expected Result: test passes

- [ ] 1b.3a: Extend Tantivy index

  **What to do**:
  - Add fields: title, author, isbn, publisher, barcode
  - Update index schema
  - Rebuild strategy supports bib data

  **Acceptance Criteria**:
  - [ ] Search returns bib fields

  **QA Scenarios**:
  - Scenario: Search by title returns bib results
    - Tool: Bash (curl)
    - Steps: `curl 'http://localhost:8080/api/v1/search?q=Rust' | jq '.results[0].title'`
    - Expected Result: JSON with title field

- [ ] 1b.3b: Background worker items → Tantivy

  **What to do**:
  - Poll items table for new/modified records
  - Index to Tantivy

  **Acceptance Criteria**:
  - [ ] Items appear in search index

  **QA Scenarios**:
  - Scenario: Worker indexes new item to Tantivy
    - Tool: Rust test
    - Steps: `cargo test --lib -p librery-shell test_worker_indexes_item`
    - Expected Result: test passes

- [ ] 1b.4a: Patron loans API

  **What to do**:
  - GET /api/v1/patrons/{id}/loans
  - Returns active loans for patron

  **Acceptance Criteria**:
  - [ ] Endpoint returns loans

  **QA Scenarios**:
  - Scenario: Returns loans for authenticated patron
    - Tool: Bash (curl)
    - Steps: 
      1. `curl -X POST http://localhost:8080/auth/login -c cookies.txt`
      2. `curl -b cookies.txt http://localhost:8080/api/v1/patrons/1/loans`
    - Expected Result: JSON with loans array

  - Scenario: Rejects unauthenticated
    - Tool: Bash (curl)
    - Steps: `curl -w '%{http_code}' http://localhost:8080/api/v1/patrons/1/loans`
    - Expected Result: HTTP 401

- [ ] 1b.4b: Patron holds API

  **What to do**:
  - GET /api/v1/patrons/{id}/holds
  - Returns holds for patron

  **Acceptance Criteria**:
  - [ ] Endpoint returns holds

  **QA Scenarios**:
  - Scenario: Returns holds for authenticated patron
    - Tool: Bash (curl)
    - Steps: 
      1. Login: `curl -X POST http://localhost:8080/auth/login -c cookies.txt`
      2. `curl -b cookies.txt http://localhost:8080/api/v1/patrons/1/holds`
    - Expected Result: JSON with holds array

- [ ] 1b.5: Update OpenAPI spec

  **What to do**:
  - Generate updated OpenAPI spec
  - Include all new endpoints
  - Run openapi-generator-cli to update TypeScript SDK

  **Acceptance Criteria**:
  - [ ] Spec includes: /auth/login, /auth/logout, /patrons/{id}/loans, /patrons/{id}/holds
  - [ ] Generated SDK compiles

  **QA Scenarios**:
  - Scenario: OpenAPI spec includes all new endpoints
    - Tool: Bash (curl)
    - Steps: `curl http://localhost:8080/api/openapi.json | jq '[.paths | keys[]]'`
    - Expected Result: Array contains /auth/login, /auth/logout, /patrons

---

## Final Verification Wave

- [ ] F1. Plan Compliance Audit
- [ ] F2. Code Quality Review
- [ ] F3. API Integration Test
- [ ] F4. Scope Fidelity Check

---

## Commit Strategy

| Wave | Commit Message | Files |
|------|----------------|-------|
| 0 | `docs: add README and project setup` | `README.md`, `.gitignore` |
| 1 | `feat(core): add patron entity and repository` | `librery-core/src/domain/patron.rs` |
| 1 | `feat(auth): add login endpoint with biscuit token` | `librery-shell/src/api/auth/*` |
| 1 | `feat(auth): add logout endpoint` | `librery-shell/src/api/auth/*` |
| 1 | `feat(auth): add auth middleware` | `librery-shell/src/middleware/*` |
| 2 | `feat(core): add item and bibliographic record entities` | `librery-core/src/domain/item.rs`, `bib.rs` |
| 2 | `feat(search): extend tantivy index with bib fields` | `librery-shell/src/search/*` |
| 2 | `feat(api): add patron loans and holds endpoints` | `librery-shell/src/api/patrons/*` |
| 2 | `feat(spec): update openapi with new endpoints` | `librery-shell/src/api/*` |

**Commit Requirements**:
- Each commit must pass: `cargo test && cargo clippy`
- br sync after each batch: `br sync --flush-only`

---

## Success Criteria

### Verification Commands
```bash
cargo test --lib
curl -X POST http://localhost:8080/auth/login -d '{"email":"test@test.com","password":"test"}'
curl http://localhost:8080/api/openapi.json
```

### Final Checklist
- [ ] All Must Have present
- [ ] All Must NOT Have absent
- [ ] Login returns token in httpOnly cookie
- [ ] Search includes bibliographic fields
- [ ] OpenAPI spec updated