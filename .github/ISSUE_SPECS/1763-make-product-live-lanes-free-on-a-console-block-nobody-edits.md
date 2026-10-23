# Make product live lanes free on a console block nobody edits

Successor of #1758 (2026-10-23).

## Problem

#1758 measured the standing 64-track console prepared with the lanes a product host attaches (`controls::live_lanes(model)`) and never written: 110.69 / 111.07 us p50 per block, against 102.91 / 102.97 us for the same session with no lanes, in the same run (AVX2, uncontrolled, load about 7). That is +7.6 % / +7.9 %, with identical output bits over the same 1000 blocks and 0 transactions (`artifacts/steps/opt3-1758/`). Every browser and C ABI session pays this on every block.

## Candidate stages (code read only, unmeasured)

1. The bypass shunt. An effect bank stage with a live channel on any lane gets one `BypassShunt` per bank (`crates/rack/src/lib.rs` near 1247-1258). For an effect with latency, `capture` runs on every block, bypassed or not (`crates/rack/src/lib.rs` near 1944): two plane copies and a PDC delay-line exchange for the bank (`crates/effect-contract/src/live.rs`, `BypassShunt::capture`). In the standing console only the true-peak limiter has latency (`LatencySamples(lookahead_maximum + 6)`), so only its shunts capture; the EQ and compressor shunts skip it. The no-lane plan has a shunt only where an effect is bypassed at preparation.
2. Per-lane control walks: the live effect bank stage's drain, target and bypass-mix loops (`crates/rack/src/lib.rs` near 1808-2061), and the strip input, fader and matrix banks' per-lane dirty-word loads. No bank skips an unchanged snapshot as a whole.

## Smallest closable slice

1. **Split first (record in this spec before any engine change).** Add arms to the #1758 row that attach one lane group at a time (effect lanes only; strip input lanes only; fader/matrix lanes only), with the same digest check. One descriptive run. Name the stage(s) that carry the gap.
2. **Then remove the named cost** with a mechanism that keeps every rule: bypass preserves latency, a bypass edge crossfades bit-exactly as today, and live values converge as decision 15 requires. Candidates, not rulings: keep the bypass line's dry history without a per-block copy while no lane is bypassed or ramping (for example, start it one latency length ahead of a bypass edge, or derive it from the effect's own input history); one per-bank "nothing dirty" word so a clean bank skips its per-lane walk.
3. If the mechanism needs a contract change in the bypass semantics, stop and report; root rules.

## Authorized paths

- `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs` and the console-benchmark scripts (the split arms and their record count)
- `crates/rack/src/lib.rs`, `crates/effect-contract/src/live.rs`, `crates/builtins-compiler/src/lib.rs` (only the stage that step 1 names)
- this spec

## Objective gates

1. The step 1 split, recorded before any engine change.
2. Class A: no rendered bit moves, on the console benchmark's digests and the existing bypass/latency tests (one-time base comparison as PR evidence).
3. Render allocates nothing; `tools/realtime-policy` passes.
4. One descriptive run of the #1758 row: the product-lane row is within 2 % of `sixty_four_track_console`'s p50, and the app-shape product-lane arm within 2 % of `sixty_four_track_app_shape`'s, with equal digests. If either is not, record the remaining gap and its named reason.

## Test value

A test is owed only for a new mechanism: it turns red when the skipped work was needed (a bypass edge after a long unbypassed stretch renders other bits than today).

## Dependencies

- #1758 (the row).
- #1747 merged first (it edits `crates/rack/src/lib.rs` and `crates/effect-contract/src/live.rs`).

## Evidence and decisions

