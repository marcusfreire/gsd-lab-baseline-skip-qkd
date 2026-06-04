# Walking Skeleton - QKD ETSI 014 Mock + SKIP Baseline

**Phase:** 1
**Regenerated:** 2026-06-04

## Capability Proven End-to-End

A lab integrator can inspect the repository and see the corrected Phase 0
contract set, two-KME local topology, provider independence boundary, SKIP
handoff semantics, and future implementation locations before full service
logic exists.

## Architectural Decisions

| Decision | Choice | Rationale |
|---|---|---|
| KME simulator | Two instances of the official versioned `kms/` Rust simulator: `KME-A` and `KME-B` | Preserves the ETSI 014 topology and avoids hiding both sides behind one central KME. |
| Key Provider implementation | Future Python/FastAPI service under `services/keyprovider/`, instantiated as A and B | Keeps provider logic configurable while preserving separate runtime state. |
| Simulated encryptor implementation | Future Python/FastAPI or CLI scaffold under `services/encryptor-sim/` | Gives SKIP a first consumer before Cisco or real IKEv2/RFC8784 work. |
| Data layer | Separate KME state per KME and separate SQLite database per Key Provider | Preserves independence and exposes synchronization assumptions. |
| Key synchronization | Deterministic fake key-source/seed policy for KME-A and KME-B | Lets identical ETSI `key_ID` values resolve to identical bytes without modeling physical QKD. |
| Auth | mTLS semantics documented; local `x-sae-id` only when mTLS is disabled | Keeps SAE identity as an authorization policy while labeling local HTTP as non-production. |
| Deployment target | Docker Compose local lab runtime | Proves service topology and configuration shape without requiring working service logic. |

## Stack Touched in Phase 1

- [ ] Project contracts - README and `docs/*.md`.
- [ ] Routing - documented ETSI 014 `status`, `enc_keys`, `dec_keys`; documented SKIP `/capabilities`, `/key`, and `/entropy`.
- [ ] Database - documented separate KME and Key Provider state; no schema implementation yet.
- [ ] Service scaffold - `services/keyprovider/` and `services/encryptor-sim/` placeholders only.
- [ ] Deployment - parseable Docker Compose skeleton with `kme-a`, `kme-b`, `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and `encryptor-b-sim`.

## Out of Scope

- Full KME service implementation outside the existing `kms/` simulator.
- Full Key Provider SKIP server logic.
- Full simulated encryptor behavior.
- Real Cisco encryptor integration.
- Real IKEv2/RFC8784 PPK negotiation.
- TLS, mTLS, authentication hardening, and production key protection.
- NetSquid, BB84, QBER, reconciliation, privacy amplification, or quantum-channel modeling.

## Subsequent Slice Plan

Each later phase adds executable behavior on top of this baseline without
casually changing the Phase 0 contracts:

- Phase 2 candidate: Minimal provider state core and KME integration wiring
  against the `kms/` simulator.
- Phase 3 candidate: SKIP Key Provider API implementation with contract tests.
- Phase 4 candidate: Simulated encryptor end-to-end flow proving
  `key_hex_alice == key_hex_bob`.
- Phase 5 candidate: TLS/authentication hardening and RFC8784/Cisco integration
  preparation.
