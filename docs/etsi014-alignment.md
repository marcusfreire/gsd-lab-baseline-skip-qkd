# ETSI 014 Alignment

## Purpose

This document records the architecture consensus required by `reavaliar.md`.
It aligns the QKD ETSI 014 + SKIP baseline with the official versioned `kms/`
experimental Key-Management Simulator, ADRs 0001-0005, and the supported ETSI
GS QKD 014 subset before any endpoint or data-model implementation work
proceeds.

This is a design gate. The baseline may scaffold components, but it must not
implement ETSI endpoints, SKIP behavior, provider persistence, or encryptor
logic until this alignment is accepted.

## Authority

Primary local sources:

- `kms/README.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`
- `docs/architecture.md`
- `docs/api-etsi014-mock.md`
- `docs/api-skip.md`
- `docs/data-model.md`
- `docs/security-assumptions.md`

External protocol authority remains ETSI GS QKD 014 for the KME-to-SAE
boundary and `draft-singh-skip-00` for the Key Provider-to-encryptor boundary.
This repository documents only the supported local subset.

## Alignment Decision

The baseline must be a two-KME ETSI 014 experiment, not a single logical KME
mock hidden behind two Key Providers.

```text
                 ETSI GS QKD 014 subset                      SKIP subset

  +-----------------------------+                     +----------------------+
  | KME-A                       |                     | Encryptor-A sim      |
  | kms simulator instance      |                     | localSystemID=Alice  |
  | KME_ID=KME-A                |                     +----------+-----------+
  | connected SAE: SAE-A        |                                |
  | independent KME DB          |                                | GET /key?remoteSystemID=Bob
  +--------------+--------------+                                v
                 ^                                  +-------------+-------------+
                 | status, enc_keys                 | KeyProvider-A             |
                 | caller SAE: SAE-A                | ETSI client identity SAE-A|
                 | peer SAE: SAE-B                  | SKIP server for Alice     |
                 |                                  | independent provider DB   |
                 |                                  +-------------+-------------+
                 |                                                |
                 |                                                | keyId handoff
                 |                                                v
                 |                                  +-------------+-------------+
                 |                                  | KeyProvider-B             |
                 |                                  | ETSI client identity SAE-B|
                 |                                  | SKIP server for Bob       |
                 |                                  | independent provider DB   |
                 |                                  +-------------+-------------+
                 |                                                ^
                 v                                                | GET /key/{keyId}?remoteSystemID=Alice
  +--------------+--------------+                                |
  | KME-B                       |                     +----------+-----------+
  | kms simulator instance      |                     | Encryptor-B sim      |
  | KME_ID=KME-B                |                     | localSystemID=Bob    |
  | connected SAE: SAE-B        |                     +----------------------+
  | independent KME DB          |
  +-----------------------------+

  KME-A and KME-B share only a deterministic fake key-source policy for this
  baseline, so the same ETSI key_ID resolves to identical key bytes on both
  sides. They do not share service storage.
```

## Divergence Review

| Area | Current baseline risk | `kms` / ETSI-aligned resolution |
|---|---|---|
| KME topology | A single central KME hides cross-KME behavior. | Use separate `KME-A` and `KME-B` simulator instances. |
| KME implementation | A project-local simplified KME could drift from `kms`. | Treat the committed `kms/` directory as the official KME simulator source for the baseline. |
| Public ETSI names | Local names can break client fidelity. | Preserve `source_KME_ID`, `target_KME_ID`, `master_SAE_ID`, `slave_SAE_ID`, `key_ID`, and `key_IDs`. |
| Public ETSI routes | Convenience endpoints can become non-interoperable. | Preserve `status`, `enc_keys`, and `dec_keys` routes under `/api/v1/keys`. |
| SAE identity | Trusting request data would weaken the protocol model. | Resolve caller SAE from mTLS Common Name, or from `x-sae-id` only when mTLS is disabled. |
| Topology | Treating topology as display metadata misses authorization bugs. | Treat topology as a security policy boundary. |
| Key lifecycle | Repeated decrypt-key retrieval would violate the ETSI storage invariant. | Successful `dec_keys` consumes the key; repeat retrieval fails. |
| Key storage | Shared storage can mask ownership and synchronization problems. | Use independent KME DBs and independent provider DBs. |
| Key source | Independent random generation cannot prove equal end-to-end PPK material. | Use a deterministic common fake key-source policy for synchronized test material. |
| Quantum modeling | Physical QKD simulation distracts from API fidelity. | Do not use NetSquid, BB84, or quantum-channel modeling in this baseline. |

