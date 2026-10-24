# Publish each observation tap's arm state from render and return arm tags

Slice S26a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Split from the planner's S26: this slice is the arm state; #1586 is the read command. It
delivers the #1057 design note's P7 (`docs/handoffs/one-edit-api/1057-design-note.md:1128`,
section 8.4 at `:1414`), for which no issue was filed. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

Each observation tap tells its readers, together with its latest window, whether it is armed, the
first sample of the block that armed it, and the arm tag of the edit that armed it. Render writes
these three facts when it applies the record, so a reader on any thread sees exactly what render
applied. A transaction's reply lists the arm tag of each `Observe` edit, and the tags of the
subscriptions that a structural commit removed. The browser's host-side arm mask and arm samples
are deleted.

## Context

- **The slot.** `ObservationSlot` (`crates/engine/src/realtime/observe.rs:75`) holds one window
  under a single-writer sequence lock; `ObservationPublisher::publish` (`:127`),
  `ObservationReader::read` (`:169`, bounded by `MAXIMUM_READ_ATTEMPTS` `:71`), `acknowledge`
  (`:195`).
- **Render already knows the arm.** `ObservationLane::arm` (`crates/effect-contract/src/live.rs:1258`)
  sets the tap's window at the block's first sample; `apply_observe` (`:1104-1112`) calls it at
  block entry. It publishes nothing about the arm.
