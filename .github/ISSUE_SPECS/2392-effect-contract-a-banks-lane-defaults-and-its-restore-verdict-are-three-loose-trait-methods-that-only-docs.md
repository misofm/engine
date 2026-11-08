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

## Also carried (#2378 C15, independent, a one-line cleanup) -- moved to #2410
Moved: #2410's spec now carries this row explicitly as its fader contract item (it is amended on this issue's branch). As filed, #2410 changed only the matrix and the input trim, and its row 1 says the fader already follows the rule. So it did not own the stored word until the amendment. Not done here.

`builtins`' `FaderRampStage` keeps the kernel's `f32` countdown word in `ramp[c].remaining`. That word is rebuilt from the `u32` countdown at the top of every ramping block (`process_plane`, `crates/builtins/src/lib.rs` ~3704-3710) and has no other reader. It should become a local of `process_plane`. Because `gain_mute_ramp_block` (`crates/lane/src/kernels/builtins.rs` ~305) takes the whole `&mut GainMuteRamp<L>`, of which `remaining` is a field, this needs either a change to that kernel's signature in the `lane` crate or a scoped `GainMuteRamp` copy; the `lane` crate is then touched. The same applies to any other stage whose `f32` countdown is pure per-block scratch. The change must move no bit and must leave `fader_bank_lane_words` as it is. It may land as its own commit here or as its own issue.

## Gates
- The workspace compiles with the new hook. No bank keeps a separate `lane_default_bytes` or `validate_track_state_payload` override outside it. This is compile evidence, not a source-grepping test.
- `effect-compiler`'s `restore_verdict`, `effect-contract`'s `lane_swap_tests` and `rack`'s lane-swap suites are green and unchanged in what they assert.
- No bit moved: a one-time comparison against base of a lane-swapping console render, recorded as PR evidence.
- Gates for every touched crate: `cargo test --locked`, clippy `-D warnings`, `cargo fmt --check`, `cargo doc -D warnings`, `scripts/check-workspace-policy.sh` and the realtime policy tool. Also the call-graph gate on a rebuilt artifact.

## Evidence / attempt record

### Attempt 1 (Terra), branch `codex/issue-2392-lane-swap-pairing`

**Claims re-verified on base `231494259`.** All hold. `PreparedNativeEffectBank` had `lane_default_bytes` (default `None`), `exchange_lane_defaults` (default `false`) and `validate_track_state_payload` (default `effect.state.unchecked`) as three independent methods, paired only by the docs. Compile evidence of the drift on base: deleting the compressor bank's `validate_track_state_payload` override (`crates/compressor/src/lib.rs` 1745-1755 at base) still compiles (`cargo check -p compressor`: `Finished`), so a bank that declares its defaults silently inherits the declining verdict and every swap of it refuses `Payload` at run time. Readers of `lane_default_bytes`: `swap_bank_lanes` and `swap_bank_lanes_scratch_bytes` (`live.rs`), `rack`'s `lane_swap_scratch_fits` (`crates/rack/src/lib.rs` 830), no other (`hosts`, `tools` and `graph` call none of the three).

**Shape (how the contract and the multiband note are reconciled).** The contract asks for one type with three methods and no defaults; the multiband note asks that the restore validator stays a trait method so multiband keeps its own and `restore_verdict` is unchanged. Both hold:
- `effect_contract::LaneSwap` (`crates/effect-contract/src/lib.rs`): `lane_default_bytes(&self) -> usize`, `exchange_lane_defaults(&mut self, lane, bytes) -> bool` and `validate_lane_payload(&self, track, version, input)`, none with a default. `Option<usize>` became `usize`: absence is now the hook's `None`.
- `PreparedNativeEffectBank::lane_swap(&self) -> Option<&dyn LaneSwap>` and its twin `lane_swap_mut(&mut self) -> Option<&mut dyn LaneSwap>`, both `None` by default. Each banking launch effect implements `LaneSwap` and returns `Some(self)` from both.
- `validate_track_state_payload` stays a trait method. Its default now reads `lane_swap()?.validate_lane_payload(..)` and otherwise declines `effect.state.unchecked` as before. So the eight swapping banks no longer override it (their verdict is the `LaneSwap` body), the multiband compressor keeps its override outside the capability, and every caller of the trait method -- the swap and `effect-compiler/tests/restore_verdict.rs` -- reads one body. `swap_bank_lanes` still calls `validate_track_state_payload` (unchanged lines), so the verdict `restore_verdict` checks is the verdict the swap reads.
- Two hooks are one object by contract, not by type. `swap_bank_lanes` reads the defaults size through both (`lane_swap_defaults`, `live.rs`) and refuses `Defaults` before any write when either is absent or they disagree, so twin drift cannot reach the exchange (where it would half-rotate the defaults). `rotate_lane_defaults` exchanges through `lane_swap_mut`. `swap_bank_lanes_scratch_bytes` and `rack::lane_swap_scratch_fits` read the `&` twin.
- Converted: compressor, de-esser, dynamic EQ, gate/expander, parametric EQ, soft clip, transient shaper, true-peak limiter; the test banks in `rack/tests/{lane_swap,lane_rest,bypass_skip}.rs` and `effect-contract`'s `lane_swap_tests::Words`; three test call sites that exchanged through the bank trait (`de-esser`/`dynamic-eq` `glide_tests.rs`, the limiter's `lib.rs` test) now reach `lane_swap_mut()`. Bodies moved verbatim (`Some(N)` became `N`); each `LaneSwap` impl is a `REALTIME_POLICY` region, so the de-esser's and dynamic EQ's exchange, which sat outside one at base, are now inside one.
- Workspace evidence: after the change the only `fn validate_track_state_payload` overrides are the trait's own default and `crates/multiband-compressor/src/lib.rs` 2986; no bank has a `lane_default_bytes` or `exchange_lane_defaults` outside a `LaneSwap` impl (the trait methods no longer exist).

