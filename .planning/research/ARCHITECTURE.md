# Architecture Research

**Domain:** QKD ETSI 014 mock + SKIP Key Provider integration baseline
**Researched:** 2026-06-02
**Confidence:** HIGH for component boundaries and protocol flow.

## Standard Architecture

### System Overview

```text
+------------------------------------------------------------------+
|                         Local Docker Network                      |
|                                                                  |
|  +------------------+        ETSI 014-like        +-------------+ |
|  | KeyProvider-A    | <-------------------------- | KME Mock    | |
|  | SQLite DB A      |                             | logical QKD | |
|  +--------+---------+ --------------------------> +------+------+ |
|           |               key collection                 ^        |
|           |                                              |        |
|        SKIP HTTP(S)                                      |        |
|           |                                              |        |
|  +--------v---------+                         +----------+-----+  |
|  | Encryptor-A sim  | ---- keyId handoff ---> | Encryptor-B sim |  |
|  +------------------+                         +----------+-----+  |
|           ^                                              |        |
|           |                                              |        |
|        SKIP HTTP(S)                                      |        |
|           |                                              |        |
|  +--------+---------+        ETSI 014-like                |        |
|  | KeyProvider-B    | <-----------------------------------+        |
|  | SQLite DB B      |                                             |
|  +------------------+                                             |
+------------------------------------------------------------------+
```

Phase 0 should document this topology and create the directories/Compose skeleton. Later phases fill in service logic.

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| KME mock | Logical ETSI GS QKD 014 source of synchronized key material. | FastAPI service with in-memory or local state; exposes `status`, `enc_keys`, `dec_keys` shape. |
| KeyProvider-A | Initiator-side KP; collects from KME as ETSI client and serves SKIP to Encryptor-A. | FastAPI service + SQLite DB A + KME client + SKIP router. |
| KeyProvider-B | Responder-side KP; retrieves matching KME key material and serves SKIP to Encryptor-B. | Same implementation profile as A, separate config and DB. |
| Encryptor-A sim | Initiates SKIP key request and hands `keyId` to peer simulation. | Lightweight FastAPI service or script in later phase. |
| Encryptor-B sim | Receives `keyId` and requests matching key from KP-B. | Lightweight FastAPI service or script in later phase. |
| Docs/test plan | Contract definition and verification roadmap. | Markdown docs plus future pytest contract/e2e tests. |

## Recommended Project Structure

```text
.
├── README.md
├── docs/
│   ├── architecture.md
│   ├── api-etsi014-mock.md
│   ├── api-skip.md
│   ├── data-model.md
│   ├── security-assumptions.md
│   └── test-plan.md
├── infra/
│   └── docker-compose.yml
├── services/
│   ├── kme-mock/
│   │   ├── app/
│   │   └── tests/
│   ├── keyprovider/
│   │   ├── app/
│   │   └── tests/
│   └── encryptor-sim/
│       ├── app/
│       └── tests/
└── tests/
    ├── contract/
    └── e2e/
```

### Structure Rationale

- **`services/keyprovider/`:** one implementation should be configurable as A or B to avoid duplicated logic, while Compose runs two independent instances with separate DB volumes.
- **`services/kme-mock/`:** isolates ETSI-like behavior from provider and SKIP concerns.
- **`services/encryptor-sim/`:** keeps simulated consumers explicit and separate from Key Provider tests.
- **`docs/`:** Phase 0's main deliverable; documentation is the technical plan and contract.
- **`tests/contract/`:** future place for protocol conformance tests against docs.
- **`tests/e2e/`:** future place for Docker Compose flow tests.

## Architectural Patterns

### Pattern 1: Protocol Adapters Around a Small Domain Core

**What:** Keep ETSI client code, SKIP route handlers, and local persistence as adapters around a small key-inventory/domain layer.

**When to use:** As soon as Key Provider implementation begins.

**Trade-offs:** Slightly more structure than route-handler-only code, but prevents protocol details from being mixed with state transitions.

### Pattern 2: Configured Service Instances Instead of Duplicated Services

**What:** KeyProvider-A and KeyProvider-B should run the same code with different `LOCAL_SYSTEM_ID`, `REMOTE_SYSTEM_ID`, `DB_PATH`, and `KME_BASE_URL`.

**When to use:** Starting in service scaffolding.

**Trade-offs:** Requires careful config validation, but avoids divergent A/B behavior.

