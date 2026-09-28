# Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live console lanes

Source: `docs/handoffs/live-control-2026-09-28/FINDINGS.md` section 7, verified in `VERIFY.md`. **The Amendments subsection supersedes the draft wherever they conflict.** Owner ruling (2026-09-28): approved; the C ABI adapter moves onto the core's live-control lanes. Depends on #1042 (land first). Ramp lengths come from the session's `controlSmoothing` settings (separate issue); until that lands, use the per-change smoothing the command carries, else 0.

### Title

Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live console lanes

### Context

- A fan's fader, mute or pan change through `miso_engine_v1_submit_command` is a `SessionTransactionApply`.
  Today it always replaces the plan, which resets source rings and restarts all DSP state
  (`capi/src/runtime/control.rs:46-53`, `:711`).
- The engine already has live lanes for exactly these values, used by the browser
  (`HostConsoleRequest::control_queue_depth`, `TrackControlProducer`).
- capi does not request them (`compile.rs:410`).
- Findings: `docs/handoffs/live-control-2026-09-28/FINDINGS.md`.

### Scope

1. **Bound the drains.** Bound the four fader and matrix drains by `available_at_entry()`
   (`builtins-compiler/src/lib.rs:958`, `:994`, `:4257`, `:4320`). This is its own commit.
2. **Attach the console in capi.** Prepare with `Concurrent` delivery and keep `track_controls` in
   `ProviderEpoch`. Effect producers stay idle in this slice.
3. **Add the live delta.** Add `live_builtin_delta` behind host-core's `control-provider` feature,
   so the browser build does not compile it.
4. **Commit live deltas without a new plan.** In `command()`'s `Structural` arm: when the delta is
   `Some`, run `compiled_model_admission`, check room, push to the newest epoch, then commit the
   prepared token. Otherwise run the existing path unchanged.
5. **Charge the resources.** Charge the console producer table and queue rows in capi's resource
   report, and update the `resource_lifecycle` oracles.
6. **Document the behaviour.** Update the C header comment, `docs/C_ABI_V1_QUALIFICATION.md` and
   `docs/CONTROL_PROTOCOL_SEMANTICS.md`. Value-only transactions apply at each stage's next block
   and are never lost. Structural transactions replace the plan.

### Decisions to freeze before coding

- **D1. Classification.** Classify by committed-model delta. A delta is live when `fader` and
  `matrix_or_pan` are the only fields that differ and every value passes the render-side setter's
  domain. Anything else is structural. Edit opcodes are not inspected.
