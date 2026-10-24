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