## Formal Roles

| Role | Baseline component | Responsibility | Non-responsibility |
|---|---|---|---|
| KME | `KME-A`, `KME-B` | Expose the ETSI 014 supported subset, authorize SAE topology, issue and store ETSI key containers, enforce one-time `dec_keys`. | Acts neither as a SKIP server nor as an encryptor. |
| SAE | `SAE-A`, `SAE-B` identities represented by Key Providers | Identify the application endpoint authorized to request or retrieve ETSI key material from a KME. | It is not a separate process in this baseline. |
| Key Provider | `KeyProvider-A`, `KeyProvider-B` | Act as ETSI 014 clients toward local KMEs and SKIP servers toward local simulated encryptors. Persist provider-local mappings/state only. | Must not share a central provider DB or consult the remote KME. |
| Encryptor | `Encryptor-A sim`, `Encryptor-B sim` | Consume SKIP keys and perform simulated `keyId` handoff for future PPK identity flow. | Does not implement Cisco behavior, IPsec, or IKEv2/RFC8784 negotiation. |

## SAE-A and SAE-B Representation

| ETSI identity | Represented by | Local KME | SKIP local system | Peer |
|---|---|---|---|---|
| `SAE-A` | `KeyProvider-A` when it calls ETSI 014 | `KME-A` | `Alice` | `SAE-B` / `Bob` |
| `SAE-B` | `KeyProvider-B` when it calls ETSI 014 | `KME-B` | `Bob` | `SAE-A` / `Alice` |

The simulated encryptors are SKIP clients, not ETSI SAEs. They see
`localSystemID`, `remoteSystemID`, `keyId`, and hex `key`. The Key Providers
bridge those SKIP concepts to ETSI `master_SAE_ID`, `slave_SAE_ID`, `key_ID`,
and base64 `key`.

## ETSI 014 Supported Surface

The KME simulator side must align with ADR 0002 for the supported subset:

| Method | Path | Baseline use |
|---|---|---|
| `GET` | `/api/v1/keys/{slave_SAE_ID}/status` | Topology/status check by the master SAE. |
| `POST` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Full outbound key request. |
| `GET` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Simple outbound key request form. |
| `POST` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Full decrypt-key retrieval by key ID. |
| `GET` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Simple decrypt-key retrieval form. |

Public JSON must use ETSI names at the wire boundary even if implementation
internals use idiomatic names.

## Complete Lifecycle

```text
1. status
   KeyProvider-A as SAE-A -> KME-A
   GET /api/v1/keys/SAE-B/status

2. enc_keys
   KeyProvider-A as SAE-A -> KME-A
   POST /api/v1/keys/SAE-B/enc_keys
   body: { "number": 1, "size": 256 }

3. ETSI key container
   KME-A returns:
   { "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }

4. Provider conversion
   KeyProvider-A converts:
   ETSI base64 -> bytes -> SKIP hex

5. SKIP outbound delivery
   Encryptor-A -> KeyProvider-A
   GET /key?remoteSystemID=Bob
   response: { "keyId": "SKIP-SAE-A-SAE-B-QKD-000001", "key": "<hex>" }

6. key handoff
   Encryptor-A gives keyId to Encryptor-B through simulated handoff.
   This models future RFC8784 PPK_IDENTITY transport only.

7. SKIP inbound lookup
   Encryptor-B -> KeyProvider-B
   GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice

8. dec_keys
   KeyProvider-B maps SKIP keyId -> ETSI key_ID QKD-000001.
   KeyProvider-B as SAE-B -> KME-B
   POST /api/v1/keys/SAE-A/dec_keys
   body: { "key_IDs": [ { "key_ID": "QKD-000001" } ] }

9. ETSI key container
   KME-B returns:
   { "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }

10. Provider conversion and equality
    KeyProvider-B converts:
    ETSI base64 -> bytes -> SKIP hex
    Required invariant: key_hex_alice == key_hex_bob

11. consumption
    Successful dec_keys consumes QKD-000001 on KME-B.
    Repeating dec_keys for the same key_ID must fail.
```

