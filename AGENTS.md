<!-- GSD:project-start source:PROJECT.md -->

## Project

**QKD ETSI 014 Mock + SKIP Baseline**

This project is an executable local baseline for integrating a logical ETSI GS QKD 014 mock API, independent SKIP Key Providers, and simulated encryptors. It is aimed primarily at lab/research users who need a reproducible environment to validate how QKD-derived key material can be collected by Key Providers and exposed through SKIP for future IKEv2/RFC8784 PPK use.

The first milestone is deliberately pre-implementation: it defines the architecture, API contracts, data model, security assumptions, test plan, Docker Compose skeleton, and service directory structure before service logic is built.

**Core Value:** Prove, with precise contracts and a runnable local shape, that independent Key Providers can act as ETSI 014 mock clients toward a simulated KME and as SKIP servers toward encryptors while preserving compatible key material on both sides.

### Constraints

- **Stack**: Python/FastAPI for services - chosen for fast API iteration, testability, SQLite support, and simple local service scaffolding.
- **Runtime**: Docker Compose is the primary local runtime - the baseline must be easy to run as multiple local services.
- **Persistence**: Each Key Provider uses its own SQLite database - provider state must survive restarts without introducing a central provider database.
- **Protocol Contract**: SKIP behavior follows `draft-singh-skip-00` - endpoint names, JSON fields, status codes, `localSystemID`, `remoteSystemID`, `keyId`, `key`, and entropy behavior must be documented against that draft.
- **ETSI Scope**: ETSI GS QKD 014 is simulated logically - only the KME behaviors required for key collection are modeled in the first baseline.
- **Identifier Mapping**: `skip_key_id` / SKIP `keyId` is deterministically derived from `qkd_key_id` - the derivation must be stable, documented, collision-aware, and must not reveal key material.
- **Security**: Simulated key material is not production-protected - security assumptions must explicitly avoid implying production readiness.
- **Phase 0 Boundary**: No full service logic before the technical plan and documentation baseline exist - the first phase is documentation, scaffolding, and acceptance criteria.

<!-- GSD:project-end -->

<!-- GSD:stack-start source:research/STACK.md -->

## Technology Stack

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Python | 3.12.x or 3.13.x | Service implementation language | Stable FastAPI support, broad crypto/testing ecosystem, and straightforward Docker images. Prefer 3.12 for conservative compatibility; 3.13 is viable if dependencies are pinned and tested. |
| FastAPI | 0.136.3 | REST API framework for KME mock, Key Providers, and simulated encryptors | Direct OpenAPI generation, Pydantic validation, and good fit for explicit protocol contracts. |
| Uvicorn | 0.48.0 | ASGI runtime | Standard FastAPI runtime path with simple local service startup. |
| Pydantic | 2.13.4 | Request/response validation | FastAPI depends on Pydantic/Starlette; explicit models are useful for ETSI/SKIP contract tests. |
| SQLite | Built into Python | Per-provider local state | Matches the requirement for independent KeyProvider-A and KeyProvider-B local persistence without a central database. |
| SQLAlchemy | 2.0.50 | Database abstraction | Lets each service use a small repository layer and keeps schema definitions explicit for later migrations. |
| Docker Compose | Current local Docker plugin | Local multi-service runtime | Best fit for running KME mock, KeyProvider-A, KeyProvider-B, and simulated encryptors with explicit network names, ports, volumes, and environment. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| httpx | 0.28.1 | HTTP client and tests | Key Providers call KME mock; simulated encryptors call SKIP; integration tests call services. |
| pytest | 9.0.3 | Test runner | Contract and integration tests for ETSI mock and SKIP flows. |
| pytest-asyncio | Pin current during implementation | Async endpoint/client tests | Needed if service tests use async `httpx.AsyncClient`. |
| typer | Pin current during implementation | Minimal CLI for inspection/debug | Optional, but useful for provider state inspection without expanding the REST surface. |
| cryptography | Pin current during implementation | Random bytes, hashes/HMAC if needed | Use for cryptographic primitives only if stdlib is insufficient; avoid inventing primitives. |
| ruff | Pin current during implementation | Lint/format | Keeps generated scaffolding consistent across services. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| Docker Compose | Multi-service local runtime | Primary operator path for the baseline. |
| OpenAPI JSON | API contract review | FastAPI can emit docs automatically, but `docs/api-*.md` remains the human contract. |
| pytest markers | Separate unit, contract, and e2e tests | Important because Phase 0 begins with docs/scaffold before full logic. |
| Makefile or task runner | Repeatable commands | Add only if commands become repeated; avoid tool churn in Phase 0. |

