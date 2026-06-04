# Coding Conventions

**Analysis Date:** 2026-06-04

## Naming Patterns

**Files:**
- Use Rust module filenames in `snake_case`: `kms/src/routes/etsi014.rs`, `kms/src/storage/sqlite.rs`, `kms/src/storage/memory.rs`.
- Use `mod.rs` for module roots: `kms/src/routes/mod.rs`, `kms/src/storage/mod.rs`, `kms/src/config/mod.rs`, `kms/src/tests/mod.rs`.
- Use kebab-case for documentation and ADR files: `docs/api-etsi014-mock.md`, `docs/security-assumptions.md`, `kms/adrs/0005-testing-and-fidelity-guidelines.md`.
- Future Python/FastAPI service files are not present yet. Place them under `services/keyprovider/` and `services/encryptor-sim/` while keeping the service directory names already established by `services/keyprovider/README.md` and `services/encryptor-sim/README.md`.

**Functions:**
- Use Rust `snake_case` for functions and handlers: `get_status`, `post_enc_keys`, `retrieve_keys`, `resolve_calling_sae_id` in `kms/src/routes/etsi014.rs`.
- Handler names should encode HTTP method plus ETSI route when applicable: `get_enc_keys`, `post_enc_keys`, `get_dec_keys`, `post_dec_keys` in `kms/src/routes/etsi014.rs`.
- Use helper functions for shared route logic instead of duplicating endpoint behavior: `issue_keys` and `retrieve_keys` consolidate GET/POST forms in `kms/src/routes/etsi014.rs`.
- Constructor-like functions use `new`: `MemoryKeyStorage::new` in `kms/src/storage/memory.rs`, `SqliteKeyStorage::new` in `kms/src/storage/sqlite.rs`, `AuthenticatedPeerAcceptor::new` in `kms/src/tls.rs`.

**Variables:**
- Use internal Rust `snake_case` even when public API names are ETSI-specific: `source_kme_id`, `target_kme_id`, `master_sae_id`, `slave_sae_id` in `kms/src/routes/etsi014.rs`.
- Preserve public ETSI JSON names with `#[serde(rename = "...")]`: `source_KME_ID`, `target_KME_ID`, `master_SAE_ID`, `slave_SAE_ID`, `key_ID`, and `key_IDs` in `kms/src/routes/etsi014.rs`.
- Keep constants in `SCREAMING_SNAKE_CASE`: `SAE_ID_HEADER`, `DEFAULT_KEY_SIZE_BITS`, `DEFAULT_MAX_KEYS_PER_REQUEST`, `DEFAULT_MAX_KEY_COUNT` in `kms/src/routes/etsi014.rs`.
- Avoid exposing full key material in logs or identifiers. Use metadata such as `key_count`, SAE IDs, KME IDs, or key IDs as shown in `kms/src/routes/etsi014.rs`; project constraints in `AGENTS.md` require fingerprints instead of full key material when key diagnostics are needed.

**Types:**
- Use Rust `UpperCamelCase` for structs, enums, traits, and error types: `Config`, `StorageConfig`, `StatusResponse`, `ApiError`, `Etsi014KeyStorage` in `kms/src/config/mod.rs`, `kms/src/routes/etsi014.rs`, and `kms/src/storage/mod.rs`.
- Use explicit protocol response/request structs near the route layer: `StatusResponse`, `GetKeyRequest`, `GetKeyIdsRequest`, `KeyContainerResponse`, and `ErrorResponse` in `kms/src/routes/etsi014.rs`.
- Use trait abstraction for storage boundaries: `Etsi014KeyStorage` in `kms/src/storage/mod.rs` with implementations in `kms/src/storage/memory.rs` and `kms/src/storage/sqlite.rs`.

## Code Style

**Formatting:**
- Use Rustfmt for Rust formatting. The documented verification command is `cargo fmt --all -- --check` in `README.md`, `docs/test-plan.md`, and `kms/adrs/0005-testing-and-fidelity-guidelines.md`.
- No `rustfmt.toml` is detected; use standard Rustfmt defaults.
- No Python formatter config is detected. Future Python/FastAPI services should add explicit `pyproject.toml` settings before service logic expands beyond scaffolds in `services/keyprovider/` and `services/encryptor-sim/`.

**Linting:**
- Use Clippy for Rust linting. The documented strict command is `cargo clippy --workspace --all-targets -- -D warnings` in `README.md`, `docs/test-plan.md`, and `kms/adrs/0005-testing-and-fidelity-guidelines.md`.
- No ESLint, Prettier, Biome, pytest, or Ruff config files are detected in the repo root.
- Future Python service linting is planned through Ruff by the project stack guidance in `AGENTS.md`; add config before introducing substantial Python code.

## Import Organization

