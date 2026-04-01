# Architectural Decisions: Phase 1b

> Decision log for core stabilization

---

## D1: Actor/Patron Relationship

| | |
|---|---|
| **Date** | April 2026 |
| **Decision** | Use composition (has-a) over inheritance (is-a) |
| **Rationale** | DRY, Orthogonality, Reversibility - Pragmatic Programmer |
| **Alternative considered** | Inheritance (Actor extends Patron or vice versa) |
| **Outcome** | Actor + PatronCredentials - separate but related |

---

## D2: FCIS Purity

| | |
|---|---|
| **Date** | April 2026 |
| **Decision** | bcrypt ONLY in shell, never in core |
| **Rationale** | Core must be pure - no side effects |
| **Implementation** | Core stores hash string, Shell does verification |

---

## D3: Parallel Execution Structure

| | |
|---|---|
| **Date** | April 2026 |
| **Decision** | Auth chain sequential, but runs parallel to bib track |
| **Rationale** | Max throughput while maintaining data dependencies |
| **Verification** | Dependency matrix matches wave structure |

---

## References

- Momus review rejected plan twice for dependency/parallelism issues
- Metis Fix: `ses_2b7ccdfb2ffeO5mmu4gstL0fP9`

---

## Phase 1.5 Decisions (April 2026)

### D4: Phase 1.5 Scope

| | |
|---|---|
| **Date** | April 2026 |
| **Decision** | Fix critical API gaps before Phase 2 OPAC |
| **Rationale** | Backend is ready but 4 gaps block functional frontend |
| **Alternative considered** | Continue backend phases (1c, 1d...) |
| **Outcome** | Phase 1.5: search metadata, catalog, checkout, patron me |

### D5: Dependency Structure

| | |
|---|---|
| **Date** | April 2026 |
| **Decision** | T1, T2, T4 independent; T3 blocked by T2; T5 blocked by all |
| **Rationale** | Correct dependency flow based on actual code requirements |
| **Verification** | `bv --robot-triage` confirms T2 as top pick (unblocks T3) |