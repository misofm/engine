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
- **D4. The span window.** Render stages at most `automation_capacity` parameter spans per block,
  in D3's order; a dirty cell beyond that stays dirty for the next block and is neither dropped
  nor counted as superseded. A new test proves every launch effect has at most
  `automation_capacity` live parameter cells, so for launch effects this never defers. Sizing the
  window for stored automation is #1306.
- **D5. Producers and admission.** `EffectControlProducer` writes cells: `preflight` keeps its
  domain checks, writes are infallible. The EQ owner transaction keeps its revision checks and
  loses its capacity check. In `commit_live` the queue-capacity rebuild (`:1266-1273`) and the
  effect room checks (`:1315-1320`, `:1339-1348`) go; every remaining check still precedes the
  first write. The browser's effect band leaves the room pass and `in_flight`.
- **D6. Header and docs.** The 16-record effect room and "an effect edit that could never fit its
  lane rebuilds" leave the header; `docs/C_ABI_V1_QUALIFICATION.md` follows.
- **D7. The acked-batch question: can an ack ever precede a drop? No.** Every fallible check
  precedes the first cell write and writes cannot fail; a replaced value is in the committed
  model, render applies the newest, and #1312's counter records the replacement. D4 defers, never
  drops. Observe records keep FIFO order.

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
3. **Window bound (new effect-contract test).** For every launch effect descriptor, the live
   parameter cell count is at most its `automation_capacity`. A synthetic descriptor with more
   cells than capacity defers the excess one block and applies it then.
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
- Gate 3: a window that drops a dirty cell instead of deferring it.
- Gate 5: the two hosts staging effect cells differently.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
- *Report live_values_superseded in the browser status and prove both hosts drain strip cells
  alike* (#1399): the cross-host test gate 5 extends.

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
