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
- Later (2026-09-28): #1039 retired the wasmtime console benchmark, including
  the `--issue183` arm (owner ruling R9, `engine-footprint-2026-09-28.md`), and
  R7 (#1038) removes the eight-lane wasm measurement build. Re-measuring now
  means restoring both from git history.

## Supersession note (2026-09-29, #1038): the measurement hooks are removed

Owner ruling R7 (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`) reverses the
"not foreclosed" bullet above, and #1038 carries it out. **The wasm build has one width, `Simd4`.
The eight-lane measurement cfg (`miso_wasm_simd8`) and its arms are removed. A future
re-measurement re-adds a cfg in its own issue.**

- **Removed:** the `miso_wasm_simd8` arm of `Backend::current()` (`crates/lane/src/backend.rs`),
  `target-smoke`'s eight-lane wasm assertion, the workspace `check-cfg` entry, and `wasm-gates`'
  `--expect-backend simd8`, which no wasm guest can report any more. #1039 had already retired the
  console benchmark and its `--issue183` arm.
- **Why:** the owner's 2026-09-28 rule that modes production never needs are removed entirely,
  including test-only entry points. The hooks kept nothing that worked: the paired capture could
  not re-run, because `artifacts/issue183/` exists and the runner refused to overwrite it, and the
  operator script's repository root was wrong.
- **Lost:** an ad-hoc `--cfg miso_wasm_simd8` leg, such as #976's bit-exact differential. A harness
  can still run `Simd8` on wasm through `lane::Simd8` directly, as `tools/wasm-gate-corpus` does at
  all three widths.
- **A stale flag is inert.** `RUSTFLAGS='-C target-feature=+simd128 --cfg miso_wasm_simd8'` now
  compiles without a warning, because no source names the cfg, and it selects `Simd4`: the gate
  guest built with it is byte-identical to the one built without it.
- **Kept:** the null result and its evidence above. Reopening now needs the cfg and a console
  harness restored as well as a fresh paired record.
