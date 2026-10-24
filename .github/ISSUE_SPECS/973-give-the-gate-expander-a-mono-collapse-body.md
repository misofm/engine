# Give the gate/expander a mono-collapse body


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A. Base `6ca203f8`. Template for three sibling issues
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
* A console-level check: a mono fixture track with a gate as a `console.pre_insert` slot (the
  lowered first rack, once `simd1`) collapses (its cohort's `bank_collapse_counters()` rises) and
  its digest equals the forced-off arm's.
* Two recorded mutations: link computed as `|p|` (fails the subnormal case); `desymmetrize` skips one
  `GateState` field (fails the mid-hold disengage).
* `cargo test -p gate-expander`, fmt, clippy `-D warnings`.

## Non-goals

The sidechain-connected gate (never banks); the other three effects (sibling issues).


## Decision and evidence record (attempt 1, 2026-10-24)

* Scope note `scope/973.md` accepted by root. Paths: `src/lib.rs`, `src/kernel.rs`,
  `tests/mono_collapse.rs` (new), `tests/MUTATIONS.md`, this spec. The spec's anchors are stale.
* Also fixed: the scalar `channel_symmetry` was the default `false`, so the compile pooled a mono
  track with a gate as Stereo. It now returns the same `DESIGNED` term as the bank, connected or
  not (the term speaks for designed words only, as the compressor's; the randomized differential
  with `witness: true` holds on its connected mono scenarios).
* Kernel: one body, `gate_frames::<L, CONNECTED, RAMPING, MONO>`; `gate_block` is its dual
  instantiation, so `corpus.rs` is unchanged. `MONO` reads and writes the left plane only and calls
  the dual `channel_step` with the left word as own, partner and dry.
* D7 in a collapsed block: the left plane decides, and a failed lane is re-seeded on both
  channels, as the dual block on `L == R` does. The right `timing` is each channel's own committed
  values and the disengage copy does not carry it, so a left-only re-seed left the right payload's
  attack time stale (found red by the new test). Reports duplicate left onto right.
* `desymmetrize_channels` copies `state[1] = state[0]` and `coef[1] = coef[0]`; `channels_agree`
  compares those words bit for bit (31 lane words per channel, so no fixed point is needed).
* Root decision: the spec's console-level gate is dropped (tests-overboard); the rack chain and the
  randomized differential's collapsed arm already cover the bank surface.
* Class A evidence (PR, not committed): a scratch digest of the dual scalar and native-bank output,
  reports and payloads over three link modes, two holds, asymmetric channels and automation was
  `77de805f037e2064` on base `3b5740cdd` and on this branch, debug and release.
* Wasm: `process_bank` keeps 162 `f32x4` operations (base 162 vector / 8 scalar, branch 162 / 0;
  the roster table's 180 / 8 was an older build). The drop to 0 scalar is codegen drift, fewer
  operations: the dual source path is unchanged (`gate_block` is `gate_frames::<.., false>`);
  `process_bank_mono` is 82 / 0. The roster row `gate_expander.*4wide6f32x4` now matches two
  kernels, so it must split into dual (`12process_bank`) and collapsed (`17process_bank_mono`)
  rows in `scripts/check-web-audioworklet-callgraph.py` (root authorized it, with the dual
  ceiling set to 0.10 by the four-times-ratio rule, measured ratio 0).
* Root also authorized `tests/randomized.rs` `witness: true` and the effect-contract
  `channels_agree` note. The apart arm of the transition test makes the channels differ in the
  hold countdown alone, so the `channels_agree` word list is defended.
