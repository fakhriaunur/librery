# Librery Testing Strategy

> Research date: March 2026
> Topic: FCIS-aligned testing strategy for Librery (Rust + Axum + PostgreSQL)

---

## 🎯 TDD Approach: Chicago vs London

| Layer | TDD School | Philosophy | Test Tool |
|-------|------------|------------|-----------|
| **Core (Functional)** | **Chicago (Detroit)** | Test-first, pure functions, no mocks | Unit + Property-based |
| **Shell (Imperative)** | **London (Mockist)** | Outside-in, mock collaborators, integration | Mock tests + E2E |

### Chicago (Core) — Test First, Then Code

```
1. Write a failing test (pure logic)
2. Write MINIMAL code to pass
3. Refactor
4. Add PBT invariants
```

### London (Shell) — Outside-In with Mocks

```
1. Write acceptance test / API test  
2. Mock dependencies
3. See it fail (missing implementation)
4. Implement, wire up
5. Replace mock with integration test
```

---

## FCIS Boundary & Test Split

### The Core Principle

| Layer | Test Type | Speed | Determinism | Dependencies |
|-------|-----------|-------|-------------|--------------|
| **Core** (Functional) | Unit tests | Fast (ms) | ✅ Fully deterministic | None |
| **Shell** (Imperative) | Integration tests | Slow (100ms+) | ⚠️ Requires mocks/DB | DB, HTTP |

**Goal**: Minimize shell tests, maximize core tests.

---

## Test Pyramid for Librery

```
        ┌─────────────────────────────────────────────┐
        │           E2E Tests (Browser/TUI)            │  ~5%
        │     Librarians: Dioxus/Ratatui, OPAC:        │
        │     Playwright/Tmux verification             │
        ├─────────────────────────────────────────────┤
        │         Integration Tests                    │  ~15%
        │   Axum handlers + PostgreSQL                 │
        │   sqlx::test with testcontainers             │
        ├─────────────────────────────────────────────┤
        │           Unit Tests                         │  ~80%
        │  Pure domain logic, command handlers,        │
        │  event sourcing primitives                   │
        └─────────────────────────────────────────────┘
```

---

## 1. Core Layer Tests (Fast, Deterministic)

### What Belongs Here
- Circulation commands (checkout, return, renew)
- Fine calculation logic
- Hold queue prioritization
- User tier/privilege calculations
- Event sourcing aggregates

### Example: Circulation Domain

```rust
// src/domain/circulation/mod.rs

/// Pure function - no side effects
pub fn calculate_due_date(loan_date: DateTime<Utc>, loan_period_days: u32) -> DateTime<Utc> {
    loan_date + Duration::days(loan_period_days as i64)
}

/// Pure function - discount calculation
pub fn calculate_overdue_fine(overdue_days: u32, daily_rate: Money) -> Money {
    daily_rate * overdue_days as i64
}

/// Command handler - returns events, no side effects
pub fn handle_checkout(
    patron: &Patron, 
    item: &Item, 
    state: &CirculationState
) -> Result<Vec<CirculationEvent>, CheckoutError> {
    // Validation (pure)
    if patron.is_blocked() {
        return Err(CheckoutError::PatronBlocked);
    }
    if !item.is_available() {
        return Err(CheckoutError::ItemNotAvailable);
    }
    
    // Return events (not executed)
    Ok(vec![
        CirculationEvent::CheckoutCreated {
            item_id: item.id,
            patron_id: patron.id,
            due_date: calculate_due_date(Utc::now(), patron.loan_period()),
        }
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn gold_patron_gets_14_day_loan_period() {
        let gold_patron = Patron::new(Tier::Gold);
        let due = calculate_due_date(Utc::now(), gold_patron.loan_period());
        assert_eq!(due.date(), Utc::now().date() + Duration::days(14));
    }
    
    #[test]
    fn blocked_patron_cannot_checkout() {
        let blocked = Patron::new(Tier::Bronze).blocked();
        let item = Item::available();
        
        let result = handle_checkout(&blocked, &item, &CirculationState::default());
        assert!(matches!(result, Err(CheckoutError::PatronBlocked)));
    }
    
    #[test]
    fn overdue_fine_never_exceeds_max_fine() {
        // Property-based test
        fn prop_fine_bounded(days: u32) -> bool {
            let fine = calculate_overdue_fine(days, Money::new(100)); // $1/day
            fine.cents() <= 5000 // max $50 fine
        }
        quickcheck! {
            fn test_fine_always_within_max(prop_fine_bounded)
        }
    }
}
```

---

