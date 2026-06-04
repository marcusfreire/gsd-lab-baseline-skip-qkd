# ETSI 014 Mock API

## Authority and Scope

The ETSI API baseline follows the supported subset documented by `kms/README.md`
and ADR 0002. Public routes, methods, JSON names, and key lifecycle behavior
must remain faithful to ETSI GS QKD 014 for the supported subset.

This project uses two KME simulator instances:

- `KME-A`
- `KME-B`

Each Key Provider talks only to its local KME.

## Identity Policy

Caller SAE identity is resolved before protocol authorization.

| Mode | Caller SAE source | Rule |
|---|---|---|
| mTLS enabled | client certificate Common Name | Required for mTLS experiments. |
| mTLS disabled | `x-sae-id` header | Local HTTP test mode only. |

`x-sae-id` must be rejected or ignored as an identity source when mTLS is
enabled.

## Endpoints

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/api/v1/keys/{slave_SAE_ID}/status` | Report KME/SAE topology and key limits for a master SAE requesting a slave SAE. |
| `POST` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Full ETSI Get key request form. |
| `GET` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Simple Get key request form with query parameters. |
| `POST` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Full ETSI Get key with key IDs request form. |
| `GET` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Simple decrypt-key request form with `key_ID` query parameter. |

## JSON Names

Public JSON must preserve ETSI names:

- `source_KME_ID`
- `target_KME_ID`
- `master_SAE_ID`
- `slave_SAE_ID`
- `key_ID`
- `key_IDs`
- `additional_slave_SAE_IDs`
- `extension_mandatory`
- `extension_optional`

Internal implementation names may be idiomatic, but serialization must preserve
the public ETSI names.

## Get Status

`KeyProvider-A` calls `KME-A` as `SAE-A`:

```http
GET /api/v1/keys/SAE-B/status
```

In local HTTP mode:

```http
x-sae-id: SAE-A
```

Expected response shape:

```json
{
  "source_KME_ID": "KME-A",
  "target_KME_ID": "KME-B",
  "master_SAE_ID": "SAE-A",
  "slave_SAE_ID": "SAE-B",
  "key_size": 256,
  "stored_key_count": 1000000,
  "max_key_count": 1000000,
  "max_key_per_request": 128,
  "max_key_size": 256,
  "min_key_size": 256,
  "max_SAE_ID_count": 0,
  "status_extension": {}
}
```

## Get Key: `enc_keys`

`KeyProvider-A` calls `KME-A` as `SAE-A`:

```http
POST /api/v1/keys/SAE-B/enc_keys
content-type: application/json
```

```json
{ "number": 1, "size": 256 }
```

Expected response shape:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "<base64>"
    }
  ],
  "key_container_extension": {}
}
```

`enc_keys` must bind the key to:

- `master_sae_id = SAE-A`
- `slave_sae_id = SAE-B`
- `key_ID = QKD-000001`

ETSI keys are base64 strings on the ETSI API.

## Get Key With Key IDs: `dec_keys`

`KeyProvider-B` calls `KME-B` as `SAE-B`:

```http
POST /api/v1/keys/SAE-A/dec_keys
content-type: application/json
```

```json
{
  "key_IDs": [
    { "key_ID": "QKD-000001" }
  ]
}
```

Expected response shape:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "<base64>"
    }
  ],
  "key_container_extension": {}
}
```

`dec_keys` must verify:

- caller slave SAE is `SAE-B`;
- path master SAE is `SAE-A`;
- stored ownership matches `master_sae_id = SAE-A` and `slave_sae_id = SAE-B`;
- `key_ID` exists and is available.

Successful `dec_keys` consumes the key. A second request for the same `key_ID`
must fail.

## Key Source Synchronization

KME-A and KME-B must not share service storage. They may use a
deterministic seed/fake key-source so the same `key_ID` maps to the same 256-bit
key bytes in both KME instances.

## Error Behavior

The implementation should preserve ETSI-shaped status behavior where defined.
At minimum, tests must cover:

- unknown caller SAE rejection;
- unknown slave SAE rejection;
- unknown master SAE rejection;
- wrong slave ownership rejection;
- wrong master ownership rejection;
- unsupported key size rejection;
- unsupported mandatory extension rejection;
- repeated `dec_keys` rejection after one-time consumption.

## Deviations from Full ETSI GS QKD 014

This is a supported subset for local experiments. It does not claim full ETSI
conformance, production KMS readiness, trusted-node routing, physical QKD,
cluster coordination, HSM integration, or operational security hardening.
