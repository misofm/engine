# Give a plan a source-read clock that leads its render clock

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-12, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287), W3 (clocks).
Split from *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355), which
depends on it.
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan can read its sources ahead of its render clock by a fixed offset, and that offset never
moves backwards on an ordinary rebuild. Seeks anchored on the source-read clock (D15-12) apply in
the right block after any number of rebuilds. Only a host-declared discontinuity resets the offset,
and the source PCM then continues with no frame repeated or skipped. The control thread can read
both clocks.

## Context

- `RenderTime` has one field, `absolute_sample` (`crates/engine/src/realtime/plan.rs:497-500`).
  The graph executor passes it to the source section:
  `source_set.begin_block(time.absolute_sample, ...)` (`crates/graph/src/lib.rs:3256`). The
  source driver hands it to every consumer: `consumer.begin_block_at(first_sample)`
  (`crates/source/src/lib.rs:1701-1721`). The consumer's own `next_frame` decides which PCM plays;
  the block sample decides only when a held anchored seek applies (`begin_block_at`, `:1115`).
- `RenderTime { .. }` literals appear in about 50 files (tests, tools, hosts). This slice does not
  change the struct, so none of them move.
- `PreparedRenderPlan` has one clock, `next_absolute_sample` (`plan.rs:609`). `render_contiguous`
  (`:870-883`) refuses any other sample, and the C ABI host passes the sample
  (`miso_engine_v1_render_f32_planar`, `crates/capi/src/ffi.rs:807`). So the render clock belongs
  to the host and never jumps. At a swap the incoming plan adopts the outgoing clock
  (`adopt_absolute_sample`, `plan.rs:617`; `RealtimePlanOwner::enter_block`,
  `crates/engine/src/realtime/plan_exchange.rs:375-437`; `adopt_predecessor_plan`, `plan.rs:899`).
- `PreparedPlanExecutor` (`plan.rs:269`, defaulted methods at `:271-295`) has one production
  implementation, `GraphExecutor` (`crates/graph/src/lib.rs:3124`).
- The successor entry points take a `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`). `PlanStateInventory` (`:558-564`) is the
  predecessor's control-side record; every successor is prepared from it.
- `plan_exchange` (`plan_exchange.rs:194`) allocates the shared counters that publisher and owner
  hold; `plan_exchange_resource_report` (`:163-191`) counts them row by row.
- *Anchor every seek on the plan's source-read clock* (#1316) D1 renames `begin_block_at`'s
  parameter `source_read_sample` and leaves this issue to make it differ from the render clock.
