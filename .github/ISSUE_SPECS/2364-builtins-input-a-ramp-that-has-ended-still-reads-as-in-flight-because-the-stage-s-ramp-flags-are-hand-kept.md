# builtins input: a ramp that has ended still reads as in flight, because the stage's ramp flags are hand-kept

Tracker #2258, item GR-8 ("InputStage keeps hand-maintained derived caches"). Pattern 1 of the
tracker: a derived cache cleared by hand.

## Mission
`builtins::InputStage`'s two ramp flags, `ramping` (trim) and `filter_ramping`, must say exactly
what the authoritative countdowns (`remaining`, `filter_remaining`) say after every writer, with
one definition and one writer, checked against a fresh scan wherever a block boundary reads them.

## Verified (base `2aa2aa60c`, `crates/builtins/src/lib.rs`)
The flags were recomputed inline at eight sites with three different spans: trim over `L::WIDTH`
lanes of the channels just settled (`settle` 1830-1858) or of both channels (`import_lane`
2950); filter over member lanes and always both channels (`settle_filter` 1554,
`apply_prepared_filter` 1695, `import_lane` 2954); and `set_trim_signed` ORed the trim flag in
(1752). Two writers leave a flag **stale true** on reachable paths:

1. **A stepped trim or polarity record that ends the last trim ramp.** `set_trim_signed` sets
   `ramping = true` for a non-zero window and otherwise leaves it. A window of `0` -- an edit's
   explicit `0`, which `controls::LiveRamps::resolve` passes through ("an explicit 0 included,
   which steps that edit") -- ends the lane's countdown on the addressed channels but never
   clears the flag.
2. **A collapsed block that ends the filter ramps.** `process_mono` calls
   `settle_filter(prefix, 0..1)` (2187), which advances channel `0` only, but `settle_filter`
   publishes `filter_ramping` over both channels *before* `mirror_filter_ramp` copies channel
   `0` onto channel `1`, so it reads channel `1`'s pre-block countdown.

What a stale flag costs, until the next block's settle clears it: `lanes_at_rest` declines the
whole bank (a lost rest skip), and the next block runs a ramp body instead of the settled one (a
wasted ramp kernel, which also drives the kernel's `f32` countdown residue `ramp.remaining`
further negative; that word is reloaded from the `u32` at the top of every ramping block and
never read otherwise). **No rendered bit moves**: a ramp body over settled lanes is the settled
arithmetic. Measured below.

Not a defect: the `L::WIDTH`-versus-members span difference the tracker names is unreachable.
Every countdown writer is guarded to member lanes (`BuiltinInputBank` refuses `lane >= members`
before `set_trim_db`, `set_polarity_invert` and `apply_prepared_filter`; `import_lane` asserts
it), so a padding lane's countdown is always `0` and the two spans agree.

Not changed: `symmetry` already has the tracker's model (a debug assertion against a fresh walk
at its point of use, `lane_channel_symmetry`), and `plan` is a conservative permission refreshed
by every coefficient and state writer outside render; neither has a stale case found here.

## Decision / model
- **One definition:** `InputStage::countdowns_in_flight(channels) -> (trim, filter)`, a
  fixed-length masked OR over member lanes (no data-dependent branch).
- **One writer:** `InputStage::refresh_ramp_flags(channels)`, the only assignment of either flag
  after `new`, called by every countdown writer once it has written: `set_trim_signed`, `settle`,
  `settle_filter`, `apply_prepared_filter`, `import_lane` and `reset_with_kind`. A one-plane body
  passes channel `0` alone, because channel `1` holds the block's starting countdowns until the
  mirror (the collapsed-stage invariant); every other writer passes both channels.
- **Checked where read:** `debug_assert!(self.ramp_flags_are_current())` (both flags against a
  fresh scan of both channels) at the three block-boundary readers: `process` (after the
  disengage copy), `process_mono` and `lanes_at_rest`.
- The fields stay. A scan at the point of use would add about 6 ns per bank per block to the
  settled path (measured below), which is the path the trim flag exists to keep at "one `bool`
  test per bank per block". Keeping the fields also keeps `size_of::<InputStage<_>>()`, so no
  plan fixture or sealed byte count moves.

## Non-goals
- No change to the symmetry cache, the elision plan, the ramp kernels or any SIMD lane word.
- GR-4 (collapse mode) and GR-7 (ramp encodings) are separate tracker items.

## Gates
- Red-on-base regression tests, in `crates/builtins/src/tests.rs`:
  `a_stepped_retarget_that_ends_the_last_trim_ramp_leaves_the_stage_at_rest` and
  `a_collapsed_block_that_ends_the_filter_ramps_leaves_the_stage_at_rest`; and a guard for the
  two writers outside a block, `a_reset_and_a_lane_import_republish_the_ramp_flags`. All at every
  lane width, with padding lanes.
