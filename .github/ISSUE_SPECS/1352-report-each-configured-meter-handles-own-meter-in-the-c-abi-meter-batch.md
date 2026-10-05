# Report each configured meter handle's own meter in the C ABI meter batch

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0: the no-shortcuts principle).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The C ABI documents the meters it serves, refuses a meter handle it does not serve, and its
`METER_BATCH` events (`0x8020`) report each configured handle's own measurement: the plan
output's left and right peaks over each configured window of `meter_period_blocks` rendered
blocks, with honest flags.

## Context

- **The defect.** `collect_render_activity` stages, for every configured meter handle, one
  record `{ handle, component: Left, flags: 1, value: peak }`, where `peak` is one maximum over
  both output planes of the last rendered block (`crates/capi/src/runtime/control.rs:537-557`).
  Every handle, whatever it names, gets the master peak labelled as the left channel, and
  `meter_period_blocks` is checked only for being nonzero
  (`crates/protocol/src/controller.rs:2445-2450`).
- **No catalog.** The C ABI provider has no meter-handle table, and nothing validates the handles
  a host configures (`TelemetryConfigure` arm, `controller.rs:3059-3075`;
  `SessionControlProvider::telemetry_configure`, `crates/host-core/src/control_provider.rs:440-458`).
  The protocol defines handles as revision-scoped and the meter components left, right and
  aggregate, flag bits valid, clipped and held (`docs/CONTROL_PROTOCOL_REGISTRY.md:13`,
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:29`).
- **The measurement.** After a successful render, capi scans both planes for one peak when
  `render_peak_observed` is set (`crates/capi/src/ffi.rs:886-912`) and publishes it through
  `SharedPlanState::render_peak_bits` (`crates/capi/src/runtime/plan.rs:5-27`, `:234-243`; the
  control plane's after #1309).

## Decisions frozen for this slice

- **D1. One served meter.** Meter handle `1` is the plan output. The header documents it and that
  every other handle is refused. Configuring any other handle is refused with `NOT_FOUND` through
  the `Result` that #1351 gives `telemetry_configure`; the stored configuration does not change.
- **D2. Per-plane peaks.** The scan publishes the left plane's and the right plane's peak
  separately (two words in place of `render_peak_bits`). It stays gated by `render_peak_observed`
  and allocation-free.
- **D3. Windows.** The control thread publishes `meter_period_blocks` beside the gate. Render folds
  the per-plane maxima over that many blocks in plan-local state and, when a window closes,
  publishes its two peaks, the window's end sample and a window sequence. A window open across a
  plan swap restarts at the swap block.
- **D4. The batch.** For each closed window the control thread has not staged, it stages one
  batch: `{1, Left, flags, left}` and `{1, Right, flags, right}`. `flags` has the valid bit, and
  the clipped bit when that peak exceeds `1.0`. The held bit is never set. A window replaced by a
  newer one before the control thread read it is counted into `TELEMETRY_COALESCED` (the window
  sequence gap). The aggregate component is not emitted.
- **D5. The acked-batch question** does not arise: meter batches are lossy telemetry by contract,
  and D4 counts what it coalesces.

## Deliverables

1. D1 in host-core's provider; D2 in capi's render entry; D3 in the control plane's render-side
   plan state; D4 in `collect_render_activity`.
2. Header text (`crates/capi/include/miso_engine_v1.h`) and `docs/C_ABI_V1_QUALIFICATION.md`.

## Authorized paths

- `crates/capi/src/ffi.rs` (the peak scan only), `crates/capi/include/miso_engine_v1.h` (prose),
  capi tests.
- `crates/control-plane/src/plan.rs`, `crates/control-plane/src/control.rs`
  (`collect_render_activity` and the gate only).
- `crates/host-core/src/control_provider.rs` (the meter check in `telemetry_configure`).
- `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- Strip, send or effect meters on the C ABI (a later issue would add handles to D1's table).
- Browser meters, which have their own path.

## Objective gates

1. **Own measurement (new capi test).** A session whose left output is a sine at 0.5 and right at
   0.25: configure handle 1, period 4. After 4 rendered blocks and a control call, the batch holds
   Left 0.5 and Right 0.25 (to the sine's sampled peak, compared by bits against the same scan run
   in the test), flags valid only.
2. **Clipped (same file).** An output peak of 1.5 sets the clipped bit on that plane only.
3. **Period (same file).** With period 4, three blocks and a call stage nothing.
4. **Refusal (same file).** Configuring handle 2 answers `NOT_FOUND` and leaves the configuration
   unchanged.
5. **Superseded oracle bytes.** `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
   (`crates/capi/src/runtime/tests.rs:1857`) pins a meter batch with one left-labelled record;
   re-pin those wire bytes with this issue as the reason.
6. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   (allocations, deallocations, locks, syscalls and `total_violations` 0);
   `bash scripts/check-realtime-policy.sh`.
7. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p control-plane --features
   test-support`; `cargo test --locked -p host-core --features control-provider,test-support`;
   `bash scripts/check-capi-abi.sh`; `cargo fmt --all -- --check`; `cargo clippy --locked
   --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a batch that reports one peak for both planes or labels it wrongly (today's defect).
- Gate 2: a flag word that never reports clipping.
- Gate 3: a cadence that ignores the configured period.
- Gate 4: a handle accepted that names nothing.

## Dependencies

- *Report each configured counter's own value in the C ABI counter snapshot* (#1351), which adds
  the `telemetry_configure` refusal path.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