- **D2. Timing contract.** A live edit is admitted all or nothing and applied at the next drain of
  each destination stage (at most one block of skew). It is never lost. No block-atomic claim
  (#444) is made.
- **D3. Fader and mute ramp.** Options: 0 (step at the block boundary; the browser SDK default,
  bit-exact to a re-prepared plan), or a fixed ramp such as one quantum. The owner rules.
- **D4. Queue depth.** Use a fixed per-track depth (the prototype used 16). A full queue is typed
  `Backpressure` before commit. No compile-limit ABI field is added.

### Objective gates

1. **PCM through the C ABI.** Through the exported entry points, a `SetTrackFader` or
   `SetTrackMatrixOrPan` transaction changes the next block of the *same* plan: no pending
   provider, and no re-seek or resubmission. A mute-and-unmute round trip is bit-identical to a
   console-free reference plan fed from the same ring. Check at 1 and 10 tracks and at the four
   launch rates.
2. **Persistence.** The snapshot and the revision carry the value, and `SESSION_COMMITTED` is
   emitted. A later structural replacement starts at the committed value, checked against a control
   run without the edit.
3. **No ack before a drop.**
   - A full queue gives `Backpressure` with the model, revision and replay unchanged.
   - Fault injection between push and commit leaves no ack and no committed revision.
   - A live edit while a candidate is pending lands in the candidate.
4. **Realtime.** A two-thread barrier test (the shape of
   `barrier_schedule_separates_one_source_producer_from_exclusive_render`) submits live edits while
   render runs. It shows zero allocations and frees on render (the `resource_lifecycle` allocator)
   and a final state equal to the last committed model.
5. **Unchanged behaviour.**
   - Every existing structural capi test passes unchanged.
   - Console digests are byte-identical: `gain_pan_profile digests` and the host-web console tests.
   - The shipped artifact's hash moves only because of the bounded drains; record the move.
6. **Builds.**
   - Native `--all-targets --all-features`, `clippy -D warnings`, and the wasm `simd128` check pass.
   - AArch64 passes through #1017's CI once that exists; until then, a compile check on the targets.

### Out of scope

These are successors, each a separate issue:

- **L2:** effect parameters and bypass (`UpsertEffectParam`, `SetEffectBypass`) through
  `EffectControlProducer`, with provider readback.
- **L3:** EQ prepared targets (`EqTargetPreparer`), and input trim and polarity.
- Solo stays an app-side composition of mutes in one transaction.
- The block-atomic batch claim (#444) is added only if a product asks for it.
- Sample-accurate transient automation.

### Dependencies

None that block it. Draft 02 and its step 4 are independent and can land first.

### Amendments (Sol verification, 2026-09-28)

See `VERIFY.md` in this folder. These amendments supersede the text above where they conflict.

**A0. New predecessor, L0: "Make capi's plan replacement safe against a concurrent control call"
(F1, pre-existing; L1 depends on it).**

- **The bug.** `RealtimePlanOwner::enter_block` retires the old plan at the start of a render
  call. capi publishes `active_epoch` only after that call returns (`capi/src/runtime/plan.rs:215-216`).
  In that window, any `synchronize_plan_epochs` (submit, seek, command, event) returns `INTERNAL`.
  It then leaves a stale retired provider and a stale report row behind, so every later structural
  command returns `BACKPRESSURE`.
- **The fix, about 25 lines in `synchronize_plan_epochs`.**
  - A reclaimed epoch equal to `providers.epoch` proves that the one pending candidate is active,
    so promote it there.
  - Then keep only the report rows that the published epoch, the current epoch or a pending epoch
    can still read.
- **Gates.**
  - A deterministic split-render test: render through `RealtimePlanOwner` without publishing, make
    a control call, then publish. After that, three more structural commands must succeed.
  - A two-thread test that races live or structural edits against render across a swap, with zero
    `INTERNAL` in 20 runs. The unmodified base fails 14 of 20.

**A1. Scope changes.**

1. **Bound the drains (its own commit).** Additionally:
   - wrap the four drains in `REALTIME_POLICY` regions;
   - add a `check-realtime-policy.sh` rule that refuses an unbounded `try_pop` loop there, with a
     mutation in `test-realtime-policy.sh`;
   - record the artifact move (`476e58ad…` → `57dc99ab…`, +267 bytes on this base) and repin
     `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`.
2. **Attach the console.** Keep the track producers for the newest epoch. The same
   `control_queue_depth` also attaches every effect lane (15,361 bytes on the fixture). Either keep
   those producers for L2 and charge them, or add a builtins-only console request to host-core.
   Rule on which.
3. **The live delta.**
   - Address tracks by id, never by index.
   - Call the *same* domain checkers that preparation and the render-side setters use: export
     `builtins`' fader check, and use `Matrix2x2::checked` and `pan_matrix`. Do not replicate the
     range.
4. **Commit order.** Run `compiled_model_admission` and the room check, then push, then commit.
   The commit that follows the first push must be infallible by construction: a fallible validate
   before the push, an infallible publish after it. The live arm runs after the existing
   `plan_alive` and response-size checks.
5. **Resources.** Charge the `Vec<TrackControlProducer>` table and its `Box<str>` ids in
   `capi_retained_bytes`, and update both `resource_lifecycle` oracles. The prototype misses by
   24 bytes per retained epoch.
6. **Documentation.** Also document:
   - that the app must drain the reliable lane after each live edit (one `SESSION_COMMITTED` each,
     capacity 2, `compile.rs:98`), or the third edit gets a replayed `BACKPRESSURE` response;
   - that the audible change lags by `latency_samples`, because compensation is post-fader.

**A2. Decisions.**

- **D1.** Add: a `Pan` ↔ `Matrix` switch and a smoothing-only change are live. A transaction that
  rewrites identical values is live with zero records; it still advances the revision and emits
  `SESSION_COMMITTED`.
- **D2 (reworded).** An edit is admitted all or nothing.
  - It is applied no later than the first block whose render call begins after the submit returns.
  - A stage or track that has not drained yet may apply it one block earlier.
  - It is audible `latency_samples` later.
  - It is never lost.
  - #444's block-atomic contract is declined knowingly, and a "length snapshot" is not claimed as a
    substitute for it.
- **D3 (Sol's advice): a ramp.**
  - A step is a click on a mute: out-of-band energy -31 dB relative to the tone, against -70 dB for
    a one-quantum ramp.
  - A drag gives zipper noise: -50 dB against -88 dB.
  - After a ramp ends, the output equals the step's bits (D11 snap).
  - A ramp matches the browser bit for bit whenever both hosts use the same length.
  - Suggested: a fixed 5-10 ms, derived from the session rate at preparation (at least one
    quantum), stated in the header, with the SDK's fan-facing default set to the same value.
- **D4.** Unchanged.

**A3. Gates, corrected.**

1. **Gate 1.** Replace "changes the next block" with the D2 wording. The round-trip bit-identity
   is checked `latency_samples` plus one quantum after the unmute. On the 10-track parity shape
   (486 samples of latency), blocks 5-6 differ and 7 onward are identical. Keep 1 and 10 tracks and
   the four launch rates.
2. **Gate 2.** Add restore: a session compiled from `SessionSnapshotGet` after a live edit renders
   the edited value.
3. **Gate 3.** Add:
   - an exact replay pushes nothing (queue room unchanged);
   - reliable-event backpressure pushes nothing;
   - a live edit while a candidate is pending leaves the current plan's queues untouched.

   Replace the fault-injection clause: a fault *after* the push must be impossible by construction
   (A1.4). A test that only checks "no ack, no revision" passes while the plan plays an unacked
   value.
4. **Gate 4.**
   - The two-thread test also races a structural swap (catches F1).
   - It counts allocations and frees around each render call only.
   - Its final block is compared bit for bit with a fresh plan compiled from the final committed
     model after latency and ramp.
   - Put it in `crates/capi/tests/resource_lifecycle.rs`, which is already exempt from the `unsafe`
     scan, or justify a new exemption.
5. **Gate 5.** The `gain_pan_profile` digests show that static rendering did not move. They are not
   evidence for the drains, because `console-workload` pushes no fader or matrix records. The drain
   evidence is the `builtins-compiler` and `host-web` console tests plus the A1.1 policy rule.
6. **Gate 7 (new).** A domain-boundary test (-144, 24, ±0, NaN, ±1 matrix coefficients) through
   both the lowering and the render-side setter.

**A4. Dependencies.** L0 must land first, or ship inside L1 as its own first commit. Draft 02 with
step 4 stays independent. Step 4 also removes:

- protocol `queue.rs`'s `release_automation_admission` and its helper;
- the two test-only decode counters in `controller.rs`;
- the `delivery_ownership.rs` exemption in `scripts/check-realtime-policy.sh:27,31`.

Rebase #1034 on step 4.

**A5. The #140 ruling, corrected.** Close #140 as **descoped**, not superseded, and name an owner
for what it still gated:

- **Stored session automation rendering.** `docs/SESSION_SCHEMA_V1.md:97-100` and
  `docs/REALTIME_MEMORY.md:13` point at #140. Give it a successor issue, or an explicit
  out-of-scope ruling, and re-point both docs.
- **`AutomationEnqueue`.** Either refuse it as `UNAVAILABLE` and stop advertising it, which closes
  #349 IO-5, or keep IO-5 open.

