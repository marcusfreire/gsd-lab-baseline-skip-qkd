# KMS Experimental Key-Management Simulator

`kms` is a Rust Key Management Entity simulator for tools and experiments that
use Quantum Key Distribution devices. Its first supported protocol is ETSI GS
QKD 014, exposed under `/api/v1/keys`.

This crate is not production KMS software. It is a high-fidelity simulator for
local experiments, interoperability work, and SAE client development.

## What It Does

The simulator provides application-facing KME behavior without requiring
physical QKD hardware:

- exposes ETSI GS QKD 014 `status`, `enc_keys`, and `dec_keys` endpoints;
- models KME-to-SAE topology from YAML configuration;
- treats topology as a protocol security policy boundary;
- issues simulated 256-bit QKD key material from operating-system randomness;
- stores issued keys with ETSI master/slave SAE ownership metadata;
- enforces one-time `dec_keys` retrieval by deleting returned keys;
- supports SQLite and in-memory storage backends;
- supports plain HTTP test mode and HTTPS with required client certificates;
- resolves mTLS caller SAE identity from the client certificate Common Name.

## Protocol Surface

The ETSI 014 implementation exposes these routes:

- `GET /api/v1/keys/{slave_SAE_ID}/status`
- `POST /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `GET /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `POST /api/v1/keys/{master_SAE_ID}/dec_keys`
- `GET /api/v1/keys/{master_SAE_ID}/dec_keys`

Public JSON fields intentionally preserve ETSI names such as `source_KME_ID`,
`target_KME_ID`, `master_SAE_ID`, `slave_SAE_ID`, `key_ID`, and `key_IDs`.
Internal Rust names should remain idiomatic and use Serde `rename` attributes
where needed.

## Requirements

- Rust toolchain with Cargo.
- Linux, macOS, or another platform supported by the Rust dependencies.
- For SQLite storage, a writable database path.
- For mTLS mode:
  - server certificate;
  - server private key;
  - client CA certificate;
  - client certificates whose subject Common Name matches configured SAE IDs.

## Build

From the workspace root:

```bash
cargo build -p kms
```

For release builds:

```bash
cargo build --release -p kms
```

## Run

From the workspace root:

```bash
cargo run -p kms -- --config config/example-kms.yaml
```

The server mode depends on `use_mTLS`:

- `use_mTLS: true` starts HTTPS with required client certificate
  authentication;
- `use_mTLS: false` starts plain HTTP and accepts caller SAE identity from
  `x-sae-id`.

`x-sae-id` is only valid when mTLS is disabled.

## Test

Recommended verification commands:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

To run only the `kms` tests:

```bash
cargo test -p kms
```

If the default Cargo target directory is not writable in your environment, use
a writable target directory:

```bash
cargo test --target-dir /tmp/quiin-cargo-target -p kms
```

## Configuration

Configuration is YAML. The example file is `config/example-kms.yaml`.

```yaml
listen: "0.0.0.0:8080"
use_mTLS: true

tls:
  server_cert_path: "./data/certs/kme-server.pem"
  server_key_path: "./data/certs/kme-server-key.pem"
  client_ca_cert_path: "./data/certs/ca.pem"

storage:
  type: "sqlite"
  params:
    path: "./data/kms.db"

kme_topology:
  KME_01:
    connected_saes:
      - SAE_01
      - SAE_02
  KME_02:
    connected_saes:
      - SAE_03
      - SAE_04
```

### `listen`

Socket address to bind.

Examples:

- `127.0.0.1:8080`
- `0.0.0.0:8080`
- `0.0.0.0:8443`

### `use_mTLS`

Controls transport authentication mode.

When `true`:

- the server starts HTTPS;
- client certificates are required;
- caller SAE identity comes from the client certificate subject Common Name;
- the `tls` section is required.

When `false`:

- the server starts plain HTTP;
- caller SAE identity comes from the `x-sae-id` request header;
- this mode is for local tests and development only.

### `tls`

Required only when `use_mTLS: true`.

Options:

- `server_cert_path`: PEM certificate chain presented by the KME server.
- `server_key_path`: PEM private key for the KME server certificate.
- `client_ca_cert_path`: PEM CA certificate used to verify client
  certificates.

Client certificate Common Names must match SAE IDs in `kme_topology`.

### `storage`

Selects key persistence backend.

Options:

- `type`: `sqlite` or `memory`.
- `params`: backend-specific string parameters.

SQLite parameters:

- `path`: SQLite database path.

Example:

```yaml
storage:
  type: "sqlite"
  params:
    path: "./data/kms.db"
```

Memory storage example:

```yaml
storage:
  type: "memory"
  params: {}
```

SQLite is the default configured durable backend in the example configuration.
Memory storage is useful for ephemeral runs, unit tests, and local development.

### `kme_topology`

Declares which SAEs are connected to each simulated KME.

The topology is an authorization boundary. Protocol handlers use it to:

- verify the caller SAE is known;
- resolve `source_KME_ID` for the caller SAE;
- resolve `target_KME_ID` for the requested peer SAE;
- reject requests for unknown SAEs;
- reject `dec_keys` calls whose master or slave SAE does not match stored key
  ownership.

Example:

```yaml
kme_topology:
  KME_01:
    connected_saes:
      - SAE_01
      - SAE_02
  KME_02:
    connected_saes:
      - SAE_03
      - SAE_04
```

## Example Requests

Plain HTTP test mode requires `use_mTLS: false` and an `x-sae-id` header.

Status:

```bash
curl -H 'x-sae-id: SAE_01' \
  http://127.0.0.1:8080/api/v1/keys/SAE_03/status
```

Issue keys:

```bash
curl -X POST \
  -H 'content-type: application/json' \
  -H 'x-sae-id: SAE_01' \
  -d '{ "number": 1, "size": 256 }' \
  http://127.0.0.1:8080/api/v1/keys/SAE_03/enc_keys
```

Retrieve a key:

```bash
curl -X POST \
  -H 'content-type: application/json' \
  -H 'x-sae-id: SAE_03' \
  -d '{ "key_IDs": [ { "key_ID": "550e8400-e29b-41d4-a716-446655440000" } ] }' \
  http://127.0.0.1:8080/api/v1/keys/SAE_01/dec_keys
```

## Product Requirements Document

### Problem Statement

QKD client experiments in this repository need a high-fidelity Key Management
Entity simulator that behaves like QKD devices exposing ETSI GS QKD 014. The
simulator must let VPN prototypes and future SAE clients validate protocol
behavior, topology authorization, SAE identity handling, key ownership, and
one-time decrypt-key retrieval without requiring physical QKD hardware.

The existing `kms` crate already implements a minimal ETSI 014 server, but the
product direction needs to be explicit: `kms` is an experimental simulator, not
a production KMS, and it should be structured so future experimental
key-distribution protocols can be added inside the same binary.

### Solution

Evolve and document the `kms` crate as a single configurable experimental
key-management simulator. ETSI GS QKD 014 remains the first supported protocol.
The public ETSI API must strictly follow the protocol surface for supported
methods, JSON field names, status codes, and error data models. Internally, the
crate should use idiomatic Rust names and clear module boundaries for protocol
routes, configuration, topology/security policy, caller identity, key sources,
and ETSI key storage.

The simulator will provide realistic application-facing KME behavior for
experiments:

- topology-backed SAE authorization;
- mTLS caller identity from client certificate Common Name;
- plain HTTP `x-sae-id` identity only when mTLS is disabled;
- simulated QKD key material from operating-system randomness;
- an explicit key-source interface for future deterministic and experimental
  key sources;
- ETSI-shaped storage with one-time `dec_keys` consumption;
- SQLite as the default durable storage backend;
- memory storage for ephemeral runs and tests;
- layered tests covering unit behavior and real Axum router protocol fidelity.

### User Stories

1. As a QKD VPN developer, I want a local ETSI 014 KME simulator, so that I can
   test SAE clients without physical QKD devices.
2. As a QKD VPN developer, I want the simulator to expose ETSI 014 routes under
   `/api/v1/keys`, so that client code can use the same endpoint structure as a
   real KME.
3. As a QKD VPN developer, I want `Get status` to return source and target KME
   information from configured topology, so that startup compatibility checks
   exercise realistic KME metadata.
4. As a QKD VPN developer, I want `Get key` to issue key containers with
   `key_ID` and base64 key material, so that clients can test outbound key
   acquisition.
5. As a QKD VPN developer, I want `Get key with key IDs` to return matching
   keys by ID, so that clients can test receive-side key resolution.
6. As a protocol implementer, I want public JSON fields to preserve ETSI names
   such as `source_KME_ID`, `master_SAE_ID`, and `key_ID`, so that wire
   compatibility is not broken by idiomatic Rust internals.
7. As a protocol implementer, I want Rust internals to use idiomatic names and
   Serde renames, so that the code remains maintainable while the API remains
   standard-compliant.
8. As a security reviewer, I want topology to be an authorization boundary, so
   that unknown or unauthorized SAE relationships cannot issue or retrieve key
   material.
9. As a security reviewer, I want caller identity to be resolved before protocol
   authorization, so that route handlers do not trust unvalidated request data.
