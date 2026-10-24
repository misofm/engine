# Pin the de-esser's render kernels and add its audit and AArch64 rows

Filed by root as #15's slice S3 (backlog sprint, 2026-10-24).

## Smallest closable slice

1. Add the de-esser's render kernels (dual and mono-collapsed bank bodies) to the web KERNEL_ROSTER in `scripts/check-web-audioworklet-callgraph.py` with pinned ceilings (never loosen a row), and to the exact-symbol table #1889 adds if that has landed.
2. Add an `audit de-esser` subcommand row and its CI and AArch64 rows, as the shipped effects have.
3. Console eligibility (decision 12's list) only if the owner says yes; the question is on root's owner list.

## Paths

`scripts/check-web-audioworklet-callgraph.py`, `tools/audit/src/**`, `.github/workflows/qualification.yml` (rows only), this spec. After #15 S2 merges.

## Decision record (2026-10-24, attempt 1)

- Scope note wins for paths: `scripts/run-aarch64-tests.sh` (where the AArch64 rows live), `tools/audit/Cargo.toml` and `Cargo.lock` (the dep edge) are added. Root authorized one audit-native step in `qualification.yml`. Root also authorized `docs/ENGINE_ENV_VOCABULARY.md` (its rows for the new strace markers, which `check-env-vocabulary.sh` requires).
- The exact-symbol table (#1889) already pins the de-esser's five symbols; this issue adds only the two `KERNEL_ROSTER` rows (`run_block` closures `KB` / `Kb1_`, the dynamic EQ's shape), ceiling 0.10 (measured scalar 0, the table rule's floor).
- One connected scalar plus the native-width bank (the bank binds only unconnected). Console eligibility is untouched (owner question on root's list).

## Evidence (2026-10-24, attempt 1)

- Named twin (`build-web-audioworklet.sh --module-only --named-twin`, pinned nightly): de-esser dual 561 vector / 0 scalar, collapsed 283 / 0. `check-web-audioworklet.sh` passes on a fresh build; analyser `--self-test` passes. The roster goes from 15 rows (main) to 17, plus the collapsed limiter's forwarding rule; the prose counts were correct on main and now read seventeen / eighteenth / sixteen of eighteen with zero scalar (the two parametric-EQ rows carry 1 scalar each).
- `audit de-esser` (x86-64-v3, release): `bank_width 8`, `bank_available true`, `total_violations 0`. Parameters: 1 kHz band, -40 dB threshold, 8:1, 20 dB range; a one-off peak check read 0.034 against the tone's 0.251, so the dynamics run (not committed).
- Test value: an allocation in `process_bank_inner` aborts the audit (exit 134, no record); no other gate counts the de-esser's allocations.
- AArch64 row: no local arm64 runner or qemu; CI's `aarch64-release` proves it.
