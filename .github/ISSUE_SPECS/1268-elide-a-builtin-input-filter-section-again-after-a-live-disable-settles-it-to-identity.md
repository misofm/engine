# Elide a builtin input filter section again after a live disable settles it to identity

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4, D15-6).
Code anchors verified on `main` at `6fb211594`.

Follow-up found while planning *Apply value-only input HPF and LPF edits to the running C ABI plan
through prepared targets* (#1262). It concerns the builtin input bank on both hosts: the browser
already disables filters live.

## Product outcome

After a live edit disables an input filter section, the section stops costing work once its
coefficient ramp settles, and the strip's output is bit-identical, signed zeros included, to a
strip whose filter was never enabled. The first deliverable measures whether `main` already does
this; what follows depends only on that measurement.

## Context

- **The elision rule.** A section is skipped only when its six coefficient words are the identity
  **and** both integrators are exactly `+0.0`, in every bank lane (`section_is_identity`,
  `crates/lane/src/kernels/builtins.rs:1146-1163`). The state words are compared bitwise because
  identity coefficients over a `-0.0` integrator would emit `-0.0` where the elided form gives
  `+0.0`.
- **The original premise.** The filed body (on `54b0a1bf8`) said that after a live disable the
  integrators keep their last values, because `svf_step` does not drive them to zero with identity
  coefficients (`crates/lane/src/kernels.rs:643-652`), so the section is never elided again.
- **What `main` shows.** The filter ramp bodies clear a lane's integrators when its ramp completes
  onto identity coefficients: `ic1`/`ic2` `andnot(identity)` at
  `crates/lane/src/kernels/builtins.rs:882-883` (dual body, `input_chain_ramp_block_filter`, `:801`)
  and `:980-981` (mono body, `input_chain_ramp_block_filter_mono`, `:915`). The mask is per lane,
  so only settled lanes are cleared. `refresh_filter_plan` (`crates/builtins/src/lib.rs:1266-1279`)
  runs right after the ramp prefix (`:1659`) and recomputes the plan. The decision-15 review found
  the premise likely stale (agreed plan, D15-4 context; round-1 review, B4 item 4).
- **What no test checks.** `filter_ramp_endpoint_is_partition_invariant_and_reset_honors_kind`
  (`crates/builtins/tests/filter_liveness.rs:223`) checks a disabled section's coefficient words,
  and `input_tail_is_infinite_while_a_filter_target_is_ramping` (`:314`) its tail. Neither reads the
  integrators, the elision plan, or signed-zero output after a disable.
- **Readback.** `builtins::test_support::bank_lane_state_words` and `bank_elision_plan`
  (`crates/builtins/src/lib.rs:5471-5479`, feature `test-support`). The bank renders through
  `BuiltinInputBank::process` (`:3615`, dual body) and `process_mono` (`:3631`, mono body).

## Decisions frozen for this slice

- **D1. Measure first (the first deliverable).** Write gate 1's test and run it on `main` before
  any other change. Record the result, with the command output, in this spec's decision record.
- **D2. If gate 1 is green on `main`:** the premise is stale. No production code changes. Commit
  the test as the regression guard, record the mutation evidence of gate 2, and close the issue.
- **D3. If gate 1 is red on `main`:** record which leg fails (dual or mono body, state or plan or
  output). Fix it in the failing body: when a lane's section ramp completes onto identity
  coefficients, set that lane's two integrators of that section to `+0.0` (per lane, under the
  ramp's own `done` mask; banking must not couple lanes' bits), and leave `refresh_filter_plan` to
  elide it. Then gate 1 must be green and gate 2's mutation must turn it red.
- **D4. Class.** Under D3, zeroing the state of an identity section changes no output sample
  except the sign of an exact zero, and moves those bits toward the never-enabled plan's. The PR
  says so.
