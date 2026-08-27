# Ruling: wasm stays four-lane — the Simd8 backend switch is closed as a null

**Owner ruling (2026-08-27, issue #183):** no backend switch, and no per-effect
width exception. The wasm guest keeps `Simd4` banks everywhere.

## The evidence the ruling rests on

Two sealed paired W4/W8 records, same fixture, same harness:

| console W8/W4 ratio | pre-round-2 (`artifacts/issue183/`) | post-round-2 (`artifacts/issue183-post-round2/`) |
|---|---|---|
| 64-track console | 0.894 | 0.948 |
| 128-track stretch | 0.890 | 0.946 |
| compressor_only | 0.97 | 1.03 |
| eq_only | 1.00 | 1.18 |
| builtins/dispatch_only | 1.00 | 1.15 |
| idle | 0.98 | 1.23 |
| nine_track_baseline | 1.21 | 1.42 |

The pre-round-2 record's −11% console win was, by row decomposition, almost
entirely the limiter's — and effect-loop round 2 (#198) removed the same cost
at W4 by moving the detector history out of linear memory into locals. What
remained of the W8 advantage after that is a −5% console win purchased with
regressions on every decomposition row and +42% on small sessions: the doubled
live-vector pressure under Cranelift's sixteen registers taxes every kernel,
and only the limiter still converts any width (~14 µs of its increment).

Two blockers, recorded in `docs/rulings/wasm-simd8-survey.md`, would also have
gated any switch: soft clip's `width_is_native` table silently falls back to
scalar at W8 on wasm32, and the wasm harness pins the four-lane backend name in
three places.

## What the ruling forecloses, and what it does not

- Foreclosed: the global backend switch, and a limiter-only (per-effect) width
  exception — the owner explicitly declined the added width machinery for one
  effect's ~14 µs.
- Not foreclosed: re-measurement. The `--issue183` bench arm and the
  `miso_wasm_simd8` opt-in cfg remain in the tree; a future engine (relaxed-SIMD
  FMA per #172, a register-richer baseline, a materially different kernel mix)
  can re-run the same paired capture and reopen with evidence. Reopening
  requires a fresh paired record on the then-current base — this ruling's
  numbers describe the round-2 tree, nothing later.
