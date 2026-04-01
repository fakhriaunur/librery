# Librery Phase 1.5: Critical API Gaps

> Work Plan | Target: Fix API gaps blocking Phase 2 OPAC
> Generated: April 2026

---

## TL;DR

> **Quick Summary**: Fix 4 critical API gaps that block a functional SvelteKit OPAC: search metadata, checkout implementation, catalog detail, patron profile.

> **Deliverables**:
> - Search returns bibliographic metadata (title, author, isbn)
> - Checkout endpoint actually processes loans (not stub)
> - Catalog detail endpoints (GET /catalog/{id})
> - Patron "me" endpoint (GET /patrons/me)

> **Estimated Effort**: 3-5 days
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Search → Catalog → Checkout → Patron

---

## Context

### Phase 1b Completed
- ✅ PatronCredentials + login/logout
- ✅ Biscuit token auth with middleware
- ✅ BibliographicRecord entity
- ✅ ItemRepository
- ✅ Patron loans/holds API
- ✅ OpenAPI spec

### Critical Gaps (from Metis exploration)
- ❌ Search returns NO bibliographic metadata (title, author, isbn)
- ❌ Checkout is STUB (`{status: "ok"}`)
- ❌ No catalog detail endpoints (GET /catalog/{id})
- ❌ No patron "me" endpoint (GET /patrons/me)

### Metis Review
**Identified Gaps** (addressed):
- Search response shape doesn't include bib fields
- Checkout endpoint doesn't use domain logic
- Missing catalog endpoints for OPAC book detail page
- Missing patron profile endpoint for dashboard

---

## Work Objectives

### Core Objective
Make the backend API fully functional for Phase 2 OPAC frontend.

### Concrete Deliverables
- [ ] 1.5.1: Fix search to return bibliographic metadata
- [ ] 1.5.2: Implement actual checkout endpoint
- [ ] 1.5.3: Add catalog detail endpoints
- [ ] 1.5.4: Add patron "me" endpoint
- [ ] 1.5.5: Update OpenAPI spec

### Definition of Done
- [ ] `curl 'http://localhost:8080/api/v1/search?q=Rust' | jq '.results[0].title'` returns title
- [ ] Checkout creates actual loan record
- [ ] `curl 'http://localhost:8080/api/v1/catalog/1'` returns book details + availability
- [ ] `curl -H 'Cookie: auth_token=...' http://localhost:8080/api/v1/patrons/me` returns profile
- [ ] OpenAPI spec includes all new endpoints

### Must Have
- Search includes title, author, isbn in results
- Checkout uses existing domain logic (circulation.rs)
- Catalog returns availability status
- Patron "me" uses auth middleware

### Must NOT Have
- No new entities (use existing ones)
- No pagination (Phase 3)
- No advanced search features (Phase 3)
- No payment/fine handling (Phase 3)

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: YES (TDD)
- **Framework**: Rust built-in `#[test]` + curl for API

### QA Policy
Every task MUST include agent-executed QA scenarios.

---

## br Skill Invocation

> **CRITICAL**: When `/start-work librery-phase1.5-critical-api-gaps` is invoked:
> 1. `br init` if not done
> 2. `br create` beads for each task
> 3. `br update <id> --status in_progress` as work progresses
> 4. `br close <id> --reason "Done"` when complete
> 5. `br sync --flush-only` before commits

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (ALL parallel — no dependencies):
├── T1: Fix search to return bibliographic metadata
├── T2: Catalog detail + BibliographicRecordRepository
└── T4: Patron "me" + actors table

