# Technology Stack

**Analysis Date:** 2026-06-04

## Languages

**Primary:**
- Rust 2024 edition - Implemented ETSI GS QKD 014 KME simulator in `kms/`; declared by `kms/Cargo.toml`.

**Secondary:**
- Python 3.12 - Planned runtime for future Key Provider and simulated encryptor services; currently used only as placeholder commands and images in `infra/docker-compose.yml`.
- YAML - Runtime/service configuration for Docker Compose in `infra/docker-compose.yml`; KME runtime configuration is YAML loaded by `kms/src/config/mod.rs`.
- Markdown - Contract and planning documentation in `README.md`, `docs/api-etsi014-mock.md`, `docs/api-skip.md`, `docs/data-model.md`, and `kms/README.md`.
- JavaScript/Node - Not active. `package-lock.json` exists with no packages and no corresponding `package.json`.

## Runtime

**Environment:**
- Rust/Cargo workspace - Root workspace at `Cargo.toml` with member `kms`.
- Tokio async runtime - `kms/src/main.rs` uses `#[tokio::main]` and Axum servers.
- Docker Compose - Primary local multi-service runtime skeleton in `infra/docker-compose.yml`.
- Container images:
  - `rust:1.87-bookworm` for `kme-a` and `kme-b` Compose placeholders in `infra/docker-compose.yml`.
  - `python:3.12-slim-bookworm` for future `keyprovider-*` and `encryptor-*-sim` Compose placeholders in `infra/docker-compose.yml`.

**Package Manager:**
- Cargo - Rust dependency and build manager for `Cargo.toml` and `kms/Cargo.toml`.
- Lockfile: missing for Rust (`Cargo.lock` not detected in repo).
- npm - Not active; `package-lock.json` is empty and no `package.json` was detected.

## Frameworks

**Core:**
- Axum 0.8.8 - HTTP routing and JSON handlers for ETSI 014 endpoints in `kms/src/routes/mod.rs` and `kms/src/routes/etsi014.rs`.
- axum-server 0.7.2 - HTTPS server support with rustls acceptor integration in `kms/src/main.rs` and `kms/src/tls.rs`.
- Tokio 1.50.0 - Async runtime, networking, and server execution in `kms/src/main.rs`.
- SQLx 0.8.6 - SQLite connection pool and queries in `kms/src/storage/mod.rs` and `kms/src/storage/sqlite.rs`.
- Rustls 0.23.31 / tokio-rustls 0.26.4 - mTLS server authentication and client certificate handling in `kms/src/tls.rs`.
- FastAPI 0.136.3 - Planned stack for future Python services per `AGENTS.md`, `services/README.md`, and `services/keyprovider/README.md`; no FastAPI code is implemented yet.

**Testing:**
- Rust built-in test harness with `#[tokio::test]` - Async KME API tests in `kms/src/tests/etsi014.rs`.
- tower 0.5.2 test utility - Axum router `oneshot` testing in `kms/src/tests/etsi014.rs`.
- pytest 9.0.3 - Planned future Python contract/integration test runner per `AGENTS.md`; no pytest tests detected.

**Build/Dev:**
- clap 4.6.0 - CLI parsing for `kms --config <path>` in `kms/src/main.rs`.
- tracing 0.1.44 and tracing-subscriber 0.3.23 - Structured logging and env-filter setup in `kms/src/main.rs`.
- serde 1.0.228, serde_json 1.0.149, serde_yaml 0.9.34 - JSON request/response serialization and YAML config loading in `kms/src/config/mod.rs` and `kms/src/routes/etsi014.rs`.
- Docker Compose - Multi-service topology and local persistent volumes in `infra/docker-compose.yml`.

## Key Dependencies

**Critical:**
- `axum` 0.8.8 - Defines the ETSI 014 route surface and handler extraction in `kms/src/routes/etsi014.rs`.
- `sqlx` 0.8.6 with `sqlite` - Durable KME key storage backend in `kms/src/storage/sqlite.rs`.
- `rustls`, `tokio-rustls`, `axum-server`, `rustls-pemfile`, `x509-parser` - Required mTLS mode, certificate parsing, and SAE Common Name extraction in `kms/src/tls.rs`.
- `base64` 0.22.1 - ETSI key material encoding in `kms/src/routes/etsi014.rs`; ETSI keys remain base64 per `docs/data-model.md`.
- `getrandom` 0.2.17 - Current simulated 256-bit key generation source in `kms/src/routes/etsi014.rs`.
- `uuid` 1.22.0 - Current ETSI `key_ID` type in `kms/src/routes/etsi014.rs` and storage key identifier type in `kms/src/storage/mod.rs`.

**Infrastructure:**
- `anyhow` 1.0.102 - Error propagation in `kms/src/main.rs`, `kms/src/config/mod.rs`, `kms/src/storage/mod.rs`, and `kms/src/tls.rs`.
- `async-trait` 0.1.89 - Async storage trait implementation in `kms/src/storage/mod.rs`, `kms/src/storage/memory.rs`, and `kms/src/storage/sqlite.rs`.
- `futures-util` 0.3.31 and `tower-layer` 0.3.3 - Custom TLS acceptor service layering in `kms/src/tls.rs`.
- Docker named volumes - `kme_a_data`, `kme_b_data`, `kp_a_data`, and `kp_b_data` in `infra/docker-compose.yml`.

## Configuration

**Environment:**
- Compose service configuration is declared in `infra/docker-compose.yml`.
- KME Compose placeholders define `KME_ID`, `KME_SOURCE`, `USE_MTLS`, `SAE_ID`, `PEER_SAE_ID`, `KEY_SOURCE_MODE`, `KEY_SOURCE_SEED`, and `SQLITE_PATH` in `infra/docker-compose.yml`.
- Key Provider Compose placeholders define `LOCAL_SAE_ID`, `PEER_SAE_ID`, `LOCAL_SYSTEM_ID`, `REMOTE_SYSTEM_ID`, `KME_BASE_URL`, `KME_ID`, `SKIP_KEY_ID_FORMAT`, `SQLITE_PATH`, `ETSI_KEY_ENCODING`, and `SKIP_KEY_ENCODING` in `infra/docker-compose.yml`.
- Encryptor simulator Compose placeholders define `LOCAL_SYSTEM_ID`, `REMOTE_SYSTEM_ID`, and `SKIP_BASE_URL` in `infra/docker-compose.yml`.
- No `.env` files were detected at repo root or first-level directories.

**Build:**
- `Cargo.toml` defines the workspace.
- `kms/Cargo.toml` defines the implemented KME crate and dependencies.
- `infra/docker-compose.yml` defines the local topology, ports, service dependencies, and persistent volumes.
- KME runtime config is YAML loaded by `Config::load` in `kms/src/config/mod.rs`; documented example shape is in `kms/README.md`.

## Platform Requirements

**Development:**
- Rust toolchain with Cargo for `cargo build -p kms`, `cargo test -p kms`, and workspace verification from `README.md` and `kms/README.md`.
- Docker Compose plugin for the local six-service scaffold in `infra/docker-compose.yml`.
- Writable SQLite paths for KME and planned provider persistence.
- PEM server certificate, PEM server private key, and PEM client CA certificate when `use_mTLS: true`, as documented in `kms/README.md` and loaded by `kms/src/tls.rs`.

**Production:**
- Not applicable for production deployment. The repository documents a local simulator baseline, not production KMS/VPN software, in `README.md`, `docs/security-assumptions.md`, and `kms/README.md`.

---

*Stack analysis: 2026-06-04*
