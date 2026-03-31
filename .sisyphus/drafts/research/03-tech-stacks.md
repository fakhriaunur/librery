# Tech Stacks & Software Architecture

> Research date: March 2026
> Topic: Open Source LMS/LIS Technical Architecture

---

## Moodle (LMS)

### Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | PHP 8.1+ |
| **Framework** | Custom MVC (PHP-based) |
| **Database** | MySQL/MariaDB, PostgreSQL, Oracle, Microsoft SQL Server |
| **Frontend** | JavaScript, jQuery, Mustache templates |
| **Caching** | Redis, Memcached |
| **Session** | Database or file-based |

### Current Version

- **Latest**: Moodle 5.0.6 (February 2026), 5.1 (October 2025)
- **Next**: Moodle 5.2 (unreleased, code freeze March 2026, release April 2026)
- **LTS**: Moodle 4.5 (Long-term support)

### Architecture Pattern

- **Monolithic PHP application**
- Modular plugin architecture (2000+ plugins)
- Event-driven hooks system
- Theme system with Bootstrap base

### Key Components

```
moodle/
├── lib/              # Core libraries
├── mod/              # Activity modules
├── blocks/           # Dashboard blocks
├── theme/            # Theme files
├── course/           # Course management
├── enrol/            # Enrollment plugins
└── auth/             # Authentication plugins
```

### Deployment

- Self-hosted (LAMP/LEMP stack)
- MoodleCloud (SaaS)
- Enterprise support via Moodle HQ

---

## Open edX (LMS)

### Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | Python 3.10+ |
| **Framework** | Django (multiple services) |
| **Database** | MySQL (learner data), MongoDB (course content) |
| **Frontend** | React (MFEs), Mako templates (legacy) |
| **Caching** | Redis, Memcached |
| **Message Queue** | RabbitMQ |
| **Search** | Elasticsearch |

### Current Version

- **Latest**: Open edX "Ulmo.1" (January 2026)
- **Previous**: "Ulmo" (December 2025)
- Releases every 6 months with flower-inspired codenames

### Architecture Pattern

- **Microservices / Independently Deployable Applications (IDAs)**
- API-first design
- XBlock framework for custom learning components
- Event-driven analytics pipeline

### Key Components

| Component | Purpose |
|-----------|---------|
| **edx-platform** | Core LMS + Studio |
| **XBlock** | Custom learning components |
| **Analytics Pipeline** | Data processing |
| **Chrome LMS** | React-based MFE |
| **Discovery** | Course search |
| **Credentials** | Certificate generation |

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Open edX Platform                        │
├─────────────────────────────────────────────────────────────┤
│  Frontend MFEs (React)                                      │
│  ├── Learning MFE                                           │
│  ├── Account MFE                                            │
│  ├── Gradebook MFE                                          │
│  └── Discovery MFE                                          │
├─────────────────────────────────────────────────────────────┤
│  Backend Services (Django)                                  │
│  ├── LMS (Learning Management System)                       │
│  ├── Studio (Course Authoring)                              │
│  ├── XBlock Runtime                                         │
│  └── Authentication (OAuth2/OIDC)                           │
├─────────────────────────────────────────────────────────────┤
│  Data Layer                                                 │
│  ├── MySQL (learner data)                                   │
│  ├── MongoDB (course content)                               │
│  ├── Elasticsearch (search)                                 │
│  └── S3/Cloud Storage (media)                               │
└─────────────────────────────────────────────────────────────┘
```

### Deployment

- Native Kubernetes support ( tutor )
- AWS/Azure marketplace
- OpenStack deployment
- Edly, EduNEXT ( hosted options )

---

## Koha (LIS)

### Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | Perl |
| **Database** | MySQL/MariaDB |
| **Web Server** | Apache (Plack/PSGI) |
| **Frontend** | JavaScript, jQuery, Bootstrap 3 |
| **Templates** | Template::Toolkit (TT2) |

### Current Version

- **Latest**: Koha 25.11.01 (January 2026), 25.05.08 (February 2026)
- **Release Cycle**: Year.Month format (25.11 = November 2025)
- Yearly release with monthly maintenance patches

### Architecture Pattern

- **Monolithic Perl CGI application**
- REST API via Plack
- MySQL database with stored procedures

### Key Modules

```
koha/
├── Koha/             # Core Perl modules
├── koha-tmp/         # Background jobs
├── C4/               # Legacy kernel
├── Koha/Schema/      # DBIC ORM
└── t/                # Tests
```

### Deployment

- Debian/Ubuntu packages
- Community hosted (Koha US, New Zealand)
- Docker containers available

---

## FOLIO (LIS)

### Technology Stack

| Layer | Technology |
|-------|------------|
| **Backend** | Node.js, Java (modules vary) |
| **Frontend** | React, Stripes framework |
| **Database** | PostgreSQL (per-module) |
| **API Gateway** | Okapi |
| **Message Queue** | Kafka (optional) |
| **Authentication** | OAuth2, JWT |

### Current Version

- **Latest**: FOLIO "Sunflower R1 2025" (GA: September 2025)
- **Next**: "Trillium R1 2026" (GA: March 2026)
- **Release Cycle**: 2x/year with flower-inspired codenames
- **Support Window**: ~9 months per release

### Architecture Pattern

- **Microservices with shared API gateway**
- Modular "app" architecture
- Tenant-isolated multi-tenancy

### Key Modules

| Module | Purpose |
|--------|---------|
| **Okapi** | API Gateway / proxy |
| **mod-users** | Patron management |
| **mod-inventory** | Catalog management |
| **mod-circulation** | Check-in/out, loans |
| **mod-acquisitions** | Acquisitions |
| **mod-orders** | Order management |
| **mod-permissions** | Access control |

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         FOLIO Platform                           │
├─────────────────────────────────────────────────────────────────┤
│  Frontend (Stripes/React)                                       │
│  └── Apps: Users, Inventory, Circulation, Acquisitions, etc.   │
├─────────────────────────────────────────────────────────────────┤
│  Okapi (API Gateway) ───────────────────────────────────────►   │
│     │  Routes requests to appropriate module                   │
│     │  Tenant isolation, authentication                        │
├─────────────────────────────────────────────────────────────────┤
│  Backend Modules (Individually Deployed)                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │
│  │ mod-users   │ │mod-inventory│ │mod-circulation             │
│  │  (Node.js)  │ │ (Node.js)   │ │  (Node.js)                 │
│  └─────────────┘ └─────────────┘ └─────────────┘               │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │
│  │mod-acq      │ │ mod-orders  │ │ mod-patron  │               │
│  └─────────────┘ └─────────────┘ └─────────────┘               │
├─────────────────────────────────────────────────────────────────┤
│  PostgreSQL (one per module)                                    │
└─────────────────────────────────────────────────────────────────┘
```

