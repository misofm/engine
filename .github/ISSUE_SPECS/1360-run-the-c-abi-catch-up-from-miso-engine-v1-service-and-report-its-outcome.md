# Run the C ABI catch-up from miso_engine_v1_service and report its outcome

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A mobile host that adds a latent effect during playback gets a seamless swap through the C ABI.
- `SESSION_TRANSACTION_APPLY` returns at once with path `rebuild`.
- The host keeps rendering on its audio thread and calls `miso_engine_v1_service` from any
  control thread.
- The watermark shows the revision completing `EXACT` (or with a counted fallback) at the adoption
  sample. Nothing in submit waits for render.

## Context

- *Add miso_engine_v1_service for bounded control work between edits* (#1348):
  - `SessionState::service` in the control-plane crate (D1);
  - every control call services first (D2);
  - one call does bounded work (D3);
  - the entry point takes the session handle (D4);
  - its duty text (D5).
- After *Extract the C ABI control plane into a portable crate both hosts call* (#1309), the
  structural path of `crates/capi/src/runtime/control.rs` (reservation at `:962`) lives in
  `crates/control-plane`.
- The catch-up and its outcomes are host-core library code from #1354-#1359. The watermark and its
  flags are #1314's.
- The release audit runs `./target/release/audit capi` (`.github/workflows/qualification.yml:715`)
  from `tools/audit/src/capi.rs`.

## Decisions frozen for this slice

- **D1. Classify.** The control plane's rebuild path calls `host_core::warm_lead` (#1354 D3), the
  only computation of the lead `P`.
  - With `P > 0` and an off-thread host, the candidate is prepared warm (#1354 D3) and published
    `CopyAndReturn`; when it also restarts strips, it is composed with their duck as *Duck-swap
    the strips a latency growth restarts, and fall back to the transition when no catch-up can
    finish* (#1397) D1 states.
  - If warm preparation returns `WarmUnavailable` (#1354 D3, #1355 D7), the candidate is prepared
    as #1397's transition (D2) at once. A valid edit is never refused for it.
  - With no growth, it is published as today.

  The transaction response is `rebuild` in every case (*Report each transaction's edit path in its
  response*, #1313).
- **D2. Service step.** `SessionState::service` gains one step after `synchronize_plan_epochs`,
  `CatchUp::service(CATCH_UP_SLICE_BLOCKS)`. That step covers:
  1. taking the catch-up's own candidate back when its cell is `Returned`, with `withdraw()`
     (`Withdrawal::Returned { reason }`, #1311 D6; #1354's `Copied` or `CopyRefused`, `Late`, or
     #1358's `PreRollBound`). There is no return queue, and `synchronize_plan_epochs` drains
     nothing and never takes a returned candidate (#1348 D7). A `Withdrawal::InFlight` (#1311 D6,
     #1354 D1) is retried at the next call; nothing waits on render. When a structural edit
     superseded an in-flight catch-up (*Supersede a running catch-up by a structural edit*, #1357
     D1), this item completes that withdrawal once the copy has ended and publishes the held newer
     candidate (#1357 D1a);
  2. one bounded slice of rendering;
  3. the deadline check (#1358 D3);
  4. publication (#1355 D5) or a fallback publication (#1358).

  `CATCH_UP_SLICE_BLOCKS` is a host-core constant with the value #1286 D3 item 6 derives; its comment names the record row and
  this spec restates no formula. #1348 D3's bound is restated to include it.
- **D3. Threads.** The catch-up renders on whichever thread calls a session function, and the
  header says so. `CanonicalFpEnv` (#1321) protects that thread's control word.
- **D4. Header and qualification doc.**
  - `miso_engine_v1.h` beside the live-edit paragraph, and `docs/C_ABI_V1_QUALIFICATION.md`: a
    latency-growing edit is completed by service calls; its outcome flags; a paused host stays
    pending with no fallback; a stop declaration turns it into a plain rebuild (#1359).
  - #1348 D5's sentence about held edits cites *Hold live edits during a catch-up and apply them at
    the adoption sample* (#1356).
  - No new symbol and no new feature bit.
- **D5. Audit.** `audit capi` gains one leg. During playback it adds a muted track with a
  true-peak limiter, services from a second thread until the watermark covers the revision, and
  checks `EXACT`, zero render allocations, and output continuity against the reference built the
  way #1355 gate 1 builds it.
- **D6. Acked-batch question.** Submit's ack follows every fallible step (D15-17). Every later
  path completes the revision through the watermark (#1355-#1359). An ack can never precede a drop.

## Deliverables

1. D1-D3 in `crates/control-plane/src/`.
2. D4 in `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md`.
3. D5 in `tools/audit/src/capi.rs`.
4. C ABI tests in `crates/capi/tests/latency_growth.rs` (new).

## Authorized paths

- `crates/control-plane/src/`, `crates/capi/src/`, `crates/capi/include/miso_engine_v1.h`
- `crates/capi/tests/latency_growth.rs` (new), `tools/audit/src/capi.rs`
- `docs/C_ABI_V1_QUALIFICATION.md`, `crates/host-core/src/catch_up.rs` (the slice constant)

## Non-goals

- The browser (#1361). New ABI symbols.

## Objective gates

1. **Exact through the ABI.** A render thread renders 128-frame blocks at 48 kHz through
   `miso_engine_v1_render_f32_planar`. A control thread applies the growth transaction and then
   calls `miso_engine_v1_service` in a loop. The rendered stream equals the reference, and the
   watermark reaches the revision with `EXACT`.
2. **No service, no progress, no fallback.** With no control calls, the revision stays pending,
   and render continues the predecessor without a gap. The first service call after the deadline
   in render samples publishes `PREROLL_FALLBACK` or `TRANSITION_FALLBACK`, which the watermark
   reports.
3. **Bounded call.** One `service` call renders at most `CATCH_UP_SLICE_BLOCKS` blocks.
4. **Audit.** `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   passes with the new leg.
5. Commands:
   - `cargo test --locked -p capi`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a service step that never publishes, or a classify that publishes `Next`, gives a gap or
  a pending revision. Red.
- Gate 2: a wall-clock deadline, or catch-up work done inside submit, changes when the revision
  completes. Red.
- Gate 3: a service loop that renders until caught up exceeds its bound. Red.

## Dependencies

- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Turn a pending catch-up into a plain rebuild at a host-declared stop* (#1359), and through it
  #1354-#1357.
- *Fall back from a missed catch-up deadline: bounded render-thread pre-roll, then the transition*
  (#1358): the deadline check and the pre-roll publication D2 calls.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when no
  catch-up can finish* (#1397): D1's composition and transition.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Record the swap block's cost on the 64-track console* (#1286).
