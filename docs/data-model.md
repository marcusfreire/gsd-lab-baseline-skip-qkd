# Data Model

## Identifier Domains

| Identifier | Domain | Example | Notes |
|---|---|---|---|
| KME ID | ETSI topology | `KME-A`, `KME-B` | Identifies each KME simulator instance. |
| SAE ID | ETSI caller/peer identity | `SAE-A`, `SAE-B` | Used for mTLS Common Name or local `x-sae-id`. |
| ETSI `key_ID` | ETSI key lifecycle | `QKD-000001` | Public ETSI JSON name; maps to stored KME key record. |
| SKIP `keyId` | SKIP handoff | `SKIP-SAE-A-SAE-B-QKD-000001` | Deterministic identifier exposed to encryptors. |
| SKIP system ID | SKIP peer label | `Alice`, `Bob` | `remoteSystemID` values used by simulated encryptors. |

Do not collapse these identifiers into one untyped field.

## Key Material Formats

| Boundary | Field | Encoding |
|---|---|---|
| ETSI 014 | `key` | Base64 string. |
| Key Provider internal | key bytes | Raw bytes after base64 decode. |
| SKIP | `key` | Hexadecimal string. |

Required conversion:

```text
ETSI base64 -> bytes -> SKIP hex
```

Tests must prove this conversion is exact.

## ETSI Key Record

Each KME stores ETSI key records with:

- `key_ID`
- `master_sae_id`
- `slave_sae_id`
- key bytes
- availability/consumption state

`enc_keys` creates or emits records bound to the caller master SAE and path
slave SAE.

`dec_keys` verifies stored ownership before returning key material:

- caller SAE must equal `slave_sae_id`;
- path SAE must equal `master_sae_id`;
- key must still be available.

Successful `dec_keys` consumes the key by deleting it or marking it
unavailable. Repeating `dec_keys` with the same `key_ID` must fail.

## KME State

KME state is per instance:

- `KME-A` owns `KME-A` storage.
- `KME-B` owns `KME-B` storage.

KME-A and KME-B must not rely on shared KME storage for normal operation.
They may share a deterministic seed/fake key-source policy so the same
`key_ID` maps to identical key bytes in both KME instances.

## Provider State

Key Provider state is per provider:

- `KeyProvider-A` owns its local provider DB.
- `KeyProvider-B` owns its local provider DB.

There is no central shared database between Key Providers.

Future provider tables may include:

- `skip_keys`
- `etsi_key_mappings`
- `key_requests`
- `provider_events`

`skip_keys` should store:

- `skip_key_id`
- `key_ID`
- `master_sae_id`
- `slave_sae_id`
- `localSystemID`
- `remoteSystemID`
- fingerprint, not full key in logs
- lifecycle status

## Deterministic SKIP Mapping

The required mapping is:

```text
skip_key_id = "SKIP-" + master_SAE_ID + "-" + slave_SAE_ID + "-" + key_ID
```

Example:

```text
key_ID      = QKD-000001
master SAE  = SAE-A
slave SAE   = SAE-B
skip_key_id = SKIP-SAE-A-SAE-B-QKD-000001
```

Reverse mapping for the responder side:

```text
SKIP-SAE-A-SAE-B-QKD-000001 -> QKD-000001
```

The SKIP key ID must not reveal key material.

## Logging

Never log complete key material. Use a fingerprint such as a truncated hash:

```text
key_fingerprint = first_12_hex_chars(SHA-256(key_bytes))
```

The fingerprint is for debugging and audit correlation only; it is not a key
identifier in the ETSI or SKIP public APIs.
