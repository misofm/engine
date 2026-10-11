# Elide a builtin input filter section again after a live disable settles it to identity

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4, D15-6).
Code anchors verified on `main` at `6fb211594`; refreshed 2026-10-11 against the branch head
`622aba3b1` (Root Amendment 1).

Follow-up found while planning *Apply value-only input HPF and LPF edits to the running C ABI plan
through prepared targets* (#1262). It concerns the builtin input bank on both hosts: the browser
already disables filters live.

## Product outcome

After a live edit disables an input filter section, the section stops costing work once its
coefficient ramp settles: the block that completes the ramp leaves the lane's integrators exactly
`+0.0`, and the plan elides the section again. The strip's output is bit-identical to a strip whose
filter was never enabled. For the HPF section, which the LPF stage follows, that holds with or
without the clear: a stale HPF integrator cannot change an output bit, the sign of an exact zero
included, because the LPF stage that follows, elided or running over `+0.0` integrators, ends in
`v + 0.0` (see Context, "What the older tests check, and what they do not"), so for the HPF the
clear's only observable effect is the state words and the plan, that is, cost. For the LPF
section nothing follows it, so a stale LPF integrator can change the sign of an exact zero at the
output, and the clear is also what keeps the LPF's output bit-identical (D4). The first
deliverable measures whether `main` already does this; what follows depends only on that
measurement.

## Context

- **The elision rule.** A section is skipped only when its six coefficient words are the identity
  **and** both integrators are exactly `+0.0`, in every bank lane (`section_is_identity`,
  `crates/lane/src/kernels/builtins.rs:1590-1599`). The state words are compared bitwise, so a
  section whose integrators hold any other bits, `-0.0` included, is never elided. (Corrected
  2026-10-11, root direction, after attempt 1; scoped again the same day after the spec check: the
  filed reason, that identity coefficients over a `-0.0` integrator would emit `-0.0` where the
  elided form gives `+0.0`, does not hold for the HPF section at the output. The LPF stage that
  follows it ends in `v + 0.0`, which sends `-0.0` to `+0.0`, so a stale HPF integrator never
  changes an output bit and only costs work. The LPF section has no stage after it, so for the LPF
  a stale integrator can still change the sign of an exact zero; see "What the older tests check,
  and what they do not".)
- **The original premise.** The filed body (on `54b0a1bf8`) said that after a live disable the
  integrators keep their last values, because `svf_step` does not drive them to zero with identity
  coefficients (`crates/lane/src/kernels.rs:1076-1077`, with the step's body in `svf_step_when` at `:1093`), so the section is never elided again.
- **What `main` shows.** The filter ramp bodies clear a lane's integrators when its ramp completes
  onto identity coefficients: `ic1`/`ic2` `andnot(identity)` at
  `crates/lane/src/kernels/builtins.rs:1166-1167` (dual body, `input_chain_ramp_block_filter_body`,
  `:1071`, called from `input_chain_ramp_block_filter`, `:1017`) and `:1410-1411` (mono body,
  `input_chain_ramp_block_filter_mono_body`, `:1328`, called from
  `input_chain_ramp_block_filter_mono`, `:1280`). The mask is per lane,
  so only settled lanes are cleared. `refresh_filter_plan` (`crates/builtins/src/lib.rs:1313-1335`)
  runs right after the ramp prefix (`:1799` in `process`, `:1944` in `process_mono`) and recomputes
  the plan. The decision-15 review found
  the premise likely stale (agreed plan, D15-4 context; round-1 review, B4 item 4).
- **What the older tests check, and what they do not.** (Corrected 2026-10-11, root direction,
  after attempt 1.) `filter_ramp_endpoint_is_partition_invariant_and_reset_honors_kind`
  (`crates/builtins/tests/filter_liveness.rs:223`) checks a disabled section's coefficient words.
  (The tail test that stood beside it, `input_tail_is_infinite_while_a_filter_target_is_ramping`,
  was replaced by #1329, `11f59fa42`.) Two older tests
  also read the integrators and the elision plan after a disable:
  `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time` (`filter_liveness.rs`) reads
  `state_words`, `current_words` and the elision plan on the scalar chain and on one bank lane
  (dual body only), and `filter_prefix_ends_at_countdown_for_dual_and_mono_paths`
  (`crates/builtins/src/tests.rs`) reads the plan of a one-lane stage on both bodies. No older
  test reads a bank lane's integrators and the plan next to the lane-isolation output of its
  neighbours on the one-plane body. For the HPF section no output leg can prove the clear: with
  identity coefficients and a stale pair `(ic1, ic2)` of finite integrators, the settled section's
  output is `fma(0, v2, fma(0, v1, 1 * v0))`, where `v0` is the section's input and `v1 = ic1 + d1`,
  `v2 = ic2 + d2` are the step's outputs with `d1`, `d2` signed zeros (`svf_step_when`,
  `crates/lane/src/kernels.rs:1093`). `v1` and `v2` equal `ic1` and `ic2` bit for bit except where
  an integrator is itself a zero, and the result is `v0` unchanged whenever `v0` is not a zero. The
  LPF stage that follows, elided or running over `+0.0` integrators, ends in `v + 0.0`, so a stale
  HPF state never changes an output bit and only costs work, because the section is never elided
  again. The state and plan legs prove the clear; the output legs guard lane isolation. This holds
  for the HPF only. The LPF section is last in the chain and nothing adds `+0.0` after it: a stale
  identity LPF over negative integrators fed an exact `-0.0` (a rested real HPF passes `-0.0`
  through) can emit `-0.0`, where the never-enabled twin, whose LPF is elided, emits `+0.0`. The
  spec check measured it on the real kernel (`input_chain_block_elided`, f32 lane, 120 Hz HPF at
  48 kHz, LPF integrators `(-0.1, -0.05)`, input `-0.0`): bank output `0x80000000`, twin `0x0`;
  for a stale HPF the same probe gives `0x0` and `0x0`.
- **Readback.** `builtins::test_support::bank_lane_state_words` and `bank_elision_plan`
  (`crates/builtins/src/lib.rs:5822-5830`, feature `test-support`). The bank renders through
  `BuiltinInputBank::process` (`:3909`, dual body) and `process_mono` (`:3925`, mono body).

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
- **D4. Class.** Under D3, zeroing the state of an identity section moves its state words and the
  plan to the never-enabled plan's, and the effect on the output depends on the section.
  - **HPF (section 0).** The clear changes no output bit, the sign of an exact zero included. With
    identity coefficients and a stale pair `(ic1, ic2)` of finite integrators the settled section's
    output is `fma(0, v2, fma(0, v1, 1 * v0))` (`v1 = ic1 + d1`, `v2 = ic2 + d2`, `d1` and `d2`
    signed zeros: equal to `ic1` and `ic2` bit for bit except where an integrator is itself a
    zero), and the LPF stage that follows, elided or running over `+0.0` integrators, ends in
    `v + 0.0`, which sends `-0.0` to `+0.0`; so a stale HPF integrator never changes an output
    sample and only costs work, because the section is never elided again.
  - **LPF (section 1).** The clear changes no output sample except the sign of an exact zero, and
    moves that bit to the never-enabled plan's: nothing follows the LPF to add `+0.0`, so a stale
    LPF over negative integrators and an exact `-0.0` at its input can emit `-0.0` where the
    never-enabled twin emits `+0.0` (Context, "What the older tests check, and what they do not").
  The PR says so.
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

1. **The D1 check.** The test `a_settled_live_disable_is_re_elided_and_bit_identical_to_never_enabled`
   in `crates/builtins/tests/filter_liveness.rs`, on a four-member bank (`bank_with_members(4)`) at
   48 kHz and its never-enabled twin, both fed the same input in 24-frame blocks. It runs both
   bodies, once through `process` with a `Left`-addressed target (dual body) and once through
   `process_mono` with a `Both`-addressed target (mono body), and two scenarios: the plain one
   (lane 1 only) and a `neighbour` one, in which lane 3 also holds a settled HPF in the bank and in
   the twin, so a clear that reaches a lane that never ramped shows. Each runs for every history
   length in `6..14` blocks, which moves the sawtooth's phase, and so the integrators, at the
   disable.
   - History: apply a prepared HPF target (120 Hz, from `prepare_input_filter_pair`) to lane 1, then
     render the history blocks of a signed per-lane sawtooth (about [-0.4, 0.4), a different period
     per lane, so negative samples are included). During the history lanes 0, 2 and 3 are
     bit-identical to the twin's; lane 1's first two state words are not both zero and its HPF section
     is not elided; in the `neighbour` scenario the twin's lane 3 holds a live state.
   - Disable: apply the disabled pair's targets to lane 1 and render six blocks (the 64-update ramp
     completes mid-block 2), every sample an exact `-0.0`, the twin's own input.
   - After the ramp-completing block (blocks 2 to 5):
     - `bank_lane_state_words(&bank, 1)` is all eight words `+0.0` bits;
     - `bank_elision_plan(&bank)` elides the HPF section on both channels; in the `neighbour`
       scenario the left HPF stays live (lane 3 needs it) and the right HPF is elided under the dual
       body and live under the mono body (lane 3's `Both` target holds both).
   - From the next block on (blocks 3 to 5), lane 1's output, left and right plane, is
     bit-identical to the twin's.
   - Lanes 0, 2 and 3 are bit-identical to the twin's in output, and equal in state words, in every
     block after the disable.

   The state and plan legs prove the clear. In this scenario (a disabled HPF, followed by the
   never-enabled LPF) no output bit depends on a stale HPF integrator (D4), so the output legs
   guard lane isolation, and the bit-identity of lane 1 holds with or without the clear. The test
   disables the HPF only; for a disabled LPF the output can depend on the clear (D4), which this
   gate does not exercise (see Root Amendment 1, open item).

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

- Gate 1's test: red if a disabled section keeps stale integrators (no re-elision), if the plan
  refresh misses a settled section, or if the clear touches a lane that never ramped. (Corrected
  2026-10-11, root direction, after attempt 1.) The state and plan legs that catch the first two
  defects are also read by older tests (see Context), so they are not unique catches. The unique
  catch is the third: a one-plane (`process_mono`) ramp completion that clears the integrators of
  every lane when any lane settles wipes a neighbouring lane's live HPF state, and only this
  test's neighbour-lane output leg turns red on it. The test disables the HPF only and claims no
  output difference from a stale HPF integrator: none exists (D4), so its output legs compare bits
  only as a lane-isolation guard. It does not exercise a disabled LPF, where a stale integrator can
  change the sign of an exact zero (D4). Gate 2 is the evidence that the state legs turn red when a
  clear is removed.

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
| M1: dual body, the two `ic` clear lines (`:1166-1167`) replaced by no-ops | RED (`filter_liveness.rs:1341`, lane 1 state; `:1347` after the follow-ups) | `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths`, `a_disable_in_the_enables_drain_still_completes_and_elides`, `a_disable_freezes_the_recursion_words_and_moves_only_the_mix`, `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time`, `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs` |
| M2: mono body, the two `ic` clear lines (`:1410-1411`) replaced by no-ops | RED (`:1341`; `:1347` after the follow-ups) | `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths` only |
| M4a: both bodies, `identity = done` (drops `current_target_identity`; clears every settled lane) | RED (`:1266` output leg; run on an earlier revision, where the line was `:1265`) | `symmetric_bank_target_keeps_mono_and_dual_paths_equal`, and in the dual run `live_peak_gain_bounds_the_real_kernel_on_its_worst_histories` |
| M4b dual: any settled lane makes the mask all lanes | RED (`:1266` output leg; earlier revision, where the line was `:1265`) | `bank_filter_target_updates_one_member_and_leaves_padding_identity` |
| **M4b mono: any settled lane makes the mask all lanes** | **RED** (`:1266`, output leg) | **none** |
| M4b mono with the state and plan assertions removed from the test | RED (`:1266`, output leg alone) | n/a |
| none (restored) | green | none |

M1 and M2 each turn the matching leg red (spec gate 2), but older tests already catch both, so gate 2
is evidence for the regression guard, not a unique catch. M4a and M4b were run with the first
revision of the history (a sine, replaced by an integer sawtooth because `f32::sin` is a
disallowed method); M1, M2, M4b mono and the restored run were repeated on the final test.

**Deviation: the history's sign (verdict MINOR).** Gate 1 asks for "several blocks of negative
samples" before the disable. The test drives a signed sawtooth (`history_input`, about [-0.4, 0.4))
instead, because it must differ per lane and move the integrators' phase at the disable. It does no
harm: the clear is checked as exact `+0.0` integrator bits, whatever sign the history left, and
the verifier's output-only runs showed the history's sign cannot reach the output (a stale
identity HPF's signed zero is normalised by the following LPF stage in the bank and the twin
alike). Only the `neighbour` scenario and this history shape deviate from the body's letter.

**Follow-ups (after the verdict, PASS).** The assertion context now names the body, the scenario
and the history length (M1 and M2 failed with the same text before); in the `neighbour` scenario
the right HPF's plan entry is checked too (elided under the dual body, where lane 3 holds only a
left HPF; live under the one-plane body, where lane 3's `Both` target holds both). Re-run after
the edit: `filter_liveness` 15 passed; M4b mono (`if L::mask_any(identity) { all lanes }` on the
mono body's `identity`): RED at `filter_liveness.rs:1266`, "left, lane 3, frame 1, mono body,
neighbour true, 6 history blocks, block 0"; M1 (dual clear lines no-oped): RED at
`filter_liveness.rs:1347`, "lane 1 integrators are +0.0 after dual body, neighbour false, 6 history
blocks, block 2"; restored: green (`crates/lane/src/kernels/builtins.rs` byte-equal to base).

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

## Root Amendment 1 (2026-10-11): text aligned with the attempt-1 evidence; no gate weakened

(corrected 2026-10-11 after the spec check: the no-output-bit claim is scoped to the HPF section)

Root direction, after the attempt-1 verdict (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-dfg/1268-attempt1.md`,
PASS). The body made two claims the evidence refutes for the HPF section: that a stale integrator can
change the sign of an exact zero at the output, and the output leg as proof of the clear. One
statement now stands in the Product outcome, D4, gate 1 and Test value, scoped to the HPF section:
with identity coefficients the settled HPF's output is `fma(0, v2, fma(0, v1, 1 * v0))`, where `v1`
and `v2` are the step's outputs (equal to the stale `ic1` and `ic2` bit for bit except where an
integrator is itself a zero), and the LPF stage that follows ends in `v + 0.0`, so a stale HPF
integrator never changes an output bit and only costs work (the section is never elided again); the
state and plan legs prove the clear, and the output legs guard lane isolation. The verifier's
output-only runs (state and plan assertions removed) stay green under both clear mutations, M1 and M2.

The spec check (2026-10-11, verdict `stream-dfg-spec-check`) found that this amendment first stated
the claim for every section. It does not hold for the LPF section: nothing follows it, so a stale
identity LPF over negative integrators and an exact `-0.0` input can emit `-0.0` where the
never-enabled twin emits `+0.0` (measured: bank `0x80000000`, twin `0x0`; the same probe on a stale
HPF gives `0x0` and `0x0`). The original D4's caveat ("no output sample except the sign of an exact
zero") is therefore restored in true form for the LPF in D4, and the Product outcome, Context, D4,
gate 1's closing note and Test value are scoped to the HPF section, with the LPF stated separately.
The Decision record and the Attempt record state the output claim without that scope; read them with
it.

Changed passages:

- **Header:** anchors refreshed against branch head `622aba3b1`.
- **Product outcome:** "signed zeros included" no longer attaches to the clear. It now says the
  ramp-completing block leaves `+0.0` integrators and the plan elides the section, that the output
  is bit-identical with or without the clear, and that the clear's observable effect is state and
  plan (cost).
- **Context, code anchors:** `section_is_identity` `:1146-1163` to `:1590-1599`; `svf_step`
  `kernels.rs:643-652` to `:1076-1077` (and `svf_step_when` `:1093`); the clears `:882-883` and
  `:980-981` to `:1166-1167` and `:1410-1411` (with the `_body` functions at `:1071` and `:1328`);
  `refresh_filter_plan` `lib.rs:1266-1279` to `:1313-1335`, its calls `:1659` to `:1799` and
  `:1944`; readback `lib.rs:5471-5479` to `:5822-5830`; `process` `:3615` to `:3909`,
  `process_mono` `:3631` to `:3925`. Each checked against the files on the branch head. The
  tail test `input_tail_is_infinite_while_a_filter_target_is_ramping` no longer exists (replaced by
  #1329, `11f59fa42`); the passage says so.
- **D4:** from "no output sample except the sign of an exact zero" to "no output bit, the sign of an
  exact zero included", with the equation.
- **Gate 1:** states what the PASSed test asserts: both bodies, the plain and the `neighbour`
  scenario, history lengths `6..14`, signed sawtooth history (negative samples included), state
  words `+0.0` and HPF elided from the ramp-completing block (with the neighbour's plan entries),
  lane 1 bit-identical to the twin from the next block, other lanes identical in output and state
  every block. The command is unchanged.
- **Test value:** "The test makes no signed-zero claim" now reads that the test claims no output
  difference from a stale HPF integrator, because none exists, and that it does not exercise a
  disabled LPF.
- **Gate 1, state words (spec-check correction):** "lane 1's first two state words are not zero"
  now reads "not both zero", which is what the test asserts (`assert_ne!(&live[..2], &[0, 0])`,
  `filter_liveness.rs`), and the neighbour's live-state check uses the same assertion.

**Open item for root.** Gate 1 disables the HPF only, so none of its output legs can see the clear for
the LPF section, where a stale integrator can change the sign of an exact zero at the output (D4).
Whether gate 1, or a sibling test, should also disable the LPF (a rested real HPF, an exact `-0.0`
input, negative stale LPF integrators) is for root to decide. This amendment adds no such gate.

Not weakened: gate 1 asserts everything the test asserts (the earlier text asked for less: no
`neighbour` scenario, no history lengths, no other-lane state words); gate 2 (the mutations) and the
defect named in Test value are untouched. The Decision record and the Attempt record are history
and stay as written, including their "left as filed" wording about the Context, which this
amendment now supersedes.
