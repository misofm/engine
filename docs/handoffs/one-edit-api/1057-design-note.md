# Design note: one edit API on every host (#1057)

Issue: *Design: one edit API on every host over the core's committed session model* (#1057),
stream K of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
D15-11, also D15-2, D15-3, D15-10 and D15-17). Attempt 1.

**Commit.** Every `path:line` in this note was checked on `8be19c86e`
(`8be19c86e4c8d16530f422d430cfc211c71db5d3`, `main`). The spec's anchors were written on
`6fb211594`. Between the two commits the only code change is
`crates/capi/tests/resource_lifecycle.rs`, so every spec anchor is unchanged; each one was read
again on `8be19c86e`.

**Host for every measurement.** `Linux devbox 6.8.0-139-generic #139-Ubuntu SMP PREEMPT_DYNAMIC
x86_64`, AMD EPYC 7313P 16-Core Processor (32 threads), 503 GiB RAM. Other worktrees built at the
same time: the load average was 18 to 48. Every number is therefore *uncontrolled* and
descriptive (AGENTS.md benchmark rules: one invocation, one warmup, two measured rounds, no retry).

**Authority.** Decisions D1-D7 of the spec are decision 15's (root, under the owner's delegation).
This note records them and does not reopen them. The other choices here are design decisions for
the implementing issues under D15-11. The personal-mix question (section 7) stays open for the
owner. Findings that touch a decided item go to the coordinator (section 6.4).

**The design in brief.**

- One edit format on both hosts: the protocol's `SESSION_TRANSACTION_APPLY` frame. One commit
  function: the portable crate's session apply (`control-plane`, #1309).
- The SDK's live controls become thin builders of transaction edits, addressed by stable ID. The
  measured live-edit hop gives no reason for a second path (section 3). To make this complete, the
  transaction gains lane-granular setters for every live value (proposed issue P1).
- Solo and observation subscriptions are host-local monitoring state. They travel with the edit
  through the control plane, keyed by stable ID, and the control plane composes them into every
  plan it publishes. The meter lease stays with the render half, where its state is (section 8).
- A save is a read of the committed model on the control thread or in the Worker. Render never
  takes part.
- Measured costs: the control plane adds about 0.33 MB (11 %) to the shipped module; the hop of a
  live edit is 2.7-7.5 ms (p50) in a Chromium Worker on the 64-track documents. That cost is the shared
  apply, not the transport. On a page that is not cross-origin isolated the same apply runs on the
  audio thread and is longer than one quantum (finding F1, proposed issue P2).

## 1. The edit path on each host

