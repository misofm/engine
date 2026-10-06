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

## Attempt record

### Attempt 1 (implementer, 2026-10-06)

Test code only; no `src` change outside `crates/conformance/src`.

**D1, home.** `conformance::ramp_endpoint` (`crates/conformance/src/ramp_endpoint.rs`, `pub mod` in
`lib.rs`). It is not behind a feature: the crate's other shared harnesses (`randomized.rs`,
`EffectDifferential`) are not either, and `realtime-audit` only forwards engine's render-audit
instrumentation, which this harness does not use. No `Cargo.toml` change and no new dependency
(`check-conformance-boundaries.sh` exit 0).

**The harness** holds `Section` (`Common`, `Channels`), `RampWord`, `Move` (with `whole`),
`FRAMES = 72`, the request (48 kHz `qualities[1]` row asserted, quantum 128, dual mono, sidechain
connected or not), the snapshot and ramp-word readers, the signal, the one-frame render with the
endpoint check after each block, the unclamped-law reach check, the one-block partition comparison,
and the native-width bank half. Entry points: `check_every_move(factory, &RampEndpoints, sidechain:
&[bool])` and `check_every_bank_move(factory, &RampEndpoints)`. Each admits the boxed factory into
a registry once (`randomized::admit`) and reads the tail-bound entry from it, the entry
`tail_bound_of` gave every request before. No effect is named in it.

Two changes of form, no change of reach: (1) the delay's `values_with` hook is the data field
`RampEndpoints::rest` (`&[(0, DELAY_TIME_MS)]`), applied before the move's own `start`, as the delay
copy did; a bank's resting lanes would rest at the same values (no banking effect sets any). (2) A
connected run of an effect without a sidechain port now panics instead of silently running
unconnected (the spec's hazard); the compressor's connected run is the only one and its port exists.

**Per crate (gate 3)** each `tests/ramp_endpoint.rs` keeps only: its module doc (render site,
effect-specific remarks, test value), `WORDS`, `MOVES` (unchanged: same words, starts, targets,
`whole` flags), the `ENDPOINTS` constant, the delay's `DELAY_TIME_MS`, and the two test functions
(one for the delay), each a single harness call with the factory and the sidechain list (compressor
`[false, true]`, every other effect `[false]`).

**Line counts.** Before: compressor 563, delay 407, gate-expander 500, multiband-compressor 591,
soft-clip 493, transient-shaper 498, true-peak-limiter 481 (3,533). After: 162, 95, 99, 179, 83, 90,
78 (786), plus the harness module 519; 1,305 in all.

**Gate 1.** Debug and `cargo test --locked --release -p compressor -p delay -p gate-expander -p
multiband-compressor -p soft-clip -p transient-shaper -p true-peak-limiter --test ramp_endpoint`:
pass, 13 tests in 7 targets, the same names and counts as before (`--list` before the change: two
per effect, one for the delay).

**Gate 2, mutation re-run.** Each mutant applied alone to the new tree by a scratch script, the
named crates' `--test ramp_endpoint` run in debug, the file restored (`git status` of every `src`
clean afterwards; the seven targets green after).

| Mutant (#1409 table) | Result on the shared harness |
| --- | --- |
| site 2 `next_value` back to `current += step` | red: compressor connected (threshold, frame 33, `0xc2a00001`); transient (attack amount, frame 48, `0x3f800001`); delay (feedback, frame 33, `0x3f733334`) |
| site 2 `advance_block` first word back to `current + step` | **green** on the delay (see below) |
| site 4 `advance_where` back to `add` | red: compressor unconnected (threshold, frame 33, `0xc2a00001`) |
| site 4 gather: target from lane `W - 1 - lane` | red: compressor bank test (threshold, frame 0) |
| site 5 gate prologue back to `add` | red: gate (threshold, frame 33) |
| site 6 `run_segment` back to `add` | red: multiband (low threshold, frame 33) |
| site 6 `Side::segment` target from lane `W - 1 - track` | red: multiband bank test (low threshold, frame 0) |
| site 7 `ramp_word` at `RAMPING` back to `value + step` | red: delay (feedback, final snapshot depends on the partition) |
| site 7 D5 choice inverted | red: delay (feedback, final snapshot) |
| site 8 drive back to `add` | red: soft clip (drive gain, frame 33, `0x427c620b`) |
| site 8 D5 choice inverted (`if !L::mask_any(L::mask_not(settled))`) | **green** (see below) |
| site 8 `target_vector` from lane `W - 1 - lane` | red: soft clip bank test (drive gain, frame 0) |
| site 9 limiter back to `add` | red: limiter (limit coefficient, frame 33, `0x3d6655c2`) |
| #1409 attempt 2, site 7 left damping `g` | red: delay (damping coefficient, final snapshot) |
| site 7 left feedback | red: delay (feedback, final snapshot) |
| site 7 right damping `g` | red: delay (damping coefficient, final snapshot) |
| site 7 right feedback | red: delay (feedback, final snapshot) |
| site 7 cross position | red: delay (cross feedback, final snapshot) |

The two green rows are green on the pre-move tree too: with the seven old copies restored from
`8fc41c5b0` and the same mutants applied, both tests pass. So the move loses no reach, but #1409's
table no longer describes `8fc41c5b0`:
- Soft clip's D5 choice changed shape after #1409 (#1452 undo 5): the settled copy now holds the
  three words instead of adding a zero step. Inverted, a ramping block holds its words at rest,
  which stays inside the endpoints and is partition- and lane-invariant, so an endpoint gate cannot
  see it (it is a frozen ramp, not an overshoot).
- The delay's `advance_block` first-word mutant: the first word of a block differs from the
  clamped one only on a frame where `current + step` passes the target; no gate-2 test of the
  delay reaches that on `8fc41c5b0`. Not investigated further (outside this slice: no new moves).
Both are open items for root, not defects of this slice.

**Gate 4 and the rest.** `cargo clippy --locked --workspace --all-targets -- -D warnings`: pass.
`cargo fmt --all -- --check`: pass. `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p conformance
--no-deps`: pass. `check-workspace-policy.sh`, `check-conformance-boundaries.sh`,
`check-realtime-policy.sh`: exit 0. No engine or render code changed, so `check-cross-targets.sh`
was not run.
