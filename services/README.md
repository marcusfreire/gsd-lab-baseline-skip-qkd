# Services Scaffold

This directory is reserved for future Python/FastAPI services used by the QKD
ETSI 014 + SKIP baseline.

The KME simulator is the official versioned `../kms/` baseline component. It is
not services/kme-mock, and Phase 0 must not create a competing KME mock service
under `services/`.

Planned service families:

- `keyprovider/` - one configurable future Key Provider implementation that
  can run as `KeyProvider-A` or `KeyProvider-B`.
- `encryptor-sim/` - future simulated encryptors that consume SKIP before real
  Cisco or real IKEv2/RFC8784 integration.

Phase 0 contains scaffold documentation only. It does not include route
handlers, FastAPI app entrypoints, database schemas, or working service logic.