- *Reset latency floors at a host-declared discontinuity* (#1323) D3 adds
  `SuccessorBase::discontinuity`. The declaration does not say whether a seek follows.

## Decisions frozen for this slice

- **D1. Two clocks.** `PreparedRenderPlan` gains `source_read_offset: u64` (0 by default), with
  `source_read_offset()` and `set_source_read_offset(&mut self, offset)`. The setter is a
  control-thread call on an unpublished plan; it forwards to a new defaulted trait method
  `PreparedPlanExecutor::set_source_read_offset(&mut self, offset: u64) {}`. `GraphExecutor` stores
  it, and `crates/graph/src/lib.rs:3256` passes `time.absolute_sample + offset` (a `checked_add`;
  overflow is `RenderError::TimeOverflow`) as the source-read sample. `RenderTime` is unchanged.
  `ΣP` names the running plan's offset.
- **D2. Inheritance.** `PlanStateInventory` records its plan's offset. Host-core sets every
  successor's offset at preparation from `base.inventory`:
  - an ordinary successor (a rebuild, a transition, a re-preparation): the predecessor's offset;
  - a discontinuity successor (`base.discontinuity`, #1323 D3): 0;
  - a warm successor: the predecessor's plus its `lead_samples` (*Prepare a warm successor whose
    carried nodes lead the predecessor by P*, #1354). That case is *Adopt a warm successor with a
    raw-frame prime at the first ready block* (#1355), on top of this rule.

  So the source-read clock is monotone except at a declaration.
  - **The declaration's offset term.** This slice adds the term #1323 D2 leaves to it: the
    declaration also rebuilds when the newest plan (the pending candidate, or else the active
    plan) has `source_read_offset() > 0`, read from its inventory row, even when no floor is
    raised. After a warm growth a later edit can make every node's natural arrival equal its
    floor while the plan still reads ahead; without the term that declaration publishes nothing
    and `ΣP` never resets. The test stays in `crates/control-plane/src/` beside #1323's floor
    term.
- **D3. Stop without a seek.** At the discontinuity successor's adoption block the source-read clock
  steps from `render + ΣP` to `render`. Each consumer keeps its `next_frame`: no frame is rewound,
  repeated or skipped. A stop that is not followed by a seek therefore resumes the stems where
  render's consumers stood, which is `ΣP` later in content than the last audible output. A
  declared stop owes no continuity. A held anchored seek keeps its anchor and compares it with the
  new clock. This step is the only backwards move of the source-read clock, and only a host
  declaration causes it.
- **D4. Readers.** `plan_exchange` allocates one more shared record, two `AtomicU64`
  (`render_clock`, `source_read_clock`), on the control thread; it is a new row in
  `plan_exchange_resource_report`. After each rendered block, render stores the active plan's next
  render sample and that sample plus its offset (`Release`). `PlanPublisher::render_clock()` and
  `PlanPublisher::source_read_clock()` each load one word (`Acquire`), so neither read can tear.
  Before the first block both read the initial plan's values. The warm-successor deadline
  (*Fall back to the transition when a warm successor is not ready by its deadline*, #1358) uses
  the render clock; hosts anchor seeks on the source-read clock, which jumps by `P` at a warm
  adoption (#1287, the lemma's seeks and clock).
- **D5. Acked-batch question.** No queue changes. An offset moves no PCM. An ack can never precede
  a drop.

## Deliverables

1. D1 and D4 in `crates/engine/src/realtime/plan.rs` and `plan_exchange.rs` (re-exports in
   `mod.rs`), and the one call in `crates/graph/src/lib.rs`.
2. D2 in `crates/host-core/src/prepare.rs`, with a `test-support` function
   `test_only_set_source_read_offset(&mut PreparedHost, u64)` that sets the plan's offset and its
   inventory row together.
3. D2's declaration term in `crates/control-plane/src/`, with a `test-support` hook that applies
   `test_only_set_source_read_offset` to the next plan the session prepares.
4. Gates in `crates/host-core/tests/source_read_clock.rs` (new), the engine's `plan_exchange.rs`
   unit tests and the control-plane crate's unit tests (gate 5).

## Authorized paths

- `crates/engine/src/realtime/plan.rs`, `plan_exchange.rs`, `mod.rs`
- `crates/graph/src/lib.rs` (`GraphExecutor`'s offset field and the source-section call only)
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/source_read_clock.rs` (new)
- `crates/control-plane/src/` (the declaration's offset term, its `test-support` hook and gate 5)

## Non-goals

- No readiness check, prime or adoption, and no warm offset (#1320, #1355). No floor logic
  (#1285, #1323).
- No change to `RenderTime`, to the render clock's contiguity rule, or to any host's render call.
- No browser or C ABI surface for the clocks; hosts reach them through the control plane later
  (#1360, #1361).

## Objective gates

1. **Anchor on the source-read clock (the old #1287 gate 4, per D15-12).** A plan prepared with
   offset `3 * 128` at 48 kHz, quantum 128. A `seek_at` anchored at source-read sample `A` applies
   in the block whose source-read sample is `A`, that is render sample `A - 384`; the output
   equals a zero-offset reference whose anchor is `A - 384`.
2. **Ordinary rebuild inherits.** From gate 1's plan, an ordinary successor (a fader-only prepared
   change) reports offset `384`. A `seek_at` held across the swap applies in the same block as in
   a run without the rebuild.
3. **Declaration resets, no frame lost.** From gate 1's plan, a successor prepared with
   `discontinuity: true` reports offset 0. Its first block plays each source's next frame after
   the predecessor's last played frame: none repeated, none skipped (a ramp-PCM source whose
   sample values equal their frame index). `source_read_clock()` equals `render_clock()` from that
   block on.
4. **Readers.** Through 8 blocks and two swaps (offset 384, then 0), after each block
   `render_clock()` equals the plan's next sample and `source_read_clock()` equals it plus the
   active plan's offset (engine unit test with a test executor).
5. **A declaration resets an offset with no raised floor.** Control-plane unit test with
   `test-support`, 48 kHz, quantum 128. A session's active plan has no floor above any natural
   arrival and offset 384 (set by D2's hook before publication). The control plane's declaration call (#1323 D2)
   publishes one plan (the swap counter grows by one), and after one render the active plan reports offset 0
   and `source_read_clock()` equals `render_clock()`. With offset 0 instead, it publishes nothing
   (#1323 gate 3).
6. Commands:
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p graph`, `cargo test --locked -p source`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a seek compared on the render clock applies 3 blocks early. Red.
- Gate 2: an ordinary successor that defaults its offset to 0 applies the held seek `ΣP` late. Red.
- Gate 3: a reset that rewinds or advances a consumer repeats or skips frames. Red.
- Gate 4: a reader that stores the offset from the outgoing plan reports the wrong source-read
  sample after a swap. Red.
- Gate 5: a declaration test that checks floors only publishes nothing for a plan that leads with
  no raised floor, so `ΣP` is never reset. Red.

## Dependencies

- *Anchor every seek on the plan's source-read clock* (#1316).
- *Reset latency floors at a host-declared discontinuity* (#1323): `SuccessorBase::discontinuity`.

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.** Each fact in Context still holds; only the lines moved.

- `RenderTime` is `crates/engine/src/realtime/plan.rs:576-579`. The plan's clock field is `:755`,
  its getter `:910`, `adopt_absolute_sample` `:918`, `render_contiguous` `:1172-1185`,
  `adopt_predecessor_plan` `:1201-1207`.
- `PreparedPlanExecutor` is `plan.rs:273`; its defaulted methods now run to about `:360` and
  include #1311's `prime_ready` (`:349`).
- The graph's source-section call is `crates/graph/src/lib.rs:4843-4844`, in the block entry that
  #1639 changed (the jump-length read is `:4823-4826`, just before it). `GraphExecutor` is
  `:3192`, its `PreparedPlanExecutor` impl `:4588`.
- The source driver's block entry is `crates/source/src/lib.rs:1752-1785`; the
  `begin_block_at(first_sample)` call is `:1772`. `begin_block_at` is `:1115`.
- `miso_engine_v1_render_f32_planar` is `crates/capi/src/ffi.rs:818`.
- `RealtimePlanOwner::enter_block` is `plan_exchange.rs:742-795`; the clock adoption is `:779`.
  `plan_exchange` is `:367`, `plan_exchange_at_revision` `:410`, and both call
  `plan_exchange_inner` (`:436-470`), which is where D4's record is allocated.
  `plan_exchange_resource_report` is `:337-361`.
- `SuccessorBase` is `crates/host-core/src/prepare.rs:1070-1097`. It now has `inventory`,
  `committed`, `revision` (#1277), `edit_ramps` (#1394), `monitor` (#1559) and `observation`
  (#1584). It has no `new` constructor yet; #1323 D3 adds it. `PlanStateInventory` is `:711-757`.
- The control plane's successor construction site is `crates/control-plane/src/control.rs:1670`.

**Code notes for the implementer.**

- `GraphExecutorWithoutSplitPairTable` (`crates/graph/src/lib.rs:4438`) mirrors
  `GraphExecutor`'s layout for the largest-allocation cap. D1's new offset field goes in both.
  This is inside D1's "offset field" scope.
- D4's new record and D1's field can move pinned measurements outside the authorized paths:
  `crates/capi/tests/resource_lifecycle.rs` (the capi retained measurement) and
  `hosts/host-web/tests/browser-v1/expected.json` (browser boot memory). The control plane
  charges the exchange through `crates/control-plane/src/compile.rs:237`. If a pin moves, re-pin
  it from the measurement with the reason, as batch 13a and 13b did. Add those two files to the
  authorized paths for that purpose only.
- #1637 shipped node time as `s = r + ΣP - a(n)`. No code reads `ΣP` yet, so it is 0 today.
  *Build the timeline consumer and producer* (#1640) and *Give every plan a timeline that
  carries like a source* (#1641) will pass this slice's source-read sample to the timeline.
  Nothing in this slice changes for that.
- Host-core builds a source set for every plan (`crates/host-core/src/prepare.rs:2579`), with no
  entries for an empty session (#1682). D1 then offsets a clock that no source reads, and D4
  still publishes both clocks.

**Dependencies on GitHub.**

- #1316 and #1323 are open. Both are in the batch-15/16 list.
- #1323 depends on *Supersede an unadopted candidate plan by compare-and-swap* (#1310, open),
  and #1310 depends on #1344 (open). Neither is in the batch-15/16 list. So D2's declaration
  term and gate 5 cannot run until #1310 and #1323 land.
- Batch order (root, 2026-10-15): batch 15a is the controls lane, #1344, #1398 and #1310.
  #1324 left batch 15a (root ruling, 2026-10-15); it lands in a later batch after #1288, #1391,
  #1363 and #1325.
  The rest of batch 15 follows. #1323 lands after #1310 (R5 in #1323).
- #1314 and #1309 are closed. The control plane is `crates/control-plane/src/`, as the spec says.
- #1354, #1355, #1320, #1358, #1360 and #1361 are open. They are follow-ons or non-goals, not
  dependencies.

**Decisions.**

- D1-D5 still hold against the code and decision 17. The clocks get no host surface here, and a
  later host surface goes through the control plane (decision 17, C1).
- #1323 D1 adds a C export (`miso_engine_v1_declare_discontinuity`). Decision 17 C13 routes
  new features through protocol messages. That is #1323's question, not this slice's: gate 5
  calls the control plane, not the export.

**Gates and commands.** `crates/host-core/tests/source_read_clock.rs` is new. Every named script
and package exists (`realtime-policy` is `tools/realtime-policy`). The features
`engine/realtime-audit`, `host-core/test-support` and `control-plane/test-support` exist.

**Realtime-path edits.** D1 and D4 edit `crates/engine/src/realtime/plan.rs`,
`plan_exchange.rs` and `mod.rs`. D1 also edits the graph's render block entry. That slice gets
one attempt.
