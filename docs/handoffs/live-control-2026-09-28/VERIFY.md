# Verification of the #1020 live-control research

Date: 2026-09-28. Verifier: Sol (adversarial). Subject: `FINDINGS.md` in this folder, as committed
in `e3bf9d8c` on `codex/batch-slim-1`. Every fact below was reproduced on that tree by a build, a
test or a grep. Two parts ran as parallel passes and are marked as such: the drain check (item 3)
and the deletion check (item 4). No product code, test, script or workflow in this repository
changed. Nothing was pushed, and no GitHub issue was edited. The scratch copies and their `target/`
directories were deleted afterwards. Section 7 of `FINDINGS.md` now ends with an **Amendments**
section that applies the findings below.

## 1. Verdict

**Direction sound; do not adopt the ruling as worded.** Option A, value-only
`SessionTransactionApply` lowered onto the console lanes, is the right design. I rebuilt it
independently, and it does what the research says. Deleting the unwired stack is safe. The ruling
needs three corrections first:

1. **The C ABI's plan replacement is broken under real concurrency (F1).** This predates the
   research. The research called the "epoch race" safe, but only tested it on one thread. Any
   control call that lands during a render call that swaps plans returns `INTERNAL`. After that,
   every later structural edit returns `BACKPRESSURE`, forever. L1's own two-thread gate hits it in
   14 of 20 runs. A bounded fix (L0) must land before L1, or inside it.
