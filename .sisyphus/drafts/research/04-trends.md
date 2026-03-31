# Trends & Recommendations

> Research date: March 2026
> Topic: LMS/LIS Industry Trends, Recommendations, Market Context

---

## 2026 LMS Trends

### Major Industry Shifts

| Trend | Description | Impact |
|-------|-------------|--------|
| **AI-Native Platforms** | AI is now operational, not experimental — adaptive paths, automated grading, content generation | Table stakes, not differentiator |
| **LXP Convergence** | ~70% of enterprise contracts include LXP components | Shift from assigned to self-driven exploration |
| **Skills-Based Learning** | Moving from course completion to skill acquisition | AI-powered skills taxonomy |
| **Microlearning** | Bite-sized modules, just-in-time learning | >90% completion rates |
| **AR/VR Immersive Learning** | Virtual simulations for healthcare, manufacturing | Safe practice environments |
| **Blockchain Credentials** | Verifiable digital badges | Platform: Blockcerts |
| **Hybrid Delivery** | Seamless online/in-person integration | Now the norm |

### Market Growth

- **Global LMS Market**: $24.05B (2024) → projected $70.83B (2030)
- **CAGR**: ~14% (2025-2034)

---

## LIS Trends

### Industry Shifts

1. **Cloud-native platforms** - Alma, FOLIO leading the way
2. **Microservices architecture** - Modular, independently deployable
3. **API-first design** - Critical differentiator
4. **E-resource focus** - Journals, databases, licensing
5. **Discovery layer integration** - VuFind, Primo, EDS

### Market Observations

- FOLIO = first new LSP in 10+ years
- Open source gaining traction (Koha in SMB, FOLIO in academic)
- API capabilities now critical for integration

---

## Recommendations

### For Building an LMS

| Goal | Approach |
|------|----------|
| From scratch | Study Moodle's architecture patterns, plugin system |
| Enterprise focus | AI + HRIS deep integration, compliance tracking |
| SMB focus | Simplified UX, mobile-first, self-service onboarding |
| Open source | PostgreSQL + Python (Django/FastAPI) + React |

### For Building a LIS

| Goal | Approach |
|------|----------|
| SMB/Public library | Koha (free, community, web-based) |
| Modern academic | FOLIO (microservices, open, cloud-native) |
| Enterprise with legacy | Symphony (customizable, proven at scale) |
| Consortia/Multi-branch | Evergreen (Georgia PINES proven) |

---

## Technical Recommendations

### Starting Fresh (Greenfield)

1. **Database**: PostgreSQL (relational, proven, active community)
2. **API Layer**: REST with OpenAPI spec, consider GraphQL overlay
3. **Frontend**: React or Vue.js (component-based, maintainable)
4. **Backend**: Python (FastAPI/Django) or Node.js (TypeScript)
5. **Authentication**: OAuth2/OIDC (keycloak, auth0)
6. **Message Queue**: RabbitMQ or Kafka for async jobs
7. **Deployment**: Docker + Kubernetes (container-native)

### Architecture Patterns to Follow

**Microservices if**:
- Large team (4+ teams)
- Complex domain (distinct LMS vs LMS vs discovery)
- Need independent scaling

**Modular Monolith if**:
- Small team (<5 developers)
- Simpler domain
- Faster initial development

### Key Learnings from Open Source Projects

1. **Moodle**: Plugin architecture drives ecosystem
2. **Open edX**: XBlock = extensibility without core modification
3. **FOLIO**: Okapi gateway pattern enables polyglot services
4. **Koha**: Community-driven, distributed contributions

---

## Summary

| Platform | Lessons |
|----------|---------|
| **Moodle** | Plugin ecosystem is everything |
| **Open edX** | Componentize via XBlock |
| **Koha** | Simple Perl + MySQL works |
| **FOLIO** | Microservices need clear boundaries |

The industry is moving toward **cloud-native, API-first, modular** architectures. Start with clear domain boundaries, build the API layer early, and invest in documentation for community adoption.