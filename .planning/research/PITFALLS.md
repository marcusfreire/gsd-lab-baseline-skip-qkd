# Pitfalls Research

**Domain:** QKD ETSI 014 mock + SKIP Key Provider integration baseline
**Researched:** 2026-06-02
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: Implementing "SKIP-like" Instead of SKIP

**What goes wrong:**
The provider exposes convenient PPK endpoints but misses SKIP paths, query parameters, response fields, status codes, or semantics from `draft-singh-skip-00`.

**Why it happens:**
SKIP looks simple enough to approximate, but later real encryptor integration will depend on exact behavior such as `/capabilities`, `/key`, `/key/{keyId}`, `/entropy`, `localSystemID`, `remoteSystemID`, hex `keyId`, and zeroization/lifecycle assumptions.

**How to avoid:**
Make `docs/api-skip.md` a conformance matrix against `draft-singh-skip-00`. Future tests should fail when route shapes or JSON fields drift.

**Warning signs:**
Routes mention PPK but not SKIP; response fields use local names like `skip_key_id` instead of SKIP `keyId`; `/entropy` is omitted without an explicit decision.

**Phase to address:**
Phase 0, before service logic.

---

### Pitfall 2: Collapsing ETSI and SKIP Identifiers

**What goes wrong:**
`qkd_key_id`, ETSI `key_ID`, SKIP `keyId`, and future RFC8784 PPK identity are treated as the same value.

**Why it happens:**
All are key identifiers and the baseline requires deterministic derivation, so it is tempting to reuse one string everywhere.

**How to avoid:**
Document each identifier domain separately. Derive SKIP `keyId` deterministically from `qkd_key_id`, but keep type, encoding, and protocol ownership explicit.

**Warning signs:**
Data model has only one `key_id`; docs do not distinguish base64 ETSI key material from hex SKIP key material.

**Phase to address:**
Phase 0 data model.

---

### Pitfall 3: Breaking Provider Independence

**What goes wrong:**
KeyProvider-A and KeyProvider-B share a central DB or hidden service state to make matching keys easy.

**Why it happens:**
Shared state is simpler for a demo, but it invalidates the baseline's independent-provider premise.

**How to avoid:**
Run the same provider code twice with different config and separate SQLite DBs. Use KME mock behavior plus deterministic ID mapping as the common source of truth.

**Warning signs:**
Compose has one provider DB volume; provider code imports a shared repository for both A and B; tests pass only because both providers read the same records.

**Phase to address:**
Phase 0 architecture and Compose skeleton; Phase 1 implementation.

---

### Pitfall 4: Over-Implementing ETSI 014

**What goes wrong:**
The KME mock tries to implement full ETSI GS QKD 014 behavior and pulls the project away from SKIP/PPK integration.

**Why it happens:**
The standard includes useful details such as HTTPS, mutual authentication, status data, key request extensions, key containers, and multiple SAEs.

**How to avoid:**
Declare a minimal compatibility profile: status, initiator key request, responder key retrieval by IDs, JSON payloads, and documented deviations.

**Warning signs:**
Phase 0 tasks include TLS certificate management, multicast, extension negotiation, or QKD network management.

**Phase to address:**
Phase 0 docs.

---

### Pitfall 5: Treating Simulated Keys as Production Secrets

**What goes wrong:**
Docs or demos imply the baseline is secure for production key handling.

**Why it happens:**
The domain is security-sensitive and uses terms like QKD, SKIP, PPK, TLS, and post-quantum.

**How to avoid:**
Write `docs/security-assumptions.md` plainly: simulated material is for integration validation only; TLS/auth/secret storage/zeroization are future hardening unless implemented and tested.

**Warning signs:**
README says "secure" without qualifying assumptions; logs print raw keys without noting the risk; no warning about local-only use.

**Phase to address:**
Phase 0.

---

### Pitfall 6: Losing RFC8784 Compatibility While Deferring IKE

**What goes wrong:**
The project defers IKEv2 correctly but chooses key IDs or PPK material formats that are hard to map into RFC8784 later.

**Why it happens:**
Simulated encryptors can accept any JSON shape unless constrained by docs.

**How to avoid:**
Document that future IKEv2/RFC8784 will use the SKIP `keyId` as the PPK identity input, with attention to RFC8784 PPK_ID_FIXED/OPAQUE choices and base64 interoperability recommendations.

**Warning signs:**
`keyId` is a human label with spaces or non-stable formatting; PPK length/entropy assumptions are absent.

