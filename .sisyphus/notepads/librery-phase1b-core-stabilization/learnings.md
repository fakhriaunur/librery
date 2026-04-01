# Learnings: Phase 1b Core Stabilization

> Recorded during planning and execution

---

## Design Decisions

### 1. FCIS Purity Enforcement

| Rule | Application |
|------|-------------|
| Core = NO side effects | PatronCredentials struct only stores hash |
| Shell does I/O | bcrypt lives in librery-shell, not core |
| Dependency location matters | crypto libs only in shell Cargo.toml |

### 2. Composition over Inheritance (Actor/Patron)

**Choice**: has-a relationship over is-a

**Reasoning** (Pragmatic Programmer):
- DRY: No duplicate fields between Actor and Patron
- Orthogonality: Can add/remove credentials without touching Actor
- Reversibility: Easy to add API key auth later

**Implementation**:
```rust
// General form
struct Actor { id, actor_type, name, email, status }

// Composition - separate but related
struct PatronCredentials { actor_id, password_hash, created_at }
```

### 3. Parallel vs Sequential

**Key insight**: True parallelism requires independent tasks

- T1 (Creds) ║ T5 (BibRecord) - INDEPENDENT
- T2-T4 (Auth) depends on T1
- T6-T8 require T4 (auth) AND T5 (bib data)

---

## Patterns Discovered

### Parallel Execution Waves (Anti-Pattern to Avoid)

**WRONG**: Claim parallel but matrix shows sequential
```
Wave 1: T1, T2, T3, T4  ← but T2 blocks T1! ❌
```

**RIGHT**: Matrix matches wave structure
```
Wave 0: T0
Wave 1: T1 ║ T5  ← independent
Wave 2: T2 (depends T1)
Wave 3: T3 (depends T2)
...
```

---

## References

- The Pragmatic Programmer (10th/20th Anniversary)
- FCIS boundaries: `.sisyphus/drafts/specs/ddd/librery-fcis-boundaries.md`

---

## Execution Progress (April 2026)

### Completed Tasks
| Task | Status | Notes |
|------|--------|-------|
| T0: README.md | ✅ | Created with project overview |
| T1: PatronCredentials | ✅ | Composition with Actor via actor_id FK |
| T5: BibliographicRecord | ✅ | title, author, ISBN, publisher, year, marc21 |

### Commit Log
```
5bf8938 feat(core): add PatronCredentials and BibliographicRecord entities
bfd6329 feat(auth): add login endpoint with Biscuit token
c7ab226 feat(auth): add logout endpoint
a7c18ab feat(auth): add auth middleware for protected routes
d62e8b0 feat(search): extend tantivy with bibliographic fields
8de5a37 feat(core): add ItemRepository and extend ProjectionWorker
67f0d9b feat(api): add patron loans and holds endpoints
7715371 feat(spec): update openapi with all new endpoints
```

### Key Files Created
- `librery-core/src/domain/patron_credentials.rs`
- `librery-core/src/domain/bibliographic_record.rs`
- `librery-shell/src/patron_credentials_repository.rs`
- `librery-shell/src/api/auth/login.rs`
- `librery-shell/src/api/auth/logout.rs`
- `librery-shell/src/middleware/auth.rs`
- `librery-shell/src/item_repository.rs`
- `librery-shell/src/api/patrons.rs`
- `README.md`

### Phase 1b Complete
All tasks T0-T9 completed successfully.

---

## Phase 1.5 Execution (April 2026)

### Commits
```
f7073ba fix(search): include bibliographic metadata in results
11c7064 feat(api): add catalog detail + BibliographicRecordRepository
18e2bdc feat(api): implement actual checkout endpoint
7a4da82 feat(api): add patron profile + actors table
3d193ad chore: update dependencies and gitignore
```

### Execution Notes
- T5 (OpenAPI spec) was already up-to-date from previous tasks
- All endpoints documented: checkout, search, login, logout, catalog, patron/me, loans, holds
- All schemas registered in utoipa ApiDoc

### Phase 1.5 Complete
All tasks T1-T5 completed successfully.

---

## Phase 1.5 Planning (April 2026)

### Beads Created

| ID | Task | Priority | Blocked By |
|----|------|----------|------------|
| librery-hn2 | Phase 1.5 Epic | P0 | — |
| librery-f1d | T1: Fix search metadata | P1 | — |
| librery-it8 | T2: Catalog + BibRecordRepo | P1 | — |
| librery-a6j | T4: Patron me + actors table | P1 | — |
| librery-ybx | T3: Checkout implementation | P1 | T2 |
| librery-w30 | T5: Update OpenAPI spec | P2 | T1, T2, T3, T4 |

### Dependency Graph (Verified with bv --robot-triage)

```
Wave 1 (parallel):
├── T1 (search fix) — independent
├── T2 (catalog + repo) — independent, unblocks T3
└── T4 (patron me + actors) — independent

Wave 2 (after T2):
└── T3 (checkout) — blocked by T2

Wave 3 (after all):
└── T5 (OpenAPI) — blocked by T1, T2, T3, T4
```

### Robot Verification Results

- **Top pick**: librery-it8 (T2: Catalog) — correctly unblocks T3
- **Ready**: T1, T2, T4 — all independent, can start immediately
- **Blocked**: T3 (by T2), T5 (by T1, T2, T3, T4)
- **No cycles**: ✅

### Metis Findings (Phase 1.5 Review)

1. **Missing BibliographicRecordRepository** — must be created in T2
2. **No actors table** — must be created in T4 for patron profiles
3. **index_document() never called** — search fix requires indexing
4. **Dependency matrix error** — T3 doesn't depend on T1, only T2

### Momus Review

- **Status**: APPROVED [OKAY]
- All referenced files verified
- QA scenarios executable
- No blocking issues