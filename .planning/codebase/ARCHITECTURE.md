<!-- refreshed: 2026-06-04 -->
# Architecture

**Analysis Date:** 2026-06-04

## System Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                 Local Docker Compose Baseline                │
│                 `infra/docker-compose.yml`                   │
├──────────────────┬──────────────────┬───────────────────────┤
│ `kme-a` / `kme-b`│ `keyprovider-*`  │ `encryptor-*-sim`     │
│ `kms/` Rust KME  │ Python scaffold  │ Python scaffold       │
└────────┬─────────┴────────┬─────────┴──────────┬────────────┘
         │                  │                     │
         ▼                  ▼                     ▼
┌─────────────────────────────────────────────────────────────┐
│                Protocol Contracts and Docs                   │
│ `docs/api-etsi014-mock.md`, `docs/api-skip.md`,              │
│ `docs/data-model.md`, `docs/security-assumptions.md`         │
└─────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│                   Implemented KME Simulator                  │
│ `kms/src/main.rs` -> `kms/src/routes/*` -> `kms/src/storage/*`│
└─────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| KME binary | Load YAML config, create storage, build router, choose HTTP or mTLS server mode. | `kms/src/main.rs` |
| ETSI router root | Mount ETSI 014 routes under `/api/v1/keys`. | `kms/src/routes/mod.rs` |
| ETSI 014 handlers | Own public `status`, `enc_keys`, and `dec_keys` request/response models, validation, identity resolution, ownership checks, and one-time consumption. | `kms/src/routes/etsi014.rs` |
| Config module | Deserialize `listen`, `use_mTLS`, storage selection, TLS paths, and `kme_topology`. | `kms/src/config/mod.rs` |
| Storage abstraction | Define ETSI-shaped `KeyData` and `Etsi014KeyStorage`; select memory or SQLite backend from config. | `kms/src/storage/mod.rs` |
| Memory storage | Provide ephemeral ETSI key storage for tests and local runs. | `kms/src/storage/memory.rs` |
| SQLite storage | Persist ETSI key records with `key_id`, `master_sae_id`, `slave_sae_id`, and key bytes. | `kms/src/storage/sqlite.rs` |
| mTLS layer | Build rustls config, require client certificates, extract client certificate Common Name as SAE identity. | `kms/src/tls.rs` |
| Compose scaffold | Declare six local services and per-instance environment/volumes. | `infra/docker-compose.yml` |
| Key Provider scaffold | Reserve the future shared Python/FastAPI provider implementation for KeyProvider-A and KeyProvider-B. | `services/keyprovider/README.md` |
| Encryptor simulator scaffold | Reserve the future Python/FastAPI simulated encryptor implementation. | `services/encryptor-sim/README.md` |

## Pattern Overview

**Overall:** Contract-first local baseline with an implemented Rust KME simulator and scaffolded Python service families.

**Key Characteristics:**
- Keep `kms/` as the official KME simulator source for both `KME-A` and `KME-B`; do not create `services/kme-mock/`.
- Preserve ETSI 014 public routes and JSON names at the handler boundary using Serde `rename` attributes in `kms/src/routes/etsi014.rs`.
- Treat SAE/KME topology as authorization policy, not only metadata.
- Keep Key Provider and KME persistence per instance; do not introduce a central provider database.
- Keep Phase 0 Python service directories scaffold-only until route handlers, DB schema, and app entrypoints are explicitly implemented.

## Layers

**Runtime Orchestration:**
- Purpose: Run the local shape of two KMEs, two Key Providers, and two simulated encryptors.
- Location: `infra/docker-compose.yml`
- Contains: Service declarations, environment variables, ports, volumes, and placeholder commands.
- Depends on: Docker Compose, `kms/`, `services/keyprovider/`, `services/encryptor-sim/`.
- Used by: Local operators and future integration/e2e tests.

