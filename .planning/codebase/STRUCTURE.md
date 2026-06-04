# Codebase Structure

**Analysis Date:** 2026-06-04

## Directory Layout

```text
gsd/
├── AGENTS.md                 # Project constraints and workflow guidance
├── README.md                 # Baseline overview, topology, flow, verification commands
├── baseline.md               # Detailed baseline planning/reference document
├── objetivo.md               # Project objective/planning reference
├── Cargo.toml                # Rust workspace manifest
├── docs/                     # Human protocol, architecture, security, data-model, and test contracts
├── infra/                    # Docker Compose local runtime scaffold
├── kms/                      # Official Rust ETSI 014 KME simulator crate
├── services/                 # Future Python/FastAPI service scaffolds
└── .planning/                # GSD planning artifacts and generated codebase maps
```

## Directory Purposes

**Root:**
- Purpose: Hold workspace-level project docs, Rust workspace membership, and GSD instructions.
- Contains: `README.md`, `AGENTS.md`, `Cargo.toml`, `baseline.md`, `objetivo.md`.
- Key files: `README.md`, `AGENTS.md`, `Cargo.toml`.

**`docs/`:**
- Purpose: Define protocol and implementation contracts before full service logic.
- Contains: Architecture, ETSI 014 API, SKIP API, data model, security assumptions, test plan, ETSI alignment, phase plan.
- Key files: `docs/architecture.md`, `docs/api-etsi014-mock.md`, `docs/api-skip.md`, `docs/data-model.md`, `docs/security-assumptions.md`, `docs/test-plan.md`, `docs/etsi014-alignment.md`, `docs/phase-0-plan.md`.

**`infra/`:**
- Purpose: Declare local multi-service runtime shape.
- Contains: Docker Compose service and volume definitions.
- Key files: `infra/docker-compose.yml`.

**`kms/`:**
- Purpose: Implement the official Rust ETSI GS QKD 014 KME simulator used as both KME-A and KME-B.
- Contains: Rust crate manifest, README, ADRs, source modules, tests.
- Key files: `kms/Cargo.toml`, `kms/README.md`, `kms/src/main.rs`, `kms/src/routes/etsi014.rs`, `kms/src/storage/mod.rs`, `kms/src/tls.rs`.

**`kms/adrs/`:**
- Purpose: Record KME-specific architectural decisions.
- Contains: ADRs for simulator scope, ETSI protocol fidelity, SAE topology policy, key source/storage lifecycle, and testing guidelines.
- Key files: `kms/adrs/0001-experimental-key-management-simulator.md`, `kms/adrs/0002-etsi-014-protocol-fidelity.md`, `kms/adrs/0003-sae-identity-and-topology-policy.md`, `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`, `kms/adrs/0005-testing-and-fidelity-guidelines.md`.

**`kms/src/`:**
- Purpose: Hold the implemented KME binary modules.
- Contains: App bootstrap, config loading, Axum routes, storage backends, tests, TLS identity handling.
- Key files: `kms/src/main.rs`, `kms/src/config/mod.rs`, `kms/src/routes/mod.rs`, `kms/src/routes/etsi014.rs`, `kms/src/storage/mod.rs`, `kms/src/tls.rs`.

**`kms/src/config/`:**
- Purpose: Load KME YAML configuration.
- Contains: Config DTOs for listen address, mTLS flag, storage parameters, TLS paths, KME topology.
- Key files: `kms/src/config/mod.rs`.

**`kms/src/routes/`:**
- Purpose: Own HTTP route mounting and ETSI 014 protocol handlers.
- Contains: Router root and ETSI 014 route module.
- Key files: `kms/src/routes/mod.rs`, `kms/src/routes/etsi014.rs`.

**`kms/src/storage/`:**
- Purpose: Store ETSI key records behind a backend abstraction.
- Contains: Storage trait, key data type, memory backend, SQLite backend.
- Key files: `kms/src/storage/mod.rs`, `kms/src/storage/memory.rs`, `kms/src/storage/sqlite.rs`.

