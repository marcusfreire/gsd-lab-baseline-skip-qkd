# Phase 1 Research: Phase 0 Contracts and Scaffolding

**Phase:** 1 - Phase 0 Contracts and Scaffolding
**Researched:** 2026-06-02
**Status:** Complete
**Question:** What do I need to know to plan this phase well?

## Phase Scope

Phase 1 is a documentation and scaffolding phase. It must create the Phase 0 baseline artifacts and repository shape without implementing full KME, Key Provider, SKIP, encryptor, Cisco, or IKEv2/RFC8784 logic.

The phase must cover all v1 requirements:

- Documentation: `DOC-01` through `DOC-07`
- Technical plan and risks: `PLAN-01` through `PLAN-04`
- Protocol contracts: `PROTO-01` through `PROTO-04`
- Scaffolding: `SCAF-01` through `SCAF-04`

## Key Implementation Constraints From CONTEXT.md

- `docs/*.md` become the normative project contract after Phase 0.
- `baseline.md` must be moved to `docs/archive/baseline.md` and treated as historical context.
- `docs/api-skip.md` must be strict against `draft-singh-skip-00`.
- `docs/api-etsi014-mock.md` must be strict only within the declared minimal ETSI 014 mock profile and must list deviations from full ETSI GS QKD 014 conformance.
- KeyProvider-A and KeyProvider-B are two Compose instances of one configurable implementation under `services/keyprovider/`.
- Service bases are `services/kme-mock/`, `services/keyprovider/`, and `services/encryptor-sim/`.
- SKIP `keyId` derivation is `first_16_bytes(HMAC-SHA256(SKIP_KEY_ID_NAMESPACE_SECRET, qkd_key_id | localSystemID | remoteSystemID))`, encoded as lowercase hex.
- `SKIP_KEY_ID_NAMESPACE_SECRET` is a shared environment variable between KeyProvider-A and KeyProvider-B, not QKD key material.
- A future implementation must treat SKIP `keyId` as unique; collision means fail/report/no overwrite.
- Phase 0 acceptance requires required files, internal documentation consistency, scaffold directories, and parseable Compose config. It does not require containers to start.

## Protocol Findings

### SKIP

`draft-singh-skip-00` is an active Internet-Draft published on 2026-04-14 and expiring on 2026-10-16. It replaces the archived Cisco draft and is the pinned project contract.

SKIP is an encryptor-to-Key-Provider interface. The typical flow is:

1. Initiating encryptor requests a fresh `(keyId, key)` from its co-located KP.
2. The initiating encryptor passes `keyId` to the remote encryptor.
3. The remote encryptor requests the matching key from its co-located KP using `keyId`.

`docs/api-skip.md` needs a normative matrix for these methods:

| Method | Path | Phase 0 doc implication |
|--------|------|-------------------------|
| GET | `/capabilities` | Document response fields: `entropy`, `key`, `algorithm`, `localSystemID`, `remoteSystemID`. |
| GET | `/key?remoteSystemID={id}` | Document fresh key request for a remote KP. |
| GET | `/key?remoteSystemID={id}&size={bits}` | Document optional key-size request and supported/default sizes. |
| GET | `/key/{keyId}?remoteSystemID={id}` | Document retrieval by SKIP key ID. |
| GET | `/entropy` | Document default entropy response. |
| GET | `/entropy?minentropy={bits}` | Document requested entropy length. |

SKIP general status codes include:

- `200` for success
- `404` for unknown paths
- `405` for unsupported HTTP methods

`docs/api-skip.md` should also document method-specific bad request and failure behavior where the draft defines it. At minimum, the document must make unknown path and non-GET handling explicit because those are easy to miss in a future implementation.

Important SKIP field rules:

- `keyId` and `key` are JSON strings.
- `keyId` is hexadecimal-encoded.
- `key` is hexadecimal-encoded bytes.
- `keyId` must be 128 bits.
- The key bytes must default to 256 bits.
- The key must not be recoverable from `keyId` alone.

Security note: SKIP describes HTTP over TLS between encryptor and KP. Phase 0 can document local HTTP as a scaffold simplification only if `docs/security-assumptions.md` clearly labels TLS/authentication as future hardening and not production-ready behavior.

### ETSI GS QKD 014 Mock Profile

