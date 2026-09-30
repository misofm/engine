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
