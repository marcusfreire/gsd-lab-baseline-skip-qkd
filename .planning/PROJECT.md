# QKD ETSI 014 Mock + SKIP Baseline

## What This Is

This project is an executable local baseline for integrating a logical ETSI GS QKD 014 mock API, independent SKIP Key Providers, and simulated encryptors. It is aimed primarily at lab/research users who need a reproducible environment to validate how QKD-derived key material can be collected by Key Providers and exposed through SKIP for future IKEv2/RFC8784 PPK use.

The first milestone is deliberately pre-implementation: it defines the architecture, API contracts, data model, security assumptions, test plan, Docker Compose skeleton, and service directory structure before service logic is built.

## Core Value

Prove, with precise contracts and a runnable local shape, that independent Key Providers can act as ETSI 014 mock clients toward a simulated KME and as SKIP servers toward encryptors while preserving compatible key material on both sides.

## Requirements

### Validated

(None yet - ship to validate)

### Active

- [ ] Define a documented baseline architecture for KME mock, KeyProvider-A, KeyProvider-B, simulated encryptors, and future IKEv2/RFC8784 PPK use.
- [ ] Document a simple logical ETSI GS QKD 014 mock API focused on QKD key collection by Key Providers, not physical QKD simulation.
- [ ] Document the SKIP API according to `draft-singh-skip-00` as the project-pinned SKIP contract.
- [ ] Define KeyProvider-A and KeyProvider-B as independent services with separate local state and no shared central database between providers.
- [ ] Define the Key Provider dual role: ETSI 014 mock API client toward the simulated KME and SKIP server toward the encryptor.
- [ ] Define deterministic `skip_key_id` / SKIP `keyId` derivation from `qkd_key_id`, while ensuring key material cannot be derived from the ID alone.
- [ ] Define local SQLite persistence for each Key Provider.
- [ ] Define simulated encryptors as the first consumers of SKIP; Cisco or other real encryptor integration is deferred.
- [ ] Create Phase 0 documentation deliverables: `README.md`, `docs/architecture.md`, `docs/api-etsi014-mock.md`, `docs/api-skip.md`, `docs/data-model.md`, `docs/test-plan.md`, and `docs/security-assumptions.md`.
- [ ] Create initial infrastructure and repository shape: `infra/docker-compose.yml` and service directories without full service logic.
- [ ] Define Phase 0 risks, task decomposition, and acceptance criteria before writing service implementation code.

### Out of Scope

- Physical QKD simulation, BB84, QBER, reconciliation, privacy amplification, NetSquid, quantum-channel modeling, and hardware QKD integration - the baseline only simulates logical key availability.
- Full ETSI GS QKD 014 conformance - Phase 0 documents only the minimal logical API needed by the Key Providers.
- Real IKEv2/IPsec/RFC8784 integration - future phase after SKIP-facing behavior is validated with simulated encryptors.
- Cisco or other real encryptor integration - future phase after the SKIP contract and simulated encryptor workflow are stable.
- Central shared database between KeyProvider-A and KeyProvider-B - provider independence is a core constraint.
- Production-grade secret handling, TLS hardening, authentication, authorization, and operational security - documented as assumptions and future hardening, not claimed by the baseline.

## Context

The project starts from an existing `baseline.md` concept document describing integration between an ETSI 014-style QKD mock layer, independent Key Providers, SKIP, and future RFC8784 PPK usage. The baseline question is: how can a QKD key source exposed through an ETSI GS QKD 014-like interface feed a SKIP-based PPK delivery path without coupling encryptors directly to QKD infrastructure?

The KME simulation should expose only a simple logical API that represents key availability. It should support the behavior needed for a provider on the initiator side to collect fresh key material and for the responder side to collect corresponding material by identifier.

Each Key Provider is an independent process and service. KeyProvider-A and KeyProvider-B must not depend on a central shared database. Each provider maintains its own local SQLite state and exposes SKIP to its co-located simulated encryptor.

The SKIP-facing contract is pinned to `draft-singh-skip-00`, the active successor to the archived `draft-cisco-skip-02`. Phase 0 documentation should explicitly reference `draft-singh-skip-00` and treat `draft-cisco-skip-02` only as historical context if mentioned.

The first implementation target uses Python/FastAPI, REST APIs, a small CLI where useful for inspection/debug, SQLite for provider state, and Docker Compose as the primary local runtime path.

## Constraints

- **Stack**: Python/FastAPI for services - chosen for fast API iteration, testability, SQLite support, and simple local service scaffolding.
- **Runtime**: Docker Compose is the primary local runtime - the baseline must be easy to run as multiple local services.
- **Persistence**: Each Key Provider uses its own SQLite database - provider state must survive restarts without introducing a central provider database.
- **Protocol Contract**: SKIP behavior follows `draft-singh-skip-00` - endpoint names, JSON fields, status codes, `localSystemID`, `remoteSystemID`, `keyId`, `key`, and entropy behavior must be documented against that draft.
- **ETSI Scope**: ETSI GS QKD 014 is simulated logically - only the KME behaviors required for key collection are modeled in the first baseline.
- **Identifier Mapping**: `skip_key_id` / SKIP `keyId` is deterministically derived from `qkd_key_id` - the derivation must be stable, documented, collision-aware, and must not reveal key material.
- **Security**: Simulated key material is not production-protected - security assumptions must explicitly avoid implying production readiness.
- **Phase 0 Boundary**: No full service logic before the technical plan and documentation baseline exist - the first phase is documentation, scaffolding, and acceptance criteria.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Build an executable integration environment, not only a library or test harness | The goal is to validate service boundaries and end-to-end local shape | - Pending |
| Use a minimal logical ETSI 014 mock, not full ETSI conformance or physical QKD simulation | The integration point is key collection and delivery, not QKD physics | - Pending |
| Implement KeyProvider-A and KeyProvider-B as independent processes | Provider independence and no central shared state are core to the architecture | - Pending |
| Use SQLite per Key Provider | Local persistence is needed for inventory/restarts while keeping services independent | - Pending |
| Pin SKIP contract to `draft-singh-skip-00` | The earlier Cisco draft is archived; the project should track the active successor | - Pending |
| Derive SKIP `keyId` deterministically from `qkd_key_id` | Enables reproducible mapping between QKD and SKIP domains while keeping IDs distinct | - Pending |
| Use simulated encryptors first | Real Cisco/encryptor integration should not block baseline protocol validation | - Pending |
| Defer IKEv2/RFC8784 real integration | RFC8784 is the future consumer path, but Phase 0 and early phases validate SKIP delivery first | - Pending |
| Use Docker Compose as primary runtime | Multiple services need to run together with explicit ports and configuration | - Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check - still the right priority?
3. Audit Out of Scope - reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-06-02 after initialization*
