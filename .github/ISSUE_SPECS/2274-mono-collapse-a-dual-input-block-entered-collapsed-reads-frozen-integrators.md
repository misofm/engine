# Mono collapse: a dual input block entered collapsed reads frozen integrators, and the chain's mode is two loose flags

Item GR-4 of the implicit-state-machine tracker #2258 (audit of 2026-11-06).

## Problem
The mono-collapse phase lives in two places.
- `rack::BankChain` (`crates/rack/src/lib.rs`) kept `collapsed` ("the previous block rendered
  collapsed") and `collapse_channels_agree` ("the channels agree at the next dual block") as two
  bools. One of their four combinations, collapsed while disagreeing, is illegal, and only a
  `debug_assert!` in `disengage_collapse` refused it.
- `builtins::InputStage` (`crates/builtins/src/lib.rs`) keeps its own `collapsed`. Only
  `process_mono` sets it and only `desymmetrize` clears it. Nothing but a `debug_assert!` at the top
  of the dual `process` ties it to the chain: a dual block entered collapsed renders channel `1`
  from the integrators the collapse froze. `reset_with_kind` never clears it.

## Contract
- Find out, and record, whether a chain path can leave the chain and the stage disagreeing (a
  reset, a plan swap or carry, a lane swap, an import, a skip).
- Model the chain's phase as one `CollapseMode`, so the illegal combination cannot be represented
  and `Collapsed` has one exit, the one that desymmetrizes every prefix stage.
- The input stage owns its collapse on every entry: every way out of `collapsed` leaves both channels
  holding the counterfactual dual run's state, whatever order a caller uses.
- No rendered bit moves, banked or per-node, collapsed or not.

## Non-goals
The `collapsed` mirrors in the native effect crates (tracker FX-4); `crates/effect-contract`; the
four bind-time and per-block inputs `collapse_prefix`, `collapse_forced_off`, `collapse_source` and
`input_symmetric`, which are independent inputs to the per-block decision rather than phase (every
combination of them is legal); the fader parts of `crates/builtins/src/lib.rs` (GR-1).

## Gates
- A regression test red on base; if no chain path is reachable, a recorded proof plus the type
  change.
- A one-time no-bit-moved comparison against base (PR evidence, not a committed digest).
- `cargo test --locked` for `builtins`, `rack` and their dependents `builtins-compiler`, `graph`,
  `host-core`; clippy `-D warnings`; `cargo fmt --check`; `scripts/check-workspace-policy.sh`; the
  realtime policy tool.

## Evidence

Base is `de0878e23`; line numbers below are base lines.

**Reachability through the chain: none.** The stage flag is set only by `InputStage::process_mono`
(`builtins/src/lib.rs:2141`), which only `BankChain::run` calls, on a block whose dispatch has just
set the chain's `collapsed` (`rack/src/lib.rs:4401`). The chain's flag had three writes: bind
(`false`), `run` (`4394-4401`) and `disengage_for_carry` (`4927-4930`). Both writes that turn it off
call `disengage_collapse` (`4655`) first, which calls `desymmetrize` on every prefix slot, and
`desymmetrize` is the stage flag's only clear (`builtins:2301`). Each suspected path, checked:
- *Reset.* `BuiltinInputBank::reset` has no caller outside tests, and the `BankStage` trait has no
  reset, so the engine never resets an input bank.
- *Plan swap and carry.* `carry_input_lanes` and `carry_effect_lanes` (`graph/src/runtime.rs:7879`,
  `8313`, `8359`) call `disengage_for_carry` on the predecessor chain before exporting a lane, and
  the successor imports into a fresh, never-rendered stage.
- *Lane swap.* `BankChain::swap_lanes` (`rack:5085`) disengages before it moves a lane; an in-chain
  swap of a lane with itself returns before touching anything.
- *Import.* `import_lane` is only reached from the two paths above.
- *Silent skip.* On the first collapsed block a skip leaves the stage dual while the chain is
  collapsed. That is the benign direction: the stage then answers rest and advances clocks on both
  channels, which agree. The harmful direction (stage collapsed under a dual dispatch) has no
  write that can produce it.
- *Arming changes.* `arm_mono_collapse(false)` and `force_mono_collapse_off(true)` on a collapsed
  chain decline the next block, and `run` disengages.
- *Errors.* A prefix slot that errors in `process_mono` leaves the chain collapsed, so the next
  block still disengages every prefix slot.

**Reachable through the bank's public surface.** Each of these misbehaves on base:
- `BuiltinInputBank::process_mono` and then `process` without `desymmetrize`: the right plane is
  rendered from frozen integrators in release, and a `debug_assert!` panics in debug.
- `process_mono`, then `import_lane`, then `process`: base panics on that assertion. Attempt 1's
  self-heal then copied left over the imported lane's right channel.
- `process_mono`, then `reset`, then `advance_clocks`: the stage stayed collapsed, so the skipped
  block advanced channel `0`'s silence counter alone.
- `export_lane` and `channels_agree` on a collapsed bank: they read channel `1`'s frozen integrators
  and silence counter.

**Model.**
- *Chain:*
  - `enum CollapseMode { Agreeing, Diverged, Collapsed }` replaces `collapsed` and
    `collapse_channels_agree`. The illegal combination cannot be represented, so its
    `debug_assert!` and the redundant assignment are gone.
  - `disengage_collapse` is the only exit from `Collapsed`. It does nothing unless collapsed,
    desymmetrizes every prefix stage, then sets `Agreeing`. `disengage_for_carry` is that call.
  - `inherit_channel_agreement` now `debug_assert!`s that the chain is not `Collapsed`, because a
    successor inherits before it renders. In release a collapsed chain disengages before it records
    `Diverged`, so the chain and its stages leave `Collapsed` together. On base this combination
    was unrepresented, and a later disengage silently reset it to "agree".
  - The public accessors (`is_collapsed`, `collapse_channels_agree`, `carried_channel_agreement`)
    keep their meaning and stay `const`.
- *Input stage:* it owns its collapse on every entry.
  - The two `&mut` entries that treat channel `1`'s retained state as real take the disengage copy
    (`desymmetrize`) first: `process`, which replaces the `debug_assert!`, and `import_lane`, which
    must take the copy *before* it writes the imported lane.
  - The two `&self` readers, `export_lane` and `channels_agree`, answer channel `1`'s integrators
    and silence counter from `live_state_channel`. That is the counterfactual dual state, the same
    rule `lanes_at_rest`, `advance_clocks`, the elision plan and the filter retarget's predicate
    already followed.
  - `swap_lanes` drops its two `debug_assert!`s and needs nothing more: its exports answer
    counterfactually, and its first import on each stage disengages it.
  - `reset_with_kind` clears the flag.
  - The SIMD lane words are unchanged.

**Every `BuiltinInputBank` entry that touches per-channel state, collapsed.** "Mirrored" means
`process_mono` copies the record onto channel `1` at the bottom of every collapsed block (the trim
ramp, `coef.trim`, the filter coefficient, target, step and countdown records). "Frozen" means
channel `1`'s integrators and silence counter.

| Entry | Collapsed behaviour | Correct because |
|---|---|---|
| `process` | disengage copy, then the dual body | the copy writes the counterfactual right state |
| `process_mono` | stays collapsed | precondition: dispatched under the symmetric witness; debug-asserted for the elision plan, the trim ramp, and (since the attempt-2 fold) every member lane's `DESIGNED` words, which cover the filter coefficient, target, step and countdown records |
| `desymmetrize` | copies frozen state left to right, clears | is the disengage |
| `reset` | writes both channels, clears | one state on both channels |
| `lanes_at_rest` | reads channel `0`'s integrators | counterfactual |
| `advance_clocks` | advances channel `0`'s silence counter | counterfactual; the copy overwrites the frozen one |
| `channels_agree` | frozen words read from channel `0`; mirrored words raw | counterfactual |
| `export_lane` | frozen words read from channel `0`; mirrored words raw | counterfactual |
| `import_lane` | disengage copy, then writes the lane | the imported right channel is not overwritten afterwards |
| `swap_lanes` | export, then import, on each stage | from the two rows above, which `a_collapsed_bank_exports_the_dual_runs_state` and `an_import_into_a_collapsed_bank_keeps_the_imported_right_channel` pin; no test drives `swap_lanes` itself on a collapsed bank |
| `apply_prepared_filter` | rule-3 predicate via `live_state_channel`; writes per-channel targets | targets are not frozen; a one-channel write declines the witness |
| `set_trim_db`, `set_polarity_invert` | write per-channel ramp records | not frozen; the disengage copy deliberately leaves the ramp (`a_retarget_between_a_collapsed_block_and_the_disengage_survives_the_copy`) |
| `trim_signed`, `lane_symmetry`, `copy_response_snapshot_lane` | read mirrored words | mirrored |
| `new`, `backend`, `width`, `active_lanes`, `supports_mono_collapse`, `can_swap_lanes` | no per-channel retained state | n/a |
| `test_support` readers and `set_bank_lane_state_words` | raw retained words, documented as raw | evidence and fault-injection hooks only |

`InputBuiltins` (the scalar per-node section) has no `process_mono` and never collapses.

**What still guards the chain.** The chain could, in principle, leave `Collapsed` without
disengaging. The input stage's self-heal now masks that at audio level for the input stage. What
still catches it is `rack`'s own tests (`disengaging_copies_the_prefixs_state_once`,
`stage_witness_bank_entire_armed_decision_publishes_once_even_when_declining`, the `mono_reengage`
`bank_collapse_transitions()`/`collapse_transitions()` asserts) and `builtins-compiler`'s
`actual_graph_mono_collapse_disengages_on_input_command_and_recovers_nonfinite_input`. An effect
stage in the prefix (FX-4) would still render wrong. This is the trade for making the input stage
correct under every caller order.