2. **Closing #140 is a descope, not a supersession (F3).** Two #140 outcomes lose their owner:
   sample-timed `AutomationEnqueue` delivery, and the rendering of stored session automation
   (`docs/SESSION_SCHEMA_V1.md:97-100` names #140 as its gate). The ruling must say so and name an
   owner or an explicit out-of-scope for each.
3. **Several L1 gates are wrong or do not discriminate (F2, F4, F5).** Corrected in the Amendments.

**D3 advice: use a ramp** (section 5). A step at the block boundary is an audible click on a mute
and zipper noise on a drag. A ramp still lands on the same bits as a step once it ends, and it
matches the browser bit for bit whenever both hosts use the same ramp length.

## 2. Which claims hold

| FINDINGS claim | verdict |
|---|---|
| The live console lanes are shared host-core code; capi opts out only through `HostConsoleRequest::default()` at `capi/src/runtime/compile.rs:410` | **Holds.** Replacing that one call with `prepare_host_runtime_with_console` (depth 16) attaches the lanes. `host-web` compiles nothing new: the wasm `simd128` check passes. |
| A prototype of about 293 lines passed 3 behavioural tests, bit-identical to a console-free plan | **Holds, reproduced independently.** My rebuild is 230 lines of `host-core/src/live_delta.rs`, plus about 90 lines in capi and host-core. It passes 8 unit tests and 2 two-thread tests. The bit-identity check was widened from block 0 to 12 blocks of non-trivial PCM. `clippy -D warnings` is clean across the workspace. |
| 13,504 unwired lines can go; net -12,811 | **Holds** (parallel pass). The seven files are exactly 13,174 lines. With the plumbing (+38/-13,511) the deletion alone nets -13,473. The -12,811 figure included the prototype. `check`, `clippy`, wasm, CI's `test-debug-a` set (1,432 passed) and the host-core, realtime, protocol and CI-routing policy scripts all pass. One stale exemption remains: `scripts/check-realtime-policy.sh:27,31` still exempts `delivery_ownership.rs`. |
| No ack can precede a drop, under five conditions | **Holds. I found no path where an ack precedes a drop** (table in section 4). Two conditions are incomplete. Push-then-commit, when the commit fails, applies a value that was never acked (F4). The "epoch race" was never tested concurrently (F1). |
| "The model is the durable owner, so a replacement cannot lose a value" | **Holds on every capi path.** capi has no reset or restore path that re-seeds the builtins from anything but the compiled model. There is no plan-level reset. `restore_state_payload` is used only by effect conformance tests. `source_seek` touches only the rings. A host that restores by recompiling from `SessionSnapshotGet` gets the live values, because the snapshot carries them. |

## 3. Findings, severity-ranked

### F1 (high, pre-existing): a control call inside a swapping render call wedges capi's plan replacement

- **Mechanism.**
  - `RealtimePlanOwner::enter_block` swaps plans and commits the retired plan to the retirement
    queue at the *start* of the render call (`engine/src/realtime/plan_exchange.rs:361-424`).
  - capi publishes `active_epoch` only after the render call returns (`capi/src/runtime/plan.rs:215-216`).
  - During that window, `synchronize_plan_epochs` (`control.rs:619-661`) sees
    `active_epoch == providers.epoch`, so it does not promote. It then reclaims the retired plan,
    finds no retired provider for its epoch, and returns `Internal`.
  - It also skips removing that epoch's report row, because it compares against the stale atomic.
  - On the next call, the stale provider is pushed into `retired_providers` and never leaves, and
    the report table stays full. So every later structural command returns `Backpressure`.
- **Reproduced.**
  - A deterministic test splits a render call at the point where a concurrent control call can
    observe it. On the unmodified base: `Err(Internal)`; then `retired [0]` stays forever; then
    every later structural command returns `Backpressure`.
  - Naturally, in the two-thread test that races live edits against a structural swap: 14 of 20
    runs got `RESULT_INTERNAL` (255).
  - Every control entry point synchronizes: submit, seek, command, event.
- **Failure scenario.** A fan's app feeds PCM from a decode thread while the audio callback renders.
  Today every fader move is structural. The first move during playback swaps plans. A PCM submit
  that lands in that render call gets `INTERNAL`, and from then on every edit the fan makes returns
  `BACKPRESSURE`. After L1, fader moves no longer swap, but EQ and effect edits still do until L2.
  So L1 narrows the window but does not close it.
- **Not an ack-before-drop.** The failing call acks nothing. Its only relation to the research is
  that the research's own concurrency gate trips on it.
- **Fix (verified in scratch; +25/-14 lines in `synchronize_plan_epochs`).**
  - A reclaimed epoch equal to `providers.epoch` proves that the one pending candidate is now
    active, so promote it there.
  - Then keep only the report rows that the lagging atomic, the current epoch or a pending epoch
    can still read.
  - With the fix, the swap race passes 20 of 20 runs and every existing capi test passes.
- **Correction.** Add a bounded predecessor issue, L0, that lands before L1: the fix, the
  deterministic split-render test, and the two-thread swap test.

### F2 (medium): L1 gate 1 would fail on the 10-track fixture as written

- **The claim.** "A `SetTrackFader` ... transaction changes the next block", and a mute-and-unmute
  round trip is bit-identical "at 1 and 10 tracks".
- **What happens on the 10-track parity shape.** One track carries a bypassed true-peak limiter,
  which gives the plan 486 samples of latency. Latency compensation sits after the fader. So a
  mute applied at block 2 reaches the output at blocks 5-6, not block 3. Measured bit-identity
  after the unmute: blocks 3-4 identical, 5-6 different, 7-11 identical.
- **Under `Concurrent` delivery**, a record pushed while render is mid-block reaches any stage that
  has not drained yet. So some tracks or stages apply the edit one block *earlier* than others.
  "Next block" is only an upper bound.
- **Failure scenario.** The implementer runs gate 1 on the 10-track fixture, sees blocks 3-6
  disagree, and either weakens the gate or files a false bug.
- **Correction.**
  - Timing contract: "applied no later than the first block whose render call begins after the
    submit returns; audible `latency_samples` later".
  - Bit-identity is checked after `latency_samples` plus one quantum.

### F3 (medium): closing #140 is a descope, and it orphans two outcomes

Parallel pass, plus my own reading.

- **`AutomationEnqueue` reaching PCM.** This is #140's D4 and children B and C. Option A does not
  deliver it. capi still advertises the automation-batch capability (the capability registry,
  `protocol/src/controller.rs` near `:3668`), and #349 IO-5 ("wire the drain or retire one model")
  stays open unless capi refuses the command as `UNAVAILABLE` and stops advertising it.
- **Rendering of stored session automation.** The session `automation` table accepts
  `builtins`/`strip` targets for fader, mute and pan, and the SDK builder can author them.
  `docs/SESSION_SCHEMA_V1.md:97-100` says the table "renders nothing" and that builtin automation
  rendering "is gated on issue #140's span feed". `docs/REALTIME_MEMORY.md:13` also points at #140.
- **Failure scenario.** A producer authors a fade-out through the SDK. Fans hear no fade on any
  host, and after #140 closes no open issue owns the gap.
- **Correction.**
  - Rule "descoped", not "superseded".
  - For stored automation: a successor issue, or an explicit out-of-scope ruling.
  - For `AutomationEnqueue`: refuse it and stop advertising it, which closes IO-5; or keep IO-5
    open.
  - Re-point the two docs.
  - Rebase #1034 on draft 02's step 4, so the dead protocol items are not deleted twice.

### F4 (medium): push-then-commit turns a failed commit into an applied, unacked value

- **Reproduced.** A fault injected after the push and before the commit returns an error, and it
  leaves the revision, the model and the replay cache unchanged. But the records are already queued.
  The next block is silent (a mute was pushed) while the model says unmuted. A later structural swap
  silently reverts the audible state.
- **Today this is unreachable.** `commit_prepared_structural` fails only on `WrongController` or
  `StaleGeneration`, and both are impossible under the serialized session contract. But the
  research's gate ("fault injection between push and commit leaves no ack and no committed
  revision") *passes* in this state, so it does not discriminate.
- **Correction.** Make the commit after the first push infallible by construction: a fallible
  `validate` before the push and an infallible `publish` after it. Assert the invariant: no
  fallible step between the first push and the commit.

### F5 (medium): nothing would catch an unbounded drain; the listed digest gate cannot see the change

Parallel pass.

- **Bounding the four drains is class A for the browser.**
  - In the single-threaded browser, `available_at_entry()` equals what the `while` loop pops.
  - The artifact hash moves `476e58ad…` → `57dc99ab…` (+267 bytes; the base reproduces the checked-in
    pin).
  - `gain_pan_profile digests` are byte-identical.
  - `builtins-compiler`, `host-web --lib` and `host-core --all-features` tests pass.
  - The only side effects lost are the final empty-pop counter and one cursor reload, and nothing
    reads either.
- **No other render-side drain becomes unbounded** under a concurrent producer:
  - the input drains and `EffectControlLane::stage` are already bounded by entry count;
  - the source ring is bounded by the transfer-block count;
  - `enter_block` pops at most one;
  - observation pops at most one per queue.
- **But no gate would notice.**
  - `console-workload` pushes no fader or matrix records, so the digests cannot see the change.
  - `scripts/check-realtime-policy.sh` scans only `REALTIME_POLICY` regions (there are none in
    `builtins-compiler/src/lib.rs`), and it checks allocation, lock and I/O patterns, not loops.
- **Failure scenario.** A later edit reintroduces `while let Ok(_) = try_pop()` in a drain. Every
  listed gate stays green, and a busy control thread can then keep the render thread draining.
- **Correction.** Mark the four drains as realtime regions and add a policy rule that refuses
  unbounded pop loops there, with a mutation in `test-realtime-policy.sh`. Keep the digests only as
  evidence that static rendering did not move.

### F6 (low): the console's resources are not all charged, and L1 attaches effect lanes it does not use

- **Reproduced.** Two `resource_lifecycle` oracles fail by 24 bytes per retained epoch (161,005 vs
  160,981; 199,934 vs 199,862).
- **What is not charged.** The `Vec<TrackControlProducer>` table and its `Box<str>` track ids are
  charged nowhere. The queue rings are charged on the builtin side.
- **Effect lanes.** `control_queue_depth` also attaches every effect lane: 15,361 bytes on the
  fixture, idle in L1.
- **Correction.**
  - Charge the table.
  - Decide whether L1 keeps the effect producers for L2 (and charges them), or adds a
    builtins-only console request.

### F7 (low): the lowering duplicates render-side domain checks

- **The duplication.** The lowering must refuse exactly what the setters refuse:
  - `checked_fader_gain`'s [-144, 24] dB range, which is private in `builtins`;
  - `Matrix2x2::checked`;
  - `pan_matrix`.
- **Failure scenario.** The two copies drift. Render pops a record that its setter then refuses
  (`map_err(render_error)?`), so the value is lost *and* the block fails.
- **Correction.**
  - Export the fader check from `builtins`, and call the same checkers from preparation and from
    the lowering.
  - Add a boundary test covering -144, 24, ±0, NaN and ±1.

### F8 (low): mobile apps must drain the reliable-event lane after every live edit

- **The cause.** Every live commit emits one `SESSION_COMMITTED`, and capi fixes the reliable-event
  capacity at 2 (`compile.rs:98`).
- **What the app sees.** An app that does not drain gets a protocol `BACKPRESSURE` *response* on
  the third edit. That response is replayed, so the app must use a new request id. This is the
  existing contract and not a loss, but a fan app dragging a fader at 60 Hz will hit it.
- **Correction.** Document it in the header comment and in `C_ABI_V1_QUALIFICATION.md`.

## 4. Item-by-item evidence

### Item 1: the prototype, rebuilt

- **Design.**
  - `live_builtin_delta(current, next, producers, fader_smoothing, max_smoothing)` masks `fader`
    and `matrix_or_pan` and requires the rest of the model to be equal.
  - Tracks are addressed by id, never by index.
  - Records go only for lanes that changed, with `Both` when both lanes end equal.
  - `fits()` counts per queue; `push()` returns a typed error.
- **capi.**
  - Checks run in this order: `compiled_model_admission`, then the room check on the newest epoch
    (pending, else current), then push, then commit.
  - The live arm runs after the existing `plan_alive` and response-size checks.

| test | result |
|---|---|
| Console-attached vs console-free, 12 blocks of non-trivial PCM | bit-identical |
| Mute all 9 tracks in one transaction | revision +1, no pending plan, model muted, next block exactly zero |
| Unmute from the same ring | bit-identical to the reference block |
| Exact replay | cached response; revision and queue room unchanged; no push |
| 16 un-rendered edits, then a 17th | `Backpressure`; transaction snapshot unchanged; the same request id succeeds after one render |
| Live mute, then a structural swap | the new plan is silent; a control run without the mute is not |
| Live edit while a candidate is pending (new) | lands only in the candidate (current queues untouched) and applies at its first block |
| Fault before the push, and after it (new) | before: nothing moves; after: no ack, but the value is applied (F4) |
| Classification edges (new) | identical values give a live edit with 0 records; 24.5 dB goes structural; a pan-to-matrix switch is live; an asymmetric fader edit gives 3 records; a trim edit goes structural |
| Two threads, 400 live transactions (fader and pan on 9 tracks) while render runs (new) | 0 allocations and 0 frees inside 470+ render calls; 90-134 submits overlapped a render call; the final block is bit-identical to a fresh plan compiled from the committed model; 20 of 20 runs |
| Two threads, live edits racing a structural swap (new) | render allocation-free; fails 14 of 20 runs on `INTERNAL` (F1); 20 of 20 with the L0 fix |
| `clippy --workspace --all-targets --all-features -D warnings`; wasm `simd128` check of `host-web` and `host-core` | pass |
| Descriptive, x86-64-v3 release: one 9-track live edit (compile, delta, push, commit) | about 98 µs |

**Note for L1.** A new test file that installs a counting allocator needs an `unsafe` exemption in
`check-realtime-policy.sh`. Put the two-thread test in `crates/capi/tests/resource_lifecycle.rs`,
which is already exempt, or justify a new exemption.

### Item 2: can an ack precede a drop?

| path | outcome |
|---|---|
| A push that fails after the room check | Not reachable. Session calls are serialized, so each queue has one producer, and `available_capacity` only grows until that producer pushes (`spsc.rs:326-343`). `fits` counts per queue, including the three-record asymmetric case. |
| A value render rejects | Refused at lowering: the edit goes structural and preparation reports the diagnostic. The copies can drift (F7). `LaneLength` cannot happen, because the lane comes from the binding. |
| Plan replacement | Queued records may die with the old plan, but the replacement is compiled from the committed model, and the test confirms this. |
| Epoch race | No drop. While a candidate is pending, records go to the candidate, which is adopted at the next block (retirement is reserved). With nothing pending, nothing can swap. A control call inside the swap window fails *before* the push (F1), so it acks nothing. |
| Pending candidate | Lands in the candidate; test above. |
| Replay | The cached response is returned before the live arm runs; nothing is pushed. |
| Reliable-event backpressure | `plan_structural_command` checks the room before preparing and returns a non-OK response. Nothing is pushed or committed. The response is replayed (F8). |
| Live-queue backpressure | `RESULT_BACKPRESSURE`, with no replay entry and no event. The same request id may retry. |
| Plan destroyed | `plan_alive` is false, so the edit returns `Backpressure` before the live arm runs. |
| Reset or restore | No capi path re-seeds the builtins from anything except the compiled model (section 2). |

### Item 3: the four drains (parallel pass)

- **Verdict: class A for the browser.** The digests and hashes are in F5.
- **Delivery modes.** `Concurrent` gets separate fader and matrix bank processors (or per-node
  ones). The fused `FaderMatrixBankProcessor` and the scalar pairs require `BetweenRenderCalls`
  (`builtins-compiler/src/lib.rs:1023`, `:4309`, `:4643`, `:4681`).
- **Bits against the browser.** Fused and unfused paths are documented class A. Every non-NaN word
  is identical (`lane/src/kernels/builtins.rs:300-306`, `:394-400`). So capi's `Concurrent` path
  renders the browser's bits, before any live edit, on the same backend.
- **Not an atomic batch.** #444's text rejects a "length snapshot" as a stand-in for block-atomic
  admission. L1 should record that it declines that contract knowingly.

### Item 4: deleting the unwired stack (parallel pass)

- **It compiles and passes.** The results and measured lines are in section 2.
- **Extra deletions the drafts did not list:**
  - protocol `queue.rs`: `release_automation_admission` and its retaining-dequeue helper;
  - two test-only decode counters in `controller.rs`;
  - 8 lines in host-core `lib.rs`;
  - the stale exemption in `check-realtime-policy.sh:27,31`.
- **Nothing else needs these files.** There are no references in the SDK, the workflows, fuzz
  targets, tools, hosts, `MUTATIONS.md` or `docs/` outside the handoffs.
- **Open specs that mention them:** #1023 (defers to draft 02), #1033, and #1034 (rebase on step 4).
- **Closing #140** loses the two outcomes in F3. Its browser outcomes (live effect spans, ramped
  fader and mute, admission) were delivered long ago.