- Step 1, attempt 1 (2026-10-23). The #1758 row gains four arms of the same `console_product_lanes` shape, each timed by `SessionMeasurement::run_prepared` straight after the session rows, 0 transactions, and asserted in-run to render its `standing_row`'s bits over blocks 0..1000: `sixty_four_track_console_fader_matrix_lanes` (strip channels, no input lane), `..._strip_input_lanes` (strip channels with the input lane), `..._effect_lanes` (effect channels, no strip channel), and `sixty_four_track_app_shape_product_lanes` (the app shape's own product set). `SessionRuntime::build_with_lane_group` builds them, and the record's `live_lanes` is read from the attached channels. The record lib pins each kind to its lane set and standing row, and the aggregate holds the digest rule per standing row. Records 66 -> 74.
- The verifier's caveat: the app shape (EQ -> compressor, no limiter, 21 of 64 tracks bypassed at preparation) has no console slot with latency. It got one arm, its full product set, not a full split: its total is small (below).
- Descriptive run (`run-console-benchmark.sh --step opt4-1763` at d7854800b, one warmup and two rounds, exclusive lock, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` because the load was 17 at build time; loadavg at the start of the run 1.96, cpu 31): `artifacts/steps/opt4-1763/`. All 74 records validated. Digests equal per standing row (console 6bae1dd0..., app shape 3ce17e3e...).

| Arm (p50 per block, us) | Round 1 | delta | Round 2 | delta |
|---|---|---|---|---|
| `sixty_four_track_console` (no lanes) | 102.45 | | 102.24 | |
| fader/matrix | 102.77 | +0.32 | 102.61 | +0.37 |
| fader/matrix + strip input | 103.20 | +0.75 | 103.17 | +0.93 |
| effects only | 110.08 | +7.63 | 110.24 | +8.00 |
| product set (all three) | 111.35 | +8.91 | 112.09 | +9.86 |
| `sixty_four_track_app_shape` (no lanes) | 82.58 | | 82.96 | |
| app shape, product set | 84.69 | +2.11 | 84.85 | +1.89 |

- Reading: the effect lanes carry the gap (+7.6 / +8.0 us of +8.9 / +9.9). Fader/matrix and strip input together cost +0.75 / +0.93 us (under 1 %). The groups add up to within about 0.5 / 0.9 us of the whole set. On the app shape, which has no latency, the whole product set costs +2.1 / +1.9 us (2.6 % / 2.3 %), so the EQ and compressor effect lanes there cost about 1.2 us (an estimate by subtraction that assumes the console's strip-side cost carries over, not a bound). Inference, not measured: most of the console's effect-lane cost belongs to the true-peak limiter's banks, which is consistent with the bypass shunt's per-block `capture` (only a slot with latency feeds the line when no lane is bypassed). No per-stage profile: `perf record` does not run on this host.
- Proposed mechanism for step 2 (no projected saving): when no lane of the bank needs dry (`any_bypassed` false, `crates/rack/src/lib.rs` near 1917), `dry_*` has no reader in that block (the mix at near 2043 is skipped), so `capture` can feed the latency line write-only: store the input words at the cursor and advance it, with no copy into `dry_*` and no read of the old words. The line then holds exactly the words the exchange leaves, so a later bypass edge reads the same delayed dry and no bit moves. This needs no change to the bypass semantics. Alternative, which needs an effect-contract addition and so a root ruling: read the delayed dry from the limiter's own input history. If the remaining gap after step 2 is still above 2 %, the next candidate is a per-bank "nothing dirty" skip of the effect bank's per-lane control walk.

## Step 1 verifier and the step 2 scope (root, 2026-10-23)

- Step 1 verifier: PASS. Root added the one missing validator case (a console arm read against the
  app-shape row). The effects-only arm is a decomposition arm built from production entries, not a
  configuration a host prepares (host-core always attaches fader/matrix lanes, `prepare.rs` near
  1640).
- Step 2 scope: two mechanisms, because the app shape's +2.3-2.6 % is not the shunt (its banks do
  not feed a line, `feeds_line()` is false):
  1. the latency-line feed without the dry copy while no lane is bypassed. The verifier showed it
     bit-exact (`pdc_delay_block` is a swap; `dry_*` is read only by `crossfade_bank` in the same
     block). Conditions: write exactly as the chunked exchange does when `frames` exceeds the line
     length; a collapsed block feeds `block.left` into both lines; the per-node path keeps
     `capture`.
  2. a per-bank "nothing dirty" skip of the live effect stage's per-lane walk (drain, prepared
     targets, the `needs_dry` scan, `advance_bypass_mix`; `crates/rack/src/lib.rs` near 1808-2060),
     and the same for the strip input, fader and matrix banks if their walk is measurable.
  Reading the delayed dry from the limiter's own history would need an effect-contract addition and
  a root ruling; it is not in scope.

## Step 2, attempt 1 (2026-10-23)

- Mechanism 1: `BypassShunt::feed_line` (`crates/effect-contract/src/live.rs`) stores a block's
  input in the latency line, with no dry staging, at the positions and cursor the exchange in
  `capture` leaves (the newest `length` words when the block is longer than the line). The live
  effect bank stage calls it on a block no lane reads dry, and `capture` otherwise; a collapsed
  block feeds `block.left` into both lines. The per-node path still calls `capture`.
- Mechanism 2: the live effect bank stage's drain (`crates/rack/src/lib.rs`) skips its per-lane
  cell walk when the last walk ran under the same snapshot publication and found the bank quiet
  (no span, no prepared target, no running crossfade). Exact by the gate's guarantee: every write
  of a publication at or below `S` is stored and marked before that publication, so the walk under
  `S` applied it, and a write above `S` waits in its dirty word or the deferred mask for the first
  drain under a newer snapshot. `LiveSnapshot::ALL` never skips. The walk records three verdicts
  (`LaneWalk`: targets, needs_dry, ramping) and the process body reads them instead of walking
  the lanes again. A lane swap and the carry paths clear the quiet mark. The observation queues are
  still drained every block: the push is the acknowledgement (#1584 D6a), and no publication
  follows it, so a skipped queue drain could hold an acknowledged subscription indefinitely.
- The strip input, fader and matrix banks (`crates/builtins-compiler`) are not changed.
- Class A: the 52 digests of the console benchmark (every session row, every product-lane arm,
  both fader-ride arms, the automation, hoist, meter, observation, placement and mono arms) are
  equal to the base 2724b2f58 in both measured rounds.
- Descriptive run (`run-console-benchmark.sh --step opt5-1763` at 45864d7a7, one warmup and two
  rounds, exclusive lock, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, loadavg 3.56 at the start):
  `artifacts/steps/opt5-1763/`. All 74 records validated.

| Arm (p50 per block, us) | Round 1 | delta | Round 2 | delta |
|---|---|---|---|---|
| `sixty_four_track_console` (no lanes) | 102.25 | | 102.97 | |
| fader/matrix | 103.70 | +1.45 | 103.05 | +0.08 |
| fader/matrix + strip input | 103.58 | +1.33 | 103.83 | +0.86 |
| effects only | 104.01 | +1.76 | 103.87 | +0.90 |
| product set (all three) | 105.30 | +3.06 (+2.99 %) | 104.78 | +1.81 (+1.76 %) |
| `sixty_four_track_app_shape` (no lanes) | 80.62 | | 80.19 | |
| app shape, product set | 82.99 | +2.36 (+2.93 %) | 81.95 | +1.75 (+2.19 %) |

- Gate 4 is not met in round 1 for either row, and in round 2 for the app shape. The effect-lane
  cost fell from +7.6 / +8.0 us to +1.8 / +0.9 us; the app shape did not move (+2.1 / +1.9 us
  before). Remaining gap, named reasons, not split by measurement (no profile is possible on this
  host): (1) the strip input, fader and matrix banks' per-lane dirty-word walk, untouched, which
  step 1 measured at +0.75 / +0.93 us; (2) the observation-queue drain, one producer-cursor load
  per lane per block, which cannot skip (above); (3) noise: the no-lane console row moved 0.72 us
  between the two rounds on a loaded host. The next candidate is (1), in
  `crates/builtins-compiler/src/lib.rs`, shared with #1765 and not started without root.
- The app shape did not move: +2.36 / +1.75 us after, against +2.11 / +1.89 us before. So step 1's
  estimate of about 1.2 us for its EQ and compressor lanes was not the cell walk, and the reasons
  above are plausible for the app shape but not measured.
- Verifier follow-up: the quiet-slot test (`rack/tests/lane_swap.rs`) gains a prepared-target step;
  the quiet mark without its `targets` term turns it red on the first quiet block after the target.
