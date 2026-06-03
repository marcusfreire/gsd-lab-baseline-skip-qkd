# Phase 1: Phase 0 Contracts and Scaffolding - Context

**Gathered:** 2026-06-02
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase delivers the Phase 0 baseline for the QKD ETSI 014 mock + SKIP integration project. The deliverable is documentation, a technical plan, risk/acceptance criteria, Docker Compose skeleton, and service directory scaffolding. It must not implement full KME, Key Provider, SKIP, encryptor, Cisco, or IKEv2/RFC8784 logic.

</domain>

<decisions>
## Implementation Decisions

### Document Authority
- **D-01:** After Phase 0, files under `docs/*.md` are the normative project contracts for architecture, APIs, data model, tests, and security assumptions.
- **D-02:** `baseline.md` is historical/original context, not a normative contract after Phase 0.
- **D-03:** Phase 0 should move `baseline.md` to `docs/archive/baseline.md` so the repository makes its historical status explicit.

### Protocol Contract Format
- **D-04:** `docs/api-skip.md` and `docs/api-etsi014-mock.md` must use normative matrices plus JSON examples. Endpoint documentation should include method, path, query/body fields, response fields, error/status behavior, source/authority, and examples.
- **D-05:** `docs/api-skip.md` is strict against `draft-singh-skip-00`.
- **D-06:** `docs/api-etsi014-mock.md` is strict only within the declared minimal ETSI 014 mock profile. It must explicitly list deviations from full ETSI GS QKD 014 conformance.

### Service Scaffolding
- **D-07:** KeyProvider-A and KeyProvider-B must be represented by a single configurable implementation under `services/keyprovider/`.
- **D-08:** `infra/docker-compose.yml` must instantiate the keyprovider implementation separately for A and B using distinct configuration, ports, and SQLite storage paths or volumes.
- **D-09:** Phase 0 service directory bases are `services/kme-mock/`, `services/keyprovider/`, and `services/encryptor-sim/`. The Compose file instantiates A/B roles rather than duplicating source directories.

### SKIP keyId Derivation
- **D-10:** SKIP `keyId` is derived as the first 16 bytes of `HMAC-SHA256(SKIP_KEY_ID_NAMESPACE_SECRET, qkd_key_id | localSystemID | remoteSystemID)`, encoded as lowercase hexadecimal.
- **D-11:** `SKIP_KEY_ID_NAMESPACE_SECRET` is documented as an environment variable shared by KeyProvider-A and KeyProvider-B. It is not QKD key material; it exists to make deterministic SKIP key IDs compatible across providers without exposing the raw `qkd_key_id`.
- **D-12:** The data model must require uniqueness for SKIP `keyId`. If a future implementation detects a collision, it must fail the operation, record/report the error, and never overwrite existing key material.

### Phase 0 Acceptance
- **D-13:** Phase 0 is accepted when required files exist, documentation is internally consistent, scaffold directories exist, and the Docker Compose file is parseable.
- **D-14:** `docs/test-plan.md` must include a manual consistency checklist and may use `docker compose -f infra/docker-compose.yml config` when Docker Compose is available. Phase 0 must not require containers to start successfully or provide working service logic.

### the agent's Discretion
- The agent may choose exact Docker Compose service names, port numbers, placeholder filenames, and doc section ordering if the decisions above remain true.
- The agent may choose how much placeholder code to include in scaffolding, but it must not implement complete service logic in Phase 0.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Scope
- `.planning/PROJECT.md` - Project constraints, out-of-scope boundaries, and key decisions.
- `.planning/REQUIREMENTS.md` - Phase 0 requirement IDs and traceability.
- `.planning/ROADMAP.md` - Phase 1 goal, MVP mode, success criteria, and plan outline.
- `.planning/research/SUMMARY.md` - Research conclusions that shape this phase.

### Historical Source
- `baseline.md` - Original baseline document. Phase 0 should move it to `docs/archive/baseline.md` and treat it as historical context after the move.

### External Protocol References
- `draft-singh-skip-00` - Normative SKIP contract for `docs/api-skip.md`.
- `ETSI GS QKD 014` - Source for the minimal logical KME mock profile and for documenting deviations from full conformance.
- `RFC8784` - Future IKEv2 PPK context only; do not implement real IKEv2/RFC8784 in Phase 0.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `baseline.md`: Contains the original architecture narrative and should be archived rather than rewritten in place.
- `.planning/research/*.md`: Contains protocol, stack, architecture, feature, and pitfall research that should inform the docs.
- `AGENTS.md`: Contains project-level constraints that downstream agents should follow.

### Established Patterns
- The repository is currently planning/documentation-first. There is no implemented service code to preserve.
- GSD planning artifacts are already committed and should remain the source of phase scope.
- Planning documents use Markdown and should remain plain and testable rather than aspirational.

### Integration Points
- New docs belong under `docs/`.
- Historical source belongs under `docs/archive/`.
- Runtime skeleton belongs under `infra/docker-compose.yml`.
- Service scaffolding belongs under `services/kme-mock/`, `services/keyprovider/`, and `services/encryptor-sim/`.

</code_context>

<specifics>
## Specific Ideas

- The SKIP API documentation must avoid a "SKIP-like" shortcut and must explicitly track `draft-singh-skip-00`.
- The ETSI 014 mock documentation must state that the mock is logical and minimal, while still being strict about the declared profile.
- The deterministic `keyId` rule must not imply that key material can be recovered from the ID.
- Phase 0 verification should be possible without a running implementation.

</specifics>

<deferred>
## Deferred Ideas

- Real Cisco or other hardware encryptor integration remains future work.
- Real IKEv2/RFC8784 integration remains future work.
- Full ETSI GS QKD 014 conformance remains future work unless explicitly promoted into a later phase.

</deferred>

---

*Phase: 1-Phase 0 Contracts and Scaffolding*
*Context gathered: 2026-06-02*
