# Research Summary

**Domain:** QKD ETSI 014 mock + SKIP Key Provider integration baseline
**Researched:** 2026-06-02
**Confidence:** HIGH

## Key Findings

**Stack:** Python/FastAPI is appropriate for the baseline because the project is API-contract-heavy, local, and test-driven. Use FastAPI with Pydantic models, Uvicorn, HTTPX, pytest, and per-provider SQLite via SQLAlchemy. Docker Compose should be the primary runtime.

**Protocol Boundary:** ETSI GS QKD 014 and SKIP must stay separate. ETSI 014 is the KME-facing key delivery API between SAE and KME. SKIP is the encryptor-facing API exposed by each Key Provider. The Key Provider is the bridge, not a protocol merger.

**ETSI 014 Mock:** The mock should be a minimal logical profile of ETSI GS QKD 014: status, initiator-side key request, and responder-side key retrieval by key IDs. It should document deviations from full ETSI conformance and avoid QKD physics.

**SKIP Contract:** `draft-singh-skip-00` is the pinned SKIP source. The provider-facing SKIP API should include capabilities, key retrieval/generation, key-by-ID retrieval, and entropy. SKIP uses `keyId` and `key` JSON fields with hexadecimal strings; default `keyId` length is 128 bits and default key length is 256 bits.

**RFC8784 Future:** Real IKEv2/RFC8784 stays out of early phases, but the SKIP `keyId` must be treated as the future PPK identity input. RFC8784 considerations make 256-bit entropy and stable peer-specific PPK/key ID mapping important.

**Architecture:** Use one configurable Key Provider implementation instantiated twice as KeyProvider-A and KeyProvider-B. Each instance gets its own config and SQLite DB. Do not introduce a central provider database.

## Table Stakes for Requirements

- Documentation-first Phase 0.
- `README.md`.
- `docs/architecture.md`.
- `docs/api-etsi014-mock.md`.
- `docs/api-skip.md`.
- `docs/data-model.md`.
- `docs/test-plan.md`.
- `docs/security-assumptions.md`.
- `infra/docker-compose.yml`.
- Service directory skeletons.
- Explicit Phase 0 acceptance criteria and risks.

## Recommended Roadmap Shape

1. **Phase 0: Contracts and Scaffolding** - Create docs, technical plan, Compose skeleton, and directories. No full service logic.
2. **Phase 1: KME Mock and Provider State Core** - Implement minimal ETSI mock and per-provider SQLite/state layer.
3. **Phase 2: SKIP Key Provider API** - Implement SKIP endpoints and contract tests against `draft-singh-skip-00`.
4. **Phase 3: Simulated Encryptor E2E Flow** - Add simulated encryptors and automated e2e proof of matching key material.
5. **Phase 4: Hardening and Future Integration Preparation** - Improve security assumptions, TLS/auth plan, lifecycle semantics, and RFC8784/Cisco integration readiness.

## Watch Out For

- Do not make a SKIP-like API; make a SKIP contract matrix.
- Do not use a shared provider database.
- Do not collapse `qkd_key_id`, ETSI `key_ID`, SKIP `keyId`, and RFC8784 PPK identity into one untyped field.
- Do not imply production security from local simulated keys.
- Do not implement real Cisco/IKEv2 before simulated SKIP flow is correct.

## Sources

- https://datatracker.ietf.org/doc/html/draft-singh-skip-00 - active SKIP draft pinned for the project.
- https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf - ETSI GS QKD 014 REST key delivery API.
- https://datatracker.ietf.org/doc/html/rfc8784 - future IKEv2 PPK constraints.
- https://pypi.org/project/fastapi/ - FastAPI current package metadata.
- https://hub.docker.com/_/python/ - official Python Docker image tags.

---
*Research summary for: QKD ETSI 014 mock + SKIP baseline*
*Researched: 2026-06-02*