10. As a security reviewer, I want mTLS mode to derive SAE identity from the
    authenticated client certificate Common Name, so that experiments can model
    ETSI deployment behavior.
11. As a local test author, I want non-mTLS mode to accept `x-sae-id`, so that
    in-process and plain HTTP tests remain simple.
12. As a security reviewer, I want `x-sae-id` to be accepted only when mTLS is
    disabled, so that header identity cannot override authenticated transport
    identity.
13. As a QKD client developer, I want wrong-slave retrieval attempts to be
    rejected, so that client authorization bugs are caught during experiments.
14. As a QKD client developer, I want wrong-master retrieval attempts to be
    rejected, so that key ownership rules are enforced.
15. As a QKD client developer, I want retrieved ETSI decrypt keys to be consumed
    once, so that clients that incorrectly fetch the same key repeatedly fail
    during testing.
16. As a simulator user, I want issued-but-unconsumed keys to persist with
    SQLite storage, so that experiments can survive short process restarts.
17. As a test author, I want memory storage, so that tests can run quickly and
    discard state after each run.
18. As a simulator maintainer, I want an explicit key-source interface, so that
    key generation can evolve without changing protocol handlers.
19. As a simulator maintainer, I want operating-system randomness to remain the
    current runtime key source, so that simulated QKD material is unpredictable
    without requiring hardware.
20. As a test author, I want future deterministic key sources, so that tests can
    assert exact key lifecycle behavior when needed.
21. As an experiment designer, I want future replayable or fault-injection key
    sources, so that client behavior under unusual KME conditions can be tested.
22. As a simulator maintainer, I want future key-distribution protocols in the
    same binary, so that shared topology, identity, configuration, and storage
    concerns are reused.
23. As an experiment operator, I want protocols to be configured through the
    config module, so that simulator behavior is explicit and reproducible.
24. As a client implementer, I want unsupported ETSI request options to fail
    with protocol-shaped errors, so that unsupported behavior is visible.
25. As a client implementer, I want API errors to follow ETSI 014 where defined,
    so that clients can test realistic error handling.
26. As a simulator maintainer, I want internal infrastructure errors to use
    practical Rust error handling, so that implementation code stays simple
    outside protocol boundaries.
27. As a contributor, I want the crate documentation to say the simulator is not
    production KMS software, so that scope and security expectations are clear.
28. As a contributor, I want protocol behavior tested through the real Axum
    router, so that paths, methods, query/body forms, Serde names, and status
    codes are verified as clients observe them.
29. As a contributor, I want unit tests for topology, storage, config, identity,
    and key-source behavior, so that deep modules can be tested in isolation.
30. As a contributor, I want mTLS behavior covered by integration tests, so that
    certificate identity extraction is verified without slowing normal unit
    test runs.
31. As an `otp-vpn` developer, I want the simulator to interoperate with the
    Rust and Python VPN experiments, so that the VPNs can validate ETSI key
    retrieval workflows locally.
32. As a future protocol developer, I want ETSI-specific storage to remain
    explicitly ETSI-shaped, so that current protocol fidelity is not weakened
    by premature generalization.
33. As a future protocol developer, I want shared simulator concepts to be
    clearly separated from protocol-specific behavior, so that adding a new
    experimental protocol does not require duplicating topology or identity
    logic.
34. As a security reviewer, I want the simulator to reject malformed or invalid
    request data, so that clients cannot accidentally depend on permissive KME
    behavior.
35. As a maintainer, I want verification commands documented, so that future
    agents know how to validate changes consistently.

### Implementation Decisions

- `kms` is an experimental key-management simulator for QKD/KME experiments. It
  is not production KMS software.
- ETSI GS QKD 014 is the first supported protocol module.
- Future experimental key-distribution protocols should live inside the same
  binary and be configured through the configuration module.
- Public protocol modules own routes, request/response shapes, protocol error
  semantics, and protocol lifecycle rules.
- Shared simulator concerns include configuration, topology/security policy,
  caller identity resolution, key-source selection, storage construction, and
  observability.
- The ETSI 014 public API must preserve the protocol's paths, methods, request
  forms, response forms, field names, and status codes for the supported
  subset.
- Rust internals should use idiomatic names. Serde `rename` attributes should
  map internal names to ETSI JSON field names.
- API errors should follow ETSI 014 status codes and error response data models
  where defined.
- Internal setup, storage, and infrastructure errors may use `anyhow` when they
  are not part of a protocol response contract.
- Topology is a security policy boundary. It determines known SAEs, source KME,
  target KME, valid master/slave relationships, and key ownership checks.
