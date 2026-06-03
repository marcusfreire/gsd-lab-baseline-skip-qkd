# Phase 1: Phase 0 Contracts and Scaffolding - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-02
**Phase:** 1-Phase 0 Contracts and Scaffolding
**Areas discussed:** Autoridade dos documentos, Formato dos contratos ETSI/SKIP, Estrutura dos servicos, Regra exata de keyId, Criterio de aceite da Fase 0

---

## Autoridade dos documentos

| Option | Description | Selected |
|--------|-------------|----------|
| `docs/*.md` normativos; `baseline.md` historico | New docs become the normative contract; `baseline.md` remains original context. | x |
| `baseline.md` continua canonico; `docs/*.md` detalham | Keeps the original document authoritative but risks divergence. | |
| `README.md` como entrada canonica; docs detalham; `baseline.md` arquivado | Good human entry point, but README may become too large if normative. | |
| Outro | Freeform alternative. | |

**User's choice:** `docs/*.md` normativos; `baseline.md` historico.
**Notes:** User also chose to move `baseline.md` to `docs/archive/baseline.md`.

---

## Formato dos contratos ETSI/SKIP

| Option | Description | Selected |
|--------|-------------|----------|
| Matriz normativa + exemplos | Tables by endpoint with method/path/fields/status/source plus JSON examples. | x |
| Narrativo com exemplos | Easier to read, weaker for future contract tests. | |
| OpenAPI-first + explicacao curta | Strong for future implementation but may over-commit Phase 0. | |
| Outro | Freeform alternative. | |

**User's choice:** Matriz normativa + exemplos.
**Notes:** User clarified that SKIP is strict against `draft-singh-skip-00`; ETSI mock is strict only inside the declared minimal profile and must list deviations from full ETSI 014.

---

## Estrutura dos servicos

| Option | Description | Selected |
|--------|-------------|----------|
| Um `services/keyprovider/` configuravel, instanciado como A/B no Compose | Avoids duplicated provider code; independence comes from config/volumes/ports. | x |
| Diretorios separados `services/keyprovider-a/` e `services/keyprovider-b/` | Visually explicit, but favors divergence. | |
| Um diretorio base + overlays A/B | Middle ground, more complex for Phase 0. | |
| Outro | Freeform alternative. | |

**User's choice:** One configurable `services/keyprovider/`.
**Notes:** User chose three service bases: `services/kme-mock/`, `services/keyprovider/`, `services/encryptor-sim/`; Compose instantiates A/B roles.

---

## Regra exata de keyId

| Option | Description | Selected |
|--------|-------------|----------|
| HMAC-SHA256 truncado para 128 bits | Deterministic, hex, collision-aware, and does not reveal `qkd_key_id` directly when the namespace secret is used. | x |
| SHA-256 truncado para 128 bits sem segredo | Simpler, but correlatable if inputs are known. | |
| Mapeamento textual normalizado | Simpler, weaker, and less aligned with SKIP expectations. | |
| Outro | Freeform alternative. | |

**User's choice:** HMAC-SHA256 truncado para 128 bits.
**Notes:** `SKIP_KEY_ID_NAMESPACE_SECRET` is a shared env var between providers. Collision behavior is fail/report/no overwrite.

---

## Criterio de aceite da Fase 0

| Option | Description | Selected |
|--------|-------------|----------|
| Arquivos + consistencia + compose parseavel | All docs exist, requirements are reflected, directories exist, and Compose config is parseable. | x |
| Arquivos apenas | Faster but weaker. | |
| Compose sobe containers mesmo sem logica completa | Stronger but requires more runnable scaffolding. | |
| Outro | Freeform alternative. | |

**User's choice:** Arquivos + consistencia + compose parseavel.
**Notes:** Consistency proof is a manual checklist in `docs/test-plan.md` plus `docker compose -f infra/docker-compose.yml config` when available. Containers do not need to start in Phase 0.

---

## the agent's Discretion

- Exact Docker Compose service names, ports, placeholder filenames, and doc ordering.
- Exact scaffold placeholder style, as long as full service logic is not implemented in Phase 0.

## Deferred Ideas

- Real Cisco/hardware encryptor integration.
- Real IKEv2/RFC8784 integration.
- Full ETSI GS QKD 014 conformance.
