# Phase 0 Technical Plan

## Objective

Phase 0 establishes the local QKD ETSI 014 + SKIP baseline before full service
logic is implemented. The deliverables are authoritative documentation,
protocol contracts, security assumptions, verification criteria, a Docker
Compose skeleton, and future service directories.

The baseline uses two logical ETSI 014 KME simulator instances, `KME-A` and
`KME-B`, both sourced from the official versioned Rust `kms/` directory.
`KeyProvider-A` acts as `SAE-A` toward `KME-A`; `KeyProvider-B` acts as
`SAE-B` toward `KME-B`. The providers remain independent and use separate
SQLite state with no central provider database.

## Task Decomposition

### Baseline and architecture

- Refresh `README.md` as the repository entry point.
- Maintain `docs/architecture.md` as the two-KME topology contract.
- Keep `docs/etsi014-alignment.md` as the ETSI identity, ownership, route, and
  lifecycle gate.
- Retain `baseline.md` only as historical context, with current authority in
  `README.md` and `docs/architecture.md`.

### Protocol and data contracts

- Document the minimal ETSI 014 mock profile in `docs/api-etsi014-mock.md`.
- Document the SKIP surface in `docs/api-skip.md`, pinned to
  `draft-singh-skip-00`.
- Document identifier domains, key formats, ownership metadata, lifecycle, and
  provider persistence in `docs/data-model.md`.
- Preserve the deterministic SKIP key ID format
  `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}`.

### Security and verification

- Define `docs/test-plan.md` checks for documentation consistency, scaffold
  shape, future contract tests, and future end-to-end tests.
- Define `docs/security-assumptions.md` so local-only simulated key material is
  never confused with production-ready security.
- Require checks for one-time `dec_keys`, wrong SAE/ownership rejection, exact
  ETSI JSON names, and `ETSI base64 -> bytes -> SKIP hex` conversion.

### Runtime scaffolding

- Create `infra/docker-compose.yml` with `kme-a`, `kme-b`,
  `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and
  `encryptor-b-sim`.
- Add future service scaffold documentation under `services/`.
- Keep KME simulator code in `kms/`; do not create a competing
  `services/kme-mock` implementation.
- Avoid route handlers, database schemas, FastAPI application entrypoints, or
  working service logic during Phase 0.

## Dependencies

- `kms/` must remain in the repository and be included by the root Cargo
  workspace.
- Docker Compose is the primary local runtime shape, but Phase 0 accepts a
  parseable scaffold rather than running containers.
- Future Python/FastAPI Key Provider and simulated encryptor services depend on
  the protocol and data contracts created in this phase.
- Real Cisco and real IKEv2/RFC8784 work depends on a later validated SKIP
  flow and is not part of Phase 0.

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Incomplete SKIP conformance against draft-singh-skip-00 | Pin `docs/api-skip.md` to `draft-singh-skip-00`, document endpoint names, field names, status behavior, examples, and entropy behavior before implementation. |
| Accidental centralization of Key Provider state | Require separate KeyProvider-A and KeyProvider-B SQLite paths or volumes, explicitly prohibit a central provider database, and verify Compose plus data-model docs. |
| Confusing the ETSI 014 mock with full ETSI GS QKD 014 conformance | Document a supported minimal logical key-delivery profile, list deviations from full ETSI 014, and explicitly exclude quantum channel, NetSquid, BB84, QBER, reconciliation, privacy amplification, and production claims. |

## Phase 0 Acceptance Criteria

Phase 0 is accepted when all of the following are true:

- Required docs exist: `README.md`, `docs/architecture.md`,
  `docs/etsi014-alignment.md`, `docs/api-etsi014-mock.md`,
  `docs/api-skip.md`, `docs/data-model.md`, `docs/test-plan.md`, and
  `docs/security-assumptions.md`.
- `docs/phase-0-plan.md` documents task decomposition, dependencies, risks,
  mitigations, and acceptance criteria.
- `infra/docker-compose.yml` declares the six planned local services.
- Future service scaffolds exist for Key Provider and simulated encryptor work.
- Documentation is internally consistent with `docs/etsi014-alignment.md`.
- KeyProvider-A and KeyProvider-B have independent planned SQLite state and no
  central provider database.
- Docker Compose parsing succeeds with
  `docker compose -f infra/docker-compose.yml config` when Docker Compose is
  available.
- Phase 0 does not require containers to start and does not require full service
  logic to work.

## Deferred Work

- Full ETSI mock implementation beyond the supported `kms/` subset.
- Python/FastAPI Key Provider route handlers and repositories.
- Simulated encryptor service logic.
- Automated contract and end-to-end test implementation.
- TLS/mTLS/authentication hardening beyond documented assumptions.
- Real Cisco, IPsec, or IKEv2/RFC8784 integration.
