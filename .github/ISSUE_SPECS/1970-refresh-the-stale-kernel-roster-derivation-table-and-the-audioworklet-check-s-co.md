# Refresh the stale KERNEL_ROSTER derivation table and the AudioWorklet check's count table

Filed by root from #1920 (backlog sprint, 2026-10-24).

## Defect

The derivation table in `scripts/check-web-audioworklet-callgraph.py` and the prose count table in `scripts/check-web-audioworklet.sh` give vector/scalar counts that no longer match the named twin. For example the compressor measures 949/0 against a listed 267/0, and the parametric EQ measures 1352/1 against a listed 168/0.

## Smallest closable slice

1. Measure every roster row once on the named twin (`build-web-audioworklet.sh --module-only --named-twin`, pinned nightly) and update both tables.
2. Check that each ceiling still follows the table rule. For the parametric EQ's one scalar op, name it and confirm it is within its ceiling. Do not raise a ceiling to make a row pass: report it.
3. Docs and prose only, plus the ceilings that the rule derives. No kernel change.

## Paths

`scripts/check-web-audioworklet-callgraph.py`, `scripts/check-web-audioworklet.sh`, this spec.
