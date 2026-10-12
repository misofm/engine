# Hold effect parameter, bypass and EQ-target values in latest-target cells

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A live effect parameter value, an effect's bypass and a parametric EQ's prepared targets are
held in latest-target cells on both hosts, through the shared effect lane. A host can send any
number of such edits between two render calls, a paused host included, and none is refused for
room or turned into a rebuild because it outgrew a queue. Observation subscriptions stay a FIFO.
Superseded values are counted by #1312's `live_values_superseded`.

## Context

- **Records.** `EffectControlRecord::{Parameter, PreparedTarget, Observe, Bypass}`
  (`crates/effect-contract/src/live.rs:54-101`) ride one bounded SPSC per effect instance (or per
  bank lane) into `EffectControlLane` (`:125`).
- **Staging.** `EffectControlLane::stage` (`:341`) drains the queue at block entry, collapses
  parameter records last-wins per `(parameter_index, channel)` into the caller's window of
  `automation_capacity` spans in canonical order, and folds the channel-symmetry witness with the
  pairing rule of #1004 (`:316-340`). Prepared targets go through lane-owned FIFO target staging
  sized from the queue (`new_with_target_staging`, `:191-225`). Bypass drives the lane's
  `BypassShunt` (`:868`).
- **Producers.** `EffectControlProducer` (`crates/effect-compiler/src/prepare.rs:545`) offers
  `preflight`/`try_push` and, for the EQ, the owner transaction `begin_owner`,
  `preflight_candidate_targets`, `publish_candidate_targets`, `commit_owner` (`:785-850`).
- **C ABI admission.** `commit_live` rebuilds an instance whose records outnumber its queue
  (`crates/control-plane/src/control.rs:1266-1273`) and refuses with `LiveBackpressure` when the
  room or the owner's target preflight fails (`:1315-1320`, `:1339-1348`). The queue depth is
  `min(16, automation capacity)` (`crates/control-plane/src/compile.rs:17-26`). The header states
  it (`crates/capi/include/miso_engine_v1.h:50-55`).
- **Browser admission.** The effect band's room pass and `in_flight` accounting, and the
  prepared-owner markers (`hosts/host-web/src/lib.rs:1762-1792`, `:5568-5600`, `:5705`).

## Decisions frozen for this slice

- **D1. Cells per instance**, on #1312's primitive:
  - one cell per `(parameter_index, channel)` of every parameter whose automation rate is
    `Block`: a `Shared` parameter has one `Both` cell, a `PerLane` one a `Left` and a `Right`
    cell; one word (value bits);
  - one bypass cell (one word);
  - for an instance with prepared targets (the EQ), one cell per `(target slot, channel)` of
    `PREPARED_EFFECT_TARGET_WORDS` (12) words; a `Both` target writes both channel cells.

  The cells and their dirty words are charged in place of the queue and target staging.
- **D2. Observe stays a FIFO.** A small observation queue beside the cells keeps
  `EffectControlRecord::Observe` in order (D15-2 condition 1). Its depth is the number of the
  effect's observation taps.
- **D3. Canonical drain.** Bypass first, then parameter cells in `(parameter_index, channel)`
  order, then target cells in `(slot, channel)` order, then the observation queue. When both
  channel cells of one parameter or target are dirty with equal words, render applies one `Both`.
  The witness folds from the final cell contents: equal `Left`/`Right` values are a pair, so the
  #1004 deferral becomes structural.
- **D4. The span window holds every live cell (root ruling 5, 2026-10-05).**
  - The lane's staging window is sized at preparation to hold at least one span for every live
    parameter cell, beside `automation_capacity`. Render stages every applied cell in the block
    that reads it. There is no deferral path.
  - A deferral would split one revision across blocks, against root's binding requirement (#1432
    Amendment 1). It would also resolve a covered value under a later snapshot, so a value that
    should apply could be counted as superseded.
  - Sizing the window for stored automation is #1306.
- **D5. Producers and admission.** `EffectControlProducer` writes cells: `preflight` keeps its
  domain checks, writes are infallible. The EQ owner transaction keeps its revision checks and
  loses its capacity check. In `commit_live` the queue-capacity rebuild (`:1266-1273`) and the
  effect room checks (`:1315-1320`, `:1339-1348`) go; every remaining check still precedes the
  first write. The browser's effect band leaves the room pass and `in_flight`.
- **D6. Header and docs.** The 16-record effect room and "an effect edit that could never fit its
  lane rebuilds" leave the header; `docs/C_ABI_V1_QUALIFICATION.md` follows.
- **D7. The acked-batch question: can an ack ever precede a drop? No.** Every fallible check
  precedes the first cell write and writes cannot fail; a replaced value is in the committed
  model, render applies the newest, and #1312's counter records the replacement. D4 stages every
  applied cell in its block. Observe records keep FIFO order.

## Deliverables

1. Cells and the observation queue in `effect-contract`'s live lane, the bank and per-node paths
   included; producers in `effect-compiler`.
2. C ABI and browser admission (D5), header and docs (D6).

## Authorized paths

