---
quick_id: 260604-ehq
slug: revisar-baseline-md-conforme-adrs-0001-0
status: complete
created: 2026-06-04T13:26:05Z
---

# Quick Task Plan: Revisar baseline.md conforme ADRs do kms

## Objective

Revisar `baseline.md` para deixar de ser um documento antigo com rotas
genéricas e transformá-lo em uma descrição fiel do subconjunto ETSI GS QKD 014
adotado pelo KMS Experimental Key-Management Simulator em `kms/`.

## Inputs

- `baseline.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`
- `docs/etsi014-alignment.md`
- `docs/api-etsi014-mock.md`
- `docs/api-skip.md`
- `docs/data-model.md`

## Tasks

1. Replace old generic KME/SKIP language with the `kms/`-aligned two-KME
   baseline.
2. Preserve ETSI public routes and JSON names exactly for the supported subset.
3. Document SAE identity, topology authorization, key-source/storage lifecycle,
   one-time `dec_keys`, and test/fidelity requirements from ADRs 0001-0005.
4. Verify `baseline.md` no longer advertises generic routes such as
   `/api/v1/keys/get_key` or `GET_KEY_WITH_KEY_IDS`.

## Verification

- `rg` required ETSI route and JSON names in `baseline.md`.
- `rg` ADR-driven concepts: `Common Name`, `x-sae-id`, `one-time`,
  `dec_keys`, `SQLite`, `cargo test --workspace`.
- Negative check for stale generic route names.
