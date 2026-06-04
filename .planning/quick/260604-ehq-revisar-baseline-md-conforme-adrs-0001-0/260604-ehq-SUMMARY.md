---
quick_id: 260604-ehq
status: complete
completed: 2026-06-04
commit: 4a37040
---

# Quick Task 260604-ehq Summary

## Completed

Revised `baseline.md` to describe the project as a faithful local subset of
ETSI GS QKD 014 as adopted by the `kms/` Experimental Key-Management
Simulator, using ADRs 0001-0005 as the controlling source.

## Files Modified

- `baseline.md`
- `.planning/quick/260604-ehq-revisar-baseline-md-conforme-adrs-0001-0/260604-ehq-PLAN.md`
- `.planning/quick/260604-ehq-revisar-baseline-md-conforme-adrs-0001-0/260604-ehq-SUMMARY.md`

## Key Outcomes

- Replaced the old generic KME/SKIP baseline narrative with the committed
  `kms/` KMS simulator as the authoritative ETSI 014 implementation baseline.
- Documented the supported ETSI public routes only as `status`, `enc_keys`,
  and `dec_keys`, preserving ETSI public JSON names from the KMS ADRs.
- Captured SAE identity and topology policy: mTLS Common Name in hardened mode,
  `x-sae-id` only when mTLS is disabled, and explicit master/slave
  authorization checks.
- Captured ADR 0004 key lifecycle semantics: deterministic fake sources for
  local compatibility, SQLite default durable storage, base64 ETSI key material,
  and one-time successful `dec_keys` consumption.
- Aligned SKIP exposition with the ETSI subset by deriving hexadecimal SKIP key
  material from ETSI key bytes and using
  `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}` as the deterministic key ID.

## Verification

- Passed: `rg` checks for KMS/ADR/KME/SAE/provider alignment terms in
  `baseline.md`.
- Passed: `rg` checks for the supported ETSI route templates and public JSON
  names in `baseline.md`.
- Passed: `rg` checks for ADR-driven identity, topology, lifecycle, storage,
  and verification requirements in `baseline.md`.
- Passed: negative `rg` check for stale generic names including
  `GET_STATUS`, `GET_KEY`, `GET_KEY_WITH_KEY_IDS`,
  `/api/v1/keys/get_key`, `source_sae_id`, `target_sae_id`, `qkd_key_id`, and
  `SKIP-A-B-000001`.

## Notes

This was a documentation-only baseline correction. No Rust or Python runtime
tests were required for the changed artifact.
