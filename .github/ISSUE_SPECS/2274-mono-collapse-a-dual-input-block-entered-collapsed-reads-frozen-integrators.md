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
- The input stage owns its own transitions: every way out of `collapsed` leaves both channels
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

## Evidence (attempt 1)

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

**Reachable through the bank's public surface.** `BuiltinInputBank::process_mono` followed by
`process` without `desymmetrize` renders the right plane from frozen integrators in release, and is
a `debug_assert!` panic in debug. Reset never cleared the flag either, but that is not observable
once the dual body takes the disengage itself: reset writes `+0.0` and one silence count on both
channels, so a stage left collapsed after a reset is a collapsed stage whose channels already agree.

**Model chosen.**
- *Chain:* `enum CollapseMode { Agreeing, Diverged, Collapsed }` replaces `collapsed` and
  `collapse_channels_agree`, so the illegal combination cannot be represented and the
  `debug_assert!` plus its redundant assignment go. `disengage_collapse` is now the only exit from
  `Collapsed`: it is a no-op unless collapsed, desymmetrizes every prefix stage, and then sets
  `Agreeing`. `disengage_for_carry` is that call. `inherit_channel_agreement(false)` now disengages
  before it records `Diverged`, so a hypothetical collapsed successor cannot leave the chain
  `Diverged` over collapsed stages (on base it was an unrepresented illegal combination that a later
  disengage silently reset to "agree"). A successor inherits before it renders, so this is
  unreachable today. The public accessors (`is_collapsed`, `collapse_channels_agree`,
  `carried_channel_agreement`) keep their meaning and stay `const`.
- *Stage:* the input stage owns its own way out. `InputStage::process` entered collapsed takes the
  disengage copy (`desymmetrize`) before its body, which replaces the `debug_assert!`. From the
  chain this is one untaken branch per bank per block: the chain has always copied first.
  `reset_with_kind` clears the flag, so it always describes the state. The SIMD lane words are
  unchanged.

**Red/green.** `a_desymmetrized_bank_is_a_never_collapsed_bank` (`crates/builtins/tests/mono_collapse.rs`)
gains a second arm (`explicit = false`) that goes from collapsed blocks straight to the dual body.
On base it fails in debug with
`a dual block after a collapsed one needs the disengage copy first (desymmetrize)`, and in release
(`cargo test --release -p builtins --test mono_collapse`) with
`width=4 explicit=false block=4 word=1: right plane  left: 3135754338  right: 1027169682`. Head
passes, debug and release. Test value: the new arm turns red if a dual block entered collapsed
renders the right channel from the integrators the collapse froze. No existing test reaches the
dual body without the explicit `desymmetrize`. A dedicated reset test was written and then deleted:
it stayed green with the reset line reverted (the dual body's own disengage covers it), so it
caught nothing unique.

**Mutations (head).**
- Drop `self.desymmetrize()` from `InputStage::process`: the new arm fails (`explicit=false block=4
  word=1: right plane`).
- Drop `self.collapsed = false` from `reset_with_kind`: every test stays green, as argued above. The
  line keeps the flag truthful; it has no observable effect.
- Make `disengage_collapse` leave the chain `Diverged`: three `rack` `mono_reengage` tests fail
  (transitions `[1, 1, 1]` against `[1, 1, 0]`). That test's stale red-mutation note named the
  deleted assignment and is updated.
- Make `run` leave `Collapsed` without `disengage_collapse`: `rack`'s
  `disengaging_copies_the_prefixs_state_once` and
  `stage_witness_bank_entire_armed_decision_publishes_once_even_when_declining` fail, as does
  `builtins-compiler`'s `actual_graph_mono_collapse_disengages_on_input_command_and_recovers_nonfinite_input`.

**No bit moved.** Three scratch dumps, never committed, were run at base and at head (release,
AVX2), with only `crates/rack/src/lib.rs` and `crates/builtins/src/lib.rs` swapped between runs:
- A scratch host-core test rendering 7 documents (mono-mapped EQ bank, builtins only, symmetric
  and asymmetric EQs, symmetric and asymmetric per-node delay, misaligned cohorts) at `Simd8`,
  `Simd4` (only the builtins-only document prepares there; the console slot refuses a foreign width)
  and `Scalar` (per-node), under four live-write schedules (none, `Both` trim rides, a `Left` trim
  later equalised, a `Left` polarity flip). Each ran armed and forced off over 64 blocks, with 36
  silent blocks to reach the silent skip. That is 168 rendered cases: 2,650 collapsed blocks,
  5,506 skipped blocks and 16 cases with disengages. An FNV-1a hash of every output bit, plus the
  collapse counters, transitions and skips, was identical in all 208 lines.
- `host-core/tests/randomized.rs`'s armed-against-dual differential at
  `MISO_ENGINE_RANDOMIZED_SCALE=4` (48 seeds, 384 blocks), hashing both arms' outputs per block:
  identical.
- `host-core/tests/successor_swap.rs` (125 tests, every plan swap and carry gate), hashing every
  block rendered through its `render` helper: 12,903 block hashes over 56 tests, identical.

**Gates (head).**
- `cargo test --locked -p builtins -p rack -p builtins-compiler -p graph -p host-core`: 100 test
  binaries, all pass; the same with `host-core/test-support`, `graph/test-support`,
  `builtins-compiler/test-support` and `builtins/test-support`: 100 binaries, all pass.
- `cargo test --locked --release -p builtins --test mono_collapse`: pass.
- `cargo clippy --locked -p rack -p builtins -p builtins-compiler -p graph -p host-core
  --all-targets -- -D warnings`, with and without those features: clean.
- `cargo fmt --check`: clean. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p rack -p builtins`:
  clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (292 marked regions in 46 files)`.

**Follow-ups (not this issue).**
- FX-4: each native effect crate's `collapsed` mirror (parametric-eq, gate-expander,
  transient-shaper, soft-clip, de-esser, multiband-compressor, dynamic-eq, compressor) is cleared
  only by its `desymmetrize`. Their resets and dual bodies should get the same ownership.
- The input stage's lane export and `channels_agree` still read channel `1` directly, and are
  proven unreached while collapsed. A stage-owned answer there would finish the job.
