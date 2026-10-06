# Share one effect-ramp endpoint harness instead of seven copies

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by the decision-15 root coordinator's ruling (2) on *Keep every effect parameter
ramp inside its endpoints* (#1409, attempt 1 verdict NIT 5). Test code only; no engine code
changes. Code anchors verified on `codex/d15-stream-g` at `c7f7bbdfb`.

## Product outcome

The seven effects' ramp-endpoint gates (#1409 gate 2) run on one shared harness, so a fix to the
harness reaches every effect at once and the copies cannot drift apart. Every gate keeps the reach
it has today: each mutant that #1409's attempt record shows red stays red after the move.

## Context

- **The copies.** `tests/ramp_endpoint.rs` in `compressor` (555 lines), `delay` (393),
  `gate-expander` (492), `multiband-compressor` (576), `soft-clip` (480), `transient-shaper` (485)
  and `true-peak-limiter` (473). Each repeats the same harness (about 300-400 lines): the
  `Section`/`RampWord`/`Move` types, `FRAMES = 72`, `request` (the 48 kHz `qualities[1]` row,
  quantum 128, dual-mono link, sidechain connected or not), the one-frame-block render with the
  snapshot check after each block, the one-block partition comparison, and the native-width bank
  half (the move on the last lane, every other lane at the defaults). Only the moves, the payload
  word layout and a few effect-specific hooks (the delay's 1 ms time, the multiband `whole` flag,
  the sidechain) differ. The #1409 attempt 1 verifier found one doc defect ("first quality row" vs
  `qualities[1]`) present in all seven copies; attempt 2 fixed it seven times.
- **`effect-runtime`'s and `lane`'s `tests/ramp_endpoint.rs`** test the law (gate 1), not an
  effect; they are not copies and are out of scope.
- **A shared home already exists.** Every one of the seven crates dev-depends on `conformance`
  (with `realtime-audit`), which already holds the shared effect test harnesses
  (`crates/conformance/src/randomized.rs`, `EffectDifferential`).
- **The mutation evidence** is #1409's attempt record table (`.github/ISSUE_SPECS/1409-*.md`,
  "Mutation evidence"; on GitHub #1409 once the spec leaves the directory): sites 2 and 4-9 with the
  test, move and frame each mutant is red on, plus attempt 2's delay site-7 rows (left feedback,
  left damping, cross position).
- **CI.** The seven targets run in debug in the workspace test jobs and, since the #1409 batch
  follow-up, in release in `qualification.yml`'s `test-release` job.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator filed this slice: one shared
  harness, each test keeping its mutation evidence.
- **D1. Home.** A public module in `crates/conformance` (for example `conformance::ramp_endpoint`),
  behind the feature the seven crates already enable, unless a measured reason (a dependency the
  harness needs that `conformance` must not take) puts it elsewhere; the record states the reason.
- **D2. Per-effect data stays in each crate.** Each `tests/ramp_endpoint.rs` keeps its moves, its
  `RampWord` layout and its effect hooks, and calls the harness. The harness takes the factory and
  the hooks as arguments; no effect name appears in it.
- **D3. No reach lost.** Same moves, same frames, same snapshot and partition checks, same bank
  half at the native width. No tolerance or move changes.

## Deliverables

1. The shared harness module (D1) and the seven `tests/ramp_endpoint.rs` files reduced to their
   per-effect data and calls.
2. The attempt record: every mutant of #1409's mutation table (and attempt 2's delay site-7 rows)
   applied alone on the new tree, the named test red, reverted green; the line counts before and
   after.

## Authorized paths

- `crates/conformance/src/` (the new module and its `lib.rs` export), `crates/conformance/Cargo.toml`
  only if D1 needs a feature line
- `tests/ramp_endpoint.rs` in `compressor`, `delay`, `gate-expander`, `multiband-compressor`,
  `soft-clip`, `transient-shaper`, `true-peak-limiter`
- This spec

## Non-goals

- New moves, new effects, the law tests (gate 1), the `randomized.rs` probes, any render code.

## Hazards

- Hot files: the effect crates' tests are touched by stream A's carry slices (#1279, #1280,
  #1282) only outside `ramp_endpoint.rs`; `crates/conformance/src/randomized.rs` is not edited.
- A harness made generic over the effect must not make a move silently vacuous (for example a
  sidechain hook that never connects): the mutation re-run is the check.

## Objective gates

1. The seven targets pass in debug and in release
   (`cargo test --locked --release -p compressor -p delay -p gate-expander -p multiband-compressor
   -p soft-clip -p transient-shaper -p true-peak-limiter --test ramp_endpoint`), with the same test
   names and counts as before.
2. Every mutant of #1409's mutation table that names a gate-2 test (sites 2 and 4-9, the bank-lane
   mutants, attempt 2's delay site-7 rows) is red on the same test after the move, and green after
   revert; recorded per row.
3. No harness logic is repeated in the seven files (the record lists what stays per crate).
4. `cargo clippy --locked --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`
   pass.

## Test value

No new test behavior. A plausible defect this guards: a harness fix (or defect) applied to one copy
only, which leaves another effect's gate weaker without any red; after this slice there is one
copy. Gate 2's re-run proves the move kept each gate's catches.

## Dependencies

- #1409 and #1411 (merged with the stream G batch).