- No change to the filter design, the 64-update ramp, the elision rule, or `svf_step` (the joint
  flush is *Flush the SVF jointly so builtin and EQ filters reach exact rest*, #1328).

## Deliverables

1. Gate 1's test and its recorded result on `main` (D1).
2. Either D2 (test plus evidence, close) or D3 (fix plus test plus evidence).

## Authorized paths

- `crates/builtins/tests/filter_liveness.rs`.
- Under D3 only: `crates/lane/src/kernels/builtins.rs` (the two filter ramp bodies) and
  `crates/builtins/src/lib.rs` (the input bank's filter ramp and plan refresh).
- This spec.

## Non-goals

- No change to the filter design, the ramp length, the elision predicate or the C ABI.
- No new skip-on-silence work beyond re-eliding a settled identity section.

## Objective gates

Run every command from the repository root.

1. **The D1 check.** A test in `crates/builtins/tests/filter_liveness.rs`, on a four-member bank
   (`bank_with_members(4)`) at 48 kHz, and its never-enabled twin fed the same input:
   - apply a prepared HPF target (120 Hz, from `prepare_input_filter_pair`) to lane 1, render
     several blocks of negative samples, then apply the disabled pair's targets to lane 1 and
     render past 64 updates, with exact `-0.0` samples in every block after the disable;
   - run it once through `process` with a `Left`-addressed target (dual body) and once through
     `process_mono` with a `Both`-addressed target (mono body);
   - after the block in which the ramp completes:
     - `bank_lane_state_words(&bank, 1)` is all `+0.0` bits;
     - `bank_elision_plan(&bank)` elides the HPF section on both channels;
     - from the next block on, lane 1's output is bit-identical to the twin's;
   - lanes 0, 2 and 3 are bit-identical to the twin's in every block.

   `cargo test --locked --all-targets -p builtins --features builtins/test-support --test filter_liveness`
2. **The mutation.** With the integrator clear removed locally from the dual body (and, separately,
   from the mono body), gate 1's matching leg is red. Record both runs in the PR; do not commit the
   mutation.
3. **Nothing else changes.**
   - `cargo test --locked --all-targets -p builtins --features builtins/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - Under D3 also: `cargo test --locked --release -p audit -p bench -p console-workload`,
     `bash scripts/check-web-audioworklet.sh` and `bash scripts/check-cross-targets.sh`.
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-lane-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all -- --check`
   - 4-lane (NEON) is CI-only here.

## Test value

- Gate 1's test: red if a disabled section keeps stale integrators (no re-elision, a signed-zero
  difference on `-0.0` input), if the plan refresh misses a settled section, or if the clear
  touches a lane that never ramped. No existing test reads the integrators or the plan after a
  disable (see Context). Gate 2 is the evidence that it turns red on that defect.

## Dependencies

None. *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
(#1262) depends on this issue's result.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No allocation, lock or unbounded work on the render thread.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Decision record

**D1 result (gate 1 on `main` at `7d5362dc2`, the branch base with G #1407 and #1329 landed).**
The test `a_settled_live_disable_is_re_elided_and_bit_identical_to_never_enabled`
(`crates/builtins/tests/filter_liveness.rs`) was written first and run with no production change:
`cargo test --locked -p builtins --features builtins/test-support --test filter_liveness a_settled_live`
gave `test ... ok`, `1 passed`. Both legs (dual body through `process` with a `Left`-addressed target;
one-plane body through `process_mono` with a `Both`-addressed target) are green: lane 1's integrators
are all `+0.0` and `bank_elision_plan` elides the HPF section after the block in which the ramp
completes, and lane 1 is bit-identical to the twin from the next block. **D2 applies: the premise is
stale. No production code changed.** The two ramp bodies already clear the integrators of a lane
whose ramp completes onto identity.

**Corrections to this body's Context (found while writing the test; the anchors moved with #1328,
#1407 and #1329).**

- The clears are at `crates/lane/src/kernels/builtins.rs:1166-1167` (dual body,
  `input_chain_ramp_block_filter_body`) and `:1410-1411` (mono body,
  `input_chain_ramp_block_filter_mono_body`); `section_is_identity` is at `:1590`; `svf_step_when`
  is `crates/lane/src/kernels.rs:1093`.
- "No existing test reads the integrators or the plan after a disable" is false.
  `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time` (`filter_liveness.rs:368`)
  reads `state_words`, `current_words` and the elision plan on the scalar chain and on one bank lane
  (dual body only), and `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths`
  (`crates/builtins/src/tests.rs`) reads the plan of a one-lane stage on both bodies. So removing
  either clear is already caught (mutation table below). The new test's unique catches are listed in
  its test-value sentence.
- The signed-zero output claim does not hold for a stale non-zero integrator, so the output leg
  cannot prove the clear by itself. With identity coefficients and a stale pair `(ic1, ic2)`, the
  settled section's output is `fma(0, ic2, fma(0, ic1, 1 * x))`; the next section, elided or running
  over `+0.0` integrators, ends in `v + 0.0`, which sends `-0.0` to `+0.0`, so a stale non-zero
  state never changes an output bit. (It changes only the cost, because the section is never
  elided again.) Only a `-0.0` integrator word is observable, and `andnot(identity)` writes `+0.0`
  exactly. The output legs of the test are therefore a lane-isolation guard (below), not the proof of
  the clear. The test still asserts the twin comparison, as the spec requires.

## Attempt record

**Attempt 1** (implementer, D2 path). Only `crates/builtins/tests/filter_liveness.rs` and this spec
changed.

The test runs both bodies over `history_blocks` in `6..14` (the history's sawtooth phase moves the
integrators the ramp leaves at the disable) and two scenarios: the spec's (lane 1 enabled, then
disabled; the twin never enables anything) and a `neighbour` scenario in which lane 3 also holds a
settled HPF in the bank and in the twin, so its integrators are live while lane 1 settles. Per block
after the disable (24 frames; the 64-update ramp completes mid-block 2): from block 2 lane 1's
eight state words are `+0.0` and the plan elides the HPF on both channels (with the neighbour, the
left HPF stays live because lane 3 needs it); from block 3 lane 1 is bit-identical to the twin; the
other lanes' outputs and state words equal the twin's in every block. The `neighbour` scenario is a
deviation from the body's letter that the test-value claim needs (a clear that reaches a lane that
never ramped is invisible if every other lane has zero state).

**Test value.** Which plausible defect turns this red that no existing test catches? A one-plane
(`process_mono`) block whose ramp completion clears the integrators of every lane when any lane
settles, so a neighbouring lane with a live HPF loses its state: `M4b-mono` below is green in every
other builtins test. The state and plan legs also pin both bodies on a four-lane bank at 24-frame
blocks with a mid-block completion; the clear's removal is caught by older tests too (table).

**Mutation runs** (applied locally to `crates/lane/src/kernels/builtins.rs`, reverted, not
committed; "builtins suite" is `cargo test --locked --no-fail-fast --all-targets -p builtins
--features builtins/test-support`; the new test alone is `... --test filter_liveness a_settled_live`).

| Mutation | New test | Other builtins tests red |
|---|---|---|
| M1: dual body, the two `ic` clear lines (`:1166-1167`) replaced by no-ops | RED (`filter_liveness.rs:1341`, lane 1 state) | `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths`, `a_disable_in_the_enables_drain_still_completes_and_elides`, `a_disable_freezes_the_recursion_words_and_moves_only_the_mix`, `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time`, `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs` |
| M2: mono body, the two `ic` clear lines (`:1410-1411`) replaced by no-ops | RED (`:1341`) | `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths` only |
| M4a: both bodies, `identity = done` (drops `current_target_identity`; clears every settled lane) | RED (`:1265` output leg, when run before the history change) | `symmetric_bank_target_keeps_mono_and_dual_paths_equal`, and in the dual run `live_peak_gain_bounds_the_real_kernel_on_its_worst_histories` |
| M4b dual: any settled lane makes the mask all lanes | RED (`:1265` output leg) | `bank_filter_target_updates_one_member_and_leaves_padding_identity` |
| **M4b mono: any settled lane makes the mask all lanes** | **RED** (`:1266`, output leg) | **none** |
| M4b mono with the state and plan assertions removed from the test | RED (`:1266`, output leg alone) | n/a |
| none (restored) | green | none |

M1 and M2 each turn the matching leg red (spec gate 2), but older tests already catch both, so gate 2
is evidence for the regression guard, not a unique catch. M4a and M4b were run with the first
revision of the history (a sine, replaced by an integer sawtooth because `f32::sin` is a
disallowed method); M1, M2, M4b mono and the restored run were repeated on the final test.

**Gates.** Final test, no production change:

- `cargo test --locked --all-targets -p builtins --features builtins/test-support`: all green
  (16 test binaries, 0 failed; the `filter_liveness` binary 15 passed).
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`: green.
- `bash scripts/check-workspace-policy.sh`, `check-builtins-policy.sh`, `check-lane-policy.sh`: pass.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: clean (the first
  run caught `f32::sin`; replaced).
- `cargo fmt --all -- --check`: clean.
- D3-only gates (audit, bench, console-workload, worklet chain, cross-targets) do not apply: no
  production code changed. 4-lane (NEON) is CI-only.

**Open items for root.** (1) The body's "no existing test" claim and the signed-zero output claim
are false as written (above); the spec's Context is left as filed and corrected here. (2) The
issue closes under D2 once verified.