### Item 5: D3, step or ramp

- **How it was measured.** Out-of-band energy (above 4 kHz), relative to the total, in an
  8,192-sample Hann window at 48 kHz, for a 220 Hz tone at -6 dBFS. The ramp is the engine's D11
  linear ramp, which snaps to the target on its last sample.

| event | step | 128-sample ramp (1 quantum, 2.7 ms) | 480-sample ramp (10 ms) |
|---|---:|---:|---:|
| mute at a block boundary | -31 dB | -70 dB | -84 dB |
| one -1 dB fader tick | -53 dB | -91 dB | -105 dB |
| 60 Hz drag of -1 dB ticks | -50 dB | -88 dB | -99 dB |

(The same window with no event measures -171 dB.)

- **What a fan hears.**
  - A step mute is a click: broadband energy only 31 dB below the tone.
  - A drag is audible zipper noise on sustained material (vocals, pads, bass) on earbuds.
  - A ramp of one quantum removes about 38 dB of that energy.
- **Bits.**
  - After a ramp ends, the gain equals the target exactly (the D11 snap). Every later block is
    therefore bit-identical to the step's.
  - During the ramp, the arithmetic is the same `FaderRampStage` code in both hosts. So a mobile
    edit matches the browser bit for bit whenever both apply the same records at the same block
    with the same `smoothingSamples`.
  - The SDK defaults to 0 (`sdk/src/core/console.ts:126`). A step is therefore "the browser's
    bits" only for a browser app that leaves the default, which is not a property of the engine.
  - No cross-host gate can compare a live transition without pinning the application block anyway,
    because each host picks that block by its own timing.
