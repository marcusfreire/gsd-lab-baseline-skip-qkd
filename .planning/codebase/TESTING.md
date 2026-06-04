# Testing Patterns

**Analysis Date:** 2026-06-04

## Test Framework

**Runner:**
- Rust built-in test runner through Cargo.
- Async tests use Tokio via `#[tokio::test]` in `kms/src/tests/etsi014.rs`.
- Config: `Cargo.toml` and `kms/Cargo.toml`.

**Assertion Library:**
- Rust standard assertions: `assert_eq!` in `kms/src/tests/etsi014.rs`.
- JSON assertions use `serde_json::Value` indexing in `kms/src/tests/etsi014.rs`.

**Run Commands:**
```bash
cargo test --workspace                       # Run all Rust workspace tests
cargo test -p kms                            # Run only the KMS crate tests
cargo test --target-dir /tmp/quiin-cargo-target -p kms  # Run KMS tests when the default target dir is not writable
cargo fmt --all -- --check                   # Check Rust formatting
cargo clippy --workspace --all-targets -- -D warnings  # Strict lint check
```

## Test File Organization

**Location:**
- Rust tests are crate-internal under `kms/src/tests/`.
- The current implemented protocol test module is `kms/src/tests/etsi014.rs`.
- The test module root is `kms/src/tests/mod.rs`.
- Project-level manual and future test obligations are documented in `docs/test-plan.md`.
- KMS-specific testing guidance is documented in `kms/adrs/0005-testing-and-fidelity-guidelines.md`.

**Naming:**
- Test function names are behavior descriptions in `snake_case`: `status_returns_topology_for_master_and_slave`, `post_workflow_issues_and_consumes_keys`, `wrong_slave_or_master_cannot_retrieve_key` in `kms/src/tests/etsi014.rs`.
- Test modules use the protocol name: `etsi014` in `kms/src/tests/mod.rs` and `kms/src/tests/etsi014.rs`.

**Structure:**
```text
kms/src/
├── tests/
│   ├── mod.rs          # Test module registration
│   └── etsi014.rs      # Axum router tests for ETSI 014 behavior
└── routes/
    └── etsi014.rs      # Route handlers under test
```

## Test Structure

**Suite Organization:**
```rust
fn test_config() -> Config {
    // Build deterministic topology for tests.
}

fn test_app() -> axum::Router {
    // Build the real router with in-memory storage.
}

async fn send_request(app: axum::Router, request: Request<Body>) -> (StatusCode, Value) {
    // Drive the router through tower::ServiceExt::oneshot and parse JSON.
}

#[tokio::test]
async fn post_workflow_issues_and_consumes_keys() {
    let app = test_app();
    // issue enc_keys, retrieve dec_keys, then prove repeat retrieval fails.
}
```

**Patterns:**
- Build a full Axum router with real route wiring using `routes::build_router` in `kms/src/tests/etsi014.rs`.
- Use `MemoryKeyStorage` for deterministic, isolated router tests in `kms/src/tests/etsi014.rs`.
- Construct HTTP requests with `Request::builder()` and `Body::from` or `Body::empty()` in `kms/src/tests/etsi014.rs`.
- Parse response bodies with `axum::body::to_bytes` and `serde_json::from_slice` in `send_request` in `kms/src/tests/etsi014.rs`.
- Assert observable HTTP status codes and public JSON field names rather than private helper behavior for route-level protocol fidelity.

## Mocking

**Framework:** No standalone mocking framework detected.

**Patterns:**
```rust
let app_state = Arc::new(AppState {
    storage: Box::new(MemoryKeyStorage::new()),
    config: test_config(),
});

routes::build_router(app_state)
```

**What to Mock:**
- Use in-memory storage for router tests that do not need SQLite persistence: `kms/src/storage/memory.rs` and `kms/src/tests/etsi014.rs`.
- Use synthetic topology in tests through `test_config` in `kms/src/tests/etsi014.rs`.
- Future tests may use deterministic fake key sources for cross-KME synchronization, as required by `docs/test-plan.md` and `kms/adrs/0005-testing-and-fidelity-guidelines.md`.

**What NOT to Mock:**
- Do not mock the Axum router for protocol tests. Exercise `routes::build_router` through `tower::util::ServiceExt` as shown in `kms/src/tests/etsi014.rs`.
- Do not mock public JSON field names, HTTP methods, or paths. Assert actual API behavior against `docs/api-etsi014-mock.md`.
- Do not use real key material values in snapshots or logs. Assert encoding shape, equality, length, or fingerprints.

## Fixtures and Factories

**Test Data:**
```rust
let mut topology = HashMap::new();
topology.insert(
    "KME_01".to_string(),
    KmePeerConfig {
        connected_saes: vec!["SAE_01".to_string(), "SAE_02".to_string()],
    },
);
topology.insert(
    "KME_02".to_string(),
    KmePeerConfig {
        connected_saes: vec!["SAE_03".to_string(), "SAE_04".to_string()],
    },
);
```

**Location:**
- Inline test helpers and fixtures live in `kms/src/tests/etsi014.rs`.
- No shared fixture directory is detected.
- Future integration fixtures for certificates, Compose scenarios, or deterministic key sources should be added near the tests that use them unless they become shared across multiple protocol modules.

## Coverage

**Requirements:** No numeric coverage target is enforced.

**View Coverage:**
```bash
# No coverage command is configured in the repo.
```

## Test Types

**Unit Tests:**
- Not yet separated into dedicated unit-test modules for config, storage, TLS, or validation helpers.
- `kms/adrs/0005-testing-and-fidelity-guidelines.md` requires future unit tests for configuration loading, topology lookup, key-source behavior, ETSI storage implementations, one-time lifecycle rules, request validation helpers, and factored protocol error mapping.

**Integration Tests:**
- Router-level integration tests exist in `kms/src/tests/etsi014.rs`; they exercise the real Axum router without opening live sockets.
- Live HTTP, mTLS certificate identity extraction, SQLite restart persistence, and cross-client interoperability are documented future integration areas in `kms/adrs/0005-testing-and-fidelity-guidelines.md`.
- Docker Compose validation is documented in `docs/test-plan.md` with `docker compose -f infra/docker-compose.yml config`.

**E2E Tests:**
- Not implemented.
- Future end-to-end SKIP flow is documented in `docs/test-plan.md`: Encryptor-A obtains a key from KeyProvider-A, simulated handoff transfers `keyId`, Encryptor-B retrieves matching key material from KeyProvider-B, and the test asserts both SKIP hex keys match.

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn get_shortcuts_work_for_single_key_flow() {
    let app = test_app();
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/keys/SAE_03/enc_keys?number=1&size=256")
        .header("x-sae-id", "SAE_01")
        .body(Body::empty())
        .expect("request should build");

    let (status, body) = send_request(app, request).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["keys"][0]["key_ID"].is_string(), true);
}
```

**Error Testing:**
```rust
let (status, body) = send_request(app, unsupported_extension_request).await;

assert_eq!(status, StatusCode::BAD_REQUEST);
assert_eq!(
    body["message"],
    "not all extension_mandatory parameters are supported"
);
```

---

*Testing analysis: 2026-06-04*
