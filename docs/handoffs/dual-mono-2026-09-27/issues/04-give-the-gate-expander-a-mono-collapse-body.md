# Give the gate/expander a mono-collapse body

Draft, not yet a GitHub issue. Class A. Base `6ca203f8`. Template for three sibling issues
(multiband compressor, soft clip, transient shaper), which follow the same shape.

## Problem

A mono-sourced track collapses to one plane only if **every** upstream slot of its bank chain has a
one-plane body (`BankChain::collapse_prefix_of`, `crates/rack/src/lib.rs:1919`). The gate/expander
banks (`crates/gate-expander/src/lib.rs:890-960`, `bank_impl!`) but keeps the trait defaults:
`supports_mono_collapse() == false` (`crates/effect-contract/src/lib.rs:1863`) and
`lane_channel_symmetry` / `channel_symmetry` false (`:1743`). So one gate anywhere in the strip
turns off the collapse for the whole chain, builtins included, and the track pools as stereo.

## Outcome

`PreparedGate<L, false>` implements the mono trio exactly as the compressor does
(`crates/compressor/src/lib.rs:975-1030`, `process_bank_inner::<MONO>`):

* `lane_channel_symmetry(lane)` and the scalar `channel_symmetry()`: bitwise comparison of every
  per-channel designed word and ramp field the kernel reads (list them in the doc comment, as the
  compressor does at `:660-700`);
* `supports_mono_collapse() -> true`;
* `process_bank_mono`: the dual body with the right plane never read; every cross-channel term (the
  `link_max` / `link_avg` detector link, `crates/gate-expander/src/lib.rs:421-422`) computed on the
  left plane read twice **in the dual operation order** (`0.5*|p| + 0.5*|p|` is not `|p|` for a
  subnormal `p`); per-lane right-channel reports duplicated from the left;
* `desymmetrize_channels`: copy the whole left per-channel state (`state[0]`, the `GateState` at
  `crates/gate-expander/src/lib.rs:339`, and every per-channel ramp) onto the right;
* `channels_agree`: bitwise comparison of the two channels' state.

## Authorized paths

`crates/gate-expander/src/lib.rs`, `crates/gate-expander/tests/mono_collapse.rs` (new), the crate's
`tests/MUTATIONS.md`, this issue's spec.

## Gates

* New `crates/gate-expander/tests/mono_collapse.rs` modelled on
  `crates/compressor/tests/mono_collapse.rs`: for each link mode (`DualMono`, `Maximum`, `Average`),
  `process_bank_mono` then `desymmetrize_channels` then `process_bank` renders the left plane, the
  left state and the reports bit-identical to an always-dual run on `L == R` input; include a
  subnormal-content case under `Average` and a `-0.0` case; a disengage in the middle of a hold or
  release phase.
* `channels_agree` is false after one dual block with different L/R words, true after
  `desymmetrize_channels`.
* A console-level check: a mono fixture track with a gate in `simd1` collapses (its cohort's
  `bank_collapse_counters()` rises) and its digest equals the forced-off arm's.
* Two recorded mutations: link computed as `|p|` (fails the subnormal case); `desymmetrize` skips one
  `GateState` field (fails the mid-hold disengage).
* `cargo test -p gate-expander`, fmt, clippy `-D warnings`.

## Non-goals

The sidechain-connected gate (never banks); the other three effects (sibling issues).
