# External Integrations

**Analysis Date:** 2026-06-04

## APIs & External Services

**Local ETSI GS QKD 014 API:**
- KME simulator - Implemented by `kms/` and exposed under `/api/v1/keys`.
  - SDK/Client: Axum server in `kms/src/routes/mod.rs` and `kms/src/routes/etsi014.rs`; future Python Key Providers call it over HTTP(S) per `services/keyprovider/README.md`.
  - Auth: mTLS client certificate Common Name when `use_mTLS: true`; `x-sae-id` header only in local HTTP mode when mTLS is disabled.
  - Routes: `GET /api/v1/keys/{slave_SAE_ID}/status`, `GET|POST /api/v1/keys/{slave_SAE_ID}/enc_keys`, and `GET|POST /api/v1/keys/{master_SAE_ID}/dec_keys` in `kms/src/routes/etsi014.rs`.

**Local SKIP API:**
- Key Provider SKIP service - Planned provider-to-encryptor REST boundary documented in `docs/api-skip.md`; no route handlers are implemented in `services/keyprovider/`.
  - SDK/Client: Planned Python/FastAPI service per `AGENTS.md`, `services/README.md`, and `services/keyprovider/README.md`.
  - Auth: Not implemented; simulated local encryptors are the only documented consumers.
  - Routes: `GET /capabilities`, `GET /key?remoteSystemID={id}`, `GET /key/{keyId}?remoteSystemID={id}`, and `GET /entropy` in `docs/api-skip.md`.

**Future VPN/IKE Integration:**
- RFC8784/IKEv2 PPK and real Cisco encryptors - Future work only.
  - SDK/Client: Not detected.
  - Auth: Not applicable.
  - Scope marker: `README.md`, `docs/api-skip.md`, and `services/encryptor-sim/README.md` explicitly keep real Cisco integration and real IKEv2/RFC8784 negotiation out of the current baseline.

**External SaaS APIs:**
- Not detected. No Stripe, Supabase, AWS, cloud database, hosted auth, monitoring, or webhook client imports were found in implemented source files.

## Data Storage

**Databases:**
- SQLite for KME storage.
  - Connection: YAML storage parameter `storage.params.path` loaded by `kms/src/config/mod.rs` and consumed in `kms/src/storage/mod.rs`.
  - Client: SQLx `SqlitePool` in `kms/src/storage/sqlite.rs`.
  - Schema: `keys(key_id TEXT PRIMARY KEY, master_sae_id TEXT NOT NULL, slave_sae_id TEXT NOT NULL, key BLOB NOT NULL)` created in `kms/src/storage/sqlite.rs`.
- In-memory KME storage.
  - Connection: Not applicable.
  - Client: `Mutex<HashMap<KeyID, KeyData>>` in `kms/src/storage/memory.rs`.
  - Use: Tests and ephemeral runs; warns that data is lost on restart in `kms/src/storage/mod.rs`.
- Per-provider SQLite storage.
  - Connection: `SQLITE_PATH` per `keyprovider-a` and `keyprovider-b` in `infra/docker-compose.yml`.
  - Client: Planned Python service; not implemented in `services/keyprovider/`.
  - Constraint: Provider databases must be independent with no central shared provider DB per `README.md`, `docs/data-model.md`, and `services/keyprovider/README.md`.

**File Storage:**
- Local filesystem only.
- KME SQLite volumes: `kme_a_data` and `kme_b_data` in `infra/docker-compose.yml`.
- Provider SQLite volumes: `kp_a_data` and `kp_b_data` in `infra/docker-compose.yml`.
- Certificate files are read from paths in YAML `tls` config by `kms/src/tls.rs`; do not store private key contents in repo documentation.

**Caching:**
- None detected.

## Authentication & Identity

