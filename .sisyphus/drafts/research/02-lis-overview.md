# LIS / Library Management System Market Overview

> Research date: March 2026
> Topic: Library Information Systems

---

## Market Leaders

### Commercial/Enterprise Solutions

| Platform | Vendor | Latest Version | Key Characteristics |
|----------|--------|----------------|---------------------|
| **Alma** | Ex Libris (ProQuest) | Cloud (SaaS) | Leading cloud-native Library Services Platform (LSP), widely used in academic libraries worldwide |
| **Symphony** | SirsiDynix | 5.x | Enterprise-grade ILS used by large public and academic libraries, highly customizable |
| **FOLIO** | EBSCO | Sunflower R1 (2025) | First new LSP launched in 10+ years, open source with commercial support options |
| **Axiell** | Axiell | Various | Strong in European and public library markets, complete suite for libraries, museums, schools |
| **WorldShare WMS** | OCLC | Cloud (SaaS) | Cloud-based ILS tightly integrated with WorldCat |

### Open Source Solutions

| Platform | Latest Version | Key Characteristics |
|----------|----------------|---------------------|
| **Koha** | 25.11.01 (Jan 2026) | First free/open source ILS, web-based, used globally by public, school, academic libraries |
| **SLiMS** | 9.7.x "Bulian" (2025) | Senayan Library Management System, popular in Southeast Asia, lightweight, multi-language |
| **Evergreen** | 3.16.0 (Nov 2025) | Designed for large consortia and multi-branch libraries, powers Georgia's PINES consortium |
| **FOLIO** | Sunflower R1 (Sept 2025) | Modern microservices architecture, cloud-native, modular app-based design |

---

## Core Features

| Category | Features |
|----------|----------|
| **OPAC** | Online Public Access Catalog — patron-facing search interface |
| **Circulation** | Check-in/out, holds, renewals, loan rules, self-service |
| **Cataloging** | MARC records, metadata (Dublin Core, RDA, BIBFRAME), authority control |
| **Patron Management** | Borrower records, categories, privileges, authentication |
| **ERM** | Electronic Resource Management — license tracking, COUNTER stats |
| **Acquisitions** | Vendor management, purchase orders, budgets, serials |
| **Discovery** | Enhanced search, faceted browsing, recommendations |
| **API Integrations** | LDAP, SAML, SIP2, REST APIs |

---

## Architecture Comparison

| Platform | Latest Version | Architecture | Deployment | Target Market |
|----------|----------------|--------------|------------|---------------|
| Alma | Cloud (SaaS) | Cloud-native SaaS | Fully managed | Large academic |
| FOLIO | Sunflower R1 (2025) | Microservices | Self-host or EBSCO-hosted | Mid-large academic |
| Koha | 25.11.01 | Monolithic PHP | Self-host or hosted | Small-mid all types |
| SLiMS | 9.7.x "Bulian" | Monolithic PHP | Self-host (LAMP) | Small-mid, SE Asia |
| Evergreen | 3.16.0 | Monolithic Perl | Self-host or hosted | Large consortia |
| Symphony | 5.x | Monolithic (Java) | On-premise or hosted | Enterprise |

---

## Use Cases by Library Type

### Academic Libraries
- **Typical Platforms**: Alma, FOLIO (Sunflower R1), WorldShare WMS, Symphony
- **Key Needs**: 
  - Complex e-resource management (journals, databases, e-books)
  - Course reserves integration with LMS (Canvas, Blackboard)
  - Interlibrary loan (ILL) via OCLC
  - Research analytics and bibliometrics
  - Student/faculty authentication via institutional SSO

### Public Libraries
- **Typical Platforms**: Koha (25.11), Evergreen (3.16), Symphony, Axiell
- **Key Needs**:
  - Multi-branch management and consortia support
  - Community programming and event management
  - Public computer reservation
  - Digital inclusion services

### School Libraries
- **Typical Platforms**: Koha, SLiMS (9.x), Accessit, Alexandria, Follett Destiny
- **Key Needs**:
  - Integration with student information systems (SIS)
  - Curriculum alignment and reading programs
  - Simple OPAC for young users

---

## Integration Capabilities

### Discovery Layers
- EDS, Summon, Primo, VuFind

### Link Resolvers
- SFX, 360 Link, Alma Link resolver

### Authentication
- CAS, Shibboleth, LDAP, OAuth

### Document Delivery
- REX, Clio, ILL systems

---

## Release Cycles Summary

| Platform | Release Cadence | Naming Convention |
|----------|-----------------|-------------------|
| **Koha** | Yearly + monthly patches | Year.Month (25.11) |
| **SLiMS** | ~1-2 years | Codename (9.x "Bulian") |
| **Evergreen** | ~2x/year + monthly | Version.x (3.16) |
| **FOLIO** | 2x/year | Flower names (Sunflower → Trillium) |

---

## Resources

- Koha: https://koha-community.org
- SLiMS: https://slims.web.id
- FOLIO: https://folio.org
- Evergreen: https://evergreen-ils.org
- Research.com: "Best Library Management Software 2026"
- Virtual World Solutions: "Top 10 Free and Open-Source Library Software 2025"