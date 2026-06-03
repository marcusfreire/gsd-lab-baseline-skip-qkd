# Stack Research

**Domain:** QKD ETSI 014 mock + SKIP Key Provider integration baseline
**Researched:** 2026-06-02
**Confidence:** HIGH for protocol constraints; MEDIUM for implementation package versions because Python ecosystem versions change frequently.

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

Phase 0 should not install or implement full service logic yet. When service scaffolding begins, use pinned dependencies similar to:

```bash
pip install fastapi==0.136.3 uvicorn==0.48.0 pydantic==2.13.4 httpx==0.28.1 SQLAlchemy==2.0.50 pytest==9.0.3
```

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

**If Phase 0 stays documentation/scaffold-only:**
- Create service directories, empty FastAPI app placeholders, and Docker Compose service definitions.
- Avoid dependency installation unless needed to validate compose/build shape.

**If Phase 1 implements minimal APIs:**
- Use FastAPI routers per protocol boundary.
- Define Pydantic models for every request/response.
- Use SQLite repositories behind service interfaces, not direct DB calls inside route handlers.

**If later phases add TLS/authentication:**
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

---
*Stack research for: QKD ETSI 014 mock + SKIP baseline*
*Researched: 2026-06-02*