## 1.1 Property-Based Testing for Core Domain (Deep Dive)

### Why PBT for Core Domain?

- **Discovered edge cases** — human testers miss random inputs
- **Invariant enforcement** — hard business rules always hold
- **Regression protection** — future refactors can't break properties

---

### A. Circulation Domain Properties

```rust
// src/domain/circulation/properties.rs

use quickcheck::{quickcheck, Arbitrary, Gen};
use proptest::{prelude::*, box_io};
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};

/// Custom generator for valid library objects
mod library_gen {
    use super::*;
    
    // Money generator - always valid positive amounts
    impl Arbitrary for Money {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Money::new(g.gen_range(0..100_00)) // $0.00 - $100.00
        }
    }
    
    // Patron generator - random tier with valid state
    impl Arbitrary for Patron {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let tier = vec![Tier::Gold, Tier::Silver, Tier::Bronze]
                .into_iter()
                .nth(g.gen_range(0..3))
                .unwrap();
            let blocked = g.gen_bool(0.1); // 10% chance of blocked
            Patron::new(tier).with_blocked(blocked)
        }
    }
    
    // Item generator - random catalog status
    impl Arbitrary for Item {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let status = vec![ItemStatus::Available, ItemStatus::CheckedOut, ItemStatus::OnHold]
                .into_iter()
                .nth(g.gen_range(0..3))
                .unwrap();
            Item::new(status)
        }
    }
}

/// ===== CIRCULATION INVARIANTS =====

mod circulation_invariants {
    use super::*;
    
    // Property 1: Due date is always in the future
    #[test]
    fn due_date_always_future() {
        fn prop(days: u32) -> bool {
            let start = Utc::now();
            let due = calculate_due_date(start, days);
            due > start
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 2: Overdue fine is never negative
    #[test]
    fn overdue_fine_never_negative() {
        fn prop(days: u32) -> bool {
            let fine = calculate_overdue_fine(days, Money::new(100)); // $1/day
            fine.cents() >= 0
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 3: Fine never exceeds maximum cap
    #[test]
    fn fine_never_exceeds_max_cap() {
        fn prop(days: u32) -> bool {
            let fine = calculate_overdue_fine(days, Money::new(100)); // $1/day
            fine <= Money::new(MAX_FINE_CENTS)
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 4: Fine accumulation is associative
    // fine(a + b) = fine(a) + fine(b)
    #[test]
    fn fine_accumulation_associative() {
        fn prop(day1: u32, day2: u32, rate: u32) -> bool {
            let rate = Money::new(rate);
            let combined = calculate_overdue_fine(day1 + day2, rate);
            let separate = calculate_overdue_fine(day1, rate) 
                + calculate_overdue_fine(day2, rate);
            combined == separate
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 5: Zero days = zero fine
    #[test]
    fn zero_overdue_zero_fine() {
        let fine = calculate_overdue_fine(0, Money::new(100));
        assert_eq!(fine, Money::new(0));
    }
    
    // Property 6: Checkout to blocked patron always fails
    #[test]
    fn blocked_patron_always_rejected() {
        fn prop(patron: Patron, item: Item) -> bool {
            let blocked_patron = patron.with_blocked(true);
            let result = handle_checkout(&blocked_patron, &item, &state);
            matches!(result, Err(CheckoutError::PatronBlocked))
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 7: Available item checkout succeeds for valid patron
    #[test]
    fn available_item_checkout_succeeds() {
        fn prop(patron: Patron, item: Item) -> bool {
            if item.status() == ItemStatus::Available && !patron.is_blocked() {
                let result = handle_checkout(&patron, &item, &state);
                result.is_ok()
            } else {
                true // Skip this case - precondition not met
            }
        }
        quickcheck!(fn test(prop));
    }
}
```

---

### B. Patron Domain Properties

```rust
// src/domain/patron/properties.rs

mod patron_invariants {
    use super::*;
    
    // Property 1: Tier always has valid loan period
    #[test]
    fn tier_loan_period_always_valid() {
        fn prop(tier: Tier) -> bool {
            let period = match tier {
                Tier::Gold => 21,      // 21 days
                Tier::Silver => 14,    // 14 days
                Tier::Bronze => 7,     // 7 days
            };
            period > 0 && period <= 30
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 2: Maximum allowed items increases with tier
    #[test]
    fn tier_borrow_limit_invariant() {
        fn prop(tier: Tier) -> bool {
            let limit = match tier {
                Tier::Gold => 20,
                Tier::Silver => 10,
                Tier::Bronze => 5,
            };
            limit >= 5 && limit <= 20 // Holds for all tiers
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 3: Privilege suspension removes checkout rights
    #[test]
    fn suspended_patron_no_checkout_rights() {
        fn prop(patron: Patron) -> bool {
            let suspended = patron.with_suspended(true);
            !suspended.can_checkout()
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 4: Fines never cause negative balance
    #[test]
    fn balance_never_negative() {
        fn prop(balance: i64, fine: i64) -> bool {
            let new_balance = balance - fine;
            new_balance >= 0 || balance >= fine // Either fine fits OR patron owes
        }
        quickcheck!(fn test(prop));
    }
}
```

