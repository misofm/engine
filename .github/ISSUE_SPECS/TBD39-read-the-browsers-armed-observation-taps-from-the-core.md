# Read the browser's armed observation taps from the core

Successor of #1679 (the successor's armed bits come from the core's mask) and #1677 (observation
records addressed by stable key in the core), decision 17 C1 and C13. Code anchors verified on the
batch 15a branch `codex/controls-batch-15a` at `e8f72edc5`, and read-only on the in-progress #1310
tree `codex/controls-batch-15aw` (uncommitted). Issue number: assigned at filing (written here as
TBD39). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

The browser holds no copy of which observation taps are armed. The core already knows: the control
plane's `SessionState` keeps one observation slot per declared tap (armed or not, and whether a
record waits in the queue of a plan render has not adopted) and the armed bindings by tap key. The
browser reads that state wherever it needs it (the meter fold, the window read, the swap's tap
hand-over, the adoption of a replacement), so a subscription behaves the same after a rebuild, a
supersession and an adoption because one owner holds it. Decision 17 C1: "A behaviour that exists
on one host only is a defect"; a second copy of the armed state that only the browser keeps and
re-derives at every plan change is host logic.

## Context

- **The copy.** `PlanShape` (`hosts/host-web/src/lib.rs`) holds, per plan and per dense effect
  slot, `observation_armed: Box<[u32]>` (`:1390`, one bit per declared tap) and
  `observation_commanded: Box<[u32]>` (`:1402`, a bit when a tap command was admitted against this
  shape's plan), beside `observation_arm_samples` (`:1396`, the admission sample of the current
  arm). They are written in three places:
  1. `record_observe` (`:2003`): at admission of a record the control plane has just queued, it
     sets or clears the armed bit and sets the commanded bit;
  2. `build_plan_shape` (`:5716`, the loop at `:5846`-`:5906` of that function): a fresh shape takes
     `EffectObservationHandle::armed_taps` (`crates/effect-compiler/src/prepare.rs:1804`), which
     #1679 made the core's mask, copied "not repeated"; the commanded bit starts clear;
  3. `PlanShape::carry_tap_state` (`:6005`, called at the swap from `:1832`): for each carried
     reader, a tap whose commanded bit is clear takes the predecessor's armed bit and arm sample;
     a tap commanded against the successor keeps its own. Then the bits are cleared.
- **The readers of the copy.** The meter fold (`:3925`-`:3950`, skips a slot with `armed == 0`),
  `read_observation_values` (`:4493`-`:4512`, answers `Unarmed` from the bit), and the test hook
  `observation_armed_taps` (`:2700`, summed popcount; the tests call it at `tests.rs:2493`,
  `:4333`, `:7554`, `:7581` and others).
