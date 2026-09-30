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

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-30, on `f00d1aa4` (test `83194805`). One scratch `target/`, `CARGO_INCREMENTAL=0`,
on a host shared with another agent's benchmark (load 3 to 14), so wall times are indicative.

### Cause, confirmed independently

A temporary probe, since reverted, read each plan's `output_latency` at all three widths and found
the first nonzero output sample in a 48-block scalar render.

| Seed | Latency, scalar = simd4 = simd8 | First audible sample |
|---|---|---|
| 1192 | 2,461 | 2,361 |
| 1554 | 2,430 | 2,372 |
| 2045 | 2,430 | 2,372 |
| 2609 | 2,430 | 2,346 |
| 2804 | 2,461 | 2,377 |
| 5338 | 2,430 | 2,328 |

All six are `Free`, with four or five limiters on a track. The old window ends at 2,048. Across all
6,400 seeds:

- 138 plans exceed 12 blocks, and 9 exceed 2,048 samples. Exactly these six start after 2,048.
- Latency never differs by width.
- Onset precedes the reported latency by 29 to 102 samples. This is the limiter's pre-ringing, so
  the three 2,068-sample seeds (235, 3882, 4340) reached the old window with 9 audible samples.

### Could the fix hide a defect? No.

- **Superset.** The window only grows, and it always starts at sample 0. Every sample the old
  window compared is still compared.
- **Latency source.** Latency is read from the compiled artifact
  (`artifact.report().output_latency`), not recomputed. Each render uses its own plan's report. If
  two widths reported different latencies, their PCM lengths would differ and the seed would land
  in `moved`: red, not masked.
- **Tallies.** Identical to the base at both 6,400 and 64 seeds, so the `collapsed > 0` reach gate
  gained nothing from longer renders.
- **Margin.** `+4` guarantees at least 512 samples past the latency. That is what the fixed window
  already gives a plan at 12 blocks. The alternative, a full 16 blocks, cost 10 s per PR.
- **Budget.** The six seeds added no measurable time.

| 6,400 seeds, release, nightly env | Base `aa1338d3` | Fix |
|---|---|---|
| Default | 45.1 s, red `[1554]` | 42.4 s (44.1 s on another run), green |
| `RENDER_ALL=1` | 73.1 s, red on all six | 58.3 s, green, all 6,400 audible |

The nightly job's timeout is 45 minutes.

### Gates

| Gate | Result |
|---|---|
| 1. 6,400 seeds, nightly env | Green. Misaligned `[0, 2698, 1906]`, rendered 2,708, collapse 318, longest latency 2,461. By-shape counts equal the base. |
| 2. 64 seeds | Green in release (0.89 s) and debug with `graph/test-support` (23.0 s). All 9 tests pass. Tallies `[0, 34, 22]`, rendered 34, collapse 5, by shape all equal the base. |
| 3. Planted: source gain times 0, 64 seeds | Red, all 34 rendered seeds listed |
| 3. Fix reverted (the base) | Red, `[1554]`, and all six under render-all |
| 3. Sol's mutation: window = `ceil(latency / 128) - 1` blocks | Red at 6,400 via `[1554]` only. Under render-all: `[1554, 2045, 2609, 5338]`. 1192 and 2804 escape because their onset precedes the latency. Green at 64 seeds. |
| 4. `cargo fmt --all --check`, and clippy `--all-targets --all-features -D warnings` | Green |

### Test value

The nightly probe now goes red on a banking or armed-collapse defect that moves bits only in a
deep-PDC plan: output after the 16th block, or past a plan's first few output samples. Before the
fix, such a plan compared zeros (seed 1554) or 9 pre-ringing samples (seed 235). In nightly's
default mode, 39 rendered seeds now get a longer window.

### Findings

- **H:** none.
- **M:** none.
- **L1. The `+4` margin is chosen, not tested.** Setting `AUDIBLE_BLOCKS = 0` stays green at 6,400
  seeds with render-all. The vacuity gate needs only one nonzero sample, so the margin could erode
  unnoticed. It is justified above; a follow-up could assert a minimum count of audible samples.
- **L2. One doc comment is inexact.** `AUDIBLE_BLOCKS`'s comment says the plan "renders silence"
  before `output_latency`. Measured onset is 29 to 102 samples earlier, because of pre-ringing.
- **L3. Two prose claims are inexact; neither is material.**
  - The evidence says "as few as 104 audible samples." Seed 235, which nightly renders, had 9.
  - The evidence says per-PR renders are unchanged. At 64 seeds, seed 50 (latency 1,582) now
    renders 17 blocks. The cost is unmeasurable.
