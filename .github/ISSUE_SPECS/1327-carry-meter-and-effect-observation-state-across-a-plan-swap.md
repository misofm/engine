# Carry meter and effect observation state across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-14, D15-7).
Code anchors verified on `main` at `6fb211594`.

This slice carries builtin meters and effect observation taps. The spectrum observer is *Carry
spectrum capture state across a plan swap* (#1395), which reuses D1-D4's rules for its own key
and its shared mode word.

## Product outcome

An app that reads a strip's level or an effect's gain reduction sees no reset when a track is
added or another strip is edited. A meter or observation tap is unchanged when its owner's key and
configuration are the same in both plans. Such an owner keeps its open
window, its counters and its sequence through a plan swap. The host's existing reader keeps
receiving, with no generation bump, no counted loss, and no window relabelled. This replaces
#1269's P10 ("restart at the swap"), which decision 15 rejects as a shortcut (D15-14). Only
changed or added owners start fresh.

## Context

- **Builtin meters.**
  - A meter is a graph observer, `MeterObserver(MeterAccumulator)`
    (`crates/builtins-compiler/src/lib.rs:4820`, impl `:4821`).
  - `MeterAccumulator` (`crates/builtins/src/lib.rs:4553`) holds the window state (`start`,
    `frames`, `sequence`, two `MeterLane`s and the cumulative counters) and the
    `Producer<MeterSnapshot>`.
  - The host reads through `MeterConsumer` (`builtins-compiler/src/lib.rs:289`), returned in
    `HostLiveControlHandles::meters` (`crates/host-core/src/prepare.rs:445`) and requested by
    `HostMeterRequest` (`:399`).
  - A snapshot carries `reset_generation` and `window_sequence` (`builtins/src/lib.rs:4505-4518`).
- **Effect observation taps.**
  - `ObservationLane` (`crates/effect-contract/src/live.rs:588`) is held per node by
    `LiveControlEffect::observation` (`crates/graph/src/runtime.rs:1196`) and per bank lane by
    `LiveControlEffectBankStage::observations` (`crates/rack/src/lib.rs:961`).
  - Readers reach the host as `HostLiveControlHandles::effect_observations` (`prepare.rs:452`).
- **Spectrum** is #1395. `SpectrumCaptureObserver` (`crates/host-core/src/spectrum.rs:1441`) is a
  graph observer too, so #1395 registers it in this slice's observer location table (D2).
- **The browser** bumps `meter_generation` and counts a loss when a snapshot's `reset_generation`
  changes (`hosts/host-web/src/lib.rs:3545-3561`), which is what a swap to fresh meters causes.
- **The C ABI** has no strip meter, observation tap or spectrum capture. Its telemetry is the
  output peak each render publishes (`crates/control-plane/src/plan.rs:261`, read by
  `collect_render_activity`, `crates/control-plane/src/control.rs:625`). That is continuous across a
  swap already, so this slice has nothing to carry there.
- The carry scaffold, the effect carries and the source-producer precedent
  (`SourceControlSet::adopt_persisting`, `crates/host-core/src/source.rs:286`) are on `main` or come
  from #1279-#1284.

## Decisions frozen for this slice

- **D1. Keys and rule (P1).** There are two kinds of owner here:
  - A **meter** carries when its strip ID, tap, metric set and `MeterConfig` are equal in both
    plans.
  - An **observation tap** carries when its effect owner carried (#1279-#1282) and its tap
    descriptor is the same.

  Nothing in them is a live value, so there is no retarget. A non-carried owner starts fresh and
  is not a strip restart: it changes no audio.
- **D2. Move mode.** At the swap block, swap each carried observer's whole state between the plans,
  its producer included. A reader that already holds the carried producer's consumer therefore
  stays connected to the observer that renders on. The successor's fresh observer goes to the
  retiring predecessor.
  - The observer location table records each observer's owner kind and key, not its handle, so
    #1395 adds the spectrum kind without changing the section.
  - Within one kind, the table's entries are in the order of that kind's reader vector in
    `HostLiveControlHandles` (`meters`, `effect_observations`), so an entry's index is its
    reader's index.
- **D3. Carry record and reader pairing, at the swap block.** No reader is swapped on the control
  side: a commit runs before adoption, and a candidate it names can be superseded (#1310) and
  never adopted.
  - The successor's table holds a preallocated carry record, one slot per entry. D2's section
    writes it in the swap block: for each carried entry, `ObserverCarry { kind, successor,
    predecessor }` (the two entry indices). The record is allocation-free and written once, since
    a plan is adopted at most once.
  - The engine gets the record type and one accessor: `ObserverCarry` and `ObserverKind`
    (`Meter`, `Observation`; #1395 adds `Spectrum`) in `crates/engine/src/realtime/plan.rs`, a
    `PreparedPlanExecutor::observer_carries(&self) -> &[ObserverCarry]` method whose default is
    empty, and `PreparedRenderPlan::observer_carries()`. The graph executor (`impl PreparedPlanExecutor for
    GraphExecutor`, `crates/graph/src/lib.rs:3124`) returns its table's record.
  - host-core gives two pairing functions, `pair_carried_meters(successor: &mut [MeterConsumer],
    predecessor: &mut [MeterConsumer], carries: &[ObserverCarry])` and
    `pair_carried_observations` (same shape, over the observation handles). For each record of
    its kind, it swaps `successor[s]` with `predecessor[p]`. It is allocation-free, drops nothing,
    and may run on the render thread.
  - A host whose readers live inside the plan pairs them in the swap block, from the plan's host
    attachment hook. The browser is that host: *Swap and retire browser plans through the
    Worker's service loop* (#1381) D3 calls `pair_carried_meters` (its meter vector is in host-core
    order) and pairs observation handles through its own host-core-index-to-slot map, because the
    browser keeps observations in gapped per-slot storage. After the swap block the successor holds
    the carried readers, and the retiring predecessor holds the fresh ones.
- **D4. A warm successor carries by the same move.** A warm successor (*Grow latency during
  playback by adopting a primed warm successor*, #1287) renders no block before its adoption at
  `S` (D15-8 (round-5 amendment)). D2's section runs in that adoption block, as at any swap, so its
  observers carry with no overlapping window and nothing published twice. This slice adds no other
  mode.
- **D5. Both hosts.** Everything is in engine (the record type and accessor only), host-core, graph, rack, builtins and effect-contract.
  - The browser's readers live in its plan's host attachment. #1381 D3 calls D3's pairing functions from the attachment hook in the swap block, so the browser has no other carry code. *Replace the running browser session in the Rust host* (#1290) gates the result: the browser's `meter_generation` does not advance across a structural apply that carries every meter.
  - **Decision 17 amendment (2026-10-13).** The C ABI now has state to carry: since #1581 every C ABI plan holds one strip meter per strip, and since #1584 its effects' observation taps. Its readers live on the control thread, so #1587 calls D3's two pairing functions there, when the control plane promotes the adopted plan. This issue adds no C ABI code; its gates stay as written.
- **D6. A fresh meter joins the carried window grid.** A host that folds meters as one group
  needs every meter's windows to match: the browser requires equal `window_sequence`,
  `start_sample`, `end_sample` and `frames` across the group (`hosts/host-web/src/lib.rs:3485-3500`).
  A fresh `MeterAccumulator` opens its first window at its first observation (`start: None`,
  `crates/builtins/src/lib.rs:4683`), so an added strip's meter would sit off the grid forever.
  So, in the swap block, a fresh meter (no carry record) whose `MeterConfig` equals a carried
  meter's takes the open window position of the first such carried meter (lowest successor entry
  index): its `start`, `frames` and `sequence`, with its lanes and cumulative counters at zero.
  That is exactly what it would hold had it observed silence since the window opened, and the
  strip did not exist before adoption. With no carried meter of that config, it starts as today.
  It is a scalar copy in the move section: no allocation.

## Deliverables

1. D2 in `crates/builtins` (`MeterAccumulator` swap), `crates/builtins-compiler` (`MeterObserver`),
   `crates/effect-contract/src/live.rs` (`ObservationLane`), `crates/rack` and `crates/graph` (the
   observer location table and the program section).
2. D1 and D3's pairing functions in `crates/host-core/src/prepare.rs`. D3's record type, the
   executor method and the plan accessor in `crates/engine/src/realtime/plan.rs`.
3. Tests in `crates/host-core/tests/successor_swap.rs`, through the browser's preparation entry
   points (`prepare_host_runtime_with_selected_meters_between_render_calls`,
   `crates/host-core/src/prepare.rs:1051`, and, for the observation taps,
   `prepare_host_runtime_with_live_controls_successor`, `:926`).

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, `crates/rack/src/lib.rs`
- `crates/effect-contract/src/live.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- Outside stream A's ownership; root sequences it: `crates/engine/src/realtime/plan.rs`
  (`ObserverCarry`, `ObserverKind`, the executor method and the plan accessor only).

## Non-goals

- No browser code: the attachment hook is #1381, the browser gate #1290. No change to meter or
  spectrum kernels.
- No spectrum carry (#1395).

## Objective gates

1. **Meters carry, both widths.** Prepare A with meters at every tap of three strips, with a window
   several blocks long, so a window is open at the swap. B adds a muted track whose ID sorts first.
   Read through A's consumers before and after the swap. The snapshot sequence has no gap, keeps
   `reset_generation`, has `cumulative_discontinuities` 0, and every snapshot equals the reference
   (a fresh B with the same meters, fed from frame 0) field by field. The added muted track's
   meter shares the carried meters' window boundaries and sequence numbers from its first
   snapshot, and each of its snapshots equals the reference's.
2. **A changed meter starts fresh.** B changes one meter's window length. That meter's successor
   reader starts a new sequence, and every other meter carries as in gate 1.
3. **Pairing follows the record.** In gate 1's script, after the swap block, call
   `pair_carried_meters` and `pair_carried_observations` on the successor's and predecessor's
   handle vectors with the successor's `observer_carries()`. Every carried strip's snapshots then
   continue through the successor's vector at that strip's new index, as in gate 1. The added
   track's meters have no record, and their successor readers receive the fresh observers'
   snapshots.
4. **Observation taps.** With a live-controlled compressor insert and its gain-reduction tap armed,
   the tap's readings continue across the swap, equal to the reference. A compressor that restarts
   (its threshold changed, prepared on a lane without live controls) starts its tap fresh.
5. **Realtime.** The swap block and the two pairing calls make zero allocations and frees,
   measured with `bench_support::alloc`'s thread-scoped counters after warm-up.
6. Commands:
   - `cargo test --locked -p engine -p builtins -p builtins-compiler -p effect-contract -p rack -p graph -p host-core --features builtins-compiler/test-support,rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red on any of five defects:
  - a carry that copies the window but keeps the successor's producer leaves the host's reader on a
    dead queue;
  - a meter keyed by handle index instead of `(strip, tap)` reads the wrong strip after the lane
    shift;
  - a window restarted at the swap shows a counted discontinuity;
  - a lost open window shows a sequence gap;
  - a fresh meter that opens its window at adoption (D6 missing) is off the group's grid, so its
    sequence and boundaries differ from the reference's.
- Gate 2: a rule that ignores the configuration carries a window of the wrong length. It turns red.
- Gate 3: a pairing that swaps by position instead of by the record's two indices puts a reader
  on the wrong strip's producer once the added track shifts every index. It turns red.
- Gate 4: observation state that does not follow its effect owner restarts the tap on an unchanged
  effect. It turns red.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284), and through it every
  effect carry that observation taps follow.

## Amendment (decision 17, root 2026-10-13)

Dependencies gain *Prepare every browser preparation branch concurrently, as a successor too, in
host-core* (#1401), for D2's meter successor entry.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** `MeterObserver` is `crates/builtins-compiler/src/lib.rs:5943`,
  `MeterConsumer` `:942`; `MeterAccumulator` is `crates/builtins/src/lib.rs:5130`;
  `HostMeterRequest` is `crates/host-core/src/prepare.rs:436`, `HostLiveControlHandles::meters`
  `:482` and `effect_observations` `:489`; `ObservationLane` is
  `crates/effect-contract/src/live.rs:1321`; a bank's `observations` is
  `crates/rack/src/lib.rs:1184`; `impl PreparedPlanExecutor for GraphExecutor` is
  `crates/graph/src/lib.rs:3579`; `PreparedRenderPlan::carry_from` is
  `crates/engine/src/realtime/plan.rs:1158`. The entries are
  `prepare_host_runtime_with_selected_meters_between_render_calls` (`prepare.rs:1487`), its
  successor form (`:1544`, #1401) and `prepare_host_runtime_with_live_controls_successor` (`:1295`).
- **#1609's observer hook.** `GraphRuntimeObserver::block_end` exists now, and
  `GraphNodeObserverBinding` reads `accepts_block_end` once at bind (`graph/src/lib.rs:2881-2890`).
  D2 swaps the boxed observer between two bindings of the same kind and never swaps a binding, so
  the cached flag stays right.
- **D5's C ABI note is ahead of the code.** #1581, #1584 and #1587 are open: a C ABI plan has no
  strip meter and no observation tap yet. The gates are host-core gates and do not change.
- **#1584 D6 (open).** A successor is prepared with the committed taps armed, and a tap whose lane
  carries takes the predecessor's state (D2). That is exact only when #1280b has applied the
  predecessor's pending Observe records at the carry. So #1327b depends on #1280b.
- **The browser's readers today.** Since #1381a the meter consumers and observation handles are in
  `RenderCompanions`, the plan's attachment. The spectrum capture is not: it is in host-core's
  `SpectrumService`, on the control side (see #1395's batch 10 amendment).
- **Split (as plan v2).** #1327a: meters (D1's meter key, D2, D3 for meters, D6; gates 1, 2, 3 and
  5). #1327b: observation taps (D1's tap key, D3 for observations; gate 4). #1327a needs no carry
  chain slice (the observer table needs only the carry program, on `main`); #1327b needs #1280b,
  #1281 and #1282, whose owners the taps follow. The filed dependency on #1284 is file order only.
- **Engine edit (one-attempt rule).** D3's `ObserverCarry`, `ObserverKind`,
  `PreparedPlanExecutor::observer_carries` and `PreparedRenderPlan::observer_carries` are an edit of
  `crates/engine/src/realtime/plan.rs`. Recommendation: make it the chain's one `plan.rs` edit, in
  #1327a: declare `ObserverKind::Spectrum` here too (so #1395 has no engine edit), and add #1381
  D3's hook here, `HostAttachment::adopt_predecessor(&mut self, predecessor: &mut dyn
  HostAttachment, carries: &[ObserverCarry])` with an empty default, called by
  `PreparedRenderPlan::carry_from` after the executor hand-over when both plans have an attachment
  (so #1381b has no engine edit). Gate 3 then pairs the meters from a host-core test attachment's
  hook in the swap block (the pattern of `crates/host-core/tests/host_attachment_render.rs`), which
  is how the browser calls it. If root rules against the fold, #1395 and #1381b each carry their own
  `plan.rs` edit under its own one-attempt rule.
- **If the hook folds in:** the call is a new indirect call in the browser's render closure, so
  the pinned indirect-site table of `scripts/check-web-audioworklet-callgraph.py` (#1333 D4) gains
  it in the same slice, with #1381 D3's reason; that script joins this slice's authorized paths for
  that row only.

## Ruling (root, 2026-10-14): one plan.rs edit

#1327a makes the one combined edit to `crates/engine/src/realtime/plan.rs` the chain needs: the
observer carries of #1327 (`ObserverCarry`, `ObserverKind`, `observer_carries`), the
`ObserverKind::Spectrum` variant of #1395 and `HostAttachment::adopt_predecessor` with its call in
`PreparedRenderPlan::carry_from` for #1381b, with the callgraph pin in
`scripts/check-web-audioworklet-callgraph.py`. One attempt; if the permission system refuses it,
the slice stops and root is told at once. #1395 and #1381b make no edit of their own in
`crates/engine/src/realtime`.

## Ruling (root, 2026-10-14): the acknowledged subscribe

Since #1280b, a pending Observe record of a carried lane is applied at the carry to the
predecessor's `ObservationLane`, but the tap it arms is not carried to the successor until this
issue: an acknowledged subscribe can be dropped at a swap (an ack that precedes a drop). #1327b's
gates include that case: a subscribe admitted and acknowledged before a swap is armed on the
successor's tap after the swap, and the successor publishes the window the predecessor would have
published, with zero allocations and frees on the swap block.

## Decision record: #1327a meters (batch 8, worktree b8k)

**Shipped.**

- **Engine, one `plan.rs` edit (root ruling).**
  - `ObserverKind { Meter, Observation, Spectrum }` and `ObserverCarry { kind, successor, predecessor }`
    (`usize` entry indices).
  - `PreparedPlanExecutor::observer_carries()` with an empty default, and
    `PreparedRenderPlan::observer_carries()`.
  - `HostAttachment::adopt_predecessor(&mut self, predecessor: &mut dyn HostAttachment, carries:
    &[ObserverCarry])` with an empty default. `PreparedRenderPlan::carry_from` calls it after the
    executor hand-over when both plans have an attachment, with the successor executor's record.
    Both the exchange's swap block and `adopt_predecessor_plan` reach it.
  - The two new types are re-exported from `crates/engine/src/realtime/mod.rs` (one line). They
    cannot be named outside `engine` without it.
- **`scripts/check-web-audioworklet-callgraph.py`.** `enter_block` pins 4 indirect sites, from 2:
  the attachment hook and the `observer_carries()` read are new, with #1381 D3's reason.
- **builtins.** `MeterWindowPosition`, `MeterAccumulator::{config, window_position, join_window,
  rebind_handle}`.
- **builtins-compiler.** `MeterObserver` answers the four new `GraphRuntimeObserver` methods.
- **graph.**
  - `GraphRuntimeObserver::{carry_kind, adopt_binding_handle, window_position, join_window}` and
    `ObserverWindow`.
  - `GraphObserverLocation`, `GraphObserverRow`, `GraphObserverMove`, `GraphObserverJoin`,
    `observer_rows`, `install_observer_carry`, `observer_carry_retained_bytes`.
  - The executor's observer section is one boxed program (moves, joins, preallocated record), so
    `GraphExecutor` grows by one pointer.
  - The swap block checks every move first (all or nothing), then exchanges the boxed observers
    between the two bindings, then runs the D6 joins.
- **host-core.**
  - `MeterInventoryRow` and `MeterConfigKey` in `PlanStateInventory`, `resolve_meter_carry`,
    `pair_carried_meters`, and the carry program now exists when a meter may carry.
  - Test seam `test_only_prepare_host_runtime_with_selected_meters_on`.
  - The meter request and its configuration are built by `requested_meter_owners` and
    `requested_meter_config`, shared with the carry decision.

**Departures from the text.**

- **Only the reader's consumer is swapped.** `pair_carried_meters` exchanges the `consumer` field,
  not the whole `MeterConsumer`. Handles are `index + 1`, so an added strip shifts every handle.
  The swapped-in observer takes its binding's handle (`adopt_binding_handle`), so a snapshot
  published after the swap names the reader that receives it. A snapshot published before the
  swap keeps the predecessor's handle.
- **`pair_carried_observations` is not here.** It is #1327b.
- **Gate 2 is split.** A meter's window length is the preparation's one `meter_period_frames`, so no
  caller can change one meter's configuration alone.
  - Test A changes the length for every meter: nothing carries, the record is empty and each
    meter's first window opens at the adoption block (sequence 0).
  - Test B changes one meter's metric set: it alone is missing from the record. By D6 it joins the
    carried grid, so its first snapshot after the swap is window 1, not a new sequence.
- **Gate 1's added muted track.** Its meters share the carried boundaries and sequence numbers
  at every tap. They equal the reference field by field at the post-fader and post-matrix taps
  only. A pre-fader tap sees the source in the reference from sample 0, while D6 holds the added
  strip as silent before adoption, so that first window's energy cannot equal the reference's.
  This is what D6 says (the strip did not exist before adoption). The gate text over-claimed.

**Gates -> tests** (`crates/host-core/tests/successor_swap.rs`, module `meter_carry`):

| Gate | Tests |
|---|---|
| 1 | `meters_carry_through_the_predecessors_readers_at_{eight,four}_lanes` |
| 2 | `a_changed_meter_starts_fresh_and_the_others_carry` |
| 3 | `paired_readers_follow_the_record_at_{eight,four}_lanes` |
| 5 | `the_meter_swap_block_allocates_and_frees_nothing` |

- Gate 1 reads A's own consumers.
- Gate 3 pairs in a test attachment's hook, so the swap block includes the pairing.
- With the D6 join removed, gates 1 and 3 turn red at the added strip's first snapshot (checked).

**Measured.**

- Web AudioWorklet module: 3,924,130 B shipped, 4,491,023 B named twin.
- Boot high-water growth: 20,447,232 B, unchanged.
- Callgraph: the one pin row above.

**Known interim state.** `host-web` test
`structural_apply::meter_delivery_restarts_on_the_successor_and_never_publishes_a_misshaped_frame`
is red. The browser's successor meter consumers now sit on fresh observers that the swap hands to
the retiring plan until #1381b's attachment hook pairs them, so no window folds after a swap.
#1381b and #1290d replace that test's "restart" expectation with "no restart".

**Open item for root.** `HostAttachment::adopt_predecessor`'s doc in `plan.rs` links
`[PreparedRenderPlan::carry_from]`, which is `pub(crate)`. `RUSTDOCFLAGS='-D warnings' cargo doc -p
engine` fails on that link (`rustdoc::private_intra_doc_links`). It needs a one-line doc fix to
plain backticks in `crates/engine/src/realtime/plan.rs`. That is a second edit under the
one-attempt rule, so it was not made.

### Known red test until #1381b (batch 11)

`hosts/host-web` `tests::structural_apply::meter_delivery_restarts_on_the_successor_and_never_publishes_a_misshaped_frame` fails after this slice: the browser's successor meter readers sit on observers the swap hands to the retiring plan until #1381b's attachment hook pairs them (the browser rebuild path is test-support only until #1562b). #1381b, in this batch, turns the test from "restart" to "carry"; the batch lands only with it green (root, 2026-10-15). Root accepted the `crates/engine/src/realtime/mod.rs` re-export and a doc-only fix of `plan.rs` (a link to the crate-private `PreparedRenderPlan::carry_from` is plain code).

## Decision record: #1327b observation taps (batch 8, worktree b8k)

**Shipped.** No edit under `crates/engine/src/realtime`: #1327a's `ObserverKind::Observation`, `ObserverCarry`,
`observer_carries` and `HostAttachment::adopt_predecessor` are enough.

- **effect-contract.** `ObservationLane::{same_taps, swap_state}`. `swap_state` exchanges the two lanes' tap
  tables (arm state, open window, sequence, peaks, publisher) and armed count by pointer, and refuses unless both
  lanes declare the same number of taps with the same fold at each index. Each lane keeps its own
  `default_window_blocks`: a carried tap keeps the window length it was armed with.
- **rack.** `LiveControlEffectBankStage::observation_mut(lane)`.
- **graph.**
  - `GraphTapMove` (the sites of one effect move, plus the two reader entries) and `install_observation_carry`.
    A tap move binds to its effect move in the sorted effect table, so it is installed after
    `install_effect_carry`, and refused if it names no effect move, if its successor holds no observation lane, or
    if a site or entry repeats.
  - `ObserverCarryProgram` gains the tap table. Its record holds one slot per meter move and per tap move, and
    `observer_carry_retained_bytes` takes a third argument, the tap count.
  - The effect section marks each tap move `taken` where its effect move actually carried (lane to lane, lane to
    per-node, per-node to lane, per-node to per-node; a refused restore marks nothing).
  - `Runtime::carry_taps` runs after the section's own pending-`Observe` loop (#1280 D3), so the predecessor's tap
    already holds every acknowledged subscribe as its next block would have applied it. It then swaps the two
    lanes' tap state and appends one `ObserverCarry { kind: Observation, successor, predecessor }` per swap after the
    meter records.
- **host-core.**
  - `ObservationInventoryRow` (reader entry, effect site, tap count) in `PlanStateInventory`, in the order of
    `HostLiveControlHandles::effect_observations`.
  - `resolve_observation_carry`: a reader set carries when its effect instance's move is in the effect carry and both
    plans' sets have as many taps. A restarting effect, or one a rebuild removes, carries nothing.
  - `pair_carried_observations(successor, predecessor, carries)`: swaps the `readers` of the two
    `EffectObservationHandle`s the record names. Allocation-free.

**The acknowledged subscribe.** A subscribe admitted before the swap sits in the predecessor's observe queue. The
carry pops it into the predecessor's tap at `predecessor_next_sample`, the sample the successor's first block
starts at, and the tap then moves to the successor with its window and publisher. The successor arms nothing
itself and publishes the window the predecessor would have. Today's red test `pending_observe_swap` asserted the
tap armed on A; it now asserts it armed on the successor, with the successor's reader (paired) equal to the
reference's window field by field, per-node and bank forms in both directions.

**Departures from the text.**

- **The tap is keyed by its effect owner, not by its own table.** D1 says the same, but the carried state moves in
  the effect section's loop (a tap is a field of the effect instance, not a graph observer), so it has its own
  small table beside the meters' instead of rows in `observer_rows`.
- **A restarted or removed owner's subscribe is dropped, as #143 D7 says.** The fresh tap starts unarmed. Arming the
  successor's taps at preparation is #1584 D6 (open). The removed strip's tap stays armed on the retiring
  predecessor.
- **Gate 4's "restart" case uses a prepared change (link mode).** With live controls attached, a threshold change
  on an insert is a live difference and carries (#1280c), so it cannot start the tap fresh.

**Gates -> tests.**

| Gate | Tests |
|---|---|
| 4 and the acknowledged subscribe, per-node, bank, both form changes, eight lanes | `successor_swap`: `observation_carry::taps_follow_their_effect_through_every_form_at_eight_lanes`, `pending_observe_records_are_applied_at_the_carry_at_eight_lanes`, `full_observe_queues_are_applied_at_the_carry_at_eight_lanes`, `pending_observe_records_cross_the_form_change_at_eight_lanes` |
| 4, the same at four lanes | `..._at_four_lanes` twins: ignored where the build does not execute four lanes (#1017), as every console-effect test |
| 4, restart starts fresh | `observation_carry::a_restarted_effect_starts_its_tap_fresh_at_{eight,four}_lanes` |
| 5, swap block and the hook's pairing | `observation_carry::the_tap_swap_block_allocates_and_frees_nothing` (and the allocator/audit assertion inside the form tests) |
| lane exchange | `effect-contract/tests/observation_lane`: `a_lane_swap_carries_the_open_window_and_the_publisher`, `a_lane_swap_refuses_another_menu` |

**Known state.** `host-web` `structural_apply::meter_delivery_restarts_on_the_successor_...` is still red, as the
#1327a record says (#1381b). No other `host-web` test changed: the browser's rebuild path is test-support only and
no host-web test swaps observation taps. #1381b pairs the browser's observation handles with
`pair_carried_observations` from the attachment hook, through its host-core-index-to-slot map.