**Tests** (`crates/builtins/tests/mono_collapse.rs`). Base results are at `de0878e23`. Attempt-1
results are at `d1f31d4c5`.
- `a_desymmetrized_bank_is_a_never_collapsed_bank`, new arm `explicit = false`: collapsed blocks
  straight into the dual body.
  - Base, debug: `a dual block after a collapsed one needs the disengage copy first (desymmetrize)`.
  - Base, release: `width=4 explicit=false block=4 word=1: right plane left: 3135754338 right: 1027169682`.
  - Test value: red if a dual block entered collapsed renders the right channel from frozen
    integrators.
- `an_import_into_a_collapsed_bank_keeps_the_imported_right_channel`, the reviewer's probe:
  - Base, debug: the same assertion panic. Base, release: `width=4 word=1` mismatch.
  - Attempt 1: `width=4 word=0`.
  - Test value: red if a disengage copy taken after an import overwrites the imported lane's right
    channel.
- `a_reset_bank_forgets_its_collapse`, the reviewer's probe, reduced to its retained-silence
  assertion:
  - Base: `width=4 lane=0: the retained silence counters`.
  - Test value: red if a reset leaves the stage collapsed, so the next skipped block advances
    channel `0`'s silence clock alone.
- `a_collapsed_bank_exports_the_dual_runs_state`, new:
  - Base and attempt 1: `width=4: channels_agree`.
  - Test value: red if `export_lane` or `channels_agree` reads the frozen right integrators or
    silence counter. A carry that reached a collapsed bank would then hand a successor stale state,
    or the proof would refuse agreeing channels.

