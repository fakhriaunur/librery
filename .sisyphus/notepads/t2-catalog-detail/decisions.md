# T2 Catalog Detail Endpoints - Decisions

## Decision 1: BibliographicRecordRepository Trait Location
- **Choice**: Added trait to `librery-core/src/lib.rs`
- **Rationale**: Follows existing pattern where repository traits are defined in core
- **Alternative**: Could have been in domain module, but lib.rs is the established location

## Decision 2: Availability Calculation
- **Choice**: Filter items by `bib_id` and return all matching items with status
- **Rationale**: Simple approach that provides full availability picture
- **Alternative**: Could have aggregated to single availability status, but multiple items per bib is realistic

## Decision 3: Route Protection
- **Choice**: Added catalog endpoint to protected routes (requires auth)
- **Rationale**: Consistent with other data endpoints
- **Alternative**: Could be public for OPAC, but auth layer can be adjusted later

## Decision 4: Response Structure
- **Choice**: Return full bibliographic record plus availability array
- **Rationale**: Single call provides all needed information
- **Alternative**: Could have separate endpoints for bib details and availability
