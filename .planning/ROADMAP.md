# Roadmap: QKD ETSI 014 Mock + SKIP Baseline

## Overview

The first milestone establishes a documentation-first and scaffold-first baseline for the QKD ETSI 014 mock + SKIP integration environment. It deliberately stops before full service logic: the outcome is a precise technical plan, protocol contracts, security assumptions, test plan, Docker Compose skeleton, and service directory structure that make the later KME, Key Provider, and simulated encryptor implementation safe to plan.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Phase 0 Contracts and Scaffolding** - Create the technical plan, docs, Docker Compose skeleton, and service directories before implementation logic.

## Phase Details

### Phase 1: Phase 0 Contracts and Scaffolding
**Goal:** As a lab/research integrator, I want a precise Phase 0 baseline of contracts, risks, acceptance criteria, and service scaffolding so that later implementation can proceed without protocol ambiguity or hidden provider coupling.
**Mode:** mvp
**Depends on:** Nothing (first phase)
**Requirements:** [DOC-01, DOC-02, DOC-03, DOC-04, DOC-05, DOC-06, DOC-07, PLAN-01, PLAN-02, PLAN-03, PLAN-04, PROTO-01, PROTO-02, PROTO-03, PROTO-04, SCAF-01, SCAF-02, SCAF-03, SCAF-04]
**Success Criteria** (what must be TRUE):
  1. User can inspect the repository and find all Phase 0 deliverables: README, architecture, ETSI mock API, SKIP API, data model, test plan, security assumptions, initial Docker Compose file, and service directory skeletons.
  2. User can read the Phase 0 technical plan and see decomposed tasks, dependencies, risks, mitigations, and acceptance criteria before full service logic exists.
  3. User can verify from the docs that `draft-singh-skip-00` is the pinned SKIP contract and that the Key Provider role is client toward the ETSI 014 mock and server toward the encryptor.
  4. User can verify from the docs and Compose skeleton that KeyProvider-A and KeyProvider-B are independent instances with separate planned SQLite state and no central provider database.
  5. User can see that real Cisco encryptors and real IKEv2/RFC8784 integration are future work, while simulated encryptors are the first planned consumers.
**Plans:** 4 plans

Plans:
- [ ] 01-01: Write baseline overview, architecture, and Phase 0 technical plan.
- [ ] 01-02: Define ETSI 014 mock API, SKIP API, and data model contracts.
- [ ] 01-03: Define test plan, risks, security assumptions, and acceptance criteria.
- [ ] 01-04: Create Docker Compose skeleton and service directory scaffolding without full logic.

## Progress

**Execution Order:**
Phases execute in numeric order: 1

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Phase 0 Contracts and Scaffolding | 0/4 | Not started | - |

## Future Milestone Candidates

These are intentionally outside the current v1 milestone and should be promoted only after Phase 0 is complete.

- Minimal KME mock and provider state core.
- SKIP Key Provider API implementation with contract tests.
- Simulated encryptor end-to-end flow.
- TLS/authentication hardening and RFC8784/Cisco integration preparation.
