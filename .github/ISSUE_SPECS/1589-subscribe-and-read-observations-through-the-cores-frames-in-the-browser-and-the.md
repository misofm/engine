# Subscribe and read observations through the core's frames in the browser and the SDK

Slice S27a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C9 and C13). Split from the planner's S27: this slice moves the browser and the SDK onto the core's
`Observe` edit and `OBSERVATION_READ`; #1590 deletes what is left of the browser's own observation
path and the SDK's own sharing. Code anchors verified on `main` `df585e503`. Feature sprint mode
(owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or
mutation runs.

## Product outcome

A browser or headless app subscribes to observation taps exactly as a C ABI app does: the SDK
sends `Observe` edits in a `SESSION_TRANSACTION_APPLY` frame and reads with `OBSERVATION_READ`,
both through the core's command entry. A subscription survives plan rebuilds in the browser too.
`ObservationSubscription.armed()` resolves with the sample at which every selection of the
subscription is in effect, from the arm tag that render published. The browser's command kinds 7
and 8 and its last command export are gone.

## Context

- **The SDK's path today.** `ObservationSubscriptionOwner` (`sdk/src/core/observation-subscriptions.ts:471`)
  builds kind-7/8 lane edits (`#edit`, `:866-879`, through `EffectEdits.observe`,
  `sdk/src/core/live-controls.ts:989`) and submits them (`#submit`, `:881`); it reads through
  `ObservationSubscriptionTransport.readObservations` (`:141-160`), which the browser serves
  through the worklet's read (`sdk/src/browser/engine.ts:607-630`;
  `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1125`) and the headless engine through the
  module's read export (`sdk/src/core/boundary.ts:706`). Receipts carry `appliedAtSample`
  (`observation-subscriptions.ts:107`, `:118`) from the command report.
- **What #1565 leaves.** Kinds 7 and 8 (`hosts/host-web/src/lib.rs:861`, `:863`) are the only
  command kinds; `admit_commands_staged` keeps only the observation path; the export
  `miso_engine_web_v1_command_submit` (`hosts/host-web/src/ffi.rs:4015`) runs them on the worklet
  thread; the staging is allocated only with observation taps (#1565 D4).
- **The core path.** #1584 (`Observe` edit, the binding table, capacity on every plan, and its D7a:
  the browser refuses `Observe` with `observation.unavailable` until this slice), #1585 (arm state in the slot, arm tags in the reply),
  #1586 (`OBSERVATION_READ`, `ObservationIndex`, read-only observers). #1562 D1 gives the browser
  one command entry (`miso_engine_web_v1_control_*` and the internal `control` port message), and
  #1563 gives the SDK the frame codec (`sdk/src/core/control-frame.ts`).
- **The settle rules.** The #1057 design note, section 8.4, "How `armed()` settles"
  (`docs/handoffs/one-edit-api/1057-design-note.md:1457`).
