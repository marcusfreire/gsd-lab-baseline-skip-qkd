# ADR 0003: Use SAE Identity and Topology as Security Policy

## Status

Accepted

## Context

ETSI GS QKD 014 describes access by master and slave SAE identities connected
to KMEs. The simulator models this through configured KME topology and caller
identity resolution.

The current simulator can run in two transport modes:

- plain HTTP for local tests and development;
- HTTPS with mutual TLS for experiments that should model real ETSI transport
  behavior.

## Decision

Treat configured KME topology as an authorization boundary, not only as status
metadata.

Protocol handlers must validate caller SAE identity and requested peer SAE
relationships against the configured topology before issuing or releasing key
material.

Identity resolution depends on transport mode:

- when mTLS is enabled, the caller SAE identity comes from the authenticated
  client certificate Common Name;
- when mTLS is disabled, the caller SAE identity may come from the `x-sae-id`
  header;
- `x-sae-id` is acceptable only when running without mTLS configuration.

An unknown caller SAE, unknown requested SAE, or mismatched key ownership must
be rejected according to the ETSI 014 API's authorization and error behavior.

## Consequences

- The simulator catches client behavior that ignores ETSI master/slave
  ownership rules.
- Plain HTTP mode remains useful for unit tests, router tests, and local client
  development, but it is not a model of authenticated deployment behavior.
- mTLS experiments require client certificates whose subject Common Name
  exactly matches the intended SAE ID in the configured topology.
