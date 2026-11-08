# Effect contract: a bank's lane defaults and its restore verdict are three loose trait methods that only docs pair

Successor of #2378 row C2(a), which came from #2324's follow-up. The issue is filed when #2378's bundle reached its bound.

## Problem
A render-time lane swap (`effect_contract::swap_bank_lanes`, `crates/effect-contract/src/live.rs` ~3377) needs three things from a `PreparedNativeEffectBank`:
- `lane_default_bytes`: the size of the lane's prepared defaults. `None` declines the swap.
- `exchange_lane_defaults`: the in-place exchange of those defaults.
- `validate_track_state_payload`: the restore's verdict, which the swap reads before its first write (#2324).

All three are separate trait methods with independent defaults: `None`, `false`, and `effect.state.unchecked`. Only the trait docs (`crates/effect-contract/src/lib.rs` ~2649 and ~3053) and the `effect-compiler` `restore_verdict` test hold them together. A bank can declare its defaults and still inherit the declining validator. It can also declare a validator and no exchange. The swap then refuses at run time, or the verdict and the restore drift apart, with nothing at compile time to catch it.

## Contract
- One type states the lane-swap capability as a whole. For example, a `LaneSwap` trait whose three methods have no defaults (the defaults size, the exchange and the validation), reached through one `PreparedNativeEffectBank` hook such as `fn lane_swap(&self) -> Option<&dyn LaneSwap>` and its `&mut` twin. A bank then either has all three or none.
- Every caller reads the hook:
  - `swap_bank_lanes` and `swap_bank_lanes_scratch_bytes` (`live.rs`);
  - `rack`'s `lane_swap_scratch_fits` (`crates/rack/src/lib.rs` ~830);
  - any other reader of `lane_default_bytes`.
- Every banking launch effect that implements the lane swap implements it:
  - compressor;
  - de-esser;
  - dynamic EQ;
  - gate/expander;
  - parametric EQ;
  - soft clip;
  - transient shaper;
  - true-peak limiter.

  The multiband compressor implements `validate_track_state_payload` (`crates/multiband-compressor/src/lib.rs` ~2986) but declares no `lane_default_bytes` and no exchange, so it does not swap lanes. `effect-compiler/tests/restore_verdict.rs` (~303) asserts the validators of all nine banks. The hook therefore pairs only the defaults size with the exchange; the restore validator stays a separate trait method, so multiband keeps it and `restore_verdict` is unchanged. Making multiband swappable is out of scope.

  So do the test banks in `crates/rack/tests/{lane_swap,lane_rest,bypass_skip}.rs` and `effect-contract`'s `live.rs` tests.
- No behaviour changes. Every swap, refusal and restore verdict is the base's.

## Notes
- The cost is the trait change across the banks, not the render path. `swap_bank_lanes` already dispatches through `&mut dyn PreparedNativeEffectBank`, and the wasm call-graph gate's render closure stops at `executor.render`, so a second trait object here adds no pinned `call_indirect` site. The gate still runs: `scripts/check-web-audioworklet.sh` on the rebuilt artifact.
- `swap_bank_lanes` runs on the render thread (`Runtime::pack_lanes`). The hook and its calls stay allocation-free and inside `REALTIME_POLICY` regions.

## Non-goals
- Changing what any bank validates or exchanges.
- The limiter's history/main-ring tie (#2393).
- Third-party effects (R6a).

## Also carried (#2378 C15, independent, a one-line cleanup)
`builtins`' `FaderRampStage` keeps the kernel's `f32` countdown word in `ramp[c].remaining`. That word is rebuilt from the `u32` countdown at the top of every ramping block (`process_plane`, `crates/builtins/src/lib.rs` ~3704-3710) and has no other reader. It should become a local of `process_plane`. Because `gain_mute_ramp_block` (`crates/lane/src/kernels/builtins.rs` ~305) takes the whole `&mut GainMuteRamp<L>`, of which `remaining` is a field, this needs either a change to that kernel's signature in the `lane` crate or a scoped `GainMuteRamp` copy; the `lane` crate is then touched. The same applies to any other stage whose `f32` countdown is pure per-block scratch. The change must move no bit and must leave `fader_bank_lane_words` as it is. It may land as its own commit here or as its own issue.

## Gates
- The workspace compiles with the new hook. No bank keeps a separate `lane_default_bytes` or `validate_track_state_payload` override outside it. This is compile evidence, not a source-grepping test.
- `effect-compiler`'s `restore_verdict`, `effect-contract`'s `lane_swap_tests` and `rack`'s lane-swap suites are green and unchanged in what they assert.
- No bit moved: a one-time comparison against base of a lane-swapping console render, recorded as PR evidence.
- Gates for every touched crate: `cargo test --locked`, clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`, `scripts/check-workspace-policy.sh` and the realtime policy tool. Also the call-graph gate on a rebuilt artifact.

## Evidence / attempt record
(empty)