- **Advice.**
  - A fixed ramp of 5-10 ms, derived from the session rate at preparation (at least one quantum).
  - State it in the C header.
  - Set the SDK's fan-facing default to the same length, so web and mobile fans hear the same
    transition.
  - Pan already ramps by the session's declared `smoothing_samples`.

### Item 6: draft L1

- **Mostly small and bounded**: about 320 production lines, measured in scratch. It is
  self-contained except for F1, which it cannot pass without.
- **Gates that do not discriminate or are wrong:**
  - gate 1 (F2);
  - gate 3, fault injection (F4);
  - gate 5, digests as evidence for the drains (F5).
- **Gates that are missing:**
  - a two-thread swap race (F1);
  - a policy rule against unbounded drains (F5);
  - a domain-boundary test (F7);
  - the reliable-lane documentation (F8).
- The Amendments section under section 7 of `FINDINGS.md` gives the corrected scope.

## 5. Not verified

- **AArch64.** No `aarch64` target is installed here, and I did not install one. The prototype was
  checked on x86-64-v3 and wasm `simd128` only.
- **Mobile hardware.** No device or simulator run, no NEON bit-identity check, and no measure of the
  per-edit compile cost on a phone.
- **Listening.** Section 5's numbers are spectral measurements, not a blinded listening test.
- **L2 and L3 paths.** Effect parameters and bypass, and EQ prepared targets, were not prototyped.