- `crates/effect-contract/src/live.rs` (before #1280; then E5), `crates/effect-contract/src/` tests.
- `crates/effect-compiler/src/prepare.rs` (the producer and lane attachment only).
- `crates/rack/`, `crates/graph/` only where they construct the lane or call `stage` (stream A's
  crates; sequenced by the coordinator).
- `crates/control-plane/src/`, `crates/capi/` (header prose, tests).
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (effect band; stream H's files).
- `hosts/host-web/tests/strip_cells_cross_host.rs` (gate 5's extension of #1399's test).
- `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- Strip, input and route lanes (#1312, #1346, #1347). Automation spans (#1058, #1306).
- The bypass crossfade (#1341).

## Objective gates

1. **Many edits, one block (new capi test).** Paused host: 40 edits that change only the left
   channel of a compressor's threshold (a `PerLane` parameter, so one `Left` cell), then 40 edits
   of one EQ band's gain on both channels (a `Both` target, so two cells), each `RESULT_OK`; one
   render equals a twin that made only the last of each; `LIVE_VALUES_SUPERSEDED` grows by
   `39 + 2 × 39 = 117` in #1312 D2's per-cell unit.
2. **Bypass ahead of parameters (new capi test).** One transaction lifts a bypass and changes a
   parameter: the block renders as the twin's rebuild of the committed model.
3. **Window holds every live cell (new effect-contract test).**
   - For every launch effect descriptor, the prepared staging window is at least its live
     parameter cell count.
   - A synthetic descriptor with more live parameter cells than `automation_capacity` writes
     every cell in one transaction, and one block applies all of them.
   - Mutation (PR evidence): the window sized to `automation_capacity` alone turns it red.
4. **Symmetry (keep green, no change):** the #1004 tests on a both-channel `PerLane` edit keep a
   mono bank collapsed; a one-channel edit still clears `LIVE`.
5. **Both hosts agree (extend the cross-host test of *Report live_values_superseded in the browser
   status and prove both hosts drain strip cells alike*, #1399 gate 2,
   `hosts/host-web/tests/strip_cells_cross_host.rs`):** two parameter edits and one EQ gain edit
   between two blocks, through both admissions: bit-identical PCM and equal counters.
6. **Superseded tests.** Delete `an_effect_edit_larger_than_its_queue_rebuilds`,
   `a_full_effect_lane_refuses_before_anything_changes`,
   `an_eq_edit_designing_more_targets_than_its_queue_rebuilds` and
   `an_eq_bypass_and_targets_without_room_for_both_refuse_before_anything_changes`
   (`crates/capi/src/runtime/live_tests.rs:1943`, `:2016`, `:2419`, `:2680`); gate 1 replaces
   them. Rewrite the browser effect-band backpressure assertions the same way.
7. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   (all violation counts 0); `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-effect-contract.sh`; `bash scripts/check-web-audioworklet.sh`; the browser
   legs of `qualification.yml`'s `browser` job.
8. **Workspace.** `cargo test --locked -p effect-contract`; `cargo test --locked -p
   effect-compiler --features test-support`; `cargo test --locked -p capi`; `cargo test --locked
   -p host-web --features test-support`; the DSP crates' tests as `test-debug-b` runs them;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: an effect lane left a queue, or a miscount of superseded parameter or target values.
- Gate 2: a drain order that applies parameters before the bypass changes.
- Gate 3: a window smaller than the live cell count, which would drop or defer a cell and so
  split a revision across blocks.
- Gate 5: the two hosts staging effect cells differently.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
- *Report live_values_superseded in the browser status and prove both hosts drain strip cells
  alike* (#1399): the cross-host test gate 5 extends.
- **Deferred into this issue by #1504** (recorded in #1504's "Batch follow-ups", m1).
  `crates/graph/tests/live_snapshot_drains.rs` probes a partial bank (inactive lanes), not a live
  bypassed lane, because no probe seam exists for a bypassed lane until the effect lanes read cells.
  This issue adds that case: a bank with one lane live-bypassed whose drain is skipped under the
  block's snapshot. Until then only rack's `un_bypassing_returns_the_current_wet_signal` covers it.
- **Header wording (#1312 batch follow-ups, NIT 5).** `crates/capi/include/miso_engine_v1.h` says a
  transaction with both fader and effect values can see its effect values land one block before its
  fader values. This issue deletes that sentence together with the "up to one quantum apart" text
  when the effect values join the cells.

Dependent: *Carry live-controlled effect lanes across a plan swap* (#1280) carries the cells this
issue creates, so it lands after this issue and edits `crates/effect-contract/src/live.rs` after it.

## Amendment 1 (root, 2026-10-05)

**Status.** Root amends this issue before any attempt. It adds one requirement to the rewrite of
the effect lane's drain, which this issue already makes in `crates/effect-contract/src/live.rs`
and in the `crates/rack` code that calls `stage` (its Authorized paths). The attempt budget does
not change.

**Why.** The realtime-policy tool (stream J, *Check realtime regions with a Rust syntax-tree tool*)
refuses, inside a marked region, a call to a function that pops when the call is in a loop or a
closure (slice B2b-2 (#1444), root ruling R3). Root ruled (2026-10-05) that each real render loop that
calls a popping function goes into the issue that owns its file, so that it is marked and passes
that rule, or is restructured, before the tool lands. One such loop is in the code this issue
rewrites. Stream A owns `crates/rack`; this issue's existing named exception for the rack code
that calls `stage` covers the change.

**Problem, added** (verified on `codex/d15-stream-b` at `b8392df66`, stream B batch 1, merged with
`origin/main` at `6d28a80ec`; `crates/rack` and `crates/effect-contract` are the same on both and
on `6b9067ede`).
- `LiveControlEffectBankStage::drain` (`crates/rack/src/lib.rs:1257-1284`) is render code: it is
  the body of `BankStage::begin_block` for the live-control bank stage (`:1211-1213`). It loops
  `for lane in 0..lane_count` (`:1261`) and calls `channel.stage(&mut self.staging, first_sample,
  observation)` once per lane (`:1268`).
- `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:341`) drains the lane's record
  queue: count `:349-352`, bound `:359-365`, pop `:366`. So the rack loop calls a popping function
  in a loop. It sits outside every marked region, which is the tool's stated limit "a loop in an
  unmarked caller"; marked as it is, B2b-2 refuses `:1268`.
- After this issue, parameters, bypass and EQ targets are cells, and `Observe` stays a FIFO (D2).
  So `stage` (or what replaces it) still pops, once per lane, and the bank loop still reaches it.

**Decisions, added.**
- **D8. No render loop reaches a pop through a call.** In the code this issue writes, every loop on
  the render path whose body pops a queue holds that pop in its own body, in the counted form that
  the realtime drain rule accepts:
  - the count is read once at entry, `let available = <recv>.available_at_entry()[.min(<cap>)];`
    or the `map_or(0, <Type>::available_at_entry)` form, and the pop's innermost loop is
    `for _ in 0..available` or the `while available != 0 { available -= 1; .. }` form (tool slices
    B1a-D6 and B1a-D8);
  - every loop around it is a finite form (tool slice B1b-D1) that selects a different lane's queue
    on each pass (#1418 Amendment 1, D1), as `for lane in self.lanes.iter_mut()` does;
  - the pop's receiver is the counted receiver (#1426 Amendment 1, D1).

  **The shape (fourth review, MAJOR-5).** #1418's invariance rule lets the selecting loop name the
  lane only in its binding and in the receivers of `available_at_entry` and `try_pop`. So:
  1. a first loop over the lanes stages each lane's cells (parameters, bypass, EQ targets) with
     pop-free lane methods;
  2. a second, drain-only loop, `for lane in self.lanes.iter_mut()` (or `.enumerate()` of it),
     holds each lane's counted observation drain. It names `lane` only in its binding, its count
     and its pop, and hands each record to a sink that is not reached through `lane`: for example
     the `observation` that `LiveControlEffectBankStage::drain` already takes (`:1268`), with the
     lane's index.

  The two loops may sit in the rack stage, or in one `live.rs` function over the lanes that the
  rack stage and the per-node path each call once, in no loop. The implementer picks the place and
  records why. These shapes are refused, measured on `6d28a80ec` with attempt 2's gate: one lane
  loop that both stages and drains; a drain loop that also calls a method on the lane
  (`channel.apply_observe(..)`); and the index form `self.lanes[lane]` in an `if let`. If the
  record cannot be applied without naming the lane in the drain loop, stop and report; #1418's
  rule is not relaxed for this issue.

  A wrapper that moves the pop one call deeper, or a rename, does not meet D8 (B2b-2-D3).
- **D9. Mark it.** Put the function that holds the bank's drain loop in a realtime region
  (`// REALTIME_POLICY_BEGIN` and `// REALTIME_POLICY_END`, replacing blank lines where there are
  some). If the function it calls per lane, or the per-node path's drain, is not already in a
  region, mark it too. Raise the region floor by the regions it adds, under STREAMS' standing
  exception for the realtime-policy floors:
  - in the awk gate, the region floor line (`:76` on `b8392df66`, `:74` on `6d28a80ec`);
  - in the awk self-test, whose base tree sits exactly on the floors, the pad loop, its comment and
    the region-floor message (`scripts/test-realtime-policy.sh:445-454` and `:1414` on
    `b8392df66`), as `98a2d6bfc` authorized for #1314. Without it the self-test exits 1.

  `Policy::workspace()` in `tools/realtime-policy` is not on `main` while this issue runs: the J
  tool batch pushes once, after this issue lands. The batch re-measures its floors when it
  rebases.
- **D10. The per-node path.** A per-node effect calls the lane's drain once per block, in no loop.
  That is allowed as it is; D8 applies only to loops.

**Authorized paths, added.** `scripts/check-realtime-policy.sh` (the region floor line only) and
`scripts/test-realtime-policy.sh` (the pad loop, its comment and the region-floor message only),
both under STREAMS' standing exception for the realtime-policy floors.

**Objective gates, added.**
- **9. The marked bank loop passes (the committed guard).** `bash scripts/check-realtime-policy.sh`
  and `bash scripts/test-realtime-policy.sh` pass with D9's markers and floors. Stream J's tool is
  not on `main` yet: B2b-2's gate 1 checks this function when the batch lands, and B2b-2 commits a
  twin of D8's shape and holds today's shape red (its gate 3).
- **10. #1418's rule accepts it (PR evidence).** On a scratch export of this change, replace
  `scripts/check-realtime-policy.sh` with attempt 2's gate (`1418-attempt2-check-realtime-policy.sh`
  in the probes folder, with this change's floors) and run it: it prints
  `realtime policy: ok (..)`. Record the line.

*Test value, added.* No new test: D8 changes where the pop sits, not what the lane stages, so
gates 1-5 hold the behaviour. The markers are the committed guard: they are red if a later change
moves the pop back behind a call in the lane loop.

**Dependencies, added.**
- Before (other streams): stream J's slice B2b-2 (*Check realtime regions with a Rust syntax-tree
  tool: refuse a call to a popping function in a marked loop or closure*), which waits for this
  issue.

