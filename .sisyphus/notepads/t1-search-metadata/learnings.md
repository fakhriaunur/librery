# T1 Search Metadata - Learnings

## Pattern: Tantivy Field Extraction
- Use `get_first(field).and_then(|v| v.as_str()).map(|s| s.to_string())` for optional text fields
- Use `get_first(field).and_then(|v| v.as_i64()).unwrap_or(0)` for required i64 fields
- Fields marked TEXT | STORED in schema are available for extraction after search

## Pattern: API Response Struct Updates
- When adding fields to internal structs, check for corresponding API response structs
- Update both the struct definition AND the mapping code
- Use `Option<T>` for fields that may not exist in all document types

## Architecture: FCIS Boundary
- Changes confined to shell layer (librery-shell)
- Core domain (librery-core) untouched
- SearchIndex is part of the CQRS read model in the shell