**Compile evidence (scratch, not committed), on the compressor bank.**
- `LaneSwap` impl without `validate_lane_payload`: `error[E0046]: not all trait items implemented, missing: validate_lane_payload` at `impl<L: Lane> LaneSwap for PreparedCompressorBank<L>`.
- Without `exchange_lane_defaults`: `error[E0046]: ... missing: exchange_lane_defaults`.
- The hooks kept, the `LaneSwap` impl deleted: `error[E0277]: the trait bound PreparedCompressorBank<L>: LaneSwap is not satisfied` at both `Some(self)`.
- Base contrast above: the same drift compiled.

**No behaviour change.**
- No bit moved: `graph-compiler`'s `silent_bank_skip::a_moved_strip_renders_the_bits_of_an_unmoved_one` (gate 3: a console of builtins, EQ, compressor, limiter, fader and matrix under 13 forced render-time lane swaps across a plan swap), instrumented in scratch with an FNV-1a digest over every block's moved-plan host PCM words and both plans' observed words, `--release`: base `231494259` `inserts=false 7d31fea2b6652670`, `inserts=true 8e4dfb8b8dc1d599`; branch identical. The limiter takes the payload path (validation and exchange through `LaneSwap`); EQ and compressor the #2229 direct path.
- Lane-swap results identical to base: `effect-contract` `lane_swap_tests`, `rack`'s `lane_swap`, `lane_rest` and `bypass_skip`, every launch effect's `swap_tests`/`rest_tests` and `effect-compiler`'s `restore_verdict` are green and assert what they asserted at base (the test banks' bodies moved verbatim).

**New test case (one claim).** `lane_swap_tests::every_lane_swap_refusal_leaves_both_lanes_unchanged` gains three `Defaults` cases with a bank whose `lane_swap_mut` is `None` while `lane_swap` is not (first bank, second bank, one bank). Test value: red when `swap_bank_lanes` reads only one twin of the hook, so a bank whose `&mut` twin is absent passes the `Defaults` check and half-rotates the defaults. No existing test can catch it: the twins are new.

**Mutations of the change (each reverted).**
- M1, `validate_track_state_payload`'s default ignores `lane_swap` (always `unchecked`): red, `lane_swap_tests` (`case 10`) and `restore_verdict` (`miso.compressor seed 0x23240000 lane 0: its own payload validates`, `left: Err(unchecked)`).
- M2, `lane_swap_defaults` drops the `lane_swap_mut` read: red, `lane_swap_tests` panics `a checked lane's defaults exchange` (the new cases).
- M3, the exchange helper reports `true` without exchanging: red, `lane_swap_tests` (the accepted swap's defaults).
- M4, `swap_bank_lanes_scratch_bytes` adds `0` for the defaults: red, `lane_swap_tests` (`scratch_bytes == 24 + DEFAULTS`).
- M5, `rack::lane_swap_scratch_fits` stops comparing the peer's size (`defaults(processor).is_some()`): **survives**, and the same mutation of the base line (`processor.lane_default_bytes().is_some()`) survives on base too. No `rack` test pairs banks of different defaults sizes; `swap_bank_lanes` still refuses `Defaults` itself. Pre-existing, recorded as a follow-up, not widened here.

**C15 (#2378, the `FaderRampStage` `f32` countdown word).** Not done here. Attempt 1 recorded it as "superseded by #2410", and that was wrong: #2410 as filed changed only the matrix and the input trim (its row 1 says the fader already follows the rule). Attempt 2 amends `.github/ISSUE_SPECS/2410-*.md` on this branch to carry C15 explicitly:
- the problem row 1 now names the stored `ramp[c].remaining`;
- a fader contract item drops the word and builds it as a `process_plane` local, through either a `lane` kernel signature change or a scoped `GainMuteRamp` copy;
- a size-change item;
- a mutation item.
The coordinator syncs #2410's GitHub body to that file.

**Gates (all green).**
- `cargo test --locked`, one crate at a time: `effect-contract` 84, `compressor` 104, `de-esser` 21, `dynamic-eq` 18, `gate-expander` 71, `parametric-eq` 133, `soft-clip` 42, `transient-shaper` 46, `true-peak-limiter` 81, `multiband-compressor` 81, `rack` 75, `effect-compiler` 59 (with `restore_verdict`) passed, none failed. `graph-compiler --test silent_bank_skip` 14 passed (the lane-swapping consumer, with its allocation-free move test).
- `cargo clippy --locked --all-targets -- -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` over the ten touched crates; `cargo fmt --check`; `scripts/check-workspace-policy.sh` (`ok`); `realtime_policy` (`ok (307 marked regions in 48 files)`).
- Call-graph gate: rebuilt artifact (`scripts/build-web-audioworklet.sh --named-twin`, nightly-2026-08-20, shipped module `210298df2f5c...`), then `scripts/check-web-audioworklet.sh --without-metadata-regeneration` with wabt 1.0.34 (CI's apt version): passed; `miso_engine_web_v1_render` `indirect_sites=11` against the pinned table, every render-thread export clean. The hook adds no pinned site: the swap's dispatch was already through `&mut dyn PreparedNativeEffectBank`, behind the executor's indirect call.
- Graph plan fixtures: no struct changed (a trait change only), so none regenerated.

**Follow-ups (not filed).**
- `rack`: no test pairs two banks of different defaults sizes, so `lane_swap_scratch_fits`'s peer comparison is unpinned (M5, survives on base too).
