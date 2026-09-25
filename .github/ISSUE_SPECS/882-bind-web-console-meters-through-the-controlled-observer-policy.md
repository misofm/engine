# Bind web console meters through the controlled observer policy

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
3. `bash scripts/test-web-audioworklet.sh` passes; `scripts/check-host-core-policy.sh` and `scripts/check-realtime-policy.sh` pass.
4. Browser qualification `npm run qualify -- --artifacts <dir> --sdk-root sdk --browser all --check-matrix --self-test-mutations` passes against the rebuilt artifact (rebuild with `scripts/build-web-audioworklet.sh`, then repin per its REPIN mode and re-record the matrix in a separate checkpoint, as the qualification workflow requires).

## Dependencies

None. Pairs with "Keep the route fold eligible when the folded lane is observed".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
