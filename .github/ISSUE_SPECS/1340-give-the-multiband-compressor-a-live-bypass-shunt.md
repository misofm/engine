# Give the multiband compressor a live bypass shunt

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E4, D15-7).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A multiband compressor insert's bypass is a live update on both hosts, in both directions. Lifting
a session bypass is heard at the next block: the lane's wet path has been running all along, so
the output from that block on is the output of an instance that was never bypassed. A bypassed
multiband lane shares its bank with enabled lanes, and a hot bypassed lane never moves a bank-mate's
bits. This closes decision 14's F4 for the multiband compressor.

## Context

- **Why the bypass is prepared today.** `PREPARED_BYPASS_EFFECTS = ["miso.multiband-compressor"]`
  (`crates/effect-compiler/src/prepare.rs:260`, doc `:246-259`). The multiband banks, but its bank
  recovery (D7) is whole-bank: `Instance::finish` calls `bank::finish_block`
  (`crates/multiband-compressor/src/lib.rs:1196-1219`), which zeroes both channels of every lane and
  resets both `Side`s (`crates/effect-runtime/src/bank.rs:219-235`). A bypassed lane still runs its
  wet path under a lowered bypass, so a legal hot input on it (about `6e29`) would silence its
  enabled bank-mates. That couples lanes' bits, which decision 12 forbids (#1100).
- **What a lift does today.** The browser's `COMMAND_EFFECT_BYPASS` admits a lift that renders
  nothing different, because the processor was prepared bypassed (`hosts/host-web/src/lib.rs:836-844`).
  The C ABI classifier returns `LiveRebuild::PreparedBypass` for any multiband bypass change
  (`crates/host-core/src/live_delta.rs:136-140`, `:376-383`), so the edit is a rebuild. Decision
  15 makes that rebuild a D15-9 transition (*Duck-swap a strip whose state cannot continue across a
  plan swap*, #1324), and *Refuse commands that would be acknowledged with no effect* (#1315), which
  lands before this slice, refuses the browser's inert lift (its D4, keyed on
  `lowers_session_bypass`). D1 here lifts that refusal for the multiband (D8).
- **The precedent.** The transient shaper's per-lane recovery (#1092): `recover_lanes`
  (`crates/transient-shaper/src/lib.rs:507-538`) zeroes both channels of each failed lane, resets
  that lane's state with `replace_lane` (`:827`), and leaves every other lane bit-unchanged. The
  gate-expander does the same per channel (`crates/gate-expander/src/lib.rs:651-700`).
- **The multiband's reset.** `Side::discontinuity_reset` (`lib.rs:698-707`) clears `filter`
  (`Lr4State`, `:475`), `gain_db`, and snaps every track's ramps. The scalar instance (`W = 1`)
  runs exactly this on its one lane when D7 fires.
- **The render switch.** `render` (`lib.rs:1022-1057`) selects `BYPASS` at compile time from the
  prepared `instance.bypass`. A lowered bypass prepares `bypass = false`, so a bypassed lane runs
  the wet path and the rack's `BypassShunt` (`crates/effect-contract/src/live.rs:867`) selects the
  dry signal per lane. The multiband's latency is 0 (`lib.rs:348`), so the shunt has no line.
- **Padding.** The multiband declines a padded bank until #1069 closes (`lib.rs:1569-1574`). This
  issue does not change that.