**Size.** D8 and D9 add about an hour to this issue's rewrite of the same two functions.

## Amendment 2 (root, 2026-10-05): revision-bounded cells

Root ruling 5 (2026-10-05): D4's window holds every live cell, and its deferral and gate 3's
deferral case go. The cells read under the block's snapshot (#1432 Amendment 1, #1312
Amendment 2). D3's drain order is unchanged.

## Amendment 3 (root, 2026-10-12): the observation queue's one-record-per-tap rule (attempt-1 M1)

Root's ruling on the attempt-1 verdict's M1, copied from root's review of the proposal. Decisions
D2, gate 6 (below, named "Gate 6 (M1)" so it is not mistaken for gate 6 of the issue above),
authorized paths and D8.

**D2 Amendment 1 (root, 2026-10-12; attempt-1 M1).** An `Observe` record overwrites its tap's whole
state at the block that drains it, and every record of one submission drains in one block. So a
submission names each observation tap at most once. Admission refuses a second `Observe` for the
same (strip, rack, effect, tap) in one submission with `COMMAND_REASON_MALFORMED` at that record's
wire index, before the room pass and before any write. Admission never coalesces: Observe records
stay FIFO and are never superseded (D15-2). The queue depth stays the number of the effect's
declared taps. With this rule, every admitted submission fits an empty queue, so
`COMMAND_REASON_BACKPRESSURE` from the effect band only means records of earlier submissions are
not yet drained, and it clears after one render. The SDK's managed `update()` sends one re-arm
(`armed=true`, new window) for a tap its handle alone owns, not a disarm and an arm. Every managed
submission sends at most one record per tap. The C ABI has no observation path and does not change.

**Gate 6 (M1).** In `sdk/test/observation-subscription-evals.mjs`, against the real headless engine
(`createOfflineEngine`):
1. Subscribe a managed handle to the compressor's Gain Reduction tap at `windowBlocks: 2` and
   render until it is ready. Then `update()` the same selection with `windowBlocks: 4`. It
   resolves. After 4 renders the reading has `window.blocks === 4` and `window.firstSample >=` the
   update receipt's `appliedAtSample`. The submit hook records exactly one Observe edit for the
   update.
2. A raw `liveControls.submit` of `observe(false, 2)` then `observe(true, 4)` on that tap in one
   call returns `ok=false`, `reason=MALFORMED`, `admitted=0`, and the next reading still has
   `window.blocks === 4`.
3. Subscribe, then close, with no render between them: both resolve.

Also restore the one-submit re-arm in `observation-evals.mjs:172-187`, as one `observe(true, 2)` on
the armed tap with no withdrawal and no render before it.

*Test value.*
- Bullet 1 turns red if the SDK sends disarm plus arm for one tap, or if the observation depth
  cannot hold a whole submission. No existing eval catches either, because none calls `update()`
  with a window change on the real engine.
- Bullet 2 turns red if admission sends a batch that can never fit to BACKPRESSURE, which invites
  a retry that can never succeed, or if it partly applies the batch.
- Bullet 3 turns red if consecutive managed operations between renders throw. This is the parent's
  accepted behaviour, which the 1-slot queue changed.

If bullet 3 is red, the fix is SDK-side: on BACKPRESSURE, retry after the applied watermark moves,
the same flow-control policy as `sdk/src/core/writer.ts:245-253`.

**Authorized paths, added.** `sdk/src/core/observation-subscriptions.ts`,
`sdk/test/observation-subscription-evals.mjs`, `sdk/test/observation-evals.mjs`,
`sdk/src/browser/shipped-host.d.ts` and `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
(the twins; one sentence in the reason docs, and the m2 declaration prose).

**D8 amendment (J3 #1442).** The stage drain in `crates/effect-contract/src/live.rs` uses a direct
count and direct pops:
`if let Some(control) = self.control.as_mut() { let available = control.available_at_entry();
let mut remaining = available; while remaining != 0 { remaining -= 1; let Ok(record) =
control.try_pop() else { break; }; ... } }`. The `.map(Consumer::try_pop)` and
`map_or(0, Consumer::available_at_entry)` forms go. (On this branch the popping drain left in
`live.rs` is the observation drain, `drain_observations`, the cells having replaced the record
drain; the shape is applied to it, receiver `observe`.)

## Amendment 4 (root, 2026-10-12): the SDK's wait on observation backpressure (attempt-2 M1)

Root's ruling on the attempt-2 verdict's M1 (`submix-verdicts/1345-attempt2.md`, "SDK wait: hang,
reorder, alternatives"). It supersedes the last paragraph of Amendment 3 ("If bullet 3 is red, the
fix is SDK-side: on BACKPRESSURE, retry after the applied watermark moves") and Gate 6 (M1)
bullet 3 as written there.

**D2 Amendment 2 (root, 2026-10-12; attempt-2 M1).**
- Headless (caller-driven render): no `nextRender` wait. A backpressured managed submit is refused
  at once with the existing typed MisoEngineError code `backpressure` (commandFailure), which is
  retryable. The caller renders and retries. Remove `nextRender` from the headless transport.
- Browser (autonomous render): the managed owner may wait for the next render and resend, ONLY on
  `backpressure`. The wait is bounded by `requestDeadlineMs`, and at the deadline it ends in the same
  typed `backpressure` error. It never waits without a bound while the AudioContext does not render.
- Gate 6 bullet 3, per engine: headless, subscribe then an awaited close with no render gives a typed
  `backpressure`; a render and a retry then succeed. Browser: it resolves within the deadline, and
  with the context suspended it ends in typed `backpressure` at the deadline. Each case gets a
  test-value sentence. Include the exact hang the verifier probed (`await handle.close()` before
  render) as a case.
- m1: add a test that turns red if the retry runs on any reason other than `backpressure` (e.g. a
  MALFORMED refusal must reject at once, with no resend).
- Fold the NITs from the attempt-2 verdict (stale "command queue" prose at
  `host-core/src/prepare.rs` and `effect-compiler/src/prepare.rs`; the doc of
  `a_submission_names_each_observation_tap_once` overclaiming; `round_trip.rs` comment stays as
  accepted). The `worklet-host.js` "worklet-side queue depth" prose stays with #1524.

**Named exceptions ratified by root (2026-10-12).** Files outside this issue's path lists that the
attempts touched, now part of its decision record:
- `sdk/src/headless/engine.ts` and `sdk/src/browser/engine.ts` (the owner's transport seam);
- `sdk/src/core/writer.ts` (prose) and `sdk/test/writer-evals.mjs` (attempt-1 m10);
- `tools/console-workload`, `tools/audit` and `tools/parameter-metadata` (prose and attempt-1 m9);
- `crates/host-core` prose (and `crates/effect-compiler/src/prepare.rs` doc prose);
- the D15-2 ruling document (the parenthetical naming #1345, #1346 and #1347);
- attempt 1's `crates/graph-compiler`, `crates/host-core`, ABI-fixture and callgraph-script edits
  (attempt-1 m13).

**Authorized paths, added.** `sdk/src/browser/engine.ts`, `sdk/src/headless/engine.ts` (above).

**Gate 6 (M1) bullet 3, as built.** Per engine, in `sdk/test/observation-subscription-evals.mjs`:
- Headless, against the real engine (`createOfflineEngine`): subscribe, then `await handle.close()`
  with no render rejects at once with `MisoEngineError` code `backpressure`, and nothing is admitted
  (the handle is open, `pump()` is not "already in flight", the tap stays armed after the next
  render); a render and the same `close()` resolve and the tap is unarmed after the next render.
  The same holds for `update()` (the handle keeps its configuration) and for a `subscribe` made
  while a close's disarm holds the slot.
- Browser: no browser runs on the development machine, so the real browser entry (`createEngine`)
  runs over a scripted host that models the effect's one-slot queue (a command is admitted while
  the slot is free, refused with result `backpressure` / reason `backpressure` while an earlier
  command holds it, and a render, `renderedQuanta` moving, frees it). With the context rendering
  within the deadline, the close resolves after the render and the same record is sent again; with
  the context suspended and `requestDeadlineMs: 80` it rejects with `backpressure` after the
  deadline, and the engine stops polling the host. The CI browser legs exercise the same owner
  against the worklet (see Attempt 3, open item 1).
- m1: a `malformed` refusal on the browser engine rejects at once (`invalidArgument`), with no
  resend and no wait for a render.

*Test value.*
- Headless close/update/subscribe: turns red if the headless engine waits for a render (a hang for a
  caller that awaits before it renders), if a refusal admits part of the submission, or if it leaves
  the owner busy. No other eval awaits a managed operation on a drained-less slot.
- Browser wait: turns red if the wait has no bound (a suspended context holds the caller and keeps
  polling), or if the resend does not follow a render.
- Browser malformed: turns red if the retry runs on any reason other than `backpressure` (the
  mutation that drops the reason check left all 367 attempt-2 evals green).

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b3`; checkpoints `09190ea55`, `122bdbbc0`, `ba14de159` and the docs commit that follows them)

