# Phase 1: Phase 0 Contracts and Scaffolding - Context

**Gathered:** 2026-06-04
**Status:** Ready for replanning

<domain>
## Phase Boundary

This phase delivers the Phase 0 baseline for the QKD ETSI 014 + SKIP project.
The deliverable is documentation, risk and acceptance criteria, Docker Compose
shape, and service directory scaffolding. It must not implement complete KME,
Key Provider, SKIP, encryptor, Cisco, or IKEv2/RFC8784 logic.

The architecture has been re-evaluated against:

- `reavaliar.md`
- `kms/README.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`
- `AES_VPN_Prototypes.md`

The `kms/` Rust project is the KME simulator for this baseline. It is not a
production KMS. The baseline models logical ETSI 014 key delivery only; it does
not model a quantum channel, BB84, NetSquid, QBER, reconciliation, privacy
amplification, or optical-layer QKD behavior.

</domain>

<decisions>
## Implementation Decisions

### Document Authority
- **D-01:** After Phase 0, `README.md`, `docs/*.md`, `infra/docker-compose.yml`,
  and `AGENTS.md` are the normative project contracts for architecture, APIs,
  data model, tests, security assumptions, runtime shape, and agent constraints.
- **D-02:** `baseline.md`, if present, is historical context only after the
  Phase 0 documentation baseline exists.
- **D-03:** The committed rebaseline docs supersede the older Phase 1 context
  choices that described a one-KME simulator service-source framing and
  namespace-secret SKIP key ID derivation.

### KME Simulator Topology
- **D-04:** The baseline uses two separate KME simulator instances: `KME-A` and
  `KME-B`.
- **D-05:** `KME-A` owns the local KME endpoint and storage for `SAE-A`.
- **D-06:** `KME-B` owns the local KME endpoint and storage for `SAE-B`.
- **D-07:** The KME simulator implementation is sourced from `kms/`; the Phase 0
  Compose file may reference it as a scaffold without implementing service
  logic in this repository.
- **D-08:** KME-A and KME-B must not be collapsed into a single central KME.
- **D-09:** KME-A and KME-B must not rely on shared KME storage for normal
  operation. They may share a deterministic seed or fake key-source policy so
  the same ETSI `key_ID` maps to identical key bytes in both KME instances.

### SAE and Key Provider Roles
- **D-10:** `KeyProvider-A` acts as `SAE-A` when calling `KME-A`.
- **D-11:** `KeyProvider-B` acts as `SAE-B` when calling `KME-B`.
- **D-12:** `KeyProvider-A` consults only `KME-A`; `KeyProvider-B` consults only
  `KME-B`.
- **D-13:** `KeyProvider-A` and `KeyProvider-B` are independent services with
  separate provider state and separate SQLite databases.
- **D-14:** There is no central shared database between Key Providers.

### ETSI 014 Contract
- **D-15:** The public ETSI API must preserve these supported routes and
  methods:
  - `GET /api/v1/keys/{slave_SAE_ID}/status`
  - `POST /api/v1/keys/{slave_SAE_ID}/enc_keys`
  - `GET /api/v1/keys/{slave_SAE_ID}/enc_keys`
  - `POST /api/v1/keys/{master_SAE_ID}/dec_keys`
  - `GET /api/v1/keys/{master_SAE_ID}/dec_keys`
- **D-16:** Public ETSI JSON names must preserve ETSI names, including
  `source_KME_ID`, `target_KME_ID`, `master_SAE_ID`, `slave_SAE_ID`, `key_ID`,
  and `key_IDs`.
- **D-17:** ETSI key material is exposed as base64 in the ETSI API. Key
  Providers must convert ETSI base64 to bytes and then to SKIP hex.
- **D-18:** `docs/api-etsi014-mock.md` is strict only within the declared
  minimal ETSI 014 profile. It must explicitly list deviations from full ETSI
  GS QKD 014 conformance.

### SAE Identity and Key Lifecycle
- **D-19:** SAE/KME topology is an authorization policy, not a naming
  convenience.
- **D-20:** In mTLS mode, caller SAE identity comes from the client certificate
  Common Name.
