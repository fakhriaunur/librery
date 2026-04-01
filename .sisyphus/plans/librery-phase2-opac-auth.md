# Librery Phase 2: Unified Patron App

> Work Plan | Target: SvelteKit OPAC + Patron Authentication
> Generated: April 2026
> **Latest Verified Versions** (April 2026):
> - SvelteKit 2.50 / Svelte 5
> - biscuit-auth 6.0.0
> - openapi-generator-cli 2.31.0
> - vitest (via `npx sv add vitest`)
> - @playwright/test (via `npx sv add playwright`)

---

## TL;DR

> **Quick Summary**: Deliver a lightning-fast web OPAC with SvelteKit, integrate with Tantivy search, and add patron authentication via Biscuit tokens.

> **Deliverables**:
> - SvelteKit workspace with Justfile
> - OPAC Search endpoint integrated with Tantivy
> - Patron authentication with Biscuit tokens
> - Patron Dashboard as protected route

> **Estimated Effort**: 1-2 weeks
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: SvelteKit → Search → Auth

---

## Context

### Phase 1 Completed
- ✅ Complete circulation: check-in, renew, lost, holds
- ✅ MPSC actor with dead letter queue
- ✅ Tantivy search index with background projection
- ✅ Search endpoint with timestamps
- ✅ All integration tests pass

### Original Request
Build Librery, a next-generation library management system using FCIS architecture in Rust.

### Tracer Bullets Reference
Phase 2 scope defined in `.sisyphus/drafts/research/tracer-bullets-updated.md`

---

## Work Objectives

### Core Objective
Deliver a unified patron web experience for discovery and account management.

### Concrete Deliverables
- [ ] 2.1a: SvelteKit workspace setup
- [ ] 2.1b: Justfile with openapi-generator-cli for TypeScript SDK
- [ ] 2.2a: OPAC search bar → Tantivy integration
- [ ] 2.2b: Browser native fetch API with HTTP/3 via Alt-Svc
- [ ] 2.3a: Axum issues Biscuit token (HttpOnly, Secure cookie)
- [ ] 2.3b: Patron Dashboard as protected route

### Definition of Done
- [ ] SvelteKit OPAC renders and is accessible
- [ ] Search via Tantivy works from browser
- [ ] Patron can log in and see dashboard
- [ ] Auth protects dashboard route
- [ ] QA: vitest + @testing-library/svelte tests pass
- [ ] QA: Playwright E2E passes (search → login → hold)

### Must Have
- SvelteKit with static rendering for OPAC
- Justfile automates openapi-generator-cli
- Biscuit token authentication
- Protected patron route

### Must NOT Have
- Staff features in this phase (Phase 3)
- PostgreSQL (SQLite only for now)
- Complex RBAC (basic auth only)

---

## Verification Strategy

### Test Decision
- **Infrastructure exists**: YES (Phase 1)
- **Automated tests**: YES
- **Framework**: vitest + @testing-library/svelte
- **If TDD**: Each task follows RED → GREEN → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.

---

## br Skill Invocation

> **CRITICAL**: When `/start-work librery-phase2-opac-auth` is invoked, the executor MUST:
> 1. **INITIALIZE** br: `br init` if not already done
> 2. **CREATE** beads for each task: Use `br create` with appropriate priority
> 3. **UPDATE** status as work progresses: `br update <id> --status in_progress`
> 4. **CLOSE** when complete: `br close <id> --reason "Done"`
> 5. **SYNC** before commits: `br sync --flush-only`
>
> Reference: `/br` skill for command reference

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — SvelteKit foundation):
├── T1: SvelteKit workspace setup
├── T2: Justfile with openapi-generator-cli
└── T3: TypeScript SDK generation

Wave 2 (After Wave 1 — Search + Auth):
├── T4: OPAC search bar component
├── T5: Tantivy search integration
├── T6: HTTP/3 via Alt-Svc
├── T7: Biscuit token auth in Axum
├── T8: Protected patron dashboard route
└── T9: E2E Playwright tests
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
| T9 | — | T8 |

---

## TODOs

- [ ] 2.1a: SvelteKit workspace setup

  **What to do**:
  - Create SvelteKit project in workspace
  - Add to Cargo workspace members
  - Configure for static rendering (for OPAC)

  **Must NOT do**:
  - No SSR for OPAC (static only)

  **Acceptance Criteria**:
  - [ ] SvelteKit compiles
  - [ ] Dev server starts

- [ ] 2.1b: Justfile with openapi-generator-cli

  **What to do**:
  - Add openapi-generator-cli to Justfile
  - Generate TypeScript SDK from OpenAPI schema
  - Wire SDK to SvelteKit lib

  **Acceptance Criteria**:
  - [ ] Justfile runs generate-sdk
  - [ ] Generated types compile

- [ ] 2.2a: OPAC search bar → Tantivy

  **What to do**:
  - Create search bar component
  - Connect to /api/v1/search endpoint
  - Display results with timestamps

  **Acceptance Criteria**:
  - [ ] Search returns results

- [ ] 2.2b: Browser fetch with HTTP/3

  **What to do**:
  - Configure Alt-Svc header for HTTP/3 upgrade
  - Use native fetch API

  **Acceptance Criteria**:
  - [ ] HTTP/3 works (or graceful fallback)

- [ ] 2.3a: Biscuit token auth

  **What to do**:
  - Add biscuit dependency to shell
  - Implement token generation on login
  - Set HttpOnly, Secure cookie

  **Acceptance Criteria**:
  - [ ] Login returns token in cookie

- [ ] 2.3b: Protected dashboard route

  **What to do**:
  - Create /patron/dashboard route
  - Add auth middleware
  - Display patron info

  **Acceptance Criteria**:
  - [ ] Unauthenticated access redirects to login
  - [ ] Authenticated access shows dashboard

- [ ] T9: E2E Playwright tests

  **What to do**:
  - Test: patron searches → logs in → places hold
  - Verify full flow works

  **Acceptance Criteria**:
  - [ ] All E2E tests pass

---

## Final Verification Wave

- [ ] F1. Plan Compliance Audit
- [ ] F2. Code Quality Review
- [ ] F3. Real Manual QA
- [ ] F4. Scope Fidelity Check

---

## Commit Strategy

| Wave | Commit Message | Files |
|------|----------------|-------|
| 1 | `feat(opac): add sveltekit workspace with justfile` | `librery-opac/*`, `justfile` |
| 1 | `feat(opac): generate typescript sdk from openapi` | `librery-opac/src/lib/*.ts` |
| 2 | `feat(opac): add search bar with tantivy integration` | `librery-opac/src/routes/*` |
| 2 | `feat(opac): add http/3 support via alt-svc` | `librery-shell/src/*` |
| 2 | `feat(auth): add biscuit token authentication` | `librery-shell/src/auth/*` |
| 2 | `feat(opac): add protected patron dashboard` | `librery-opac/src/routes/patron/*` |
| 2 | `test(e2e): add playwright tests for search→login→hold` | `librery-opac/tests/*` |

**Commit Requirements**:
- Each commit must pass: `cargo test && cargo clippy`
- br sync after each batch: `br sync --flush-only`

---

## Success Criteria

### Verification Commands
```bash
cargo test --lib
just test-e2e
cargo check
```

### Final Checklist
- [ ] All Must Have items present
- [ ] All Must NOT Have items absent
- [ ] All tests pass
- [ ] E2E tests pass