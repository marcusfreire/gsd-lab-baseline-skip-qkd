---
quick_id: 260603-vfo
type: quick
status: in_progress
must_haves:
  truths:
    - docs/etsi014-alignment.md exists and answers every task in reavaliar.md.
    - The document formalizes KME, Key Provider, SAE, and Encryptor roles.
    - The document identifies SAE-A and SAE-B component representation.
    - The document defines the full status -> enc_keys -> key handoff -> dec_keys -> key consumption lifecycle.
    - The document states independent KME databases, deterministic common key-source policy, and simulated synchronization.
    - The document defines ETSI master_SAE_ID/slave_SAE_ID ownership and one-time dec_keys behavior.
    - README/test plan link or verify the new alignment document.
  artifacts:
    - docs/etsi014-alignment.md
    - README.md
    - docs/architecture.md
    - docs/test-plan.md
    - .planning/phases/01-phase-0-contracts-and-scaffolding/01-CONTEXT.md
---

# Quick Task 260603-vfo: Execute reavaliar.md

## Objective

Execute `reavaliar.md` by producing the missing formal ETSI 014 alignment
document before any endpoint or data-model implementation work.

## Tasks

1. Create `docs/etsi014-alignment.md` with the architectural re-evaluation
   against `kms/README.md`, ADRs 0001-0005, and the current baseline docs.
2. Include diagrams showing the two-KME topology and the lifecycle from
   `status` through one-time `dec_keys` consumption.
3. Formalize roles, SAE-A/SAE-B component mapping, storage/key-source choices,
   ETSI ownership, divergences, and acceptance criteria.
4. Link the new document from `README.md` and add Phase 0 checks in
   `docs/test-plan.md`.

## Verification

- `test -f docs/etsi014-alignment.md`
- `rg -n "KME-A|KME-B|KeyProvider-A|KeyProvider-B|SAE-A|SAE-B|master_SAE_ID|slave_SAE_ID|one-time|dec_keys|key handoff|simulated synchronization|key-source" docs/etsi014-alignment.md`
- `rg -n "etsi014-alignment" README.md docs/test-plan.md`
- `git diff --check`