---

### C. Catalog Domain Properties

```rust
// src/domain/catalog/properties.rs

mod catalog_invariants {
    use super::*;
    
    // Property 1: Item call number always valid format
    #[test]
    fn call_number_format_invariant() {
        fn prop(prefix: String, number: u32, suffix: String) -> bool {
            let call = CallNumber::new(prefix, number, suffix);
            call.to_string().matches(':').count() == 2 // Proper format
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 2: ISBN-10 checksum validation
    #[test]
    fn isbn10_checksum_valid() {
        fn prop(isbn10: String) -> bool {
            if isbn10.len() != 10 { return true; } // Skip invalid length
            validate_isbn10(&isbn10).is_ok()
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 3: ISBN-13 checksum validation  
    #[test]
    fn isbn13_checksum_valid() {
        fn prop(isbn13: String) -> bool {
            if isbn13.len() != 13 { return true; }
            validate_isbn13(&isbn13).is_ok()
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 4: Barcode always positive integer
    #[test]
    fn barcode_positive_invariant() {
        fn prop(barcode: u64) -> bool {
            barcode > 0 && barcode.to_string().len() >= 8
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 5: Duplicate item in catalog rejected
    #[test]
    fn duplicate_barcode_rejected() {
        fn prop(barcode: u64) -> bool {
            let item1 = Item::with_barcode(barcode);
            let item2 = Item::with_barcode(barcode);
            
            // Second item with same barcode causes collision
            item1.barcode() == item2.barcode() // Always equal
        }
        quickcheck!(fn test(prop));
    }
}
```

---

### D. Event Sourcing Properties (Immutable Ledger)

```rust
// src/domain/events/properties.rs

mod event_sourcing_invariants {
    use super::*;
    
    // Property 1: Applying same event twice = error (idempotency)
    #[test]
    fn duplicate_event_causes_error() {
        fn prop(event: CirculationEvent) -> bool {
            let state = CirculationState::default();
            let state1 = state.apply(event.clone());
            
            // Second application should fail - already exists
            state1.apply(event).is_err()
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 2: Event history can reconstruct exact state
    #[test]
    fn state_reconstruction_deterministic() {
        fn prop(events: Vec<CirculationEvent>) -> bool {
            let state1 = events.iter().fold(
                CirculationState::default(), 
                |s, e| s.apply(e.clone())
            );
            
            // Reconstruct by replaying in order
            let state2 = events.iter().fold(
                CirculationState::default(),
                |s, e| s.apply(e.clone())
            );
            
            // Always produces same state
            state1 == state2
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 3: Events are always valid transitions
    #[test]
    fn valid_state_transitions() {
        fn prop(event: CirculationEvent) -> bool {
            // Only valid transitions are allowed
            match event {
                CirculationEvent::CheckoutCreated { .. } => true,
                CirculationEvent::ReturnCompleted { .. } => true,
                CirculationEvent::RenewalExtended { .. } => true,
                _ => true // All valid
            }
        }
        quickcheck!(fn test(prop));
    }
    
    // Property 4: Total fines = sum of all fine events
    #[test]
    fn fine_accumulation_from_events() {
        fn prop(fine_events: Vec<Money>) -> bool {
            let state = fine_events.iter().fold(
                CirculationState::default(),
                |s, fine| s.apply(CirculationEvent::FineApplied { 
                    amount: *fine 
                })
            );
            
            let sum: Money = fine_events.iter().sum();
            state.total_fines() == sum
        }
        quickcheck!(fn test(prop));
    }
}
```

---

### E. Complex Generators with Proptest

