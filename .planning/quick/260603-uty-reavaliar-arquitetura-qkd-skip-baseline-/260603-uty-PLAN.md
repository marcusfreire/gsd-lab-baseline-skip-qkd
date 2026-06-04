---
quick_id: 260603-uty
type: quick
status: in_progress
must_haves:
  truths:
    - README and docs describe two KME instances, KME-A and KME-B.
    - ETSI API docs preserve status, enc_keys, dec_keys, and ETSI JSON names.
    - Key Providers are documented as SAE-A and SAE-B toward their local KME.
    - Test plan covers e2e SKIP PPK equality, one-time dec_keys, ownership rejection, JSON name fidelity, and base64-to-hex conversion.
    - Compose declares KME-A, KME-B, KeyProvider-A, KeyProvider-B, and simulated encryptors without central provider DB.
  artifacts:
    - README.md
    - docs/architecture.md
    - docs/api-etsi014-mock.md
    - docs/api-skip.md
    - docs/data-model.md
    - docs/test-plan.md
    - docs/security-assumptions.md
    - infra/docker-compose.yml
    - AGENTS.md
---

# Quick Task 260603-uty: Reavaliar arquitetura qkd-skip-baseline

## Objective

Execute `reavaliar.md` by updating the repository documentation and local runtime
shape to align with `kms/README.md`, ADRs 0001-0005, and
`AES_VPN_Prototypes.md`.

## Tasks

1. Replace the old generic/single-KME framing with a two-KME topology:
   `KME-A`, `KME-B`, `KeyProvider-A`, `KeyProvider-B`, `Encryptor-A sim`, and
   `Encryptor-B sim`.
2. Document ETSI GS QKD 014 public route and JSON fidelity for `status`,
   `enc_keys`, `dec_keys`, `source_KME_ID`, `target_KME_ID`, `master_SAE_ID`,
   `slave_SAE_ID`, `key_ID`, and `key_IDs`.
3. Document lifecycle and security invariants: SAE identity from mTLS CN or
   local `x-sae-id`, topology authorization, ownership checks, one-time
   `dec_keys`, base64 ETSI keys, hex SKIP keys, and key fingerprints for logs.
4. Update Docker Compose scaffold to show KME-A/KME-B, independent Key
   Providers, separate volumes, and deterministic fake key-source synchronization.
5. Run source checks and Compose config validation when Docker Compose is
   available.

## Verification

- `rg -n "KME-A|KME-B|KeyProvider-A|KeyProvider-B" README.md docs infra/docker-compose.yml AGENTS.md`
- `rg -n "status|enc_keys|dec_keys" docs/api-etsi014-mock.md`
- `rg -n "source_KME_ID|target_KME_ID|master_SAE_ID|slave_SAE_ID|key_ID|key_IDs" docs/api-etsi014-mock.md docs/data-model.md`
- `rg -n "base64.*bytes.*hex|ETSI base64 -> bytes -> SKIP hex" docs`
- `docker compose -f infra/docker-compose.yml config` when Docker Compose is installed.