- **The browser writes it on the wrong thread.** At admission, `ReadyOwnership::push`
  (`hosts/host-web/src/lib.rs:1936`) writes `observation_armed` (`:1747`, write `:1973`) and
  `observation_arm_samples` (`:1752`, write `:1978`); both are built in `compile_ready`
  (`:7390-7447`) and read by the meter fold (#1582's `GainReductionTaps`), by
  `read_observation_values` (`:6207`, the arm-sample read `:6223`) and by
  `observation_armed_taps` (`:2478`). With the control half in a Worker (#1332), render-read state
  written there is the race the design note's F3 forbids.
- **#1534's predicate.** A window counts only when `first_sample >= arm_sample`. The first window
  after an arm opens at the arm block's first sample, so the slot's arm sample is the same number
  the browser stores today.
- **The bindings.** #1584's binding table on the protocol controller holds one `TapBinding` per
  armed tap; #1584 D6 builds a successor's armed taps from it (`ArmedTap`).
- **The record.** `EffectControlRecord::Observe` (`live.rs:123`); the enum's size bound is
  `crates/effect-contract/tests/live_control.rs:36` (at most 64 bytes).

## Decisions frozen for this slice

- **D1. The record carries a tag.** `EffectControlRecord::Observe` gains `arm_tag: u64`. `0` means
  "no tag" (the browser's kinds 7 and 8 until #1589). The size bound stays.
- **D2. The slot carries the arm state.** `ObservationSlot` gains `armed: AtomicU32`, `arm_sample:
  AtomicU64` and `arm_tag: AtomicU64`. `ObservationPublisher::publish_arm(armed: bool, arm_sample:
  u64, arm_tag: u64)` stores them under the same sequence lock as `publish`. New
  `ObservationReader::read_state(&self) -> Option<ObservationState>`, `ObservationState { window:
  Option<ObservationWindow>, armed: bool, arm_sample: u64, arm_tag: u64 }`, reads all of them in
  one locked read with the same retry bound (`None` when it gives up). Before any arm the state is
  unarmed, sample `0`, tag `0`. `read` stays.
- **D3. Render writes it.** `ObservationLane::arm` takes the tag and calls `publish_arm(armed,
  first_sample, arm_tag)` for the tap. A tap that #1584 D6 builds armed publishes its arm state at
  its first rendered block, with `ArmedTap::arm_tag` (new field). The carry's drain of pending
  records (#1280 D3) passes the record's tag through. Nothing else writes the arm state.
- **D4. The controller issues tags.** `ProtocolController` keeps `next_arm_tag: u64` beside the
  binding table (#1584 D3), starting at 1. The transaction apply gives each net record of the token
  the next tag, in record order, on the scratch copy; `checked_add` overflow refuses the transaction
  as an internal error. `commit_prepared_structural` installs the counter with the table, so a
  dropped token issues no tag. `TapBinding` gains `arm_tag`, the tag of the record that armed it,
  and the control plane pushes each record with its tag.
- **D5. The reply.** `TransactionApplied` gains two optional fields (schema fields 3 and 4 after
  #1313's `edit_path`): `observe_arm_tags` `PackedU64`, present exactly when the transaction holds
  `Observe` edits, one entry per `Observe` edit in transaction order: the tag of the tap's binding
  after that edit (a join reports the binding's existing tag; a disarm that releases the binding
  reports its disarm record's tag; a disarm that keeps it reports the binding's tag); and
  `removed_arm_tags` `PackedU64`, present exactly when the commit dropped bindings (#1584 D6), the
  dropped bindings' tags in `TapKey` order.
- **D6. Delivery reads the slot.** host-core's `GainReductionTaps` implementation (#1582 D3) reads
  `armed` and `arm_sample` from `read_state`. host-web deletes `observation_armed`,
  `observation_arm_samples`, their writes in `push`, their construction and resource rows in
  `compile_ready`, and reads the slot in `read_observation_values` and `observation_armed_taps`.
  #1534's predicate is unchanged; only its input moves. The browser's kinds 7 and 8 push records
  with `arm_tag: 0`. Browser bytes do not change.
- **D7. Docs.** The protocol registry lists the two reply fields; the semantics document says that
  an `Observe` is in effect when a read reports its tag armed (#1586), not when the reply returns.

## Deliverables

1. D1 and D3 in `crates/effect-contract/src/live.rs` and the carry drain; D2 in
   `crates/engine/src/realtime/observe.rs`.
2. D4 and D5 in `crates/protocol` (the counter on the controller and the token,
   `TransactionApplied`, its schema, the corpus) and the record push in
   `crates/control-plane/src/control.rs`.
3. `ArmedTap::arm_tag` in `crates/host-core/src/prepare.rs` and `crates/effect-compiler/src/prepare.rs`.
4. D6 in `crates/host-core/src/meters.rs` and `hosts/host-web/src/lib.rs`; D7.

## Authorized paths

- `crates/engine/src/realtime/observe.rs`, `crates/effect-contract/src/live.rs`,
  `crates/effect-contract/tests/live_control.rs`, `crates/rack/src/lib.rs` and
  `crates/graph/src/runtime.rs` (only where they call `arm` or drain `Observe`).
- `crates/effect-compiler/src/prepare.rs`, `crates/host-core/src/{prepare.rs,meters.rs}`,
  `crates/host-core/tests/effect_observation.rs`.
- `crates/control-plane/src/control.rs`.
- `crates/protocol/src/{message_wire.rs,message_wire/tests.rs,schema.rs,controller.rs,controller/tests.rs}`,
  `crates/conformance/src/protocol_corpus.rs`, `fuzz/corpus/complete-schema-manifest.md`.
- `hosts/host-web/src/{lib.rs,tests.rs}`.
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS}.md`.

## Non-goals

- No read command (#1586). No browser record change (#1590 deletes the browser records).
- No change to the window fold or to #1534's rule.

## Objective gates

Commands: `cargo test --locked -p engine -p effect-contract -p rack -p graph`,
`cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p host-web --all-features`, `cargo run --locked --release -q -p realtime-policy`,
and #1534 gate 3's worklet commands.

1. **Render publishes the arm.** In `effect_observation.rs`: push an arm record with tag 7 and
   render one block starting at sample `S`; `read_state` returns `armed`, `arm_sample == S`,
   `arm_tag == 7` before any window closes; a disarm with tag 8 returns `armed == false`, tag 8.
   Every bank lane and the per-node path give the same state (the legs of
   `every_observation_leg_renders_the_corpus_to_the_same_bytes`, `:289`).
2. **Tags in the reply.** Through the C ABI: arm (tag 1), a second subscriber's arm (reports 1),
   one disarm (reports 1), the last disarm (reports 2). A commit that removes the effect of an
   armed tap reports its tag in `removed_arm_tags`; a commit that removes none has no field.
3. **Torn reads are impossible.** A stress test publishes arms and windows in a tight loop on one
   thread while another reads `read_state`: every result is one consistent publication (its
   fields come from one call), as the existing slot stress test checks for windows.
4. **Browser unchanged.** The browser tests of #1534 (`a_resubscribed_tap_leaves_the_meter_frame_until_its_first_new_window`),
   `selected_observation_reads_are_bounded_stable_and_non_consuming` (`hosts/host-web/src/tests.rs:6318`)
   and `the_meter_frame_carries_the_app_shaped_gain_reduction` (`:6670`) pass unchanged with the
   arrays deleted.
5. **Realtime.** Rendering with arm and disarm records after warm-up makes `allocations == 0 &&
   frees == 0` (`bench_support::alloc`) on both hosts.

## Test value

- Gate 1 turns red if the arm state is written anywhere but render, or a lane leg publishes a
  different arm sample.
- Gate 2 turns red if a client cannot match a reply to the arm it caused, or a removed
  subscription leaves no trace.
- Gate 3 turns red if the arm state and the window can be read from two different publications.
- Gate 4 turns red if moving the arm state changes a browser byte or #1534's rule.
- Gate 5 turns red if publishing the arm state allocates on render.

## Dependencies

- #1584 (the overlay, the bindings, the armed successor taps), #1582 (`GainReductionTaps`).
- *Fold only windows published since a tap's arm into the meter frame's gain reduction* (#1534):
  lands first; this slice moves its input.
- *Carry live-controlled effect lanes across a plan swap* (#1280): the carry drain that passes the
  tag. `crates/effect-contract/src/live.rs` follows `STREAMS.md` row 82 (B #1312, B #1345, A #1280,
  E #1341); this slice rebases after them. `hosts/host-web/src/lib.rs` is stream H's file.

## Supersedes and shrinks

- None.

## Decision record: slice #1585a (2026-10-24, attempt 2)

Split by root (scope note, R1-R3 accepted; R4 replaced by root's attempt-2 ruling): #1585a is the
arm state in the slot, the arm tags (D1, D3, D4 as R3 moved it to the control plane) and the
browser deletion; #1585b is only the protocol reply fields (D5: `observe_arm_tags`,
`removed_arm_tags`), D7's docs, gate 2 and the schema-hash and corpus re-pin. No wire field and no
re-pin in #1585a.

- **R1.** `ObservationSlot` holds one eight-word seqlock record: five window words, then `armed`,
  `arm_sample`, `arm_tag`. `publish` and `publish_arm` each take the writer's own words
  (`PublishedWords::writer_words`), change their part and publish all eight. The slot grows
  56 -> 80 bytes (allocation 72 -> 96), charged through `observation_slot_retained_bytes()`.
- **D4 (root, attempt 2).** `SessionState::next_arm_tag` (starts at 1; `0` is "none") tags every
  queued record: `push_effect_observe` (kinds 7/8 and the live `Observe` edit), the forwards
  (a forward keeps its donor slot's tag unless the transaction rewrites the tap) and a structural
  transaction's own records (tagged after the commit, so a refused rebuild issues none). Each
  observation slot entry keeps the tap's last issued tag (`observation_issued_tag`), and an armed
  binding keeps its record's tag (`ObservationBinding::arm_tag`, set by the fold or the commit).
  `try_push_observe`, `ObservationForward` and `ObservationArm` carry the tag.
- **R2 (amended).** `ObservationTap` keeps `arm_sample` and `arm_tag`. A prepared arm carries its
  binding's tag and is published by the drain at the top of the plan's first block
  (`ObservationLane::publish_prepared_arms`, from the rack bank drain and the per-node `stage`),
  not by `accumulate`: `accumulate` is skipped for a tap whose effect reads no resident sample and
  for an inactive bank lane, the drain never is. A bank lane with an armed tap is never at rest,
  so the whole-bank silent skip (#1107) does not pass it by either. `disarm_all` publishes nothing;
  only tests call it.
- **Status contract (root, attempt 2; R4 withdrawn).** The browser's selected read reports
  `Pending` while the slot's published tag is below the tap's last issued tag (tags are monotone,
  so `<` and not `!=`: a carried tap whose last record was a disarm keeps its slot's tag after
  a rebuild rebuilds the slot entries from the bindings, and must not read `Pending` for ever), and
  while every attempt of the read tears; otherwise the slot's state is the answer. The issued tag
  is looked up by the tap's stable key on the newest plan, not by the rendering plan's producer
  position, which names another plan's producer between a commit and its adoption. Gate 4's three
  browser tests pass with their original assertions; `selected_observation_reads_...` adds a
  `Pending` read after the arm's block.
- **D6.** Deleted: `PlanShape::{observation_armed, observation_arm_samples,
  observation_commanded, reader_slots}`, `note_observe`, `record_observe`, `carry_tap_state` and
  its loop, the forward loop, `SessionState::observation_forwarded` and its accessor, and
  `EffectObservationHandle::armed_taps`.
- **Paths.** Beyond the scope note's #1585a list: `crates/controls/src/observation.rs` (the
  binding tag, moved from #1585b), `crates/graph/src/runtime.rs` (the per-node drain's prepared
  publish), `crates/control-plane/tests/observation_bindings.rs` (one assertion); compile-forced
  mechanical edits (the record's `arm_tag`, the push's tag argument, `0` in tests and tools) in
  `effect-contract/{src/symmetry.rs,tests/paired_spans.rs}`, `host-core/src/limiter_linked_session.rs`,
  `host-core/tests/{support/mod.rs,symmetry_witness.rs,successor_swap.rs}`,
  `rack/tests/{lane_rest.rs,lane_swap.rs,live_control_bank.rs}`, `tools/console-workload/src/lib.rs`;
  gate 3 lives in `crates/engine/tests/observation_transport.rs`.

### Evidence

- Gate 1: `render_publishes_each_taps_arm_state_on_every_leg` (host-core `effect_observation.rs`;
  console bank, dynamic bank, per node), tags 7 and 8. Test value: red if the arm state is written
  anywhere but render, if the record's drain does not publish it with its tag, or if a bank lane
  or the per-node path publishes another sample than its block's. Red on: `arm` not publishing;
  `arm` publishing `first_sample + 1`; the rack drain passing `first_sample + 1` to lanes 1..W.
- Prepared arm: assertions in `a_plan_replacement_keeps_the_subscriptions_the_host_armed`
  (binding tag 5 published at the first block, nothing before render, and a replacement whose
  first block is silence reads armed). Red on: host-core passing tag 0 for the binding's;
  `arm_prepared` publishing off render; the drain's publication removed. Not red on the attempt-1
  design (publish in `accumulate`) for this fixture: the compressor reads a resident sample on
  silence, so the silent assertion guards the drain placement against regression, not that case.
- Tags and status: `selected_observation_reads_are_bounded_stable_and_non_consuming` is red when
  `push_effect_observe` issues no tag and when the read ignores the tag.
  `an_observe_in_a_structural_transaction_arms_the_new_effect_and_keeps_the_carried_one` asserts
  the structural record's fresh tag in the binding and the slot; red when the commit issues none.
  Survives: the slot builder dropping the binding's tag (the one-block window between a rebuild's
  adoption and its first block is inside one render call, so no read sees it).
- Hand-over gap: `a_pending_disarm_leaves_the_restarted_tap_unarmed` disarms a carried tap in the
  successor's own queue (`Swap::successor_disarms`) and asserts its slot unarmed and the armed
  count without it. Red when the carry drains the successor's own queue before the taps exchange.
- Gate 3: `a_million_windows_are_read_whole_and_in_order` reads `read_state`; red when
  `read_state` takes the arm words from a second read.
- Gate 5: the existing measured tests pass; `realtime-policy` ok.
- Torn reads: a read that tears on every attempt reads `Pending`; no test (a tear cannot be forced
  deterministically on one thread).
- Tests deleted: `the_browsers_armed_bits_are_the_mask_host_core_prepared` (the mask is gone) and
  `a_tap_command_admitted_at_sample_zero_survives_the_hand_over` (it defended the deleted
  `observation_commanded`; the render order it also exercised is now the successor_swap assertion
  above, in the crate that owns it).
- capi nine-track reference budgets: the fixture's EQs declare no tap, so only
  `capi_retained_bytes` moves, 197,471 -> 197,455 (-24 for the deleted forward `Vec`, +8 for the
  tag counter); every ceiling stands.