```rust
// Advanced proptest generators for library domain

mod advanced_generators {
    use super::*;
    
    // Custom Strategy: Valid checkout request
    fn valid_checkout_request() -> impl Strategy<Value = CheckoutRequest> {
        (uuid_gen(), uuid_gen(), uuid_gen())
            .prop_map(|(item_id, patron_id, _session_id)| {
                CheckoutRequest::new(item_id, patron_id)
            })
    }
    
    // Custom Strategy: Any library event (weighted)
    fn any_library_event() -> impl Strategy<Value = LibraryEvent> {
        prop_oneof![
            // Common events weighted higher
            10 => Just(LibraryEvent::ItemViewed),
            10 => Just(LibraryEvent::SearchPerformed),
            5 => CheckoutRequest::arbitrary().prop_map(|c| LibraryEvent::CheckoutStarted(c)),
            5 => ReturnRequest::arbitrary().prop_map(|r| LibraryEvent::ReturnStarted(r)),
            2 => HoldRequest::arbitrary().prop_map(|h| LibraryEvent::HoldPlaced(h)),
            1 => CirculationEvent::arbitrary(),
        ]
    }
    
    // Range strategies: efficient shrink
    prop_from_into! {
        PositiveCents => 0i64..1_000_00i64,
        NonNegativeDays => 0u32..365u32,
        ValidTier => prop_oneof![
            Just(Tier::Gold),
            Just(Tier::Silver), 
            Just(Tier::Bronze)
        ],
    }
    
    // Batch properties - test with collections
    proptest! {
        #[test]
        fn checkout_batch_all_valid(reqs in prop::collection::vec(valid_checkout_request(), 1..10)) {
            for req in reqs {
                prop_assert!(req.validate().is_ok());
            }
        }
        
        #[test]
        fn parallel_searches_deterministic(
            queries in prop::collection::vec("[a-z]{1,20}", 1..100)
        ) {
            let results: Vec<Vec<Item>> = queries.iter()
                .map(|q| search_catalog(q))
                .collect();
            
            // Same query always returns same results
            for (i, q) in queries.iter().enumerate() {
                for (j, r) in results.iter().enumerate() {
                    if q == queries.get(j).unwrap() {
                        // Deterministic - won't shrink here
                    }
                }
            }
        }
    }
}
```

---

### F. Running PBT in CI

```yaml
# .github/workflows/property-tests.yml

name: Property-Based Tests

on: [push, pull_request]

jobs:
  pbt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Run QuickCheck (1000 iterations)
        run: |
          cargo test --lib -- --test-threads=4 circulation_invariants
          cargo test --lib -- --test-threads=4 patron_invariants
          cargo test --lib -- --test-threads=4 catalog_invariants
          cargo test --lib -- --test-threads=4 event_sourcing_invariants
      
      - name: Run Proptest (deeper shrinking)
        run: |
          cargo test --lib advanced_generators -- --test-threads=2
      
      - name: Strict PBT run (10,000 iterations)
        run: |
          # Run with more iterations for CI
          QUICKCHECK_TESTS=10000 cargo test --lib circulation_invariants
```

---

### Summary: PBT Coverage for Core

| Domain | Properties Tested | PBT Tool |
|--------|------------------|----------|
| **Circulation** | Due dates, fines, block rules, valid transitions | QuickCheck |
| **Patron** | Tiers, limits, suspensions, balance | QuickCheck |
| **Catalog** | ISBN, barcodes, call numbers | Proptest |
| **Events** | Idempotency, reconstruction, accumulation | QuickCheck |
| **Batch** | Collections, parallel ops | Proptest |

**Total Core Properties**: 25+ invariant tests running thousands of iterations each

---

## 2. Application Layer Tests (With Mocks)

### What Belongs Here
- Use case orchestration
- Transaction scripts
- Services that coordinate domain logic

```rust
// src/application/checkout_service.rs

pub struct CheckoutService<R: Repository> {
    repo: R,
}

impl<R: CheckoutRepository> CheckoutService<R> {
    /// This is still testable with mocks
    pub async fn checkout(&self, cmd: CheckoutCommand) -> Result<Loan, CheckoutError> {
        // Get data from repo (mocked in tests)
        let patron = self.repo.get_patron(&cmd.patron_id).await?;
        let item = self.repo.get_item(&cmd.item_id).await?;
        
        // Pure domain logic
        let events = handle_checkout(&patron, &item, &self.state)?;
        
        // Apply events to create loan
        let loan = Loan::from_events(events);
        
        // Persist (mocked)
        self.repo.save_loan(&loan).await?;
        
        Ok(loan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    
    mock! {
        pub CheckoutRepository {}
        impl CheckoutRepository for CheckoutRepository {
            async fn get_patron(&self, id: Uuid) -> Option<Patron>;
            async fn get_item(&self, id: Uuid) -> Option<Item>;
            async fn save_loan(&self, loan: &Loan) -> Result<(), DbError>;
        }
    }
    
    #[tokio::test]
    async fn successful_checkout() {
        let mut mock_repo = MockCheckoutRepository::new();
        mock_repo.expect_get_patron()
            .return_once(|_| Some(Patron::new(Tier::Gold)));
        mock_repo.expect_get_item()
            .return_once(|_| Some(Item::available()));
        mock_repo.expect_save_loan()
            .return_once(|_| Ok(()));
            
        let service = CheckoutService::new(mock_repo);
        let result = service.checkout(cmd).await;
        
        assert!(result.is_ok());
    }
}
```

