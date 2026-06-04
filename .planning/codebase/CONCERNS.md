# Codebase Concerns

**Analysis Date:** 2026-06-04

## Tech Debt

**Phase 0 runtime is scaffold-only:**
- Issue: `infra/docker-compose.yml` declares all six services but each service command only prints a Phase 0 scaffold message instead of starting KME, Key Provider, SKIP, or encryptor logic.
- Files: `infra/docker-compose.yml`, `services/keyprovider/README.md`, `services/encryptor-sim/README.md`, `docs/test-plan.md`
- Impact: `docker compose` can validate shape but cannot exercise the promised ETSI-to-SKIP flow, ports, health, TLS, persistence, or service dependencies.
- Fix approach: Replace scaffold commands in `infra/docker-compose.yml` with real service entrypoints once implementation phases begin; add health checks and run-level tests after `docs/test-plan.md` moves beyond Phase 0 acceptance.

**Key Provider and encryptor services have no implementation:**
- Issue: `services/keyprovider/` and `services/encryptor-sim/` contain README scaffolds only; there are no FastAPI apps, routers, models, repositories, clients, or tests.
- Files: `services/keyprovider/README.md`, `services/encryptor-sim/README.md`, `services/README.md`, `docs/api-skip.md`
- Impact: SKIP endpoints `/capabilities`, `/key`, `/key/{keyId}`, and `/entropy` are contract-only. The project cannot prove SKIP field behavior, key ID parsing, base64-to-hex conversion, or separate provider SQLite persistence.
- Fix approach: Add Python service skeletons under `services/keyprovider/` and `services/encryptor-sim/` with FastAPI routers, Pydantic models, provider repositories, and contract tests following `docs/api-skip.md`.

**KME key source lacks deterministic synchronization support:**
- Issue: The baseline requires synchronized fake key material for KME-A/KME-B by `key_ID`, but `kms/src/routes/etsi014.rs` generates UUID key IDs and random bytes directly with `Uuid::new_v4()` and `getrandom()`.
- Files: `kms/src/routes/etsi014.rs`, `infra/docker-compose.yml`, `docs/api-etsi014-mock.md`, `docs/architecture.md`
- Impact: Independent KME-A/KME-B instances cannot return matching bytes for the same logical key ID, so the core `key_hex_alice == key_hex_bob` invariant cannot be implemented with the current KME behavior alone.
- Fix approach: Introduce a key-source abstraction in `kms/` with deterministic mode configured by `KEY_SOURCE_MODE`/`KEY_SOURCE_SEED` or YAML config; make `enc_keys` issue reproducible ETSI key IDs and bytes in the paired-baseline mode.

**Route handler owns too many protocol concerns:**
- Issue: `kms/src/routes/etsi014.rs` combines routing, request validation, topology lookup, identity resolution, key generation, storage lifecycle, logging, and error mapping in one file.
- Files: `kms/src/routes/etsi014.rs`
- Impact: Changes to topology policy, key source behavior, storage semantics, or ETSI error mapping are high-blast-radius and harder to unit test independently.
- Fix approach: Extract topology authorization, caller identity resolution, request validation, key-source selection, and key lifecycle operations into focused modules while keeping the route layer as the HTTP boundary.

## Known Bugs

**Concurrent `dec_keys` retrieval can return the same key more than once:**
- Symptoms: `retrieve_keys` first loads and authorizes all requested keys, then deletes them in a second loop. Two concurrent requests can both read the same key before either delete commits.
- Files: `kms/src/routes/etsi014.rs`, `kms/src/storage/sqlite.rs`, `kms/src/storage/memory.rs`
- Trigger: Send two simultaneous `dec_keys` requests for the same `key_ID` against SQLite or memory storage.
- Workaround: None in code. Current tests cover sequential repeat retrieval only in `kms/src/tests/etsi014.rs`.

**SQLite storage can panic on malformed persisted key length:**
- Symptoms: `SqliteKeyStorage::get_key_by_id` uses `.expect("Key has wrong size")` when converting a database BLOB to `[u8; 32]`.
- Files: `kms/src/storage/sqlite.rs`
- Trigger: A corrupted SQLite row, manual DB edit, migration mistake, or future schema bug stores a non-32-byte `key` BLOB.
- Workaround: Keep SQLite files private to the simulator and avoid manual mutation; replace the `expect` with an error return before accepting untrusted or migrated DB files.

**Optional ETSI extensions are rejected instead of ignored:**
- Symptoms: `validate_get_key_request` rejects any non-empty `extension_optional`, even though optional extensions are usually safe to ignore when unsupported.
- Files: `kms/src/routes/etsi014.rs`, `docs/api-etsi014-mock.md`
- Trigger: A client sends `extension_optional` with any value.
- Workaround: Do not send `extension_optional` until the supported ETSI subset decides whether optional extension rejection is intentional.

## Security Considerations

