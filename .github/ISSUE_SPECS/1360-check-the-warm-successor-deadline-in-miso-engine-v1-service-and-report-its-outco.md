# Check the warm-successor deadline in miso_engine_v1_service and report its outcome

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287). Code anchors
verified on `main` at `6fb211594`.

## Product outcome

A mobile host that adds a latent effect during playback gets a seamless swap through the C ABI.
- `SESSION_TRANSACTION_APPLY` returns at once with path `rebuild`.
- The host keeps rendering on its audio thread and keeps its sources queued ahead. Render adopts
  the warm successor by itself, at the first block where every carried source is ready.
- The host calls `miso_engine_v1_service` (or any session call) from a control thread. Those calls
  only check the deadline, and run the transition fallback once it has passed.
- The watermark shows the revision completing `EXACT`, `TRANSITION_FALLBACK` or with `SUPERSEDED`.
  Nothing in submit waits for render.

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
- Warm preparation (`warm_lead`, `WarmUnavailable`) is host-core code from #1354. The `Primed`
  publication and render's prime adoption are #1311's and #1355's. The deadline
  (`PRIME_DEADLINE_SAMPLES`, in render samples) is #1358's. `fall_back_to_transition` and the
  session counter `transition_reprepare_refusals` are #1397's. The watermark, its flags and
  `miso_engine_v1_plan_watermark` are #1314's.
- The C header's live-edit paragraph is `crates/capi/include/miso_engine_v1.h:35-62`; its session
  thread rule is `:18-22`.
- The release audit runs `./target/release/audit capi` (`.github/workflows/qualification.yml:715`)
  from `tools/audit/src/capi.rs`.

## Decisions frozen for this slice