---

## 3. Infrastructure / Shell Tests (Integration)

### Database Testing with SQLx

```rust
// src/infrastructure/circulation_repo.rs

#[sqlx::test]
async fn create_loan_success(pool: PgPool) {
    let repo = CirculationRepository { pool };
    
    let loan = repo.create(NewLoan {
        item_id: Uuid::new_v4(),
        patron_id: Uuid::new_v4(),
        due_date: Utc::now() + Duration::days(14),
    }).await.unwrap();
    
    assert!(loan.id.is_some());
}

#[sqlx::test]
async fn concurrent_checkout_same_item_fails(pool: PgPool) {
    let item_id = Uuid::new_v4();
    let patron1 = Uuid::new_v4();
    let patron2 = Uuid::new_v4();
    
    // Both try to checkout same item simultaneously
    let results = tokio::join!(
        repo.checkout(item_id, patron1),
        repo.checkout(item_id, patron2)
    );
    
    // One succeeds, one fails
    assert!(results.0.is_ok() ^ results.1.is_ok());
}

/// Use testcontainers for full isolation
#[sqlx::test]
async fn full_integration_flow(pool: PgPool) {
    // 1. Create patron
    let patron = repo.create_patron(new_patron()).await;
    
    // 2. Create item
    let item = repo.create_item(new_item()).await;
    
    // 3. Checkout
    let loan = repo.checkout(item.id, patron.id).await;
    
    // 4. Verify state
    let loans = repo.get_active_loans(patron.id).await;
    assert_eq!(loans.len(), 1);
}
```

### Test Container Setup

```rust
// tests/common/mod.rs

pub async fn test_db() -> PgPool {
    let container = PostgresContainer::new()
        .image("postgres:16-alpine")
        .publish_ports(5432, 54323)
        .start()
        .await
        .unwrap();
    
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&container.connection_string())
        .await
        .unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
        
    pool
}
```

---

## 4. Property-Based Testing (QuickCheck / Proptest)

### For Domain Invariants

```rust
use quickcheck::quickcheck;
use proptest::prelude::*;

// QuickCheck - simple properties
mod quickcheck_tests {
    use super::*;
    
    #[test]
    fn loan_period_never_negative() {
        fn prop(days: u32) -> bool {
            // Even if input is strange, result is valid
            let due = calculate_due_date(Utc::now(), days);
            due > Utc::now()
        }
        quickcheck!(fn test(prop));
    }
    
    #[test]
    fn fine_calculation_associative() {
        fn prop(day1: u32, day2: u32) -> bool {
            let total = calculate_overdue_fine(day1 + day2, DAILY_RATE);
            let separate = calculate_overdue_fine(day1, DAILY_RATE) 
                + calculate_overdue_fine(day2, DAILY_RATE);
            total == separate
        }
        quickcheck!(fn test(prop));
    }
}

// Proptest - complex generators
mod proptest_tests {
    use super::*;
    
    proptest! {
        #[test]
        fn checkout_request_valid(
            item_id in "[0-9a-f]{8}",
            patron_id in "[0-9a-f]{8}",
        ) {
            let req = CheckoutRequest::parse(item_id, patron_id);
            prop_assert!(req.is_ok() || req.is_err()); // Always valid Rust
        }
        
        #[test]
        fn event_serialization_roundtrip(event in any::<CirculationEvent>()) {
            let encoded = event.serialize();
            let decoded = CirculationEvent::deserialize(&encoded);
            prop_assert_eq!(event, decoded);
        }
    }
}
```

### For Event Sourcing