## Ownership Model

ETSI ownership is attached to the key record:

| Field | Required value in A-to-B flow |
|---|---|
| `master_SAE_ID` | `SAE-A` |
| `slave_SAE_ID` | `SAE-B` |
| `key_ID` | Example: `QKD-000001` |
| `key` | ETSI base64 key material |

`enc_keys` establishes ownership from the caller master SAE and the path slave
SAE. `dec_keys` must release key material only when all conditions hold:

- caller SAE is the stored slave SAE;
- path `{master_SAE_ID}` is the stored master SAE;
- requested `key_ID` exists;
- requested `key_ID` has not already been consumed.

Wrong master, wrong slave, unknown caller, unknown peer, unavailable key, or
repeated retrieval must fail before key material is released.

## KME Storage, Key Source, and Simulated Synchronization

Decision:

- `KME-A` has its own KME database.
- `KME-B` has its own KME database.
- `KeyProvider-A` has its own provider database.
- `KeyProvider-B` has its own provider database.
- There is no central provider database.
- There is no shared KME service database.
- For this baseline, KME-A and KME-B may use a common deterministic fake
  key-source policy so a given `key_ID` maps to the same key bytes on both
  sides.

This simulated synchronization is a test fixture for API and lifecycle
alignment. It is not a quantum channel and is not a physical QKD model.

## One-Time `dec_keys`

One-time consumption is mandatory for this baseline because ADR 0004 treats it
as a core ETSI invariant:

```text
Before dec_keys:
  KME-B storage has QKD-000001 available for master=SAE-A, slave=SAE-B.

Successful dec_keys:
  KME-B returns QKD-000001 once to caller SAE-B on path SAE-A.
  KME-B deletes or marks QKD-000001 unavailable.

Repeated dec_keys:
  KME-B rejects QKD-000001 because it is no longer available.
```

Key Providers must not cache this behavior away. Tests must still prove that
the KME-side retrieval is one-time.

## Diagram Updates

The architectural diagram must show:

- two KMEs, not one;
- local-only ETSI access by each Key Provider;
- independent KME and provider persistence;
- simulated key-source synchronization as the only cross-KME coupling;
- SKIP key handoff between simulated encryptors as a future PPK identity stand-in.

```text
KME-A DB      KME-A      <--- ETSI 014 ---      KeyProvider-A DB
   |            ^                                  |
   |            | SAE-A                            | SKIP keyId/key
   |            |                                  v
   |       deterministic                    Encryptor-A sim
   |       fake key-source                         |
   |       synchronization                         | keyId handoff
   |                                               v
   |                                        Encryptor-B sim
   |                                               ^
   |            | SAE-B                            | SKIP keyId/key
   |            |                                  |
KME-B DB      KME-B      <--- ETSI 014 ---      KeyProvider-B DB
```

## Implementation Gate

Endpoint and data-model implementation may start only after this document is
accepted as the architectural consensus for Phase 0/Phase 1 planning.

Acceptance criteria for this gate:

- this document exists as `docs/etsi014-alignment.md`;
- roles for KME, Key Provider, SAE, and Encryptor are formalized;
- SAE-A and SAE-B component representation is explicit;
- the full `status -> enc_keys -> key handoff -> dec_keys -> consumption`
  lifecycle is documented;
- independent KME DBs and independent Key Provider DBs are documented;
- deterministic common key-source policy and simulated synchronization are
  documented;
- ETSI `master_SAE_ID` / `slave_SAE_ID` ownership is documented;
- one-time `dec_keys` behavior is documented;
- diagrams show the corrected two-KME architecture;
- no endpoint or data-model implementation is introduced by this gate.
