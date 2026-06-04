# Test Plan

## Phase 0 Verification

Phase 0 verifies contracts and scaffolding. It does not require containers to
start, running service logic, or successful end-to-end traffic. The baseline is
accepted when documentation and scaffolds prove the intended shape and future
test obligations.

## Manual Checklist

- Required docs exist: README, architecture, ETSI alignment, ETSI mock API,
  SKIP API, data model, test plan, and security assumptions.
- `infra/docker-compose.yml` exists and declares `kme-a`, `kme-b`,
  `keyprovider-a`, `keyprovider-b`, `encryptor-a-sim`, and
  `encryptor-b-sim`.
- Future service scaffolds exist for Key Provider and simulated encryptors.
- SKIP is pinned to `draft-singh-skip-00`.
- ETSI public JSON names include `source_KME_ID`, `target_KME_ID`,
  `master_SAE_ID`, `slave_SAE_ID`, `key_ID`, and `key_IDs`.
- The topology uses two KME instances with separate KME state.
- KeyProvider-A and KeyProvider-B use separate planned SQLite state with no
  central provider database.
- KME-A and KME-B use deterministic fake key-source synchronization for
  matching logical key bytes by `key_ID`.
- SKIP `keyId` uses `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}`.
- Phase 0 has no full service logic, route handlers, database schemas, or
  working FastAPI entrypoints.

## Documentation Consistency Checks

Run these source checks after documentation/scaffolding changes:

```bash
test -f README.md
test -f docs/architecture.md
test -f docs/etsi014-alignment.md
test -f docs/api-etsi014-mock.md
test -f docs/api-skip.md
test -f docs/data-model.md
test -f docs/test-plan.md
test -f docs/security-assumptions.md
test -f infra/docker-compose.yml
test -f Cargo.toml
test -d kms
test -f kms/Cargo.toml
test -f kms/src/routes/etsi014.rs
git ls-files --error-unmatch Cargo.toml kms/Cargo.toml kms/src/routes/etsi014.rs >/tmp/qkd-skip-kms-files.txt
rg -n "KME-A|KME-B|KeyProvider-A|KeyProvider-B" README.md docs infra/docker-compose.yml
rg -n "official versioned|part of the official versioned baseline|not services/kme-mock|../kms" README.md docs infra/docker-compose.yml
rg -n "master_SAE_ID|slave_SAE_ID|one-time|dec_keys|key handoff|key-source|simulated synchronization" docs/etsi014-alignment.md
rg -n "status|enc_keys|dec_keys" docs/api-etsi014-mock.md
rg -n "source_KME_ID|target_KME_ID|master_SAE_ID|slave_SAE_ID|key_ID|key_IDs" docs/api-etsi014-mock.md docs/data-model.md
rg -n "base64.*bytes.*hex|ETSI base64 -> bytes -> SKIP hex" docs
rg -n "NetSquid|BB84|quantum channel" README.md docs/security-assumptions.md docs/architecture.md
```

## Docker Compose Check

If Docker Compose is available, run:

```bash
docker compose -f infra/docker-compose.yml config
```

Phase 0 does not require containers to start.

## Risk Coverage

The Phase 0 checks must keep these plan and security risks visible:

- Incomplete SKIP conformance against draft-singh-skip-00.
- Accidental centralization of Key Provider state.
- Confusing the ETSI 014 mock with full ETSI GS QKD 014 conformance.

## KMS Simulator Checks

The `kms/` simulator is part of the official versioned baseline. It is
validated with:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

For only the KMS crate:

```bash
cargo test -p kms
```

If the default Cargo target directory is not writable:

```bash
cargo test --target-dir /tmp/quiin-cargo-target -p kms
```

## Future Contract Tests

### ETSI route fidelity

Tests must cover exact methods and paths:

- `GET /api/v1/keys/{slave_SAE_ID}/status`
- `POST /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `GET /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `POST /api/v1/keys/{master_SAE_ID}/dec_keys`
- `GET /api/v1/keys/{master_SAE_ID}/dec_keys`

### ETSI JSON names

Tests must assert public JSON names:

- `source_KME_ID`
- `target_KME_ID`
- `master_SAE_ID`
- `slave_SAE_ID`
- `key_ID`
- `key_IDs`

### SAE identity and topology policy

Tests must prove:

- mTLS mode derives SAE identity from client certificate Common Name;
- local HTTP mode may use `x-sae-id`;
- `x-sae-id` is accepted only when mTLS is disabled;
- unknown caller SAE is rejected;
- unknown master SAE is rejected;
- unknown slave SAE is rejected;
- wrong slave ownership is rejected;
- wrong master ownership is rejected.

### One-time `dec_keys`

Tests must prove:

1. `enc_keys` stores or emits a key with `master_sae_id=SAE-A` and
   `slave_sae_id=SAE-B`.
2. `dec_keys` by `SAE-B` for path `SAE-A` succeeds once.
3. Repeating `dec_keys` with the same `key_ID` fails.

### Base64 to hex conversion

Tests must prove:

```text
ETSI base64 -> bytes -> SKIP hex
```

The expected output is that `KeyProvider-A` and `KeyProvider-B` return the same
hex key for the same synchronized logical ETSI `key_ID`.

## Future End-to-End Tests

### End-to-end SKIP flow

The end-to-end test must run:

1. `Encryptor-A` calls `GET /key?remoteSystemID=Bob`.
2. `KeyProvider-A` calls `KME-A` as `SAE-A` and obtains `QKD-000001`.
3. `KeyProvider-A` returns `SKIP-SAE-A-SAE-B-QKD-000001` and `key_hex_alice`.
4. Simulated handoff sends `keyId` to `Encryptor-B`.
5. `Encryptor-B` calls
   `GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice`.
6. `KeyProvider-B` calls `KME-B` as `SAE-B` and retrieves `QKD-000001`.
7. `KeyProvider-B` returns `key_hex_bob`.
8. Assert `key_hex_alice == key_hex_bob`.

## Out-of-Scope Tests

Do not add tests for:

- NetSquid;
- BB84 physical simulation;
- quantum channel behavior;
- Cisco encryptor integration;
- real IKEv2/RFC8784 negotiation.
