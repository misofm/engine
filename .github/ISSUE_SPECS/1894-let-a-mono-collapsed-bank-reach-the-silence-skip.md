# Let a mono-collapsed bank reach the silence skip

Filed by root from the #1871 verifier's finding (backlog sprint, 2026-10-24).

## Product outcome

While a bank lane is mono-collapsed, its right side is frozen. The bank's `lanes_at_rest` takes `sides[0].rest_lanes() & sides[1].rest_lanes()` (`crates/multiband-compressor/src/lib.rs` near line 1941; the gate-expander has the same shape), and `advance_clocks` asserts the same AND. So during a collapsed run the rack-level silence skip fires only if the right side was already at rest when the collapse began. The answer is a subset of the true one, so it is sound, but a collapsed lane that falls silent keeps processing (memory: skip work on silence).

## Smallest closable slice

1. In every mono-collapse effect (gate-expander, multiband, soft clip, transient shaper and any other on the mono-collapse contract), answer `lanes_at_rest` and advance the clocks from side 0 alone for a collapsed lane, and keep the disengage copy as the one place side 1 is made equal again.
2. State the rule once in the shared mono-collapse contract doc in `crates/effect-contract`.
3. One test in the owning crate of the shared contract (or one per effect if the logic is per effect): a collapsed lane that falls silent is reported at rest and skipped, and on disengage it renders the never-collapsed bits.

## Order

After #1871, #1872 and #1873 merge (same files).

## Paths

The mono-collapse effects' `src/lib.rs` and tests, `crates/effect-contract/src/lib.rs` (doc), this spec.

## Decision record

- Scope (root, 2026-10-24, from the scope note): two PRs under this issue. PR 1: the effect-contract
  doc, builtins input stage, parametric-eq, compressor, gate-expander, multiband-compressor and two
  rack doc passages. PR 2: soft-clip, transient-shaper, true-peak-limiter, de-esser, dynamic-eq.
- Rule (root revises its own #1107 D3 reading, stated once in `PreparedNativeEffectBank::lanes_at_rest`):
  each stage holds a `collapsed` flag, set by its rendered mono body and cleared only by its
  disengage copy. While set, `lanes_at_rest` answers from the left channel and `advance_clocks`
  advances the left clocks only. No trait change; the rack needs no code change. Class A.

## PR 1 record (attempt 1)

- Five stages carry the flag: builtins `InputStage` (already had it; now read by the predicate and
  the silence-counter advance), EQ, compressor, gate (`rest_lanes`) and multiband (`Instance`;
  `advance_silence` takes `live_sides()`). Two test-module struct literals (`rest_tests.rs` in EQ
  and gate) gained the field.
- Gate per crate, in its existing `tests/mono_collapse.rs`: dual signal, collapsed signal,
  collapsed `+0.0` until `lanes_at_rest() == all`, `advance_clocks` for three blocks the reference
  renders, disengage, payload (builtins: state and silence words) and dual-block bit compare
  against the never-collapsed bank. Mutant "AND the frozen right again": red in all five crates
  (the rest bound is never reached). Mutant "advance no counter": red in builtins, EQ, multiband.
- Class A A/B (PR evidence, scratch harness on the compiled-plan path of
  `graph-compiler/tests/silent_bank_skip.rs`, since `console-workload` cannot feed a silent span
  after a signal span): mono bank dual with signal for 30 blocks (collapse forced off), collapsed
  signal to block 60, then silence to block 400. Output digest identical on main and branch
  (`dee65cc39e65c522`), skip-vs-forbidden bit-equal every block on both; silent skips 400 on main,
  672 on the branch.
- Attempt 2 (verifier finding): each gate now renders, after the disengage, one dual block with a
  `+0.0` left plane and a live right one and asserts `lanes_at_rest() & all == 0`. Mutant "drop
  `collapsed = false` in desymmetrize": red in all five crates (builtins by its dual-after-mono
  debug assertion). The compressor gate runs dual mono: an Average-linked detector moved the left
  channel too and hid the stuck flag.

## PR 2 record (attempt 1)

- Five more stages carry the flag, set by the rendered mono body and cleared only by the disengage
  copy: soft clip (`SoftClip`; a skip advances the left cursor alone), transient shaper (`Shaper`;
  no clocks), de-esser and dynamic EQ (`advance_clocks_masked(.., 1)` while collapsed), and the
  true-peak limiter (`LimiterCore`: the scan, the #182 S2 claim's ramp legs and the phase advance
  read the left channel alone). No effect was left out: in each, every right word the predicate
  reads is stale while collapsed and overwritten by the copy, so no rendered bit moves (class A).
  `transient-shaper/src/rest_tests.rs` gained the field in its struct literal (outside the named
  paths, as PR 1's two `rest_tests.rs`).
- Gate per crate, the PR 1 form, in `tests/mono_collapse.rs` (de-esser, dynamic EQ: `tests/bank.rs`).
  The limiter's run carries one ceiling and release retarget while collapsed, so the right ramps
  freeze in flight. Mutants: "AND the frozen right" red in soft clip, shaper, de-esser, dynamic EQ;
  in the limiter only both legs together (scan and claim) are red, since the claim is the scan's
  cached answer. "Skip advances no clock" red in de-esser, dynamic EQ, limiter (phase); green in
  soft clip, whose cursor no payload word or output bit can see on a ring of `+0.0`. "Disengage
  keeps the flag" red in all five.
- Class A on the compiled-plan path: `graph-compiler/tests/silent_bank_skip.rs` gate 1 (skip allowed
  against forbidden, bit-equal every block) runs the limiter in its collapsed console bank and
  passes. No main-against-branch digest was taken for PR 2.