- **D1. Classify.** The control plane's rebuild path calls `host_core::warm_lead` (#1354), the
  only computation of the lead `P`, with a `WarmConfig` built from `P_MAX_SAMPLES` (#1358) and
  `PRIME_BYTES_MAX` for the session's rate and quantum. This slice writes `PRIME_BYTES_MAX` into
  `crates/host-core/src/warm.rs` with the value *Record the swap block's cost on the 64-track
  console* (#1286) D3 derives; the constant's comment names the record row.
  - With `Warm` and `P > 0`, the candidate's admission uses `AdmissionPeak::WithReprepare` (#1398
    D4), so the transition's re-preparation inside a later service step never exceeds the caller's
    caps. The candidate is prepared warm (#1354) and published `Primed { not_before, lead_blocks }`
    (#1311, #1355). When it also restarts strips, it is composed with their duck as *Duck-swap the
    strips a latency growth restarts, and fall back to the transition when a warm successor cannot
    adopt* (#1397) D1 states.
  - With `Warm { lead_samples: 0, restart_whole }` (the restarts confine the growth to restarted
    strips), it is an ordinary rebuild plus those restarts: prepared with that `WarmLead` (#1354
    D4) and published as a planned duck-swap (#1397 D1, last paragraph). No deadline is pending.
  - If warm preparation returns `WarmUnavailable` (#1354), the edit takes
    `fall_back_to_transition` (#1397 D2) at once, in the same submit. A valid edit is never refused
    for it.
  - With `Ordinary`, it is published as today.

  The transaction response is `rebuild` in every case (*Report each transaction's edit path in its
  response*, #1313).
- **D2. Service step.** `SessionState::service` gains one step after `synchronize_plan_epochs`:
  the deadline check of #1358.
  - If no `Primed` candidate is pending, or its deadline in render samples has not passed, the step
    does nothing.
  - Otherwise it withdraws the candidate (`withdraw()`, #1343 D5, through #1358 D3's
    `check_prime_deadline`). `Withdrawal::Taken` means render
    adopted it exactly, and the step publishes nothing. `Withdrawal::Withdrawn` hands the candidate
    to `fall_back_to_transition` as its donor (#1397 D2), which re-prepares and publishes the
    transition.
  - One step does at most one withdrawal and at most one re-preparation. It never loops on render
    progress and never waits, so #1348 D3's bound holds with this step added. There is no slice
    constant and no rendering on the control thread.
- **D3. Threads.** The readiness check, the claim and the prime run on the render thread inside
  `miso_engine_v1_render_f32_planar` (#1355). The deadline check and any re-preparation run on
  whichever control thread calls a session function. The growth submit itself does no adoption
  work: it prepares, publishes and returns.
- **D4. Header and qualification doc.** `miso_engine_v1.h`, beside the live-edit paragraph
  (`:35-62`), and `docs/C_ABI_V1_QUALIFICATION.md` state:
  - A latency-growing edit is adopted by render, not by a control call. For it to be exact, the
    host keeps at least `P + q` frames of each source queued ahead of render, where `P` is the
    latency growth rounded up to whole render quanta and `q` is the render quantum. A host that
    keeps its rings full always qualifies, because ring headroom covers `P_MAX` (#1358).
  - At adoption the engine reads each carried source `P` frames further ahead in that one render
    call. From that block on, the source-read clock leads render by `P` more (#1396), and a command
    or `seek_at` that arrives during that call is applied at that block on the new clock.
  - Outcomes: `MISO_ENGINE_V1_OUTCOME_EXACT` when render adopted it;
    `MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK` when the successor was unavailable at submit or
    not ready by the deadline (the edited strips duck and fade back in);
    `MISO_ENGINE_V1_OUTCOME_SUPERSEDED` set when a later structural edit or a declared stop (#1323)
    replaced it before adoption. A paused host stays pending and never falls back, because the
    deadline counts render samples.
  - Any session call may do at most one bounded service step, and that step may re-prepare once.
  - The counter `TRANSITION_REPREPARE_REFUSALS` (#1351) is read with `COUNTERS_GET` or a
    configured `COUNTER_SNAPSHOT`. It counts transition re-preparations that were refused anyway
    and retried at the next service step. It stays 0 in a correct build.
  - No new symbol and no new feature bit.
- **D5. Counter surface.** The service step's counter refresh (#1348 D8) copies the session's
  `transition_reprepare_refusals` (#1397) into the provider's `TRANSITION_REPREPARE_REFUSALS`
  (#1351 D6), beside the telemetry counters.
- **D6. Audit.** `audit capi` gains one leg. During playback, with every source queued ahead, it
  adds a muted track with a true-peak limiter, renders on one thread and services from a second
  until the watermark covers the revision. It checks `EXACT`, zero render allocations, and output
  equal to the predecessor rendered on with no swap.
- **D7. Acked-batch question.** Submit's ack follows every fallible step (D15-17). After it, the
  revision completes through the watermark: `EXACT` by render, `TRANSITION_FALLBACK` by the
  transition, `SUPERSEDED` by #1310. A refused re-preparation keeps the donor and retries; it never
  completes the revision as nothing. An ack can never precede a drop.

## Deliverables

1. D1, D2, D3 and D5 in `crates/control-plane/src/`, and D1's `PRIME_BYTES_MAX` in
   `crates/host-core/src/warm.rs`.
2. D4 in `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md`.
3. D6 in `tools/audit/src/capi.rs`.
4. C ABI tests in `crates/capi/tests/latency_growth.rs` (new).

## Authorized paths

- `crates/control-plane/src/`, `crates/capi/src/`, `crates/capi/include/miso_engine_v1.h`
- `crates/host-core/src/warm.rs` (`PRIME_BYTES_MAX` only)
- `crates/capi/tests/latency_growth.rs` (new), `tools/audit/src/capi.rs`
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- The browser (#1361). New ABI symbols. The readiness check and the prime themselves (#1320,
  #1355).

## Objective gates

1. **Exact through the ABI.** A render thread renders 128-frame blocks at 48 kHz through
   `miso_engine_v1_render_f32_planar`, with every source queued at least `P + q` frames ahead. A
   control thread applies the growth transaction and then calls `miso_engine_v1_service` in a
   loop. The rendered stream equals the predecessor rendered on with no swap, bit for bit, and the
   watermark reaches the revision with `EXACT`.
2. **Submit does no adoption work; render adopts without service.** With no render running, the
   growth `SESSION_TRANSACTION_APPLY` returns `OK` with path `rebuild` and the revision pending.
   Then render runs with sources queued ahead and no further session call: render adopts, the
   output equals the predecessor rendered on, and `miso_engine_v1_plan_watermark` reads the
   revision with `EXACT`.
3. **No service, no fallback.** With the frames past each consumer withheld, render runs for twice
   the deadline with no session call: the revision stays pending, and the output equals the
   predecessor rendered on. The first service call after that publishes the transition, and the
   watermark reports `TRANSITION_FALLBACK`.
4. **Bounded and patient.** Before the deadline, 1,000 service calls with no render leave the
   candidate pending and publish nothing. After it, one call does exactly one withdrawal and one
   re-preparation (the control-plane test counters).
5. **Refusal counter (control-plane test, `test-support`).** With one injected re-preparation
   refusal, the service step keeps the revision pending, `COUNTERS_GET` reports
   `TRANSITION_REPREPARE_REFUSALS` 1, and the next service call publishes the transition.
6. **Audit.** `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   passes with the new leg.
7. Commands:
   - `cargo test --locked -p capi`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a classify that publishes the candidate as an ordinary `Next`, or a host path that does
  not raise the source-read offset, gives a gap or a shifted stream. Red.
- Gate 2: the growth submit itself must do no adoption work: a submit that waits for render never
  returns with no render running, and a design that needs a control call to adopt leaves the
  revision pending. Red.
- Gate 3: a deadline counted in wall time or in control calls, or a fallback started without a
  service call, changes when and how the revision completes. Red.
- Gate 4: a service step that withdraws before the deadline, or loops over several withdrawals or
  re-preparations, breaks the counts. Red.
- Gate 5: a refresh that never copies the session counter, or a refusal that completes the
  revision, reports 0 or completes early. Red.

## Dependencies

- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `warm_lead` and
  `WarmUnavailable`.
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): `Primed`.
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): render's
  adoption.
- *Fall back to the transition when a warm successor is not ready by its deadline* (#1358): the
  deadline D2 checks, and `P_MAX_SAMPLES`.
- *Record the swap block's cost on the 64-track console* (#1286): the `PRIME_BYTES_MAX` value D1
  writes.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397): D1's composition, `fall_back_to_transition` and the session
  counter.
- *Reset latency floors at a host-declared discontinuity* (#1323): the declared stop D4 describes.
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398):
  `AdmissionPeak::WithReprepare`.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Report each configured counter's own value in the C ABI counter snapshot* (#1351): the
  `TRANSITION_REPREPARE_REFUSALS` ID.