- **Tests that pin today's behaviour.**
  - `crates/graph-compiler/tests/bypass_cohorts.rs:716` (`a_mixed_bypass_multiband_cohort_keeps_its_prepared_bypass`)
    and `:794` (`a_hot_bypassed_multiband_lane_leaves_its_neighbours_bits_unchanged`, whose
    positive control is the whole-bank recovery under #1087's lowering).
  - `crates/effect-compiler/tests/native_session.rs:516-528` asserts the multiband's prepared
    bypass and the list.
  - `crates/multiband-compressor/tests/nonfinite.rs:168` (`the_boundary_is_the_shared_limit_and_a_bank_shares_its_reset`)
    asserts that one lane's failure zeroes the whole bank block.
  - `crates/host-core/tests/live_delta.rs:1450-1470` (`a_prepared_bypass_change_needs_a_rebuild`).

## Decisions frozen for this slice

- **D1. Per-lane D7.** Replace `Instance::finish`'s call to `bank::finish_block` with a per-lane
  recovery on the transient shaper's model. Check both channels once per block with `check_block`;
  on failure, take `nonfinite_lane_mask` over both channels. For each failed lane: zero that lane's
  words in both channels for the whole block; on both `Side`s, set that lane of every `Lr4State`
  word and of both `gain_db` words to `+0.0`; snap that track's ramps. Every other lane is untouched.
  This is exactly what `discontinuity_reset` does to the scalar instance's one lane, so a lane's
  bits in a bank equal its bits alone (class A). Per-track `ProcessReport` charging is unchanged
  (both counters, the whole frame count). The shared `NonFiniteReport` keeps the failed mask and
  adds one block.
- **D2. Lower the session bypass.** Remove the multiband from `PREPARED_BYPASS_EFFECTS`. Its
  session bypass then lowers to per-lane shunt state like every other bankable effect: prepared
  `bypass = false`, `initial_bypass` set, a channel-less lane (`EffectControlLane::without_channel`)
  or a live lane. A mixed-bypass cohort shares one program key and binds one bank.
- **D3. Delete the machinery that becomes empty.** This issue lands after *Give the delay a live
  bypass shunt* (#1339), which lowers the delay's bypass. With both gone, no launch effect keeps a
  prepared session bypass. Delete `PREPARED_BYPASS_EFFECTS`, `lowers_session_bypass`, and
  `LiveRebuild::PreparedBypass` with its arm in `classify` (`live_delta.rs:376-383`). Every caller
  lowers unconditionally. Rewrite the doc comments that name the exception:
  `prepare.rs:40-67` and `:233-268`, `crates/effect-contract/src/lib.rs:1151-1156`,
  `crates/effect-contract/src/live.rs:116-118` and `:846-866`,
  `crates/host-core/src/live_delta.rs:128-140` and `:160-170`, the browser's
  `COMMAND_EFFECT_BYPASS` comment (`hosts/host-web/src/lib.rs:836-844`) and its SDK twin
  (`sdk/src/browser/shipped-host.d.ts:202-208`), and `docs/C_ABI_V1_QUALIFICATION.md:393`.
- **D3b. The SDK's copy.** The SDK refuses a live lift on the listed effects with a
  `MisoUsageError` (`PREPARED_BYPASS_EFFECTS`, `sdk/src/core/live-controls.ts:143-159`, used at
  `:930-950`), and `sdk/test/console-evals.mjs` holds that copy to the engine (`:994-1030` and the
  catalog test after `:1040`). After #1339 the list holds only the multiband; delete the constant,
  the refusal and its doc, and rewrite the catalog test so every catalog effect's live lift renders
  the authored-unbypassed session bit for bit.
- **D4. The prepared `BYPASS` arm stays.** `PreparedEffectMetadata::bypass` remains part of the
  effect contract (conformance and direct preparation use it). Only the session lowering changes.
- **D5. Carry (D15-7).** The live bypass is lane state. A banked lane's bypass and shunt words carry
  through *Carry live-controlled effect lanes across a plan swap* (#1280, D2 and D4), and a per-node
  instance's through *Carry per-node effect instances across a plan swap* (#1282, D2). The sentence
  of #1282's D1 that says a committed multiband bypass change is a prepared bypass that is not
  carried no longer holds: under the C ABI that change is now a live `Bypass` record, and the owner
  carries. No new payload word: recovery state is not carried.
- **D6. Switch shape.** This slice lands after *Crossfade the bypass switch over the session
  ramp* (#1341, stream E, D15-13 E5), which crossfades every shunt over the session mute ramp, the
  banked one included. The multiband's live bypass therefore crossfades from its first commit; this
  slice adds no switch code.
- **D8. #1315's refusal lifts.** Once `lowers_session_bypass` is deleted, #1315 D4's browser
  refusal has no instance left to refuse, and a multiband lift is admitted and heard. The multiband
  half of #1315's gate 3 test (lift refused) is rewritten as an admitted lift that renders the
  authored-unbypassed session after the crossfade. If #1315's prepared-bypass refusal path then has
  no caller, this slice deletes it with its doc comment text.

## Effect evidence (AGENTS.md list)

- **Equations, coefficients, smoothing, latency, tail:** unchanged. The crossover, the
  Giannoulis-Massberg-Reiss static curve and the smoother are untouched. Latency stays 0.
- **NaN and denormal behaviour:** D7 stays a once-per-block boundary check (`BLOCK_LIMIT = 1e30`,
  `bank.rs:32`), now attributed and recovered per lane. Denormals are still flushed inside the
  recurrences. No per-value `is_finite` is added to a render path.
- **Citations:** master plan §4.4 (the boundary check); decision 12 (banking never couples bits).
- **Listening evidence:** not required. No audible path changes for a lane that does not trip D7.

## Deliverables

1. D1 in `crates/multiband-compressor/src/lib.rs`, with a lane-replace helper local to the crate.
2. D2 and D3 in the effect compiler, the effect contract docs and the C ABI classifier.
3. The tests in "Objective gates", and the deletions in "Test value".
4. A row in `crates/multiband-compressor/tests/MUTATIONS.md` for D1's red mutation.

## Authorized paths

- `crates/multiband-compressor/src/lib.rs`, `crates/multiband-compressor/tests/nonfinite.rs`,
  `crates/multiband-compressor/tests/MUTATIONS.md`
- `crates/effect-compiler/src/prepare.rs`, `crates/effect-compiler/src/lib.rs` (re-exports only),
  `crates/effect-compiler/tests/native_session.rs`
- `crates/effect-contract/src/lib.rs`, `crates/effect-contract/src/live.rs` (doc comments only;
  stream A and stream E edit code in `live.rs`, so rebase on their merges)
- `crates/graph-compiler/tests/bypass_cohorts.rs`
- `crates/host-core/src/live_delta.rs` (D3 only: the variant, its arm and its docs; stream B owns
  the file, so coordinate the merge order with its coordinator),
  `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs` (rewrite `a_prepared_bypass_change_rebuilds_and_renders_the_committed_model`,
  `:2716-2722`, see gate 5; stream B owns the file)
- `docs/C_ABI_V1_QUALIFICATION.md` (the one sentence at `:393`)
- `hosts/host-web/src/lib.rs` (the `COMMAND_EFFECT_BYPASS` doc comment, and #1315's
  prepared-bypass refusal if D8 leaves it no caller), `hosts/host-web/src/tests.rs` (D8's
  rewrite of #1315's gate-3 multiband half),
  `sdk/src/browser/shipped-host.d.ts` (doc only), `sdk/src/core/live-controls.ts` and
  `sdk/test/console-evals.mjs` (D3b only); stream H owns these, so coordinate the merge

## Non-goals

- Making the multiband console-eligible, or letting it bind a padded bank (`CONSOLE_ELIGIBLE_EFFECTS`,
  `prepare.rs:223-230`; `lib.rs:1569-1574`).
- A live crossover (*Make the multiband compressor's crossover live*, #1338).
- New crossfade code (#1341 owns it).
- Any change to the delay (#1339).

## Objective gates

1. **Per-lane D7 in a bank.** At the build's bank width (`BankWidth::for_backend`), a bank of
   `varied_values` tracks fed the seeded signal, with `NaN` planted at frame 5 of lane 3 in block
   0. Lane 3's block is `+0.0` on both channels and its report charges `FRAMES` to both counters.
   Every other lane's output, every block, is bit-identical to the same lane rendered without the
   planted value, and to that track rendered alone at `W = 1`. Lane 3's blocks after the failure
   are bit-identical to a fresh scalar instance's after its own recovery. This replaces the bank
   half of `the_boundary_is_the_shared_limit_and_a_bank_shares_its_reset`; its scalar half stays.
2. **Mixed bypass binds one bank.** Rewrite `a_mixed_bypass_multiband_cohort_keeps_its_prepared_bypass`
   as `a_mixed_bypass_multiband_cohort_binds_one_bank`: every bypassed entry has
   `initial_bypass == true`, `metadata.bypass == false` and a lane; the cohort binds
   `TRACKS / lanes()` banks; the render is bit-identical to today's prepared-bypass lowering
   (`todays_lowering`) and to `Backend::Scalar` at music levels.
3. **A hot bypassed lane in a bound bank.** `a_hot_bypassed_multiband_lane_leaves_its_neighbours_bits_unchanged`
   keeps its name and assertion (zero neighbour taps moved), now with the hot lane inside a bound
   mixed bank (assert the bank binds). Its in-test positive control under #1087's lowering is
   removed, because the lowering is now the production path; D1's red mutation replaces it.
4. **Lowering.** `native_session.rs` asserts that the bypassed multiband entry is lowered
   (`initial_bypass`, `!metadata.bypass`, a lane), and that every launch descriptor lowers.
5. **A live lift is heard, bit-exactly.** In `crates/host-core/tests/live_delta.rs`: the classifier
   turns a multiband bypass change, in each direction, into one `EffectControlRecord::Bypass`
   ahead of any parameter records. Then, on a plan prepared with live controls and the multiband
   insert bypassed and `control_smoothing.mute_ms = 0`, a `Bypass(false)` record admitted before
   block 4: blocks 0-3 equal the dry input, and every block from 4 on is bit-identical to a plan
   prepared with the insert enabled and fed the same input (the wet path ran throughout). With the
   default table, the same holds from the first block after #1341's crossfade ends. On the C ABI, rewrite
   `a_prepared_bypass_change_rebuilds_and_renders_the_committed_model` as the live case: the
   bypass change commits with no plan replacement, and the render from the first block after the
   crossfade equals the control that never bypassed.
6. **Realtime.** The bank render path still allocates nothing (`tests/no_alloc_render.rs`), the
   recovery block included.
7. Commands:
   - `cargo test --locked --all-targets -p multiband-compressor -p effect-runtime --features math/lane,lane/test-support`
   - `cargo test --locked -p effect-compiler -p graph-compiler -p host-core --features effect-compiler/test-support,graph/test-support,host-core/test-support`
   - `cargo test --locked -p host-web -p capi --features host-web/test-support,host-core/test-support`
   - the browser and SDK legs: `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-types.sh` and
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts` (runs `console-evals.mjs`)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all -- --check`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-effect-runtime-policy.sh`,
     `bash scripts/check-realtime-policy.sh`
   - `bash scripts/check-cross-targets.sh` and `scripts/run-aarch64-tests.sh` (CI
     `aarch64-debug`/`aarch64-release` when no arm64 host)

## Test value

- Gate 1: a recovery that still resets the whole bank, or that resets a lane but skips its ramp
  snap, moves a neighbour's bits or the failed lane's later bits; it turns red where today's
  test asserts the opposite.
- Gate 2: a lowering that leaves the multiband prepared, or that lowers it with a different program
  key, binds no mixed bank; it turns red.
- Gate 3: with D1 reverted and D2 kept, the hot bypassed lane zeroes its bank-mates; it turns red.
  No other test runs a hot lane inside a mixed bound bank.
- Gate 5: a lane whose live bypass is admitted but whose processor stays prepared bypassed renders
  dry after the lift (the F4 defect); it turns red. A shunt that resets the wet path on lift moves
  bits after block 4; it turns red.
- Superseded and deleted in the same PR: the bank half of
  `the_boundary_is_the_shared_limit_and_a_bank_shares_its_reset`, the old body of
  `a_mixed_bypass_multiband_cohort_keeps_its_prepared_bypass`, and
  `a_prepared_bypass_change_needs_a_rebuild` (`live_delta.rs:1450`), whose subject no longer exists,
  and the SDK's `KEEPS_PREPARED_BYPASS` assertions (`console-evals.mjs:994-1030`).
  The harness's `p1_lowering` (`bypass_cohorts.rs:256-274`) and its `Lowering::P1` arm read
  `PREPARED_BYPASS_EFFECTS` and become the production lowering; delete them too.

## Dependencies

- *Multiband compressor: a ramp's cut moves a lane's bits in a bank* (#1069).
- *Give the delay a live bypass shunt* (#1339), for D3's deletions.
- *Carry live-controlled effect lanes across a plan swap* (#1280).
- *Carry per-node effect instances across a plan swap* (#1282).
- *Crossfade the bypass switch over the session ramp* (#1341), so the live switch never steps (D6).
- *Refuse commands that would be acknowledged with no effect* (#1315), whose multiband refusal
  this slice lifts (D8).
