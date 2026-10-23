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
4. One descriptive run of the #1758 row: the product-lane row is within 2 % of `sixty_four_track_console`'s p50, with equal digests. If it is not, record the remaining gap and its named reason.

## Test value

A test is owed only for a new mechanism: it turns red when the skipped work was needed (a bypass edge after a long unbypassed stretch renders other bits than today).

## Dependencies

- #1758 (the row).
- #1747 merged first (it edits `crates/rack/src/lib.rs` and `crates/effect-contract/src/live.rs`).