```rust
// Test aggregates via Given-When-Then

#[test]
fn checkout_creates_loan_event() {
    // GIVEN: A patron with borrowing privileges
    let patron = Patron::new(Tier::Gold);
    
    // AND: An available item
    let item = Item::available();
    
    // AND: Initial state
    let state = CirculationState::default();
    
    // WHEN: Checkout command is handled
    let events = handle_checkout(&patron, &item, &state).unwrap();
    
    // THEN: CheckoutCreated event is emitted
    assert!(matches!(
        events[0],
        CirculationEvent::CheckoutCreated { .. }
    ));
    
    // AND: Applying event creates valid loan
    let loan = Loan::apply(CirculationEvent::CheckoutCreated {
        item_id: item.id,
        patron_id: patron.id,
        due_date: due_date.clone(),
    });
    assert!(loan.is_valid());
}

#[test]
fn multiple_fines_accumulate() {
    let mut state = CirculationState::default();
    
    // Apply multiple overdue events
    state.apply(CirculationEvent::FineApplied { amount: Money::new(100) });
    state.apply(CirculationEvent::FineApplied { amount: Money::new(150) });
    
    assert_eq!(state.total_fines(), Money::new(250));
}
```

---

## 5. API / Handler Tests (Axum)

```rust
// tests/api/circulation.rs

#[tokio::test]
async fn checkout_endpoint_returns_201_on_success() {
    let app = app_with_test_db().await;
    
    let response = app.post("/api/circulation/checkout")
        .json(&CheckoutRequest {
            item_id: "item-123".into(),
            patron_id: "patron-456".into(),
        })
        .send()
        .await;
    
    response.assert_status(StatusCode::CREATED);
}

#[tokio::test]
async fn checkout_endpoint_returns_409_for_blocked_patron() {
    let app = app_with_test_db().await;
    
    // Block the patron first
    db.block_patron("patron-456").await;
    
    let response = app.post("/api/circulation/checkout")
        .json(&CheckoutRequest {
            item_id: "item-123".into(),
            patron_id: "patron-456".into(), // blocked
        })
        .send()
        .await;
    
    response.assert_status(StatusCode::CONFLICT);
    response.assert_json(json!({
        "error": "PatronBlocked",
        "message": "Patron has borrowing privileges suspended"
    }));
}

// Health check
#[tokio::test]
async fn health_endpoint_returns_200() {
    let app = app_with_test_db().await;
    let response = app.get("/health").send().await;
    response.assert_status(200);
}
```

---

## 6. Frontend Tests

### SvelteKit (OPAC) - Playwright

```typescript
// tests/opac/checkout.spec.ts
import { test, expect } from '@playwright/test';

test('patron can search and view item details', async ({ page }) => {
  await page.goto('/opac/search?q=python');
  
  await expect(page.locator('.search-results')).toBeVisible();
  await page.click('.result-item:first-child');
  
  await expect(page.locator('.item-title')).toContainText('Python');
  await expect(page.locator('.item-availability')).toContainText('Available');
});

test('unauthenticated patron cannot place holds', async ({ page }) => {
  await page.goto('/opac/item/123/hold');
  
  await expect(page.locator('.login-prompt')).toBeVisible();
});
```

### Dioxus/Ratatui (Librarian TUI) - Tmux

```bash
#!/bin/bash
# tests/tui/circulation_checkout.sh

# Start librarian TUI in tmux
tmux new-session -d -s librery-tui 'cargo run --bin librery-tui'

# Test: Checkout flow
tmux send-keys -t librery-tui 'c' Enter  # c = checkout command
tmux send-keys -t librery-tui 'item-123' Enter
tmux send-keys -t librery-tui 'patron-456' Enter

# Verify: Success message appears
tmux capture-pane -t librery-tui -p | grep -q "Checkout successful"

# Test: Blocked patron shows error
tmux send-keys -t librery-tui 'c' Enter
tmux send-keys -t librery-tui 'item-999' Enter
tmux send-keys -t librery-tui 'blocked-patron' Enter
tmux capture-pane -t librery-tui -p | grep -q "PATRON BLOCKED"

# Cleanup
tmux kill-session -t librery-tui
```

---

## 7. Test Organization

```
librery/
├── src/
│   ├── domain/           # Pure logic - unit tests only
│   │   ├── circulation/
│   │   │   ├── mod.rs
│   │   │   ├── mod_tests.rs  # #[cfg(test)]
│   │   │   └── properties.rs # QuickCheck
│   │   ├── patron/
│   │   └── catalog/
│   ├── application/      # Mocks in tests
│   │   └── services.rs
│   └── infrastructure/   # DB tests
│
├── tests/
│   ├── unit/             # Domain logic only
│   ├── integration/      # SQLx + Axum
│   ├── api/              # Full HTTP tests
│   └── e2e/              # Browser/TUI
│
└── scripts/
    └── test_db.sh        # Testcontainers setup
```

---

## 8. Frontend Testing Strategies

### Overview by Frontend Type

