# Librery: Master Research Document

> Consolidated: March 2026
> Project: Next-Generation Library Management System

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Market Analysis](#2-market-analysis)
3. [Competitive Landscape](#3-competitive-landscape)
4. [Technical Architecture](#4-technical-architecture)
5. [Frontend Strategy](#5-frontend-strategy)
6. [Testing Strategy](#6-testing-strategy)
7. [Build & Infrastructure](#7-build--infrastructure)
8. [Development Roadmap](#8-development-roadmap)
9. [Strategic Positioning](#9-strategic-positioning)

---

## 1. Executive Summary

### What is Librery?

Librery is a next-generation, open-source Library Management System (LMS) / Library Information System (LIS) designed to disrupt legacy platforms through:

- **Ultra-lightweight deployment**: Single statically-linked binary
- **Enterprise-grade data integrity**: Event-sourced immutable ledger
- **Edge-optimized**: Runs on constrained hardware (Raspberry Pi, older servers)
- **Zero-administration**: Minimal IT oversight required

### Core Architectural Pattern

```
┌─────────────────────────────────────────────────────────────────┐
│                    Librery Architecture                          │
├─────────────────────────────────────────────────────────────────┤
│  Frontend (Reactive Shells)                                     │
│  ├── Patron OPAC: SvelteKit (RS)                                │
│  ├── Staff App: Flutter + FRB (FCRS)                            │
│  └── Terminal: Ratatui (FCIS)                                   │
├─────────────────────────────────────────────────────────────────┤
│  Backend: Rust + Axum (Imperative Shell)                        │
│  ├── HTTP/3 (QUIC) + HTTP/2 fallback                            │
│  ├── Utoipa (OpenAPI 3.1)                                       │
│  └── Extism (WASM Plugins)                                      │
├─────────────────────────────────────────────────────────────────┤
│  Core: Pure Rust (Functional Core)                              │
│  ├── Event-Sourced Circulation Ledger                           │
│  ├── CQRS + Tantivy Search                                      │
│  └── CRDT Offline Sync                                          │
├─────────────────────────────────────────────────────────────────┤
│  Data: SQLx + PostgreSQL/SQLite                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Target Markets

| Market | Description |
|--------|-------------|
| **Decentralized Digital Frontiers** | Regions with digital adoption but infrastructure/budget challenges (e.g., Indonesian education sector) |
| **Mid-to-Large Consortia** | Organizations seeking escape from vendor lock-in |
| **Resource-Constrained Libraries** | Rural districts, school libraries, community branches |

---

## 2. Market Analysis

### LMS Market Overview

| Platform | Version | Type | Target |
|----------|---------|------|--------|
| **Moodle** | 5.0.6 | Open Source | Higher Ed, Enterprise |
| **Open edX** | Ulmo.1 | Open Source | MOOCs |
| **Docebo** | SaaS | Commercial | Enterprise |
| **Canvas** | SaaS | Commercial | K-12, Higher Ed |
| **Cornerstone** | SaaS | Commercial | Enterprise HR |

**Market Size**: $24.05B (2024) → $70.83B (2030) @ 14% CAGR

### LIS Market Overview

| Platform | Version | Architecture | Target |
|----------|---------|--------------|--------|
| **Alma** | Cloud SaaS | Cloud-native LSP | Large Academic |
| **FOLIO** | Sunflower R1 | Microservices | Mid-Large Academic |
| **Koha** | 25.11.01 | Monolithic PHP | Public, School |
| **SLiMS** | 9.7.x "Bulian" | Monolithic PHP | SE Asia, Schools |
| **Evergreen** | 3.16.0 | Monolithic Perl | Large Consortia |

### Market Gaps Identified

| Gap | Description |
|-----|-------------|
| **Deployment & Maintenance** | Existing systems require complex stacks (LAMP, Perl/Linux). Global deficit in IT admins. |
| **Performance vs. Footprint** | Lightweight systems lack enterprise scaling; scalable systems require massive servers. |
| **Extensibility & Lock-in** | Legacy systems hardcoded to MARC21; custom integrations painful. |

---

## 3. Competitive Landscape

### Comparison Matrix

| System | Architecture | Tech Stack | TCO | Scalability |
|--------|--------------|------------|-----|-------------|
| **SLiMS** | Monolithic | PHP/MySQL | Very Low | Moderate |
| **Koha** | Monolithic | Perl/MySQL | Moderate | High |
| **Evergreen** | Monolithic | Perl/PostgreSQL | High | Extreme |
| **Alma** | Cloud SaaS | Closed | Very High | Extreme |
| **Librery** | FCIS | Rust/PostgreSQL | Very Low | Extreme |

### Librery's Differentiation

| Factor | Legacy Systems | Librery |
|--------|---------------|---------|
| **Deployment** | Multi-daemon stack | Single binary |
| **Offline Sync** | None | CRDT-based |
| **HTTP Support** | HTTP/1.1 | HTTP/3 + fallback |
| **Extensibility** | Hardcoded | WASM plugins |
| **Search Engine** | External (Elasticsearch) | Embedded (Tantivy) |

---

## 4. Technical Architecture

### FCIS Pattern Implementation

```
┌─────────────────────────────────────────────────────────────────┐
│                    Functional Core (Pure Rust)                  │
├─────────────────────────────────────────────────────────────────┤
│  Domain Logic:                                                  │
│  ├── Circulation: Event-Sourced Aggregate                       │
│  ├── Patron: Tier-based privileges                              │
│  ├── Catalog: MARC21/UNIMARC support                            │
│  └── Financial: Fine calculation engine                         │
│                                                                  │
│  Properties:                                                    │
│  ├── Pure functions (no side effects)                           │
│  ├── Mathematically provable invariants                         │
│  └── Testable without mocks                                     │
└─────────────────────────────────────────────────────────────────┘
                              ↓ Commands / ↑ Events
┌─────────────────────────────────────────────────────────────────┐
│                  Imperative Shell (Axum + Rust)                 │
├─────────────────────────────────────────────────────────────────┤
│  Infrastructure:                                                │
│  ├── SQLx (PostgreSQL/SQLite ledger)                            │
│  ├── Quinn (HTTP/3 QUIC) + Hyper (HTTP/2 fallback)              │
│  ├── Utoopa (OpenAPI 3.1 generation)                            │
│  └── Extism (WASM runtime)                                      │
│                                                                  │
│  Responsibilities:                                              │
│  ├── Database I/O                                               │
│  ├── Network transport                                          │
│  ├── Authentication (Biscuit tokens - Patron OPAC & Staff App) │
│  └── Plugin execution                                           │
└─────────────────────────────────────────────────────────────────┘
```

### Network Protocol Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Transport** | QUIC (UDP 443) + TCP fallback | Low-latency, multiplexed |
| **Application** | HTTP/3 → HTTP/2 → HTTP/1.1 | Universal compatibility |
| **Upgrade** | Alt-Svc header injection | Seamless protocol negotiation |
| **Encryption** | TLS 1.3 | Secure communication |

### Data Layer

| Component | Technology | Justification |
|-----------|------------|---------------|
| **Primary Ledger** | SQLx + PostgreSQL/SQLite | Zero-overhead compile-time queries |
| **Search Index** | Tantivy | Embedded full-text search, no external deps |
| **Cache** | Moka | Pure Rust concurrent cache |
| **Event Store** | JSONB in PostgreSQL | Immutable ledger with SQL ACID |

### Plugin Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      Extism WASM Runtime                        │
├─────────────────────────────────────────────────────────────────┤
│  Plugin Types:                                                  │
│  ├── slims-migrator.wasm: Legacy MySQL → Librery events        │
│  ├── marc21-converter.wasm: MARC21 → Librery models            │
│  └── custom integrations: Community-contributed                │
│                                                                  │
│  Security:                                                      │
│  ├── Zero-trust sandbox execution                               │
│  ├── Capability-based manifests                                 │
│  └── No network access by default                               │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. Frontend Strategy

### Taxonomy by User Type

| Frontend | Pattern | Tech Stack | Use Case |
|----------|---------|------------|----------|
| **Patron OPAC** | Reactive Shell (RS) | SvelteKit | Public search, account, holds |
| **Staff App** | Full FCRS | Flutter + FRB | Circulation, cataloging, analytics |
| **Terminal** | Full FCIS | Ratatui | Power users, offline admin |

### Patron OPAC (SvelteKit)

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| **Bundle Size** | 50-100kb | Loads on 3G connections |
| **SEO** | Critical | Google indexing for catalog discovery |
| **Accessibility** | WCAG compliant | Native HTML DOM |
| **Rendering** | SSR + Hydration | Fast first paint, interactive |

### Staff App (Flutter + FRB)

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| **Visualization** | fl_chart | Enterprise-grade charts |
| **Offline** | CRDT (Automerge) | Local edits sync when online |
| **Networking** | Rust reqwest via FFI | HTTP/3 UDP multiplexing |
| **Platforms** | iOS, Android, Desktop | Single codebase |

### Terminal (Ratatui)

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| **Latency** | Zero | Direct Rust core embedding |
| **Offline** | Direct SQLite | No network required |
| **Users** | Power librarians | Keyboard-driven workflows |

---

## 6. Testing Strategy

### TDD Approach: Chicago vs London

| Layer | TDD School | Philosophy | Test Tool |
|-------|------------|------------|-----------|
| **Core** | Chicago (Detroit) | Test-first, pure functions | Unit + Property-based + Mutation (release-gate) |
| **Shell** | London (Mockist) | Outside-in, mock collaborators | Mock tests + Integration |

### Test Pyramid

```
        ┌─────────────────────────────────────────────┐
        │           E2E Tests (Browser/TUI)            │  ~5%
        ├─────────────────────────────────────────────┤
        │         Integration Tests                    │  ~15%
        ├─────────────────────────────────────────────┤
        │           Unit Tests                         │  ~80%
        └─────────────────────────────────────────────┘
```

### Property-Based Testing (Core Domain)

| Domain | Properties | Tool |
|--------|------------|------|
| **Circulation** | Due dates, fines, block rules | QuickCheck |
| **Patron** | Tiers, limits, suspensions | QuickCheck |
| **Catalog** | ISBN, barcodes, call numbers | Proptest |
| **Events** | Idempotency, reconstruction | QuickCheck |

### Frontend Testing

| Frontend | Unit/Component | E2E Tool |
|----------|----------------|----------|
| **OPAC (SvelteKit)** | Vitest + Testing Library | Playwright |
| **Staff App (Flutter)** | Flutter Widget Tests | Flutter Integration Tests |
| **Terminal (Ratatui)** | Vitest | Tmux automation |

---

## 7. Build & Infrastructure

### Build System

| Tool | Purpose |
|------|---------|
| **just** | Command runner (Justfile) |
| **cargo** | Rust workspace |
| **pnpm** | SvelteKit frontend |
| **flutter + FRB** | Staff app |

### Containerization

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Edge Runtime** | Podman | Daemonless, rootless, low overhead |
| **Orchestration** | MicroShift | Lightweight K8s for edge devices |
| **IaC** | OpenTofu | Open-source declarative infra |

### Deployment Artifacts

| Artifact | Contents |
|----------|----------|
| **Server (Edge Node)** | Static Rust binary + Extism + SvelteKit assets |
| **Client (Staff App)** | Flutter APK/IPA/EXE with embedded librery-core |

---

## 8. Development Roadmap

### Phase 0: The First Tracer Bullet

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **0.0** | Hello World | Minimal test passes (verify test infra works) |
| **0.1** | Pure Core | ItemAggregate + CheckOut command. Chicago TDD + proptest |
| **0.2** | Shell/DB/Network | SQLite via SQLx (adapter pattern). Axum POST /api/v1/checkout. HTTP/3 + fallback. Health check |
| **0.3** | Contract | Utoipa generates OpenAPI 3.1 schema at compile time |
| **0.4** | TUI & Client | Ratatui screen. Native Rust reqwest (HTTP/3) hits Axum |

**Success Criteria**: Typing an ID in terminal → HTTP/3 request → Immutable ItemCheckedOut event in SQLite → Verified

**Expected Duration**: **Couple of days** (max)

### Phase 1: Expanding the Ledger & CQRS Search

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **1.1** | Complete Circulation | ItemAggregate: Check-in, Renew, Lost, Holds. cargo-mutants (release-gate) |
| **1.2** | MPSC Actor | Tokio MPSC for lock-free event ordering. Dead letter queue for failures |
| **1.3** | Search Index | Tantivy embedded. Background worker SQLx → Tantivy. Handle eventual consistency |

### Phase 2: The Unified Patron App

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **2.1** | SvelteKit Setup | SvelteKit workspace. openapi-generator-cli via Justfile |
| **2.2** | OPAC Search | Search bar → Tantivy. Browser native fetch. HTTP/3 upgrade |
| **2.3** | Auth Flow | Biscuit Token in HttpOnly cookie. Patron Dashboard protected route |

**Note**: Patron OPAC is online-only (browser). Offline verification is Staff App only.

### Phase 3: The Universal Staff App & Offline CRDTs

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **3.1** | Flutter Init | Flutter workspace. flutter_rust_bridge embeds librery-core |
| **3.2** | Rust-Networking | All Flutter calls route through Rust core. reqwest HTTP/3 |
| **3.3** | Offline Sync | Local SQLite queues commands. Automerge for metadata |
| **3.4** | Dashboard | fl_chart executive dashboard |

### Phase 4: Extensibility & Migration

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **4.1** | Extism Runtime | Embed Extism WASM runtime into Axum |
| **4.2** | CSV Import | Simple CSV → Librery schema. Add rollback/validation |
| **4.3** | SLiMS Migrator | slims-migrator.wasm (lower priority, after CSV) |
| **4.4** | Plugin Security | Capability-based manifests |

### Phase 5: Edge Deployment & Load Testing

| Milestone | Focus | Deliverable |
|-----------|-------|-------------|
| **5.1** | Containerfile | Single static binary. Scratch-based OCI image |
| **5.2** | Infrastructure | OpenTofu provisions MicroShift. Podman containers |
| **5.3** | Load Testing | k6 tests HTTP/3 UDP + MPSC queues. Prove school-hour traffic handling |

---

### Key Decisions

| Decision | Rationale |
|----------|-----------|
| **SQLx adapter pattern** | Works for both SQLite (edge) and PostgreSQL (consortia) in parallel |
| **CSV import before SLiMS** | Simpler column mapping. Faster time-to-value |
| **Phase 0 = couple of days** | Minimal viable slice — just checkout via terminal |
| **Biscuit offline = Staff App only** | Patron OPAC is browser-based (online) |

---

## 9. Strategic Positioning

### Market Gap Addressed

Librery solves the **"Deployment & Maintenance Gap"** that stalls digitalization in underfunded regions:

| Problem | Librery Solution |
|---------|------------------|
| Complex stack (LAMP, Perl/Linux) | Single binary deployment |
| IT admin shortage | Zero-administration |
| Vendor lock-in | Open-source, self-hostable |
| Offline operation | CRDT-based sync |

### Competitive Advantages

| Advantage | Description |
|-----------|-------------|
| **Single Binary** | Compile once, deploy anywhere |
| **HTTP/3 Native** | Optimized for mobile/unreliable networks |
| **Embedded Search** | No external Elasticsearch dependency |
| **WASM Extensibility** | Safe plugin ecosystem |
| **CRDT Offline** | Staff can work without connectivity |

### Target Users

| User Type | Interface | Key Features |
|-----------|-----------|--------------|
| **Patrons** | SvelteKit OPAC | Search, holds, account |
| **Librarians** | Flutter App | Circulation, cataloging, analytics |
| **Power Users** | Ratatui Terminal | Offline admin, batch operations |
| **IT Admins** | MicroShift/OpenTofu | Edge deployment, monitoring |

---

## Appendix: Resources

### Existing Systems Referenced

- Moodle: https://moodle.org
- Open edX: https://openedx.org
- Koha: https://koha-community.org
- SLiMS: https://slims.web.id
- FOLIO: https://folio.org
- Evergreen: https://evergreen-ils.org
- Alma: https://www.exlibrisgroup.com/products/alma

### Research Sources

- SANA Labs: "Best LMS Software on the Market in 2025"
- Docebo: "22 Best Learning Management Systems in 2025"
- Research.com: "Best Library Management Software 2026"
- Virtual World Solutions: "Top 10 Free and Open-Source Library Software 2025"
- CYPHER Learning LMS Trends 2025

---

*Document Version: 1.1*
*Last Updated: March 2026*