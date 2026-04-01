# Phase 1b Learnings

## T5: BibliographicRecord Entity (2026-04-01)

### Patterns Followed
- ID types follow `pub struct TypeName(pub String)` pattern in `ids.rs`
- Domain entities use `#[derive(Debug, Clone, PartialEq, Eq)]`
- Optional fields use `Option<T>` for non-required metadata
- Module exports in `mod.rs` include both `pub mod` and `pub use` statements

### Implementation Notes
- `BibRecordId` added to `ids.rs` following existing pattern
- `BibliographicRecord` struct created with MARC21 support via `Option<String>`
- `publication_year` uses `Option<i32>` for partial catalog records
- Pure domain entity - no I/O, no side effects, no repository yet (T7)

### Verification
- `cargo check -p librery-core` passes
- rust-analyzer hint about unlinked-file is stale (file is properly included in mod.rs)

## T4: Auth Middleware (2026-04-01)

### Biscuit-auth API Usage
- Use `AuthorizerBuilder::new().time().build(&biscuit)` to create authorizer with time fact
- `Biscuit::authorizer()` creates authorizer without policies (query-only)
- `authorizer.query()` returns `Vec<T>` where T implements `TryFrom<Fact>`
- For tuple results: `query::<(String,)>("data($id) <- patron($id)")` or let type inference work with `Vec<(String,)>`

### Axum Middleware Pattern
- Use `axum::middleware::from_fn()` for function-based middleware
- Middleware receives `Request` and `Next`, returns `Result<Response, StatusCode>`
- Access `AppState` from `req.extensions().get::<AppState>()`
- Insert custom data via `req.extensions_mut().insert()`

### Route Protection
- Create separate `Router` for protected routes with middleware layer
- Merge protected router with public routes
- Auth middleware runs before handler, attaches `AuthenticatedPatron` to request extensions

### Files Created
- `librery-shell/src/middleware/auth.rs` - Auth middleware implementation
- `librery-shell/src/middleware/mod.rs` - Module exports

### Files Modified
- `librery-shell/src/lib.rs` - Added `pub mod middleware`
- `librery-shell/src/main.rs` - Added protected routes router with auth middleware

### Verification
- `cargo check -p librery-shell` passes
- All LSP diagnostics clean on changed files
