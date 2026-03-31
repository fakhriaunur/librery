# **Librery: Domain Context Map**

This document defines the Bounded Contexts within Librery and the strategic relationships between them, ensuring a decoupled, scalable architecture aligned with Domain-Driven Design (DDD) principles.

## **1\. Bounded Contexts & Types**

| Context Name | Type | Primary Responsibility | Architectural Strategy |
| :---- | :---- | :---- | :---- |
| **Circulation** | **Core** | Core physical state machine (holds, transits, loans). | Event-Sourced Ledger, strict invariants. |
| **Cataloging** | Supporting | Bibliographic metadata (MARC21, BIBFRAME). | CRDTs (Automerge) for offline collaborative editing. |
| **Patron Mgmt** | Supporting | PII, authentication (Biscuit), capabilities. | Strict lifecycle/anonymization enforcement. |
| **Financial** | Supporting | Fine calculation, lost item billing. | Event listener (reacts to Circulation events). |
| **Discovery (OPAC)** | Generic | Patron search interface. | CQRS Read Model (Tantivy embedded index). |
| **Integration** | Edge | SIP2, NCIP, Legacy Migrations. | WASM Plugin host environment. |

## **2\. Context Relationships Map**

The following relationships define how contexts interact and protect their internal models.

### **Anti-Corruption Layer (ACL)**

* **Integration ? Core Contexts:** External protocols (SIP2, Z39.50) and legacy migrations are isolated behind WASM plugins acting as an ACL. They translate messy external formats into pure Librery::Commands before they hit the Functional Core.

### **Customer/Supplier (C/S)**

* **Circulation (Customer) ? Patron Mgmt (Supplier):** Circulation depends on Patron Mgmt for eligibility checks. Patron Mgmt must provide a stable API (e.g., check\_eligibility(patron\_id) \-\> Result\<Capabilities\>).

### **Conformist (CF)**

* **Circulation ? Cataloging:** Circulation needs item metadata but does not dictate how it is structured. It conforms to the read models published by the Cataloging context.

### **Event-Driven / Pub-Sub**

* **Circulation ? Financial:** Circulation does not calculate money. It emits domain events (e.g., ItemReturnedLate). The Financial context listens to these events and applies penalty matrices independently.

### **Open Host Service (OHS) & Published Language (PL)**

* **Librery (Local) ? Federated Peers:** Librery exposes inter-library loan capabilities via a strict OHS and PL. Federated partners (or their WASM adapters) must speak this language to interact with the local instance.
