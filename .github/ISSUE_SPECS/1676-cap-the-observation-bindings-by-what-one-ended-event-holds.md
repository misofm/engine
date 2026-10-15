# Cap the observation bindings by what one ended event holds

Successor of #1584 D6a (batch 12a, root 2026-10-15), decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `45a2ab9df`
(PR #1675). Issue number: assigned at filing (written here as #1676). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A subscription the engine acknowledges can always be ended honestly: no commit that removes a
subscribed target is refused because its `OBSERVATION_ENDED` event would not fit one control
frame, and the memory reserved for that event follows the bindings the session can hold, not the
frame size.

## Context

- #1584 D6a ends every binding whose target is gone with one `OBSERVATION_ENDED` per commit
  (root ruling A). The event must fit `maximum_control_frame_bytes`; a commit whose ended keys do
  not fit is refused as a resource limit (`crates/control-plane/tests/observation_bindings.rs`,
  `an_end_event_over_one_control_frame_refuses_the_removal_and_changes_nothing`). So a valid
  removal can be refused, and the app must disarm first.
- The batch 12a review fold charges each `OBSERVATION_ENDED` cell a flat ceiling bounded by the
  frame size (`ended_event_ceiling`, `crates/control-plane/src/compile.rs:572`;
  `observation_reserve_bytes`, `:553`). The reserve grows with `maximum_control_frame_bytes`
  (about 4x at the browser's 16 KiB frame against the 4 KiB reference); `capi_retained_bytes`'
  ceiling rose from 181,696 to 205,312 with it.

## Decisions frozen for this slice

- **D1. Cap at admission.** A subscribe (an Observe that would add a binding) is refused with a
  typed refusal before any ack when the bindings table already holds as many keys as one ended
  event can carry in one control frame (computed once at preparation from the frame limit and the
  largest encoded key the session's IDs allow). Nothing is queued and no state moves.
- **D2. Reserve by capacity.** The ended-event reserve per cell is the smaller of the frame bound
  and the encoded size of the bindings capacity (the slots the plan has), so a session with few
  observation slots reserves little whatever the frame size.
- **D3. Same on both hosts**, with a parity row.

## Objective gates

1. With a small `maximum_control_frame_bytes`, the subscribe that would exceed the cap is refused
   with the typed code, and a removal of every subscribed target afterwards commits with one event
   holding every key (the over-frame refusal can no longer be reached by a valid removal).
2. `capi_retained_bytes` on the reference session falls back toward its pre-fold figure at the
   4 KiB frame, and the reserve no longer scales with the frame for a plan with few slots; the
   ceiling is re-pinned to the measured value (plus 10 %, rounded up to 64) at both lane widths.
3. The charge-equals-retained capi tests stay exact; `cargo test --locked -p control-plane
   --features test-support -p capi -p host-web --features host-web/test-support`; clippy.

## Authorized paths

- `crates/control-plane/src/{compile.rs,control.rs}`, `crates/controls/src/observation.rs`,
  `crates/protocol` (the refusal code, if a new one is needed), their tests,
  `crates/capi/tests/resource_lifecycle.rs` (the ceiling), docs, this spec's decision record.

## Dependencies

- #1584 D6a (PR #1675).

## Size

Half a day.

## Decision record: #1676 (batch 13b)

**What shipped.**

- **D1, the cap.** `control_plane::compile::observation_binding_cap(frame_bytes)` measures, with the
  real codec, the `OBSERVATION_ENDED` frame of one key and what a second adds, with both IDs at
  `STABLE_ID_MAXIMUM_BYTES` (127), and returns how many keys one control frame holds (1 KiB frame:
  3, 4 KiB: 12, the browser's 16 KiB: 51). `SessionState` keeps it (`observation_cap()`), with an
  exact `observation_armed()` count. Each observation slot now carries `armed` (the table's state
  with every push folded in) beside its pending push, so a subscription is judged without a key
  lookup or an allocation inside `process()`. A rebuild rebuilds the slots from the bindings it
  keeps (`observation_slots(producers, bindings)`), so the count survives a swap and drops the
  taps a transaction ended. `SessionState::admit_observation_changes(changes)` admits a batch by
  the taps it leaves armed (arms of unarmed taps add, disarms of armed taps remove, a window change
  adds nothing) and returns `ObservationLimit { index, cap }` for the first arm that crosses. It
  moves nothing, so a refusal is whole and precedes every ack.
- **Browser mapping.** `admit_commands` calls it after pass one and before the room pass; a
  refusal is `RESULT_BACKPRESSURE` / `COMMAND_REASON_BACKPRESSURE` at the record that crossed it
  (doc comments on the Rust constant and both `.d.ts` copies say so). Nothing else in the adapter.
- **D2, the reserve.** The ended-event heap per cell is now `min(slots, cap)` keys times
  (`size_of::<ObservationTapKey>()` + two maximum IDs), not the frame size. The binding table and
  its transient copies use the same `min(slots, cap)`.
- **Charge by high water.** The slots in that charge are the most any committed plan of the session
  has had (`observation_slots_high_water`, passed to `prepare_runtime`), not only the plan being
  prepared. A cell holds an event until its frame is delivered, so an event a larger earlier plan
  emitted can outlive that plan; charging the shrunk plan by its own slots would let a cell hold
  more than its row says. The first plan charges its own slots, so the reference figures are exact.
- **Docs.** `docs/CONTROL_PROTOCOL_SEMANTICS.md` (observation paragraph) states the cap and its
  figures.

**Departures.**

1. **Refusal code.** The browser's `COMMAND_REASON_*` vocabulary is gate-pinned as contiguous from
   0 across six files, and 12-14 are retired and never reused; a new reason would need a vocabulary
   decision. The typed refusal is `ObservationLimit` in the core; the browser maps it to the
   existing backpressure reason (unsubscribe a tap, then retry succeeds). No `crates/protocol`
   change and no re-pin. A dedicated reason is successor work if the owner wants one.
2. **Parity row.** Only the browser subscribes today (the C ABI's `Observe` edit is #1584 D1-D5 and
   `observation_taps` stays 0 there), so there is no C ABI path to put in
   `hosts/host-web/tests/control_parity.rs`. Both hosts share the one core cap, the one reserve
   and the one slot state; the C ABI edit must call `admit_observation_changes` (or its own edit
   check against `observation_cap()`).
3. `push_effect_observe` does not itself refuse past the cap: a batch can push its arms before its
   disarms, so the cap is judged on the batch, by `admit_observation_changes`, which every caller
   must call first. The over-frame refusal in the commit stays as a defence; its existing test now
   says it pushes past the cap on purpose.

**Gates and tests.**

- Gate 1: `control-plane/tests/observation_bindings.rs`
  `a_subscription_past_the_cap_is_refused_and_the_removal_of_all_still_commits` (small frame; the
  cap+1 arm is refused with index and cap, a disarm in the batch makes room, re-arming an armed tap
  adds nothing, admission moves nothing, and removing the strip commits with one event holding all
  `cap` keys), `the_cap_is_what_one_frame_carries_with_the_longest_ids` (cap longest keys fit the
  frame, one more does not, at 1/4/16 KiB), `the_armed_count_follows_the_bindings_across_a_rebuild`;
  browser: `host-web` `a_subscription_past_the_cap_is_refused_and_an_unsubscribe_makes_room`.
- Gate 2: `the_reserve_follows_the_slots_and_a_shrunk_plan_keeps_the_larger_reserve` (the reserve
  at 4 KiB equals the reserve at 16 KiB for a few slots; a plan that shrank from 14 to 8 slots
  charges at least the larger reserve). `capi_retained_bytes` on the reference session measured
  165,782 (x86-64, 4 KiB frame; the row does not depend on the lane width; it was 186,614 after the
  12a fold and 165,758 before it), so the ceiling is re-pinned to 165,782 plus 10 %, rounded up to
  64: 182,400, at both widths (was 205,312). The four-lane figure is not measured here (no AArch64).
- Gate 3: `capi` `resource_lifecycle` (charge equals retained, exact) and the full crate suites
  pass; clippy `-D warnings` on control-plane, controls, capi and host-web, all targets and
  features.

**Successor candidates.** A dedicated browser command reason for the cap (needs a vocabulary
decision). The C ABI `Observe` edit must admit against `observation_cap()`. 51 simultaneous taps at
the browser frame is the product limit; a larger frame raises it linearly.

### Attempt 2 (batch 13b)

**Root ruling (2026-10-15).** A cap refusal is never backpressure. The browser vocabulary gains
command reason 15, `observationLimit`; reasons 12-14 stay retired and are never reallocated, and the
declared values are exactly `range(0, max + 1)` minus `{12, 13, 14}`. This supersedes attempt 1's
departures 1 (the cap reported as `backpressure`) and 3 (an uncapped public push).

- **The reason, in every spelling.** `COMMAND_REASON_OBSERVATION_LIMIT = 15` in
  `hosts/host-web/src/lib.rs` (result `RESULT_UNSUPPORTED`, the same result as
  `observationUnbound`: the session's configuration cannot take one more tap, and only an
  unsubscribe changes that); the host JS table (`null` at 12-14; `validCommandReason` refuses a
  `null` entry); the `.d.ts` `MisoCommandReason.ObservationLimit = 15` and its SDK mirror; the
  metadata generator and the ABI-layout generator rows (`tools/parameter-metadata`); the schema
  gate's own list; the ABI-layout gate's list (now explicit `(value, name)` pairs). The SDK assets
  and `sdk/src/generated/{abi,catalog}.ts` were regenerated with `node codegen/assets.mjs` and
  `node codegen/generate.mjs`. The C ABI header declares no command reasons, so nothing changes
  there. No schema hash or ABI digest is pinned for these files, so nothing was re-pinned
  (`abiVersion` stays `65536`: a new reason value is additive).
- **Gates for the gap.** `check-command-reason-vocabulary.py`, `check-parameter-metadata-v1.py`
  and `check-abi-layout-v1.py` require "dense except the retired set". The vocabulary gate reads
  the JS table's `null` holes (a hole at a non-retired value is red), takes the generator's values
  from the Rust constants, maps the schema gate's hole-free list onto the non-retired values, and
  requires the schema gate's `RETIRED_COMMAND_REASONS` to be its own. New red mutations: the JS
  bound accepting a retired `null`; the JS table losing a hole or holing a live value;
  `observationLimit` moved to 12 or to 16 in the Rust constants; dropped from the `.d.ts`, the
  generator or the schema list; on 12 in the `.d.ts`; the schema retired set losing 14; plus eight
  direct cases of the dense rule (31 mutations in all). The metadata gate's self-test gains four
  (reallocated 12, gap at 16, missing 15, missing 11) and the ABI-layout gate's two. The on-disk
  drift mutation in `test-web-audioworklet.sh` now adds a reason at 16. The hermetic worklet test
  (`scripts/test-web-audioworklet.mjs`) settles a reason-15 acknowledgement as a typed refusal with
  result 7 and a healthy host afterwards.
- **SDK.** `#submit` already resent only on `backpressure`; a comment now names
  `observationLimit` among the reasons thrown at once, and `commandFailure` names the reason and the
  record in the message, with a hint for the cap ("unsubscribe another tap first").

**Review findings.**

1. *Short IDs let the removal half pass with no cap.* The core test
   `a_subscription_past_the_cap_is_refused_and_the_removal_of_all_still_commits` now renames
   `comp1` to a 127-byte strip ID and adds 127-byte insert IDs at the 1 KiB frame, so the `cap`
   keys of the removal fill the frame. Its second half pushes `cap + 1` taps past admission (the
   test-support hook `test_push_observe_unadmitted`) and requires the same removal to be refused
   by the over-frame defence (`ResourceFault::Limit`). **Red without the fix:** raising the cap by
   one (`observation_binding_cap(..)? + 1` in `compile.rs`) turns it red at the removal (`cap + 1`
   longest keys are refused); run and observed red, then restored.
2. *`push_effect_observe` was public and uncapped.* It is now `pub(crate)`. The one public entry
   is `SessionState::admit_and_push_observation_batch(changes)` (`ObservationChange` items): it
   runs the cap check, then the queue-room check (moved from the browser into the core, since the
   queues are the core's), then queues every change, and returns `ObservationRefusal::{Limit,
   Full, NoProducer}` with nothing written on a refusal. A `debug_assert!` checks armed <= cap
   after the push, in the core and again in the browser after its admission. The browser's
   `admit_commands` calls it once after pass one; the room-refusal order is unchanged (cap, then
   room, both before any write) and its own host-side state (armed masks, arm samples, in-flight
   counts) is written only after the core admits. One departure in order: the browser's in-flight
   counter overflow check (unreachable: the queue room bounds it far below `u32::MAX`) now runs
   before the core call, because nothing may be refused after a push. No token was needed. Tests
   that push without admission use the test-support hook only.
3. *A refusal at record index > 0.* The host-web test submits `[unsubscribe 0, subscribe cap,
   subscribe cap + 1]` with the cap full and requires `observationLimit` at rejected index 2, with
   nothing kept. The core test also requires `ObservationLimit { index: 1 }` for a batch whose
   first change re-arms an armed tap. **Red without the fix:** mapping the crossing change to index
   0 in `lib.rs` turns the host-web test red (`rejected_index` 0, expected 2); run and observed.
4. *The host-web test expected backpressure.* `a_subscription_past_the_cap_is_refused_and_an_unsubscribe_makes_room`
   now expects `(RESULT_UNSUPPORTED, COMMAND_REASON_OBSERVATION_LIMIT, 0, 0)`. **Red without the
   fix:** mapping the cap back to `COMMAND_REASON_BACKPRESSURE` turns it red (6, 8 against 7, 15);
   run and observed.
5. *The SDK must not resend the cap.* New eval in `sdk/test/observation-subscription-evals.mjs`:
   a scripted host refuses one subscription with result 7, reason 15; the call rejects at once with
   a `MisoEngineError` whose message names `observationLimit at record 0`, and exactly one command
   was sent although a render happens 30 ms later. **Red without the fix:** letting `#submit` retry
   on `observationLimit` turns it red; run and observed.

**Docs.** `docs/CONTROL_PROTOCOL_SEMANTICS.md` (observation paragraph) now says the browser reports
the cap as `RESULT_UNSUPPORTED` with reason `observationLimit` and that the core admits and queues a
batch in one call. `hosts/host-web/MUTATIONS.md` lists the new mutations.

**Review fold (batch 13b).** Three MINOR findings on attempt 2 (`fba4eeab8`, PASS).

1. *The core only documented "each tap named once".* `admit_and_push_observation_batch` now
   refuses a second naming of a (producer position, tap index) in one batch with the new
   `ObservationRefusal::Duplicate { index }`, at the second naming and before anything is pushed;
   the precondition sentence is gone from the doc. The browser maps it to `MALFORMED` at that
   record, the answer its own pass one already gives a duplicate, so browser behaviour does not
   change. Test: `a_batch_that_names_one_tap_twice_is_refused_at_the_second_naming`
   (`crates/control-plane/tests/observation_bindings.rs`), at the 1 KiB frame with the cap full:
   `[disarm A, arm A, arm B]` and `[arm B, arm B]` are each `Duplicate { index: 1 }`, move nothing
   and queue nothing, and a later batch naming each tap once is admitted. Every shipped effect
   declares one tap, so its queue has one slot and the cap+1 outcome itself is not reachable from a
   test: the batch is reached through the public core call, and the room check masks the
   undercount. **Red without the fix:** with the check disabled, the first batch is `Full
   { index: 0 }` and the second `Limit { index: 0 }`; both run and observed red, then restored.
2. *The room check was O(n^2).* Admission is now three linear passes over the batch with no
   allocation: pass 1 marks each change's slot and counts each producer's records, pass 2 is the
   cap, pass 3 the room, reading one count per producer. The scratch is a per-slot `mark` in
   `ObservationSlot` and a per-producer `ObservationBatchCount { mark, needed }` box built with the
   plan's slots in `observation_slots` (and charged as a retained row beside the offsets); a batch
   stamp makes stale marks invalid without clearing, and on the stamp's wrap every mark is reset
   (bounded, allocation-free). Refusal order: **duplicate** (the first second-naming in batch
   order, over the whole batch), then **cap** (the arm that crosses), then **no producer or full**
   (one check, the first offending record in order). A duplicate therefore wins over a cap or room
   refusal at an earlier record, because the cap count is only sound once duplicates are out.
3. *`hosts/host-web/MUTATIONS.md` was stale.* The vocabulary self-test row now says thirty-one
   in-memory mutations (the script reports 31 plus 8 dense-rule cases), and the metadata self-test
   row says reason 11 is renumbered to 16, and lists #1676's four new mutations.
