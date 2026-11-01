# Check the warm-successor deadline in miso_engine_v1_service and report its outcome

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287). Code anchors
refreshed on `main` at `7d89eecc5` (attempt 1).

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
  structural path of `crates/capi/src/runtime/control.rs` lives in `crates/control-plane`
  (`SessionState::service` at `crates/control-plane/src/control.rs:4133`).
- Warm preparation (`warm_lead`, `WarmConfig`, `WarmUnavailable`) is host-core code from #1354.
  Classification with an injected `WarmConfig`, the `Primed` publication and the `PrimedCandidate`
  record are *Classify a latency-growth edit and publish its warm successor from the control
  plane* (#1403). Render's prime adoption is #1311's and #1355's. The deadline
  (`prime_deadline_samples`, in render samples) is #1358's. `fall_back_to_transition` and the
  session counter `transition_reprepare_refusals` are #1397's. The watermark, its flags and
  `miso_engine_v1_plan_watermark` are #1314's.
- The C header's live-edit paragraph is `crates/capi/include/miso_engine_v1.h:50-124`; its session
  thread rule is `:18-26`.
- The C ABI builds its `SessionState` through the public `control_plane::compile_children`
  (`crates/control-plane/src/compile.rs:2544`, called at `crates/capi/src/runtime/mod.rs:198`). The
  structural fault and owner counters for tests are `TestOwnerCounters`
  (`crates/control-plane/src/control.rs:951`, read by `test_lifecycle_counters`, `:1223`).
- The release audit runs `./target/release/audit capi` (`.github/workflows/qualification.yml:889`)
  from `tools/audit/src/capi.rs`.

## Decisions frozen for this slice

- **D1. The production `WarmConfig`.** (Amended, root ruling Q2 of attempt 1: the config is
  central. `host_core::warm::production_config(fs, q)` builds it, and the control plane's public
  `compile_children` and `compile_children_with_boot_command` pass it, from the parsed session's
  rate and quantum, so the C ABI and the browser get it with no host code. The gates keep the
  crate-private `compile_children_warm` for `None` and for custom bounds.) Every product session
  holds
  `warm: Some(WarmConfig { p_max: p_max_samples(fs, q), prime_bytes_max: PRIME_BYTES_MAX })` for
  the session's rate and quantum (#1403 D1; `p_max_samples` is *Grow the default source ring by the
  warm-prime headroom*, #1406 D1), so its rebuilds are classified by #1403 D2 and routed by #1397
  D1-D2. Before this slice only `test-support` sessions hold a `WarmConfig`; every arm of #1403 D2
  exists
  once #1397 has landed, which this slice depends on.
  This slice writes `PRIME_BYTES_MAX` into `crates/host-core/src/warm.rs` with the value *Record
  the swap block's cost on the 64-track console* (#1286) D3 derives; the constant's comment names
  the record row. It writes no classification, publication or routing of its own. A valid edit is
  never refused for its warm path, and the transaction response is `rebuild` in every case
  (*Report each transaction's edit path in its response*, #1313).
- **D2. Service step.** `SessionState::service` gains one step after `synchronize_plan_epochs`:
  the deadline check of #1358.
  - If the control plane holds no #1403 record, or the record's deadline in render samples has not
    passed, the step does nothing.
  - Otherwise it withdraws the candidate (`withdraw()`, #1343 D5, through #1358 D3's
    `check_prime_deadline`). `Withdrawal::Taken` means render
    adopted it exactly, and the step publishes nothing. `Withdrawal::Withdrawn` hands the candidate
    to `fall_back_to_transition` as its donor (#1397 D2), which re-prepares and publishes the
    transition. If that re-preparation is refused, #1358 D3 republishes the donor and sets its
    record again, so the next service call's step withdraws it again.
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
    host keeps at least `P + q` frames of each source queued past that source's read position,
    where `P` is the latency growth rounded up to whole render quanta and `q` is the render
    quantum. A host that keeps its rings full always qualifies with the default ring
    (`source_ring_frames = 0`), whose headroom covers `P_MAX` (#1406).
  - A host that passes a nonzero `source_ring_frames` needs at least `stall + P_MAX + q` frames
    for warm growth: `stall` is the stall body of the default rule (#1354 D1's
    `stall_ring_frames`) and `P_MAX` is #1406's `p_max_samples` for the session's rate and
    quantum. With
    less, a growth whose `P + q` exceeds the ring's frames above `stall` takes the transition
    (`WarmUnavailable::LeadBound`, #1354 D2), which still completes the edit.
  - At adoption the engine reads each carried source `P` frames further ahead in that one render
    call. From that block on, the source-read clock leads render by `P` more (#1396), and a command
    or `seek_at` that arrives during that call is applied at that block on the new clock.
  - Outcomes: `MISO_ENGINE_V1_OUTCOME_EXACT` when render adopted it;
    `MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK` when the successor was unavailable at submit or not
    ready by the deadline (the edited strips duck and fade back in);
    `MISO_ENGINE_V1_OUTCOME_SUPERSEDED` set when a later structural edit replaced it before
    adoption. A declared stop (#1323) completes it `EXACT`, with `SUPERSEDED` only when it had
    itself folded in earlier revisions (#1323 D4). A paused host stays pending and never falls back,
    because the deadline counts render samples.
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

1. D1's config in `crates/control-plane/src/compile.rs` (the public session compile; amended by
   root ruling Q2), D2, D3 and D5 in `crates/control-plane/src/`, and D1's `PRIME_BYTES_MAX` and
   `production_config` in `crates/host-core/src/warm.rs`.
2. Two `test-support` owner counters beside `TestOwnerCounters`' existing ones: deadline
   withdrawals and transition re-preparations, each raised once per event by D2's step.
3. D4 in `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md`.
4. D6 in `tools/audit/src/capi.rs`.
5. C ABI tests in `crates/capi/tests/latency_growth.rs` (new).

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
   re-preparation (deliverable 2's counters each grow by exactly 1).
5. **Refusal counter (control-plane test, `test-support`).** With one injected re-preparation
   refusal, the service step keeps the revision pending and republishes the donor (#1358 D3),
   `COUNTERS_GET` reports `TRANSITION_REPREPARE_REFUSALS` 1, and the next service call withdraws
   it again and publishes the transition (deliverable 2's withdrawal counter reads 2).
6. **Audit.** `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   passes with the new leg.
7. Commands:
   - `cargo test --locked -p capi`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-workspace-policy.sh`, `cargo run --locked --release -q -p realtime-policy`
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
- Gate 6: an audit leg that services from the render thread, or a growth whose prime allocates,
  fails the audit's allocation or thread checks; a production session built without a
  `WarmConfig` takes the ordinary path and fails the leg's `EXACT` and output checks. Red.

## Dependencies

- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `WarmConfig`
  and `stall_ring_frames`.
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403):
  the classification D1 configures and the record D2 checks.
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): `Primed`.
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): render's
  adoption.
- *Fall back to the transition when a warm successor is not ready by its deadline* (#1358): the
  deadline D2 checks, and the republished donor on a refused re-preparation.
- *Grow the default source ring by the warm-prime headroom* (#1406): `p_max_samples` and the
  default ring D4 describes.
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

## Amendment (controls coordinator, batch 15a, 2026-10-15)

- `AdmissionPeak::WithReprepare` named above did not ship with #1398 (root ruling: code with no
  production caller is a placeholder). #1403 adds the reprepare admission peak together with its
  first caller; see its batch 15a amendment.

## Amendment (attempt 1, 2026-11-01: root rulings on the plan's questions)

- **Q1.** #1286 first. Its record (PR #2228, `artifacts/steps/swap-carry-base/`) gives
  `PRIME_BYTES_MAX = 3,493,032` bytes: the smaller of its two rounds (round 1), native row binding.
  Inputs: `o` 177,157 ns, `a_P` 625,831 ns, `b_P` 2,072,576 B, `β` 216.481 ps/B, `m` 0.7.
- **Q2.** The config is central (D1 and deliverable 1 above are amended). #1361's deliverable 3 (the
  browser wiring) is dropped with it.
- **Q3.** Firefox is out (owner ruling 2026-10-25): Chromium and WebKit only.
- **Q4.** A changed growth expectation is the feature. Each one is worked out again and reported; no
  gate gets weaker.
- **Q5.** One PR closes #1360 (S1 to S4).
- **Paths beyond the list above**, each a direct consequence of the slices:
  `crates/host-core/src/prepare.rs` (one doc line on `WarmConfig`), `hosts/host-web/src/tests.rs`
  (one growth expectation worked out again, S2), and `tools/realtime-policy/src/policy.rs` with
  `docs/REALTIME_DEPENDENCY_POLICY.md` (the new test binary's unsafe C calls join the ownership
  allowlist).
- **Stated limit (#2229, not in scope).** At 96 kHz and quantum 128, the first warm adoption block
  (p99 1.11 ms) and every carrying-swap move block (1.07 ms) are above 70% of the 1.333 ms quantum.
  A warm adoption is not slower than the move it replaces, so this issue goes on; #2229 owns the
  margin.

## Evidence (attempt 1)

Branch `codex/issue-1360-warm-deadline`, from `7d89eecc5`.

- **S1** (`SessionState::service`, `control.rs:4133`): the step runs after `synchronize_plan_epochs`
  and before the first counter refresh. A `Backpressure` (the render half is gone) is skipped, so
  every other session call goes on. `TestOwnerCounters` gains `deadline_withdrawals` and
  `transition_reprepares`, raised once per withdrawal in `fall_back_pending_warm`.
  - New test `the_service_step_runs_the_deadline_step_once` (gates 4 and 5 through `service`).
    Red with the step taken out of `service` (`(0, 0)` against `(1, 1)`), and red with the step run
    twice per call (`(2, 2)` against `(1, 1)`).
  - The four step-level tests now take the step from the block's service. The rig's unserviced
    mode makes no control call at all (its feed uses `submit_unserviced`). Run 3 of
    `a_refused_fallback_at_the_deadline_keeps_the_donor_pending` is worked out again: B's own service
    step withdraws W first (withdrawal 2) and B supersedes the transition at its `S`, `EXACT |
    SUPERSEDED`, no underrun. The paused 1,000-step loop of
    `a_warm_candidate_not_ready_by_its_deadline_takes_the_transition` is deleted: the new test owns
    that claim through `service`.
- **S2** (`PRIME_BYTES_MAX`, `production_config`, `compile_children_bounded`).
  - New test `a_publicly_compiled_session_publishes_a_growth_primed`: red with no production
    config (the growth is published as an ordinary successor).
  - Sweep, by a temporary probe at each warm classification and fallback over the capi, host-web,
    control-plane and host-core suites: one expectation changed.
    `hosts/host-web/src/tests.rs::the_plan_latency_record_follows_the_active_plan` feeds each source
    only the block it renders, so its growth (a limiter on an audible track) is never ready. Old:
    an ordinary duck-swap adopted at sample 1,408, epoch 1. New: warm, held to its duck's `S`
    1,408, the transition at 1,408 + 7,040 = 8,448, adopted at its own `S`, ceil_q(8,448 + 128 +
    480) = 9,088, epoch 2. The test now services after each block, as the Worker's loop does; red
    on the old config (1,408 against 9,088).
  - These now take the transition at submit (#1397a: a duck is pending) or the warm path and pass
    unchanged: `crates/capi/tests/strip_transitions.rs::every_restarting_edit_ducks_and_a_send_or_value_edit_does_not`,
    and in `hosts/host-web/tests/control_parity.rs`
    `a_rebuild_that_adds_latency_moves_the_plan_latency_alike_on_both_hosts`,
    `a_stale_effect_row_edit_is_refused_alike_on_both_hosts`,
    `both_hosts_boot_an_empty_session_and_add_the_first_stem_alike` and
    `both_hosts_designate_the_master_strip_after_an_empty_boot_alike`.
- **S3** (`crates/capi/tests/latency_growth.rs`; D4 in the header's "Latency growth during
  playback" paragraph and in `docs/C_ABI_V1_QUALIFICATION.md`). Gates 1 to 3, one test each:
  `a_growth_with_sources_queued_ahead_is_adopted_exact`,
  `render_adopts_the_growth_with_no_session_call` and
  `a_growth_never_ready_falls_back_only_at_a_service_call` (the transition is adopted at
  `ceil_q(clock + q + 480)`). With no production config, gate 1 diverges at block 29, gate 2 at
  block 6, and gate 3 completes the revision with no service call.
- **S4** (`tools/audit/src/capi.rs::growth_leg`): a render thread and a service thread, `EXACT`,
  zero render-thread violations (added into the record's totals; the record's keys do not change),
  output equal to the predecessor rendered on. With no production config the leg fails at block 29.
- **No bit moved elsewhere.** A scratch harness rendered every `fixtures/session/v1` session that
  compiles (17; `json-invalid-duplicate-key.json` does not parse and `canonical.json` is refused at
  compile on both trees) for 300 blocks through the C ABI, plain and live (six mute toggles and one
  non-growth structural edit), at `7d89eecc5` and at the head: all 34 digests and watermarks are
  equal. `audit capi`'s main `pcm_digest` is `30dcfd806e7ba058` on both.
- **Checks.** fmt; workspace clippy (`--all-targets --all-features -D warnings`); release tests of
  control-plane, host-core, capi, host-web and audit (`--all-features`) and realtime-policy;
  `check-workspace-policy.sh`, `realtime-policy` and `check-test-support-ci.py`;
  `check-capi-abi.sh`; `audit capi` (0 violations); `build-web-audioworklet.sh` and
  `check-web-audioworklet.sh`: all pass.

## Attempt 2 (2026-11-01): rebased onto `1fa4cee4e`

- **Rebase.** Onto `origin/main` `1fa4cee4e`, which holds #2232 (`fe37db741`, PR #2235), #2137 and
  #1713/#1718. No conflict; #1713's deletions stay.
- **#2232.** `warm_lead` decides on `GraphCompiler::project` and prepares no effect processor: at 256
  tracks its peak fell from 222.7 MB to 14.0 MB, with the same answers.
- **The bus add's transition is the intended behaviour.** The 256-track `console-limits` bus add
  answers `Unavailable(Misaligned)`, which is correct. With `warm: None` it completed `EXACT`
  falsely: the output had a hard 10.1 ms gap of zeros, then the whole stream ran 486 samples late
  (#2232's diagnosis). Under D15-8 an `Unavailable` growth takes the transition, so S2's path (the
  transition, a ramped duck, `TRANSITION_FALLBACK`) is right.
- **Sweep, again.** The same probe over the capi, host-web, control-plane and host-core suites gives
  the same set as attempt 1 (plus this issue's own gates); no expectation changed again.
- **No bit moved, again.** The 34 digests of `1fa4cee4e` and the head are equal; `audit capi`'s main
  `pcm_digest` is `30dcfd806e7ba058` on both.
- **Browser.** `npm run qualify -- --check-matrix --self-test-mutations`, `sdk/dist` deleted: Chromium
  `single` and `worker`, WebKit `single` and `worker` all pass; `console-limits` replies `ok`,
  path `rebuild`, render allocations 0.
- **Checks.** fmt; workspace clippy (CI flags); release tests of control-plane, host-core, capi,
  host-web, audit and realtime-policy (`--all-features`); workspace policy, `realtime_policy`,
  `check-test-support-ci.py`; `check-capi-abi.sh`; `audit capi` (0 violations); web build and
  check; aarch64 `cargo check --release --tests --all-features` of the touched crates.

## Review folds (attempt 2 verify: PASS, three MINORs)

- **M1.** `miso_engine_v1_service` reported every service failure as `source.epoch`. The control
  plane's `SessionState::service_step` now names the failing part (`ServiceFailure::Epoch` or
  `::Deadline`); `service()` wraps it for every other caller. The C entry reports the deadline
  step's failure as a rebuild refusal: `COMPILE_REJECTED` with the compile diagnostic, or
  `INTERNAL` with `control.internal`. No test: no existing test can make the step's transition
  fail cheaply (an allocation refusal in `observation.after`), and a new hook is not worth it.
- **M2.** The header and the qualification doc said the transition ducks the edited strips. It
  ducks every strip whose arrival the growth moves (`grown_strips`): a bus add with a latent effect
  on the new bus ducks every track routed into it.
- **M3.** The audit leg's claim is the render thread's counters across the adoption block. Its
  bit-identity check is dropped (gate 1 owns exactness), and it keeps one precondition: the
  watermark covers the growth with `EXACT` inside the audited blocks. D6's "output equal to the
  predecessor" is gate 1's from here on.

### History: attempt 1's browser blocker (fixed upstream by #2232)

`npm run qualify -- --check-matrix --self-test-mutations` fails in all four legs (Chromium and
WebKit, `single` and `worker`) at `console-limits`: the #2191 bus add on the 256-track console
session (a submix with the console's post-insert true-peak limiter, a route from every track into
it) fails with `internal` from a control call. With the production config switched off, the
Chromium `worker` leg passes. Native reproduction (`AudioWorkletEngineHost`, same document and
frame):
- `warm_lead` answers `Unavailable(Misaligned)`, so the edit takes the transition at submit
  (`TRANSITION_FALLBACK`, adopted at 1,152) instead of the ordinary rebuild (`EXACT`, at 512).
- Peak RSS: 175 MB with no config; 200 MB when `warm_lead` runs but its answer is ignored; 300 MB
  with the transition. The transition restarts the grown strips at rest, so it allocates fresh
  state that the ordinary rebuild carried. `warm_lead` also adds 1.9 s of control-thread time
  (1.08 s to 2.94 s), which in `single` mode runs in the worklet's message handler.
- The Wasm module's memory is capped at 512 MiB (`build-web-audioworklet.sh`, 8,192 pages), and
  this session's boot already reaches about 300 MB there, so the transition's allocation fails and
  the host reports `internal`.

So, through S2, a valid console edit (a bus add) is refused by its warm path, against D1 ("a valid
edit is never refused for its warm path"), and on a 256-track console it also ducks every grown
strip. Attempt 1 asked for a ruling before S2 merged: for example, admit the classification's and the
transition's peak (a typed refusal, not `internal`), answer `Ordinary` for a growth whose duck
set the session cannot afford, or bound `warm_lead`'s passes. It is not fixed here (owner of
`warm_lead`: #1354/#1403; of the transition: #1397).
