# Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live console lanes

**Umbrella.** Rewritten on 2026-10-04 from the single-issue spec of 2026-09-28, and again on
2026-10-05 for decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1 to D15-6
and D15-17). Code anchors verified on `main` at `6fb211594`.

The title keeps the original wording. The scope covers every C ABI value that decision 14
classifies live (its F5: the input section, effect parameters and effect bypass) and model-only
edits (its F8). Decision 15 D15-6 adds sends, `follows_mute`, VCA offsets and VCA membership.

- Research record: `docs/handoffs/live-control-2026-09-28/` (`FINDINGS.md`, `VERIFY.md`). Batch
  record: `docs/handoffs/live-updates-1053/README.md`.
- Earlier texts are in git history
  (`git show 54b0a1bf8:.github/ISSUE_SPECS/1053-deliver-value-only-fader-mute-and-pan-transactions-to-the-running-c-abi-plan-thr.md`
  for the 2026-09-28 spec and amendments A0-A5; `6fb211594` for the 2026-10-04 umbrella).
- The rule: decision 14, *Live update versus plan rebuild* (#1259,
  `docs/rulings/live-update-versus-rebuild-2026-10-04.md`), with decision 15's answers to its
  Q3, Q4, F3, F4 and F9.

## Product outcome

A phone, or any C ABI host, keeps one edit API: `miso_engine_v1_submit_command` with
`SESSION_TRANSACTION_APPLY`. The engine classifies each committed transaction itself:

- **Live update.** The committed session before and after the transaction differ only in live
  values: fader, mute, pan or matrix, input trim, polarity, HPF and LPF, live effect parameters,
  effect bypass, sends, VCAs. The engine writes them to the running plan's live lanes. No plan is
  compiled; source rings, effect state and the render position continue; every value ramps
  (D15-1); nothing acked is ever lost.
