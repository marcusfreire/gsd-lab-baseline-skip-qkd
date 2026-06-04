---
quick_id: 260604-etr
slug: revisar-objetivo-md-para-alinhar-arquite
status: complete
created: 2026-06-04T13:40:30Z
---

# Quick Task Plan: Revisar objetivo.md

## Objective

Revisar `objetivo.md` para alinhar o objetivo, arquitetura e texto completo ao
baseline atual: duas instancias `kms/` como KMEs ETSI 014, Key Providers
independentes, SKIP para encryptors simulados e preparacao explicita para
integracao futura com roteadores Cisco.

## Inputs

- `objetivo.md`
- `baseline.md`
- `docs/architecture.md`
- `docs/etsi014-alignment.md`
- `docs/api-etsi014-mock.md`
- `docs/api-skip.md`
- `docs/data-model.md`
- `docs/security-assumptions.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`

## Tasks

1. Remove obsolete generic KME contracts, `qkd_key_id`, `GET_KEY`, and
   `services/mock-kme` guidance.
2. Reframe the objective around the official `kms/` ETSI 014 subset and the
   current `baseline.md`.
3. Document Cisco as a future SKIP/RFC8784 consumer boundary with readiness
   criteria, without claiming real Cisco integration in the baseline.
4. Preserve exact ETSI paths, JSON names, key lifecycle, identity policy,
   storage separation, and SKIP key format rules.

## Verification

- Required-term `rg` checks for KMS, ETSI routes, SKIP fields, Cisco readiness,
  and ADR-driven lifecycle/security requirements.
- Negative `rg` checks for stale generic endpoint names and old identifier
  terminology.
- `git diff --check`.
