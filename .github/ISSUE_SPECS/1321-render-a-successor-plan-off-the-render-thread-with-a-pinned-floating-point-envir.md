# Render a successor plan off the render thread with a pinned floating-point environment

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 step 3, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A control thread can advance a prepared plan by whole blocks, in bounded slices, and get the
same bits the render thread would produce, whatever floating-point control word that thread
runs with. This is the executor for the warm-successor catch-up (*Pre-roll a successor whose
latency grows*, #1287): C ABI hosts run it from `miso_engine_v1_service`, and the browser runs it
in its Worker.

## Context

- Every native render entry pins the canonical FP environment for the whole call:
  - `StartedRenderSession::render_planar` (`crates/host-core/src/render_session.rs:103-115`, guard
    at `:111`), after `StartedRenderSession::start` attests the environment (`:80-91`);
  - the C ABI's `miso_engine_v1_render_f32_planar` (`crates/capi/src/ffi.rs:817`).
- The guard is `lane::CanonicalFpEnv` (`crates/lane/src/fpenv.rs:288`, `enter` at `:326`). It is
  neither `Send` nor `Sync`, and it restores the caller's word on drop, unwinding included. On
  Wasm the FP behaviour is fixed, so the guard is empty (`:370`).
- `crates/engine` does not depend on `lane` (`crates/engine/Cargo.toml`), so the pin cannot live
  in `PreparedRenderPlan`. `host-core` already depends on both.
- `PreparedRenderPlan::render_contiguous` (`crates/engine/src/realtime/plan.rs:870`) enforces the
  plan's own clock. `render` (`:852`) runs the executor inside
  `engine::realtime::audit::in_render_scope` (`crates/engine/src/realtime/audit.rs:176`) on
  whichever thread calls it.
- `crates/host-core/tests/fp_environment.rs` shows the pattern a pin test needs: a subnormal
  parametric-EQ fixture (`fixtures/session/v1/parametric-eq-nine-track.json`), a guarded arm, and
  an unguarded control arm that must differ.

## Decisions frozen for this slice

- **D1. Home.** A new module, `crates/host-core/src/catch_up.rs`, exported from
  `crates/host-core/src/lib.rs` with a row in its module table.
- **D2. Type.** `OffThreadPlanRenderer` owns one `PreparedRenderPlan` and one discard output
  buffer: `output_channels * quantum` `f32`s, allocated in the constructor. It is `Send`.
  - `OffThreadPlanRenderer::new(plan) -> Result<Self, (PreparedRenderPlan, FpEnvironmentRejection)>`
    attests the environment with `lane::attest_fp_environment`, the way
    `StartedRenderSession::start` does, and returns the plan on refusal.
  - `into_plan(self) -> PreparedRenderPlan` gives the plan back for publication. No allocation and
    no free.
  - `next_absolute_sample(&self) -> u64` forwards the plan's clock.
- **D3. Bounded slice.**
  `render_slice(&mut self, until_sample: u64, max_blocks: u32) -> Result<SliceReport, RenderError>`.
  - It enters `CanonicalFpEnv` once for the slice.
  - Then it renders contiguous blocks with `render_contiguous` into the discard buffer, while the
    plan's next sample plus one quantum is at or below `until_sample` and fewer than `max_blocks`
    blocks have run.
  - `SliceReport { blocks: u32, next_absolute_sample: u64 }`.
  - A render error stops the slice and is returned. The error is sticky, as on the render path.
  - The caller's control word is restored on every exit.
- **D4. Realtime rules.** A slice runs render code, so it follows render's rules: no allocation,
  free, lock, log or I/O inside it. It is marked `REALTIME_POLICY_BEGIN`/`END`. Each block is in a
  render scope already through `render`.
- **D5. Not a render entry.** It never touches the realtime plan owner. It never runs on the render
  thread. It does not advance the host's clock. The output is discarded. Who calls it, and how
  much work goes into one service call, belong to *Catch up a returned successor and adopt it
  exactly at a scheduled sample* (#1355) and *Run the C ABI catch-up from miso_engine_v1_service
  and report its outcome* (#1360).

## Deliverables

1. D1-D5.
2. `crates/host-core/tests/off_thread_render.rs` with gates 1-4.

## Authorized paths

- `crates/host-core/src/catch_up.rs` (new), `crates/host-core/src/lib.rs`
- `crates/host-core/tests/off_thread_render.rs` (new)

## Non-goals

- No peek binding, no catch-up policy, no publication and no adoption (#1355).
- No browser wiring (*Run the browser catch-up in the Worker's service loop*, #1361).
- No change to `StartedRenderSession` or the C ABI render entry.

## Objective gates

1. **The pin holds off-thread.** Use the subnormal fixture of `tests/fp_environment.rs`. Prepare
   three identical plans.
   - *Reference:* render 16 blocks with the canonical word on the test thread, then 8 more through
     `StartedRenderSession`.
   - *Guarded:* on a spawned thread that sets FTZ+DAZ (FZ on AArch64), advance 16 blocks with
     `render_slice`. Move the plan back and render 8 more through `StartedRenderSession`.
   - *Unguarded control:* the same as guarded, but the 16 blocks use `render_contiguous` directly.

   The guarded arm's 8 blocks are bit-identical to the reference's, and the control arm's differ.
   On Wasm the test is not built, because the guard is empty there.
2. **Slice bounds.** On a 128-frame plan at sample 0:
   - `render_slice(1_000, 100)` renders 7 blocks and reports next sample 896;
   - `render_slice(10_000, 3)` then renders 3 blocks and reports 1,280;
   - `render_slice(1_280, 5)` then renders nothing.
3. **Word restored.** After `render_slice` on a thread with a non-canonical word,
   `lane::read_fp_control_word()` equals the word set before the call.
4. **Realtime.** After one warm-up slice, a 32-block `render_slice` makes zero allocations and
   frees on its thread. Measure with `bench_support::alloc::current_thread_delta_since`.
5. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support --test off_thread_render`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: if the `CanonicalFpEnv::enter()` in `render_slice` is deleted, or moved inside a branch
  that does not cover every block, the guarded arm equals the control arm and differs from the
  reference. Red. No existing test renders off the render entry points.
- Gate 2: an off-by-one in the `until_sample` comparison renders a block past the caller's
  target, which would make a catch-up overshoot its adoption sample. Red.
- Gate 3: a slice that installs the canonical word without the restoring guard (a bare
  `write_fp_control_word`) leaves the service thread in the canonical word. Red.
- Gate 4: a discard buffer allocated per slice, or a `Vec` report, counts an allocation. Red.

## Dependencies

- *Pre-roll a successor whose latency grows* (#1287), its first slice: the recorded proof confirms
  that a catch-up renders whole successor blocks off the render thread.
