# QKD ETSI 014 + SKIP Baseline

## Purpose

This repository defines a local baseline for integrating two ETSI GS QKD 014
KME simulator instances, two independent SKIP Key Providers, and simulated
encryptors for future IKEv2/RFC8784 PPK work.

The KME simulator used by this baseline is the experimental Rust `kms/`
project. The `kms/` directory is part of the official versioned baseline, not
an external reference or optional example. KME-A and KME-B are two configured
instances of this simulator.

The baseline does not model a quantum channel. It models only logical ETSI 014
key delivery through KME behavior that preserves public ETSI routes, methods,
JSON names, SAE identity policy, and one-time decrypt-key lifecycle.

## Corrected Topology

The baseline uses two KME instances:

```text
+--------------------------------------------------------------------------------+
|                      ETSI 014 + SKIP Baseline Architecture                       |
|                                                                                |
|  +------------------+                                  +------------------+     |
|  | KME-A            |                                  | KME-B            |     |
|  | ETSI 014 server  |                                  | ETSI 014 server  |     |
|  | DB KME-A         |                                  | DB KME-B         |     |
|  +--------+---------+                                  +---------+--------+     |
|           ^                                                      ^              |
|           | ETSI 014                                             | ETSI 014     |
|           | caller SAE: SAE-A                                    | caller SAE: SAE-B
|           | GET /api/v1/keys/SAE-B/status                        | POST /api/v1/keys/SAE-A/dec_keys
|           | POST /api/v1/keys/SAE-B/enc_keys                     |              |
|  +--------+---------+                                  +---------+--------+     |
|  | KeyProvider-A    |                                  | KeyProvider-B    |     |
|  | SAE-A client     |                                  | SAE-B client     |     |
|  | SKIP server      |                                  | SKIP server      |     |
|  | DB KP-A          |                                  | DB KP-B          |     |
|  +--------+---------+                                  +---------+--------+     |
|           |                                                      |              |
|           | SKIP                                                 | SKIP         |
|           | GET /key?remoteSystemID=Bob                          | GET /key/{keyId}?remoteSystemID=Alice
|           v                                                      v              |
|  +--------+---------+          keyId handoff           +---------+--------+     |
|  | Encryptor-A sim  | -------------------------------->| Encryptor-B sim  |     |
|  +------------------+                                  +------------------+     |
|                                                                                |
|  Seed/fake key source: synchronized logical QKD material for KME-A and KME-B.  |
|  No NetSquid, no BB84 physical simulation, no quantum link.                    |
+--------------------------------------------------------------------------------+
```

## Components

| Component | Role |
|---|---|
| `kms/` | Official Rust ETSI 014 KME simulator source included in this baseline. |
| `KME-A` | Instance of `kms/` for `SAE-A`; public API is `/api/v1/keys`. |
| `KME-B` | Instance of `kms/` for `SAE-B`; public API is `/api/v1/keys`. |
| `KeyProvider-A` | Independent provider that acts as `SAE-A` toward `KME-A` and as SKIP server toward `Encryptor-A`. |
| `KeyProvider-B` | Independent provider that acts as `SAE-B` toward `KME-B` and as SKIP server toward `Encryptor-B`. |
| `Encryptor-A sim` | First SKIP consumer; asks `KeyProvider-A` for an outbound PPK. |
| `Encryptor-B sim` | First SKIP receiver; asks `KeyProvider-B` for the key identified by the handoff `keyId`. |

Each Key Provider has its own local SQLite state. There is no central provider
database and no shared Key Provider database. KME-A and KME-B may share a
deterministic seed/fake key-source policy so the same ETSI `key_ID` resolves to
identical key bytes on both sides.

## End-to-End Flow

1. `KeyProvider-A`, identified as `SAE-A`, calls
   `GET /api/v1/keys/SAE-B/status` on `KME-A`.
2. `KeyProvider-A` calls `POST /api/v1/keys/SAE-B/enc_keys` on `KME-A` with
   `{ "number": 1, "size": 256 }`.
3. `KME-A` returns `{ "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }`.
4. `KeyProvider-A` converts ETSI base64 key material to bytes, then to SKIP
   hexadecimal.
5. `KeyProvider-A` creates `skip_key_id` as `SKIP-SAE-A-SAE-B-QKD-000001`.
6. `Encryptor-A` calls `GET /key?remoteSystemID=Bob`.
7. `KeyProvider-A` returns `{ "keyId": "SKIP-SAE-A-SAE-B-QKD-000001", "key": "<hex>" }`.
8. `Encryptor-A` gives `keyId` to `Encryptor-B` through simulated handoff. This
   represents the future RFC8784 `PPK_IDENTITY`.
9. `Encryptor-B` calls
   `GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice`.
10. `KeyProvider-B` resolves ETSI `key_ID` `QKD-000001` from the SKIP key ID.
11. `KeyProvider-B`, identified as `SAE-B`, calls
    `POST /api/v1/keys/SAE-A/dec_keys` on `KME-B` with
    `{ "key_IDs": [ { "key_ID": "QKD-000001" } ] }`.
12. `KME-B` returns `{ "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }`.
13. `KeyProvider-B` converts ETSI base64 key material to bytes, then to SKIP
    hexadecimal.
14. `KeyProvider-B` returns
    `{ "keyId": "SKIP-SAE-A-SAE-B-QKD-000001", "key": "<hex>" }`.
15. The end-to-end test must prove `key_hex_alice == key_hex_bob`.

## ETSI 014 Contract

The public ETSI API must preserve these supported routes and methods:

- `GET /api/v1/keys/{slave_SAE_ID}/status`
- `POST /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `GET /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `POST /api/v1/keys/{master_SAE_ID}/dec_keys`
- `GET /api/v1/keys/{master_SAE_ID}/dec_keys`

Public JSON names must preserve ETSI names, including `source_KME_ID`,
`target_KME_ID`, `master_SAE_ID`, `slave_SAE_ID`, `key_ID`, and `key_IDs`.

## Security Boundary

The SAE/KME topology is an authorization policy. In mTLS mode, the caller SAE
identity comes from the client certificate Common Name. In local HTTP mode,
`x-sae-id` may be accepted only when mTLS is disabled.

Successful `dec_keys` retrieval consumes the ETSI key. Repeating `dec_keys` for
the same `key_ID` must fail.

## Using `kms/`

Use `kms/` as the KME simulator crate for both local KME services. From the
repository root, the Rust workspace includes `kms` as a member. The expected
verification path is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

To run only the KME simulator tests:

```bash
cargo test -p kms
```

The Compose scaffold mounts `../kms` into both `kme-a` and `kme-b`, with
separate KME IDs and separate storage paths. Future implementation work should
extend the official `kms/` crate for ETSI 014 simulator behavior rather than
creating a competing `services/kme-mock/` implementation.

## Documentation

- [Architecture](docs/architecture.md)
- [ETSI 014 Alignment](docs/etsi014-alignment.md)
- [ETSI 014 Mock API](docs/api-etsi014-mock.md)
- [SKIP API](docs/api-skip.md)
- [Data Model](docs/data-model.md)
- [Test Plan](docs/test-plan.md)
- [Security Assumptions](docs/security-assumptions.md)

## Out of Scope

- NetSquid.
- BB84 or any physical QKD protocol simulation.
- Quantum channel modeling.
- Central shared Key Provider database.
- Real Cisco encryptor integration.
- Real IKEv2/RFC8784 negotiation.
- Production KMS or production VPN security claims.