- **Browser qualification.** *Qualify the browser observation wait against the real AudioWorklet
  in three browsers* (#1530) and *Qualify observation unsubscribe on a real AudioContext in three
  browsers* (#1535) add real-browser gates on today's path.

## Decisions frozen for this slice

- **D1. The browser's control half owns observation.** The browser's preparer keeps every plan's
  effect observation handles: the acknowledging readers go with the render half to #1582's meter
  fold, and the control half's `SessionState` builds #1586's `ObservationIndex` from observers.
  Every browser plan carries observation capacity for every declared tap (#1584 D7). The browser's
  control half sets `ProviderFeatures::observation` (#1584 D7a) and capability bit 15 (#1586), so
  it accepts `Observe` and serves `OBSERVATION_READ`.
- **D2. Kinds 7 and 8 go.** `COMMAND_OBSERVE_SUBSCRIBE`, `COMMAND_OBSERVE_UNSUBSCRIBE`, what is
  left of `admit_commands_staged`, `into_observe_record`, the command staging buffer
  (`BUFFER_COMMAND`; the value 6 is never reused), `miso_engine_web_v1_command_submit`, `miso_engine_web_v1_command_report_ptr`,
  `WebCommandReport` and every `COMMAND_*` and `COMMAND_REASON_*` constant are deleted, with their
  rows in `tools/parameter-metadata/src/abi_layout.rs`, `tools/realtime-policy/render-thread-exports.json`,
  `scripts/check-web-audioworklet.sh` (the `command_submit` allocation leg), the shipped host's
  `.d.ts` twins and `scripts/test-web-audioworklet.mjs`. One producer now writes each observation
  queue: the control half.
- **D3. The SDK sends frames.** `ObservationSubscriptionTransport` becomes
  `{ apply(edits: readonly ObserveEdit[]): Promise<ObserveApplyReply>; read(selections):
  Promise<ObservationReadPage>; ... }`, implemented by the browser engine over the `control`
  message and by the headless engine over the module's control entry, both with #1563's codec.
  `ObserveEdit` addresses by strip ID and console slot or insert ID (never an index). The reply
  gives `observe_arm_tags` and `removed_arm_tags` (#1585 D5). Every typed refusal becomes a
  `MisoEngineError` whose diagnostic is the core's code; `observation.backpressure` keeps today's
  retry rule (the browser waits for one render up to `requestDeadlineMs`; headless throws at once).
- **D4. `armed()` replaces `appliedAtSample`.** `ObservationSubscription` and its receipt lose
  `appliedAtSample` and gain `armed(): Promise<bigint>`, which settles once, by section 8.4's
  rules: it resolves when, for every selection, a read reports the binding's tag armed, with the
  largest of their arm samples; it rejects with `wrongState` and `sdk.observation.closed` on close
  first, `sdk.engine.disposed` on disposal, `sdk.observation.owner_removed` when a reply lists the
  binding's tag as removed, and with `internal` and `sdk.observation.superseded` when a read shows
  a newer tag on a pending binding. `readLatest()` and the notifications read rows from
  `OBSERVATION_READ` (its `arm_tag`, `arm_sample` and window fields).
- **D5. The sharing stays in the SDK for now.** The owner still counts references per tap and
  window, so each SDK binding holds exactly one core reference. #1590 deletes it.
- **D6. The parity script.** `hosts/host-web/tests/protocol_cross_host.rs` (#1570) gains one
  `Observe` arm, two render blocks and one `OBSERVATION_READ`; both legs return byte-identical
  frames.

## Deliverables

1. D1 in the browser preparer and `hosts/host-web/src/lib.rs`; D2 in host-web, the worklet
   scripts, the checkers and the generated mirrors.
2. D3-D5 in `sdk/src/core/observation-subscriptions.ts`, `sdk/src/core/observation.ts`,
   `sdk/src/browser/engine.ts`, `sdk/src/headless/engine.ts`, `sdk/src/core/boundary.ts`.
3. D6; the SDK evals; the host-web tests that used kinds 7 and 8, each rewritten to send `Observe`
   through #1562's entry and keep its claim, or deleted because a core test holds the claim (the
   PR lists each one).

## Authorized paths

- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/web/*.js`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `hosts/host-web/tests/protocol_cross_host.rs`,
  `hosts/host-web/qualification/{qualification.js,run.mjs}`.
- `crates/control-plane/src/control.rs` and the browser's provider features (D1).
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`,
  `tools/realtime-policy/render-thread-exports.json`, `scripts/check-web-audioworklet.sh`,
  `scripts/check-abi-layout-v1.py` and its self-test fixture, `scripts/test-web-audioworklet.mjs`.
- `sdk/src/**` (observation files, the two engines, `boundary.ts`, generated files by regeneration
  only), `sdk/test/{observation-evals.mjs,observation-subscription-evals.mjs,browser-evals.mjs,headless-path-evals.mjs}`,
  `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated).

## Non-goals

- No deletion of the observation map and read exports or of the `Web*Observation*` records
  (#1590). No change to the meter lease or the meter frame.
- No new browser export.

## Objective gates

Commands: `cargo test --locked -p host-web --all-features`, `cargo test --locked -p control-plane --features test-support`,
`cargo run --locked --release -q -p realtime-policy`, #1534 gate 3's worklet commands,
`bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh <artifacts>`,
`bash scripts/check-sdk-generated.sh <artifacts>`, `python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check <artifacts>`, and the browser qualification run of #1530 and
#1535 on Chromium, Firefox and WebKit.

1. **Same frames on both hosts.** D6's step is byte-identical on the two legs.
2. **armed().** On the headless engine: `subscribe` returns before any render; `armed()` stays
   pending until a render applies the arm, then resolves with the arm sample the read reports. Close
   before that rejects `sdk.observation.closed`; removing the effect in a transaction rejects
   `sdk.observation.owner_removed`.
3. **Rebuilds keep subscriptions in the browser.** A subscribed compressor tap keeps reporting after
   a structural transaction that adds a track (the windows continue), on the headless engine and in
   the browser evals.
4. **Old kinds refused.** A command record with kind 7 or 8 cannot be submitted: the export is gone
   from the shipped module's export set (`check-web-audioworklet.sh`'s exact list).
5. **Real browsers.** #1530's observation-wait gate and #1535's unsubscribe gate pass on the new
   path in three browsers.
6. **Realtime.** The worklet's render-locked allocation count stays exactly zero over the
   qualification runs.

## Test value

- Gate 1 turns red if the browser and the C ABI answer the same subscription differently.
- Gate 2 turns red if `armed()` resolves before render applied the arm or on another arm's tag.
- Gate 3 turns red if the browser still drops subscriptions at a rebuild.
- Gate 4 turns red if a second producer can still write an observation queue.
- Gate 5 turns red if the frame path regresses the real-context wait or unsubscribe.
- Gate 6 turns red if the switch adds an allocation to the audio thread.

## Dependencies

- #1584, #1585, #1586; #1562 (the command entry), #1565 (kinds 7 and 8 left alone), #1563 (the
  frame codec), #1570 (the parity script).
- #1530 and #1535 (their gates exist first; this slice keeps them green).
- `hosts/host-web/src/lib.rs` is stream H's file; `sdk/src/browser/engine.ts` follows `STREAMS.md`
  row 104. This slice rebases after them.

## Supersedes and shrinks

- None.

## Decision record

- **Split (scope note, 2026-10-25).** #1589a: the browser's session serves `Observe` and
  `OBSERVATION_READ` (D1, D6, gate 1). #1589b: the SDK on frames (D3-D5; gates 2, 3, 5, 6). #1589c:
  kinds 7 and 8 deleted (D2; gate 4). Order a -> b -> c.
- **R1 (root).** The boot word `live_control_observation_taps` keeps its meaning until #1590 D5;
  D1's "capacity for every declared tap" is #1590's. #1589a: the session serves the edit and the
  read only when the browser's preparer binds a queue for every declared tap
  (`RuntimePreparer::observation_queues`: a nonzero word). Preparation refuses an effect that
  declares more taps than the word (`effect.observation.taps`), so a nonzero word binds every
  declared tap of every plan. Otherwise `Observe` is refused `observation.unavailable` and
  neither `0x000f` nor bit 15 is advertised: with the word `0` the live path would have answered an
  internal fault and the rebuild path would have acknowledged a subscription no queue holds.
- **R2 (root).** One `ObservationIndex` per plan, the epoch's; #1586a's `PlanShape` copy is deleted,
  and its bytes move to the control plane's row. The browser's preparer keeps its readers in its
  plan's `RenderCompanions` (D1: the render half pairs them at the swap block and folds their gain
  reduction), so `prepare_runtime` builds the epoch's index over that list
  (`RuntimePreparer::kept_observations`). The browser's control entry lends the active plan's
  readers with that plan's epoch (`SessionState::command_with_plan_observations`), and the kept read
  exports read through `SessionState::observation_index(epoch)` for the epoch of the rendering
  plan, so an index and its readers are always one plan's.
- **Carried readers (#1674, #1587b).** The C ABI pairs its epoch's readers at promotion from the swap
  record. The browser does not need that: its readers live in the plan and its own swap hook
  (`RenderCompanions::adopt_predecessor`) pairs them in the swap block, so a carried tap's reader is
  the one render publishes to from the adoption block on; the core read reads those readers, so a
  carried tap does not read `Pending` after a rebuild.
- **Gate 1 (D6).** `protocol_cross_host.rs`'s one-script gate runs on the observation fixture with
  taps for every declared tap and gains steps 16-18 (one `Observe` arm, two blocks, one
  `OBSERVATION_READ`, a ready row); step 1's exception loses `0x000f` and bit 15.

## Decision record: slice #1589b (2026-10-25)

- **D3 as built.** `ObservationSubscriptionTransport` is `{ observationMap, apply, read, sampleRateHz,
  observationEnded?, renderWait?, ... }`; `liveControls()` and `readObservations()` are gone from it.
  `observationFrameTransport(endpoint)` (`sdk/src/core/observation-subscriptions.ts`) builds `apply`
  and `read` from an engine's command entry, the same on both engines: headless over
  `controlFrame` (the module's control entry), browser over the worklet's `control` message, each
  with its endpoint's one request-ID allocator. The codec is in `sdk/src/core/control-frame.ts`
  (`encodeObserveTransaction`, `encodeObservationRead`, `decodeObservationReadReply`), held to
  `crates/protocol/tests/fixtures/observation-frames.json` (`observation_frames.rs`, a byte gate on
  the model of #1563 and #1596b; the reply with arm tags is #1563's fixture). The observation map
  stays (#1590): it checks a selection and names its effect before any edit.
- **Refusals.** A refused reply raises `MisoEngineError` with `code` the result class of its status
  (`spectrumRefusalResult`, the mapping #1596b uses) and `diagnosticCode` the core's code; a failed
  call (`backpressure`, a full frame) is `controlCallFailure`'s error. The retry rule is kept: only
  `code: "backpressure"` waits for a render (browser, up to `requestDeadlineMs`); headless throws.
- **`revision_conflict` resend (new).** An `Observe` transaction needs the exact committed revision,
  and the browser reads it from its queue (`queue.revision`, noted from every reply), so another
  sender's commit can race it. An `Observe` writes no model value, so the same edits at the
  revision the refusal reports are the same request: it is resent, at most 8 times. Headless reads
  the core's revision in the same call and never conflicts.
- **Window change.** An `update` that changes the window of a tap the handle alone holds sends the
  disarm of the old window and the arm of the new one in one transaction (the core's one re-arm,
  #1584 gate 1); kinds 7/8 sent the arm alone.
- **D4 as built.** `armed()` per configuration, by section 8.4: a binding records the tag its arm
  edit got (`observeArmTags`, one per edit, checked); a read that reports `armed` with that tag sets
  the binding's arm sample; the handle resolves with the largest once every selection has one. A
  read of another tag never resolves it, and a `ready` row of another tag reads `pending`; a newer
  tag on a pending binding rejects `internal` `sdk.observation.superseded` (unreachable by the
  reference rule; the subscription is left as it is). `armed()` settles only at a read: `pump()`,
  and in the browser the delivery timer. Close and an `update` that changes the selections or the
  window reject `sdk.observation.closed`; dispose rejects `sdk.engine.disposed`; a headless
  `loadSession` rejects `sdk.observation.closed` (the handles close; the engine is not disposed).
- **Owner removal from the event.** `sdk.observation.owner_removed` is set by `OBSERVATION_ENDED`,
  not by a reply's `removedArmTags`: the removing commit is any sender's (a structural `apply`, a
  replace, a frame the app sent itself), and only the event reaches the owner for all of them. The
  event's revision filter now uses the arm reply's revision.
- **Reads.** A read of more bindings than one reply frame holds (`64 + 192 n`, #1586 D4; 85 rows at
  the browser's 16 KiB frame) is sent as several `OBSERVATION_READ`s, in order. An `Observe`
  transaction over the endpoint's frame limit is refused `MisoUsageError` before it is sent (a
  stated limit: about 80 selections per subscribe or update in the browser, where kinds 7/8 took
  256 records).
- **D5.** The SDK's reference counting stays; each SDK binding holds one core reference.
- **SDK public changes.** `ObservationSubscription.appliedAtSample` and
  `ObservationSubscriptionReceipt.appliedAtSample` are deleted; both gain `armed(): Promise<bigint>`.
  `ObservationSubscriptionBounds.maximumCommandRecords` is deleted (the core's frame and edit limits
  bound a transaction now). `ObservationSubscriptionTransport` changes as above;
  `observationFrameTransport` and `ObservationFrameEndpoint`, `ObserveApplied` are new exports of
  the module (not of the package entries). `EngineLiveControls.submitManaged` is deleted, and
  `LiveControlBeforeSubmit` loses its `managed` argument. The browser engine no longer refuses a
  subscription with `MisoUsageError` when `policy.liveControls.observationTaps` is 0: the map of a
  session that binds no tap has no binding, so the subscription is refused `MisoEngineError`
  `unsupported` `sdk.observation.selection` (headless already did). A preflight that finds the tap
  armed by another subscriber is still `MisoUsageError` (message reworded). The public
  `readObservations()` of both engines keeps the numeric read export (#1590).
- **Qualification.** #1530's wait rows log the `Observe` transactions through `host.control` (call
  result, reply status, the edits' bytes); #1535's unsubscribe run arms and disarms with
  `observeThroughSdk` (the SDK codec, in `sdk-response-entry.ts`) and its ack gate is the reply's
  `monitor` path; the resident row adds `sdk-observation-armed` (R4). `sdk-response-entry.ts` is
  outside the scope note's paths: its #1530 logger and resident row read `host.command` and
  `appliedAtSample`, which this slice removes from the SDK path. Kinds 7/8 stay for #1589c (the E12
  and staging-read rows still use `host.observe`).