- Mutation record over every new writer site and the definition.
- `cargo test --locked` for builtins and its consumers (rack, builtins-compiler, graph-compiler,
  host-core); clippy `-D warnings`; `cargo fmt --check`; `cargo doc -D warnings`;
  `scripts/check-workspace-policy.sh`; `realtime_policy`.
- One-time no-bit-moved comparison against base (PR evidence, not a committed test).

## Evidence (attempt 1)

**Red on base** (base `2aa2aa60c` plus only the two regression tests):
```
---- tests::a_stepped_retarget_that_ends_the_last_trim_ramp_leaves_the_stage_at_rest stdout ----
panicked at crates/builtins/src/tests.rs:918:9:
assertion `left == right` failed: W=1: no countdown is in flight, so no ramp is
  left: 0
 right: 1

---- tests::a_collapsed_block_that_ends_the_filter_ramps_leaves_the_stage_at_rest stdout ----
panicked at crates/builtins/src/tests.rs:961:9:
assertion `left == right` failed: W=1 collapsed=true: every filter countdown ended, so no ramp is in flight
  left: 0
 right: 1
```
The dual control of the second test passes on base. `a_reset_and_a_lane_import_republish_the_ramp_flags`
is green on base (both writers recomputed in full there); it guards the single-writer refactor,
whose two out-of-block writers no other test reaches (M6 and M7 below survive every other test).

**Test value.**
- Stepped retarget: a writer that sets the flag without clearing it, or a definition that reads
  only the addressed channel, the addressed lane or the first member. No existing test steps a
  trim over a ramp in flight and then reads rest.
- Collapsed filter: a one-plane writer that publishes over a channel it has not yet mirrored. No
  existing test reads rest after a collapsed block ends a filter ramp.
- Reset and import: an out-of-block countdown writer that does not republish; the import
  direction (a lane carried mid-ramp into a settled stage) is the one that would drop a ramp.

**Mutations** (`cargo test --locked -p builtins --lib`, all on head):

| Mutant | Result |
|---|---|
| M1 `set_trim_signed` back to `if smoothing_samples > 0 { ramping = true }` | RED: stepped retarget |
| M2 `settle_filter` publishes over `0..2` instead of its channels | RED: collapsed filter |
| M3 definition reads only the first channel of its range | RED: stepped retarget, reset/import |
| M4 definition reads only lane `0` (`lane < 1`) | RED: stepped retarget (`W=4: the last lane ramps`), reset/import |
| M5 `settle` does not publish | RED: `rest_tests::an_input_lane_called_at_rest_stays_there_on_a_silent_block` (point-of-use assertion) |
| M6 `import_lane` does not publish | RED: reset/import only |
| M7 `reset_with_kind` does not publish | RED: reset/import only |
| M8 `apply_prepared_filter` does not publish | RED: rest probe, collapsed filter, reset/import, three filter tests |

**No bit moved.** A temporary (uncommitted) probe module drove `InputStage` at W=1, 4 and 8, 12
seeds each, 3000 blocks per seed: random members (padding present), random trim, polarity and
filter retargets on random selectors with windows `{0, 0, 17, 64, 200}`, lane exchanges through
`export_lane`/`import_lane`, rare resets, blocks of `{1, 17, 32, 64, 128}` frames with silence
runs, and a collapsed (`process_mono`) block whenever the stage was collapse-eligible (half the
time). It hashed (FNV-1a) every output sample and report count, every exported lane word except
the kernel's countdown residue, the residue separately, and `lanes_at_rest` per block. Release
build, base `2aa2aa60c` against head:
- output and exported state: **identical in all 36 runs** (rerun on base reproduced its hashes);
- countdown residue: differs in all 36 (base ran the stale ramp bodies), as expected;
- `lanes_at_rest`: head claims rest on 1521 of 108,000 block boundaries where base claimed none,
  and never withdraws a lane base claimed. The rest claim itself is defended by `rest_tests`.

The same probe in a debug build ran the three point-of-use assertions over all 36 runs: clean.

**Cost.** Release, best of 5 x 10^7 calls, on a loaded machine (load average about 15):
`countdowns_in_flight(0..2)` is 6.2 ns at every width. Base's inline scans, for comparison: trim
0.5-0.8 ns, filter 3.0 / 10.0 / 19.4 ns at W=1/4/8. The settled block path adds nothing in a
release build (the assertions are debug-only). A ramping block's settles now publish both flags
(at most two scans, about 12 ns, against ramp blocks of roughly 240-3300 ns here), and each
retarget adds one scan. Whole-block timings on this machine were too noisy to resolve the
difference (two head runs of the same trim-ramp loop gave 768 and 606 ns/block at W=4).

**Gates.** `cargo test --locked -p builtins`, `-p rack`, `-p builtins-compiler`,
`-p graph-compiler`, `-p host-core`: see the attempt record below. clippy
`-p builtins --all-targets -D warnings`, fmt, doc (`-p builtins`), workspace policy and
`realtime_policy` (298 marked regions in 48 files): clean. `size_of::<InputStage<_>>()` does not
change (no field added or removed), so `graph_fixture` was not regenerated.
