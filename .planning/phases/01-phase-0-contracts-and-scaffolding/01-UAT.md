---
status: testing
phase: 01-phase-0-contracts-and-scaffolding
source:
  - .planning/phases/01-phase-0-contracts-and-scaffolding/01-01-SUMMARY.md
  - .planning/phases/01-phase-0-contracts-and-scaffolding/01-02-SUMMARY.md
  - .planning/phases/01-phase-0-contracts-and-scaffolding/01-03-SUMMARY.md
  - .planning/phases/01-phase-0-contracts-and-scaffolding/01-04-SUMMARY.md
started: 2026-06-04T15:52:54-03:00
updated: 2026-06-04T15:52:54-03:00
---

## Current Test
<!-- OVERWRITE each test - shows where we are -->

number: 1
name: Cold Start Smoke Test
expected: |
  From a clean local checkout with no Phase 0 service logic required, running `docker compose -f infra/docker-compose.yml config` succeeds and shows the six-service scaffold: `kme-a`, `kme-b`, `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and `encryptor-b-sim`.
awaiting: user response

## Tests

### 1. Cold Start Smoke Test
expected: From a clean local checkout with no Phase 0 service logic required, running `docker compose -f infra/docker-compose.yml config` succeeds and shows the six-service scaffold: `kme-a`, `kme-b`, `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and `encryptor-b-sim`.
result: [pending]

### 2. User Flow: Inspect Baseline Entry Points
expected: Opening `README.md`, `docs/architecture.md`, and `docs/phase-0-plan.md` shows a precise Phase 0 baseline with two logical KME instances from `kms/`, independent KeyProvider-A/KeyProvider-B roles, simulated encryptors, risks, dependencies, and acceptance criteria before service logic exists.
result: [pending]

### 3. User Flow: Inspect Protocol Contracts
expected: Opening `docs/api-etsi014-mock.md`, `docs/api-skip.md`, and `docs/data-model.md` shows the supported ETSI `status`, `enc_keys`, and `dec_keys` subset, exact public JSON names, `draft-singh-skip-00` SKIP fields, deterministic `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}` mapping, ETSI base64 versus SKIP hexadecimal encoding, and fingerprint-only logging.
result: [pending]

### 4. User Flow: Inspect Verification and Security Posture
expected: Opening `docs/test-plan.md` and `docs/security-assumptions.md` shows Phase 0 verification criteria, manual/documentation checks, future contract and end-to-end test expectations, explicit non-production security assumptions, deferred TLS/mTLS/authentication hardening, and named risk coverage.
result: [pending]

### 5. User Flow: Inspect Runtime Service Scaffold
expected: Opening `services/README.md`, `services/keyprovider/README.md`, `services/encryptor-sim/README.md`, and `infra/docker-compose.yml` shows README-only future implementation boundaries, KME services referencing `../kms`, separate provider SQLite paths, and no central provider database.
result: [pending]

### 6. Technical Check: No Full Service Logic Boundary
expected: The repository does not contain `services/keyprovider/app/main.py` or `services/encryptor-sim/app/main.py`, and the service directories remain documentation/scaffold-only as required by the Phase 0 boundary.
result: [pending]

### 7. Technical Check: Provider and KME Independence
expected: The docs and Compose scaffold consistently show KeyProvider-A calling only KME-A, KeyProvider-B calling only KME-B, two separate KME simulator instances, separate provider SQLite state, and no central shared provider database.
result: [pending]

### 8. Coverage Check: User Story Outcome
expected: The repository now gives a lab/research integrator enough precise contracts, risks, acceptance criteria, and scaffolding for later implementation to proceed without protocol ambiguity or hidden provider coupling.
result: [pending]

## Summary

total: 8
passed: 0
issues: 0
pending: 8
skipped: 0
blocked: 0

## Gaps

[none yet]