**`kms/src/tests/`:**
- Purpose: Exercise KME protocol behavior through the real Axum router.
- Contains: Router tests for status, issue/retrieve lifecycle, GET shortcuts, validation failures, ownership failures, and one-time consumption.
- Key files: `kms/src/tests/mod.rs`, `kms/src/tests/etsi014.rs`.

**`services/`:**
- Purpose: Reserve future Python/FastAPI service families.
- Contains: Scaffold documentation for Key Provider and simulated encryptor services.
- Key files: `services/README.md`, `services/keyprovider/README.md`, `services/encryptor-sim/README.md`.

**`services/keyprovider/`:**
- Purpose: Future shared Key Provider implementation location.
- Contains: README only.
- Key files: `services/keyprovider/README.md`.

**`services/encryptor-sim/`:**
- Purpose: Future simulated encryptor implementation location.
- Contains: README only.
- Key files: `services/encryptor-sim/README.md`.

**`.planning/`:**
- Purpose: GSD planning state, phase artifacts, quick-task artifacts, research docs, and codebase maps.
- Contains: `codebase/`, `phases/`, `quick/`, `research/`.
- Key files: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`.

## Key File Locations

**Entry Points:**
- `kms/src/main.rs`: KME binary entry point and server startup.
- `kms/src/routes/mod.rs`: Axum router root for `/api/v1/keys`.
- `infra/docker-compose.yml`: Local six-service runtime scaffold.

**Configuration:**
- `Cargo.toml`: Rust workspace manifest.
- `kms/Cargo.toml`: Rust crate dependencies and binary package metadata.
- `kms/src/config/mod.rs`: YAML config schema.
- `infra/docker-compose.yml`: Runtime environment variables for KME, provider, and encryptor instances.

**Core Logic:**
- `kms/src/routes/etsi014.rs`: ETSI 014 request/response models, route handlers, validation, topology checks, key lifecycle.
- `kms/src/storage/mod.rs`: Storage trait and backend factory.
- `kms/src/storage/memory.rs`: In-memory key storage.
- `kms/src/storage/sqlite.rs`: SQLite key storage schema and CRUD operations.
- `kms/src/tls.rs`: mTLS config and client certificate CN extraction.

**Testing:**
- `kms/src/tests/etsi014.rs`: KME router behavior tests.
- `kms/src/tests/mod.rs`: KME test module root.
- `docs/test-plan.md`: Baseline test plan.

**Contracts:**
- `docs/architecture.md`: Human architecture contract.
- `docs/api-etsi014-mock.md`: ETSI 014 route and JSON contract.
- `docs/api-skip.md`: SKIP route and field contract.
- `docs/data-model.md`: Identifier, key material, storage, and mapping rules.
- `docs/security-assumptions.md`: Security scope and non-production boundaries.

## Naming Conventions

**Files:**
- Rust module files use snake_case: `kms/src/routes/etsi014.rs`, `kms/src/storage/sqlite.rs`.
- Rust module directories use `mod.rs` for module roots: `kms/src/config/mod.rs`, `kms/src/routes/mod.rs`, `kms/src/storage/mod.rs`, `kms/src/tests/mod.rs`.
- Markdown planning and contract docs use lowercase kebab-case in `docs/`: `docs/api-etsi014-mock.md`, `docs/security-assumptions.md`.
- GSD generated codebase maps use uppercase filenames: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`.
- ADR files use numeric prefixes and kebab-case titles: `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`.

**Directories:**
- Rust implementation lives under `kms/src/<module>/`.
- Future service families live under `services/<service-family>/`.
- Runtime infrastructure lives under `infra/`.
- Human protocol contracts live under `docs/`.

## Where to Add New Code

