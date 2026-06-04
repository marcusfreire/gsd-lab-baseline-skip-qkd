# Simulated Encryptor Service Scaffold

This directory is the future Python/FastAPI implementation location for
simulated encryptors.

The simulated encryptors are the first SKIP consumers:

- `Encryptor-A sim` requests a fresh key from `KeyProvider-A` with
  `GET /key?remoteSystemID=Bob`.
- `Encryptor-B sim` receives the simulated `keyId` handoff and requests matching
  key material from `KeyProvider-B` with
  `GET /key/{keyId}?remoteSystemID=Alice`.

Real Cisco integration and real IKEv2/RFC8784 negotiation are future work. The
Phase 0 simulated encryptor scaffold does not implement IPsec, IKEv2, Cisco
behavior, or working SKIP client logic.

Relevant contracts:

- `../../docs/api-skip.md`
- `../../docs/data-model.md`
- `../../docs/security-assumptions.md`
