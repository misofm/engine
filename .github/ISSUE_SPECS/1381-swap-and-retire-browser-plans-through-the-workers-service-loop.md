# Swap and retire browser plans through the Worker's service loop

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This slice follows *Run the browser control plane in a Worker and keep the AudioWorklet
render-only* (#1332).

## Product outcome

The browser engine holds its plan the way the C ABI does.
- The Worker owns the control plane's session state (`control_plane::SessionState`: the committed
  model at revision 0, the plan publisher and the retirer).
- The worklet renders through `RealtimePlanOwner`, so a published successor is adopted at a block
  boundary with the clock continued.
- The Worker runs the control plane's service step continuously, so every retired plan is
  reclaimed in the Worker and never on the audio thread.

Rendered bits do not change. This is the substrate that *Replace the running browser session in
the Rust host* (#1290) publishes through.

## Context

- **The browser renders the plan directly.** `render_next` calls `ready.host.plan.render`
  (`hosts/host-web/src/lib.rs:3212`, `:3234-3239`). There is no publisher, no candidate and no
  retirement.
- **The exchange.**
  - `plan_exchange` returns publisher, owner and retirer
    (`crates/engine/src/realtime/plan_exchange.rs:194`).
  - `RealtimePlanOwner::enter_block` (`:375`) adopts a candidate, continues the clock, runs
    `carry_from`, and commits the old plan to the retirement queue.
  - `PlanRetirer::try_reclaim` (`:512`) is the only way a retired plan leaves that queue.
- **The control plane is capi's today.** *Extract the C ABI control plane into a portable crate
  both hosts call* (#1309) moves it to `crates/control-plane` (lib `control_plane`):
  - `SessionState` (today `crates/capi/src/runtime/control.rs:237`) owns the controller,
    provider epochs, publisher, retirer and render diagnostics;
  - `PlanState` (today `crates/capi/src/runtime/plan.rs:53`) wraps `RealtimePlanOwner` and
    `SharedPlanState` for render.
- **Preparation is fixed to the C ABI inside it.**
  - `prepare_runtime` (today `compile.rs:572`) prepares with `C_ABI_LIVE_LANES` and
    `LIVE_QUEUE_DEPTH` (`compile.rs:12-18`, `:586-600`).
  - The session constructor builds the exchange with capacity 1 (`:755-761`).
  - The browser prepares differently: it adds spectrum capture and meters, and selects its own
    live lanes (`compile_ready`, `hosts/host-web/src/lib.rs:6617`, `:6644-6681`).
- **The render-activity code.** #1309 D7 keeps `RENDER_DIAGNOSTIC_CODE` as
  `capi.render.activity`, because `host-core` sizes the slot by that literal
  (`crates/host-core/src/control_provider.rs:117`). It leaves the browser's choice to this
  stream.
- **The service step.** *Add miso_engine_v1_service for bounded control work between edits*
  (#1348) defines the crate's bounded service step: reclaim retired plans, synchronize epochs,
  stage telemetry.

## Decisions frozen for this slice

- **D1. Adapter preparation.** The crate gains one trait the adapter supplies,
  `RuntimePreparer`. It has `prepare(compiled, limits, successor: Option<SuccessorBase>) ->
  Result<PreparedRuntime, CompileFailure>`.
  - capi's implementation is today's `prepare_runtime`, unchanged.
  - The session constructor takes `&dyn RuntimePreparer` and calls nothing else to prepare.
  - The browser's implementation runs `compile_ready`'s three branches with the concurrent
    preparation variants. It never uses `_between_render_calls`, because the producers live in
    the Worker while render runs on the worklet thread.
- **D2. Boot.**
  1. The Worker's boot builds `control_plane::SessionState` from the boot document through the
     browser preparer. The committed model is at revision 0.
  2. It moves the render half (the host's render-side state with `control_plane::PlanState`)
     into #1332's transfer token.
  3. Browser-only boot steps run as today, in the same order: the document budgets, the VCA
     bound, the bridge buffers.
- **D3. Render.** `render_next` renders through `PlanState`'s `RealtimePlanOwner`. A block whose
  `enter_block` applied a successor rebinds the render-side companions of the adopted epoch
  (meter, observation and spectrum bindings) before it renders. That rebind is allocation-free:
  the companions travel with the published candidate.
- **D4. Service loop.**
  - The Worker calls the crate's service step (#1348) after every message it handles. It also
    calls it on a repeating timer of one quantum's duration. The browser may clamp the timer;
    correctness does not depend on the period, because deadlines are counted in render samples
    (D15-17).
  - The loop starts when boot succeeds and stops before dispose.
  - In `single` mode, the worklet calls the same service step after `process()` has rendered a
    block in which `enter_block` applied a successor. It runs outside the render-locked window
    and never inside the render export.
- **D5. Render-activity code.** The browser uses the crate's `capi.render.activity` code
  unchanged. Renaming it would change a protocol-visible code on both hosts and the literal that
  host-core sizes by. That is not this slice's to change.

## Deliverables

1. D1 in `crates/control-plane` (the trait and the constructor parameter) and in capi (its
   implementation).
2. D2-D4 in `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs`, and the control Worker and
   worklet scripts.
3. A test-support hook in host-web, `republish_committed_for_test()`. It prepares a successor of
   the committed model through the browser preparer and publishes it. It exists only for gates 2
   and 3.
4. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/Cargo.toml` (the
  `control-plane` dependency, and the `bench-support` dev-dependency), `hosts/host-web/web/`
- These are outside stream H's ownership; root sequences them after #1309 and #1348:
  - `crates/control-plane/src/`, the trait and constructor parameter only;
  - `crates/capi/src/runtime/`, capi's `RuntimePreparer` implementation only.

## Non-goals

- No structural edit (#1290). No live edit in the Worker (*Admit browser live edits in the Worker
  through the committed model*, #1382).
- No catch-up (*Run the browser catch-up in the Worker's service loop*, #1361).
- No watermark in the browser status (*Publish the applied-revision watermark in the browser
  status*, #1349).

## Objective gates

1. **Same bits.** The browser legs' native-digest gate and the `direct-oracle.mjs` parity pass
   unchanged in Chromium, Firefox and WebKit, on the isolated leg and the non-isolated leg.
2. **Swap and reclaim, two threads.** Native host-web test.
   - The control half runs on thread A, the render half on thread B. Boot nine tracks and render
     6 blocks.
   - On A, call `republish_committed_for_test()`. Render 6 more blocks on B.
   - The block after publication reports `SwapOutcome::Applied`. The clock continues. Every block
     is bit-identical to a run without the republish.
   - On B, `bench_support::alloc` counts `allocations == 0 && frees == 0` around every render
     call.
   - The next service step on A reclaims exactly one plan.
3. **Single mode.** Gate 2 on one thread gives the same blocks. The retired plan is reclaimed by
   the service step that follows the swap block.
4. **C ABI unchanged.** `cargo test --locked -p capi` passes. `target/release/audit capi` reports
   0 allocations, locks and syscalls, with the same `pcm_digest` as the base (PR evidence).
5. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `bash scripts/check-cross-targets.sh`, `bash scripts/check-host-core-policy.sh`
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 2: turns red if the browser still renders the plan directly (no adoption), if the clock
  restarts on adoption, if the render thread drops the retired plan, or if the Worker never
  reclaims it.
- Gate 3: turns red if single mode leaks retired plans, or reclaims them inside the render
  export.
- Gate 1 and gate 4 are not new. They hold the bits of both hosts across the move.

## Dependencies

- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
