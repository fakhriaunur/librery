# Librery - Strategic DDD Context Map

> Generated: March 2026
> Model: Library Management System

---

## CML Model

```cml
// Librery - Strategic DDD Context Map

BoundedContext Circulation {
    type = APPLICATION
    domainVisionStatement = "Core physical state machine managing holds, transits, loans, and checkouts. Event-sourced ledger with strict invariants."
    implementationTechnology = "Rust, Event Sourcing"
    responsibilities = "Checkout, return, renew, holds, loans, physical item state management"
    knowledgeLevel = CONCRETE
}

BoundedContext Catalog {
    type = APPLICATION
    domainVisionStatement = "Bibliographic metadata management supporting MARC21, BIBFRAME, and offline collaborative editing via CRDTs."
    implementationTechnology = "Rust, Automerge (CRDT)"
    responsibilities = "Bibliographic records, item metadata, MARC21 cataloging"
    knowledgeLevel = CONCRETE
}

BoundedContext Actor {
    type = APPLICATION
    domainVisionStatement = "Unified actor management for Patron, Librarian, Admin, and Machine. Handles authentication via Biscuit tokens and RBAC via capabilities."
    implementationTechnology = "Rust, Biscuit Tokens"
    responsibilities = "Authentication, authorization, capabilities, role-based access control"
    knowledgeLevel = CONCRETE
}

BoundedContext Financial {
    type = APPLICATION
    domainVisionStatement = "Fine calculation and billing. Reacts to Circulation events to apply penalty matrices."
    implementationTechnology = "Rust, Event-driven"
    responsibilities = "Fine calculation, lost item billing, payment tracking"
    knowledgeLevel = CONCRETE
}

BoundedContext Search {
    type = APPLICATION
    domainVisionStatement = "CQRS read model providing full-text search via embedded Tantivy index. Fed by Circulation and Catalog."
    implementationTechnology = "Rust, Tantivy"
    responsibilities = "Full-text search, discovery, OPAC search interface"
    knowledgeLevel = CONCRETE
}

BoundedContext Integration {
    type = APPLICATION
    domainVisionStatement = "Edge context hosting WASM plugins for external integrations. Acts as Anti-Corruption Layer for SIP2, legacy migrations."
    implementationTechnology = "Rust, Extism (WASM)"
    responsibilities = "SIP2 protocol, legacy migrations, external system integration"
    knowledgeLevel = CONCRETE
}

ContextMap LibreryContextMap {
    type = SYSTEM_LANDSCAPE
    state = AS_IS

    contains Circulation, Catalog, Actor, Financial, Search, Integration

    // Customer-Supplier: Circulation depends on Actor for eligibility checks
    Actor [ D , CF ]<-<-[ U , CS ] Circulation {
        implementationTechnology = "Rust function calls"
        downstreamRights = INFLUENCER
        exposedAggregates = "Actor,Capabilities"
    }

    // Conformist: Circulation conforms to Catalog's metadata model
    Catalog [ D , CF ]<-<-[ U , OHS ] Circulation {
        implementationTechnology = "Rust read model"
        downstreamRights = INFLUENCER
        exposedAggregates = "BibliographicRecord,Item"
    }

    // Event-Driven: Circulation emits events, Financial reacts
    Circulation [ D , CF ]->[ U , ES ] Financial {
        implementationTechnology = "Domain events"
        downstreamRights = INFLUENCER
    }

    // Conformist: Search reads from both Circulation and Catalog
    Circulation [ D , CF ]<-<-[ U , OHS ] Search {
        implementationTechnology = "Tantivy index"
        downstreamRights = INFLUENCER
    }

    Catalog [ D , CF ]<-<-[ U , OHS ] Search {
        implementationTechnology = "Tantivy index"
        downstreamRights = INFLUENCER
    }

    // Anti-Corruption Layer: Integration protects core contexts from external systems
    Integration [ D , ACL ]<-<-[ U , OHS , PL ] Circulation
    Integration [ D , ACL ]<-<-[ U , OHS , PL ] Catalog
    Integration [ D , ACL ]<-<-[ U , OHS , PL ] Actor
    Integration [ D , ACL ]<-<-[ U , OHS , PL ] Financial {
        implementationTechnology = "WASM plugins (Extism)"
        downstreamRights = INFLUENCER
    }
}
```

---

## Context Map Diagram

```
                    ┌─────────────┐
                    │   Catalog   │ ← (Supporting, Upstream)
                    │  (MARC21)   │
                    └──────┬──────┘
                           │ CF (Conformist)
                           ↓ reads/conforms
    ┌──────────┐     ┌─────────────┐     ┌─────────────┐
    │  Actor   │ ←── │ Circulation │ ──→ │  Financial  │
    │ (Supplier│     │   (Core)    │     │(Event-driven)
    │   C/S)   │     └──────┬──────┘     └─────────────┘
    └──────────┘            │
                           │ events
                           ↓
                    ┌─────────────┐     ┌─────────────┐
                    │   Search    │     │ Integration │
                    │  (Generic)  │     │   (Edge)    │
                    └─────────────┘     │    ACL      │
                           ↑            └─────────────┘
                           │
                    ┌──────┴──────┐
                    │ Integration │
                    │  (WASM)     │
                    └─────────────┘
```

---

## Relationship Legend

| Symbol | Meaning |
|--------|---------|
| **D** | Downstream |
| **U** | Upstream |
| **CS** | Customer-Supplier |
| **CF** | Conformist |
| **ES** | Event-Driven (Pub-Sub) |
| **ACL** | Anti-Corruption Layer |
| **OHS** | Open Host Service |
| **PL** | Published Language |

---

## Context Types Summary

| Context | Type | Notes |
|---------|------|-------|
| **Circulation** | Core | Event-sourced ledger |
| **Catalog** | Supporting | MARC21, CRDT |
| **Actor** | Supporting | Auth + RBAC (Patron/Librarian/Admin/Machine) |
| **Financial** | Supporting | Fines, event-driven |
| **Search** | Generic | CQRS read model (Tantivy) |
| **Integration** | Edge | SIP2, WASM plugins, ACL |

---

## Notes

- **Federation** (OHS for inter-library loan) deferred to v2
- **Observability** handled in Shell infrastructure, not a BC
- **Actor** context includes: Patron, Librarian, Admin, Machine types
- **Origin** field on entities (Local vs Federated) handled within contexts, not a separate BC

---

*Document Version: 1.0*
*Generated: March 2026*