**New ETSI 014 KME Behavior:**
- Primary code: `kms/src/routes/etsi014.rs`
- Supporting modules: add focused modules under `kms/src/` when behavior becomes too large for a route helper, such as `kms/src/key_source/` or `kms/src/topology/`.
- Storage changes: `kms/src/storage/mod.rs`, `kms/src/storage/sqlite.rs`, `kms/src/storage/memory.rs`.
- Tests: `kms/src/tests/etsi014.rs` for router-visible behavior; add focused test modules under `kms/src/tests/` for new module-level behavior.

**New KME Storage Backend:**
- Implementation: `kms/src/storage/<backend>.rs`
- Registration: `kms/src/storage/mod.rs`
- Config schema: `kms/src/config/mod.rs`
- Tests: `kms/src/tests/` and router tests when behavior affects public lifecycle.

**New KME Transport/Auth Behavior:**
- Implementation: `kms/src/tls.rs` for mTLS extraction and rustls setup.
- Route identity policy: `resolve_calling_sae_id` in `kms/src/routes/etsi014.rs`.
- Config schema: `kms/src/config/mod.rs`.
- Tests: `kms/src/tests/` with integration tests for socket/certificate behavior when needed.

**New Key Source:**
- Implementation: add a new module under `kms/src/`, for example `kms/src/key_source/`.
- Integration point: replace direct `Uuid::new_v4()` and `getrandom` use in `kms/src/routes/etsi014.rs` with a configured key-source abstraction.
- Config schema: `kms/src/config/mod.rs`.
- Tests: `kms/src/tests/` and end-to-end tests proving KME-A/KME-B deterministic compatibility.

**New Key Provider Service Logic:**
- Primary code: `services/keyprovider/`.
- Use one configurable implementation for both KeyProvider-A and KeyProvider-B.
- Keep ETSI client code, SKIP routers, provider SQLite repositories, and service config separated inside `services/keyprovider/` when implementation starts.
- Tests: add provider tests under `services/keyprovider/` once Python package structure exists.
- Contract references: `docs/api-etsi014-mock.md`, `docs/api-skip.md`, `docs/data-model.md`.

**New Simulated Encryptor Logic:**
- Primary code: `services/encryptor-sim/`.
- Keep simulated SKIP client behavior separate from real IKEv2/Cisco integrations.
- Tests: add tests under `services/encryptor-sim/` once Python package structure exists.
- Contract references: `docs/api-skip.md`, `docs/data-model.md`.

**New Runtime Wiring:**
- Compose updates: `infra/docker-compose.yml`.
- Avoid embedding secrets in Compose. Use environment variable names or mounted secret paths only.
- Keep KME-A, KME-B, KeyProvider-A, and KeyProvider-B volumes separate.

**New Documentation:**
- Protocol/API contracts: `docs/`.
- KME architectural decisions: `kms/adrs/`.
- GSD generated maps: `.planning/codebase/`.

**Utilities:**
- Rust KME shared helpers: add focused modules under `kms/src/`, then expose through `mod` declarations in `kms/src/main.rs`.
- Python provider helpers: add under `services/keyprovider/` only after Python package structure is created.
- Python encryptor helpers: add under `services/encryptor-sim/` only after Python package structure is created.

## Special Directories

**`.codex/`:**
- Purpose: Local Codex/GSD skills, agents, hooks, templates, and workflow definitions.
- Generated: Yes
- Committed: Project-specific tooling directory is present in the repo.

**`.agents/`:**
- Purpose: Agent-related local directory.
- Generated: Yes
- Committed: Present but contains no project skills for this mapping.

**`.planning/`:**
- Purpose: GSD planning artifacts and generated codebase reference docs.
- Generated: Yes
- Committed: Planning artifacts are part of the GSD workflow.

**`kms/adrs/`:**
- Purpose: KME architecture decision records.
- Generated: No
- Committed: Yes

**`services/keyprovider/` and `services/encryptor-sim/`:**
- Purpose: Future Python/FastAPI service locations.
- Generated: No
- Committed: Yes

---

*Structure analysis: 2026-06-04*