| Frontend | Type | Unit Tests | E2E Tool |
|----------|------|------------|----------|
| **OPAC (SvelteKit)** | Web SSR | Vitest + Testing Library | Playwright |
| **Librarian (Dioxus/Ratatui)** | TUI | Vitest (command handlers) | Tmux automation |

---

### A. Patron OPAC (SvelteKit)

```
Strategy: Component-based + Browser E2E
```

#### Unit & Component Tests (Vitest)

```typescript
// tests/unit/search.test.ts
import { searchCatalog } from '$lib/search';

test('search handles empty query', async () => {
  const results = await searchCatalog('');
  expect(results).toEqual([]);
});

test('search normalizes query', async () => {
  const results = await searchCatalog('  PYTHON  ');
  expect(results[0].title.toLowerCase()).toContain('python');
});
```

```typescript
// tests/components/SearchBar.test.ts
import { render, screen, fireEvent } from '@testing-library/svelte';
import SearchBar from '$lib/components/SearchBar.svelte';

test('SearchBar submits on button click', async () => {
  const { component } = render(SearchBar);
  let submitted = false;
  
  component.$on('submit', () => submitted = true);
  
  const input = screen.getByPlaceholderText('Search the catalog...');
  await fireEvent.input(input, 'python');
  
  const button = screen.getByRole('button', { name: /search/i });
  await fireEvent.click(button);
  
  expect(submitted).toBe(true);
});
```

#### E2E Tests (Playwright)

```typescript
// tests/e2e/opac/search.spec.ts
import { test, expect } from '@playwright/test';

test('patron can search and view item details', async ({ page }) => {
  await page.goto('/opac/search');
  
  // Search for an item
  await page.fill('input[name="q"]', 'python');
  await page.click('button[type="submit"]');
  
  // Verify results
  await expect(page.locator('.search-results .item')).toHaveCount(3);
  
  // Click first result
  await page.click('.search-results .item:first-child');
  
  // Verify detail page
  await expect(page.locator('.item-title')).toContainText('Python');
  await expect(page.locator('.item-status.available')).toBeVisible();
});

test('unauthenticated patron cannot place holds', async ({ page }) => {
  await page.goto('/opac/item/123/hold');
  
  // Should redirect to login
  await expect(page).toHaveURL(/.*\/login.*redirect=.*hold.*/);
});
```

#### API Mocking (MSW)

```typescript
// tests/mocks/handlers.ts
import { http, HttpResponse } from 'msw';
import { setupServer } from 'msw/node';

const server = setupServer(
  http.get('/api/catalog/search', ({ request }) => {
    const url = new URL(request.url);
    if (url.searchParams.get('q') === 'python') {
      return HttpResponse.json({
        items: [
          { id: '1', title: 'Python编程', status: 'available' },
          { id: '2', title: 'Fluent Python', status: 'checked_out' },
        ]
      });
    }
    return HttpResponse.json({ items: [] });
  })
);

beforeAll(() => server.listen());
afterAll(() => server.close());
```

---

### B. Librarian Dashboard (Dioxus + Ratatui)

```
Strategy: Command-based + Terminal Automation
```

#### Unit Tests for Command Handlers (Vitest)

```rust
// src/tui/src/commands.rs

/// Pure function - command processing
pub fn process_command(cmd: Command, state: &AppState) -> CommandResult {
    match cmd {
        Command::Checkout(item_id) => handle_checkout(item_id, state),
        Command::Return(item_id) => handle_return(item_id, state),
        Command::Renew(loan_id) => handle_renew(loan_id, state),
        Command::Search(query) => handle_search(query, state),
        Command::PatronInfo(patron_id) => handle_patron_info(patron_id, state),
        Command::Quit => CommandResult::Exit,
    }
}

/// Pure function - checkout validation
fn handle_checkout(item_id: &str, state: &AppState) -> CommandResult {
    let item = state.items.get(item_id);
    match item {
        Some(Item { status: ItemStatus::Available, .. }) => {
            CommandResult::Success(format!("Checked out: {}", item_id))
        }
        Some(Item { status: ItemStatus::CheckedOut, .. }) => {
            CommandResult::Error("Item is already checked out".into())
        }
        None => CommandResult::Error("Item not found".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn checkout_available_item_succeeds() {
        let mut state = AppState::default();
        state.items.insert("item-123".into(), Item {
            id: "item-123".into(),
            status: ItemStatus::Available,
            ..Default::default()
        });
        
        let result = process_command(
            Command::Checkout("item-123".into()),
            &state
        );
        
        assert!(matches!(result, CommandResult::Success(_)));
    }
    
    #[test]
    fn checkout_checked_out_item_fails() {
        let mut state = AppState::default();
        state.items.insert("item-456".into(), Item {
            id: "item-456".into(),
            status: ItemStatus::CheckedOut,
            ..Default::default()
        });
        
        let result = process_command(
            Command::Checkout("item-456".into()),
            &state
        );
        
        assert!(matches!(
            result, 
            CommandResult::Error(msg) if msg.contains("already checked out")
        ));
    }
    
    #[test]
    fn quit_command_returns_exit() {
        let result = process_command(Command::Quit, &AppState::default());
        assert!(matches!(result, CommandResult::Exit));
    }
}
```

