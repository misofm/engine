# Give the delay a live bypass shunt

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E4, D15-7, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A session bypass on a `miso.delay` insert becomes per-lane shunt state, like every other launch
effect except the multiband compressor. Lifting or setting it is a live update on both hosts: the
browser's `COMMAND_EFFECT_BYPASS` and a C ABI transaction that changes only the bypass are heard at
the next block, with no plan rebuild. A bypassed delay emits its dry input, even on a block where
its own feedback ring trips its finiteness check. Its echoes keep building while it is bypassed, so
the lift is heard with the tail already in place.

## Context

- **The delay's bypass is prepared today.** `NEVER_BANKED_EFFECTS = ["miso.delay"]`
  (`crates/effect-compiler/src/prepare.rs:244`) does two jobs. Its doc (`:233-243`) says the delay
  never banks, and that its session bypass stays prepared for exactness of its D7 check.
  `lowers_session_bypass` (`:264-267`) is false for it. Preparation keeps the bypass prepared
  (`bypass: effect.bypass && !lowered`, `:450-455`). `attach_effect_live_controls` seeds the lane
  bypassed, but nothing reaches the processor (`:1431-1439`). Decision 14 F4 records the result:
  a live lift is acked and never heard.
- **The C ABI rebuilds instead.** `classify` returns `LiveRebuild::PreparedBypass` for a bypass
  change on an effect that does not lower its bypass (`crates/host-core/src/live_delta.rs:380-384`;
  doc `:136-140`, `:165-167`). The header says the same (`crates/capi/include/miso_engine_v1.h:39-41`).
  The classifier reads `lowers_session_bypass`, so it changes when the list changes. It needs no
  logic edit.
- **The D7 check** (`crates/delay/src/lib.rs:22-23`, `finish_block` `:919-950`, `recover_lane`
  `:961-979`) scans each lane's output, the ring cells the block wrote and the damping state. On a
  failure it zeroes the output, clears the damping state and drops the history. Under a prepared
  bypass the output is the dry input (`mix_sample`, `:1175-1178`), so a trip zeroes a dry block.
- **Why the prepared bypass is the wrong behaviour.** The dry block a delay receives is always
  finite and under `1e30`: the input stage sanitises every out-of-range sample to `+0.0`, and every
  stage after it zeroes a block that leaves that range (`BypassShunt` doc,
  `crates/effect-contract/src/live.rs:859-862`). D7 exists to keep the effect's own output from
  carrying non-finite or runaway values downstream. A bypassed delay's output is that dry block,
  which is already in range. Zeroing it is a dropout of a valid signal, caused by wet state nobody
  hears. The test `the_delay_keeps_its_prepared_bypass_because_a_shunt_would_move_a_bit`
  (`crates/graph-compiler/tests/bypass_shunt_identity.rs:798-858`) pins exactly that dropout: a
  1 ms delay at `0.95` feedback fed `9e29` trips the ring check, and the prepared bypass zeroes the
  block. So this correctness reason does not hold, and decision 15 D15-13 E4 gives the delay a live
  shunt.
- **The shunt path already exists per node.** A never-banked effect with a control lane becomes
  `NodeKind::LiveControlEffect` (`crates/graph/src/runtime.rs:4636-4648`). It captures the dry
  block, runs the effect and copies the dry block back when bypassed (`:3530-3584`). The delay's
  latency is 0 (`crates/delay/src/lib.rs:263`), so its shunt has no line and costs two
  quantum-sized dry buffers.
- **Pinned lists.** `crates/effect-compiler/tests/native_session.rs:508-545` pins the delay's
  prepared bypass and the lowered set. `bypass_shunt_identity.rs:754-788` pins
  `NEVER_BANKED_EFFECTS` to the effects that decline a bank. That stays true for the delay.
- **Other copies of the rule.** The browser's `COMMAND_EFFECT_BYPASS` doc
  (`hosts/host-web/src/lib.rs:836-844`) and its test `a_live_bypass_cannot_lift_the_delay_or_multiband_session_bypass`
  (`hosts/host-web/src/tests.rs:9099-9140`). The SDK's `PREPARED_BYPASS_EFFECTS`
  (`sdk/src/core/live-controls.ts:143-159`, used at `:935-946`) and its engine cross-check
  (`sdk/test/console-evals.mjs:1027-1029`, `:1044-1067`). The shipped type doc
  (`sdk/src/browser/shipped-host.d.ts:203-208`). The contract docs
  (`crates/effect-contract/src/lib.rs:1151-1157`, `live.rs:117-120`, `:862-865`).
  `docs/C_ABI_V1_QUALIFICATION.md:393`.
