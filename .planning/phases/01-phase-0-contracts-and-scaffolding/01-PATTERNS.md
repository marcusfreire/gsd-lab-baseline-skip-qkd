# Phase 1 Pattern Map: Phase 0 Contracts and Scaffolding

**Phase:** 1 - Phase 0 Contracts and Scaffolding
**Generated:** 2026-06-02
**Status:** Superseded by 2026-06-04 architecture rebaseline

## Supersession Notice

This pattern map predates the architecture reassessment that made `kms/` part
of the official versioned baseline.

Current executor guidance:

- KME simulator code lives in the committed `kms/` directory.
- `KME-A` and `KME-B` are separate runtime instances of `kms/`.
- Do not create `services/kme-mock/` as a competing KME simulator source.
- Future service scaffolds are `services/keyprovider/` and
  `services/encryptor-sim/`.
- SKIP `keyId` mapping is textual:
  `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}`.

## Summary

There is no implemented service codebase yet. This phase is a documentation and scaffolding baseline, so the closest reusable patterns are existing planning documents, `baseline.md`, `AGENTS.md`, and GSD artifact conventions.

## Files to Create or Modify

| File or Path | Role | Closest Existing Analog | Notes |
|--------------|------|-------------------------|-------|
| `README.md` | Human entry point | `baseline.md`, `.planning/PROJECT.md` | Must describe purpose, scope, local topology, and Phase 0 status. |
| `docs/archive/baseline.md` | Historical source | `baseline.md` | Move existing file here; docs become normative after Phase 0. |
| `docs/architecture.md` | Architecture contract | `baseline.md`, `.planning/research/ARCHITECTURE.md` | Must define KME mock, KeyProvider-A/B, encryptor sims, and future IKE boundary. |
| `docs/api-etsi014-mock.md` | ETSI mock contract | `.planning/research/STACK.md`, `baseline.md` | Must define strict minimal profile and deviations from full ETSI 014. |
| `docs/api-skip.md` | SKIP contract | `.planning/research/FEATURES.md`, `01-RESEARCH.md` | Must be strict against `draft-singh-skip-00`. |
| `docs/data-model.md` | Data model contract | `.planning/PROJECT.md`, `01-CONTEXT.md` | Must separate `qkd_key_id`, ETSI `key_ID`, SKIP `keyId`, key material, and provider state. |
| `docs/test-plan.md` | Verification plan | `.planning/REQUIREMENTS.md`, `01-RESEARCH.md` | Must include manual checklist and optional Compose parse command. |
| `docs/security-assumptions.md` | Security boundary | `.planning/research/PITFALLS.md`, `01-CONTEXT.md` | Must state local-only, non-production, simulated key assumptions. |
| `infra/docker-compose.yml` | Runtime skeleton | none | Must be parseable and instantiate A/B services without requiring full logic. |
| `services/kme-mock/` | Service scaffold | none | Placeholder only. |
| `services/keyprovider/` | Shared provider scaffold | none | One configurable provider implementation for A/B. |
| `services/encryptor-sim/` | Simulated encryptor scaffold | none | One configurable encryptor simulation base. |

## Established Patterns

- Planning documents use Markdown with concrete, testable assertions.
- GSD files use phase-prefixed artifact names under `.planning/phases/01-phase-0-contracts-and-scaffolding/`.
- Existing project constraints live in `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, and `AGENTS.md`.
- Historical domain narrative currently lives in `baseline.md` and should be moved, not duplicated as the normative source.

## Implementation Notes for Plans

- Avoid modifying `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, or `.planning/ROADMAP.md` during execution unless the plan explicitly needs a planning-state update.
- Do not implement full FastAPI service logic in Phase 0.
- Each plan should include a `<threat_model>` block because security enforcement is enabled.
- Each plan should list generated file paths under "Artifacts this phase produces".

## PATTERN MAPPING COMPLETE