- **What the core holds.** `SessionState` (`crates/control-plane/src/control.rs`) keeps
  `observation_pending: Box<[ObservationSlot]>` (`:426`; `ObservationSlot` at `:670`: `armed`,
  `pending`, `mark`, `key_bytes`), an armed count (`observation_armed()`, same file), and the
  armed bindings by key (`observation_bindings()`, `:3019`, `controls::ObservationBindings` with
  `window_of(&TapKey)`). Admission (`admit_and_push_observation_batch`) is the one way a host arms
  or disarms a tap (#1676), and the browser's `record_observe` runs after it, so the two copies are
  written by the same call and held equal by that sequence, not by one owner.
- **#1310 adds a third writer (verified, not yet in the browser).** In the uncommitted tree,
  `ObservationSlot` gains `queued` ("a record for this tap waits in the queue of a plan render has
  not adopted"), and `observation_forwards` (`control.rs:857`) re-queues, on the replacement B, the
  bound state of every tap whose slot is `queued` on a withdrawn candidate A, because a tap B
  carries from the running plan P0 takes P0's state at the swap and would lose the record. The
  batch 15a fold is to copy B's bits into the browser's shape from those forwards after a
  superseding commit. `hosts/host-web/src/lib.rs` is not modified in that tree as read; the fold
  is the coordinator's statement, and this issue starts by re-reading it.
- **The rule that decides the carry** is the same in both copies and exists once in the core after
  #1310: a tap carried across a swap takes the running plan's state unless a command named it
  against the successor. The browser's `observation_commanded` is that "named it" bit; the core's
  `queued` is its counterpart for a plan not yet adopted.
- **Tests that pin the copy.** `the_browsers_armed_bits_are_the_mask_host_core_prepared`
  (`tests.rs:7890`, #1679), `an_armed_tap_and_its_windows_follow_the_effect_through_a_swap`
  (`:7540`), `a_tap_command_admitted_at_sample_zero_survives_the_hand_over` (`:7607`),
  `a_restarted_effects_subscription_keeps_publishing_windows_on_the_browser` (`:7801`),
  `a_removed_inserts_subscription_ends_with_an_event_and_has_no_slot` (`:7988`); the qualification
  rows `observationTimeline` in `hosts/host-web/tests/browser-v1/expected.json` (`:122`) and the
  `resourceCeilings`, driven by `browser-correctness.js`.

## Decisions to take (the implementer proposes, the coordinator rules)

- **The core accessor.** What the core returns for the browser's questions: for a producer
  position and a tap, `armed` and the window (from the slots and bindings), and for the swap and
  the adoption, the state the carry rule resolves. The proposal is one read-only function on
  `SessionState` that takes the producer position and tap index and allocates nothing, so the
  worklet's message handler and `process()` can call it (no allocation in `process()`).
- **What replaces `observation_arm_samples`.** The arm sample is the transport's freshness stamp
  (a resident cell keeps its last window, so a read is fresh only if the window starts at or after
  the arm sample). Decide whether it stays a browser-side render-transport value (it is not an
  "armed" fact; it is a sample of this host's block clock) or moves with the bits. The proposal
  keeps it in the shape as transport, and the issue states why it is not a second copy of
  state: it holds a sample number, never a bit that could disagree with the core.
- **The adoption read.** At the swap block the browser must know, for a carried tap, whether a
  command named it against the successor. Decide whether the core's `queued` flag (or a sibling
  the implementer adds) is that fact, and prove with a test that it equals
  `observation_commanded` for every sequence the five tests above drive (command before the
  commit, after the commit before adoption, at sample zero, across a supersession).
- **Where the read happens.** The meter fold runs in the worklet's message handler on a
  non-isolated page and in the worklet on an isolated one; the issue names which instance may
  read the control plane's `SessionState` there (decision 15, D15-10), or the core publishes the
  armed words on the plan's render-half state for the worklet to read without a lock.

## Deliverables

1. The core accessor of the first decision, with unit tests in `control-plane`.
2. `PlanShape::observation_armed` and `PlanShape::observation_commanded` deleted, with
   `record_observe`'s mask and commanded writes, `build_plan_shape`'s copy of
   `EffectObservationHandle::armed_taps`, and `carry_tap_state`'s armed-bit merge. The meter
   fold and `read_observation_values` read the core. `observation_armed_taps` (test hook) reads
   the core's count.
3. After #1310 lands, the superseding commit's forwards reach the browser through the same read;
   no browser code copies `observation_forwards`' result.
4. The #1679 test is deleted (its subject, a browser copy of the mask, no longer exists); the
   others are rewritten only where they named a deleted field.

## Authorized paths

- `hosts/host-web/src/lib.rs` and `hosts/host-web/src/tests.rs`.
- `crates/control-plane/src/control.rs` and `crates/control-plane/src/compile.rs` (the accessor;
  the slot tables' sizing only if the accessor needs a new field), and their tests.
- `crates/host-core/src/prepare.rs` and `crates/effect-compiler/src/prepare.rs` only if
  `EffectObservationHandle::armed_taps` has no remaining reader and is deleted.
- `hosts/host-web/tests/browser-v1/expected.json` only for a re-pin the gates below measure, with
  its reason.
- This spec file's decision record.

## Non-goals

- No change to which taps an effect declares, to the window length, to the observation wire, or to
  `OBSERVATION_ENDED`.
- No change to the C ABI: it has no copy to delete, and its tap state already comes from the same
  core.
- No move of the meter fold or the window cells (decision 17 C13's later slices).
- No allocation, lock or syscall added to `process()`; no new field behind a `cfg` for one target.

## Objective gates

1. The browser's observation tests (`structural_apply` and the `selected_observation_*`,
   `observation_*` rows of `hosts/host-web/src/tests.rs`) and the qualification's observation
   rows pass unchanged, apart from the deleted #1679 test and tests that named a deleted field.
2. A subscription acked across a plan rebuild, across a supersession (#1310: the second structural
   commit while the first waits unadopted) and across the adoption block publishes windows: a new
   test drives each of the three sequences and reads at least one window after each, with the
   subscription taken before the rebuild and no re-arm.
3. The browser source holds no per-shape armed or commanded state, **proved by type**: the
   fields `observation_armed` and `observation_commanded` are gone from `PlanShape`, and no type
   in `hosts/host-web/src` has a field of the shape `Box<[u32]>` indexed by dense effect slot that
   holds tap bits. The proof is that the crate compiles and the tests above run without them, not
   a source grep.
4. `allocations == 0` in `process()` after warm-up for the observation path, measured with
   `bench_support::alloc` (the existing `a_bus_observation_admits_and_renders_without_allocating`
   passes); the realtime policy check passes.
5. `cargo test -p host-web --all-features`, `-p control-plane --all-features`,
   `-p host-core --all-features`; `cargo clippy -D warnings` and `cargo fmt --check` on the
   touched crates; `scripts/check-workspace-policy.sh`; the browser qualification observation
   rows (run on a machine with the browser matrix; record if not run).

## Test value

- The sequence test (gate 2): plausible defect that turns it red and no existing test catches: a
  subscription acked while a candidate waits, then superseded by a second structural commit, that
  stops publishing after the adoption because the replacement's armed state came from a source
  that never saw the queued record. Existing browser tests cover rebuild and adoption separately
  and take P0's bits; none drives a supersession through the browser's tap state.
- The equivalence test for the adoption read (third decision): red if the core's "named against
  the successor" fact differs from the deleted `observation_commanded` bit for any drive of the
  five existing sequences; the deleted bit is the oracle, so the test is written before the
  deletion and kept as PR evidence, not committed (a one-time comparison against a pre-change
  base, per the repository rule).
- The #1679 test is deleted, not rewritten: its claim, that the browser's bits equal the core's
  mask, holds by construction once there is one owner.

## Dependencies

- #1310 (batch 15a), for the supersession sequence and the forwards; this issue starts after it
  merges.
- #1679 (the mask the browser copies today) and #1677 (the stable-key addressing the core's slots
  and bindings use).

## Size

Half a day.

## Decision record: TBD39

To be filled by the implementer. The accessor decision goes first.