- **Until this lands** (context, not a deliverable): a committed delay bypass change is a plan
  rebuild. Under decision 15 D15-9 that rebuild runs behind the duck-swap of *Duck-swap a strip
  whose state cannot continue across a plan swap* (#1324).

## Decisions frozen for this slice

- **D1. Split the two roles.** `NEVER_BANKED_EFFECTS` keeps only "declines a bank" and still
  names `miso.delay`. Its doc loses the prepared-bypass sentence. `lowers_session_bypass(id)`
  becomes `!PREPARED_BYPASS_EFFECTS.contains(&id)`. `PREPARED_BYPASS_EFFECTS` stays
  `["miso.multiband-compressor"]`. Emptying it and deleting `lowers_session_bypass` and
  `LiveRebuild::PreparedBypass` belongs to *Give the multiband compressor a live bypass shunt*
  (#1340).
- **D2. Lowering.** A session-bypassed delay is prepared with `bypass = false`. Its bit rides a
  channel-less lane (`EffectControlLane::without_channel(true)`) when no live controls are
  attached, or the live lane otherwise. It renders as a per-node `LiveControlEffect`. An enabled
  delay with no live controls stays `NodeKind::Effect`, byte for byte.
- **D3. D7 under a shunt.** While a lane is bypassed, its output is the dry input, bit for bit,
  on every block. A trip still runs `recover_lane` on the wet path: the history is dropped, the
  damping state is cleared and `nonfinite_*_blocks` counts the block. Then the shunt copies the dry
  block over the zeroed output. The delay kernel does not change.
- **D4. Behaviour change, stated.** The delay's bits move only on a block where a bypassed lane's
  ring or damping state trips D7. There the output becomes the dry block instead of zeros. Every
  other block is bit-identical to today, because the shunt and `mix_sample`'s bypass select emit
  the same words, `-0.0` included. No checked-in digest moves. If one does, the PR explains it.
- **D5. Carry.** The delay's bypass is now lane state. Under D15-7 a rebuild that also changes a
  delay's bypass carries the instance (ring and feedback tail), then applies the committed bypass
  as a lane record. This replaces the delay leg of #1282's gate 3 ("a committed bypass change on
  the delay is not carried"). The multiband leg stays until #1340.
- **D6. Crossfaded from the first commit.** This slice lands after *Crossfade the bypass switch
  over the session ramp* (#1341). #1341 crossfades every shunt, per-node and banked, over the
  session mute ramp, so the delay's new live bypass crossfades from the day it ships; this slice
  adds no switch code of its own.
- **D7. Acked-batch question.** After this slice a delay bypass record is never acked without
  effect: it changes the lane flag that the next block's shunt reads. *Refuse commands that would
  be acknowledged with no effect* (#1315) lands first; its browser refusal of a prepared-bypass
  lift keys on `lowers_session_bypass` (#1315 D4), so D1 lifts it for the delay with no further
  edit. The delay half of #1315's gate 3 (a delay lift refused) is rewritten here as gate 5's
  admitted, heard lift; its multiband half stays until #1340.

## Deliverables

1. D1 and D2 in `crates/effect-compiler/src/prepare.rs`, with its docs and comments updated.
2. Doc updates only: `crates/effect-contract/src/lib.rs`, `crates/effect-contract/src/live.rs`,
   `crates/host-core/src/live_delta.rs` (`:136-140`, `:165-167`, `:380-381`),
   `hosts/host-web/src/lib.rs:836-844`, `sdk/src/browser/shipped-host.d.ts:203-208`,
   `crates/capi/include/miso_engine_v1.h:39-41`, `docs/C_ABI_V1_QUALIFICATION.md:393`.
3. SDK: drop `"miso.delay"` from `PREPARED_BYPASS_EFFECTS` (`sdk/src/core/live-controls.ts:156-159`)
   and fix its docs (`:143-155`, `:930-940`). In `sdk/test/console-evals.mjs`, the delay lines at
   `:1027` and `:1029` assert that the lift does not throw.
4. The tests below. Delete the superseded ones in the same PR.

## Authorized paths

- `crates/effect-compiler/src/prepare.rs`, `crates/effect-compiler/tests/native_session.rs`
- `crates/effect-contract/src/lib.rs`, `crates/effect-contract/src/live.rs` (docs only)
- `crates/graph-compiler/tests/bypass_shunt_identity.rs`, `crates/graph-compiler/tests/bypass_cohorts.rs`
- `crates/host-core/src/live_delta.rs` (docs only), `crates/host-core/tests/live_delta.rs`
  (coordinate with stream B, which owns `live_delta.rs`)
- `crates/host-core/tests/successor_swap.rs` (coordinate with stream A)
- `crates/capi/src/runtime/live_tests.rs`, `crates/capi/include/miso_engine_v1.h` (docs only;
  coordinate with stream B)
- `hosts/host-web/src/lib.rs` (doc comment only), `hosts/host-web/src/tests.rs`,
  `sdk/src/core/live-controls.ts`, `sdk/src/browser/shipped-host.d.ts`, `sdk/test/console-evals.mjs`
  (coordinate with stream H)
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- No change to the delay kernel, its D7 check or its prepared-bypass code path. Other callers and
  oracles still use `PreparedEffectMetadata::bypass`.
- The delay still never banks. No bank kernel.
- No new crossfade code (#1341 owns it). No multiband change (#1340), and no deletion of
  `lowers_session_bypass` or `LiveRebuild::PreparedBypass` (#1340).

## Hazards

- `capture_dry` (`runtime.rs:3537`) must stay true on every bypassed block. A delay has no line,
  so on an enabled block it is false and costs nothing.
- `bypass_cohorts.rs`'s "today's lowering" oracle (`todays_lowering`, `:123`) must not treat the
  delay as prepared-bypassed after D1.

## Objective gates

1. **A bypassed delay passes its dry block when its ring trips** (new, `bypass_cohorts.rs`, real
   compiler and graph). One track with one session-bypassed delay insert: 1 ms, feedback `0.95`,
   damping `0`, mix `1`. The source is `9e29` for 4 blocks, then deterministic noise near
   -20 dBFS for 8 blocks. The session output is bit-identical, every block, to the same session
   with the insert removed. The delay's `nonfinite_left_blocks` (through the test-support report
   or the graph's effect counters) is at least 1.
2. **Lowering shape** (`native_session.rs:508-545`, rewritten). A session-bypassed delay has
   `metadata.bypass == false`, `bank_preparation.bypass == false`, `initial_bypass == true` and a
   channel-less bypassed lane. The lowered list is every launch effect but the multiband.
   `NEVER_BANKED_EFFECTS == ["miso.delay"]` still holds.
3. **Classifier** (`crates/host-core/tests/live_delta.rs`). `a_prepared_bypass_change_needs_a_rebuild`
   (`:1450-1470`) loops over the multiband only. A new test changes the delay's bypass, in both
   directions, and gets exactly one `EffectControlRecord::Bypass` carrying the post-commit bit,
   with no rebuild.
4. **C ABI live lift** (new, `crates/capi/src/runtime/live_tests.rs`). A track with a
   session-bypassed delay insert. A transaction that lifts only the bypass commits with no plan
   replacement. With `control_smoothing.mute_ms = 0`, from the next block its output is
   bit-identical to a control booted with the delay enabled and fed the same PCM; with the default
   table, from the first block after #1341's crossfade ends. The wet path ran all along, so no rebuild or carry is needed for
   equality. The lift block allocates and frees nothing (`bench_support::alloc` thread counters).
5. **Browser** (`hosts/host-web/src/tests.rs:9099-9140`, rewritten). The delay leaves the exceptions.
   Lifting a session-bypassed delay live renders the authored-unbypassed session bit for bit after
   the crossfade (a step with `mute_ms = 0`), and the multiband stays the exception. The delay half
   of #1315's gate 3 test (lift refused) is rewritten as this admitted lift. SDK: `console-evals.mjs`'s cross-check (`:1044-1067`) passes
   with the shorter list.
6. **Carry with a bypass change** (new, `crates/host-core/tests/successor_swap.rs`). Session A has
   a bypassed delay insert ringing at 50 ms and feedback `0.9`. B is A with the delay unbypassed,
   plus a muted track whose ID sorts first, so the change is a rebuild. Swap after block 6. Every
   block equals a reference that runs A and lifts the delay's bypass live at block 7. #1282's gate-3
   delay leg is deleted, and its multiband leg stays.
7. **Deleted, superseded:** `the_delay_keeps_its_prepared_bypass_because_a_shunt_would_move_a_bit`
   (`bypass_shunt_identity.rs:798-858`), whose claim gate 1 now reverses. Update the red-mutation
   doc of `the_never_banked_list_is_exactly_the_launch_effects_that_decline_a_bank` (`:754-760`).
8. Commands:
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo test --locked -p delay`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit && ./target/release/audit capi && ./target/release/audit delay --blocks 100000`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
     `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-types.sh && bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-effect-runtime-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a delay that keeps its prepared bypass zeroes the tripped dry block. A lowering that
  builds no shunt for a never-banked effect lets the wet runaway through. Either turns it red, and
  no existing test checks a bypassed delay's dry block on a trip.
- Gate 3: a classifier that still rebuilds on a delay bypass (a hard-coded effect ID instead of
  `lowers_session_bypass`) turns it red.
- Gate 4: a C ABI path that acks the lift but renders the prepared bypass, or that replaces the
  plan, turns it red. The existing #1266 test covers only the multiband.
- Gate 6: a carry rule that still treats the delay's bypass as prepared restarts the ring at rest,
  and the tail vanishes at block 7. That turns it red.

## Dependencies

- *Carry per-node effect instances across a plan swap* (#1282). This slice lands after it and
  replaces the delay leg of #1282's gate 3 (D5).
- *Crossfade the bypass switch over the session ramp* (#1341), so the live switch never steps (D6).
- *Refuse commands that would be acknowledged with no effect* (#1315), whose delay refusal this
  slice lifts (D7).
- The final deletion of `lowers_session_bypass`, `PREPARED_BYPASS_EFFECTS`'s last entry and
  `LiveRebuild::PreparedBypass` is in *Give the multiband compressor a live bypass shunt* (#1340),
  which lands after this one.
