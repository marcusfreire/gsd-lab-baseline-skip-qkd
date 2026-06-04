# Architecture

## Authority

This architecture is based on:

- `reavaliar.md`
- `kms/README.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`
- `AES_VPN_Prototypes.md`

The `kms/` Rust project is the KME simulator for this baseline. The `kms/`
directory is part of the official versioned baseline and is the source used to
instantiate KME-A and KME-B. It is not production KMS software.

The formal ETSI 014 alignment gate is documented in
`docs/etsi014-alignment.md`. It records the consensus required before endpoint
or data-model implementation proceeds.

## Scope

The baseline models application-facing ETSI GS QKD 014 key delivery and SKIP
key delivery to simulated encryptors. It does not model a quantum channel,
BB84, NetSquid, QBER, reconciliation, privacy amplification, or optical-layer
QKD behavior.

## Corrected Topology

The architecture has two separate KME simulator instances:

- `KME-A` owns the local KME endpoint for `SAE-A`.
- `KME-B` owns the local KME endpoint for `SAE-B`.
- `KeyProvider-A` acts as `SAE-A` toward `KME-A`.
- `KeyProvider-B` acts as `SAE-B` toward `KME-B`.
- `KeyProvider-A` consults only `KME-A`.
- `KeyProvider-B` consults only `KME-B`.
- `KeyProvider-A` and `KeyProvider-B` have separate provider databases.

```text
KME-A <--- ETSI 014 as SAE-A --- KeyProvider-A <--- SKIP --- Encryptor-A
  |                                                               |
  | synchronized fake key-source by key_ID                        | keyId handoff
  v                                                               v
KME-B <--- ETSI 014 as SAE-B --- KeyProvider-B <--- SKIP --- Encryptor-B
```

## Components

### KME-A and KME-B

`KME-A` and `KME-B` are separate runtime instances of the official `kms/`
simulator crate included in this repository. Each exposes the ETSI 014 subset
under `/api/v1/keys`:

- `GET /api/v1/keys/{slave_SAE_ID}/status`
- `POST /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `GET /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `POST /api/v1/keys/{master_SAE_ID}/dec_keys`
- `GET /api/v1/keys/{master_SAE_ID}/dec_keys`

Each KME has its own storage. For the paired baseline, a deterministic
seed/fake key-source must make both KME instances know the same logical key
bytes for the same ETSI `key_ID`. Future ETSI 014 simulator work belongs in
`kms/`, not in a competing `services/kme-mock/` tree.

### KeyProvider-A

`KeyProvider-A` is an independent service with local provider state. It acts as:

- `SAE-A` when calling `KME-A`;
- a SKIP server for `Encryptor-A`;
- the outbound side that requests ETSI `enc_keys` for `SAE-B`.

### KeyProvider-B

`KeyProvider-B` is an independent service with local provider state. It acts as:

- `SAE-B` when calling `KME-B`;
- a SKIP server for `Encryptor-B`;
- the inbound side that resolves a SKIP `keyId` back to ETSI `key_ID` and calls
  `dec_keys` for `SAE-A`.

## SAE/KME Authorization Policy

Topology is a protocol security policy boundary.

- In mTLS mode, caller SAE identity comes from the client certificate Common
  Name.
- In local HTTP mode, caller SAE identity may come from `x-sae-id`.
- `x-sae-id` is valid only when mTLS is disabled.
- Unknown caller SAE, unknown peer SAE, wrong master, wrong slave, or wrong
  ownership must be rejected before key material is released.

`enc_keys` must store or emit key records with ownership metadata:

- `master_sae_id`
- `slave_sae_id`
- `key_ID`
- key bytes

`dec_keys` must verify:

- caller slave SAE equals stored `slave_sae_id`;
- path master SAE equals stored `master_sae_id`;
- requested `key_ID` exists and is still available.

Successful `dec_keys` consumes the key by deleting it or marking it unavailable.
Repeating `dec_keys` with the same `key_ID` must fail.

## Data Flow

1. `KeyProvider-A` uses identity `SAE-A` and calls
   `GET /api/v1/keys/SAE-B/status` on `KME-A`.
2. `KeyProvider-A` calls `POST /api/v1/keys/SAE-B/enc_keys` on `KME-A` with
   `{ "number": 1, "size": 256 }`.
3. `KME-A` returns ETSI base64 key material and `key_ID`, for example
   `QKD-000001`.
4. `KeyProvider-A` decodes base64 to bytes and encodes bytes as hex for SKIP.
5. `KeyProvider-A` exposes `keyId` `SKIP-SAE-A-SAE-B-QKD-000001` through
   `GET /key?remoteSystemID=Bob`.
6. `Encryptor-A` hands the `keyId` to `Encryptor-B` as simulated future
   RFC8784 PPK identity exchange.
7. `Encryptor-B` calls
   `GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice`.
8. `KeyProvider-B` maps the SKIP `keyId` back to ETSI `key_ID` `QKD-000001`.
9. `KeyProvider-B` uses identity `SAE-B` and calls
   `POST /api/v1/keys/SAE-A/dec_keys` on `KME-B` with
   `{ "key_IDs": [ { "key_ID": "QKD-000001" } ] }`.
10. `KME-B` returns the same base64 key bytes for `QKD-000001`.
11. `KeyProvider-B` decodes base64 to bytes and encodes bytes as hex for SKIP.
12. The e2e invariant is `key_hex_alice == key_hex_bob`.

## Persistence Boundary

There is no central provider database shared between Key Providers.

- `KME-A` storage belongs to `KME-A`.
- `KME-B` storage belongs to `KME-B`.
- `KeyProvider-A` SQLite state belongs to `KeyProvider-A`.
- `KeyProvider-B` SQLite state belongs to `KeyProvider-B`.

The only cross-side synchronization in the baseline is the deterministic
seed/fake key-source policy that makes the two KME instances produce or load
the same key bytes for the same `key_ID`.

## Runtime Scaffold

The Phase 0 scaffold is declared in `infra/docker-compose.yml` with six local
services:

- `kme-a`
- `kme-b`
- `keyprovider-a`
- `keyprovider-b`
- `encryptor-a-sim`
- `encryptor-b-sim`

The `kme-a` and `kme-b` services reference `kms/` as their simulator source.
The Key Provider services use separate planned SQLite paths or volumes. The
Compose commands are placeholders and do not implement API logic.

## Future Boundary

RFC8784 remains a future consumer boundary. The SKIP `keyId` handoff represents
future `PPK_IDENTITY`, but this baseline does not implement IKEv2, IPsec, Cisco
integration, or production PPK negotiation.