**KME Process Bootstrap:**
- Purpose: Create shared app state and start the KME service.
- Location: `kms/src/main.rs`
- Contains: CLI config path parsing, tracing initialization, YAML config loading, storage construction, router construction, HTTP/mTLS server branching.
- Depends on: `kms/src/config/mod.rs`, `kms/src/storage/mod.rs`, `kms/src/routes/mod.rs`, `kms/src/tls.rs`.
- Used by: `cargo run -p kms -- --config ...` and future Compose commands.

**KME Protocol Routes:**
- Purpose: Expose the supported ETSI GS QKD 014 subset.
- Location: `kms/src/routes/`
- Contains: `/api/v1/keys/{slave_sae_id}/status`, `/enc_keys`, and `/dec_keys` handlers.
- Depends on: `AppState`, `Config`, `Etsi014KeyStorage`, `AuthenticatedPeer`.
- Used by: SAE clients, future Key Provider services, and router tests in `kms/src/tests/etsi014.rs`.

**KME Storage:**
- Purpose: Store issued ETSI keys with ownership metadata and enforce deletion on successful `dec_keys`.
- Location: `kms/src/storage/`
- Contains: `Etsi014KeyStorage`, `KeyData`, memory backend, SQLite backend.
- Depends on: `sqlx`, `uuid`, `async_trait`.
- Used by: `issue_keys` and `retrieve_keys` in `kms/src/routes/etsi014.rs`.

**Transport Identity:**
- Purpose: Resolve caller SAE identity before protocol authorization.
- Location: `kms/src/tls.rs` and `resolve_calling_sae_id` in `kms/src/routes/etsi014.rs`
- Contains: Required client-certificate verification in mTLS mode and `x-sae-id` local HTTP fallback when mTLS is disabled.
- Depends on: `rustls`, `axum_server`, `x509_parser`, Axum `Extension`.
- Used by: All ETSI route handlers before releasing status or key material.

**Provider and Encryptor Scaffolds:**
- Purpose: Reserve future Python/FastAPI service locations without implementing logic in Phase 0.
- Location: `services/keyprovider/`, `services/encryptor-sim/`
- Contains: README contract pointers only.
- Depends on: `docs/api-etsi014-mock.md`, `docs/api-skip.md`, `docs/data-model.md`, `docs/security-assumptions.md`.
- Used by: Future implementation phases.

## Data Flow

### Primary ETSI KME Request Path

1. KME process starts with a required config path and loads YAML config (`kms/src/main.rs:28`).
2. Storage backend is selected from config and boxed behind `Etsi014KeyStorage` (`kms/src/main.rs:40`, `kms/src/storage/mod.rs:30`).
3. Router mounts ETSI routes under `/api/v1/keys` (`kms/src/routes/mod.rs:8`).
4. Request enters `status`, `enc_keys`, or `dec_keys` route (`kms/src/routes/etsi014.rs:24`).
5. Caller SAE identity is resolved from mTLS `AuthenticatedPeer` or `x-sae-id` when mTLS is disabled (`kms/src/routes/etsi014.rs:385`).
6. Topology is checked using `kme_topology` (`kms/src/routes/etsi014.rs:426`).
7. For `enc_keys`, request options are validated, 256-bit key material is generated, ownership metadata is stored, and base64 key material is returned (`kms/src/routes/etsi014.rs:223`).
8. For `dec_keys`, stored ownership is verified, returned keys are deleted, and base64 key material is returned (`kms/src/routes/etsi014.rs:303`).

### Baseline SKIP-To-ETSI Flow

