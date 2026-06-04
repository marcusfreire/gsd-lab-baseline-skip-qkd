# Security Assumptions

## Security Posture

This repository is a local experimental baseline. It is not production KMS,
production VPN, or production key-management software.

The committed `kms/` simulator is part of the official baseline and is high
fidelity for the supported ETSI 014 subset, but it does not provide HSM
integration, clustered consistency, compliance controls, or production secret
lifecycle management.

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
