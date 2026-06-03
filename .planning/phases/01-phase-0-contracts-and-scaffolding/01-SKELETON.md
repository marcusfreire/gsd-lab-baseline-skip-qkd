# Walking Skeleton - QKD ETSI 014 Mock + SKIP Baseline

**Phase:** 1
**Generated:** 2026-06-02

## Capability Proven End-to-End

A lab integrator can inspect the repository and see the complete Phase 0 contract set, local multi-service topology, provider independence boundary, and future implementation locations before any full service logic exists.

## Architectural Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Framework | Python/FastAPI for future service implementation | Matches the project stack decision and keeps API contracts easy to validate later. |
| Data layer | Separate SQLite database per Key Provider | Preserves KeyProvider-A and KeyProvider-B independence without a central provider database. |
| Auth | Deferred; local HTTP scaffold only | Phase 0 documents TLS/auth as future hardening and avoids production security claims. |
| Deployment target | Docker Compose local lab runtime | Proves service topology and configuration shape without requiring full implementation logic. |
| Directory layout | `services/kme-mock/`, `services/keyprovider/`, `services/encryptor-sim/`, `infra/`, `docs/` | Separates protocol docs, runtime skeleton, and future service implementation locations. |

## Stack Touched in Phase 1

- [ ] Project scaffold - documentation and service directory skeletons only.
- [ ] Routing - documented API routes for ETSI 014 mock and SKIP, no implemented handlers.
- [ ] Database - documented per-provider SQLite paths and volumes, no schema implementation yet.
- [ ] UI - not applicable; the first consumer is a simulated encryptor, not a frontend.
- [ ] Deployment - parseable Docker Compose skeleton for local lab topology.

## Out of Scope (Deferred to Later Slices)

- Full KME mock service logic.
- Full Key Provider SKIP server logic.
- Full simulated encryptor behavior.
- Real Cisco encryptor integration.
- Real IKEv2/RFC8784 PPK negotiation.
- TLS, mTLS, authentication, and production key protection.

## Subsequent Slice Plan

Each later phase adds executable behavior on top of this baseline without changing the Phase 0 contracts casually:

- Phase 2 candidate: Minimal KME mock and provider state core.
- Phase 3 candidate: SKIP Key Provider API implementation with contract tests.
- Phase 4 candidate: Simulated encryptor end-to-end flow.
- Phase 5 candidate: TLS/authentication hardening and RFC8784/Cisco integration preparation.
