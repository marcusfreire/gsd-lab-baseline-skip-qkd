# SKIP API

## Authority

SKIP behavior is documented against `draft-singh-skip-00` for the Key
Provider-to-encryptor boundary. This baseline uses simulated encryptors first.
Real Cisco integration is future work.

## Role

Each Key Provider is both:

- an ETSI 014 SAE client toward its local KME;
- a SKIP server toward its local simulated encryptor.

`KeyProvider-A` acts as `SAE-A` toward `KME-A`. `KeyProvider-B` acts as `SAE-B`
toward `KME-B`.

## Endpoints

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/capabilities` | Report SKIP service capabilities. |
| `GET` | `/key?remoteSystemID={id}` | Return a fresh outbound key for a remote system. |
| `GET` | `/key?remoteSystemID={id}&size={bits}` | Return a fresh outbound key with requested size. |
| `GET` | `/key/{keyId}?remoteSystemID={id}` | Return key material for an existing SKIP key ID. |
| `GET` | `/entropy` | Return entropy from the provider. |
| `GET` | `/entropy?minentropy={bits}` | Return requested entropy length. |

## Field Rules

| Field | Representation |
|---|---|
| `localSystemID` | SKIP local system label, e.g. `Alice` or `Bob`. |
| `remoteSystemID` | SKIP remote system label, e.g. `Bob` or `Alice`. |
| `keyId` | Deterministic SKIP identifier, e.g. `SKIP-SAE-A-SAE-B-QKD-000001`. |
| `key` | Hexadecimal string derived from ETSI base64 key material. |
| `entropy` | Entropy response value for the SKIP entropy endpoint. |
| `size` | Requested key size in bits; baseline uses 256. |

ETSI `key` values are base64. SKIP `key` values are hexadecimal.
The default key material size for this baseline is 256 bits.

## Outbound Key Request

`Encryptor-A` calls `KeyProvider-A`:

```http
GET /key?remoteSystemID=Bob
```

`KeyProvider-A` obtains ETSI key material through:

```http
POST /api/v1/keys/SAE-B/enc_keys
```

The returned ETSI container is:

```json
{ "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }
```

`KeyProvider-A` must:

1. decode `<base64>` to bytes;
2. encode bytes to lowercase hexadecimal;
3. create `keyId` as `SKIP-SAE-A-SAE-B-QKD-000001`;
4. return:

```json
{
  "keyId": "SKIP-SAE-A-SAE-B-QKD-000001",
  "key": "<hex>"
}
```

## Inbound Key Request

`Encryptor-B` receives the SKIP `keyId` through simulated handoff and calls
`KeyProvider-B`:

```http
GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice
```

`KeyProvider-B` must map:

```text
SKIP-SAE-A-SAE-B-QKD-000001 -> QKD-000001
```

Then it calls `KME-B` as `SAE-B`:

```http
POST /api/v1/keys/SAE-A/dec_keys
```

```json
{
  "key_IDs": [
    { "key_ID": "QKD-000001" }
  ]
}
```

`KeyProvider-B` decodes ETSI base64 to bytes, encodes bytes to hex, and returns:

```json
{
  "keyId": "SKIP-SAE-A-SAE-B-QKD-000001",
  "key": "<hex>"
}
```

The end-to-end test must prove `key_hex_alice == key_hex_bob`.

## Deterministic `keyId`

The Phase 0 reassessment replaces earlier opaque/HMAC-style mapping with a
protocol-debuggable deterministic identifier:

```text
SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}
```

For the required flow:

```text
SKIP-SAE-A-SAE-B-QKD-000001
```

The `keyId` is an identifier only. It must not contain or reveal key material.

## Status Codes

The SKIP service should use:

- `200` for successful responses;
- `400` for malformed requests or unsupported requested size;
- `404` when a `keyId` is unknown or unavailable;
- `405` for unsupported HTTP methods;
- `500` only for unexpected server failures.

## Entropy Behavior

`GET /entropy` and `GET /entropy?minentropy={bits}` are documented as SKIP
contract endpoints for future Key Provider services. Phase 0 records the route
and JSON field contract only; implementation must not claim production entropy.
Responses use an `entropy` field and must honor `minentropy` only within the
limits documented by the future implementation.

## Phase 0 Limits

The SKIP service is documented for simulated encryptors. It does not implement
real Cisco integration, real IKEv2/RFC8784 negotiation, or production key
management.