1. `Encryptor-A sim` calls future `KeyProvider-A` with `GET /key?remoteSystemID=Bob` (`docs/api-skip.md`).
2. `KeyProvider-A` acts as `SAE-A` and calls only `KME-A` at `/api/v1/keys/SAE-B/enc_keys` (`services/keyprovider/README.md`).
3. `KME-A` returns ETSI `key_ID` and base64 key material.
4. `KeyProvider-A` converts ETSI base64 bytes to SKIP hex and emits `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}` (`docs/data-model.md`).
5. Simulated handoff gives `keyId` to `Encryptor-B sim`.
6. `Encryptor-B sim` calls future `KeyProvider-B` with `GET /key/{keyId}?remoteSystemID=Alice` (`services/encryptor-sim/README.md`).
7. `KeyProvider-B` acts as `SAE-B`, maps the SKIP ID back to ETSI `key_ID`, and calls only `KME-B` at `/api/v1/keys/SAE-A/dec_keys`.
8. End-to-end success requires matching SKIP hex values on both sides.

**State Management:**
- `KME-A` and `KME-B` each own separate KME storage volumes in `infra/docker-compose.yml`.
- `KeyProvider-A` and `KeyProvider-B` each own separate planned SQLite provider volumes in `infra/docker-compose.yml`.
- In the implemented KME code, `AppState` holds immutable config plus a boxed storage trait object shared via `Arc` (`kms/src/main.rs:23`).

## Key Abstractions

**`AppState`:**
- Purpose: Share KME configuration and storage with Axum handlers.
- Examples: `kms/src/main.rs`
- Pattern: `Arc<AppState>` injected through Axum `State`.

**`Etsi014KeyStorage`:**
- Purpose: Hide memory vs SQLite persistence while preserving ETSI key lifecycle operations.
- Examples: `kms/src/storage/mod.rs`, `kms/src/storage/memory.rs`, `kms/src/storage/sqlite.rs`
- Pattern: Async trait with boxed dynamic dispatch.

**`KeyData`:**
- Purpose: Store key bytes with `master_sae_id` and `slave_sae_id` ownership.
- Examples: `kms/src/storage/mod.rs`, `kms/src/routes/etsi014.rs`
- Pattern: ETSI-specific internal record, not a protocol-neutral key object.

**ETSI Request/Response DTOs:**
- Purpose: Preserve ETSI JSON names while keeping Rust field names idiomatic.
- Examples: `StatusResponse`, `GetKeyRequest`, `KeyContainerResponse`, `GetKeyIdsRequest` in `kms/src/routes/etsi014.rs`
- Pattern: Private structs with Serde `rename` attributes.

**`AuthenticatedPeer`:**
- Purpose: Carry mTLS-derived SAE identity into route handlers.
- Examples: `kms/src/tls.rs`, `kms/src/routes/etsi014.rs`
- Pattern: Axum `Extension` attached by a custom `axum_server::accept::Accept` wrapper.

## Entry Points

**Rust KME Binary:**
- Location: `kms/src/main.rs`
- Triggers: `cargo run -p kms -- --config <path>` or future Compose command.
- Responsibilities: Load config, initialize tracing, create storage, build router, run HTTP or HTTPS with required client certificates.

**ETSI API Router:**
- Location: `kms/src/routes/mod.rs`
- Triggers: KME binary router construction.
- Responsibilities: Mount `/api/v1/keys` routes and attach `AppState`.

**ETSI 014 Endpoint Handlers:**
- Location: `kms/src/routes/etsi014.rs`
- Triggers: HTTP requests to `/api/v1/keys/{slave_SAE_ID}/status`, `/enc_keys`, and `/dec_keys`.
- Responsibilities: Protocol DTOs, validation, authorization, key issue/retrieve lifecycle.

**Docker Compose Scaffold:**
- Location: `infra/docker-compose.yml`
- Triggers: `docker compose -f infra/docker-compose.yml up`.
- Responsibilities: Declare local services, ports, volumes, and planned environment.

## Architectural Constraints

