# Pin the de-esser's render kernels and add its audit and AArch64 rows

Filed by root as #15's slice S3 (backlog sprint, 2026-10-24).

## Smallest closable slice

1. Add the de-esser's render kernels (dual and mono-collapsed bank bodies) to the web KERNEL_ROSTER in `scripts/check-web-audioworklet-callgraph.py` with pinned ceilings (never loosen a row), and to the exact-symbol table #1889 adds if that has landed.
2. Add an `audit de-esser` subcommand row and its CI and AArch64 rows, as the shipped effects have.
3. Console eligibility (decision 12's list) only if the owner says yes; the question is on root's owner list.

## Paths

`scripts/check-web-audioworklet-callgraph.py`, `tools/audit/src/**`, `.github/workflows/qualification.yml` (rows only), this spec. After #15 S2 merges.
