# Make the randomized console differential non-vacuous at the nightly seed count

The nightly workflow has failed since `d42db36b` (2026-09-29), in its "workspace release link proof"
job.

## Problem

#1051 made the nightly workflow run every committed randomized differential at 100 times its
per-PR seeds (`.github/workflows/nightly.yml:173-184`). For graph-compiler's #966 probe,
`bank_levels::randomized_consoles_compile_bind_and_render_the_scalar_bits`, that means
`PROBE_966_COUNT=6400` instead of 64.

At 6,400 seeds the probe fails its vacuity assertion (`crates/graph-compiler/tests/bank_levels.rs`,
`silent.is_empty()`):

```
rendered silence, so compared nothing: [1554, 2045, 2609, 2804, 5338]
```

No bits moved, no bind refused and no compile refused. The five seeds generate sessions whose
scalar render is silent, so their banked renders compare nothing. At 64 seeds the case never
occurs. This is a generator defect exposed by the larger seed count, not an engine defect. The
failing runs are https://github.com/misofm/engine/actions/runs/36551347341 and the 2026-09-30
run on `398e8988`.

## Smallest closable slice

1. Find why each silent seed renders silence (for example every track muted, zero gain, a silent
   source, or a route to nothing) on the current generator, which S1a (#1093) ported to the
   `console`/`inserts` schema.
2. Fix the generator so that every seed renders audibly, keeping its reach: the shapes, levels
   and banking cases it covers today.
3. Only if the generator cannot avoid silence without losing reach: skip silent seeds, and assert
   that audible seeds dominate (for example at least 99% of seeds, and never zero). Record why.

Authorized paths: `crates/graph-compiler/tests/bank_levels.rs` and this spec.

## Objective gates

1. `PROBE_966_COUNT=6400 cargo test --locked --release -p graph-compiler --test bank_levels --
   --exact randomized_consoles_compile_bind_and_render_the_scalar_bits` passes, with nightly's
   environment (`CARGO_PROFILE_RELEASE_PANIC=unwind`, `MISO_ENGINE_RANDOMIZED_SCALE=100`).
2. The per-PR count, 64, still passes, and the printed shape and level tallies reach at least what
   they reach today.
3. Vacuity is still enforced: a planted change that makes every render silent turns the test red.
4. `cargo fmt --all --check`, and clippy `-D warnings` on the crate's tests.

## Non-goals

- No engine change.
- No change to the nightly workflow.

## Attempt 1 evidence

Terra, 2026-09-30. Branch `codex/1082-nightly-silence` from `aa1338d3` (C3 batch head, with S1a's
generator port). Commit `83194805`, then this record. Only `crates/graph-compiler/tests/bank_levels.rs`
changed.

### Cause

Reproduced on the base with nightly's environment: red, `rendered silence, so compared nothing:
[1554]`. S1a's port left one silent seed among the rendered ones. With `PROBE_966_RENDER_ALL=1`,
six of the 6,400 seeds render silent: 1192, 1554, 2045, 2609, 2804 and 5338.

The sessions are not silent. The render window is too short:

- All six are `Free` shape. That shape draws up to 9 random effects per track, and one path in
  each chains four or five true-peak limiters, at 486 samples each at 48 kHz.
- PDC puts their output at 2,430 or 2,461 samples (`GraphCompileReport::output_latency`). The
  fixed 16-block window of 128 frames ends at 2,048.
- Seed 1554 renders audio from 19 blocks on.

Of all 6,400 seeds, the latency histogram tops out at 20 blocks. 9 seeds exceed 2,048 samples. The
three at 2,068 samples still reach the window, so they were not silent.

### Fix, and why not the generator

`compile_bind_render` renders `BLOCKS`, or `AUDIBLE_BLOCKS` (4) past the plan's output latency if
that is more.

- A plan whose output arrives within 12 blocks renders the same 16 blocks as before. That covers
  every reproducer, whose latency is 486, and all but about 140 of the 6,400 seeds.
- Scalar and SIMD renders of one seed get the same window, because PDC does not depend on the
  width.
- The probe now prints the longest output latency it met.

Fixing the generator instead would mean capping latency-bearing effects per path, sidechain
alignment included. That removes the deep-PDC plans from reach. The window fix leaves the generator
and its tallies untouched. No silent-seed allowance was needed: every seed is audible, render-all
included.

I also tried rendering a full `BLOCKS` past the latency, and rejected it. It raised the per-PR
debug probe from 23 s to 33 s on this host, to buy extra audio for seeds that were already audible.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. 6,400 seeds, nightly env | Tallies identical to the base: misaligned `[0, 2698, 1906]`, rendered 2708, collapse 318, same by-shape counts. Longest latency 2,461. 42.2 s (base 43.9 s, red). | green |
| 1. render-all, 6,400 | All 6,400 rendered and audible, collapse 778. 63.7 s (base 61.6 s, red on six seeds). | green |
| 2. 64 seeds | Release and debug (`--features graph/test-support`): tallies identical to the base (`[0, 34, 22]`, rendered 34, collapse 5, same by-shape counts). All 9 tests pass. Debug wall time 23.3 s (base 23.4 s). | green |
| 3. Planted all-silent source | `TrackSource` gain times 0, at 64 seeds: `rendered silence, so compared nothing` lists all 34 rendered seeds. | red |
| 3. Fix reverted | Fixed 16-block window, at 6,400 seeds, nightly env: `[1554]`. | red |
| 4. Lint | `cargo fmt --all --check`. `cargo clippy --locked -p graph-compiler --all-targets --all-features -- -D warnings`. | green |

**Test value.** No assertion changed, and the vacuity assertion `silent.is_empty()` is kept. The
window change catches a banking or armed-collapse defect that moves bits only in a plan whose output
arrives after 12 blocks: four or five limiters in series. Before, the probe compared only zeros for
such plans (seed 1554), or as few as 104 audible samples. It stays red on the revert at 6,400 seeds.
The vacuity assertion catches any change that silences every render: a dead source, a route to
nothing, or a generator that mutes everything.

Not run: nightly's `--workspace` release build, which takes 21 minutes. The probe does not depend on
workspace feature unification.
