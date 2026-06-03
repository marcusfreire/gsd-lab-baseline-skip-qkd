# Feature Research

**Domain:** QKD ETSI 014 mock + SKIP Key Provider integration baseline
**Researched:** 2026-06-02
**Confidence:** HIGH for protocol-driven features; MEDIUM for later encryptor integration priorities.

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Architecture document | Lab/research users need a clear mental model before implementation. | LOW | Must show KME mock, KeyProvider-A/B, simulated encryptors, and future IKEv2/RFC8784 boundary. |
| ETSI 014 mock API contract | Key Providers must know how to collect logical QKD keys. | MEDIUM | Should model `status`, master-side key request, and responder-side key retrieval by ID. |
| SKIP API contract pinned to `draft-singh-skip-00` | SKIP is the outward contract to encryptors. | MEDIUM | Must document `/capabilities`, `/key`, `/key/{keyId}`, `/entropy`, JSON fields, and status codes. |
| Deterministic QKD-to-SKIP ID mapping | The project requirement requires `skip_key_id` derived from `qkd_key_id`. | MEDIUM | Must preserve SKIP's rule that the key cannot be recovered from key ID alone. |
| Independent KeyProvider-A/B state | Provider independence is core to the experiment. | MEDIUM | Separate SQLite DBs and no central provider DB. |
| Docker Compose skeleton | Environment must be executable locally. | LOW | Phase 0 only needs initial service topology, ports, env vars, and volumes. |
| Test plan | Research users need reproducible verification. | MEDIUM | Must cover contract tests and future e2e key compatibility checks. |
| Security assumptions | Simulated key material can be misread as production-safe. | LOW | Explicitly state TLS/auth/secret handling limits. |

### Differentiators (Competitive Advantage)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Contract-first SKIP conformance matrix | Prevents "SKIP-like" drift. | MEDIUM | Map each SKIP method/status/field to documentation and future tests. |
| Protocol boundary traceability | Helps researchers trace `qkd_key_id -> keyId -> PPK identity`. | MEDIUM | Include data model and flow diagrams. |
| Explicit future Cisco/IKEv2 seam | Keeps baseline useful for later vendor/device integration. | LOW | Document as future phase without coupling now. |
| Local reproducibility over production hardening | Makes lab validation easy. | LOW | Docker Compose and deterministic IDs support repeatable demos. |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Full ETSI 014 implementation in first milestone | Looks more standards-complete. | Distracts from SKIP integration and increases scope. | Minimal logical mock with documented deviations. |
| Physical QKD simulation | Attractive for QKD research completeness. | Pulls in NetSquid/BB84/QBER and changes the project goal. | Treat KME as logical key source. |
| Central provisioning database | Makes A/B compatibility easy. | Violates independent provider constraint. | Separate stores plus deterministic ID mapping and KME-backed retrieval. |
| Real Cisco encryptor early | Seems like the ultimate validation. | Vendor integration can dominate before contract behavior is proven. | Simulated encryptors first, Cisco later. |
| Real IKEv2/RFC8784 early | Proves end-use directly. | Operationally heavy and not necessary for Phase 0. | Preserve PPK-compatible identifiers and defer IKE implementation. |

## Feature Dependencies

```text
Project Context
    -> Architecture
        -> API Contracts
            -> Data Model
                -> Docker Compose Skeleton
                -> Test Plan

SKIP Contract
    -> Key Provider Service Boundary
        -> Simulated Encryptor Contract Tests
            -> Future Cisco/IKEv2 Integration

ETSI Mock Contract
    -> QKD Key Collection Flow
        -> QKD-to-SKIP Mapping
            -> Provider State Model
```

### Dependency Notes

- **SKIP contract before provider implementation:** The provider's outward API must be shaped by the active SKIP draft, not by convenience routes.
- **ETSI mock contract before KME/provider code:** The mock can be minimal, but its deviations from ETSI 014 must be explicit.
- **Data model before Docker volumes:** SQLite files and service config need stable naming before Compose is useful.
- **Test plan before implementation:** Contract tests are the control mechanism against protocol drift.

## MVP Definition

### Launch With (v1)

- [ ] README overview and local baseline purpose.
- [ ] Architecture document with component boundaries and core flows.
- [ ] ETSI 014 mock API document.
- [ ] SKIP API document pinned to `draft-singh-skip-00`.
- [ ] Data model document covering QKD keys, SKIP keys, provider state, and ID derivation.
- [ ] Test plan covering future automated and manual verification.
- [ ] Security assumptions document.
- [ ] Initial Docker Compose file.
- [ ] Service directory skeleton without full logic.

### Add After Validation (v1.x)

- [ ] Minimal KME mock service implementation.
- [ ] Minimal KeyProvider-A/B SKIP service implementation.
- [ ] Simulated encryptor services/scripts that exercise SKIP.
- [ ] Contract tests for SKIP and ETSI mock behavior.
- [ ] E2E test proving both simulated encryptors receive compatible key material.

### Future Consideration (v2+)

- [ ] TLS and mutual authentication for KME/provider and encryptor/provider links.
- [ ] RFC8784 integration with strongSwan/libreswan or another IKEv2 stack.
- [ ] Cisco encryptor integration.
- [ ] Fuller ETSI 014 compatibility profile.
- [ ] Key lifecycle policies beyond minimal delivery and local state.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Phase 0 documentation set | HIGH | MEDIUM | P1 |
| SKIP conformance matrix | HIGH | MEDIUM | P1 |
| Independent provider state model | HIGH | LOW | P1 |
| Docker Compose skeleton | HIGH | LOW | P1 |
| Minimal KME mock implementation | HIGH | MEDIUM | P2 |
| Minimal Key Provider implementation | HIGH | HIGH | P2 |
| Simulated encryptor e2e flow | HIGH | MEDIUM | P2 |
| Real IKEv2/RFC8784 integration | MEDIUM | HIGH | P3 |
| Cisco integration | MEDIUM | HIGH | P3 |

## Competitor Feature Analysis

This is a research baseline, not a commercial product. The meaningful comparison is against protocol expectations:

| Feature | ETSI GS QKD 014 | SKIP draft | Our Approach |
|---------|------------------|------------|--------------|
| Key delivery API | SAE calls KME for status, new keys, and keys by ID. | Out of scope for KP synchronization mechanism. | KME mock provides logical ETSI-style source; KP exposes SKIP. |
| Key ID transfer | Application communicates key IDs between SAEs outside ETSI 014. | Initiating encryptor passes `keyId` to peer encryptor. | Simulated encryptors model keyId handoff without real IKE. |
| Key material format | ETSI key container uses base64 key data and UUID-style `key_ID`. | SKIP key responses use hex `keyId` and hex `key`. | Keep ETSI and SKIP identifiers/material formats distinct in docs/model. |
| Synchronization mechanism | KME network details are outside API scope. | KP-to-KP synchronization is arbitrary/out of scope. | Use KME mock as logical synchronization source; no central provider DB. |

## Sources

- https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf - ETSI key delivery model and API methods.
- https://datatracker.ietf.org/doc/html/draft-singh-skip-00 - SKIP methods, key flow, status codes, security considerations.
- https://datatracker.ietf.org/doc/html/rfc8784 - future PPK identity and entropy constraints.

---
*Feature research for: QKD ETSI 014 mock + SKIP baseline*
*Researched: 2026-06-02*
