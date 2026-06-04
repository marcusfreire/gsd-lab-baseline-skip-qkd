---
phase: 01-phase-0-contracts-and-scaffolding
plan: "02"
subsystem: docs
tags: [etsi014, skip, data-model, contracts]
requires:
  - phase: 01-01
    provides: Baseline architecture authority and ETSI alignment gate
provides:
  - ETSI 014 supported-subset API contract
  - SKIP API contract pinned to draft-singh-skip-00
  - Data model for identifiers, encodings, ownership, lifecycle, and logging
affects: [protocol-contracts, key-provider, kms, skip]
tech-stack:
  added: []
  patterns:
    - Protocol field names remain exact at public boundaries
    - ETSI base64 and SKIP hexadecimal are separate protocol representations
key-files:
  created: []
  modified:
    - docs/api-etsi014-mock.md
    - docs/api-skip.md
    - docs/data-model.md
key-decisions:
  - "ETSI mock documentation is strict only for the supported logical key-delivery subset and lists physical-QKD exclusions."
  - "SKIP key material is hexadecimal and the default baseline key material size is 256 bits."
  - "Provider state persists in separate SQLite databases and logs use fingerprints instead of full key material."
patterns-established:
  - "SKIP key IDs use SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID} and remain identifiers, not secrets."
requirements-completed:
  - DOC-03
  - DOC-04
  - DOC-05
  - PROTO-01
  - PROTO-02
  - PROTO-03
  - PROTO-04
duration: 4 min
completed: 2026-06-04
---

# Phase 01 Plan 02: ETSI Mock API, SKIP API, and Data Model Contracts Summary

**ETSI route fidelity, SKIP draft fields, and typed key-material model for the two-KME baseline**

## Performance

- **Duration:** 4 min
- **Started:** 2026-06-04T12:32:40Z
- **Completed:** 2026-06-04T12:35:59Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Tightened `docs/api-etsi014-mock.md` so the deviations section explicitly excludes physical-QKD modeling and full ETSI conformance claims.
- Extended `docs/api-skip.md` with the `entropy` field, entropy endpoint behavior, and exact `256 bits` default key-size wording.
- Extended `docs/data-model.md` with explicit `skip_key_id`, `master_SAE_ID`, `slave_SAE_ID`, `localSystemID`, `remoteSystemID`, SQLite, and fingerprint boundaries.

## Task Commits

Each task was committed atomically:

1. **Task 1: Align the ETSI 014 mock API contract with kms/** - `485bb01` (docs)
2. **Task 2: Align the SKIP API contract with draft-singh-skip-00** - `c169b8f` (docs)
3. **Task 3: Align data model identifiers, lifecycle, and persistence** - `c7f6a5e` (docs)

**Plan metadata:** pending in close-out commit.

## Files Created/Modified

- `docs/api-etsi014-mock.md` - Supported ETSI route matrix, public JSON names, base64 keys, lifecycle, and deviations.
- `docs/api-skip.md` - SKIP endpoints, fields, entropy behavior, status codes, and deterministic key ID mapping.
- `docs/data-model.md` - Identifier domains, key encodings, ownership, provider SQLite boundaries, lifecycle, and logging rules.

## Decisions Made

- Entropy is documented as a future contract endpoint in Phase 0 and must not imply production entropy.
- Data-model docs distinguish public SKIP `keyId` from internal provider `skip_key_id`.
- Full key material remains out of logs; fingerprints are the allowed diagnostic representation.

## Deviations from Plan

None - plan executed exactly as written.

---

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope change.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `01-03`: the test plan and security assumptions can now reference exact ETSI routes, SKIP fields, deterministic IDs, and encoding/lifecycle rules.

## Self-Check: PASSED

- Verified ETSI `status`, `enc_keys`, `dec_keys`, and public JSON names.
- Verified SKIP draft pinning, endpoint names, `localSystemID`, `remoteSystemID`, `keyId`, `key`, `entropy`, hexadecimal keys, and `256 bits`.
- Verified `ETSI base64 -> bytes -> SKIP hex`, fingerprints, one-time `dec_keys`, and repeat-fail language.

---
*Phase: 01-phase-0-contracts-and-scaffolding*
*Completed: 2026-06-04*