**Phase to address:**
Phase 0 data model and security assumptions.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Handwritten JSON dictionaries without schemas | Fast docs/code | Contract drift and weak tests | Only in docs examples, not implementation. |
| Duplicating KeyProvider-A and KeyProvider-B source code | Fast demo | Divergence between providers | Never; use one configurable implementation. |
| In-memory provider state | Quick service startup | Restart tests cannot validate persistence | Only for KME mock if documented; not for provider state. |
| Skipping negative status-code tests | Faster happy-path demo | Real clients fail unpredictably | Not acceptable for SKIP contract tests. |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| ETSI mock | Using SKIP hex key format inside ETSI key container | Keep ETSI-facing key material/base64 and key IDs documented separately from SKIP. |
| SKIP | Returning 404 for key not found | SKIP key request errors include 400 for malformed/missing key and 500 for read/zeroize failures; document exact mapping. |
| RFC8784 | Assuming any string can be a PPK identity without later encoding concerns | Document future PPK_ID_FIXED/OPAQUE decision and interoperable encoding constraints. |
| Docker Compose | One shared volume for both providers | Separate volumes/DB paths per provider. |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Holding DB write locks too broadly | Parallel requests fail under simple e2e tests | Keep SQLite transactions short; use one DB per provider. | Multiple concurrent key requests. |
| Generating all keys at startup | Slow startup and stale inventory | Generate on demand or seed small deterministic inventory for tests. | Larger demo inventories. |
| Synchronous HTTP calls without timeouts | Hanging e2e tests | Set explicit client timeouts. | Any unavailable service in Compose. |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Logging raw key material by default | Leaks simulated secrets and normalizes unsafe practice | Redact keys in logs; show hashes/IDs in docs. |
| Recoverable key derivation from `keyId` | Violates SKIP's keyId security intent | Derive only identifier from `qkd_key_id`; key material comes from KME. |
| Treating local HTTP as equivalent to SKIP's TLS expectation | Misleading production posture | Document local HTTP as a Phase 0/1 simplification; plan TLS/auth hardening later. |
| Low-entropy PPK material | Undermines RFC8784 future use | Generate at least 256-bit logical key material in implementation phases. |
| Exposing full topology via `remoteSystemID` | Leaks network structure | Use pseudonymous IDs or document lab-only IDs. |

## "Looks Done But Isn't" Checklist

- [ ] **SKIP API:** Routes exist but fields/status codes do not match `draft-singh-skip-00`.
- [ ] **KME mock:** Happy-path key retrieval works but `status` and `dec_keys` semantics are undocumented.
- [ ] **Provider independence:** Two services run but share a DB volume.
- [ ] **ID mapping:** `skip_key_id` is deterministic but collision handling is undefined.
- [ ] **Future RFC8784:** PPK identity format is not documented.
- [ ] **Security:** README lacks local-only/simulation warnings.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| SKIP-like drift | MEDIUM | Freeze API doc, add contract tests, rename non-SKIP endpoints internal-only. |
| Shared provider state | HIGH | Split DB schema/volumes, rewrite tests to assert independent persistence. |
| ETSI over-scope | MEDIUM | Create explicit mock profile and move full conformance to v2. |
| Secret-handling confusion | LOW | Add warnings, redact logs, document assumptions. |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| SKIP-like drift | Phase 0 | `docs/api-skip.md` contains method/status/field matrix tied to `draft-singh-skip-00`. |
| Identifier collapse | Phase 0 | `docs/data-model.md` has separate fields and mapping rules. |
| Provider shared state | Phase 0 | Compose declares separate provider DB volumes/paths. |
| ETSI over-scope | Phase 0 | `docs/api-etsi014-mock.md` defines minimal profile and deviations. |
| Production-security confusion | Phase 0 | `docs/security-assumptions.md` explicitly limits claims. |
| RFC8784 mismatch | Phase 0 | `docs/data-model.md` and `docs/security-assumptions.md` preserve future PPK constraints. |

## Sources

- https://datatracker.ietf.org/doc/html/draft-singh-skip-00 - SKIP API, key flow, keyId/key semantics, TLS/security considerations.
- https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf - ETSI 014 SAE/KME model, key delivery API, security assumptions.
- https://datatracker.ietf.org/doc/html/rfc8784 - PPK identity, PPK distribution, entropy/security considerations.

---
*Pitfalls research for: QKD ETSI 014 mock + SKIP baseline*
*Researched: 2026-06-02*