Wave 2 (After Wave 1):
├── T3: Implement actual checkout endpoint
└── T5: Update OpenAPI spec
```

### Dependency Matrix

| Task | Description | Blocked By | Parallel With |
|------|-------------|------------|---------------|
| T1 | Search metadata fix | — | T2, T4 |
| T2 | Catalog detail + BibRecordRepository | — | T1, T4 |
| T3 | Checkout implementation | T2 | T5 |
| T4 | Patron "me" + actors table | — | T1, T2 |
| T5 | OpenAPI update | T1, T2, T3, T4 | — |

---

## TODOs

- [ ] 1.5.1: Fix search to return bibliographic metadata

  **What to do**:
  - Modify `SearchIndex::search()` to include title, author, isbn from Tantivy
  - Update `SearchResult` struct to include bib fields
  - Update search endpoint response shape

  **Acceptance Criteria**:
  - [ ] Search results include `title`, `author`, `isbn`

  **QA Scenarios**:
  - Scenario: Search returns bibliographic metadata
    - Tool: Bash (curl)
    - Steps: `curl 'http://localhost:8080/api/v1/search?q=Rust' | jq '.results[0].title'`
    - Expected Result: Returns title string (not null)

- [ ] 1.5.2: Add catalog detail endpoints

  **What to do**:
  - Create `BibliographicRecordRepository` trait + SQLite implementation
  - Create `GET /api/v1/catalog/{id}` endpoint
  - Returns book details + availability status
  - Create `GET /api/v1/items/{id}/availability` endpoint

  **Acceptance Criteria**:
  - [ ] Catalog detail returns title, author, isbn, availability
  - [ ] BibliographicRecordRepository persists to SQLite

  **QA Scenarios**:
  - Scenario: Catalog detail returns book info
    - Tool: Bash (curl)
    - Steps: `curl 'http://localhost:8080/api/v1/catalog/1' | jq '.title'`
    - Expected Result: Returns title string

- [ ] 1.5.3: Implement actual checkout endpoint

  **What to do**:
  - Replace stub with real checkout using `ItemAggregate::process()`
  - Accept patron_id, item_id from auth context
  - Create loan record in database via CirculationRepository
  - Return loan_id

  **Acceptance Criteria**:
  - [ ] Checkout creates loan record in database

  **QA Scenarios**:
  - Scenario: Checkout creates loan
    - Tool: Bash (curl)
    - Steps: 
      1. Login to get token
      2. `curl -X POST http://localhost:8080/api/v1/checkout -H 'Cookie: auth_token=...' -d '{"item_id":"item-001"}'`
    - Expected Result: Returns `{loan_id: "..."}`

- [ ] 1.5.4: Add patron "me" endpoint

  **What to do**:
  - Create `actors` table in SQLite (id, actor_type, name, email, status)
  - Create `GET /api/v1/patrons/me` endpoint
  - Uses auth middleware to get patron_id from token
  - Returns patron profile (name, email, status)

  **Acceptance Criteria**:
  - [ ] Returns patron email, name, status
  - [ ] actors table created with schema

  **QA Scenarios**:
  - Scenario: Patron "me" returns profile
    - Tool: Bash (curl)
    - Steps: `curl -H 'Cookie: auth_token=...' http://localhost:8080/api/v1/patrons/me | jq '.email'`
    - Expected Result: Returns patron email

- [ ] 1.5.5: Update OpenAPI spec

  **What to do**:
  - Update utoipa ApiDoc with all new endpoints
  - Include catalog, patron/me, updated search response

  **Acceptance Criteria**:
  - [ ] OpenAPI spec includes all new endpoints

  **QA Scenarios**:
  - Scenario: OpenAPI spec includes catalog endpoint
    - Tool: Bash (curl)
    - Steps: `curl http://localhost:8080/api/openapi.json | jq '.paths | keys[]' | grep catalog`
    - Expected Result: Contains /api/v1/catalog

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
| 1 | `fix(search): include bibliographic metadata in results` | `librery-shell/src/search/*` |
| 1 | `feat(api): add catalog detail + BibliographicRecordRepository` | `librery-shell/src/api/catalog.rs`, `librery-shell/src/bib_repository.rs` |
| 2 | `feat(api): implement actual checkout endpoint` | `librery-shell/src/api/checkout.rs` |
| 2 | `feat(api): add patron profile + actors table` | `librery-shell/src/api/patrons/me.rs` |
| 3 | `feat(spec): update openapi with new endpoints` | `librery-shell/src/main.rs` |

**Commit Requirements**:
- Each commit must pass: `cargo test && cargo clippy`
- br sync after each batch: `br sync --flush-only`

---

## Success Criteria

### Verification Commands
```bash
# Search returns metadata
curl 'http://localhost:8080/api/v1/search?q=Rust' | jq '.results[0].title'

# Checkout creates loan
curl -X POST http://localhost:8080/api/v1/checkout \
  -H 'Content-Type: application/json' \
  -H 'Cookie: auth_token=...' \
  -d '{"item_id":"item-001"}' | jq '.loan_id'

# Catalog detail
curl 'http://localhost:8080/api/v1/catalog/1' | jq '.title, .author, .availability'

# Patron me
curl -H 'Cookie: auth_token=...' http://localhost:8080/api/v1/patrons/me | jq '.email'
```

### Final Checklist
- [ ] All Must Have present
- [ ] All Must NOT Have absent
- [ ] All tests pass
- [ ] All endpoints return correct data