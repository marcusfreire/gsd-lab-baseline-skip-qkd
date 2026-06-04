# Security Assumptions

## Security Posture

This repository is a local experimental baseline. It is not production KMS,
production VPN, or production key-management software. It is not
production-ready.

The committed `kms/` simulator is part of the official baseline and is high
fidelity for the supported ETSI 014 subset, but it does not provide HSM
integration, clustered consistency, compliance controls, or production secret
lifecycle management.

## Assumptions

- Key material is simulated and local to this lab baseline.
- Local HTTP is allowed only as a scaffold simplification and test mode.
- TLS, mTLS, and authentication hardening are deferred unless a later phase
  implements and verifies them.
- KeyProvider-A and KeyProvider-B remain independent and use separate provider
  state.

## Non-Production Limits

This baseline does not claim production-safe key handling, production entropy,
production TLS configuration, production authentication, production
authorization, or full ETSI GS QKD 014 conformance.

## Topology Is Authorization

SAE/KME topology is a security policy boundary.

Protocol handlers must reject:

- unknown caller SAE;
- unknown requested peer SAE;
- mismatched master/slave ownership;
- `dec_keys` requests where caller slave SAE does not match stored
  `slave_sae_id`;
- `dec_keys` requests where path master SAE does not match stored
  `master_sae_id`.

## Identity Source

| Mode | Valid identity source |
|---|---|
| mTLS enabled | client certificate Common Name |
| mTLS disabled | `x-sae-id` local test header |

`x-sae-id` must not override mTLS identity. It is acceptable only when mTLS is
disabled for local HTTP testing.

## Key Lifecycle

`enc_keys` creates or emits keys with ownership metadata.

`dec_keys` consumes keys after successful retrieval. Repeating `dec_keys` with
the same `key_ID` must fail.

## Key Material Handling

- ETSI API key material is base64.
- SKIP API key material is hexadecimal.
- Do not log complete key material.
- Use fingerprints for diagnostics, e.g. truncated SHA-256 of key bytes.
- Do not treat fingerprints as secrets or as protocol identifiers.

## Sensitive Values

Sensitive values include ETSI `key`, SKIP `key`, raw key bytes, and any future
provider persistence fields that contain key material. Full key material must
never be logged. Future logs use fingerprints and identifiers only.

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Incomplete SKIP conformance against draft-singh-skip-00 | Keep `docs/api-skip.md` pinned to the draft and verify endpoint names, fields, status codes, examples, and entropy behavior. |
| Accidental centralization of Key Provider state | Require separate provider SQLite paths or volumes and prohibit central provider databases. |
| Confusing the ETSI 014 mock with full ETSI GS QKD 014 conformance | Document the minimal logical key-delivery profile and list deviations from full ETSI 014. |
| Raw key material in logs | Never log full key material; use fingerprints and short identifiers. |
| Simulated entropy mistaken for production entropy | Treat entropy endpoints as contract scaffolding until implemented and tested; do not claim production entropy. |
| Improper SAE identity source | Use certificate Common Name in mTLS mode and allow `x-sae-id` only when mTLS is disabled. |

## KME Synchronization

KME-A and KME-B use separate storage. The baseline may use a seed/fake
key-source to ensure both sides have the same bytes for the same `key_ID`.

This is logical key-source synchronization. It is not a quantum channel, not
BB84, not NetSquid, and not physical QKD simulation.

## Deferred Hardening

Future phases may add:

- certificate generation helpers;
- strict mTLS compose profiles;
- provider-side authentication;
- structured redaction middleware;
- production-like secret storage experiments;
- RFC8784 PPK integration tests.

These are not Phase 0 production guarantees.

## Claims We Do Not Make

- We do not claim production security.
- We do not claim full ETSI GS QKD 014 conformance.
- We do not claim real QKD, BB84, NetSquid, QBER, reconciliation, privacy
  amplification, or quantum-channel behavior.
- We do not claim real Cisco, IPsec, or IKEv2/RFC8784 integration.