**Mutations (head).** Each builtins mutation is red in exactly one test:
| Mutation | Red test |
|---|---|
| Drop `process`'s disengage | `a_desymmetrized_bank_is_a_never_collapsed_bank` (`explicit=false block=4 word=1: right plane`) |
| Drop `import_lane`'s disengage | `an_import_into_a_collapsed_bank_keeps_the_imported_right_channel` (`width=4 word=0`) |
| Drop `reset_with_kind`'s clear | `a_reset_bank_forgets_its_collapse` (`the retained silence counters`) |
| `channels_agree`: read channel `1`'s integrators raw | `a_collapsed_bank_exports_the_dual_runs_state` (`channels_agree`) |
| `channels_agree`: read channel `1`'s silence counter raw | `a_collapsed_bank_exports_the_dual_runs_state` (`channels_agree`) |
| `export_lane`: read channel `1`'s integrators raw | `a_collapsed_bank_exports_the_dual_runs_state` (`the exported lane`) |
| `export_lane`: read channel `1`'s silence counter raw | `a_collapsed_bank_exports_the_dual_runs_state` (`the exported lane`). This was green until that test also rendered a silent collapsed tail. |
| `disengage_collapse` leaves `Diverged` | three `rack` `mono_reengage` tests (`[1, 1, 1]` against `[1, 1, 0]`) and three `rack` unit tests |
| `run` leaves `Collapsed` without `disengage_collapse` | six `rack` `mono_reengage` tests, two `rack` unit tests and one `builtins-compiler` test (listed above) |

With the counterfactual readers in place, the reset clear is visible only through the retained
state it leaves behind: the right silence clock (`test_support::bank_lane_silence_words`), and the
rest mask on the fault-injection surface (a stale flag answers `15`, per the attempt-2 review).
The reset test asserts the retained silence counters only. Its earlier `channels_agree` and
`export_lane` assertions stayed green under every mutation, because a stage wrongly left collapsed
answers both counterfactually, so they were dropped.