- mTLS mode derives caller SAE identity from the authenticated client
  certificate Common Name.
- Plain HTTP mode may use the `x-sae-id` header, but only when mTLS is disabled.
- Protocol handlers should operate on resolved caller identity rather than
  directly trusting arbitrary request headers.
- Key material should be produced through a key-source interface.
- The current runtime key-source implementation is operating-system randomness,
  equivalent to an OS CSPRNG source.
- Future key-source implementations may support deterministic tests, replayable
  experiments, fault injection, or other experimental key-source models.
- ETSI storage should remain specifically shaped around ETSI key storage rather
  than becoming protocol-neutral prematurely.
- ETSI storage records bind `key_ID`, `master_sae_id`, `slave_sae_id`, and key
  bytes.
- `enc_keys` stores generated keys with their original master and slave SAE
  ownership.
- `dec_keys` verifies that the caller slave SAE and path master SAE match the
  stored key ownership.
- Successful `dec_keys` retrieval deletes the returned keys from storage.
- One-time `dec_keys` consumption is a core invariant, both for storage
  correctness and for catching incorrect client behavior.
- SQLite is the default configured durable storage backend.
- Memory storage remains supported for ephemeral runs, unit tests, and local
  development.
- Unsupported ETSI features, such as unsupported mandatory extensions, should
  fail explicitly instead of being silently ignored.
- The simulator should continue to support both GET shortcut forms and POST
  forms where ETSI 014 allows them.

### Testing Decisions

- Good tests should assert externally visible behavior and security invariants,
  not private implementation details.
- Protocol fidelity must be tested through the real Axum router because clients
  observe routes, methods, query/body forms, Serde field names, status codes,
  and response bodies at that boundary.
- Unit tests should cover configuration loading and validation.
- Unit tests should cover topology lookup and SAE authorization decisions.
- Unit tests should cover caller identity resolution behavior where factored
  into reusable code.
- Unit tests should cover key-source behavior, including deterministic fake
  sources when they are introduced.
- Unit tests should cover ETSI storage implementations.
- Unit tests should cover one-time key lifecycle rules.
- Unit tests should cover request validation helpers and protocol error mapping
  when those behaviors are not embedded directly in route handlers.
- Router tests should cover `Get status` topology responses.
- Router tests should cover `enc_keys` issuing base64-encoded key containers.
- Router tests should cover `dec_keys` retrieval by `key_ID`.
- Router tests should cover GET shortcut forms and POST forms.
- Router tests should cover one-time consumption after successful `dec_keys`.
- Router tests should cover unknown caller, unknown master, and unknown slave
  rejection.
- Router tests should cover wrong-master and wrong-slave retrieval rejection.
- Router tests should cover unsupported key sizes and unsupported mandatory
  extensions.
- Router tests should cover ETSI-shaped error status codes and response bodies.
- Integration tests should cover live HTTP server behavior when unit/router
  tests cannot prove the boundary.
- Integration tests should cover mTLS identity extraction from client
  certificates.
- Integration tests should cover SQLite persistence across process restart.
- Integration tests should cover client interoperability with `otp-vpn` and
  other experimental SAE clients.
- Integration tests requiring sockets, certificates, external processes, or
  live servers may be ignored or feature-gated.
- Prior art exists in the current ETSI 014 router tests, which exercise the real
  Axum router and assert topology, issue/retrieve workflows, GET shortcuts,
  validation failures, ownership failures, and one-time consumption.
- Required verification commands are:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Out of Scope

- Production KMS deployment.
- HSM integration.
- Real QKD optical-layer simulation.
- Clustered KME coordination.
- Production-grade secret lifecycle controls beyond what is needed for
  experiments.
- Compliance certification.
- Public API behavior outside the supported ETSI 014 subset.
- NAT traversal, peer discovery, VPN data-plane protocol design, or TUN
  interface management.
- Replacing the `otp-vpn` ETSI client.
- Splitting future protocols into separate crates or binaries.
- Generalizing ETSI storage before another protocol creates a concrete need.

### Further Notes

The ADRs for `kms` define the architectural baseline:

- `kms` is one configurable experimental simulator binary.
- ETSI 014 is the first protocol module.
- The simulator is explicitly non-production.
- Topology is part of the protocol security policy.
- `x-sae-id` is only valid without mTLS.
- One-time `dec_keys` consumption is a core ETSI invariant.
- The testing strategy mirrors the `otp-vpn` crate: focused unit tests plus
  real-boundary router tests and opt-in integration tests.

The first implementation work should preserve current working behavior while
making these boundaries explicit and testable.