- **D-21:** In local HTTP mode, caller SAE identity may come from `x-sae-id`;
  `x-sae-id` is valid only when mTLS is disabled.
- **D-22:** Unknown caller SAE, unknown peer SAE, wrong master, wrong slave, or
  wrong ownership must be rejected before key material is released.
- **D-23:** `enc_keys` stores or emits key records with `master_sae_id`,
  `slave_sae_id`, `key_ID`, key bytes, and availability state.
- **D-24:** `dec_keys` must verify that caller SAE equals stored `slave_sae_id`,
  path SAE equals stored `master_sae_id`, and requested `key_ID` exists and is
  still available.
- **D-25:** Successful `dec_keys` consumes the key by deleting it or marking it
  unavailable. Repeating `dec_keys` with the same `key_ID` must fail.

### SKIP Contract
- **D-26:** `docs/api-skip.md` is strict against `draft-singh-skip-00` for the
  supported Phase 0/Phase 1 profile.
- **D-27:** The Key Provider exposes SKIP fields using draft names including
  `localSystemID`, `remoteSystemID`, `keyId`, and `key`.
- **D-28:** SKIP key material is hexadecimal.
- **D-29:** The SKIP key ID is deterministic and textual:
  `skip_key_id = "SKIP-" + master_SAE_ID + "-" + slave_SAE_ID + "-" + key_ID`.
- **D-30:** Example mapping:
  `QKD-000001`, `SAE-A`, `SAE-B` -> `SKIP-SAE-A-SAE-B-QKD-000001`.
- **D-31:** The SKIP key ID must not reveal key material. It may reveal the
  logical SAE pair and ETSI key identifier in this local baseline.
- **D-32:** The responder side maps `SKIP-SAE-A-SAE-B-QKD-000001` back to ETSI
  `key_ID` `QKD-000001` before calling `dec_keys`.

### End-to-End Invariant
- **D-33:** `KeyProvider-A` gets outbound key material from `KME-A` with
  `enc_keys` for `SAE-B`.
- **D-34:** `Encryptor-A` obtains `{ "keyId": "...", "key": "<hex>" }` from
  `KeyProvider-A` through `GET /key?remoteSystemID=Bob`.
- **D-35:** `Encryptor-A` hands `keyId` to `Encryptor-B` through simulated
  handoff. This represents future RFC8784 `PPK_IDENTITY`.
- **D-36:** `Encryptor-B` requests
  `GET /key/{keyId}?remoteSystemID=Alice` from `KeyProvider-B`.
- **D-37:** `KeyProvider-B` resolves the SKIP `keyId`, calls `KME-B` `dec_keys`
  as `SAE-B` for `SAE-A`, and returns the matching SKIP hex key.
- **D-38:** The required e2e invariant is `key_hex_alice == key_hex_bob`.

