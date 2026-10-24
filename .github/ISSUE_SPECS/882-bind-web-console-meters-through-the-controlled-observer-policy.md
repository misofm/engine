# Skip per-track web meter work while no one holds the meter lease

**Rescoped by owner decision 11 (2026-09-29, `docs/rulings/engine-footprint-2026-09-29.md`) and by decision 17 (2026-10-13).** #1080 removed the controlled observer policy. Since #1581, every plan on every host carries one post-matrix sample-peak meter per strip, and the meter level (the browser's lease, the C ABI's `TELEMETRY_CONFIGURE` table) is one core value (#1582, #1588 `MeterLevel`). The goal is unchanged and now covers both hosts: a block rendered with the level off does no per-strip meter work; class A: with the level on, every published value is unchanged. The mechanism is the core level, read by render once per block, as #1581's output meter gate already does; no controlled-activation protocol. First read the per-block cost that #1581 records (its D7); if the saving is not worth a branch in render, close this issue with that measurement. Otherwise it is sequenced right after #1582.

## Slice (scope note of 2026-10-24, root rulings R1 and R2; supersedes the original slice and gates)

- **engine:** `PreparedRenderPlan` and `RealtimePlanOwner` carry a render-thread-exclusive strip meter level, default `true` (R2: a plan with no host policy meters every block, so every direct-render test is unchanged), with `set_strip_meter_level`. `PreparedPlanExecutor::set_strip_meter_level` (default no-op) receives it once per block, through `begin_block` (D4). The owner keeps it, so an adopted successor inherits it.
- **graph:** `GraphNodeObserverBinding` records `meter` (`carry_kind() == Some(ObserverKind::Meter)`) at bind; `UnitObservation` is now one byte of bits with a `METERS_ONLY` bit (the `UnitIdentityWithoutFlags` size assertion holds). With the level off, a meters-only unit returns at once, the bank sample-peak and full meter passes do not run, and `observe` / `observe_output` skip meter bindings; taps and spectrum observe unchanged; `block_end_observers` is untouched.
- **control-plane:** `PlanState::render` sets the level per block from `OutputPeakObserver::admits(first_sample)` (the output gate's own rule), so strip and output meters observe the same blocks.
- **builtins, host-web:** no change.

## Decision record

- **R1 (root).** With the level on from boot, every published value is bit-identical (class A: no arithmetic change, fewer passes). After a level transition the first frame's `loss_count` no longer counts the off period's stale groups and drops, which is the correct count (decision 15). In the event no host-web test needed a change: every browser meter test passed unchanged.
- **R2 (root).** Default `true`; the control plane decides per block. A field on `RenderIo`/`RenderTime` and a session-level gate were rejected.
- **D1 (attempt 1, implementer): the restart block.** The scope's one rule (`admits(first_sample)` before the block) disagreed with the output meter on one case, found by the existing `an_uncarried_swap_restarts_the_output_meter_at_its_adoption_block`: a lease waiting for its activation when a swap that carries no strip meter adopts. `MeterDelivery::adopt` then moves the gate to the adoption block after render and before the output meter observes it, so the output meter observed the adoption block and the strip meters did not; their windows started one block later and never paired with the output windows again. Fix, in the authorized paths: the owner keeps a second level, `set_strip_meter_level_at_restart`, which it uses for a block that applies a swap whose successor carried no strip meter (`carry_from` records whether its `observer_carries` hold an `ObserverKind::Meter`, from the one read it already made); `PlanState::render` sets it from `OutputPeakObserver::admits_restart_block()` (`gate != INACTIVE`, the rule `adopt` applies). One extra load per block on the control-plane side.
- **D4: no new dynamic call on the render path.** A first build added two `call_indirect` sites (`render_inner` calling `set_strip_meter_level`, `render_active` reading `observer_carries`), which `check-web-audioworklet.sh`'s pinned `INDIRECT_SITES` refused. The level now reaches the executor through `PreparedPlanExecutor::begin_block(snapshot, strip_meters)`, one dispatch whose default calls `set_strip_meter_level` and `begin_live_block` statically, and the carried-meter bit is a plain field, so the pinned table is unchanged.
- **D2.** A meter that selects `HELD_PEAK` keeps every block it observed in its held peak, so after an off period its held peak restarts from the resumed block. No production meter selects it (every strip meter is `SAMPLE_PEAK`); gate 1's full meter selects everything else.
- **D3.** The C ABI's meter-carry test (`a_carried_strip_meter_keeps_its_reader_across_a_c_abi_rebuild`) now activates the output gate at 0: until #1583 wires `TELEMETRY_CONFIGURE` to the gate, C ABI strip meters stay dark, which is the product rule (nothing reads them before #1587).
- **Before number:** #1581 D7 measured the strip meter work at +6.3 us/block (+6.5 %) on the 64-strip console session. No committed timing test.

## Gates (attempt 1)

1. host-core `strip_meters.rs` `strip_meters_do_no_work_with_the_level_off_and_resume_with_the_always_on_words`: with the level off, the bank peak and full meter passes and the block peak merges do not move and no snapshot is queued; from block N on, every word equals the always-on run's (sequence counted from block N); without and with console slots.
2. control-plane `plan.rs` `the_strip_meter_level_follows_the_output_meter_gate_per_block`: the executor's level equals the gate's answer per block.
3. graph `runtime.rs` `strip_meter_level_off_skips_meter_bindings_and_keeps_every_other_observer`: a tap beside a meter still observes; a meters-only unit does not walk.
4. Existing: the host-web, capi and control-plane tests pass (the restart case is the existing host-web swap test); realtime-policy; fmt, clippy, aarch64 check, lint checks; `check-web-audioworklet.sh`.

The original body follows for its evidence; its references to the controlled policy are superseded.

<!-- original title: Bind web console meters through the controlled observer policy -->

## Product outcome

The browser host boots one `SAMPLE_PEAK` meter per track at `PostMatrix` and binds them with the permanent policy, so every meter loop runs every block whether or not the SDK holds the meter lease, and every snapshot is dropped unread when it does not. Bind them controlled and drive activation from the lease, so an unmetered block does no meter work. Class A: when the lease is held, every published value is unchanged.

## Root evidence

- `hosts/host-web/src/lib.rs` near line 7734 requests `MeterMetricSet::SAMPLE_PEAK` for every track; the `meter_lease` flag (struct field near line 2509) gates only the master-peak scan and `poll_meters`, not the per-track observers.
- `crates/host-core/src/prepare.rs` near lines 1085-1180 routes the web path to `prepare_selected_session_builtins_between_render_calls`, which reaches `crates/builtins-compiler/src/lib.rs:3582` (`MeterBindingPolicy::Permanent` -> `GraphNodeObserverBinding::new`). The controlled policy exists at `:3587` (`GraphNodeObserverBinding::controlled`) and was delivered by #816/#818/#820 for the native selected-meter owner; the web host never adopted it.
- The controlled dispatch (`observe_active_unit`, `crates/graph/src/runtime.rs` near line 1848) visits only active entries; the permanent dispatch (`observe_unit`, `:1915`) walks every member every block.

## Smallest closable slice

Authorized paths: `hosts/host-web/src/lib.rs`, `hosts/host-web/src/observation_ingress.rs` (only if activation plumbing lives there), `crates/host-core/src/prepare.rs` (only the policy selection for the web route), `hosts/host-web/web/miso-engine-v1-audio-worklet.js` and `-host.js` (only if the lease must send an activation), their tests, and this spec.

Select `MeterBindingPolicy::Controlled` for the web console meters and arm/disarm their activation when the meter lease is acquired/released, aligned to the existing `meter_activation_sample` boundary. Do not change the meter arithmetic, snapshot shape, cadence or the SDK surface.

## Non-goals

No change to the meter kernel (separate issue), to native hosts, to the master-peak scan, or to the SDK API.

## Objective gates

1. New Rust test in `hosts/host-web`: with the lease not held, a rendered block invokes no meter observer (assert through the observation activation snapshot or an observer call counter) and publishes no snapshot.
2. New Rust test: with the lease held, the sequence of published snapshots for a fixed input is bit-identical to the permanent binding on the same input (run both policies on the same plan and compare every field).
3. `bash scripts/test-web-audioworklet.sh` passes; `scripts/check-host-core-policy.sh` and `tools/realtime-policy` pass.
4. Browser qualification `npm run qualify -- --artifacts <dir> --sdk-root sdk --browser all --check-matrix --self-test-mutations` passes against the rebuilt artifact (rebuild with `scripts/build-web-audioworklet.sh`, then repin per its REPIN mode and re-record the matrix in a separate checkpoint, as the qualification workflow requires).

## Dependencies

None. Pairs with "Keep the route fold eligible when the folded lane is observed".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
