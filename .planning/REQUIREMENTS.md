# Requirements: QKD ETSI 014 Mock + SKIP Baseline

**Defined:** 2026-06-02
**Core Value:** Prove, with precise contracts and a runnable local shape, that independent Key Providers can act as SAEs toward separate local KME instances backed by the official versioned `kms/` simulator and as SKIP servers toward encryptors while preserving compatible key material on both sides.

## v1 Requirements

Requirements for the first project release: Phase 0 technical plan, documentation contracts, and service scaffolding before full service logic.

### Documentation

- [x] **DOC-01**: User can read `README.md` to understand the baseline purpose, local topology, Phase 0 scope, and what is intentionally not implemented yet.
- [x] **DOC-02**: User can read `docs/architecture.md` to identify `kms/` as the official KME simulator baseline for KME-A and KME-B, plus KeyProvider-A, KeyProvider-B, simulated encryptors, persistence boundaries, and future IKEv2/RFC8784 boundary.
- [x] **DOC-03**: User can read `docs/api-etsi014-mock.md` to understand the `kms/`-backed minimal logical ETSI GS QKD 014 profile, including status, initiator key request, responder key retrieval by key IDs, request/response examples, and documented deviations from full ETSI 014.
- [x] **DOC-04**: User can read `docs/api-skip.md` to see the SKIP API contract pinned to `draft-singh-skip-00`, including `GET /capabilities`, `GET /key`, `GET /key/{keyId}`, `GET /entropy`, query parameters, JSON fields, and status codes.
- [x] **DOC-05**: User can read `docs/data-model.md` to distinguish ETSI `key_ID`, SKIP `keyId` / `skip_key_id`, key material formats, provider state, and deterministic textual ID mapping.
- [x] **DOC-06**: User can read `docs/test-plan.md` to see planned manual, contract, and end-to-end test scenarios for Phase 0 and later implementation phases.
- [x] **DOC-07**: User can read `docs/security-assumptions.md` to understand local-only assumptions, simulated key limits, TLS/authentication deferrals, logging risks, entropy expectations, and non-production claims.

### Technical Plan

- [x] **PLAN-01**: User can read a Phase 0 technical plan that decomposes tasks, lists dependencies, identifies risks, and defines acceptance criteria before implementation code is written.
- [x] **PLAN-02**: User can see an explicit risk and mitigation for incomplete SKIP conformance against `draft-singh-skip-00`.
- [x] **PLAN-03**: User can see an explicit risk and mitigation for accidental centralization of state between KeyProvider-A and KeyProvider-B.
- [x] **PLAN-04**: User can see an explicit risk and mitigation for confusing the ETSI 014 mock with full ETSI GS QKD 014 conformance.

### Protocol Contracts

- [x] **PROTO-01**: Documentation states that the Key Provider acts as an ETSI 014 mock client toward the KME and as a SKIP server toward the encryptor.
- [x] **PROTO-02**: Documentation states that SKIP `keyId` is deterministically mapped as `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}`, while key material remains non-derivable from the identifier alone.
- [x] **PROTO-03**: Documentation states that ETSI-facing key material and SKIP-facing key material use separate protocol representations and must not be collapsed into one untyped field.
- [x] **PROTO-04**: Documentation states that real Cisco encryptor integration and real IKEv2/RFC8784 integration are future phases, with simulated encryptors used first.

### Scaffolding

- [ ] **SCAF-01**: Repository contains `infra/docker-compose.yml` declaring initial services for `kme-a`, `kme-b`, KeyProvider-A, KeyProvider-B, and simulated encryptor components, with KME services referencing `kms/`.
- [ ] **SCAF-02**: Repository contains service directory skeletons for KME mock, configurable Key Provider implementation, and simulated encryptor implementation.
- [ ] **SCAF-03**: KeyProvider-A and KeyProvider-B scaffolding use separate configuration and separate planned SQLite storage paths or volumes.
- [ ] **SCAF-04**: Phase 0 scaffolding avoids full service logic while making the intended future implementation locations clear.

## v2 Requirements

Deferred to future releases. Tracked but not in current roadmap.

### Minimal Services

- **KME-01**: KME mock can serve the documented minimal ETSI 014 logical API.
- **KME-02**: KME mock can provide matching logical key material to initiator and responder sides.
- **KP-01**: Key Provider can collect logical QKD key material from the KME mock.
- **KP-02**: Key Provider can persist local provider state in SQLite.
- **KP-03**: Key Provider can expose SKIP endpoints that satisfy contract tests against `draft-singh-skip-00`.
- **KP-04**: KeyProvider-A and KeyProvider-B can run independently with separate state.
- **ENC-01**: Simulated Encryptor-A can request a fresh SKIP key from KeyProvider-A.
- **ENC-02**: Simulated Encryptor-B can request matching key material from KeyProvider-B using the SKIP `keyId`.
- **TEST-01**: Automated end-to-end test verifies that both simulated encryptors receive compatible key material.

### Future Integration

- **IKE-01**: The baseline can export or adapt SKIP `keyId` and key material for future RFC8784 PPK usage.
- **TLS-01**: KME/provider and provider/encryptor links can be hardened with TLS and authentication.
- **CISCO-01**: A future phase can integrate with Cisco or another real encryptor without changing the SKIP contract.

## Out of Scope

Explicitly excluded from v1 to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Full service logic | Phase 0 is the technical plan, documentation, and scaffolding gate before implementation code. |
| Full ETSI GS QKD 014 conformance | The first release documents a minimal logical mock profile only. |
| Physical QKD simulation | BB84, QBER, reconciliation, privacy amplification, NetSquid, and quantum channel modeling are not part of the integration baseline. |
| Central provider database | Violates independent KeyProvider-A and KeyProvider-B architecture. |
| Real Cisco encryptor | Deferred until simulated SKIP flow is correct. |
| Real IKEv2/RFC8784 | Deferred until SKIP delivery and PPK identity semantics are validated. |
| Production security claims | Phase 0 documents assumptions and limits; it does not claim production-safe key handling. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DOC-01 | Phase 1 | Complete |
| DOC-02 | Phase 1 | Complete |
| DOC-03 | Phase 1 | Complete |
| DOC-04 | Phase 1 | Complete |
| DOC-05 | Phase 1 | Complete |
| DOC-06 | Phase 1 | Complete |
| DOC-07 | Phase 1 | Complete |
| PLAN-01 | Phase 1 | Complete |
| PLAN-02 | Phase 1 | Complete |
| PLAN-03 | Phase 1 | Complete |
| PLAN-04 | Phase 1 | Complete |
| PROTO-01 | Phase 1 | Complete |
| PROTO-02 | Phase 1 | Complete |
| PROTO-03 | Phase 1 | Complete |
| PROTO-04 | Phase 1 | Complete |
| SCAF-01 | Phase 1 | Pending |
| SCAF-02 | Phase 1 | Pending |
| SCAF-03 | Phase 1 | Pending |
| SCAF-04 | Phase 1 | Pending |

**Coverage:**

- v1 requirements: 19 total
- Mapped to phases: 19
- Unmapped: 0

---
*Requirements defined: 2026-06-02*
*Last updated: 2026-06-02 after initialization*