**No bit moved.** Three scratch dumps were run at base `de0878e23` and at head. They were never
committed, and only `crates/rack/src/lib.rs` and `crates/builtins/src/lib.rs` differed between the
runs. All are release builds on AVX2.
- *A scratch host-core matrix.*
  - 7 documents: a mono-mapped EQ bank, builtins only, symmetric and asymmetric EQs, symmetric
    and asymmetric per-node delay, and misaligned cohorts.
  - 3 backends: `Simd8`, `Simd4` and `Scalar` (per-node).
  - 4 live-write schedules: none, `Both` trim rides, a `Left` trim later equalised, and a `Left`
    polarity flip.
  - Each case ran armed and forced off over 64 blocks, 36 of them silent, so it reached the silent
    skip.
  - That is 168 cases: 128 rendered and 40 refused at prepare (`console.slot.foreign_width`).
  - At `Simd4` only the builtins-only and misaligned documents prepare; the other five put an EQ
    console slot at a foreign width.
  - The rendered cases cover 2,650 collapsed blocks, 5,506 skipped blocks and 16 cases with
    disengages.
  - Every line is identical at base and head: an FNV-1a hash of every output bit, the collapse
    counters, the transitions and the skips (or the refusal).
- *`host-core/tests/randomized.rs`*, armed against dual at `MISO_ENGINE_RANDOMIZED_SCALE=4`: 48
  seeds, 384 blocks, both arms hashed per block. Identical.
- *`host-core/tests/successor_swap.rs`*, every block rendered through its `render` helper: 12,903
  block hashes over 56 tests. Identical.

Attempt 1 miscounted the matrix as 168 rendered cases, because each refusal's diagnostic spans two
lines. Attempt 2 re-ran all three dumps at its own builtins and rack changes (head before the spec
commit) against the same base dumps. All three are identical: 208 matrix lines, 384 randomized
blocks and 12,903 swap block hashes.

**Gates (attempt-2 head).**
- `cargo test --locked -p builtins -p rack -p builtins-compiler -p graph -p host-core`: 100 test
  binaries, all pass. The same with `host-core/test-support`, `graph/test-support`,
  `builtins-compiler/test-support` and `builtins/test-support`: 100 binaries, all pass.
- `cargo test --locked --release -p builtins --test mono_collapse`: 6 pass.
- `cargo clippy --locked -p rack -p builtins -p builtins-compiler -p graph -p host-core
  --all-targets -- -D warnings`, with and without those features: clean.
- `cargo fmt --check`: clean. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p rack -p builtins`:
  clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (292 marked regions in 46 files)`.

**Gates (after the attempt-2 MINOR fold).**
- `cargo test --locked` for the same five crates with the test-support features: 100 binaries,
  all pass. The new `process_mono` `debug_assert!` holds on every collapsed block in them.
- clippy `-D warnings` with and without the features, `cargo fmt --check`, rustdoc `-D warnings`,
  workspace policy and `realtime_policy`: clean.
- The reset test is red on base and with the reset clear reverted: `width=4 lane=0: the retained
  silence counters`.
- The two rack mutation counts were re-run with `--no-fail-fast` and match the table.

## Attempt record
- **Attempt 1** (`d1f31d4c5`): FAIL, two MAJORs.
  - The self-heal in `process` silently dropped an imported lane's right channel when an import
    landed between `process_mono` and `process`.
  - The reset clear was claimed unobservable, but `advance_clocks` after a reset left the right
    silence clock behind, and `channels_agree`/`export_lane` read it.
  - Three MINORs: record the remaining tripwire; assert the unreachable inherit precondition;
    correct the no-bit-moved counts (attempt 1 reported 168 rendered cases; 128 rendered, 40
    refused).
  - The reviewer found the rack half sound and the chain reachability proof complete.
- **Attempt 2** (`8ff797c39`): PASS.
  - The input stage owns its collapse on every entry (import, export, `channels_agree`, swap),
    plus the reviewer's two probes, a collapsed-export test, the inherit `debug_assert!`, the
    entry table above, and re-run dumps.
  - The review found the entry table complete and `swap_lanes` correct, reproduced the
    no-bit-moved dumps, and merged cleanly and green with #2280.
  - Ruling on the reset test, option (a): a test of the invariant through the retained-state
    hooks is legitimate.
  - Four MINORs, folded after the PASS: the reset test drops its two assertions that could not
    fail; the `process_mono` row says what is debug-asserted, and a `debug_assert!` that every
    member lane's `DESIGNED` words are symmetric is added; the rack mutation counts are corrected;
    the `swap_lanes` row names the tests it rests on.

## Follow-ups (not this issue)
- FX-4: each native effect crate's `collapsed` mirror (parametric-eq, gate-expander,
  transient-shaper, soft-clip, de-esser, multiband-compressor, dynamic-eq, compressor) is cleared
  only by its `desymmetrize`. Their resets, imports, exports and dual bodies should get the same
  ownership.
