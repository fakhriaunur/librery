# Librery

> Next-generation library management system built with FCIS architecture in Rust.

---

## Overview

Librery is a modern, modular library management system designed for the cloud-native era. Built using **Functional Core Imperative Shell (FCIS)** architecture in Rust, it provides:

- **Event-sourced circulation** — Complete audit trail with immutable events
- **CQRS search** — Tantivy-powered full-text search
- **Offline-ready** — CRDT sync for disconnected operation
- **Multi-channel** — Staff TUI, Patron Web OPAC, Mobile App

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      SvelteKit OPAC                         │
│                    (Patron Web Interface)                   │
└─────────────────────────────────────────────────────────────┘
           ↑ HTTP/3 + Alt-Svc (with HTTP/2 fallback)
┌─────────────────────────────────────────────────────────────┐
│                       Axum Shell                            │
│                    (HTTP Server + Auth)                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  MPSC Actor │  │   Tantivy   │  │   Biscuit Auth      │ │
│  │  (FIFO)     │  │   Search    │  │   (Tokens)          │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
           ↓ SQLx + SQLite
┌─────────────────────────────────────────────────────────────┐
│                    librery-core                             │
│                   (Pure Domain Logic)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │ Circulation │  │   Actor     │  │     Catalog         │ │
│  │ (Events)    │  │  (Patrons)  │  │   (Biblio Records)  │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## Phases

### Phase 0: First Tracer Bullet ✅
- Checkout command via terminal
- Event-sourced ledger
- HTTP/3 + HTTP/2 server
- OpenAPI schema
- Ratatui TUI

### Phase 1: Expanding the Ledger & CQRS Search ✅
- Complete circulation: check-in, renew, lost, holds
- MPSC actor for strict event ordering
- Dead letter queue with retry
- Tantivy search integration
- Background worker for projection

### Phase 1b.x: Core & Backend Stabilization 🚧
- Patron entity + authentication
- Item + BibliographicRecord entities
- Extended Tantivy index for bibliographic search
- Patron loans/holds API

### Phase 2: Unified Patron App (Planned)
- SvelteKit OPAC (public)
- Authenticated patron dashboard
- Biscuit token authentication

### Phase 3: Staff App & Offline CRDTs (Planned)
- Flutter+FRB Staff App
- Offline CRDT sync
- Full cataloging capabilities

---

## Quick Start

### Prerequisites
- Rust 1.75+
- SQLite
- just (build automation)

### Build & Run

```bash
# Clone and enter directory
cd librery

# Build workspace
just build

# Run the shell (HTTP server)
just run-shell

# Run tests
just test
```

### Database

```bash
# Create SQLite database
sqlite3 librery.db

# Run migrations (future)
just migrate
```

---

## Project Structure

```
librery/
├── librery-core/          # Pure domain logic (no I/O)
│   └── src/domain/        # Entities, aggregates, events
├── librery-shell/         # Axum HTTP server
│   └── src/api/           # HTTP endpoints
│   └── src/search/        # Tantivy integration
│   └── src/actor/         # MPSC event dispatcher
├── librery-tui/           # Ratatui terminal interface
├── librery-opac/          # SvelteKit patron app (Phase 2+)
└── librery-staff/         # Flutter staff app (Phase 3+)
```

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Core | Rust, Event Sourcing |
| HTTP | Axum,HTTP/3 (quinn), HTTP/2 (hyper) |
| Database | SQLx, SQLite |
| Search | Tantivy (embedded) |
| Auth | Biscuit Tokens |
| TUI | Ratatui |
| OPAC | SvelteKit |
| Staff App | Flutter + FRB |

---

## Documentation

- [DDD Context Map](.sisyphus/drafts/specs/ddd/librery-context-map.md)
- [Tactical DDD](.sisyphus/drafts/specs/ddd/librery-tactical-ddd.md)
- [Research: LMS/LIS Market](.sisyphus/drafts/research/01-lis-market.md)
- [Testing Strategy](.sisyphus/drafts/research/05-testing-strategy.md)

---

## Contributing

1. Fork the repo
2. Create a feature branch
3. Follow FCIS architecture principles
4. Write tests first (TDD)
5. Submit PR

---

## License

Apache 2.0 — See LICENSE file