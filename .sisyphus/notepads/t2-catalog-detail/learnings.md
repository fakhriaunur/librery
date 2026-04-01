# T2 Catalog Detail Endpoints - Learnings

## Repository Pattern
- Follow `ItemRepository` pattern: trait in core, implementation in shell
- Use `async_trait` for async methods in traits
- SQLite schema creation in `ensure_schema` method
- Use `SqlitePoolOptions::new().max_connections(1)` for single-connection repos

## API Pattern
- Use `axum::extract::Path` for URL parameters
- Use `axum::extract::State` for application state
- Return `axum::response::Response` for flexible error handling
- Use `StatusCode` for HTTP status codes
- Use `Json` for JSON responses

## FCIS Architecture
- Core defines traits (interfaces)
- Shell implements traits with concrete infrastructure
- AppState holds all repositories as `Arc<dyn Trait>`
- Dependencies injected via AppState

## Testing
- Use `sqlite::memory:` for in-memory test databases
- Insert test data directly via `sqlx::query`
- Test both success and not-found cases

## OpenAPI Integration
- Add `#[utoipa::path]` attribute to endpoint functions
- Register paths and schemas in `#[derive(OpenApi)]`
- Use `ToSchema` derive for response structs
