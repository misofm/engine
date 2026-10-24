# Add the dynamic EQ's audit and AArch64 rows

Filed by root as #17's slice S3 (backlog sprint, 2026-10-24). The web kernel rows land with #17 S2 (the #1889 table enforces them).

## Smallest closable slice

1. An `audit dynamic-eq` subcommand row and its CI and AArch64 rows, as the shipped effects have (see #1920 for the de-esser's).
2. Console eligibility (decision 12's list) only if the owner says yes; the question is on root's owner list (launch: insert only).

## Paths

`tools/audit/src/**`, `.github/workflows/qualification.yml` (rows only), this spec. After #17 S2 merges.

## Decision record (2026-10-24, attempt 1)

- Scope note wins for paths: `scripts/run-aarch64-tests.sh` (where the AArch64 rows live), `tools/audit/Cargo.toml` and `Cargo.lock` (the dep edge) are added. Root authorized one audit-native step in `qualification.yml`. Lands after #1920 on the same branch.
- One subject, both shapes: the native-width bank (internal detectors; the bank binds only unconnected) and a connected scalar (external detectors). No test for the SUBJECTS table. Console eligibility is untouched.

## Evidence (2026-10-24, attempt 1)

- `audit dynamic-eq` (x86-64-v3, release): `bank_width 8`, `bank_available true`, `total_violations 0`. Four enabled bands at 0.5-2 kHz, +3 dB static gain, -40 dB threshold, 4:1, 12 dB range, so no band rests in the identity skip. One-off peak check (not committed): 0.049 scalar / 0.058 bank against 0.522 with the threshold at 0 dB, so the dynamics run.
- Test value: an allocation in `process_bank_inner` aborts the audit (exit 134, no record); the conformance test measures the scalar instance one block at a time and never the bank.
- AArch64 row: no local arm64 runner or qemu; CI's `aarch64-release` proves it.
