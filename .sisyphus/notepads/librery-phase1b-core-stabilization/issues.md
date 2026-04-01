# Issues Encountered: Phase 1b Planning

---

## Issue 1: Momus Rejection - Parallel Execution False Claim

**Problem**: Plan claimed "parallel execution" but dependency matrix showed fully sequential chain

**Impact**: BLOCKED - Task couldn't execute in parallel as claimed

**Root Cause**: Matrix didn't match wave structure

**Fix**: Rewrote dependency matrix to show true parallelism (T1 ║ T5 independent)

---

## Issue 2: Missing QA Scenarios

**Problem**: 9/10 tasks had no executable QA (only T0 had one)

**Impact**: BLOCKED - Final verification wave couldn't run

**Fix**: Added curl + rust test scenarios for ALL tasks

---

## Issue 3: Existing Code Conflicts

**Problem**: Plan referenced creating Item/Patron that already exist

**Impact**: AMBIGUOUS - Developer wouldn't know extend vs recreate

**Fix**: Clarified "EXTEND existing" for Item, Composition for Actor/Patron

---

## Issue 4: FCIS Purity - bcrypt in Wrong Layer

**Problem**: Originally debated putting bcrypt in core

**Impact**: VIOLATES FCIS - crypto is side effect

**Fix**: bcrypt in shell only, core stores hash string only

---

## Issue 5: Inheritance vs Composition

**Problem**: Was considering inheritance (Patron extends Actor)

**Impact**: Tight coupling, DRY violation

**Fix**: Composition - Actor has general form, PatronCredentials has actor_id

---

## References

- Momus Review: `ses_2b7cfb8d8ffe6OMuISP1BTmRyN`
- Metis Fix: `ses_2b7ccdfb2ffeO5mmu4gstL0fP9`

---

## Phase 1.5 Planning Issues (April 2026)

### Issue 6: Missing Repositories

**Problem**: Plan assumed BibliographicRecordRepository and ActorRepository exist
**Impact**: BLOCKER — T2 and T4 can't complete without data access
**Fix**: Added repository creation to T2 and T4 scope

### Issue 7: Dependency Matrix Error

**Problem**: T3 (checkout) was blocked by T1 (search) — incorrect
**Impact**: Would block parallel execution unnecessarily
**Fix**: T3 only depends on T2 (needs catalog/item lookup patterns)

### Issue 8: br Dependency Direction

**Problem**: `br dep add <child> <parent>` was used backwards
**Impact**: Robot triage recommended wrong tasks
**Fix**: Reversed all dependency arguments and verified with `bv --robot-triage`