**Local HTTP identity is header-based:**
- Risk: When `use_mTLS` is false, caller identity comes from `x-sae-id`, which any local caller can spoof.
- Files: `kms/src/routes/etsi014.rs`, `kms/src/main.rs`, `docs/security-assumptions.md`, `docs/api-etsi014-mock.md`
- Current mitigation: Documentation marks this as local test mode only, and `resolve_calling_sae_id` ignores `x-sae-id` when `use_mTLS` is true.
- Recommendations: Keep HTTP mode out of production-like profiles, make compose mTLS profiles explicit, and add tests proving `x-sae-id` cannot override mTLS certificate identity.

**Key material is stored in plaintext SQLite/in-memory records:**
- Risk: ETSI key bytes are persisted as raw BLOBs in SQLite and cloned in memory without encryption-at-rest, secure zeroization, or lifecycle hardening.
- Files: `kms/src/storage/sqlite.rs`, `kms/src/storage/memory.rs`, `docs/security-assumptions.md`
- Current mitigation: The project explicitly states this is a non-production local baseline and avoids logging full key material in route logs.
- Recommendations: Keep this limitation visible in docs; for stronger experiments add filesystem permissions checks, redaction middleware, and optional encrypted storage or external secret handling.

**mTLS identity trusts certificate Common Name only:**
- Risk: `tls.rs` extracts the first subject CN as SAE identity. CN-based identity can be ambiguous if certificates contain unexpected subject formats or multiple names.
- Files: `kms/src/tls.rs`, `kms/src/routes/etsi014.rs`
- Current mitigation: rustls verifies the client certificate chain against configured CA roots before CN extraction, and topology checks reject unknown CN values.
- Recommendations: Add certificate fixture tests for missing CN, multiple CNs, unknown CNs, expired certificates, and SAN/CN policy decisions before relying on mTLS experiments.

**Config can be logged with sensitive paths:**
- Risk: `kms/src/main.rs` logs the full parsed config at debug level, including TLS private key paths and storage paths.
- Files: `kms/src/main.rs`, `kms/src/config/mod.rs`
- Current mitigation: No secret values are read from `.env` files in this mapper run, and key material itself is not logged by the route handlers.
- Recommendations: Avoid debug config dumps for production-like runs; add a redacted `Debug`/display path or log only non-sensitive config summaries.

## Performance Bottlenecks

**Memory storage uses a blocking mutex in async handlers:**
- Problem: `MemoryKeyStorage` uses `std::sync::Mutex<HashMap<...>>` inside async trait methods.
- Files: `kms/src/storage/memory.rs`
- Cause: All key operations serialize on one blocking mutex, and poisoned mutexes panic through `.unwrap()`.
- Improvement path: Use `tokio::sync::RwLock`, `DashMap`, or keep memory storage test-only and document it as unsuitable for concurrent load.

**Multi-key issuance performs one DB write per key:**
- Problem: `issue_keys` loops up to 128 keys and calls `register_or_update_key` for each key separately.
- Files: `kms/src/routes/etsi014.rs`, `kms/src/storage/sqlite.rs`
- Cause: Storage API has only single-key insert/update operations and no transaction/batch method.
- Improvement path: Add a batch registration method or route-level transaction for multi-key requests once throughput matters.

**Status reports static capacity instead of actual storage count:**
- Problem: `stored_key_count` returns `DEFAULT_MAX_KEY_COUNT` rather than the number of currently stored keys.
- Files: `kms/src/routes/etsi014.rs`
- Cause: `Etsi014KeyStorage` has no count/capacity API.
- Improvement path: Add `stored_key_count` to the storage trait and return real counts for memory/SQLite backends if clients use status for operational decisions.

## Fragile Areas

**One-time key lifecycle is not atomic:**
- Files: `kms/src/routes/etsi014.rs`, `kms/src/storage/mod.rs`, `kms/src/storage/sqlite.rs`, `kms/src/storage/memory.rs`
- Why fragile: The storage trait separates `get_key_by_id` from `delete_key`, so one-time consumption is enforced by route ordering rather than an atomic consume operation.
- Safe modification: Add `consume_key(key_id)` or `consume_keys(key_ids)` to `Etsi014KeyStorage`; implement SQLite with a transaction or delete-returning pattern and memory storage under one lock.
- Test coverage: Add concurrent retrieval tests and SQLite-backed lifecycle tests; current router tests cover only sequential in-memory behavior.

**Topology policy only checks known SAEs:**
- Files: `kms/src/config/mod.rs`, `kms/src/routes/etsi014.rs`, `docs/architecture.md`
- Why fragile: `find_kme_for_sae` validates whether an SAE is known, but there is no explicit allowlist of permitted master/slave pairs or directionality.
- Safe modification: Extend config with explicit authorized SAE relationships if the baseline needs pair-level policy beyond "both SAEs are known".
- Test coverage: Add tests for disallowed known-SAE pairs once pair-level policy is introduced.