Three edit kinds use one path. The classifier decides the kind after validation:
`host_core::classify_live_delta` (`crates/host-core/src/live_delta.rs:211`). The path value is
exactly one of `live`, `model_only` and `rebuild` (D15-17, #1313).

The functions below are on `main` unless an issue is named. "(→ #N)" names the issue that changes
or adds the step. After *Extract the C ABI control plane into a portable crate both hosts call*
(#1309), every `crates/capi/src/runtime/*` function below lives in `crates/control-plane` with the
same name.

### 1.1 C ABI

| Hop | Step | Owner (crate::function, anchor) | Kinds |
|---|---|---|---|
| C1 | The host calls the session entry point. | capi `miso_engine_v1_submit_command` (`crates/capi/src/ffi.rs:635`) | all |
| C2 | The adapter enters the control plane. The service step runs first. | `SessionState::command` (`crates/capi/src/runtime/control.rs:843`); `synchronize_plan_epochs` (`:783`) and `collect_render_activity` (`:522`) become `SessionState::service` (→ #1348) | all |
| C3 | Decode, replay check, reliable-event room and reservation. | protocol `ProtocolController::prepare_command_frame` (`crates/protocol/src/controller.rs:1566`); room check `:1735-1744`; reservation `:1639` | all |
| C4 | Build and compile the prospective model. | protocol `SessionStore::prepare_transaction` (`crates/protocol/src/model.rs:917`); session `compile_session` (`crates/session/src/compile.rs:123`), which writes the canonical JSON (`:130`) | all |
| C5 | Classify. | `commit_live` (`crates/capi/src/runtime/control.rs:1065`) calls `classify_live_delta` (`:1071`) | all |
| C6L | Live: every check, then the lane writes. | `commit_live` steps 1-6 (`crates/capi/src/runtime/control.rs:1079-1335`); latest-target cells replace the queues (→ #1312, #1345-#1347) | live |
| C6S | Rebuild: prepare the successor, admit it, take over any pending candidate, reserve, commit, move the producers, publish. | `prepare_runtime` (`crates/capi/src/runtime/compile.rs:572`; through the adapter's `RuntimePreparer`, → #1400); `validate_replacement_peak` (`crates/capi/src/runtime/control.rs:939`); a pending candidate is refused today (`crates/capi/src/runtime/control.rs:959-961`) and superseded by compare-and-swap later (→ #1343, #1344, #1310); `reserve_replacement` (`crates/engine/src/realtime/plan_exchange.rs:265`); protocol commit (`crates/capi/src/runtime/control.rs:1008`); `adopt_persisting` (`:1019`); `reservation.commit()` (`:1025`) | rebuild |
| C7 | Commit: install the model, fill the reserved `SESSION_COMMITTED` slot, install the replay entry. Write the path into the response. Store the revision word last. | `ObservedPreparedToken::commit` (`crates/capi/src/runtime/control.rs:353`) → `commit_prepared_structural` (`crates/protocol/src/controller.rs:1974`); path (→ #1313 D3); revision word (→ #1314 D2) | all |
| C8 | Return the ack: the response frame with `{revision, path}`. | `SessionState::command` returns the length; capi copies the bytes | all |
| C9 | Render adopts a published plan at block entry, reads cells, publishes the watermark. | `miso_engine_v1_render_f32_planar` (`crates/capi/src/ffi.rs:807`) → `PlanState::render` (`crates/capi/src/runtime/plan.rs:201`) → `RealtimePlanOwner::enter_block` (`crates/engine/src/realtime/plan_exchange.rs:375`); watermark (→ #1314 D3) | all |
| C10 | The host observes completion and drains events. The service step reclaims retired plans. | `miso_engine_v1_plan_watermark` (→ #1314 D6); `miso_engine_v1_service` (→ #1348); `miso_engine_v1_dequeue_event` (`crates/capi/src/ffi.rs:725`); `PlanRetirer::try_reclaim` (`crates/engine/src/realtime/plan_exchange.rs:512`) | all |
| C11 | Save: page the canonical snapshot. | `SessionSnapshotGet` (`crates/protocol/src/controller.rs:2851`) of `SessionStore::canonical_snapshot` (`crates/protocol/src/model.rs:900`) | — |

Per kind:

- **Live.** C1-C5, C6L, C7, C8. Render applies the cells at the next block, or at the adoption of a
  pending candidate, whose cells take the edit (#1053 D7, D15-17).
- **Model-only.** C1-C5, C7, C8. The delta holds no record (`LiveDelta::is_empty`, → #1313 D2).
  The revision word goes to the pending candidate if there is one, else to the active plan, so
  render reports it at its next block (→ #1314 D2).
- **Rebuild.** C1-C5, C6S, C7, C8. Render adopts the successor at a block boundary; the warm
  successor and the transitions are streams C and D.

### 1.2 Browser

Worker mode (cross-origin isolated). Single mode runs W3-W5 in the worklet's message handler,
never inside `process()` (D15-10, #1332 D1).

| Hop | Step | Owner | Kinds |
|---|---|---|---|
| W1 | The app calls the SDK. The SDK keeps one control message in flight and queues later calls in call order (section 3.4). | `engine.apply`, `engine.replaceSession`, `EngineLiveControls.submit` (→ #1296, P6) | all |
| W2 | The SDK posts the frame and any monitoring operations on the page-to-control-plane port. | `miso.apply.v1` / `miso.replace.v1` (→ #1294 D1, extended in section 3.3); port (→ #1332 D7) | all |
| W3 | The Worker stages the bytes and calls the apply export, which calls the portable crate. C2-C7 run unchanged. | `miso_engine_web_v1_edit_ptr`, `miso_engine_web_v1_apply` (→ #1293 D1-D2), `miso_engine_web_v1_replace` (→ #1386); `control_plane::SessionState` (→ #1309, #1381, #1382) | all |
| W4 | Cell writes and plan publication go into the one shared `WebAssembly.Memory`. | the same writers as C6L and C6S; shared memory (→ #1380, #1332) | live, rebuild |
| W5 | The Worker drains the reliable event lane, announces the commit to the worklet, then replies. | drain (→ P4); `miso.committed.v1` and `miso.plan-shape.v1` (→ #1294 D4); reply `miso.edit.v1 {result, revision, path, diagnostic}` (→ #1294 D1) | all |
| W6 | The worklet renders through the plan owner, adopts at a block boundary, pairs the companions, publishes the watermark into the status words. | `render_next` (`hosts/host-web/src/lib.rs:3212`) through `RealtimePlanOwner` (→ #1381 D4); companions (→ #1381 D3); status words (→ #1349) | all |
| W7 | The Worker's service loop reads the watermark and posts it to the page; the SDK resolves `applied(revision)`. | `miso_engine_web_v1_service`, `miso_engine_web_v1_watermark_read` (→ #1381 D5-D6); `miso.watermark.v1` (→ #1294 D3) | all |
| W8 | Save. | `engine.snapshot()` (→ P3) over `miso.document.v1` and `miso_engine_web_v1_committed_document_ptr` (→ #1293 D4, #1294 D1) | — |

Today none of W2-W8 exists. The worklet admits 48-byte command records itself
(`receiveCommand`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1526`;
`miso_engine_web_v1_command_submit`, `hosts/host-web/src/ffi.rs:3839`;
`AudioWorkletEngineHost::submit_commands`, `hosts/host-web/src/lib.rs:3322`), and the main realm
designs EQ targets in a second Wasm instance (`#ensurePreparedControl`,
`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:781`;
`miso_engine_web_v1_prepared_command_submit`, `hosts/host-web/src/ffi.rs:3847`). Both are replaced (finding F6).

### 1.3 The acked-batch question: can an ack ever precede a drop?

**No, on every hop, on both hosts**, given the conditions named in the last column.

| Hop | Answer | Why |
|---|---|---|
| C1, W1 | No. | C1 is a synchronous call; its return value is the ack. W1: the SDK resolves a call only from the reply to the message that carried it. A queued call is not acked. A refused batch is sent again call by call, so each call gets its own result (section 3.4). |
| W2 | No. | A port message queue is a task source: messages are delivered in order, and the standard defines no capacity limit and no drop [S6]. If the Worker fails, the SDK rejects every pending call and resolves none. Before boot the reason is `worker-failed` (#1332 D6). After boot a Worker `error` or `messageerror` fails the engine the same way the worklet port does today (`#fail`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:775-778`). |
| C2 | No. | The service step commits nothing and acknowledges nothing (#1348 D6). |
| C3 | No. | Every refusal returns before the session changes. The `SESSION_COMMITTED` slot is reserved before the commit (`crates/protocol/src/controller.rs:1639`), and a full lane refuses the edit first (`:1735-1744`). |
| C4, C5 | No. | The prospective model is private to the call. A refusal drops it with nothing acked. |
| C6L, W4 | No. | Every fallible check comes before the first write; cell writes cannot fail (#1312 D1, D7; contract at `crates/capi/src/runtime/control.rs:1039-1062`). A value that a later commit replaces before render reads it is counted, and the model holds the newer value (D15-2). |
| C6S, W4 | No. | Preparation, admission and reservation fail before the commit. After it, only infallible steps run (`crates/capi/src/runtime/control.rs:1008-1025`). A pending candidate is superseded, never dropped: its revision completes `superseded` and its content is in the newer model (D15-9, #1310). |
| C7, C8 | No. | After `check_prepared_structural` (`crates/protocol/src/controller.rs:1954`) the commit cannot fail. The path is written inside it (#1313 D6). The ack is written after it. |
| W3 | No. | The same code as C2-C8. Monitoring operations are checked with the transaction, all or nothing (#1382 D4). |
| W5 | No. | The reply is posted after the export returned. Announcements to the worklet carry no command; a late one makes the worklet wait for a meter shape (#1294 D5). The Worker drains the reliable lane after each commit (P4). Without that drain, the third commit is refused at C3, before anything changes. |
| C9, W6 | No. | Render never drops a candidate or a cell value. A full retirement queue defers the swap (AGENTS.md). A withdrawn candidate goes back to the control side (#1343). |
| C10, W7 | No. | The watermark is a level that render overwrites; outcome flags are an OR (#1314 D5). Posts to the page coalesce to the latest value with the OR of the flags (#1294 D3). For the question "is revision R in effect?" a level loses nothing. |
| C11, W8 | No. | Read only. |
| Monitoring | No, with P5. | Solo writes strip mute cells. An observation subscription pushes into an effect lane after a room check. Across a swap the control plane composes the overlay into the successor before it publishes it (P5). Without P5, an acked solo or subscription would be lost at a rebuild. P5 closes that hole. |

Source submissions share the port but are not edits. A submission is acked only after its chunk is
in the ring, and a full ring returns `BACKPRESSURE` with nothing written (#1387 D7).

### 1.4 Sources

- [S1] W3C, *Web Audio API 1.1*, §2.6 "Rendering an Audio Graph": for each render quantum the
  rendering thread first processes the control message queue, then the context's associated task
  queue (every task posted from an `AudioWorkletNode`), then renders. §7.4 "Audio Glitching": a
  glitch is a catastrophic failure and must be avoided. Default render quantum: 128 frames.
  <https://www.w3.org/TR/webaudio-1.1/#rendering-loop>
- [S2] WHATWG HTML, §8.1.2.1 "Integration with the JavaScript agent formalism": a window agent and
  a worklet agent have `[[CanBlock]]` false; dedicated and shared worker agents have it true.
  <https://html.spec.whatwg.org/multipage/webappapis.html#integration-with-the-javascript-agent-formalism>
- [S3] WHATWG HTML, StructuredSerializeInternal: serializing a `SharedArrayBuffer` throws
  `DataCloneError` when the cross-origin isolated capability is false; §8.1.3.2 defines that
  capability. <https://html.spec.whatwg.org/multipage/structured-data.html#structuredserializeinternal>
- [S4] ECMA-262, `Atomics.wait`: throws a `TypeError` when the agent cannot suspend.
  <https://tc39.es/ecma262/#sec-atomics.wait>
- [S5] WebAssembly threads proposal: a shared memory must declare a maximum (the JS API
  `WebAssembly.Memory` constructor throws a `TypeError` otherwise, §4.3); growth never detaches
  the old `SharedArrayBuffer`, and `memory.buffer` then returns a new one over the same bytes.
  <https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md>,
  <https://webassembly.github.io/threads/js-api/>
- [S6] WHATWG HTML, §9.4.4 "Message ports": each port has a port message queue, a task source;
  messages are queued as tasks in order. <https://html.spec.whatwg.org/multipage/web-messaging.html#message-ports>
- [S7] W3C, *High Resolution Time*, §4: time is coarsened to 100 µs, or 5 µs when the context is
  cross-origin isolated; `timeOrigin + now()` puts a worker and a window on one timeline (§1.2).
  <https://www.w3.org/TR/hr-time-3/>

Consequences for this design: the Worker may block and wait; the main thread and the worklet may
not [S2, S4]. So render polls cells at block entry and never waits (D15-10 gate 2). Shared memory
needs isolation [S3]; a non-isolated page has single mode. A message handler on the worklet runs on
the rendering thread before the next quantum [S1]: any control work there delays render (F1).

## 2. The portable crate's boundary

The crate is `control-plane` (lib `control_plane`, #1309 D1). Both adapters call the same
functions with the same bytes. The table is the contract #1309 and #1332 implement.

### 2.1 What an adapter passes in and gets back

| Call | In | Out | Adds it |
|---|---|---|---|
| Boot | the session document bytes; `ControlLimits` (#1309 D3); `Box<dyn RuntimePreparer>` (#1400 D1); the adapter's fixed allocation sizes (#1309 D5) | `SessionState` (control half) and `PlanState` (render half, `crates/capi/src/runtime/plan.rs:53`), or a typed `CompileFailure` (#1309 D6) | #1309, #1400 |
| Command | one protocol request frame (any command; for an edit, `SESSION_TRANSACTION_APPLY`, the exact bytes the C ABI takes) | the response frame length, the frame in the crate's scratch; typed `CommandError` on refusal | #1309 (`command`, `crates/capi/src/runtime/control.rs:843`) |
| Command with monitoring | an optional request frame plus a list of `MonitorOp` (section 8.2); at least one of the two | `CommandReply { response_len, applied: Option<Applied { revision, path }> }`; `applied` is `None` when no transaction was in the call | #1382 D3 |
| Service | nothing | `Ok` or an internal error; it commits and acks nothing | #1348 D1 |
| Watermark | nothing | `PlanWatermark { revision, first_sample, outcome_flags, exact, transition_fallback, superseded }` or busy | #1314 D4, #1381 D5 |
| Events | lane | one event frame or none | `SessionState::dequeue_event` (`crates/capi/src/runtime/control.rs:1392`) |
| Snapshot, paged | `SessionSnapshotGet {offset, maximum_bytes}` through Command | pages; every page header carries the session revision (`crates/protocol/src/controller.rs:3108-3109`) | exists |
| Snapshot, whole | nothing | `(revision, &str)` borrowed from `SessionStore::canonical_snapshot` (`crates/protocol/src/model.rs:900`), valid until the next commit | #1293 D4 |
| Replace by document | document bytes | the same `CommandReply` as Command with monitoring | #1386, in the crate (F7) |
| Sources | submit, seek, `seek_at`, seek report | as today (`crates/capi/src/runtime/control.rs:1501-1552`) | #1309, #1316, #1387 |

`Command` stays exactly as the C ABI uses it, so no C ABI byte moves. `Command with monitoring`
is the same function with two more inputs; `Command` calls it with no monitoring operation. The
typed `Applied` saves the browser adapter from decoding its own crate's response to fill the
outcome record (#1293 D3).

### 2.2 What stays in capi

The FFI and its pointer checks; the ABI structs (`CompileLimits`, `PlanResourceReport`, `BytesOut`,
...); `plan_error` and `FixedBytes`; `limits_are_valid` (`crates/capi/src/runtime/compile.rs:510`); the conversions
`CompileLimits → ControlLimits`, `PlanResources → PlanResourceReport` and typed failures →
`capi.*` bytes (#1309 D3, D4, D6); `CapiPreparer` with `C_ABI_LIVE_LANES` and `LIVE_QUEUE_DEPTH`
(#1400 D3); the header and its prose. The C ABI passes no monitoring operation.

### 2.3 What stays in host-web

The Wasm exports and their staging buffers; the fixed `#[repr(C)]` records (status, edit outcome,
watermark, seek report); the Worker, worklet and main-realm JavaScript; the host hand-off
(`host_release`, `host_adopt`, #1332 D2); `WebRuntimePreparer` and `RenderCompanions` (#1400 D5,
#1381 D3); the render-half delivery of meters, observations, track responses and spectrum; the
meter lease and the telemetry lease (section 8); the browser's `ControlLimits` (F10).

What does not stay in host-web: the lowering of command records (retired, F4), the composition of
solo, VCA and follow mutes (moved into the crate by #1382 D3 and D5), and the document diff (F7).
No browser-only session logic enters the crate (D1): the monitoring overlay is host-agnostic code
that the C ABI simply does not use.

## 3. The SDK surface

### 3.1 Signatures

The types live in `sdk/src/core/` so that the headless engine (#1389) has the same surface.

```ts
export type EditPath = "live" | "model_only" | "rebuild";

export interface EditOutcome {
  readonly revision: bigint;          // the committed revision
  readonly path: EditPath;
}

export interface OutcomeFlags {
  readonly exact: boolean;
  readonly transitionFallback: boolean;
  readonly superseded: boolean;
}

export interface Watermark {
  readonly revision: bigint;          // highest revision in effect with every earlier one
  readonly sample: bigint;            // first sample fully in effect
  readonly outcome: OutcomeFlags;     // OR over the revisions the last advance covered
}

export interface SessionSnapshot {
  readonly revision: bigint;
  readonly document: Uint8Array;      // canonical JSON, UTF-8, exactly the engine's bytes
}

export interface LiveSubmitOutcome {
  readonly revision: bigint;
  readonly path: EditPath | null;     // null: the call held only monitoring operations
}

export interface BrowserEngine {      // and the headless engine
  readonly revision: bigint;                                    // #1296 D1
  readonly instanceMode: "worker" | "single";                   // #1332 D1 (browser only)
  apply(transaction: SessionTransaction): Promise<EditOutcome>; // #1296 D1
  replaceSession(session: SessionLike | Uint8Array): Promise<EditOutcome>;
  watermark(): Watermark;                                       // the status field, no I/O
  applied(revision: bigint): Promise<{ readonly sample: bigint; readonly outcome: OutcomeFlags }>;
  snapshot(): Promise<SessionSnapshot>;                         // P3
  liveControls(): Promise<EngineLiveControls>;
}

export class EngineLiveControls {
  readonly edit: LiveControlEdits;    // builders by stable ID; they never touch the engine
  submit(...edits: readonly LiveEdit[]): Promise<LiveSubmitOutcome>;
}
// LiveEdit = SessionEditSpec (#1383, with P1's lane setters) | MonitorOpSpec (section 8.2)
```

Rules:

- `apply` resolves when the control plane commits. It never waits for render (D15-17). A refusal
  rejects with `MisoEngineError` (result and diagnostic); nothing was committed.
- `watermark()` returns the last value the engine posted. `applied(revision)` resolves when a
  watermark covers the revision. It rejects only when the engine is disposed. While the
  `AudioContext` is suspended it stays pending (D15-17).
- `replaceSession` sends the document; the control plane diffs it against the committed model into
  one transaction and applies it (D4). It is not a second path.
- `snapshot()` reads the committed model (section 4).
- Every edit addresses strips, routes, VCAs, effects and taps by stable ID. No SDK message carries
  a strip index.
- `CommandReport.appliedAtSample` (`sdk/src/core/boundary.ts:1416`) is removed. Once admission
  leaves the audio thread, the control side cannot know the sample at submit; the browser's value
  today is exact only because admission runs between two renders
  (`hosts/host-web/src/lib.rs:3337`, `:3388`). Completion is the watermark (F5).
- The ramp option has one name in every builder: `rampSamples` (absent: the session default; 0: a
  step; D15-1, #1394). `SmoothingOptions.smoothingSamples`
  (`sdk/src/core/live-controls.ts:28-30`) is renamed.

### 3.2 What the live-control handles become

**Decision: thin builders of transaction edits, submitted through the one apply path.** There is no
second, low-latency path.

The decision space:

| Option | What it is | Verdict |
|---|---|---|
| A. Thin builders | `edit.track("kick").faderDb(-6)` returns a transaction edit. `submit` sends one transaction (plus any monitoring operation) through the same message and the same crate function as `engine.apply`. | **Chosen.** One format, one commit, one model, on both hosts. |
| B. Records lowered in the control half | The SDK keeps posting 48-byte records with strip indices; the Worker lowers them into a transaction (#1382 D2 as written). | Rejected. It commits through the same apply, so it is no faster (the measured transport share is about 1 %, below). It keeps a second wire format, index addressing and its staleness rules (#1296 D3), and lowering code in the adapter. |
| C. Direct lane writes | A path that writes lanes without the model, as the browser does today. | Ruled out by D1 and D7: the committed model must hold every live value, or the three-way merge returns. |

**The measured reason a second path would need does not exist.** In a Chromium Worker, on the
64-track documents, the main-thread-to-Worker post costs 0.020-0.045 ms (p50) and the encode of a
transaction 0.005 ms (p50, in Wasm; the SDK encodes in TypeScript). The shared apply costs
2.66-7.38 ms (p50). The transport is about 1 % of the hop (section 5.4). Any path that commits to the model pays the apply; B saves only the
0.005 ms encode. A shorter hop comes from a cheaper apply, for every host (P2), not from a second
path.

**Condition: lane-granular edits (P1).** The live controls move one lane of one value, but today's
transaction setters replace whole structures: `SetTrackFader` (`DualMonoFader`: both levels and
both mutes, `crates/protocol/src/model.rs:352-356`), `SetTrackBuiltins` (trim, polarity and both
filters on both lanes), `UpsertRoute` and `UpsertVca`. A thin builder for those would need a copy
of the committed values, and a stale copy would write old values back. The `Exact` expected
revision makes that a refusal, not a silent overwrite, but the SDK would still need a mirror of
the model in TypeScript. P1 adds one lane-granular setter per live value. The engine then applies
each edit against its own committed model, on both hosts, and no host needs a mirror. Effect
parameters already are lane-granular (`EffectParam` has its own `channel`,
`crates/session/src/model.rs:579-588`).

### 3.3 The internal message

`miso.apply.v1 { tag, requestId, transaction, monitor }` (#1294 D1 gains `monitor`):
`transaction` is a `Uint8Array` frame or empty; `monitor` is the encoded monitoring operations or
empty; not both empty. The Worker stages both and calls one export, so both are checked and
committed together (#1382 D4). The message stays internal (D5).

### 3.4 Ordering and batching in the SDK

- One control message is in flight per engine (#1296 D3).
- Live submissions made while a message is in flight wait in a queue, in call order.
- When the reply arrives, the SDK sends the whole queue as one transaction, edits in call order,
  at the revision of that reply. If it holds more edits than the protocol's
  `maximum_transaction_edits`, the SDK splits it in order.
- A call made through `engine.apply` or `replaceSession` is never merged with another call.
- If the engine refuses a merged message, nothing was committed. The SDK sends each queued call
  again as its own message, in call order, so each call gets its own result.

Why: a transaction costs about one apply whatever its edit count, because the apply compiles the
whole session (section 5.4). Merging keeps the Worker's load and the hop bounded when the user moves
several controls at once. The last value of an address wins inside the transaction, which is the
same rule as the latest-target cells (D15-2). Worst-case latency is one apply in flight plus one.

## 4. Saves

### 4.1 The snapshot API

| Host | API | Bytes |
|---|---|---|
| C ABI | `SESSION_SNAPSHOT_GET {offset, maximum_bytes}` through `miso_engine_v1_submit_command`, paged (`crates/protocol/src/controller.rs:2851-2870`) | the canonical JSON of the committed model; each page's response header carries the revision (`crates/protocol/src/controller.rs:3108-3109`) |
| Browser | `engine.snapshot()` → `miso.document.v1` → `miso_engine_web_v1_committed_document_ptr` and `_bytes` (#1293 D4, #1294 D1), one copy | the same canonical JSON, with its revision |
| Headless SDK | `engine.snapshot()` on the same exports, called directly (#1389) | the same |

Both hosts read the same `SessionStore`, so the same model gives the same bytes on every host.

Consistency: a C ABI host makes every session call on one thread (`crates/capi/include/miso_engine_v1.h:18-22`),
so no commit falls between two of its pages unless it sends one. Every page carries the revision,
so a host can check it. The browser copies the whole text in one handler call, so its snapshot is
atomic.

What a snapshot means: the committed model, which includes every acked edit. While a rebuild is
pending, the snapshot is ahead of what is audible; its revision says which edits it holds. An app
that needs "what is audible" waits for `applied(revision)` first.

### 4.2 Cost on the 64-track documents

The canonical JSON is written by `compile_session` at every commit (`crates/session/src/compile.rs:130`)
and cached in the compiled session. So a snapshot costs a copy, and the serialization is paid at
commit time, by every edit (section 5.4: 0.56-1.33 ms of each native commit).

| Document | Canonical bytes | C ABI native, 16,000-byte pages, p50 of 50 reads | C ABI in Chromium Worker (Wasm), same pages, one read after the 1,500 edits |
|---|---|---|---|
| `console-sixty-four-track` | 264,762 | 17 pages, 0.066-0.068 ms | 17 pages, 0.960 ms |
| `console-sixty-four-track-app` | 241,761 | 16 pages, 0.068-0.073 ms | 16 pages, 0.970 ms |
| `console-sixty-four-track-sends` | 379,298 | 24 pages, 0.069-0.083 ms | 24 pages, 1.245 ms |
| `parametric-eq-nine-track` | 14,193 | 1 page, 0.004 ms | 1 page, 0.675 ms (first call) |

Both columns include the encode and decode of each page frame. In the Chromium column the edits
changed a few bytes of each document, and the first page includes compilation of the code path. The browser's whole-document export is one copy, so it
costs no more than the paged column. Commands are in section 5.5.

### 4.3 Render does no work for a save

- The committed model and its canonical text belong to `SessionStore`, inside the protocol
  controller, inside `SessionState` (`crates/capi/src/runtime/control.rs:237-238`). Render holds
  only `PlanState` and the plan (`crates/capi/src/runtime/plan.rs:53`, render entry `:201`), which has no reference to the
  model.
- The text is produced at commit on the control thread (C ABI) or in the Worker (browser), by
  `compile_session`.
- The worklet's post-boot export set is render-locked and holds no document export (#1332 D3,
  #1333); the document exports refuse on an instance without a control half (#1293 D1).
- In single mode the copy runs in the worklet's message handler, outside `process()`. It is
  control work on the rendering thread [S1]: about 1 ms for 380 KB, measured in a Worker.

## 5. Costs, measured

All at `8be19c86e`, on the host named at the top. Scratch builds and probes are under
`/tmp/claude-1002/w1057/` and are not committed.

### 5.1 Shipped AudioWorklet module with the control plane in the closure

**Proxy.** The crate of #1309 does not exist yet. The nearest buildable proxy is the whole C ABI
crate, which holds today's control plane (`crates/capi/src/runtime/{control,compile,plan,error}.rs`)
and `protocol`, linked into host-web and made reachable.

- Scratch tree: `git -C /home/bl/misofm/wt-d15-k archive HEAD | tar -x -C /tmp/claude-1002/w1057/tree`.
- Edit: `hosts/host-web/Cargo.toml` `[dependencies]` gains `capi.workspace = true` and
  `protocol.workspace = true`; `hosts/host-web/src/lib.rs` gains `extern crate capi;` and
  `extern crate protocol;`. `cargo tree -p host-web --offline` updated `Cargo.lock` (two lines).
- Reachability: a `cdylib` exports every `#[no_mangle]` entry point of its linked crates. The proxy
  module exports all 15 `miso_engine_v1_*` functions (checked with `wasm-objdump -x -j Export`),
  so nothing of the C ABI control plane is dead-stripped.
- Build: the exact cargo line of `scripts/build-web-audioworklet.sh:113-114`, which
  `.github/workflows/qualification.yml:132` runs (its `--named-twin` form), with a shared target
  directory:
  `CARGO_TARGET_DIR=/tmp/claude-1002/w1057/target RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo --remap-path-prefix=$HOME/.cargo=/cargo --remap-path-prefix=/tmp/claude-1002/w1057/tree=/repo" cargo build --locked --release --target wasm32-unknown-unknown -p host-web`,
  then `python3 -B scripts/strip-wasm-names.py strip <named> <shipped>`. The baseline from this
  line has the digest `a9a518625f4c...` that `TMPDIR=/tmp/claude-1002/w1057/tmp bash scripts/build-web-audioworklet.sh --module-only <dir>`
  prints in the scratch tree, and that the benchmark's `prepare` built from the worktree.
- Compressed size: the repository reports none. `gzip -9 -n` is given for reference.

| Module | Shipped bytes | gzip -9 bytes | Named twin bytes |
|---|---|---|---|
| Baseline, `8be19c86e` | 2,894,202 | 926,483 | 3,313,423 |
| Proxy (capi + protocol reachable) | 3,222,313 | 1,039,871 | 3,692,798 |
| Growth | +328,111 (+11.3 %) | +113,388 (+12.2 %) | +379,375 |

Where the growth goes (named twin, `twiggy top` shallow bytes, grouped by crate):
`protocol` +186,124; `capi` +41,441, plus 7,351 in six entry points (each of the other nine is
under 500 bytes); generic `core` code
+34,985; `host_core` +21,854; `.rodata` +16,628; `alloc` +9,339; `engine` +5,311; `session`
+1,989; the `name` section +51,264 (stripped from the shipped module).

**Why this is a fair upper bound.** It over-counts the C ABI glue and capi's own preparation path
to host-core's C ABI lane entry, which the browser replaces with its own preparer (#1400). It keeps
host-web's own admission, which #1382 D5 deletes (`admit_commands` alone is 18,327 shipped bytes),
and the prepared-control path (F6). It under-counts the document diff (#1386), the new exports and
records (#1293, #1381), the overlay composition and P1's setters; each of these is small next to
`protocol`. The real growth is therefore near +0.33 MB raw and +0.11 MB compressed, and below it
after the deletions. No decided item sets a module size budget.

### 5.2 Crates in host-web's closure

`cargo tree -p host-web -e normal` at `8be19c86e`: 7 direct dependencies (`builtins`,
`builtins-compiler`, `effect-contract`, `engine`, `host-core`, `math`, `session`), 169 lines, 66
distinct crates (`--prefix none --no-dedupe | sort -u`). With the proxy: 68, adding exactly `capi`
and `protocol`. In the design, `control-plane` and `protocol` enter. `protocol` depends only on
`engine` and `session`, which are already present, and it enters through host-core's
`control-provider` feature (`crates/host-core/Cargo.toml:15`, `control-provider = ["dep:protocol"]`;
#1309 D9). No third-party crate enters.

### 5.3 Control-thread cost of a structural edit

`artifacts/steps/web-rebuild-base/report.md` (#1289) was recorded at `0b477c585`. The base moved:
93 commits since then change `crates/host-core/src/prepare.rs` (+636 lines), the C ABI control path,
`crates/builtins-compiler` and every effect crate. So the benchmark ran again:

```
TMPDIR=/tmp/claude-1002/w1057/tmp bash scripts/run-web-mixing-automation-benchmark.sh prepare /tmp/claude-1002/w1057/bench/prep
bash scripts/run-web-mixing-automation-benchmark.sh rebuild-preflight /tmp/claude-1002/w1057/bench/prep
MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 bash scripts/run-web-mixing-automation-benchmark.sh rebuild-run /tmp/claude-1002/w1057/bench/prep --step w1057-rebuild-head
```

The output was moved out of the worktree to `/tmp/claude-1002/w1057/evidence/w1057-rebuild-head/`.
Node v22.23.2 (V8 12.4.254.21, `--no-liftoff`), module `a9a518625f4c...`, CPU 31, uncontrolled
(load 23-35). Time of `miso_engine_web_v1_boot`, 25 boots per document per round; one quantum is
2.667 ms.

| Document | Base `0b477c585` p50 / max ms | HEAD round 1 p50 / max ms | HEAD round 2 p50 / max ms | HEAD round 2 p50 in quanta |
|---|---|---|---|---|
| `nine_track_eq` | 2.695 / 11.385 | 4.060 / 7.927 | 2.634 / 7.819 | 1.0 |
| `sixty_four_track_console` | 23.916 / 33.470 | 35.647 / 54.843 | 23.851 / 33.461 | 8.9 |
| `sixty_four_track_app_shape` | 23.424 / 32.394 | 33.323 / 46.618 | 23.340 / 34.233 | 8.8 |
| `sixty_four_track_console_sends` | 39.120 / 49.115 | 56.129 / 68.683 | 39.366 / 52.056 | 14.8 |

Round 2 matches the base; round 1 ran under the heaviest load. A structural edit costs about one
boot: 9-15 quanta of control work. In Worker mode it leaves the audio thread. In single mode it
blocks the audio thread, as D15-10 accepts.

### 5.4 Live-edit hop of the Worker design

**What the probe measures.** Chromium 151.0.7922.34 (Playwright 1.62.1, `channel: "chromium"`,
headless), served with `Cross-Origin-Opener-Policy: same-origin` and
`Cross-Origin-Embedder-Policy: require-corp`; `crossOriginIsolated` was true on the page and in the
Worker. A dedicated Worker instantiates the proxy module, extended with scratch `probe_*` exports
(shipped size 3,321,898 bytes), and boots the document through
`miso_engine_v1_compile_session`. For each edit the page stamps `t0`, posts `{track, db}`; the
Worker stamps receipt (post), encodes one `SetTrackFader` transaction (encode), calls
`miso_engine_v1_submit_command` (apply), drains the reliable lane (drain), then does
`Atomics.store` of the edit's sequence into a `SharedArrayBuffer` word, the cell stand-in, and
stamps it (hop to cell = stamp − `t0`). A third thread polls that word and stamps when the new
value is visible (cell visible). Clocks are `performance.timeOrigin + performance.now()` in every
agent, coarsened to 5 µs [S7]. 500 edits per round, one warmup and two measured rounds, one
invocation, each edit sent after the previous reply.

Command: `cd /tmp/claude-1002/w1057/probe && EDITS=500 node run.mjs` (server: `server.mjs`; page:
`www/{index.html,main.js,worker.js,reader.js}`).

| Document | Round | Post p50 / max | Apply p50 / p99 / max | Hop to cell p50 / p99 / max | Cell visible p50 / max |
|---|---|---|---|---|---|
| 64-track console | 1 | 0.025 / 0.440 | 4.890 / 5.485 / 6.905 | 4.930 / 5.595 / 6.935 | 0.000 / 0.045 |
| 64-track console | 2 | 0.020 / 0.180 | 2.660 / 5.385 / 5.445 | 2.715 / 5.520 / 5.560 | 0.000 / 0.075 |
| 64-track app | 1 | 0.035 / 3.035 | 4.045 / 6.995 / 8.745 | 4.125 / 7.100 / 11.170 | 0.000 / 1.820 |
| 64-track app | 2 | 0.030 / 2.035 | 4.600 / 5.610 / 8.930 | 4.645 / 5.840 / 8.970 | 0.000 / 0.125 |
| 64-track sends | 1 | 0.045 / 3.550 | 7.125 / 11.180 / 14.545 | 7.225 / 14.415 / 14.975 | 0.005 / 0.265 |
| 64-track sends | 2 | 0.030 / 0.990 | 7.375 / 8.150 / 9.015 | 7.460 / 8.345 / 9.435 | 0.010 / 0.145 |
| 9-track EQ | 1 | 0.020 / 0.565 | 0.320 / 0.375 / 0.445 | 0.345 / 0.410 / 0.905 | 0.000 / 0.095 |
| 9-track EQ | 2 | 0.020 / 2.995 | 0.335 / 0.385 / 0.510 | 0.370 / 0.440 / 3.380 | 0.000 / 0.375 |

All values in ms. Encode: p50 0.005 ms. Drain: p50 at most 0.005 ms, one `SESSION_COMMITTED` event
per commit. A p50 of −0.005 ms for cell visibility was rounded to 0.

**The same apply natively (C ABI control thread).** `/tmp/claude-1002/w1057/target/release/examples/probe_native <doc> <revision> 500`
(the probe module natively, `cargo build --release --example probe_native -p host-web` in the
scratch tree). `miso_engine_v1_submit_command` only, p50 / p99 / max ms, rounds 1 and 2:
64-track console 1.945-2.148 / 2.233-3.973 / 2.339-4.009; app 1.732-1.733 / 3.189-3.196 /
3.208-3.236; sends 2.720-2.723 / 5.234-5.240 / 5.260-5.273; 9-track 0.115-0.116 / 0.220-0.234 /
0.227-0.241.

**Where the apply goes (native, `probe_phases <doc> 300`, two runs).** 64-track console:
`classify_live_delta` 1.20-2.04 ms; `SessionStore::prepare_transaction` 0.61-1.06 ms, of which
the canonical JSON write is 0.56-1.00 ms; model clone 0.03 ms. 64-track sends: classify 2.67 ms,
prepare 1.43 ms, canonical JSON 1.33 ms. Every term grows with the session, not with the edit:
the classifier lowers every track twice (#1305 N2), the compile writes the whole canonical text,
and every transaction clones the whole replay arena (`ReplayCache::try_clone_eager`,
`crates/protocol/src/controller.rs:268-276`).

**Limits.** (1) On `8be19c86e` the control plane pushes into 16-deep live queues, not cells (#1312
has not landed); a cell write is a few atomic stores, not timed apart. To keep the queues from
filling, the Worker rendered one block after every 8 edits, outside the timed span. (2) No
AudioWorklet ran. Render picks a cell up at its next block, at most one quantum (2.667 ms at 48 kHz
and 128 frames) after the write, plus the output latency; not measured. (3) The module is the
stable, non-atomic build; its memory is not shared, so the cell stand-in is a separate
`SharedArrayBuffer`. (4) The control plane is today's capi one: no browser preparer, overlay, P1
or P2. (5) One browser and one machine; no Firefox, WebKit or iOS; the host was loaded.

### 5.5 Evidence files and commands

In `/tmp/claude-1002/w1057/evidence/` (not committed): `baseline/size.txt`,
`control-plane-proxy/size.txt`, `by-crate-delta.txt`, `scratch-proxy.diff`, `cargo-tree-head.txt`,
`cargo-tree-proxy.txt`, `w1057-rebuild-head/`, `probe-hop.json`, `probe-native.txt`,
`probe-phases.txt`, `probe-snapshot-native.txt`, `probe-host.txt`. The probe sources are in
`/tmp/claude-1002/w1057/scripts/`.

## 6. The staged plan

### 6.1 Steps and their issues

| Step | What | Issues |
|---|---|---|
| 1 | One control plane: extract the crate; adapters call it. | #1309 |
| 2 | The edit contract on the C ABI: path in the response, watermark, service step, supersession, cells, per-edit ramp. | #1313, #1314, #1348, #1343, #1344, #1398, #1310, #1311, #1312, #1345, #1346, #1347, #1394 |
| 3 | Complete the edit vocabulary: lane-granular live setters. | **P1** (new) |
| 4 | Bound the cost of a value-only edit by the edit. | #1305, **P2** (new) |
| 5 | The browser substrate: two instances on one memory, allocation gates, nightly artifact, shared memory, Worker, producers in the Worker, preparer, plan exchange and service loop. | #1331, #1333, #1334, #1380, #1332, #1387, #1401, #1400, #1381 (with F2, F10) |
| 6 | Browser live edits through the committed model, with the monitoring overlay. | #1382 (with F3, F4, F5, F6, F8), **P5** (new) |
| 7 | Browser structural edits, exports, messages, document replace, qualification. | #1290, #1293, #1386 (with F7), #1294 (with section 3.3), #1295 |
| 8 | The SDK: transaction encoder, apply, live controls as builders, saves, feeds, headless, status words. | #1383, #1385, #1296 (with F9), **P6** (new), **P3** (new), #1297, #1389, #1349, #1399 |
| 9 | Remaining live rows and the warm deadline in the browser. | #1342, #1361 |

Order inside a step follows `docs/handoffs/decision-15-2026-10-05/STREAMS.md`. New order edges:
P1 after #1394 and before P6; P6 after #1383, #1385 and P1, and before #1296 closes; P2 before
#1382's single-mode leg (F1); P5 with or right after #1382 and before #1290; P3 after #1293.

### 6.2 Proposed issues

**P1. Add lane-granular live-value edits to the session transaction** (stream B). The live
controls move one lane of one value, but `SetTrackFader`, `SetTrackBuiltins`, `UpsertRoute` and
`UpsertVca` replace whole structures, so a thin client needs a copy of the committed model (section
3.2). Add one setter per live value at the granularity a control moves it: strip fader level and
mute per lane; input trim, polarity, high-pass and low-pass per lane; route gain, mute and matrix;
VCA level and mute per lane. Each takes #1394's optional `ramp_samples`, addresses by stable ID,
and changes exactly one field of the model. In-place V1 amendment of the protocol schema (as #1313
D7): opcodes, codec, `apply_session_edit`, `docs/CONTROL_BTLV_V1.md`, the registry, the corpus
hash re-pin. Gates: each setter round-trips; applying it equals the whole-structure setter with the
other fields unchanged; the classifier gives the same delta for both forms; a setter on an unknown
ID is refused before any change.

**P2. Bound a value-only transaction's control cost by the edit, not the session** (stream B, with
J). A live edit on a 64-track document costs 2.7-7.4 ms (p50) in V8 and 1.7-2.7 ms natively,
almost all of it work proportional to the session: the canonical JSON write at every commit
(`crates/session/src/compile.rs:130`), the classifier (#1305 owns its lowering of every track), and
the clone of the whole replay arena per transaction (`crates/protocol/src/controller.rs:268-276`).
Scope: produce the canonical text only when a snapshot asks for it after a commit (bound its size
by computation, not by writing it), clone only the used part of the replay arena, and land #1305.
No rendered bit moves. Gate (the single-mode budget, F1): in V8, on the three 64-track documents, a
value-only edit's apply p99 plus the render p99 of the same document fits in one quantum (2.667 ms
at 48 kHz and 128 frames). If the gate cannot be met, root rules on F1 again with that evidence.

**P3. Snapshot the committed session from the SDK engines** (stream H). No spec owns the public
save. Add `engine.snapshot(): Promise<SessionSnapshot>` on the browser and headless engines over
#1293 D4's export and `miso.document.v1` (#1294 D1). Gates: after live edits the snapshot equals
the canonical text of the committed model with its revision; during a pending rebuild it is the
committed model and its revision; the worklet calls no export for it; single mode copies it in the
message handler, never in `process()`.

**P5. Key the monitoring overlay by stable ID and compose it into every plan the control plane
publishes** (stream H, with B for the crate). `LiveControlSoloState` is indexed by strip index of
one prepared plan (`crates/host-core/src/solo.rs:125-129`) and is built at preparation (`:160`).
#1382 gate 3 asks that a structural edit keep the overlay in effect but names no mechanism. Scope:
the crate stores the overlay as soloed track IDs and armed observation taps by stable address;
every successor is prepared and seeded from the committed model plus the overlay (effective mute
`model || VCA || solo`, `crates/host-core/src/solo.rs:259`); a strip added while any track is soloed starts solo-muted,
and its fade-in targets the effective mute; carried retargets compare effective mutes; for each
armed tap whose owner exists in the successor but did not carry (#1327 D1), the control plane
pushes the arm record into the candidate's lane before publication; an ID that leaves the model
leaves the overlay in the same commit, and the SDK learns it from the reply. The meter lease is not
part of the overlay (section 8). Gates: solo track 2, add a track, adopt: the added track is silent
and equals a fresh plan of the committed model with the same overlay; remove the only soloed track:
the others return over the solo ramp; an armed tap on a restarted effect keeps delivering.

**P6. Build the SDK's live-control edits as session transaction edits** (stream H). Rewrite
`LiveControlEdits` and `EngineLiveControls` (`sdk/src/core/live-controls.ts:432`, `:976`) to return
#1383 edit specs (with P1's setters) and monitoring operations by stable ID, and to submit through
`miso.apply.v1` with the batching of section 3.4. Remove the 48-byte record path from the SDK and
the shipped host (`sdk/src/browser/live-controls.ts:21-68`, `miso.command.v1`), rename
`smoothingSamples` to `rampSamples`, and return `LiveSubmitOutcome`. Gates: every live-control
method's frame equals the native codec's for the same edit; a merged batch that the engine refuses
is resent call by call and each call gets its own result; no SDK message carries a strip index.

P4 is an amendment, listed as F2.

### 6.3 Amendments to existing specs

| Spec | Amendment | Finding |
|---|---|---|
| #1381 | D6 also drains the reliable event lane after every message and on every service tick; D1 states the browser's `ControlLimits` | F2, F10 |
| #1382 | D2: no record lowering; the Worker receives transaction frames and monitoring operations by stable ID. D3: the meter lease leaves the overlay. D5 also deletes the prepared-control path. The reply drops `appliedAtSample`. The single-mode leg depends on P2. | F1, F3, F4, F5, F6 |
| #1294 | D1: `miso.apply.v1` gains `monitor` (section 3.3) | — |
| #1296 | D3: the map epoch and the stale-edit refusal are not needed with stable-ID addressing; the SDK still re-reads the session map after `rebuild` for client-side checks | F9 |
| #1386 | The diff lives in `crates/control-plane`, not `hosts/host-web/src/document_diff.rs` | F7 |

### 6.4 Findings for the coordinator

- **F1. Single mode blocks the audio thread on every live edit, not only on structural edits.**
  This touches D6 and D15-10, which count only the structural edit as the blocking rebuild. With
  admission in the shared control plane (#1382 D1), a single-mode live edit runs the whole apply in
  the worklet's message handler, which runs on the rendering thread before the next quantum [S1].
  Measured in a Chromium Worker, that apply is 2.66-7.38 ms (p50) on the 64-track documents; one
  quantum is 2.667 ms, and V8 renders a 64-track console in about 0.23 ms per quantum (the
  `sixty_four_track_console` row of `artifacts/steps/console-strip-after/web-mixing-automation.jsonl`,
  recorded at `f7ba70a8b`, p50 234,436 ns). Today the same fader move runs the browser's own
  admission, which writes lanes and compiles nothing (`admit_commands`,
  `hosts/host-web/src/lib.rs:4596`). Inference, not measured: the worklet runs the same V8 and the
  same code, so the apply costs the same there. Proposed: P2, with its gate as the single-mode
  budget, before #1382's single-mode leg ships. If P2 cannot meet it, root rules again (for example,
  to take the support of non-isolated pages to the owner).
- **F2. Nothing in the browser drains the reliable event lane.** Every commit reserves one
  `SESSION_COMMITTED` event (`crates/protocol/src/controller.rs:1639`), the lane holds two (`crates/capi/src/runtime/compile.rs:128`),
  and a full lane refuses the next transaction (`crates/protocol/src/controller.rs:1735-1744`). The probe drained one
  event per commit. No browser spec (#1290, #1293-#1296, #1332, #1381, #1382, #1386) mentions the
  lane, so the third browser edit would be refused. Amend #1381 D6: the control half drains the lane
  after every message and on every service tick, in both modes. A `SESSION_COMMITTED` event repeats
  the reply and is not forwarded. (This is P4.)
- **F3. The meter lease cannot live in the Worker.** #1382 D3 puts it in the overlay. Its state is
  the render half's meter delivery: the flag, the activation sample and the generation
  (`hosts/host-web/src/lib.rs:2820-2840`), read by `render_next` (`:3256`) and `poll_meters`
  (`:3430`) in the worklet. A Worker-side lease would write that state from another thread. It stays
  a render-half message (section 8).
- **F4. #1382 D2 keeps a second edit format.** It lowers 48-byte, index-addressed records in the
  Worker. Section 3.2 chooses transaction edits by stable ID with P1 and P6. The lowering is not
  needed.
- **F5. `appliedAtSample` becomes false once admission leaves the audio thread.** The value is
  `status.next_absolute_sample` at admission (`hosts/host-web/src/lib.rs:3337`, `:3388`). In the
  Worker it cannot know the block render applies an edit at. #1382 D2 keeps "today's command
  report"; the field is removed and completion is the watermark (#1349).
- **F6. The prepared-control path duplicates the shared commit.** The main realm designs EQ targets
  in its own Wasm instance and submits them with a companion
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:781-805`; `hosts/host-web/web/prepared-control.js:330`;
  `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1630`; `hosts/host-web/src/ffi.rs:3847`). The shared commit designs the same targets
  in the control plane (`crates/capi/src/runtime/control.rs:1145-1187`). #1382 D5 does not name the
  path; it is deleted with the worklet's admission.
- **F7. The document diff is session logic.** #1386 puts it in host-web. D1 keeps the adapters thin.
  The diff is host-agnostic, so it lives in the portable crate; the C ABI does not expose it (D2).
- **F8. The solo overlay has no carry mechanism.** See P5.
- **F9. #1296 D3's epoch rule exists only because of strip indices.** With every edit addressed by
  stable ID (section 3.1), an edit made before a rebuild either still names a live entity or is
  refused by the engine as unknown. It can never land on a shifted strip.
- **F10. The browser's `ControlLimits` set the cost of every edit.** The replay arena is
  `maximum_replay_bytes`, cloned whole by every transaction (`crates/protocol/src/controller.rs:268-276`), and it must
  hold the largest request plus its response (`crates/protocol/src/controller.rs:437-461`). No spec states the browser's
  values. #1381 D1 states them, and P2 removes the whole-arena clone.

## 7. The open owner question: personal mixes

**Status: open.** It blocks nothing in this issue or in decision 15. Every design choice above works
with every option below: each option ends as a full document (boot or `replaceSession`) or a
transaction (`apply`), and the engine plays a full, resolved session in every case.

> **Question for the owner: how does the app keep a fan's personal mix?**
>
> **Background.** A fan can change a session: turn a stem up, mute a track, pan a vocal. The owner
> ruled that the fan's changes are saved as a personal mix and that the producer's original does not
> change (`docs/rulings/engine-footprint-2026-09-28.md`, decision 2). The engine can already give
> the app the whole edited session as one canonical file. Three things are not decided: what the
> app stores, what happens when the producer publishes a new version, and what a fan's move does to
> a control that the producer automated.
>
> **Part 1. What the app stores.**
>
> - **A. A full copy of the edited session.** Simple: the app saves the snapshot. Cost: one full
>   file per fan (about 240-380 KB for 64 tracks). The copy loses its link to the producer's version,
>   so part 2 is hard.
> - **B. The producer's version plus a log of the fan's transactions.** Small. The app replays the
>   log on load. Cost: a long log grows without end. A transaction made against one version may not
>   apply to the next.
> - **C. The producer's version plus a short list of changed values ("overlay").** Each entry is
>   "this field of this stable ID has this value", for example "fader of track `vox` = −3 dB". Small
>   and bounded by the number of controls. On load, the app (or the SDK) turns the list into one
>   transaction. Cost: a rule for each kind of field, and a diff (the same one `replaceSession` uses)
>   to make the list from a snapshot.
>
> **Part 2. The producer publishes a new version.**
>
> - **U1. Keep the old version** for this fan until the fan chooses to move. Cost: the app keeps
>   old versions available.
> - **U2. Move automatically** and keep each fan change whose target still exists and has the same
>   kind; drop the others and tell the fan. Cost: works well with C, poorly with A, and needs care
>   with B.
> - **U3. Start again** from the new version. Cost: the fan loses the mix.
>
> **Part 3. Stored automation** (*Research: render stored session automation in the engine,
> identically on every platform*, #1058, question 2). If the producer automated a fader, a fan's
> value can override the automation, offset it, or latch it. The personal mix must store whichever
> one the owner picks for #1058. It also decides whether a fan's solo is part of the mix: today solo
> is never saved (section 8); if the owner wants it saved, solo needs a session field.
>
> **Recommendation (not a decision).** C with U2: an overlay of changed values by stable ID, moved
> to each new version automatically, with dropped entries shown to the fan. It is small, bounded,
> and needs no engine change: the app applies it as one transaction. Part 3 follows the owner's
> answer to #1058 question 2.

## 8. Host-local monitoring state

### 8.1 Ruling

**Root's position is confirmed for solo and observation subscriptions, with two precisions, and
broken for the meter lease.** Solo and observation subscriptions are host-local monitoring state,
not session content. They stay outside the committed model, travel with edits through the
portable crate (in the Worker on the browser), and never appear in a path, a revision or a
snapshot. Precisions: the overlay is keyed by stable ID, and the control plane composes it into
every plan it publishes (P5). The meter lease is also monitoring state outside the model, but it is
render-half delivery state, so it stays with the render half (F3). The proposed issue is P5; the
lease is an amendment of #1382 D3.

### 8.2 The three items

| Item | Where it lives today | What it changes | Design |
|---|---|---|---|
| Solo | `LiveControlSoloState` (`crates/host-core/src/solo.rs:129`), composed at admission (`crates/host-core/src/solo.rs:1-20`); SDK `TrackEdits.solo` (`sdk/src/core/live-controls.ts:728-740`); command kind 9 (`hosts/host-web/src/lib.rs:869`) | strip mute lanes, as `TrackFaderRecord::Mute` | a `MonitorOp::Solo { track_id, solo, ramp }` with the edit; overlay in the crate, keyed by track ID (P5) |
| Observation subscriptions | command kinds 7 and 8 (`hosts/host-web/src/lib.rs:851`, `:853`); SDK `EffectEdits.observe` (`sdk/src/core/live-controls.ts:960`) and `subscribeObservations` (`sdk/src/browser/engine.ts:205`) | `Observe` records in an effect's FIFO lane (D15-2), which arm a tap | a `MonitorOp::Observe { strip_id, rack, instance_id, tap_id, armed, window_blocks }` with the edit; overlay in the crate (P5) |
| Meter lease | `AudioWorkletEngineHost::set_meter_lease` (`hosts/host-web/src/lib.rs:2820`), export `miso_engine_web_v1_meter_lease` (`hosts/host-web/src/ffi.rs:3919`), worklet `receiveMeterLease` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1678`); SDK `subscribeMeters` (`sdk/src/browser/engine.ts:197`) | which meter frames the worklet folds and posts after render (`hosts/host-web/src/lib.rs:3256`, `:3430`; `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1859`) | unchanged: a render-half message, handled in the worklet's message handler, allocation-free; in single mode the same instance |

### 8.3 The spec's sub-questions

- **Is any of the three saved?** No. The session model and the protocol have no solo, observation or
  lease field or command (no match for `solo` in `crates/session/src/` or `crates/protocol/src/`).
  The snapshot is `SessionStore::canonical_snapshot` (`crates/protocol/src/model.rs:900`), so it
  cannot hold them.
- **Is any of them shared with a personal mix?** Not in this design. A personal mix is session
  content under every option of section 7. If the owner wants a fan's solo in the mix, solo gains a
  session field and a transaction form; section 7 lists that.
- **Is any of them replayed after a rebuild?** Not replayed from a log; composed again. Solo and
  subscriptions are inputs of every plan the control plane publishes (P5): a successor starts with
  the effective mute and with every armed tap armed. The meter lease is not per plan: it is host
  state in the render half and continues across a swap (#1290 D7).
- **How does the overlay cross a plan swap and a `replaceSession`?** A swap: the overlay is in the
  crate, not in a plan, so it does not move; successor preparation reads it (P5). Carried strips
  keep their lane state by the carry (D15-7), and their retargets compare effective mutes. A
  `replaceSession` is one transaction (D4), so it is the same case. An ID that the transaction
  removes leaves the overlay in the same commit.
- **How are its submits refused or acknowledged?** Monitoring operations ride in the same message
  and the same call as the transaction. Every check of both runs before the first write; a refusal
  changes neither the model nor the overlay (#1382 D4). Solo writes cells, which cannot fail; a
  subscription needs room in a FIFO lane, checked first. An ack never precedes a drop (section
  1.3). The lease is a level, set synchronously in the worklet handler, and acked after the flag
  is set.
- **Does the C ABI treat the same three the same way?** Yes, for what it has. It has no solo and no
  observation tap (#1327 context). Its meter output is configured by `TelemetryConfigure`, a
  protocol command that never commits a transaction or advances the revision
  (`crates/protocol/src/controller.rs:3059-3076`), and its delivery runs on the control thread
  (`collect_render_activity`, `crates/capi/src/runtime/control.rs:522`). So on the C ABI, too,
  monitoring is outside the model, goes through the control plane, and lives where its delivery
  runs. A C ABI solo is added only for a product need (engine footprint rulings); it would be a
  monitoring operation, never a transaction, so the crate needs no other change.