- **Model-only commit.** The two models differ only in fields no plan reads. No record, no rebuild.
- **Plan rebuild.** Every other transaction (#1269 makes it seamless).

The response says which path ran (D15-3, #1313), and an applied-revision watermark says when it took
effect (D15-17, #1314).

## On `main` (`6fb211594`)

- **Delivered.** The core slices (#1253-#1258), model-only edits (#1260) and effects (#1263-#1266)
  are closed. The classifier is `host_core::classify_live_delta`
  (`crates/host-core/src/live_delta.rs:211`); capi's live arm is `SessionState::commit_live`
  (`crates/capi/src/runtime/control.rs:1065`), which targets the newest epoch
  (`pending_providers.last_mut()`, `:1089`).
- **Live records step.** `LiveRamps::for_session` returns 0 for fader and mute
  (`live_delta.rs:52-58`). D15-1 replaces it (#1054).
- **Live lanes are FIFO queues of depth 16** (`LIVE_QUEUE_DEPTH`,
  `crates/capi/src/runtime/compile.rs:11-13`; requested at `:589`). A full lane is
  `control.live.backpressure` (`control.rs:404-405`). D15-2 replaces them with cells (#1312).
- **A pending candidate refuses the next structural edit** (`control.rs:959-961`). D15-9 replaces
  that with supersession (#1310).
- **A live input lane makes the strip's builtin tail infinite**
  (`crates/builtins-compiler/src/lib.rs:3559-3566`). The C ABI attaches no input lane yet. D15-4
  replaces `Infinite` with a bounded tail (#1328, #1329).
- **Guards still in the classifier:** any VCA is a rebuild (`live_delta.rs:216-217`); a mute change
  of a `follows_mute` source is a rebuild (`:278-279`); a submix strip's fields are structural by
  the mask.
- The reliable event lane holds 2 events (`compile.rs:128`).

## Decision record

- **D1. Classification is by the committed-model delta.** Edit opcodes are never inspected.
  `classify_live_delta(current, next, ramps)` reads the normalized committed and prospective models,
  in this order: the VCA guard (G3); the track set; the mask (clone `next`, copy `current`'s live
  fields into it, compare `session::canonical_session_json` bytes, never `PartialEq`, because the
  canonical `f32` spelling keeps `-0.0`); the domain (a value the setter's own checker refuses is a
  rebuild); the follow guard (G2); then the records. A record is emitted only when the value the
  render plane holds would change (D9). A transaction that rewrites identical values is live with
  zero records and still advances the revision and emits `SESSION_COMMITTED`.
- **D2. The timing contract.** An edit is admitted all or nothing. Render converges to the
  committed value no later than the first block whose render begins after the submit returns, or,
  while a successor is pending, at its adoption, which the watermark reports (D15-2 as refined by
  D15-17). It is heard up to `latency_samples` later, because the compensation delays are downstream
  of the fader. It is never lost. #444's block-atomic batch claim is declined knowingly.
- **D3. Every live value ramps (D15-1).** A record with no ramp length uses the session's
  researched `controlSmoothing` default (#1055, then #1054); an explicit 0 stays legal. #1054 covers
  every live row: fader, mute, pan or matrix (a model `smoothing_samples` of 0 means "session
  default"), input trim and polarity, sends (gain, mute, matrix), VCA offset and mute. The seam is
  `LiveRamps::for_session`. SDK defaults stop being 0: the browser and SDK resolve an absent ramp to
  the session default (#1364), and `control_smoothing` is a model-only transaction edit (#1365). A step is a click; the caller must not choose
  smoothness by omission.
- **D4. Live values are latest-target cells (D15-2, #1312; effect lanes #1345, input lanes #1346,
  route lanes #1347).** Gated conditions: levels only; a value
  is superseded only by a later commit before the same drain; every fallible check runs before the
  first cell write; an exact `live_values_superseded` counter. Each cell holds its target words and
  ramp as one unit, with a per-lane dirty mask and a canonical drain order (fader before mute).
  Each cell is #1312's triple buffer: render reads the newest completed write in one pass, never
  tears, never skips and never spins. Automation,
  Observe records, structural edits and time-stamped records stay FIFO. A paused host therefore
  never gets `BACKPRESSURE` for a live value. The ack bytes are unchanged.
- **D5. The lanes on the C ABI.** The core attaches every strip's fader and matrix lanes (#1254,
  #1256) and each effect instance's lane (#1263). The input lane arrives with #1261, the route lanes
  with #1225. Delivery stays `Concurrent`.
- **D6. The commit order, and the acked-batch question.** In the structural arm, after the existing
  `plan_alive` and response-size checks: (1) classify (D1); a rebuild takes the rebuild path;
  (2) the live admission (D8), refusal `COMPILE_REJECTED`; (3) resolve each changed strip's lanes by
  ID in the newest epoch (D7), a missing ID is `INTERNAL`; (4)
  `ProtocolController::check_prepared_structural` (`crates/protocol/src/controller.rs:1954`), failure
  `INTERNAL`; (5) write every cell (infallible); (6) commit the token (infallible after step 4,
  under the same `&mut` borrow); (7) write the response. A failure in steps 1-4 changes nothing.
  Render applies every value it reads, because D1 refused every value the setter would refuse. **So
  no ack ever precedes a drop.** A superseded value is counted, never silently lost (D15-2).
- **D7. Which plan gets the values.** The newest epoch: the pending candidate if there is one, else
  the current provider (`replacement_base_report`, `control.rs:760`, uses the same rule). While a
  warm successor catches up, a live edit is held in the control plane and written to the successor
  at publication (D15-17; *Hold live edits during a catch-up and apply them at the adoption
  sample*, #1356).
- **D8. The live admission.** The prospective compiled model lives beside the current one until the
  commit, and a value edit can grow the canonical JSON. The edit is admitted only if the graph bytes,
  the capi retained bytes and the largest named allocation stay within their maxima, counted as
  #1257 implemented. Each provider epoch keeps its `CapiResources` for this check.
- **D9. The committed model is the authority, and the plan follows it.** The committed model's
  values are the values the newest plan was prepared with, plus every value written to that plan
  since. A rebuild prepares from the committed model. `SessionSnapshotGet` returns it. A refused edit
  changes neither the model nor the plan. #1269 P1.4 uses this invariant as its carry base.
- **D10. What the host sees (D15-3, D15-17).** This reverses the 2026-10-04 "no new field" answer.
  - The response carries `{revision, path}`, where path is exactly one of `live`, `model_only` or
    `rebuild` (#1313). Fallbacks (pre-roll, transition) and supersession are known only after
    submit, so the watermark's outcome flags report them (#1314).
  - The applied-revision watermark `(revision, first sample in effect, outcome flags)` is a C ABI
    query and a browser status field. It never uses the reliable event lane (#1314).
  - The host calls `miso_engine_v1_service` from a non-realtime thread; every other control call
    also services (#1348). The browser publishes the watermark in its status (#1349).
  - One `SESSION_COMMITTED` per transaction and one `AUTOMATION_CANCELED` per queued automation
    batch remain (`commit_prepared_structural`, `controller.rs:1974`). The host still drains the
    reliable lane after each edit.
  - The header, `docs/C_ABI_V1_QUALIFICATION.md` and `docs/CONTROL_PROTOCOL_SEMANTICS.md` say all of
    this (#1313, #1314).
- **D11. Bounded drains.** Each builtin drain reads at most what is present at block entry (#1253).
  With cells (#1312) the drain is a dirty-mask scan, still bounded. **A forward hazard:** any future
  silence skip must still run every drain on a skipped block, or live edits stall through a silent
  passage.
- **D12. Model-only edits** (#1260). The mask covers `session_id`, `render_profile.id`,
  `output_profile.id` and `automation`. The automation mask holds only while no host renders stored
  automation (#1058); the first issue that renders it takes `automation` out of the mask.
- **D13. The input section** (#1261, #1262). Trim and polarity ride the strip's input lane; HPF and
  LPF ride it as prepared filter targets that host-core's `InputFilterPreparer` designs on the
  control thread. The tail is bounded (D15-4): the joint SVF flush (#1328) and the engine-wide tail
  contract (#1329) give every strip a finite tail and an exact-rest bound, and #1261 and #1262
  report it, never `Infinite`. `delay_samples` stays a rebuild (decision 14, rule 1), with a D15-9
  transition.
- **D14. Effects** (#1263-#1266, closed). A parameter is live when its descriptor's
  `automation_rate` is `Block`. Bypass rides the latency-preserving shunt, except for the delay and
  the multiband compressor, whose session bypass is prepared until *Give the delay a live bypass
  shunt* (#1339) and *Give the multiband compressor a live bypass shunt* (#1340) land (D15-13 E4);
  until then a change of it is a rebuild with a D15-9 transition. With cells (#1312) a transaction
  can no longer exceed an effect lane's capacity, so the "rebuild instead of endless
  `BACKPRESSURE`" rule (`control.rs:1051`) is deleted with the queue.
- **D15. The span window (D15-5, *Size each effect's automation span window from the producers its
  plan has*, #1306).** It is sized once, from the producers the plan really has:
  the live lane depth plus the stored-automation spans per block that preparation computes, after
  #1058's design. The caller's S bounds only `AUTOMATION_ENQUEUE` density.

## Guards

Each guard keeps a delta structural until a later slice composes what depends on it. The slice that
lifts a guard marks its bullet here as superseded.

- **G1. A submix strip's fields.** Structural by construction: D1 masks only track fields. Lifted
  for faders, pans and sends by #1225, and for the input section and effects by #1267.
- **G2. The mute of a follow source.** Lifted by #1226, which makes `follows_mute` live (D15-6, F3).
- **G3. Any VCA.** Lifted by #1247, which makes VCA offsets, mutes and membership live (D15-6, F9).
- **G4. EQ parameters.** Superseded: lifted by #1265 (`b055a48d4`) for every EQ parameter whose
  `automation_rate` is `Block`. A band's `enabled` and `kind` become live in *Make a parametric EQ
  band's enabled and kind live* (#1337, D15-13 E2).

## Slices

**Closed:** #1253, #1254, #1255, #1256, #1257, #1258, #1260, #1263, #1264, #1265, #1266.

**Open.** Each row's "Depends on" is the slice spec's own "Dependencies" section; the spec
governs. Stream F is C ABI live completeness, built on cells.

| Issue | Title | Stream | Depends on |
|---|---|---|---|
| #1312 | *Hold live values in latest-target cells on both hosts* | B | #1309, #1348 |
| #1345 | *Hold effect parameter, bypass and EQ-target values in latest-target cells* | B | #1312, #1399 |
| #1346 | *Hold strip input-lane values in latest-target cells* | B | #1312 |
| #1347 | *Hold route-lane values in latest-target cells* | B | #1312 |
| #1399 | *Report live_values_superseded in the browser status and prove both hosts drain strip cells alike* | B | #1312 |
| #1313 | *Report each transaction's edit path in its response* | B | #1309 |
| #1314 | *Publish an applied-revision watermark and complete edits asynchronously* | B | #1309, #1343 |
| #1364 | *Resolve an absent live ramp to the session default on the browser and in the SDK* | E | #1054 |
| #1365 | *Edit control_smoothing by a session transaction, model-only* | E | #1054 |
| #1393 | *Crossfade the browser's live bypass command over the session ramp* | E | #1341, #1364 |
| #1388 | *Run the blinded listening session for the live ramp defaults* | E | #1054, #1055, #1364 |
| #1394 | *Carry an optional per-edit ramp length on live session edits* | E | #1054 |
| #1268 | *Elide a builtin input filter section again after a live disable settles it to identity* | F | none |
| #1261 | *Apply value-only input trim and polarity edits to the running C ABI plan* | F | #1054, #1309, #1312, #1328, #1329, #1346, #1394 |
| #1262 | *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets* | F | #1261, #1268, #1312, #1328, #1329, #1346 |
| #1225 | *Deliver value-only send edits to the running C ABI plan* | F | #1054, #1277, #1284, #1309, #1312, #1313, #1347, #1394 |
| #1390 | *Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan* | F | #1054, #1225, #1277, #1309, #1312, #1394 |
| #1226 | *Let C ABI sends follow their source strip's mute live* | F | #1054, #1225, #1309, #1312, #1347, #1390, #1394 |
| #1247 | *Deliver value-only VCA edits to the running C ABI plan* | F | #1054, #1225, #1226, #1309, #1312, #1347, #1390, #1394 |
| #1267 | *Apply value-only submix-strip input-section and effect edits to the running C ABI plan* | F | #1261, #1262, #1309, #1312, #1345, #1346, #1390 |
| #1306 | *Size each effect's automation span window from the producers its plan has* | F | #1058, #1304, #1309, #1345 |

Prerequisites owned elsewhere: *Extract the C ABI control plane into a portable crate both hosts
call* (#1309, stream B); *Session `controlSmoothing`: configurable ramp lengths for live mute,
fader and pan changes* (#1054, stream E, after #1055); *Flush the SVF jointly so builtin and EQ
filters reach exact rest* (#1328) and *State a bounded tail and an exact-rest bound for every node*
(#1329), stream G; *Research: render stored session automation in the engine, identically on every
platform* (#1058, stream K); the carry slices #1277 and #1280 (#1269, stream A). The browser
halves are under #1269: *Admit browser live edits in the Worker through the committed model*
(#1382), *Publish the applied-revision watermark in the browser status* (#1349), and *Make a send's
follows_mute live in the browser* (#1342) for #1226. `miso_engine_v1_service` is *Add
miso_engine_v1_service for bounded control work between edits* (#1348, under #1269).

**Batch follow-ups outside the umbrella** (filed 2026-10-04 after PR #1298): #1302, #1303 (after
#1277), #1304, #1305.

## Closing the umbrella

Every open slice in the table is closed with a Sol PASS and its evidence is upstream.

## Out of scope

- Ramp lengths: #1054 (D3 is the seam).
- Stored automation rendering (#1058).
- Solo on the C ABI. The app composes solo as mutes in one transaction, which is live.
- #444's block-atomic batch claim, and sample-accurate transient automation.
- The fused fader-and-matrix pass on the C ABI. It needs `BetweenRenderCalls`, which a concurrent
  C ABI producer cannot declare.

## Answered by decision 15

- **Q1** (ship steps first?): superseded by D15-1. Every live value ramps (#1054); a step remains
  only as an explicit 0.
- **Q2** (`BACKPRESSURE` on a paused transport after 16 edits): D15-2. Live values are
  latest-target cells, so a paused host is never refused for one (#1312).
- **Q3** (tell the host which path ran?): D15-3 and D15-17 reverse the earlier "not now". The
  response carries the path (#1313), and the applied-revision watermark reports when and how it
  took effect (#1314).
- **Q4** (an infinite tail for live input filters?): D15-4. No; the SVF reaches exact rest (#1328),
  every node states a bounded tail (#1329), and #1261 and #1262 report it.
- **Q5** (#1306, formerly "size effect span windows by lane depth"): D15-5. The window is sized once from the
  plan's real producers, after #1058's design.

## History of the 2026-09-28 spec

- **A0 (the swap race).** Delivered by #1042.
- **Draft 02, its step 4 and A4.** Delivered by #1056.
- **A5.** Recorded in `docs/CONTROL_PROTOCOL_SEMANTICS.md`: #140 is descoped, stored automation is
  #1058.
- **Scope items 1-6, A1 and A2 D1-D4.** Became D1-D11 here; A1.2 is D5, A1.4 is D6.
- **"L2" and "L3"** became #1261-#1266.
- **A3's gates.** Became the gates of #1257 and #1258; gate 7 moved to #1255.