### Deployment

- EBSCO-hosted (SaaS)
- Self-host via Kubernetes/Docker
- Community support

---

## SLiMS (LIS)

### Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | PHP 8.0+ |
| **Framework** | Custom PHP (Slim framework) |
| **Database** | MySQL/MariaDB |
| **Frontend** | JavaScript, jQuery, Bootstrap 5 |
| **Templates** | Smarty |
| **API** | REST API, OAI-PMH |

### Architecture Pattern

- **Monolithic PHP application**
- Modular plugin architecture
- OAI-PMH for metadata harvesting
- MODS XML interoperability

### Key Components

```
slims9_bulian/
├── lib/                  # Core libraries
├── modules/              # Functional modules
├── template/             # Smarty templates
├── admin/                # Admin panel
├── opac/                 # Public access catalog
├── install/              # Installer
└── sysconfig/            # System configuration
```

### Features

- Bibliography database (MARC support)
- Circulation management
- Membership management
- Serial control
- OAI-PMH server/client
- Multi-language support (Indonesian, English, others)
- Z39.50 client
- Enhanced UI/UX in 9.x

### Deployment

- LAMP stack (Linux, Apache, MySQL, PHP 8)
- Self-hosted
- Popular in Southeast Asia (Indonesia, Thailand, Malaysia)

### Version History

| Version | Codename | Status |
|---------|----------|--------|
| **9.x** | Bulian | Current (9.7.x) |
| 8.x | Akasia | Previous stable |
| 7.x | Cendana | Legacy |
| 5.x | Meranti | Legacy |

- **Latest**: SLiMS 9.7.x "Bulian" (2025)
- GitHub: https://github.com/slims/slims9_bulian

---

## Evergreen (LIS)

### Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | Perl (OpenEDI) |
| **Database** | PostgreSQL |
| **Web Server** | Apache with CGI |
| **Frontend** | JavaScript, jQuery |
| **API** | REST API |

### Current Version

- **Latest**: Evergreen 3.16.0 (November 2025)
- **Release Cycle**: Feature releases ~2x/year, monthly maintenance patches

### Architecture Pattern

- **Monolithic Perl application**
- Highly scalabale for consortia
- PostgreSQL with extensive stored procedures

### Key Components

- **Open-ILS**: Core library services
- **EGC**: Staff client (JavaScript)
- **OPAC**: Public access catalog
- **Reporter**: Reporting engine

### Deployment

- Linux (Debian/RHEL)
- High availability configurations
- Georgia PINES consortium (largest US public library network)

---

## Comparison Summary

| Platform | Architecture | Language | Database | Frontend | Latest Version |
|----------|--------------|----------|----------|----------|---------------|
| **Moodle** | Monolithic | PHP 8.1 | MySQL/PostgreSQL | jQuery/JS | 5.0.6 (Feb 2026), LTS: 4.5 |
| **Open edX** | Microservices | Python 3.10 | MySQL/MongoDB | React MFEs | Ulmo.1 (Jan 2026) |
| **Koha** | Monolithic | Perl | MySQL | Bootstrap/jQuery | 25.11.01 (Jan 2026) |
| **SLiMS** | Monolithic | PHP 8.0 | MySQL | Bootstrap 5/jQuery | 9.7.x "Bulian" (2025) |
| **FOLIO** | Microservices | Node.js + Java | PostgreSQL | React/Stripes | Sunflower R1 (Sept 2025) |
| **Evergreen** | Monolithic | Perl | PostgreSQL | jQuery | 3.16.0 (Nov 2025) |

---

## Enterprise Architecture Patterns Observed

### 1. Monolithic (Legacy)
- Moodle, Koha, SLiMS, Evergreen
- Simpler deployment, harder to scale independently
- Strong plugin ecosystems

### 2. Microservices (Modern)
- FOLIO, Open edX components
- Independent scaling per service
- Loose coupling via API gateway
- Complex operational requirements

### 3. Hybrid Approach
- Open edX (some legacy Django + React MFEs)
- Migration path: extract modules into microservices

### Key Takeaways

1. **Database choice**: MySQL/PostgreSQL dominant (relational)
2. **Frontend evolution**: jQuery → React/Angular (modern MFE pattern)
3. **API first**: All platforms moving toward REST APIs
4. **Event-driven**: Message queues for async processing
5. **Containerization**: Docker/K8s becoming standard for deployment