**What landed.**
- `effect-contract/src/live.rs`: the lane is cells. `EffectLaneShape` (cell counts from a descriptor
  and the effect's target slots), `EffectCellWriter` (control side: `check`, `write(stamp, record)`,
  `try_push_observe`), `EffectControlLane` (render side: `stage` reads bypass, then the parameter
  cells in `(parameter_index, channel)` order, then the target cells in `(slot, channel)` order,
  all under the block's snapshot; `drain_observations` pops the observation queue apart). Equal
  `Left`/`Right` words of one parameter or target applied in one block are a pair: the witness keeps
  `LIVE`, a target is applied once as `Both`. The superseded count is charged only through
  `CellReader::read_counted`. `// REALTIME_POLICY_BEGIN/END` surrounds the render-side impl, `Reads`
  and `apply_observe`.
- D4: preparation raises each instance's `automation_capacity` to `max(S, live parameter cells)`; the
  window equals the capacity (the #1012 pairing equality), and `check_window` refuses a window below
  the lane's parameter cells at bind (`RackError::AutomationWindow`, an `.expect` in the per-node path).
- D2: the observation queue holds one slot per declared tap (`observation_taps` is the host's cap).
- D5: `EffectControlProducer` writes cells (`preflight` refuses a record with no cell, and refuses a
  raw `PreparedTarget`: targets only go through the owner transaction). The EQ owner keeps its
  revision/shape checks and loses capacity (`preflight_candidate_targets`,
  `publish_candidate_targets(stamp, ..)`). `commit_live` lost the queue-capacity rebuild and the room
  checks; `LiveBackpressure` and `control.live.backpressure` are gone. host-web writes effect values in
  `write_strip_cells` under the batch's one stamp (EQ via the owner marker); only `Observe` is queued
  and room-checked; `queue_available` is the observation queue's room.
- D8/D9: `LiveControlEffectBankStage::drain` is two loops (stage every lane's cells, then a drain-only
  loop over `(lane index, Consumer)` pairs whose pop is `queue.try_pop()` under a counted
  `available_at_entry`); `LiveControlEffect::stage` (graph) is region-marked. Floors: 106 regions in
  27 files (was 104).
- The #1504 deferral: `crates/graph/tests/live_snapshot_drains.rs` gained `effect_bank::
  a_live_bypassed_lane_is_drained_every_block_under_the_blocks_snapshot`.
- Docs (D6 and the doc set): `miso_engine_v1.h` (effect-before-fader sentence, "up to one quantum
  apart", the 16-record room, the rebuild-for-too-big text and `control.live.backpressure` deleted;
  the exact rule stated), AGENTS.md (the one sentence, no "until then"), D15-2 in the ruling,
  `docs/C_ABI_V1_QUALIFICATION.md` (timing, cells, the #1263/#1264/#1265/#1266 paragraphs, resource
  movement).

**Choices left open by the spec.** (1) Canonical order is by construction in `stage`; it is not
observable at the effect's output (bypass and parameters are applied in the same `stage` call and the
rack applies the shunt after), so gate 2 is an end-to-end equality gate, not an order gate; the
mutation that moves the bypass read after the parameters turns no test red and none can. (2) The
observation drain sits in the rack stage as a second loop (not a `live.rs` function) because the
per-node path calls `drain_observations` once and the bank owns the dense `(lane, queue)` table.
(3) A queue-less lane is not an `Option` in the table: the bank stores only lanes that have a queue,
with the lane index, so the pop's receiver is the loop pattern (the `Option::as_mut().map(try_pop)`
form fails #1418's rule: measured with attempt 2's gate on this tree before the change).

**Files outside the spec's list, each with its reason.**
- `crates/graph-compiler/src/{estimate,lib}.rs` and its two tests: the estimate charges what a lane
  retains (`EffectLaneResources`) so estimate equals prepared bytes exactly (four cases checked).
- `crates/host-core/src/{prepare,limiter_linked_session}.rs` and tests: the attach call takes the
  superseded count and the tap cap; test helpers write cells.
- `crates/effect-compiler/src/control.rs`: the owner loses its capacity variant.
- `crates/effect-contract/src/{lib,symmetry,prepared_target}.rs`: re-exports, pairing prose, the
  defaulted `target_slots()` on the target-preparation trait.
- `crates/capi/src/ffi.rs`, `crates/capi/tests/resource_lifecycle.rs`: `LiveBackpressure` arm and
  counter removed.
- `tools/console-workload` (src + tests), `tools/audit/src/capi.rs` (a comment),
  `tools/parameter-metadata/tests/round_trip.rs`: ported (details below).
- `hosts/host-web/tests/browser-v1/{direct-oracle.mjs,expected.json}`: the effect-bypass flood entry
  is removed; the native digest is unchanged.
- `sdk/src/core/writer.ts` (header prose), `sdk/test/{writer-evals,observation-evals}.mjs`: the
  headless gate failed on the removed effect queue (below).
- Re-pins, each individually. (a) `scripts/fixtures/abi-layout-v1-self-test.json`: stale since #1399
  (`liveValuesSuperseded` u64 at offset 48, `reserved` u64[3] at 56); regenerated with
  `--print-abi-layout`; it made `parameter-metadata`'s `the_checked_in_self_test_fixture_is_current`
  fail on the branch before this change. (b) `scripts/check-web-audioworklet-callgraph.py`: the two
  `EffectControlProducer` pins `..publish_candidate_targets` and `..preflight_candidate_targets`
  keep their site counts (4 each) and reason; only the impl-block disambiguator moved `Ms2_` to
  `Ms1_` because the producer lost an `impl` block. (c) The region floor and its self-test pad
  (104 to 106), as the standing exception. No resource-ceiling or fixture digest moved: the reference
  rows stay inside their ceilings (below), and the fixture/manifest checks pass unchanged.

**Tests moved, deleted, new.**
- Deleted (gate 6, plus those their rewrite made false): capi `an_effect_edit_larger_than_its_queue_
  rebuilds`, `a_full_effect_lane_refuses_before_anything_changes`,
  `an_eq_edit_designing_more_targets_than_its_queue_rebuilds`,
  `an_eq_bypass_and_targets_without_room_for_both_refuse_before_anything_changes`,
  `an_eq_bypass_beside_targets_filling_its_queue_rebuilds`, and the `effect_room`/`eligible_values`
  helpers; host-web's native effect-bypass timeline flood and its `expected.json` row; paired_spans'
  queue-at-capacity and `Both`-span scenarios; the SDK's four paused writer evals on EQ records.
- Rewritten: host-web backpressure and flood tests now fill the observation queue (depth 1 at the
  observed compressor); `loaded_effect_queues` became `written_effects`; the effect-contract
  `live_control`, `paired_spans`, `observation_lane` suites run on `Rig` (a lane, its writer, the plan's
  gate); host-core `symmetry_witness` (the EQ per-lane-target test became two: equal words in one block
  keep the lane, unequal or split across blocks decline it); console-workload `paired_spans` on cell
  writers (a long run of pairs before one block is `Kept`; a run ending in a lone half is retired; per-
  channel EQ owner edits are `Kept`); console-workload `automation`
  `the_eq_lowering_keeps_the_collapse_when_a_pair_lands_in_one_block` (the old "harness-only lowering
  retires the collapse" claim is false once the cells hold the latest word per channel; the retiring
  case is now a block between the two edits); `chain_shape` pushes the `Right` half (a `PerLane`
  parameter has no `Both` cell; the old test pushed a `Both` the effect refused and so never moved the
  right channel); `round_trip` renders between a subscribe and its unsubscribe (depth 1).
- New, with the defect each defends (mutation runs below):
  - effect-contract `live_window`: a window smaller than the live cell count (a cell dropped or
    deferred, splitting a revision); the synthetic effect has 85 cells (> one dirty word).
  - effect-contract `live_control`/`paired_spans`/`observation_lane`: cell identity per
    `(parameter, channel)`, last-write-wins with an exact superseded count, one transaction in one
    block under the snapshot, a value above the snapshot waiting exactly once, the pair rule in either
    write order and split across blocks, target cells folding the witness from final contents.
  - effect-compiler `native_session`: every launch effect is prepared with a window for all its live
    cells; an entry below its cells is refused at attach.
  - rack `live_control_bank`: each lane's subscriptions reach that lane's taps in order; a revision's
    bypass and gain land together under the block's snapshot; a bank window below a lane's cells is
    refused at bind.
  - graph `live_snapshot_drains` (effect bank): a live-bypassed lane is drained every block under the
    block's snapshot (the #1504 deferral).
  - capi `forty_effect_edits_of_each_kind_before_one_block_are_all_accepted_and_the_last_is_heard`
    (gate 1: 40 left-only threshold edits, then 40 `Both` EQ band-gain edits, each `RESULT_OK`, one block
    equals the twin that sent only the last of each, `LIVE_VALUES_SUPERSEDED` += 117), capi
    `a_bypass_lifted_beside_a_parameter_renders_like_the_browsers_lane` (gate 2, against host-core's
    lane reference) and `a_refused_eq_transaction_leaves_its_owners_idle_and_writes_no_cell` (D7).
  - host-web `strip_cells_cross_host::both_hosts_drain_two_parameter_edits_and_one_eq_gain_edit_alike`
    (gate 5; equal PCM, equal counters, superseded 1).
  - SDK `writer-evals` "a paused host takes any number of effect edits, refuses none, and the last of
    each lands" (the writer/host reintroducing a bound on effect edits or losing the last value) and
    three observation-queue refusal evals that keep the writer's flow-control contract on the one
    queue left.

**Gates.**

| gate | command | result |
|---|---|---|
| 1 | `cargo test --locked -p capi --lib forty_effect_edits` | pass; superseded 117 |
| 2 | `cargo test --locked -p capi --lib a_bypass_lifted_beside` | pass |
| 3 | `cargo test --locked -p effect-contract --test live_window`; effect-compiler `native_session` | pass |
| 4 | host-core `symmetry_witness`, console-workload `paired_spans`/`chain_shape` | pass (the `Both`-edit collapse stays; one-channel edit still clears `LIVE`) |
| 5 | `cargo test --locked -p host-web --features test-support --test strip_cells_cross_host` | pass |
| 6 | the four named capi tests are gone | done |
| 7 | `audit capi` | 0 violations, 100000 calls, `pcm_digest cb10fbface44a3a4` (equals the parent's); `check-realtime-policy.sh` ok (106 regions, 27 files); `test-realtime-policy.sh` ok; `check-effect-contract.sh` ok; `check-capi-abi.sh` ok; worklet chain below |
| 8 | `cargo test --locked --workspace --no-fail-fast` (4 targets failed on the first run, fixed: console-workload `chain_shape` and `paired_spans` x2, parameter-metadata `abi_layout` and `round_trip`); then effect-contract, effect-compiler `--features test-support`, capi, host-web `--features test-support`, control-plane `--features test-support`, rack, graph, host-core, console-workload, parameter-metadata | all pass; `cargo fmt --all -- --check` clean; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` clean |
| 9 | `check-realtime-policy.sh` / `test-realtime-policy.sh` | pass with D9's markers and floors |
| 10 | attempt 2's gate on a scratch export of this tree with 27 files / 106 regions and the current unsafe-exclusion line | `realtime policy: ok (106 marked regions in 27 files)` |
| extra | `check-workspace-policy`, `check-rack-policy`, `check-graph-policy`, `check-graph-determinism` (100/100), `check-host-core-policy`, `check-builtins-policy`, `check-builtins-fixtures`, `check-effect-runtime-{fixtures,policy}`, `check-protocol-control-policy`, `check-console-fixtures`, `check-console-benchmark-fixture`, `check-bench-policy`, `check-lane-policy`, `check-session-policy`, `check-ci-path-routing.py`, `check-abi-layout-v1.py --self-test` | ok |
| extra | `check-cross-targets.sh` | PASS |
| worklet | `build-web-audioworklet.sh --named-twin` (shipped `b44cf5b1d1feb075fb215a451d7777b27a3cc26883595942394c7248c1c71825`), `strip-wasm-names.py check`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts` (self-test 32 mutations), `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (private TMPDIR left empty) | pass after the callgraph re-pin (b); `check-web-audioworklet-v8-spill.py --self-test` ok (the exact-Node gate itself not run) |
| sdk | `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh`, `sdk-package.sh check` over the artifact above | pass after the SDK eval rewrite; `sdk/dist` deleted |

Not run: the Playwright `browser` legs (no browsers on this machine), AArch64.

**Resource movement** (nine-track EQ reference session, x86-64, eight lanes, `reference_session_retained_rows_stay_within_their_budgets`, parent `710f77df7` -> this change): graph session+plan 393,718 -> 406,438; graph incremental 393,694 -> 406,414; graph metadata 185,521 -> 198,241; `capi_retained_bytes` 277,481 -> 281,513 (ceiling 282,432, headroom 919); largest allocation 90,720 unchanged. No ceiling needed a re-pin. The next slice that adds capi-retained bytes will need to raise `capi_retained_bytes`'s ceiling.

**Mutation runs** (applied, red; reverted, green).

| test | mutation | result |
|---|---|---|
| native_session `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells` | drop the `.max(live_parameter_cells)` in `prepare.rs` | red |
| live_window `a_window_shorter_than_..._count_it_needs` | `check_window` always `Ok` | red |
| effect-contract pair tests (7) | `paired = false` | 7 red |
| effect-contract / console-workload (`the_eq_lowering_keeps_the_collapse_when_a_pair_lands_in_one_block`) | targets with equal words not merged to `Both` | red (`[320, 8]` vs `[512, 8]`) |
| console-workload `paired_spans` | `paired = false` | red (5 tests) |
| effect-contract `a_drain_emits_the_contract_canonical_span_order` | `Left`/`Right` cell indexes swapped | red |
| effect-contract (5 tests) and capi gate 2 | parameter index map `^ 1` | red; capi gate 1 stays green (its twin shares the writer, so a consistent swap cancels) |
| capi gate 2 | bypass cell read but ignored | red |
| capi gate 1 | target cells read uncounted | red (`39` vs `117`) |
| graph effect-bank `live_snapshot_drains` | rack `drain` skips a bypassed lane; `stage` reads `LiveSnapshot::ALL` | red, red |
| host-web `a_mixed_batch_is_one_transaction_across_every_queue` | room pass reads `command_wanted[0]` | red (a push panics at the free-room expect) |
| host-web `strip_cells_cross_host` (gate 5) | browser publishes a `Both` EQ target as `Left` | red (the owner's shape check panics before PCM is compared) |
| effect-contract canonical order (bypass read after the parameters) | moved | no test red, by construction (see choice 1) |

**Open items for root.**
1. **Observation depth (D2 as written) is one slot per declared tap.** Every launch effect declares one
   tap, so a subscribe and its unsubscribe, or an unsubscribe and a re-arm, in one block interval is a
   typed `BACKPRESSURE`. The SDK's `observation-evals` re-arm (`observe(false)` then `observe(true)` in one
   submit) and `parameter-metadata`'s round trip both hit it; I rendered a block between the records. If
   the idiom must work in one submit, D2's depth needs two slots per tap (one line in `prepare.rs`'s
   observation sizing and the tests that fill depth 1). I kept D2 literal.
2. STREAMS.md "Watermark bound on `main` today" (`:312-317`) is stale after this change: the last two
   sentences ("A live value of that revision can apply earlier ... until #1502, #1503, #1504, #1312 and
   #1345 have all landed") should be replaced by "Every live value of that revision takes effect
   together, in the block `first_sample` names." I did not edit STREAMS.md.
3. Stale text I did not touch: `docs/C_ABI_V1_QUALIFICATION.md` still says the strip lanes are "a 16-record
   fader/mute ring and a 16-record matrix/pan ring" (`:319`, #1256-era), and the #1263 intro says "No
   input or route lane is attached"; both are older than this issue.
4. `control_queue_depth` (host-core) and `live_control_command_queue_records` (browser) size no queue now;
   only `Some`/`None` and `0` mean anything. A later cleanup could drop the number.
5. `parameter-metadata`'s abi-layout fixture was stale before this change (#1399); regenerated here.
6. Gate 2's order claim (bypass before parameters) is unobservable at the output, so only construction
   and the effect-contract canonical-order test hold it.

### Attempt 2 (implementer, on `codex/d15-stream-b3`; commits `01f859ba1`, `f05ef8066`, `aacf6afaa` and the record commit that follows)

Applies root's ruling on M1 (Amendment 3) and the attempt-1 verdict's other findings
(`submix-verdicts/1345-attempt1.md`).

**M1 (D2 Amendment 1).**
- Browser admission (`hosts/host-web/src/lib.rs`): a per-effect bit mask of the taps this
  submission has named (`observe_named`, cleared with `command_wanted`) refuses a second `Observe`
  for one (strip, rack, effect, tap) with `COMMAND_REASON_MALFORMED` at that record's wire index,
  in staging, before the room pass, any cell write or the stamp. Nothing coalesces; Observe records
  stay FIFO. The reason's doc is widened in `lib.rs` and in both `.d.ts` twins.
- SDK `update()` (`sdk/src/core/observation-subscriptions.ts`): a window change on a tap this
  handle alone holds sends one re-arm (the new key's arm); the old key sends no disarm. Duplicate
  selections in one request are refused by `validateObservationSelections` (its key is track, rack,
  effect slot, tap, so two channels of one tap are a duplicate too), so a managed submission
  carries at most one record per tap.
- C ABI: no `Observe` exists there (no opcode, no observation taps: `observation_taps` is 0, so
  lanes have no queue); a duplicate cannot arise and the C ABI does not change.
- Bullet 3 was red (`subscribe` then `close` with no render: `backpressure: observation
  transaction was refused`), so the SDK retry is added: `#submit` waits for the transport's
  `nextRender` and sends again, up to 8 renders, on `backpressure` only (the same policy as the
  live-control writer: flow control retried, anything else thrown). The headless engine resolves
  `nextRender` at its next `render()` (and at reload and `dispose()`); the browser engine polls the
  host's `renderedQuanta`. A transport without it throws as before. This needed
  `sdk/src/headless/engine.ts` and `sdk/src/browser/engine.ts` beyond the amendment's path list.
- Gate 6 (M1) bullets 1-3 are in `sdk/test/observation-subscription-evals.mjs`, against the real
  headless engine. The re-arm in `observation-evals.mjs` is one `observe(true, 2)` on the armed tap
  again. The host-web tests that attempt 1 fitted to the one-slot queue now say what the rule says
  (a flood of one submission is MALFORMED; an earlier submission still holding the slot is
  BACKPRESSURE; the mixed-batch, solo, bus and idle-counter tests fill the slot with an earlier
  submission, then refuse the batch whole), and `a_submission_names_each_observation_tap_once` is
  new. `tools/parameter-metadata`'s round trip keeps one render between its subscribe and its
  unsubscribe: they are two submissions, so the second is BACKPRESSURE until a block drains the
  first (the accepted residual), and the comment says so; it is flow control, not a workaround for
  the depth.

**D8 amendment.** `EffectControlLane::drain_observations` (the one popping drain left in
`live.rs`) is the direct form: `if let Some(observe) = self.observe.as_mut() { let available =
observe.available_at_entry(); .. while remaining != 0 { remaining -= 1; let Ok(record) =
observe.try_pop() else { break; }; .. } }`. `check-realtime-policy.sh` still prints `ok (106
marked regions in 27 files)`. There is no fixture twin of `live.rs` for the realtime-policy tool on
this branch (`tools/realtime-policy` is not here; only the awk gate and its self-test), so none
was updated.

**M2.** The estimate charges the bank's observation table per lane that has a queue,
`queued_lanes * size_of::<(usize, Consumer<EffectControlRecord>)>()` (48 B), not `Option<Consumer>`
(40 B) per bank lane. Reference-session rows move by -640 B (two eight-lane banks with no queue):
`graph_session_plus_plan_bytes` 406,438 -> 405,798, `graph_incremental_plan_bytes` 406,414 ->
405,774, `graph_metadata_bytes` 198,241 -> 197,601 (no ceiling moves; the qualification table
follows). `crates/graph-compiler/tests/bypass_resources.rs` gains
`an_observed_lanes_queue_is_charged_exactly_what_bind_retains` (taps off vs on, per node and banked:
charged == retained, 736 B and 6,656 B) and `a_lanes_own_accounting_is_what_the_allocator_sees`
(a lane's `retained_resources` plus its writer's equals the audited allocator's reading, for the EQ
and the compressor, with and without a queue; exact). **m8:** the lib resource test no longer
copies the queue formula (its lanes have no queue, so the table is empty), takes a lane's heap from
the accounting the allocator test holds, drops the vacuous assertion, and checks the real attach
path's compressor lane equals one built directly from its shape.

**Smaller findings.**
- m1: `live_control_command_queue_records` no longer raises `maximum_automation_spans_per_block`
  (`prepare_caps`); the option's doc says what the word does now (the boot word and
  `control_queue_depth` stay for a successor issue).
- m2: both `.d.ts` copies (byte-identical) say effect values are cells and only an observation
  subscription is a queue record; Malformed and Backpressure docs follow Amendment 1.
- m3: `docs/C_ABI_V1_QUALIFICATION.md`: the #1256 paragraph (rings were #1256's, cells since #1312;
  no input or route lane is attached, which is still true of the code), the #1263 intro, #1264's
  "pushes one record", #1265's "prepared-target staging", #1266's bypass record, and the resource
  rows.
- m4: header: "same cell"; an edit that moves both channels of a per-lane parameter or of an EQ
  target writes two cells, a shared parameter's edit one; reflowed to 100 columns (NIT 2).
- m5: `live_window`'s docs now say what each test catches; the preparation-level test is
  effect-compiler's `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells`, which
  now also writes every live cell of each prepared launch lane (requested capacity 1) in one
  transaction and stages them in one block into a window of exactly the prepared capacity. A
  synthetic descriptor through preparation would need a synthetic factory; the launch effects
  already exceed a request of 1 (14 cells for the compressor).
- m6: graph `a_per_node_effect_waits_for_the_revision_that_covers_its_value`.
- m7: the rack test with no unique catch is deleted.
- m9: console-workload asserts `apart_kept == kept - 3 * PREROLL_BLOCKS` and its doc says the
  three EQ cohorts stay retired for the whole preroll.
- m10: the SDK writer eval stages `[1, 2, 0]` and asserts the narrowed batch is admitted while the
  host is paused with the blocked record still pending.
- m11: gate 2's test doc states the deviation (the reference is host-core's lane given the same
  cells, the convention of the file; a rebuild twin matches only after the settling window and
  would not catch more), and its real catch (the C ABI dropping a `Bypass` beside parameter
  records). The spec's "twin's rebuild" is not implemented.
- m12: capi `one_transaction_with_more_cells_than_the_old_queue_held_stays_live` (every live
  parameter of a multiband compressor on both channels in one transaction: 20 cells > 16; one
  revision, no candidate, one write per cell, the render equals the one-edit-at-a-time twin), and
  host-web `a_flood_of_effect_values_is_admitted_whole_and_the_last_is_heard` (NIT 11).
- NITs: stale queue-era prose (rack, graph, host-core, control-plane, audit, console-workload,
  `sdk/src/core/writer.ts`) and the graph test's name; D15-2's parenthetical marks #1345 and names
  #1346 and #1347 (NIT 3); the browser push arm for a raw `PreparedTarget` is `unreachable!`
  (NIT 4); `preflight_candidate_targets` asks the writer about every target, so D7 is structural
  (NIT 5); the observation-order test asserts the window in force and the reversed order (NIT 6;
  `ObservationLane::window_blocks` is a new off-thread accessor); the duplicate paired_spans test
  and the rig-loop assertion are deleted (NIT 7); the #1504 test's doc claims only what it
  catches (NIT 8); the preparation comment says why the EQ is raised too (NIT 10). NIT 9 is prose
  only, unchanged.

**Gates (all on this tree, no browsers, no AArch64).**

| gate | result |
|---|---|
| `lint` job steps (fmt; clippy `--workspace --all-targets --all-features -D warnings`; rustdoc; workspace, session, env, bench, host-core, protocol, test-support, scalar-oracle, reachability, realtime (`ok (106 marked regions in 27 files)` and its self-test and leak checks), lane, unfused, rack/builtins/graph, effect-runtime, conformance, EQ seal policies; console and builtins fixture checks; the three sub-v3 probes) | pass |
| `test-debug-a` (`--workspace --all-targets` with the job's excludes: 1,573 passed, 0 failed; doctests) and `test-debug-b` (914 passed; doctests; `conformance_fixtures --check`) | pass |
| `test-release` steps (lane, math, wasm-gates; filter_liveness, tail_contract, ramp_endpoint, designer_total; FMA cfg; loom SPSC; M1 and F1 exhaustive) | pass |
| `audit-native` steps: `audit capi` (100,000 calls, all counts 0, `pcm_digest cb10fbface44a3a4`, the parent's), delay, compressor, EQ, gate audits, builtins and graph traces, protocol allocation audit, realtime probes and traces (1,000,000), effect-contract trace, `check-capi-abi.sh` and its self-test, `check-scalar-oracle-absent --native`, graph determinism, builtins, console and effect-contract fixture checks | pass |
| `check-cross-targets.sh`, `check-release-shape.py` | pass |
| worklet chain: `build-web-audioworklet.sh --named-twin` (shipped module `f16ba0b7af05f1ff5cb7bdb54b9ab10125c949736b8dcc0cc887f36bec26561b`), `strip-wasm-names.py` self-test and check, `check-web-audioworklet.sh --without-metadata-regeneration` (callgraph pins included), `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (private TMPDIR left empty), the V8 spill self-test and gate | pass |
| `check-abi-layout-v1.py` and `--self-test` | pass |
| `sdk` job checks in source-bundle mode: `check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types`, `check-sdk-headless` (367 passed, 0 failed), `sdk-package.sh check`; `sdk/dist` deleted | pass |

**Mutation evidence (applied: red; reverted: green).**

| change | red |
|---|---|
| browser admission: delete the duplicate-tap check | host-web `a_submission_names_each_observation_tap_once`, `observation_misuse_is_typed_and_all_or_nothing`, `idle_render_skips_admission_counter_clear_without_losing_queue_credit`; and, in the shipped module rebuilt with it, the SDK gate 6 bullet 2 only |
| SDK `update()` sends disarm plus arm | gate 6 bullet 1 (`invalidArgument: observation transaction was refused`) |
| SDK `#submit` does not retry on backpressure | gate 6 bullet 3, and the dispose-while-waiting eval |
| estimate charges `Option<Consumer>` per bank lane | `an_observed_lanes_queue_is_charged_exactly_what_bind_retains` (banked: 5,888 B charged for 6,656 B retained) |
| a lane's `retained_resources` omits `dirty` | `a_lanes_own_accounting_is_what_the_allocator_sees` (3,664 vs 3,672 B) |
| per-node `LiveControlEffect::stage` reads `LiveSnapshot::ALL` | the new graph test (the only red) |
| preparation drops `.max(live_parameter_cells)` | `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells` |
| `commit_live` rebuilds an instance with more than 16 records | the new capi test only (more than 3: also the two EQ parameter tests) |
| writer batch taken from the back of the pending map | the writer eval's paused-split assertion |

**Open items for root.**
1. Waiting for a render: the headless engine's `subscribe`, `update` and `close` now wait for the
   next `render()` when an earlier subscription still holds the tap's slot. A caller that awaits
   them and never renders does not get an error, it waits (the doc on `subscribeObservations`
   says so; `dispose()` ends the wait with the stale-owner error). The alternative (throw at once,
   as before) leaves gate 6 bullet 3 red.
2. Outside the amendment's path list: `sdk/src/headless/engine.ts`, `sdk/src/browser/engine.ts`
   (the `nextRender` hook), `sdk/src/core/writer.ts` (prose), and the graph-compiler, host-core,
   ABI-fixture, callgraph-script edits of attempt 1 (m13: STREAMS' hot-file rows do not list
   #1345 for them). I did not edit STREAMS.md; its "Watermark bound on `main` today" sentence
   still needs root's replacement.
3. `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` still calls the in-flight bound "the
   worklet-side queue depth" (a stale comment; the file is not on the path list).
4. The boot word `live_control_command_queue_records` and host-core's `control_queue_depth` stay
   (frozen ABI offset 32 and an `Option` flag); a successor issue removes the number.
5. AArch64 rows (four-lane resource figures) and the Playwright legs were not run.

### Attempt 3 (implementer, on `codex/d15-stream-b3`; commits `c7791ea9e`, `bb8827b4c` and the record commit that follows)

Applies Amendment 4 (root's D2 Amendment 2) and the attempt-2 verdict (`submix-verdicts/1345-attempt2.md`).

**M1 (the unbounded wait).**
- `sdk/src/core/observation-subscriptions.ts`: the transport's `nextRender` is gone. A transport
  that renders on its own clock may carry `renderWait: { boundMs, next(timeoutMs) }`; one without it
  (the headless engine) gets the typed refusal thrown at once from `#submit`. With it, the owner
  waits only on `backpressure`: one deadline of `boundMs` is fixed at the first refusal and spans all
  resends, each wait is capped by what is left, and a wait that ends without a render (`false`:
  deadline, or the engine closed) throws the same `commandFailure`, so the caller sees the typed
  `backpressure` error. The cap of 8 resends stays as a second bound against another writer that
  refills the slot.
- `sdk/src/headless/engine.ts`: `#renderWaiters`, `#releaseRenderWaiters` and the transport's
  `nextRender` are removed; `render()` is `this.#boundary.render(..)` again. `subscribeObservations`
  states that a refusal is typed, retryable, and that the caller renders and submits again.
- `sdk/src/browser/engine.ts`: `renderWait.boundMs` is `options.requestDeadlineMs ?? 5_000`.
  `next(timeoutMs)` polls `host.status().renderedQuanta` every 4 ms, resolves `true` when it moves,
  `false` at `timeoutMs` or when the engine is closed, rejects if the host rejects, and clears both
  timers when it settles (a pending `host.status()` that never answers is still cut off by the
  deadline). The `BrowserEngine.subscribeObservations` doc states the bound.
- Evals (`sdk/test/observation-subscription-evals.mjs`): the two attempt-2 wait evals (the close that
  waits, and the dispose-while-waiting eval, whose wait no longer exists) are deleted; five evals
  replace them (367 -> 370 evals in the headless gate).

**Gate 6 (M1) bullet 3, per engine, and test value.**
- *Headless, an awaited `close()` before any render* (the verifier's hang): rejects at once with
  `MisoEngineError` code `backpressure`, the owner is released (`pump()` is not "already in
  flight"), the tap is still armed after the next render, and a render and a second `close()` leave
  it unarmed. Test value: red if the headless engine waits for a render that only the caller makes
  (the hang), if the refused close disarms or half-applies, or if it leaves the owner busy; no other
  eval awaits a managed operation on an undrained slot.
- *Headless `update()` and `subscribe`, same slot*: an update before the slot drains is refused
  typed and the handle keeps its configuration; after a render the same update goes through; a
  subscribe made while a close's disarm holds the slot is refused typed, and succeeds after the
  render. Test value: red if a refused update changes the handle's state, or if a refusal for the slot
  waits.
- *Browser, context renders within the deadline*: a close whose slot is taken resolves after the
  render and the same record is sent again (three commands: subscribe, the refused close, the close
  after the render). Test value: red if the browser owner no longer resends after a render.
- *Browser, context suspended*: `requestDeadlineMs: 80`, the close rejects `backpressure` after the
  deadline (not before 70 ms), the host is not polled afterwards, and once the context renders the
  same close succeeds. Test value: red if the wait is unbounded (a suspended context holds the
  caller and polls the host for as long as it stays suspended) or leaks its poll.
- *m1, browser, a `malformed` refusal*: rejects at once with `invalidArgument`, no wait, exactly one
  refused close and no resend. Test value: red if the retry runs on a reason other than
  `backpressure` (the mutation was green on all 367 attempt-2 evals).
- The browser cases run the real browser entry, `createEngine`, over a scripted host that models the
  effect's one-slot queue; the pattern is the one `spectrum-browser-evals.mjs` and
  `live-controls-evals.mjs` use. No browser runs on this machine.

**NITs.** `host-core/src/prepare.rs` (two places) and `effect-compiler/src/prepare.rs` name the
effect's observation queue and the control lane that drains it, not a "command queue";
`a_submission_names_each_observation_tap_once` says the reason code is what catches an order after
the room pass, and the fader and armed-tap assertions state the all-or-nothing outcome beside it.
`round_trip.rs` is unchanged (accepted). `worklet-host.js` stays with #1524.

**Gates (this tree, no browsers, no AArch64).**

| gate | result |
|---|---|
| `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `test-debug-a` steps as the job runs them (`--workspace --all-targets` with the excludes and features; doctests) | 1,615 passed, 0 failed; doctests 24 |
| `test-debug-b` steps (DSP crates; doctests; `conformance_fixtures --check`) | 916 passed, 0 failed; doctests 7 |
| `scripts/check-workspace-policy.sh` | ok |
| `scripts/check-realtime-policy.sh`, `scripts/test-realtime-policy.sh` | `ok (118 marked regions in 27 files)`, as expected (no region added); mutation tests ok |
| worklet chain: `build-web-audioworklet.sh --named-twin` (shipped module `c98f68380b79b9a68cf7c216d15f3694943969b33f05551f1f1555a0fa4e44dc`), `strip-wasm-names.py` self-test and check, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (private TMPDIR left empty), V8 spill self-test and gate | pass (the only Rust edits since attempt 2 are comments and a test doc) |
| `sdk` job checks in source-bundle mode: `check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types` (both `.d.ts` twins byte-identical, `cmp`), `check-sdk-headless` (370 passed, 0 failed), `sdk-package.sh check`; `sdk/dist` deleted | pass (`npm ci` not repeated: `sdk/node_modules` was present) |

`check-cross-targets.sh` and `audit capi` were not re-run: no engine code moved since attempt 2
(Rust edits are comments and a test doc).

**Mutation evidence (applied: red; reverted: green).**

| change | red |
|---|---|
| headless engine gets a `renderWait` whose `next` never resolves (the attempt-2 wait) | the two headless evals (the awaited close; the update, subscribe and close refusals) |
| browser `next` loses its deadline timer | the suspended-context eval (the only red) |
| `#submit` retries on any refusal (drops `reasonName !== "backpressure"`) | the browser malformed eval (the only red) |

**Open items for root.**
1. Browser leg. The scripted-host eval stands in for a browser here. The CI `browser` legs
   (`hosts/host-web/qualification/sdk-response-entry.ts`, resident-observation case) should gain the
   case against the real worklet: subscribe, then `await close()` on a context that is not rendering,
   with a small `requestDeadlineMs`, ends in typed `backpressure`; after the context renders the same
   close resolves. I read the existing resident-observation flow: it makes no same-tap operation
   after a non-rendering gap (its `OfflineAudioContext` renders once, at `startRendering()`), so the
   new bound does not change it. I did not add the case: the qualification entry is outside this
   issue's paths and cannot run here.
2. `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` ("the worklet-side queue depth") stays
   with #1524.
3. Attempt 2's open items 2 (STREAMS' "Watermark bound on `main` today"), 4 (the boot word and
   `control_queue_depth` remain) and 5 (AArch64 rows; Playwright legs not run) stand.

### Follow-ups (verdict of attempt 3: m1, NIT 1, NIT 2, m3; Sonnet 5.5, on `codex/d15-stream-b3`)

Verdict: `submix-verdicts/1345-attempt3.md`, PASS. No source behaviour moved; the follow-ups are two
evals, two doc edits and one README paragraph.

- **m1.** `sdk/test/observation-subscription-evals.mjs`, "renders that never free the slot end the
  wait at one deadline from the first refusal": a scripted host renders every 60 ms and another writer
  refills the slot after each render (`keepRefusing`), with `requestDeadlineMs` 150. The close must
  reject with typed `backpressure` between 140 ms and 300 ms. Test value: a deadline fixed again per
  resend lets the wait run to the cap of 8 resends (about 480 ms here, 45 s at the default); no other
  eval renders during a refused wait without freeing the slot. Mutation: `deadline ??=` changed to
  `deadline =` at `sdk/src/core/observation-subscriptions.ts` `#submit` is red on exactly this eval
  (371 pass, 1 fail); reverted, 372 pass.
- **NIT 2.** Same file, "closing the engine ends a pending wait at once with the stale-owner error":
  `requestDeadlineMs` 3,000, `engine.close()` after 40 ms, the pending `handle.close()` must reject with
  the "owner changed" usage error in under 1,000 ms. Test value: without the close resolution in the
  browser wait the wait runs to the deadline. Mutation: deleting `if (closed) return finish(false);` in
  `sdk/src/browser/engine.ts` is red on exactly this eval (371 pass, 1 fail); reverted, 372 pass.
- **NIT 1.** The shared `ObservationSubscription.update` doc now says the headless engine refuses at
  once (the caller renders and calls again) and the browser engine first waits for a render, up to
  `requestDeadlineMs`.
- **m3.** `sdk/README.md` (managed observation paragraph) now states the contract: two managed
  operations on the same tap need a render between them; headless rejects at once with typed
  `backpressure`, browser waits up to `requestDeadlineMs` and then ends in typed `backpressure`.
  Named exception ratified by root, 2026-10-12, attempt-3 m3: `sdk/README.md`.
- **m2** (the real-worklet browser case) is root's successor issue; not done here.