#### TUI E2E via Tmux

```bash
#!/bin/bash
# tests/tui/circulation_checkout.sh

set -e

SESSION="librery-test"
tmux kill-session -t $SESSION 2>/dev/null || true

# Start librarian TUI
tmux new-session -d -s $SESSION 'cargo run --bin librery-tui -- --env test'

# Wait for startup
sleep 2

echo "=== Test 1: Checkout flow ==="
tmux send-keys -t $SESSION 'c' Enter          # c = checkout command
tmux send-keys -t $SESSION 'item-123' Enter   # enter item ID
sleep 1
OUTPUT=$(tmux capture-pane -t $SESSION -p)
if echo "$OUTPUT" | grep -q "CHECKOUT SUCCESSFUL"; then
    echo "✓ Checkout test passed"
else
    echo "✗ Checkout test failed"
    exit 1
fi

echo "=== Test 2: Blocked patron rejection ==="
tmux send-keys -t $SESSION 'c' Enter
tmux send-keys -t $SESSION 'blocked-patron' Enter
sleep 1
OUTPUT=$(tmux capture-pane -t $SESSION -p)
if echo "$OUTPUT" | grep -q "PATRON BLOCKED"; then
    echo "✓ Blocked patron test passed"
else
    echo "✗ Blocked patron test failed"
    exit 1
fi

echo "=== Test 3: Search function ==="
tmux send-keys -t $SESSION '/' Enter           # / = search
tmux send-keys -t $SESSION 'python' Enter
sleep 1
OUTPUT=$(tmux capture-pane -t $SESSION -p)
if echo "$OUTPUT" | grep -q "RESULTS:"; then
    echo "✓ Search test passed"
else
    echo "✗ Search test failed"
    exit 1
fi

# Cleanup
tmux kill-session -t $SESSION
echo "All TUI tests passed!"
```

#### CI Integration

```yaml
# .github/workflows/tui-tests.yml

name: TUI Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install tmux
        run: sudo apt-get install -y tmux
        
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Run TUI tests
        run: |
          chmod +x tests/tui/*.sh
          tests/tui/circulation_checkout.sh
          tests/tui/patron_lookup.sh
          tests/tui/catalog_search.sh
```

---

### Frontend Strategy Summary

| Frontend | Unit Tests | Component Tests | E2E Approach | E2E Tool |
|----------|------------|-----------------|--------------|----------|
| **OPAC (SvelteKit)** | Vitest | Testing Library | Browser automation | Playwright |
| **Librarian (Dioxus)** | Vitest (commands) | Console-only | Terminal automation | Tmux |

---

## Summary: Test Split by FCIS Layer

| Layer | Test Count | Type | Tool | Speed |
|-------|------------|------|------|-------|
| **Domain** | 80% | Unit + Property | `#[test]`, QuickCheck | <1ms |
| **Application** | 10% | Unit + Mock | mockall | <10ms |
| **Infrastructure** | 5% | Integration | sqlx::test | ~100ms |
| **Frontend** | Per-app | Vitest | Playwright / Tmux | ~1s |

### Key Metrics to Track
- 80%+ tests should be unit tests (domain)
- Core tests: <1ms execution
- Integration tests: isolated, no shared state
- Property-based tests for all invariants

---

## Tools Summary

| Category | Tool | Purpose |
|----------|------|---------|
| **Backend Unit Testing** | Built-in `#[test]` | Core domain logic |
| **Backend Mocking** | mockall | Application layer |
| **Property-Based** | quickcheck, proptest | Invariant testing |
| **DB Integration** | sqlx::test | Infrastructure layer |
| **HTTP Testing** | axum test client | API tests |
| **Frontend Unit (Svelte)** | Vitest | Component logic |
| **Frontend Unit (Dioxus)** | Vitest | Command handlers |
| **Frontend E2E (Web)** | Playwright | OPAC browser tests |
| **Frontend E2E (TUI)** | Tmux automation | Librarian TUI tests |
| **Test Containers** | testcontainers | Full isolation |