**KME wire contract uses UUID key IDs while docs show deterministic `QKD-000001`:**
- Files: `kms/src/routes/etsi014.rs`, `docs/api-etsi014-mock.md`, `docs/api-skip.md`, `docs/test-plan.md`
- Why fragile: The docs and SKIP ID mapping examples assume `QKD-000001`, while the implemented KME serializes UUIDs. Future Key Provider parsing can drift if it treats examples as concrete format requirements.
- Safe modification: Decide whether ETSI `key_ID` is UUID or deterministic `QKD-*` for this baseline; update docs, tests, and SKIP parsing together.
- Test coverage: Add contract tests around SKIP `keyId` parsing and ETSI key ID format before implementing provider logic.

**Operational verification depends on unavailable local toolchain:**
- Files: `Cargo.toml`, `kms/Cargo.toml`, `docs/test-plan.md`
- Why fragile: `cargo test --target-dir /tmp/qkd-skip-cargo-target -p kms` could not run in this environment because `cargo` is not installed.
- Safe modification: Ensure CI or mapper environments include Rust, or add a Docker-based verification command using the pinned Rust image in `infra/docker-compose.yml`.
- Test coverage: Current local verification is blocked; rely on CI until the Rust toolchain is available.

## Scaling Limits

**KME storage has no pruning or TTL:**
- Current capacity: Status advertises `max_key_count = 1_000_000`, but no storage capacity enforcement exists.
- Limit: Unconsumed `enc_keys` can accumulate until SQLite or memory storage exhausts disk/RAM.
- Scaling path: Add maximum stored key enforcement, key expiration, cleanup jobs, and status counts before running long experiments.

**No service health checks or readiness gates:**
- Current capacity: Compose declares dependencies only with `depends_on`, and services do not expose real readiness.
- Limit: Once real commands replace scaffold prints, startup ordering can race KME/Provider availability.
- Scaling path: Add health endpoints to KME/Key Provider services, compose health checks, and dependency conditions.

## Dependencies at Risk

**Rust 2024 and very recent dependencies:**
- Risk: `kms/Cargo.toml` uses edition 2024 and recent versions across Axum, rustls, sqlx, tokio, and x509-parser.
- Impact: Contributors need a current Rust toolchain; older lab machines or CI images may fail before tests run.
- Migration plan: Pin a known Rust toolchain with `rust-toolchain.toml` or document the minimum Rust version needed for edition 2024 and dependencies.

**Python service dependencies are not pinned in repo manifests:**
- Risk: Project docs recommend FastAPI, Uvicorn, Pydantic, SQLAlchemy, httpx, pytest, and ruff, but there is no `pyproject.toml` or lockfile under `services/`.
- Impact: Future Python service implementation can drift across developer machines and CI.
- Migration plan: Add per-service or workspace Python dependency manifests before writing FastAPI logic.

## Missing Critical Features

**SKIP service implementation:**
- Problem: No implementation exists for `GET /capabilities`, `GET /key`, `GET /key/{keyId}`, or `GET /entropy`.
- Blocks: End-to-end SKIP validation, simulated encryptor tests, key ID handoff, and RFC8784 PPK preparation.

**Provider persistence model:**
- Problem: Separate SQLite state is planned but no Key Provider schema or repository exists.
- Blocks: Restart behavior, inbound key resolution, key ID mapping, provider independence verification, and no-central-db guarantees.

**mTLS integration tests and certificate fixtures:**
- Problem: mTLS code exists in `kms/src/tls.rs`, but test coverage listed in `docs/test-plan.md` remains future-oriented.
- Blocks: Confidence that certificate Common Name identity works at the live server boundary.

**Runnable local baseline:**
- Problem: `infra/docker-compose.yml` does not start executable API services.
- Blocks: Lab users cannot run the baseline flow without implementing service entrypoints and config files.

## Test Coverage Gaps

**KME SQLite behavior:**
- What's not tested: SQLite persistence across restart, malformed key rows, atomic consume semantics, and real stored key counts.
- Files: `kms/src/storage/sqlite.rs`, `kms/src/tests/etsi014.rs`
- Risk: Durable experiments can diverge from in-memory router test behavior.
- Priority: High

**Concurrent `dec_keys`:**
- What's not tested: Two simultaneous requests for the same key ID.
- Files: `kms/src/routes/etsi014.rs`, `kms/src/storage/mod.rs`, `kms/src/tests/etsi014.rs`
- Risk: One-time consumption can be violated under race conditions.
- Priority: High

**mTLS identity path:**
- What's not tested: Live TLS acceptor behavior, CN extraction failures, unknown CN rejection, and `x-sae-id` ignored in mTLS mode.
- Files: `kms/src/tls.rs`, `kms/src/routes/etsi014.rs`, `docs/test-plan.md`
- Risk: Security-critical identity behavior can regress unnoticed.
- Priority: High

**SKIP and end-to-end flow:**
- What's not tested: SKIP endpoints, key ID mapping, ETSI base64 to SKIP hex conversion, entropy behavior, simulated encryptor handoff, and `key_hex_alice == key_hex_bob`.
- Files: `docs/api-skip.md`, `services/keyprovider/README.md`, `services/encryptor-sim/README.md`, `infra/docker-compose.yml`
- Risk: The core project value remains unproven after Phase 0.
- Priority: High

---

*Concerns audit: 2026-06-04*