## Installation

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| FastAPI | Flask | Use Flask only if automatic validation/OpenAPI is not wanted. For protocol contracts, FastAPI is better aligned. |
| SQLite per provider | PostgreSQL | Use PostgreSQL only when multi-writer or operational DB features become requirements. It violates the current simplicity target. |
| SQLAlchemy | Raw `sqlite3` | Raw SQLite is fine for very small services, but SQLAlchemy makes schema/test evolution easier. |
| Docker Compose | Local scripts only | Scripts can be added later for development, but Compose is the primary baseline runtime. |
| Simulated encryptor service | Real Cisco/strongSwan/libreswan | Real encryptor integration belongs in a future phase after SKIP contract behavior is stable. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Central shared database between Key Providers | Breaks the independence constraint and hides synchronization problems. | Separate SQLite DB per provider. |
| Physical QKD simulators in the baseline | Pulls scope toward BB84/channel modeling instead of logical key delivery. | Minimal ETSI 014 logical KME mock. |
| Ad hoc SKIP-like endpoints | The project must follow `draft-singh-skip-00`, especially paths, JSON field names, and key semantics. | Contract-first `docs/api-skip.md` and tests. |
| Real IKEv2 in Phase 0 | Adds operational complexity before SKIP delivery is validated. | Simulated encryptors that exercise SKIP. |
| Alpine image by default | Smaller but musl/libc differences can complicate Python packages. | `python:3.12-slim-bookworm` or `python:3.13-slim-bookworm` unless image size becomes critical. |

## Stack Patterns by Variant

- Create service directories, empty FastAPI app placeholders, and Docker Compose service definitions.
- Avoid dependency installation unless needed to validate compose/build shape.
- Use FastAPI routers per protocol boundary.
- Define Pydantic models for every request/response.
- Use SQLite repositories behind service interfaces, not direct DB calls inside route handlers.
- Keep TLS/auth as explicit protocol-hardening work.
- Do not let local HTTP mocks imply production security.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| FastAPI 0.136.3 | Python >=3.10 | PyPI metadata lists Python 3.10+ support. |
| FastAPI 0.136.3 | Pydantic 2.x | FastAPI metadata/classifiers target Pydantic 2. |
| Uvicorn 0.48.0 | Python >=3.10 | Aligns with FastAPI's supported Python range. |
| SQLAlchemy 2.0.50 | SQLite | Built-in SQLite use is sufficient for per-provider state. |
| Python Docker image | `3.12.13-slim-bookworm`, `3.13.13-slim-bookworm` | Official image tags exist; 3.12 is conservative, 3.13 is current. |

## Sources

- https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf - ETSI GS QKD 014 key delivery API scope, REST/JSON/HTTPS shape, SAE/KME model.
- https://datatracker.ietf.org/doc/html/draft-singh-skip-00 - SKIP active draft used as pinned contract.
- https://datatracker.ietf.org/doc/html/rfc8784 - Future PPK/IKEv2 constraints.
- https://pypi.org/project/fastapi/ - FastAPI 0.136.3 metadata and dependencies.
- https://pypi.org/project/uvicorn/ - Uvicorn 0.48.0 metadata.
- https://pypi.org/project/pydantic/ - Pydantic 2.13.4 metadata.
- https://pypi.org/project/SQLAlchemy/ - SQLAlchemy 2.0.50 metadata.
- https://pypi.org/project/httpx/ - HTTPX 0.28.1 metadata.
- https://pypi.org/project/pytest/ - pytest 9.0.3 metadata.
- https://hub.docker.com/_/python/ - official Python Docker tags and image variant guidance.

<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->

## Conventions

Conventions not yet established. Will populate as patterns emerge during development.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->

## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->

## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->

## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:

- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->

## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
