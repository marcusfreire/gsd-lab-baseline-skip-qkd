# ADR 0002: Preserve ETSI GS QKD 014 API Fidelity

## Status

Accepted

## Context

The simulator's first client-facing protocol is ETSI GS QKD 014. The value of
the simulator depends on clients seeing the same routes, methods, JSON field
names, status codes, and key lifecycle behavior they would see from a
conforming KME.

Rust code should remain idiomatic internally, but public protocol surfaces must
match the standard.

## Decision

The ETSI 014 public API must strictly implement the protocol surface described
by ETSI GS QKD 014 for the supported subset:

- `GET /api/v1/keys/{slave_SAE_ID}/status`
- `POST /api/v1/keys/{slave_SAE_ID}/enc_keys`
- `GET /api/v1/keys/{slave_SAE_ID}/enc_keys` for the protocol's simple request
  form
- `POST /api/v1/keys/{master_SAE_ID}/dec_keys`
- `GET /api/v1/keys/{master_SAE_ID}/dec_keys` for the protocol's simple request
  form

Public JSON names must preserve ETSI naming, including fields such as
`source_KME_ID`, `target_KME_ID`, `master_SAE_ID`, `slave_SAE_ID`, `key_ID`,
and `key_IDs`.

Internal Rust names should remain idiomatic snake case. Use Serde `rename`
attributes to bridge idiomatic Rust names to ETSI JSON names.

API errors must follow the status codes and error response data model defined
by ETSI 014 where the protocol defines them. Internal setup, storage, and
infrastructure errors may use `anyhow` where no protocol-level classification
is required.

## Consequences

- Client interoperability and conformance testing take priority over local API
  convenience.
- Tests must assert paths, methods, JSON names, request forms, response forms,
  and HTTP status codes.
- Unsupported ETSI options should fail with protocol-shaped errors rather than
  being silently ignored.