### Pattern 3: Contract-First Protocol Documentation

**What:** Write `docs/api-skip.md` and `docs/api-etsi014-mock.md` before service logic.

**When to use:** Phase 0.

**Trade-offs:** Slower start, but critical because protocol drift is the largest project risk.

## Data Flow

### Initiator SKIP Flow

```text
Encryptor-A sim
    -> GET /capabilities on KeyProvider-A
    -> GET /key?remoteSystemID=Bob
KeyProvider-A
    -> request logical QKD key from KME mock as client/SAE
    -> derive SKIP keyId from qkd_key_id
    -> persist mapping in SQLite DB A
    -> return { keyId, key } to Encryptor-A sim
Encryptor-A sim
    -> hands keyId to Encryptor-B sim
```

### Responder SKIP Flow

```text
Encryptor-B sim
    -> GET /capabilities on KeyProvider-B
    -> GET /key/{keyId}?remoteSystemID=Alice
KeyProvider-B
    -> resolve qkd_key_id from deterministic keyId mapping
    -> request matching logical QKD key from KME mock
    -> persist mapping in SQLite DB B
    -> return { keyId, key } to Encryptor-B sim
```

### Future IKEv2/RFC8784 Flow

```text
IKE_SA_INIT
    -> peers signal PPK support
IKE_AUTH
    -> initiator sends PPK identity/keyId
Responder
    -> looks up matching PPK by identity
IKEv2
    -> mixes PPK into key derivation
```

Phase 0 only preserves the identifier/key compatibility needed for this future flow.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| Lab single pair | Docker Compose, SQLite per provider, one KME mock. |
| Multiple provider pairs | Add provider instances with separate DB volumes and system IDs; keep KME mock deterministic. |
| Real integration | Add TLS/auth, real encryptor adapters, key lifecycle, stronger persistence/observability. |

### Scaling Priorities

1. **First bottleneck:** protocol ambiguity, not throughput. Fix with docs and contract tests.
2. **Second bottleneck:** provider state consistency and one-time key semantics. Fix with explicit lifecycle states and DB constraints in later phases.

## Anti-Patterns

### Anti-Pattern 1: Treating KME Mock and Key Provider as One Service

**What people do:** Put QKD key source and SKIP provider behavior in the same process/state.

**Why it's wrong:** It hides the real integration boundary and makes the provider less useful for later real KME integration.

**Do this instead:** Keep KME mock and Key Providers separate services.

### Anti-Pattern 2: Using SKIP-Inspired Routes

**What people do:** Create convenient REST endpoints like `/ppk/provision` and call it SKIP.

**Why it's wrong:** It will fail later against real SKIP clients/encryptors.

**Do this instead:** Document and test the exact SKIP surface from `draft-singh-skip-00`.

### Anti-Pattern 3: Sharing Provider State Centrally

**What people do:** Use one central DB for both Key Providers.

**Why it's wrong:** It violates the independence constraint and overstates synchronization guarantees.

**Do this instead:** Separate SQLite stores; synchronization comes from KME mock/logical key source and deterministic mapping.

## Integration Points

### External/Protocol Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| ETSI 014 KME mock | Key Provider acts as SAE/client | Mock should expose only logical key delivery needed for the baseline. |
| SKIP encryptor interface | Key Provider acts as SKIP server | Contract is `draft-singh-skip-00`. |
| RFC8784 IKEv2 | Future consumer of PPK/keyId | Preserve compatible identity/key semantics but do not implement now. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| KeyProvider -> KME mock | HTTP JSON | ETSI-like API; KME and SAE IDs are explicit. |
| Encryptor sim -> KeyProvider | HTTP JSON | SKIP API; key material is hex. |
| KeyProvider -> SQLite | Local DB file | Separate DB path per provider. |
| Encryptor-A sim -> Encryptor-B sim | Simulated keyId handoff | Models future IKE keyId transfer without real IKE. |

## Sources

- https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf - SAE/KME architecture, key delivery flow, API method shape.
- https://datatracker.ietf.org/doc/html/draft-singh-skip-00 - SKIP topology, capabilities/key/entropy API, key flow.
- https://datatracker.ietf.org/doc/html/rfc8784 - future IKEv2 PPK identity/key usage.

---
*Architecture research for: QKD ETSI 014 mock + SKIP baseline*
*Researched: 2026-06-02*
