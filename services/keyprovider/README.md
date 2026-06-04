# Key Provider Service Scaffold

This directory is the future shared Python/FastAPI implementation location for
the SKIP Key Provider service.

One configurable implementation will be instantiated as:

- `KeyProvider-A`, acting as `SAE-A` toward `KME-A`.
- `KeyProvider-B`, acting as `SAE-B` toward `KME-B`.

Each instance acts as an ETSI 014 client toward its local KME and as a SKIP
server toward its local simulated encryptor. KeyProvider-A must call only
KME-A; KeyProvider-B must call only KME-B.

Provider persistence is per-provider SQLite state. There must be no central
provider database and no shared provider state volume.

Relevant contracts:

- `../../docs/api-etsi014-mock.md`
- `../../docs/api-skip.md`
- `../../docs/data-model.md`
- `../../docs/security-assumptions.md`

Phase 0 does not add FastAPI route handlers, repository schemas, app entrypoint
files, or runtime service logic.