- **Threading:** The KME uses Tokio async execution through `#[tokio::main]` in `kms/src/main.rs`; memory storage uses `std::sync::Mutex<HashMap<...>>` in `kms/src/storage/memory.rs`.
- **Global state:** No mutable global state detected. Shared runtime state is `Arc<AppState>` in `kms/src/main.rs`.
- **Circular imports:** No circular module chains detected in the implemented Rust module tree.
- **KME topology:** KeyProvider-A must call only KME-A and KeyProvider-B must call only KME-B; this is documented in `services/keyprovider/README.md` and represented in `infra/docker-compose.yml`.
- **Identifier format:** Baseline docs require deterministic `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}` IDs in `docs/api-skip.md` and `docs/data-model.md`.
- **Key material logging:** Never log full key material. Current KME route logs include SAE IDs and key counts, not full keys, in `kms/src/routes/etsi014.rs`.
- **Phase boundary:** `services/keyprovider/` and `services/encryptor-sim/` contain scaffold documentation only; do not add production claims or real IKEv2 behavior there.

## Anti-Patterns

### Competing KME Implementation

**What happens:** A new KME mock is added under `services/`.
**Why it's wrong:** The official baseline KME simulator is `kms/`; splitting KME behavior would break ETSI 014 fidelity and duplicate topology/storage policy.
**Do this instead:** Extend `kms/src/routes/etsi014.rs`, `kms/src/storage/`, and supporting modules inside `kms/`.

### Central Provider Database

**What happens:** KeyProvider-A and KeyProvider-B share one provider database or state volume.
**Why it's wrong:** The architecture requires independent Key Providers and separate SQLite persistence.
**Do this instead:** Keep separate provider DB paths like `SQLITE_PATH: /data/keyprovider-a.sqlite3` and `/data/keyprovider-b.sqlite3` in `infra/docker-compose.yml`.

### Trusting `x-sae-id` In mTLS Mode

**What happens:** Route logic reads `x-sae-id` even when mTLS is enabled.
**Why it's wrong:** In mTLS mode, SAE identity must come from the client certificate Common Name.
**Do this instead:** Use `AuthenticatedPeer` from `kms/src/tls.rs`; `resolve_calling_sae_id` already switches on `config.use_mtls` in `kms/src/routes/etsi014.rs`.

### Random KME Pairing For SKIP Baseline

**What happens:** Paired KME-A/KME-B compatibility depends on independently generated UUID IDs or OS-random key bytes.
**Why it's wrong:** The baseline requires the same ETSI `key_ID` to map to the same key bytes on both KME instances for SKIP handoff validation.
**Do this instead:** Add an explicit deterministic key-source abstraction inside `kms/` and wire it through config before building end-to-end SKIP behavior. Current implementation uses `Uuid::new_v4()` and `getrandom` in `kms/src/routes/etsi014.rs`.

## Error Handling

**Strategy:** Protocol handlers return `ApiError` converted to Axum responses; infrastructure/bootstrap paths use `anyhow`.

**Patterns:**
- Use `ApiError::bad_request` for malformed or unsupported ETSI request data in `kms/src/routes/etsi014.rs`.
- Use `ApiError::unauthorized` for unknown caller SAE or ownership mismatch in `kms/src/routes/etsi014.rs`.
- Map storage failures to `503 Service Unavailable` with a generic `"server error"` message in `kms/src/routes/etsi014.rs`.
- Use `anyhow::Context` for startup/TLS/config errors in `kms/src/main.rs` and `kms/src/tls.rs`.

## Cross-Cutting Concerns

**Logging:** Rust KME uses `tracing` and `tracing_subscriber` in `kms/src/main.rs`; route logs include SAE IDs, KME IDs, counts, and authorization warnings in `kms/src/routes/etsi014.rs`.
**Validation:** ETSI request validation lives in `validate_get_key_request`, `validate_slave_sae_id`, and ownership checks in `retrieve_keys` in `kms/src/routes/etsi014.rs`.
**Authentication:** mTLS client certificate CN extraction lives in `kms/src/tls.rs`; local HTTP `x-sae-id` fallback lives in `kms/src/routes/etsi014.rs` and is valid only when `use_mTLS` is false.

---

*Architecture analysis: 2026-06-04*