### Runtime and Scaffolding
- **D-39:** `infra/docker-compose.yml` must define separate services for
  `kme-a`, `kme-b`, `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and
  `encryptor-b-sim`.
- **D-40:** KME and Key Provider storage must use separate volumes or database
  paths for A and B.
- **D-41:** Phase 0 can use placeholder commands and service directories, but
  the runtime shape must expose the future component boundaries clearly.

### Phase 0 Acceptance
- **D-42:** Phase 0 is accepted when required docs exist, documentation is
  internally consistent with this rebaseline, scaffold directories exist, and
  the Docker Compose file is parseable when Docker Compose is available.
- **D-43:** `docs/test-plan.md` must include manual checks for e2e key equality,
  one-time `dec_keys`, wrong SAE/ownership rejection, exact ETSI JSON names, and
  ETSI base64 to SKIP hex conversion.
- **D-44:** Phase 0 must not require containers to start successfully or provide
  working service logic.

### Agent Discretion
- The agent may choose exact placeholder filenames, doc section ordering, and
  scaffold file contents if the decisions above remain true.
- The agent may keep the Python/FastAPI Key Provider scaffold separate from the
  Rust `kms/` simulator scaffold while documenting the boundary clearly.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Contracts
- `README.md` - Corrected topology, component roles, end-to-end flow, and
  current scope boundary.
- `docs/architecture.md` - Architecture authority for the two-KME topology and
  SAE/KME policy.
- `docs/etsi014-alignment.md` - Consensus gate for ETSI 014 alignment,
  component roles, ownership, lifecycle, and simulated synchronization.
- `docs/api-etsi014-mock.md` - ETSI 014 mock API contract.
- `docs/api-skip.md` - SKIP API contract against `draft-singh-skip-00`.
- `docs/data-model.md` - Identifier, key material, lifecycle, and mapping rules.
- `docs/test-plan.md` - Manual and future automated verification strategy.
- `docs/security-assumptions.md` - Security boundaries and non-production
  assumptions.
- `infra/docker-compose.yml` - Local runtime scaffold.
- `AGENTS.md` - Project constraints downstream agents must follow.

### Planning and Research
- `.planning/PROJECT.md` - Project constraints and scope.
- `.planning/REQUIREMENTS.md` - Phase 0 requirement IDs and traceability.
- `.planning/ROADMAP.md` - Phase 1 goal, MVP mode, success criteria, and plan
  outline.
- `.planning/research/SUMMARY.md` - Research conclusions that shaped the
  original Phase 0 plan.
- `.planning/quick/260603-uty-reavaliar-arquitetura-qkd-skip-baseline-/260603-uty-SUMMARY.md`
  - Rebaseline execution summary.

### KMS Simulator References
- `kms/README.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`
- `kms/src/routes/etsi014.rs`
- `kms/src/config/mod.rs`
- `kms/src/storage/mod.rs`
- `kms/src/tests/etsi014.rs`

### External Protocol References
- `draft-singh-skip-00` - Normative SKIP draft for supported SKIP behavior.
- `ETSI GS QKD 014` - Source for the minimal logical KME delivery profile and
  for documenting deviations from full conformance.
- `RFC8784` - Future IKEv2 PPK context only; do not implement real
  IKEv2/RFC8784 in Phase 0.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `kms/` contains the experimental Rust ETSI 014 KME simulator that should be
  referenced by the baseline instead of inventing a separate logical KME shape.
- `kms/src/routes/etsi014.rs` contains the practical route semantics for
  `status`, `enc_keys`, and `dec_keys`.
- `kms/src/config/mod.rs` documents the simulator configuration surface,
  including mTLS mode and topology policy.
- `kms/src/storage/mod.rs` documents the storage abstraction and lifecycle
  expectations.
- `kms/src/tests/etsi014.rs` captures route fidelity, ownership, and one-time
  consumption expectations.

### Established Patterns
- The repository is documentation/scaffold first.
- Service implementation is deferred until after the Phase 0 contracts are
  internally consistent.
- Markdown contracts must be plain, concrete, and testable.

### Integration Points
- KME simulator shape is represented by Compose services `kme-a` and `kme-b`.
- Key Provider scaffold belongs under the provider service boundary and is
  instantiated as `keyprovider-a` and `keyprovider-b`.
- Simulated encryptor scaffold belongs under the encryptor service boundary and
  is instantiated as `encryptor-a-sim` and `encryptor-b-sim`.
- Runtime topology lives in `infra/docker-compose.yml`.

</code_context>

<specifics>
## Specific Ideas

- The architecture must show two KME instances and two independent Key
  Providers, not a central KME and not a central provider database.
- The ETSI contract must use `key_ID` and `key_IDs`, not local renames.
- The SKIP contract must use `keyId`, `localSystemID`, `remoteSystemID`, and
  `key` as public field names.
- The key conversion path is exactly `ETSI base64 -> bytes -> SKIP hex`.
- Logs must use fingerprints or metadata, never complete key material.
- The rebaseline intentionally removed the older namespace-secret `keyId`
  design for this project phase.

</specifics>

<deferred>
## Deferred Ideas

- Real Cisco or hardware encryptor integration remains future work.
- Real IKEv2/RFC8784 integration remains future work.
- Full ETSI GS QKD 014 conformance remains future work unless explicitly
  promoted into a later phase.
- Physical QKD simulation, NetSquid, BB84, and quantum channel modeling remain
  out of scope.
- Production KMS hardening remains out of scope.

</deferred>

---

*Phase: 1-Phase 0 Contracts and Scaffolding*
*Context gathered: 2026-06-04*
