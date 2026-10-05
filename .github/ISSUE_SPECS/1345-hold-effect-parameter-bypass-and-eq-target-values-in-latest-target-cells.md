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
  (`crates/capi/src/runtime/control.rs:1130-1137`) and refuses with `LiveBackpressure` when the
  room or the owner's target preflight fails (`:1179-1184`, `:1203-1212`). The queue depth is
  `min(16, automation capacity)` (`crates/capi/src/runtime/compile.rs:12-19`). The header states
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
  loses its capacity check. In `commit_live` the queue-capacity rebuild (`:1130-1137`) and the
  effect room checks (`:1179-1184`, `:1203-1212`) go; every remaining check still precedes the
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