ETSI GS QKD 014 defines a REST-based SAE-to-KME key delivery API. The project does not need full ETSI conformance in Phase 0. It does need a strict minimal mock profile that mirrors the logical behaviors needed by Key Providers.

Relevant ETSI concepts for the mock profile:

- SAE requests keys from KME.
- KME delivers keys and status information.
- Master SAE calls `Get key` to obtain keys and associated key IDs.
- Slave SAE later calls `Get key with key IDs` to retrieve identical key material by key IDs.
- The application-level communication of key IDs between SAEs is outside ETSI 014.

Minimal Phase 0 ETSI mock endpoints should be documented as:

| ETSI concept | ETSI path shape | Phase 0 mock role |
|--------------|-----------------|-------------------|
| Get status | `/api/v1/keys/{slave_SAE_ID}/status` | Report logical key availability for a target SAE. |
| Get key | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Let initiator-side Key Provider collect new QKD logical key material and QKD key IDs. |
| Get key with key IDs | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Let responder-side Key Provider collect matching logical key material by QKD key ID. |

The doc must list deviations from full ETSI 014, likely including:

- Local mock may run over HTTP in Phase 0/early dev instead of HTTPS.
- Mutual TLS/certificate identity checks are not implemented in Phase 0.
- Physical QKD, trusted node routing, QBER, reconciliation, privacy amplification, multicast, and real KME security boundaries are out of scope.
- The mock represents logical synchronized key availability only.
- The mock should not imply production ETSI conformance.

### RFC8784 Future Context

RFC8784 is a future consumer context. It should not drive implementation work in Phase 0, but docs should preserve the relationship:

- SKIP `keyId` is the future PPK identity candidate.
- The PPK itself is the key material returned through SKIP.
- Real IKE negotiation, `PPK_IDENTITY`, and `SK_d`/`SK_pi`/`SK_pr` mixing are not implemented in Phase 0.

## Documentation Planning Guidance

The phase is best planned as four coherent documentation/scaffold slices matching the roadmap:

1. **Baseline overview and architecture**
   - `README.md`
   - `docs/architecture.md`
   - `docs/phase-0-plan.md` or equivalent plan section
   - Move `baseline.md` to `docs/archive/baseline.md`

2. **Protocol and data contracts**
   - `docs/api-etsi014-mock.md`
   - `docs/api-skip.md`
   - `docs/data-model.md`

3. **Verification and security assumptions**
   - `docs/test-plan.md`
   - `docs/security-assumptions.md`
   - Explicit risk/mitigation and acceptance criteria

4. **Infrastructure and service scaffold**
   - `infra/docker-compose.yml`
   - `services/kme-mock/`
   - `services/keyprovider/`
   - `services/encryptor-sim/`

## Scaffolding Guidance

The scaffold should make later implementation obvious without building the implementation now.

Recommended service directory minimum:

```text
services/
├── kme-mock/
│   └── README.md
├── keyprovider/
│   └── README.md
└── encryptor-sim/
    └── README.md
```

Optional placeholder files may include `.gitkeep`, `pyproject.toml`, or `app/README.md`, but Phase 0 should not create working FastAPI service logic unless explicitly planned as non-functional placeholder text.

Recommended Compose shape:

- `kme-mock`
- `keyprovider-a`
- `keyprovider-b`
- `encryptor-a-sim`
- `encryptor-b-sim`

`keyprovider-a` and `keyprovider-b` should point at the same `services/keyprovider/` build context, with different environment:

- `LOCAL_SYSTEM_ID`
- `REMOTE_SYSTEM_ID`
- `KME_BASE_URL`
- `SQLITE_PATH`
- `SKIP_KEY_ID_NAMESPACE_SECRET`

Use distinct provider SQLite volumes or paths. Do not create one shared provider data volume.

For Phase 0 parseability, Compose can use build contexts and placeholder commands that do not require successful service startup. The acceptance test is `docker compose -f infra/docker-compose.yml config` when Docker Compose is available.

## Data Model Planning Guidance

`docs/data-model.md` should explicitly separate identifier domains:

| Field | Domain | Notes |
|-------|--------|-------|
| `qkd_key_id` | KME/ETSI mock | Logical QKD key identifier from the KME mock. |
| ETSI `key_ID` | ETSI-shaped response | If represented separately, document how it maps to `qkd_key_id`. |
| SKIP `keyId` / `skip_key_id` | SKIP | 128-bit lowercase hex string derived from `qkd_key_id`, `localSystemID`, and `remoteSystemID`. |
| `key` | SKIP | Hex-encoded key bytes, default 256 bits. |
| `key_material` | KME/internal | Simulated logical key material; do not log as plaintext in future implementation. |

HMAC input encoding must be unambiguous. The docs should not simply write `qkd_key_id | localSystemID | remoteSystemID` without defining the delimiter/encoding. Recommend documenting either:

- length-prefixed UTF-8 fields, or
- a JSON canonical array string with exact field order.

For planning purposes, length-prefixed UTF-8 is safer to specify in docs because it avoids delimiter ambiguity while staying implementation-neutral.

## Security and Risk Guidance

`docs/security-assumptions.md` should make these limits explicit:

- Phase 0 is local and non-production.
- Key material is simulated.
- TLS/mTLS/auth are not implemented unless a future phase adds them.
- `SKIP_KEY_ID_NAMESPACE_SECRET` is not QKD material and must not be logged.
- Future implementation should redact raw key material from logs.
- Future implementation should treat `keyId` collision as a hard error.
- Full ETSI 014 and real SKIP security posture require later implementation/testing, not Phase 0 docs alone.

Risks that must be represented in the technical plan:

- SKIP drift: docs accidentally describe an API that is not `draft-singh-skip-00`.
- ETSI over-claim: docs imply full ETSI GS QKD 014 conformance.
- Provider coupling: scaffold accidentally introduces shared state between KeyProvider-A and KeyProvider-B.
- Identifier collapse: docs conflate `qkd_key_id`, ETSI `key_ID`, SKIP `keyId`, and RFC8784 PPK identity.
- Phase boundary creep: Phase 0 starts implementing service logic instead of defining contracts/scaffold.

## Verification Strategy

Phase 0 should be verified without requiring service logic.

Minimum verification checklist:

- Required files exist:
  - `README.md`
  - `docs/archive/baseline.md`
  - `docs/architecture.md`
  - `docs/api-etsi014-mock.md`
  - `docs/api-skip.md`
  - `docs/data-model.md`
  - `docs/test-plan.md`
  - `docs/security-assumptions.md`
  - `infra/docker-compose.yml`
- Required service directories exist:
  - `services/kme-mock/`
  - `services/keyprovider/`
  - `services/encryptor-sim/`
- `docs/api-skip.md` names `draft-singh-skip-00` and includes the SKIP methods matrix.
- `docs/api-etsi014-mock.md` declares a minimal mock profile and deviations from full ETSI GS QKD 014.
- `docs/data-model.md` contains the HMAC-based `keyId` rule, `SKIP_KEY_ID_NAMESPACE_SECRET`, and collision behavior.
- `infra/docker-compose.yml` defines separate A/B provider instances with separate SQLite paths/volumes.
- If Docker Compose is installed, `docker compose -f infra/docker-compose.yml config` exits successfully.

## Suggested Plan Split

The roadmap already lists four plans. Keep that split:

- `01-01`: README, architecture, technical plan, archive `baseline.md`.
- `01-02`: ETSI mock API, SKIP API, data model.
- `01-03`: Test plan, security assumptions, risk/acceptance criteria.
- `01-04`: Compose skeleton and service directories.

Wave recommendation:

- Wave 1: `01-01` and `01-02` can run in parallel after reading CONTEXT/RESEARCH, but they both touch docs and should avoid overlapping files.
- Wave 2: `01-03` depends on the contracts from `01-02` and technical plan from `01-01`.
- Wave 3: `01-04` can depend on `01-01`/`01-02` so Compose env names match the documented architecture and data model.

## Sources

- `draft-singh-skip-00`: https://datatracker.ietf.org/doc/html/draft-singh-skip-00
- `ETSI GS QKD 014 V1.1.1`: https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf
- `RFC8784`: https://datatracker.ietf.org/doc/html/rfc8784
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/research/SUMMARY.md`
- `.planning/phases/01-phase-0-contracts-and-scaffolding/01-CONTEXT.md`

## RESEARCH COMPLETE
