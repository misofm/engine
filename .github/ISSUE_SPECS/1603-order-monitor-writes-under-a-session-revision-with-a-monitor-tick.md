# Order monitor writes under a session revision with a monitor tick

> **Superseded in part (2026-10-13, root, binding):** the implementation takes option C, not this
> spec's 36/27 packing. D1 (key layout), D2 (tick), D3 (`stamp_monitor` tick rule), the ceiling
> change (deliverable 2, gate 6) and gate 3's tick wording do not apply. See "Decision record".

Decision 17, root ruling of 2026-10-13 (option B for #1559's revision gap). Architecture change in
`engine::realtime`. Code anchors verified on `93337ffbe` (main) and `79f172730` (#1559).
Issue number: assigned at filing (written here as #1603). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review. The realtime gates below are
mandatory all the same.

## Product outcome

A solo never spends a session revision. The committed revisions stay dense: N solos and then one
model edit commit that edit at `r + 1`. The plan's revision gate orders every publication by a
`(revision, monitor tick)` key. A model commit publishes `(r, 0)`. A monitor write publishes
`(r, t)` with `t >= 1` under the committed revision `r`. The applied watermark reports the
revision part only, exactly as today, so it is never above the committed revision and never
early. A caller learns that a solo is in effect from the next block, by the same rule as every
live value (D15-2). No wire, C ABI or browser status field changes.

## Context

- **The gate word.** Bit 63 is render's announcement and bits 0-62 are the revision
  (`crates/engine/src/realtime/latest_cell.rs:64`, `GATE_REVISION_MAX` at `:68`, `UNWRITTEN` at
  `:72`). Render takes one `fetch_or` snapshot per block (`RevisionGate::snapshot`, `:219`).
  Control stamps (`GatePin::stamp`, `:302`) and publishes (`GatePin::publish`, `:254`) with one
  `swap`. `stamp` refuses a revision at or below the last published one (`:303`), because the
  cell writer may then rewrite a slot that render reads (`CellWriter::target`, `:457`).
- **The plan handle.** `PlanRevisionGate` (`crates/engine/src/realtime/plan.rs:592`, `stamp` at
  `:629`, `publish` at `:635`). Every plan's gate is born at 0 (`:711`).
  `PreparedRenderPlan::stamp_held_gate` (`:752`) stamps a successor's carry retargets before
  hand-out.
- **The watermark.** Render advances it to `render.live_snapshot.revision()` after each block
  (`crates/engine/src/realtime/plan_exchange.rs:791`). `WatermarkWriter::advance`
  (`crates/engine/src/realtime/watermark.rs:186`) counts `revision - current.revision` revisions
  as completed (`:190`), so a spent number inflates `exact`.
- **The gap.** #1559's implementation record (its spec at `79f172730`, section "Implementation
  record") publishes a solo's records at `SessionStore::next_gate_revision()`
  (`crates/protocol/src/model.rs:1308` at `79f172730`) and spends that number (`gate_gap`,
  `:1279`; `note_gate_publication`, `:1479`; `PreparedSessionTransaction::gate_revision`,
  `:1213`). The next model commit skips it. The control plane threads `gate_published` through
  `ObservedPreparedToken::commit` (`crates/control-plane/src/control.rs:491-502` at `79f172730`)
  and `ProtocolController::commit_prepared_monitor` (`crates/protocol/src/controller.rs:2079` at
  `79f172730`). Its race test accepts a model revision anywhere in `r + 1 ..= r + 1 + solos`
  (`crates/capi/tests/resource_lifecycle.rs:2990-3003` at `79f172730`).
- **The live arm stamps last.** `SessionState::commit_live` takes the stamp as its last fallible
  step and publishes as its last write (`crates/control-plane/src/control.rs:1535` and `:1612` at
  `93337ffbe`). The rebuild arm stamps and publishes the candidate's gate (`:1283`, `:1315`).
  Every commit publishes on the newest plan's gate, so that gate's last published revision always
  equals the committed revision.
- **The ceiling.** The session schema bounds `revision` at `GATE_REVISION_MAX`
  (`crates/session/src/lib.rs:42`, `crates/session/src/validate.rs:41-46`,
  `docs/session-v1.schema.json:36`). The store refuses a commit at the ceiling with
  `REVISION_EXHAUSTED` (`crates/protocol/src/model.rs:1360` at `93337ffbe`).
- **The reply.** #1313 added `TransactionApplied.edit_path`; #1559 appends `Monitor = 4`
  (`crates/protocol/src/message_wire.rs:170-211` at `79f172730`). Every response frame carries a
  `revision`.
- **Readers of the stamp and the watermark.**
  - Engine: `latest_cell.rs` (gate, pin, cell writer and reader, loom models), `plan.rs`
    (`PlanRevisionGate`, `stamp_held_gate`), `plan_exchange.rs:414` and `:422` (initial revision),
    `:791` (advance), `watermark.rs` (unchanged), `mod.rs` (re-exports `:21-24`, gate tests
    `:945-1008`), `spsc.rs:1221-1231` (loom test).
  - Stampers and publishers: `crates/control-plane/src/control.rs:1143`, `:1283`, `:1315`,
    `:1535`, `:1612`; `crates/host-core/src/prepare.rs:1511` and `:2450`;
    `crates/host-core/src/limiter_linked_session.rs:512`; `hosts/host-web/src/lib.rs:5066-5228`
    (the browser's own admission until #1565). Cell writers pass a `LiveStamp` through unchanged
    (`effect-contract`, `effect-compiler`, `graph`, `builtins-compiler`,
    `host-core/src/route_controls.rs`).
  - Ceiling: `crates/session` (`lib.rs:42`, `validate.rs:41-46`, tests
    `diagnostic_parity.rs:805-823`, `json_contract_artifacts.rs:110-150`, `invalid_matrix.rs:58`),
    `crates/protocol/src/model.rs:1268` and `:1360`, `crates/capi/src/runtime/tests.rs:3936`,
    `crates/host-core/tests/successor_swap.rs:3783` and `:3804`, `latest_cell.rs` tests `:972-979`,
    `docs/session-v1.schema.json:36`, `docs/SESSION_SCHEMA_V1.md:30`,
    `docs/CONTROL_PROTOCOL_SEMANTICS.md:7`, `docs/C_ABI_V1_QUALIFICATION.md:142`,
    `sdk/src/core/session.ts:155` and `:999`, `sdk/test/builder-evals.mjs:32`, `:1091` and `:1098`,
    `sdk/test/enginectl-cli.mjs:185` and `:192`.
  - Watermark: `crates/control-plane/src/plan.rs:111`, `crates/capi/src/ffi.rs:1047`
    (`miso_engine_v1_plan_watermark`), `crates/capi/src/runtime/live_tests.rs:1322`,
    `crates/capi/tests/plan_swap_race.rs:630` and `:1061`,
    `crates/host-core/tests/live_transaction.rs:196-269`.
  - Browser: host-web has no watermark reader and no applied-revision status word at
    `93337ffbe` (its only revision words are EQ owner revisions, `hosts/host-web/src/lib.rs:3306`
    and `:3350`). Nothing there changes.

## Decisions frozen for this slice

- **D1. The key.** The gate word is `ANNOUNCED | key`. The key packs `revision` in bits 27-62
  (36 bits) and `tick` in bits 0-26 (27 bits):
  - `GATE_TICK_BITS: u32 = 27`; `GATE_TICK_MAX: u32 = (1 << 27) - 1` (134,217,727);
  - `GATE_REVISION_MAX: u64 = (1 << 36) - 1` (68,719,476,735). It stays the session's revision
    ceiling (`session::MAX_SESSION_REVISION`).
  - A public `GateKey(u64)` (`Copy`, `Eq`, `Ord`, `Hash`, `Debug`) with `revision() -> u64`,
    `tick() -> u32` and `model(revision) -> Option<GateKey>` (tick 0; `None` above the ceiling).
    Bit 63 of a key is always 0.
  - **Compare rule.** One unsigned compare of the packed `u64`. It equals the lexicographic order of
    `(revision, tick)`. `UNWRITTEN = u64::MAX` stays above every key and every snapshot.
  - **Why these widths.** The revision is persisted in the session document and grows for the
    session's whole life: 2^36 is 2.2 years of one commit per millisecond. The tick resets at every
    model commit (D2): 2^27 is 37 hours of one solo per millisecond with no other edit. JS numbers
    hold both exactly.
- **D2. The tick (ruling point 3): monotone within a revision, reset on a new revision.**
  - A model commit of revision `r` stamps `(r, 0)`. The first monitor publication after it stamps
    `(r, 1)`, the next `(r, 2)`.
  - The pin owns the tick. `stamp_monitor` reads it from the pin's last published key; no session
    state holds a tick.
  - A fresh gate (every new plan) starts at key 0. A rebuild publishes `(r + 1, 0)` on the
    candidate's gate, so the next monitor write on it is `(r + 1, 1)`. Adoption changes nothing:
    render reads keys only from the gate of the plan it runs, cells belong to one plan, and the
    watermark reads the revision part only. The adoption carry (`CellReader::peek_unread`) picks
    the greatest sequence and ignores keys, so it is unchanged.
  - *Rejected: one global monotone counter.* In the same 27 bits it would exhaust after 2^27
    monitor writes in the session's whole life, not between two model commits. It would also need
    a session-owned counter carried across plans. The watermark needs no tick, so the per-revision
    tick needs no state outside the pin.
- **D3. The pin API.** In `latest_cell.rs` and mirrored on `PlanRevisionGate`:
  - `stamp(revision)` and `publish(&gate, revision)` keep their signatures and mean key
    `(revision, 0)`. Every existing caller compiles unchanged. `stamp_held_gate(revision)` is the
    same model stamp.
  - New `stamp_monitor(revision) -> Result<LiveStamp, StampRefused>`: the key `(revision,
    last.tick + 1)`, where `last` is the pin's last published key. It refuses, in every build and
    with the pin unchanged: `revision != last.revision()` (a monitor write sits under the revision
    the gate published last, which is the committed one), and `last.tick() == GATE_TICK_MAX`.
  - New `publish_key(&gate, key)`; `LiveStamp::key() -> GateKey`. A caller reads `stamp.key()` into
    a local and publishes it, as it reads `stamp.revision()` today.
  - `StampRefused` gains `reason: StampRefusal` with `NotAbovePublished`, `AboveCeiling`,
    `NotPublishedRevision` and `TickExhausted`. `last_published` stays the revision part.
  - `GatePin::last_published()` and `RevisionGate::revision()` return the revision part.
    `LiveSnapshot::revision()` returns the revision part; new `LiveSnapshot::tick()`.
    `LiveSnapshot::ALL` is the largest key.
  - The open-stamp rule is unchanged: one key is open at a time; the held gate's open model stamp
    may be stamped again by the handle.
- **D4. The watermark (ruling point 2).** `plan_exchange.rs:791` still passes
  `live_snapshot.revision()`, now the revision part. `watermark.rs` does not change. A monitor tick
  never advances it. It reads the session revision, it is never above the committed revision, and
  `exact` counts one per model revision.
- **D5. Realtime (ruling point 6).** Render's work is unchanged: one `fetch_or` per block, the same
  slot loads and `u64` compares per pending cell, one shift for `revision()` per block. No
  allocation, lock or syscall. `stamp_monitor` and `publish_key` run on control with integer
  compares and one `swap`. D15-2 holds as written: every record of one monitor transaction rides
  one key, so it takes effect in one block; a later write before the snapshot supersedes it and
  `live_values_superseded` counts it through the per-cell sequence.
- **D6. The solo reply (ruling point 4): the next block. No wire change.**
  - A monitor-only `SESSION_TRANSACTION_APPLY` replies `TransactionApplied { edit_path: Monitor }`
    (4) with the frame's `revision` equal to the committed revision, unchanged.
  - The solo is in effect from the first block that begins after the reply returns and renders
    with the watermark at or above the reply's revision. With no replacement plan pending, that is
    the next block. With one pending, it is the block that adopts it, and the watermark reports it.
  - This holds because a monitor-only transaction is always live: it publishes on the newest
    plan's gate, whose last published revision is the committed one. A monitor-only transaction
    never takes the rebuild arm (D7).
  - *Rejected: an applied tick on the wire.* It needs a session-wide tick that stays monotone
    across plans (D2 rejected that), a new `TransactionApplied` field, a C ABI watermark word and a
    browser word, and it tells the caller nothing D15-2's rule does not already give.
- **D7. Rules for #1559's rebase** (the control plane; solo is not on `main` yet).
  - A monitor-only transaction with records stamps `revision_gate.stamp_monitor(committed)` as its
    last fallible step, writes its records with that stamp, and publishes `publish_key(key)` as its
    last write. With no record it stamps nothing (as #1559).
  - A transaction with a model edit stamps `stamp(r + 1)`, and its overlay records ride that stamp
    (as #1559 D5).
  - A monitor-only transaction never rebuilds. If the classifier returns a rebuild for one, the
    command returns `INTERNAL` with nothing written. A solo's records are strip mute and route gate
    records, and every plan carries those lanes (C6; `controls::live_lanes`,
    `crates/controls/src/live_lanes.rs:52`), so this is a defect, never a product path.
  - **Overflow.** `TickExhausted` refuses the monitor-only transaction with
    `MISO_ENGINE_V1_BACKPRESSURE` (`CommandError::Backpressure`) and nothing written; the next
    model commit clears it. The C header prose states the limit and the remedy.
  - A monitor-only transaction is not refused by `REVISION_EXHAUSTED`: it advances nothing.
- **D8. Ruling C8 (ruling point 5).** Replace the C8 paragraph of
  `docs/rulings/one-control-abstraction-2026-10-13.md` with exactly:

  > **C8. Solo.** Solo is a strip-own control with `scope: monitor`, held in the core's overlay
  > (`LiveControlSoloState` moves behind `controls`) and written by `SET_CONTROL` on both hosts. A
  > monitor write is not a session mutation (the same class as observe): it never advances or
  > spends a revision, so committed revisions stay dense. The reply carries `path: monitor` and
  > the committed revision, unchanged. Its records are published on the newest plan's revision
  > gate under that revision with a monitor tick: the gate orders publications by `(revision,
  > tick)`, and the applied watermark reports the revision only, so it is never above the
  > committed revision (amended 2026-10-13, root, option B for #1559's revision gap; #1603). A
  > monitor-only transaction is always live and applies by D15-2's convergence block: the first
  > block that begins after the reply returns, or, while a replacement plan is pending, the block
  > that adopts it, which the watermark reports when it reaches the reply's revision. A
  > transaction that mixes it with model edits is all or nothing and advances the revision by one
  > (#1559 D5 and D8, #1562 D4, formerly #1382 D3 and D4). The C ABI gains solo.

## Deliverables

1. D1-D5 in `crates/engine/src/realtime/{latest_cell.rs,plan.rs,plan_exchange.rs,mod.rs}`, with the
   unit tests and the loom model of gates 1-3.
2. The ceiling propagation: session validator message, schema regex, docs and SDK constant and
   tests, each at `68719476735`.
3. `crates/host-core/tests/monitor_tick.rs` (gates 4 and 5).
4. D8's C8 text in the ruling.
5. The amendment texts for #1559 and #1562 below, applied by those slices' own rebases (their specs
   are not on `main` in this form).

### What #1559's rebase changes

- Delete: `SessionStore::next_gate_revision`, `gate_gap`, `note_gate_publication`,
  `PreparedSessionTransaction::{monitor_gate_revision, gate_revision}`,
  `PreparedStructuralCommand::gate_revision`, and the `gate_published` argument of
  `commit_prepared_monitor`, `commit_prepared_with` and `ObservedPreparedToken::commit`.
- The live arm reads `revision` from `prospective_session().revision()` again and follows D7. The
  rebuild arm drops its `monitor_only` branch and restores
  `debug_assert_eq!(revision, committed)`.
- The race test (`resource_lifecycle.rs`, `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`)
  drops `monitors_since_commit` and requires `header.revision == r + 1` for every model commit.
- Replace the first implementation-record bullet ("A solo's records need a gate revision") with:

  > - **A solo's records ride a monitor tick** (#1603, root ruling of 2026-10-13, option B). A
  >   monitor-only transaction that produces records stamps `stamp_monitor(committed revision)` on
  >   the newest plan's gate, key `(r, t)` with `t >= 1`, and publishes that key. No revision number
  >   is spent: the store has no `next_gate_revision`, and the next model commit is `r + 1`. The
  >   applied watermark reports `r` and never a higher number. A monitor-only transaction never
  >   rebuilds; a tick exhausted after 134,217,727 monitor publications in one revision is
  >   `BACKPRESSURE` until the next model commit.

### What #1562 changes

- Its solo goes through `SessionState::command`, so it takes D7 with no browser code. The worklet's
  old path (`hosts/host-web/src/lib.rs:5066-5228`) keeps model keys (tick 0) and never calls
  `stamp_monitor`; #1565 deletes it.
- Gate 3 adds: ten solos, then a fader edit, commit the fader at `r + 1`.

## Authorized paths

- `crates/engine/src/realtime/{latest_cell.rs,plan.rs,plan_exchange.rs,mod.rs,spsc.rs}`.
- `crates/session/src/{lib.rs,validate.rs}`, `crates/session/tests/{diagnostic_parity.rs,json_contract_artifacts.rs,invalid_matrix.rs}`.
- `crates/protocol/src/model.rs` (comments and tests on the ceiling only),
  `crates/protocol/src/controller/tests.rs`.
- `crates/capi/src/runtime/tests.rs`, `crates/capi/include/miso_engine_v1.h` (ceiling prose only).
- `crates/host-core/tests/{monitor_tick.rs,successor_swap.rs}`, `crates/host-core/Cargo.toml`
  (test target only).
- `docs/session-v1.schema.json`, `docs/SESSION_SCHEMA_V1.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`, `docs/rulings/one-control-abstraction-2026-10-13.md` (C8 only).
- `sdk/src/core/session.ts`, `sdk/test/builder-evals.mjs`, `sdk/test/enginectl-cli.mjs`.

## Non-goals

- No solo, no monitor overlay and no control-plane change: #1559 owns them (D7).
- No wire, schema-hash or C struct change. No watermark field. No browser status word.
- No change to `watermark.rs`, to the cell slot rule, or to render's per-block work.
- No historical handoff or verdict record is edited (`docs/handoffs/**`).

## Objective gates

Commands: `cargo test --locked -p engine`, `RUSTFLAGS="--cfg loom" cargo test --locked -p engine
--release spsc_loom_cells`, `cargo test --locked -p session -p protocol`,
`cargo test --locked -p host-core --all-features --test monitor_tick`, `cargo test --locked -p capi`,
the SDK tests that `sdk/package.json` runs, `scripts/check-workspace-policy.sh`.

1. **Key order (engine unit tests).** On one pin: `stamp(1)`/`publish` gives key `(1, 0)`;
   three `stamp_monitor(1)` give `(1, 1)`, `(1, 2)`, `(1, 3)`; `stamp(2)` gives `(2, 0)`; the next
   `stamp_monitor(2)` gives `(2, 1)`. `stamp_monitor(1)` after `(2, 0)` and `stamp_monitor(3)` are
   `NotPublishedRevision`; `stamp(2)` after `(2, 1)` is `NotAbovePublished`; a pin at
   `(5, GATE_TICK_MAX)` refuses `TickExhausted` and still takes `stamp(6)`. Each refusal leaves
   `last_published` and the gate word unchanged. `GateKey` order equals tuple order for every pair
   in a grid of revisions `{0, 1, 2^35, GATE_REVISION_MAX}` and ticks `{0, 1, GATE_TICK_MAX}`.
2. **Cells under ticks.** One cell written at `(1, 0)`, `(1, 1)`, `(1, 2)`, `(2, 0)`, with a
   snapshot between each publication: every read applies exactly the newest value at or below its
   snapshot, `superseded` counts the writes skipped, and `snapshot.revision()` reads 1, 1, 1, 2.
3. **Loom.** New model `spsc_loom_cells_monitor_ticks_race_reads`: control publishes `(1, 0)`, two
   monitor keys and `(2, 0)` on one two-word cell while render takes snapshots and reads. Every read
   is whole, never above its snapshot, and `snapshot.revision()` never exceeds the revision of the
   newest model key whose `swap` preceded the snapshot.
4. **Watermark never above committed, never early (engine, `mod.rs` tests, two threads).** A plan
   exchange over probe plans whose executor reads one cell; render runs blocks on its own thread.
   Control runs 10,000 steps chosen by a seeded generator among a model commit (`stamp(r + 1)`), a
   monitor write (`stamp_monitor(r)`) and a successor candidate at `r + 1` published and adopted.
   Control stores `r` in an atomic before each model publication. After every block: the watermark
   revision is at most that atomic, the block that first reports `r` reads `r`'s model value, and
   `exact` equals the number of model revisions covered. The last block reads the last value
   written. Run 20 seeds.
5. **Allocation-free and the next-block signal.** `crates/host-core/tests/monitor_tick.rs` links
   `bench_support::alloc` and calls `assert_installed()` first. After warm-up, the control thread's
   counters read `allocations == 0 && frees == 0` around `stamp_monitor`, the cell writes and
   `publish_key`; the render thread's read the same around every render call. A mute written at
   `(r, 1)` with no candidate pending applies from the next block's first sample, and the watermark
   still reads `(r, first_sample of r)`. With a candidate at `(r + 1, 0)` pending and a monitor
   write at `(r + 1, 1)` on it, nothing applies before adoption; the adopting block applies both
   and the watermark reads `(r + 1, that block's first sample)`.
6. **The ceiling.** The session validator, the schema and the SDK accept `68719476735` and refuse
   `68719476736` and `u64::MAX`; the store returns `REVISION_EXHAUSTED` at `68719476735`;
   `plan_exchange_at_revision` refuses `68719476736` with `RevisionAboveCeiling`.
7. **Readers.** `miso_engine_v1_plan_watermark`'s struct and every existing watermark and race test
   pass unchanged. Host-web needs no change; its tests pass.
8. **Gates the rebases add.** #1559: 1,000 solos then one model edit commit at `r + 1`; its race
   test with solos holds dense revisions, `allocations == 0 && frees == 0` around every render
   call and a watermark never above the committed revision; a solo-only reply's `revision` equals
   the committed one; a monitor-only transaction that the classifier sends to rebuild returns
   `INTERNAL` with nothing written. #1562: its gate 3 addition above.

## Test value

- Gate 1 turns red if a monitor stamp spends a revision, if a tick survives a new revision, if a
  model stamp is accepted under published ticks, or if a refusal moves the pin.
- Gate 2 turns red if the reader compares revisions instead of keys (a tick then never applies).
- Gate 3 turns red if a monitor publication tears a read or lets render read above its snapshot.
- Gate 4 turns red if the watermark reads a tick as a revision, reports a revision before its
  values apply, or counts a monitor write as a completed revision.
- Gate 5 turns red if the stamp compare or render allocates, or if a solo applies late or early.
- Gate 6 turns red if any mirror of the ceiling drifts from the gate's.
- Gate 8 turns red if #1559 keeps the gap.

## Dependencies

- Lands first in batch 4. #1559 rebases on it and removes the gap; #1562 follows #1559.
- No other issue is needed: `main` at `93337ffbe` has the gate, the watermark and the session
  ceiling.

## Size

One day for one sonnet-high implementer. Deliverables 1, 2 and 4 are one commit, because the
gate and the ceiling must not drift. If the day runs over, split at deliverable 3: the host-core
test file lands as a second commit of the same issue in the same batch, before #1559.

## Decision record

- **Chosen design: option C (root's binding change of 2026-10-13), no session-format change.**
  The gate orders publications by an internal `u64` publication number (1 for a gate's first
  publication; 0 is the birth state; 63 bits, never exposed on any wire, session or status field).
  Every publication carries its session revision. A model commit is a new publication that carries
  the new revision (`stamp(r + 1)`); a monitor write is a new publication that carries the current
  revision (`stamp_monitor(r)`). The session revision ceiling stays `2^63 - 1`; no schema,
  validator, SDK, doc or C ABI constant changes, and deliverable 2 and gate 6 do not apply (their
  existing tests pass unchanged).
- **Why C meets root's condition.** The gate word is `ANNOUNCED | publication number` and has no
  room for a 63-bit revision, so the revision is not in the word. It lives in a three-slot record
  on the gate (`(number, revision)` per slot, plain stores only), written by `publish_key` before
  its `swap`, with the same A/B slot rule the cells use (`A` the newest publication, `B` the
  pinned one, the third slot is overwritten). Render's `snapshot` is the one `fetch_or`, then
  three number loads and one revision load of the slot with the greatest number at or below its
  snapshot. That slot is protected, so the pair is never torn and there is no retry loop and no
  lock. Render's added work is four loads per block per plan. The new loom model
  (`spsc_loom_cells_monitor_writes_race_reads`) asserts that a snapshot's revision is the revision
  of its own publication under races, and the existing candidate-claim model now asserts it
  through the record too.
- **Watermark.** `plan_exchange.rs` still advances to `live_snapshot.revision()`, now the revision
  of the last applied publication. It never exceeds the committed revision (a monitor publication
  carries the revision already committed) and `exact` counts one per model revision. `watermark.rs`
  is unchanged.
- **API (replaces D3).** `stamp(revision)` and `publish(&gate, revision)` keep their signatures
  and mean a model publication. New: `stamp_monitor(revision)` (refuses `NotPublishedRevision`
  when `revision` is not the last published revision, and `PublicationsExhausted`),
  `publish_key(&gate, key)`, `LiveStamp::key() -> GateKey`, `GateKey::{publication, revision}`,
  `LiveSnapshot::publication()`, `RevisionGate::publication`,
  `GatePin::last_published_key`, and `PlanRevisionGate::{stamp_monitor, publish_key,
  last_published_key}`. `StampRefusal` is `NotAbovePublished`, `AboveCeiling`,
  `NotPublishedRevision`, `PublicationsExhausted`. There is no tick and no tick overflow, so D7's
  `BACKPRESSURE` for an exhausted tick does not exist; only 2^63 - 1 publications on one plan's
  gate would exhaust the number, which is a typed stamp refusal.
  `LiveStamp::pinned()` returns the pinned publication number. `GatePin::publish(revision)` now
  asserts the ceiling in every build. `publish_key` debug-asserts that the key is the stamped one,
  so an abandoned stamp cannot leak its writes into a different revision.
- **Cost accepted.** Each plan's gate allocation grows from one word to the word plus a
  three-slot record (`plan_revision_gate_retained_bytes`, charged by the host preparation).
- **Gate 3 command.** `RUSTFLAGS="--cfg loom" cargo test --locked -p engine --release
  spsc_loom_cells` does not build here (loom is a dev-dependency). The run uses the qualification
  form: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
  --locked --release -p engine --lib spsc_loom`; all 14 models pass.
- **Gate 4 shape.** `the_watermark_counts_model_revisions_only_under_monitor_writes` in
  `crates/engine/src/realtime/mod.rs`. Control steps are 40% model commit, 45% monitor write, 15%
  successor (a fresh probe plan with its own gate, cell and pin, published on its own gate at
  `r + 1`, then reserved with `PlanAdoption::Next`; if the mailbox or the single retirement credit
  refuses it, the candidate is dropped and no revision is spent). Writes always go to the newest
  plan. Render checks per block: the watermark equals the snapshot revision and is at most the
  revision control announced; the applied value carries the snapshot's revision and its own
  publication number; applied values never go backwards across plans; a watermark rise names the
  block's first sample; `exact` equals `revision - first`. The run ends when the last block
  applies the last value written. 20 seeds.
- **Gate 5 shape.** `crates/host-core/tests/monitor_tick.rs`. Render and control run on the test
  thread with thread-scoped `bench_support::alloc` windows around the control calls and around
  every `render` call. The pending-candidate case uses an independent host as the candidate; its
  source reads in order from its first block, so the adopting block renders the candidate's block
  0, which the test compares to a twin of the target session at block 0.
- **Header prose.** `crates/capi/include/miso_engine_v1.h` has no ceiling prose and needs none:
  nothing it states changes. The ruling's C8 text is applied with option C's wording.