**Auth Provider:**
- Custom local SAE identity policy.
  - Implementation: `resolve_calling_sae_id` in `kms/src/routes/etsi014.rs`.
  - mTLS mode: `AuthenticatedPeerAcceptor` in `kms/src/tls.rs` requires client certificates and extracts the SAE ID from the subject Common Name.
  - Local HTTP mode: `x-sae-id` is accepted only when `use_mTLS` is disabled in `kms/src/routes/etsi014.rs`.
  - Topology authorization: `kme_topology` from `kms/src/config/mod.rs` is used to verify caller, master SAE, and slave SAE identities in `kms/src/routes/etsi014.rs`.

**External Identity Provider:**
- Not detected. No OAuth/OIDC/SAML provider is configured or imported.

## Monitoring & Observability

**Error Tracking:**
- None detected.

**Logs:**
- Rust tracing is initialized in `kms/src/main.rs` using `tracing_subscriber::fmt()` and `EnvFilter::from_default_env()`.
- ETSI handlers log status, issuing, retrieval, unauthorized ownership attempts, and backend errors in `kms/src/routes/etsi014.rs`.
- TLS setup logs certificate path metadata in `kms/src/tls.rs`.
- Key material must not be logged; `docs/data-model.md` and `AGENTS.md` require fingerprints/summaries only.

## CI/CD & Deployment

**Hosting:**
- Local Docker Compose only.
- `infra/docker-compose.yml` defines six local services: `kme-a`, `kme-b`, `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and `encryptor-b-sim`.
- No production hosting target detected.

**CI Pipeline:**
- None detected. No GitHub Actions, GitLab CI, or other CI config files were found in the repo scan.

## Environment Configuration

**Required env vars:**
- KME Compose placeholders: `KME_ID`, `KME_SOURCE`, `USE_MTLS`, `SAE_ID`, `PEER_SAE_ID`, `KEY_SOURCE_MODE`, `KEY_SOURCE_SEED`, `SQLITE_PATH` in `infra/docker-compose.yml`.
- Key Provider Compose placeholders: `LOCAL_SAE_ID`, `PEER_SAE_ID`, `LOCAL_SYSTEM_ID`, `REMOTE_SYSTEM_ID`, `KME_BASE_URL`, `KME_ID`, `SKIP_KEY_ID_FORMAT`, `SQLITE_PATH`, `ETSI_KEY_ENCODING`, `SKIP_KEY_ENCODING` in `infra/docker-compose.yml`.
- Encryptor simulator Compose placeholders: `LOCAL_SYSTEM_ID`, `REMOTE_SYSTEM_ID`, `SKIP_BASE_URL` in `infra/docker-compose.yml`.
- Implemented KME binary does not read these environment variables directly; it reads a YAML file selected by `--config` in `kms/src/main.rs` and `kms/src/config/mod.rs`.

**Secrets location:**
- No `.env` file detected.
- mTLS secrets are external PEM files referenced by YAML keys `tls.server_cert_path`, `tls.server_key_path`, and `tls.client_ca_cert_path` in `kms/src/config/mod.rs` and `kms/README.md`.
- Do not commit or document private key material; only file path configuration belongs in docs.

## Webhooks & Callbacks

**Incoming:**
- ETSI 014 local incoming REST endpoints are implemented in `kms/src/routes/etsi014.rs`.
- SKIP incoming REST endpoints are documented in `docs/api-skip.md` but not implemented.
- No third-party webhook endpoints detected.

**Outgoing:**
- Future KeyProvider-A calls `KME-A` via `KME_BASE_URL=https://kme-a:8443` in `infra/docker-compose.yml`.
- Future KeyProvider-B calls `KME-B` via `KME_BASE_URL=https://kme-b:8443` in `infra/docker-compose.yml`.
- Future Encryptor-A sim calls `KeyProvider-A` via `SKIP_BASE_URL=http://keyprovider-a:8080` in `infra/docker-compose.yml`.
- Future Encryptor-B sim calls `KeyProvider-B` via `SKIP_BASE_URL=http://keyprovider-b:8080` in `infra/docker-compose.yml`.
- No implemented outgoing HTTP client code was detected in `services/` or `kms/`.

---

*Integration audit: 2026-06-04*
