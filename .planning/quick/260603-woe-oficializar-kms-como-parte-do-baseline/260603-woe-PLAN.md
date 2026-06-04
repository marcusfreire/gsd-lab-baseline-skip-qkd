---
quick_id: 260603-woe
type: quick
status: in_progress
must_haves:
  truths:
    - README.md states that kms/ is part of the official versioned baseline.
    - docs/architecture.md states that KME-A and KME-B are runtime instances of the committed kms/ simulator.
    - Root Cargo.toml defines a Rust workspace with kms as a member.
    - docs/test-plan.md checks that Cargo.toml and kms/ files are tracked by Git.
    - .planning/PROJECT.md treats kms/ as the official Rust ETSI 014 KME simulator baseline.
  artifacts:
    - Cargo.toml
    - README.md
    - docs/architecture.md
    - docs/api-etsi014-mock.md
    - docs/etsi014-alignment.md
    - docs/test-plan.md
    - docs/security-assumptions.md
    - .planning/PROJECT.md
    - .planning/REQUIREMENTS.md
    - .planning/phases/01-phase-0-contracts-and-scaffolding/01-RESEARCH.md
    - .planning/phases/01-phase-0-contracts-and-scaffolding/01-PATTERNS.md
---

# Quick Task 260603-woe: Oficializar kms como parte do baseline

## Objective

Make the committed `kms/` directory explicit as the official Rust ETSI 014 KME
simulator baseline for KME-A and KME-B.

## Tasks

1. Add a root Rust workspace `Cargo.toml` with `kms` as a member so root-level
   Cargo commands are meaningful.
2. Update project docs to state that `kms/` is official, versioned baseline
   source, not an optional reference or competing scaffold.
3. Update planning source-of-truth artifacts so future agents do not follow the
   older one-KME, `services/kme-mock/`, or HMAC keyId assumptions.
4. Verify that `kms/` source files are tracked by Git and that planning
   decision coverage still passes.

## Verification

- `test -f Cargo.toml && rg -n "members = \\[\"kms\"\\]" Cargo.toml`
- `git ls-files --error-unmatch Cargo.toml kms/Cargo.toml kms/src/routes/etsi014.rs`
- `rg -n "official versioned|part of the official versioned baseline|official KME simulator" README.md docs .planning/PROJECT.md .planning/REQUIREMENTS.md`
- `node .codex/get-shit-done/bin/gsd-tools.cjs query check.decision-coverage-plan .planning/phases/01-phase-0-contracts-and-scaffolding .planning/phases/01-phase-0-contracts-and-scaffolding/01-CONTEXT.md`
- `git diff --check`