**Order:**
1. Standard library imports first, grouped with nested braces when useful: `use std::{net::SocketAddr, path::PathBuf, sync::Arc};` in `kms/src/main.rs`.
2. External crate imports next: `anyhow`, `axum`, `serde`, `tracing`, `uuid`, `sqlx`, and TLS crates in `kms/src/main.rs`, `kms/src/routes/etsi014.rs`, `kms/src/storage/sqlite.rs`, and `kms/src/tls.rs`.
3. Local crate imports last: `use crate::{AppState, config::Config, storage::KeyData, tls::AuthenticatedPeer};` in `kms/src/routes/etsi014.rs`.

**Path Aliases:**
- Rust uses `crate::` and `super::` module paths only. Examples: `crate::config::{Config, KmePeerConfig, StorageConfig, StorageConfigType}` in `kms/src/tests/etsi014.rs`, and `super::{Etsi014KeyStorage, KeyData}` in `kms/src/storage/sqlite.rs`.
- No TypeScript, JavaScript, or Python import aliases are detected.

## Error Handling

**Patterns:**
- Use `anyhow::Result` at application, configuration, TLS, and storage boundaries where errors are internal or operational: `main` in `kms/src/main.rs`, `Config::load` in `kms/src/config/mod.rs`, `build_rustls_config` in `kms/src/tls.rs`, and `Etsi014KeyStorage` in `kms/src/storage/mod.rs`.
- Use route-local protocol errors for public API behavior. `ApiError` in `kms/src/routes/etsi014.rs` carries an HTTP `StatusCode` and optional message, then implements `IntoResponse`.
- Return `401` without a JSON body for unauthorized identity or topology violations using `ApiError::unauthorized` in `kms/src/routes/etsi014.rs`.
- Return `400` with `{ "message": "..." }` for malformed or unsupported protocol requests using `ApiError::bad_request` in `kms/src/routes/etsi014.rs`.
- Map backend/storage failures to `503` and log internal details with `error!(error = ?err, "backend error")` in `kms/src/routes/etsi014.rs`.
- Add context to operational failures where the file or configuration source matters: `with_context` in `kms/src/main.rs` and `kms/src/tls.rs`.

## Logging

**Framework:** `tracing`

**Patterns:**
- Initialize tracing once in `kms/src/main.rs` with `tracing_subscriber::fmt().with_env_filter(...)`.
- Use structured fields instead of formatted strings for operational metadata: `info!(master_sae_id, slave_sae_id, number, "issuing keys")` in `kms/src/routes/etsi014.rs`.
- Log route-level security decisions without full key material: `warn!(requested_key_id = %key_request.key_id, expected_slave_sae_id = ..., "slave SAE is not authorized for key")` in `kms/src/routes/etsi014.rs`.
- Log storage mode and TLS configuration metadata, not certificate/key contents: `kms/src/storage/mod.rs` and `kms/src/tls.rs`.
- Do not log full ETSI base64 keys, SKIP hex keys, private keys, certificate PEM bodies, or environment secret values.

## Comments

**When to Comment:**
- Use concise doc comments for route handlers and helpers that encode protocol behavior: `get_status`, `get_enc_keys`, `post_enc_keys`, `retrieve_keys`, and validation helpers in `kms/src/routes/etsi014.rs`.
- Use route comments when registering Axum routes to explain ETSI semantics: `build_router` in `kms/src/routes/etsi014.rs`.
- Avoid comments for obvious field assignments. Keep comments focused on protocol constraints, security decisions, and lifecycle behavior.

**JSDoc/TSDoc:**
- Not applicable. No TypeScript or JavaScript source files are detected.
- Future Python code should use docstrings for protocol-facing service boundaries when route behavior is not obvious from FastAPI models and function names.

## Function Design

**Size:** Keep route handlers thin and move shared behavior into private helpers. `get_enc_keys` and `post_enc_keys` both delegate to `issue_keys`; `get_dec_keys` and `post_dec_keys` both delegate to `retrieve_keys` in `kms/src/routes/etsi014.rs`.

**Parameters:** Pass Axum extractors explicitly in handlers: `State`, `Extension`, `HeaderMap`, `Path`, `Query`, and `Json` in `kms/src/routes/etsi014.rs`. Pass shared app state as `Arc<AppState>` when helpers need storage and config.

**Return Values:** Public handlers return `Result<Json<ResponseType>, ApiError>` in `kms/src/routes/etsi014.rs`. Internal operational functions return `anyhow::Result<T>` in `kms/src/main.rs`, `kms/src/config/mod.rs`, `kms/src/storage/mod.rs`, and `kms/src/tls.rs`.

## Module Design

**Exports:** Keep module exports narrow. `kms/src/routes/mod.rs` exposes `build_router`; `kms/src/storage/mod.rs` exposes storage trait/types and concrete submodules; `kms/src/config/mod.rs` exposes deserializable config types.

**Barrel Files:** Rust module roots are used as barrel files where needed: `kms/src/routes/mod.rs`, `kms/src/storage/mod.rs`, and `kms/src/tests/mod.rs`. Do not add broad re-export barrels unless they simplify established module boundaries.

---

*Convention analysis: 2026-06-04*
