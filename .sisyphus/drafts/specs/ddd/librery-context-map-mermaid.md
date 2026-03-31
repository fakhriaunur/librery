# Librery - Context Map (Mermaid)

```mermaid
flowchart TB
    subgraph Core["CORE"]
        CIRC["Circulation"]
    end

    subgraph Supporting["SUPPORTING"]
        CAT["Catalog"]
        ACT["Actor"]
        FIN["Financial"]
    end

    subgraph Generic["GENERIC"]
        SEARCH["Search"]
    end

    subgraph Edge["EDGE"]
        INT["Integration"]
    end

    %% Relationships
    ACT -->|CS: Customer-Supplier| CIRC
    CAT -->|CF: Conformist| CIRC
    CIRC -->|ES: Event-Driven| FIN
    CIRC -->|CF: Conformist| SEARCH
    CAT -->|CF: Conformist| SEARCH
    INT -->|ACL: Anti-Corruption| CIRC
    INT -->|ACL: Anti-Corruption| CAT
    INT -->|ACL: Anti-Corruption| ACT
    INT -->|ACL: Anti-Corruption| FIN

    %% Styling
    style CIRC fill:#e74c3c,stroke:#c0392b,color:#fff
    style CAT fill:#3498db,stroke:#2980b9,color:#fff
    style ACT fill:#9b59b6,stroke:#8e44ad,color:#fff
    style FIN fill:#f39c12,stroke:#e67e22,color:#fff
    style SEARCH fill:#1abc9c,stroke:#16a085,color:#fff
    style INT fill:#95a5a6,stroke:#7f8c8d,color:#fff
```

---

## Quick Reference

| Context | Role | Tech |
|---------|------|------|
| Circulation | Core | Rust, Event Sourcing |
| Catalog | Supporting | Rust, Automerge |
| Actor | Supporting | Rust, Biscuit |
| Financial | Supporting | Rust, Events |
| Search | Generic | Rust, Tantivy |
| Integration | Edge | Rust, Extism |

---

*Generated: March 2026*