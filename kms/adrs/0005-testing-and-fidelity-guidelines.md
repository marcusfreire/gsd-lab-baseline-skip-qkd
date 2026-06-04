# ADR 0005: Testing and Fidelity Guidelines

## Status

Accepted

## Context

The `kms` crate simulates security-sensitive KME behavior. Bugs in topology
authorization, one-time key consumption, public JSON field names, request
validation, or HTTP status codes can make client experiments misleading.

The `otp-vpn` crate uses focused unit tests plus higher-level protocol and
integration tests. The simulator should mirror that discipline.

## Decision

Use layered tests for the `kms` crate.

Required verification commands:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Test Layers

### Unit Tests

Unit tests should cover internal behavior that can be exercised without HTTP:

- configuration loading and validation;
- topology lookup and SAE authorization decisions;
- key-source behavior, including deterministic fakes when added;
- ETSI storage implementations;
- one-time key lifecycle rules;
- request validation helpers;
- protocol error mapping where it is factored out of route handlers.

Internal failures may use `anyhow`, but protocol-facing validation should be
tested through expected ETSI status codes and error data models.

### Router Tests

ETSI protocol behavior must also be tested through the real Axum router.

Router tests should cover:

- exact ETSI paths and supported methods;
- GET shortcut and POST request forms;
- Serde field names matching ETSI JSON names;
- status responses from configured topology;
- `enc_keys` issuing base64-encoded key containers;
- `dec_keys` retrieval by `key_ID`;
- one-time consumption after successful `dec_keys`;
- rejection of unknown caller, master, or slave SAEs;
- rejection of wrong master/slave ownership;
- unsupported key sizes and unsupported mandatory extensions;
- ETSI-shaped error status codes and response bodies.

The existing `kms/src/tests/etsi014.rs` pattern is the baseline for these
tests.

### Integration Tests

Integration tests should prove boundaries that unit and router tests cannot
cover:

- live HTTP server behavior;
- mTLS identity extraction from client certificates;
- client interoperability with `otp-vpn` and other experimental SAE clients;
- SQLite persistence across process restart;
- future protocol modules configured in the same binary.

Tests requiring live sockets, certificate generation, or external client
processes may be ignored or feature-gated so ordinary `cargo test --workspace`
stays fast and deterministic.

## Consequences

- Protocol-fidelity regressions are caught at the HTTP boundary where clients
  observe them.
- Unit tests keep topology, storage, and key-source behavior easy to reason
  about.
- Future protocols inherit the same expectation: test reusable internals
  directly and public protocol behavior through the real router.
