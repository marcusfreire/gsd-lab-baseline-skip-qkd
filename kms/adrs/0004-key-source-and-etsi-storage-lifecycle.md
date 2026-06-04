# ADR 0004: Use Explicit Key Sources and One-Time ETSI Key Storage

## Status

Accepted

## Context

The simulator must issue key material to clients without requiring physical QKD
hardware. Today it generates 256-bit keys from the operating-system random
source and stores them with ETSI ownership metadata until the slave SAE
retrieves them.

ETSI 014 client behavior depends on `key_ID` ownership and on the post-condition
that retrieved keys are no longer available for repeated delivery.

## Decision

Model generated key material behind a key-source interface.

The current runtime key-source implementation is operating-system randomness,
equivalent to an `OsRng`-backed CSPRNG source. This represents simulated QKD
key material for experiments. The interface should allow future implementations
for deterministic tests, replayable experiments, fault injection, or adapters
that model other experimental key sources.

Keep the ETSI storage abstraction specifically shaped around ETSI key storage.
It should store keys by `key_ID` with their `master_sae_id`, `slave_sae_id`,
and key bytes.

For ETSI 014, one-time `dec_keys` consumption is a core invariant:

- `enc_keys` stores generated keys with their original master and slave SAE
  ownership;
- `dec_keys` verifies the caller slave SAE and path master SAE against that
  ownership;
- after successful retrieval, storage deletes the returned keys.

SQLite is the default configured durable storage backend. Memory storage remains
available for ephemeral runs, unit tests, and local development.

## Consequences

- The simulator can catch clients that incorrectly depend on fetching the same
  decrypt key more than once.
- Storage remains clear and protocol-faithful instead of prematurely
  generalized.
- Future key-source behavior can be added without changing ETSI route semantics.
