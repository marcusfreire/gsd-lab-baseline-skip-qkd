# ADR 0001: Build KMS as an Experimental Key-Management Simulator

## Status

Accepted

## Context

The `kms` crate supports tools and experiments that use Quantum Key
Distribution devices and Key Management Entities. Its first implemented
protocol is ETSI GS QKD 014, exposed under `/api/v1/keys`.

The crate is not intended for production use. It exists to provide a
high-fidelity simulator for QKD devices and KME APIs so clients can be tested
against realistic protocol behavior without requiring physical QKD hardware.

Future experiments may add more key-distribution protocols or
key-management features.

## Decision

Design `kms` as one experimental simulator binary configured through the
`config` module.

The architecture should separate shared simulator concerns from
protocol-specific behavior:

- protocol modules own public routes, request/response shapes, protocol error
  semantics, and protocol lifecycle rules;
- topology models which KME owns which SAE and is part of the simulator's
  security policy;
- caller identity is resolved before protocol handlers make authorization
  decisions;
- key sources provide simulated key material through an explicit interface;
- storage modules own protocol-specific key persistence and consumption
  semantics.

The ETSI GS QKD 014 implementation is the first protocol module. Additional
experimental protocols should live inside the same binary and be enabled or
configured through the same configuration layer.

The crate documentation and ADRs must make clear that the simulator is not a
production KMS.

## Consequences

- The simulator can grow beyond ETSI 014 without turning `main` or the route
  layer into a collection of unrelated special cases.
- Shared concerns such as topology, identity, key sources, and storage stay
  reusable across protocol modules.
- Production requirements such as HSM integration, clustered consistency,
  hardened secret handling, operational policy, and compliance controls are
  explicitly out of scope unless introduced for an experiment.
