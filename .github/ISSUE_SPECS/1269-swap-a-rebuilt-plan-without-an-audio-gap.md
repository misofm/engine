# Swap a rebuilt plan without an audio gap

Umbrella. Planner brief of 2026-10-04 (Sol), rewritten on 2026-10-05 for decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7 to D15-11,
D15-14 and D15-17). Code anchors verified on `main` at `6fb211594`. The 2026-10-04 text and its
two adversarial verifications are in git history
(`git show 6fb211594:.github/ISSUE_SPECS/1269-swap-a-rebuilt-plan-without-an-audio-gap.md`).

## Owner direction (2026-10-04)

> "I've never used a DAW where adding a track would cause an audio dropout."

A structural
edit (add a track, add an effect, reroute) rebuilds the plan. A DAW keeps the playback position
and the state of every unchanged node across that rebuild; the engine must do the same. Decision 14
keeps value changes on the live path even after swaps become seamless (its C2). Decision 15 is
bound by the owner principle of 2026-10-05: "Please make the decisions based on maximizing
long-term reliability and correctness. We shouldn't take any shortcuts that need to be fixed or
worked around in the future."

## Problem (verified)

Phase 1 (below) delivered the hand-over hook, source continuity and the strip input section. What
remains on `main`:

- **Most DSP state still starts at rest at a swap.** The plan exchange hands the outgoing plan to
  the successor once, after the clock adoption (`RealtimePlanOwner::enter_block`,
  `crates/engine/src/realtime/plan_exchange.rs:375`; `adopt_absolute_sample` then `carry_from` at
  `:418-421`; `RealtimePlan::carry_from`, `crates/engine/src/realtime/plan.rs:917`). Today the carry
  moves source consumers and the input section only. Fader and pan ramps, effect lanes, per-node
  effects, compensation and delay lines restart.
- **A pending candidate refuses the next structural edit** with `BACKPRESSURE`
  (`crates/capi/src/runtime/control.rs:959-961`), permanently so while the host is paused.
- **A latency change is not continuous**, and a removed or edited strip stops or restarts abruptly.
- **The browser has no structural edit path.** `AudioWorkletEngineHost` owns one prepared host for
  its life (`ReadyOwnership`, `hosts/host-web/src/lib.rs:1509`). The engine is one Wasm instance
  created inside the `AudioWorkletProcessor`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:286`), and shared memory is refused
  (`scripts/check-web-audioworklet.sh:372-373`).

## Outcome

On both hosts, a structural edit while audio plays has no gap:

- every unchanged node keeps its exact state, so its output continues bit for bit;
- every unchanged source keeps its ring, its generation and its read position;
- a new node starts at rest; a new source starts at an exact render sample the host chooses;
- a strip whose state cannot continue (added, edited in a prepared value, removed) passes through a
  declared ramp, never a click (D15-9);
- a latency change is exact on every carried path (D15-8); a strip it restarts takes its D15-9
  transition, and any fallback is counted and reported;
- no acknowledged edit is lost, and every committed revision completes observably (D15-17);
- the swap block allocates nothing, frees nothing, takes no lock and makes no syscall.

The acceptance shape every carry slice uses: render the old plan, apply the structural edit, render
on. Compare every block with a reference: the post-edit session compiled fresh and fed the same PCM
from sample zero, with each added strip contributing exact zeros before the swap (muted, or fed
zeros). The streams must be bit-identical. The test signal has no exact-zero sample, so
`+0.0`/`-0.0` sums stay out of the comparison. The master reduction sums its inputs left to right in
edge-ID order, so a muted strip's `+0.0` term moves no bit. A slice that adds a transition (fade-in,
duck-swap, removal) compares every unchanged path bit for bit and checks the transition's envelope
on the edited strip.

## Design (P1-P13; frozen by decision 15 where noted)

### P1. What carries: the "unchanged" rule

A state owner carries from the predecessor plan to the successor when all of these hold:

1. **Same stable key.** Sources by source ID. Graph nodes by `GraphNodeId`
   (`crates/graph/src/lib.rs:273`): strip stages by `(strip ID, TrackStage)`, effects by
   `(strip ID, rack, effect ID)`, compensation lines by `GraphEdgeId`, routes by route ID. Never by
   index, lane or unit position.
2. **Same envelope.** Rate, quantum and output shape. The exchange refuses a different envelope
   (`PlanPublisher::reserve_replacement`, `plan_exchange.rs:265`).
3. **Same prepared layout.** Native effect identity, quality, link mode, prepared bypass, prepared
   latency, `state_sizes` and `state_layout_version` (`PreparedEffectMetadata`,
   `crates/effect-contract/src/lib.rs:1112`); a source's `PcmSourceRingConfig`. These facts come from
   the predecessor's **state inventory**, recorded when it was prepared.
4. **Its prepared values are unchanged (D15-7).** The base of the comparison is **the committed
   model the predecessor plan was prepared from, plus every live record pushed to that plan**. It is
   not "the model before this transaction": with candidate supersession (#1310) the transaction
   before this one may never have rendered, and the plan being displaced is the one render runs.
   By #1053 D9 this base is what the predecessor renders or is about to render. Compare `f32` by
   bits.

**Carry, then retarget (D15-7).** When the same structural transaction changes an owner's **live**
values (decision 14 live rows), the owner still carries: the carry copies its state, then the
classifier's ramped records (`classify_live_delta`, `crates/host-core/src/live_delta.rs:211`) go to
the successor's lanes, exactly as a value edit would. The result is bit-identical to "live edit,
then structural edit". When it changes a **prepared** (decision 14 rule 3) value, the owner cannot
be retargeted: it restarts at rest behind a D15-9 transition (P4).

Placement does not matter. A lane may move to another bank, or between a bank and a per-node
instance; that changes no bit (AGENTS.md: banking "may couple lanes' cost, never their bits").

### P2. Where state moves

- **Off the render thread:** the successor is prepared from the committed model. A join of the
  predecessor's inventory, the two models (P1) and the successor's own inventory builds a **carry
  program**: a flat list of `(owner kind, successor location, predecessor location)` operations,
  installed in the successor before publication. All allocation, string comparison and validation
  happen here.
- **On the render thread, at the swap block:** `enter_block` owns both plans between the swap and
  the retirement push, and runs the carry program inside the render scope before the successor
  renders.
- **Mechanisms:**
  - **Move:** swap owned storage between the plans. A few words, whatever the state size.
  - **Lane copy:** through the payload methods every native effect implements
    (`snapshot_track_state_payload` and `restore_track_state_payload`,
    `crates/effect-contract/src/lib.rs:2070-2079`; the per-node pair at `:1930` and `:1975`) and one
    scratch buffer the successor preallocates. #1278 made every restore allocation-free and exact
    mid-ramp. Builtins copy plain-data lane state between two instances of the same concrete type
    (`as_any_mut`, `crates/graph/src/lib.rs:1291`).
  - **Claim-line fill (D15-8):** at a warm adoption, render fills each grown source claim line from
    the predecessor's pending frames and a raw-frame prime (P7). It copies no plan state. The carry
    runs in move mode only: the D15-8 round-5 amendment retired copy-mode carry (#1322, #1362 closed
    as not planned).
- **Bounded work:** the program's length and copy bytes are known when it is built. The swap
  block's cost is measured on the 64-track console (#1286).

R6b is not involved: "in-memory state the engine hands across a plan replacement is not persisted
state" (AGENTS.md). Nothing is persisted, versioned or migrated.

### P3. Sources keep playing (delivered)

Delivered by #1271-#1275: an unchanged source keeps its ring through a vacant successor entry; the
control side moves its producer after the protocol commit (`control.rs:1010-1021`, infallible); a
new source starts at an exact render sample through the anchored seek
(`miso_engine_v1_source_seek_at`). Decision 15 D15-12 anchors every seek on the plan's
source-read clock (*Anchor every seek on the plan's source-read clock*, #1316), and a held
`seek_at` moves with its producer when a candidate is superseded (#1310, #1319).

### P4. Added, edited and removed strips (D15-9)

- **Added strip:** fades in from its first played block, over the session mute ramp (*Fade in a
  strip that a swap adds during playback*, #1288).
- **Edited strip** whose state cannot continue (a changed prepared value; an insert added, removed
  or reordered; quality; link mode; `delay_samples`): a **duck-swap**. The live mute ramps down on
  the predecessor, the swap is scheduled at the ramp's end, and the successor fades in (*Duck-swap a
  strip whose state cannot continue across a plan swap*, #1324). Every other strip carries.
- **Removed strip:** two phases. Phase 1 ramps its live mute to zero on the running plan; phase 2
  is a scheduled swap at the ramp's end. Its source retires with phase 2, not with the commit (*Remove
  a strip in two phases: ramp out, then a scheduled swap*, #1325). Until phase 2, a submit to that
  source is accepted; after it, refused with `source.id.unknown`.
- **Mechanisms:** a scheduled swap ("adopt no earlier than S", *Adopt a successor plan no earlier
  than a scheduled sample*, #1311); candidate supersession by compare-and-swap (#1310, on #1343 and #1344); live strip fader and
  mute lanes on every plan, on both hosts (the C ABI has them since #1256; the browser gains them in
  #1326).
- **A route added to or removed from a surviving strip** ramps in or out (*Ramp a route that a
  plan swap adds to or removes from a surviving strip*, #1363).
- A planned transition is the designed path, not a fallback: its revision completes as `exact`,
  with no fallback flag (P9).
- Ramp lengths come from the session's `controlSmoothing` (D15-1, #1054).

### P5. Live controls: no admitted record is lost

- **Builtins and live sends** apply a drained record to lane state directly. The carry drains the
  predecessor's lane (bounded, #1253) and then copies it.
- **Effects** only stage a drained record; `process` applies it (`EffectControlLane::stage`,
  `crates/effect-contract/src/live.rs:341`; banks: `LiveControlEffectBankStage`,
  `crates/rack/src/lib.rs:937`, drained at `:1257`). So the successor's lane **inherits** the
  predecessor lane's unconsumed controls and applies them before its own in its first block.
- A record admitted while a successor is pending goes to the newest candidate (#1257's rule;
  `commit_live` targets `pending_providers.last_mut()`, `control.rs:1089`). A pending warm
  successor is an ordinary pending candidate (#1053 D7, D15-17): the edit goes to its cells and
  applies at adoption. Its retargets were written once, at preparation (#1277 D5).
- **Latest-target cells (D15-2, #1312)** replace the live value queues on both hosts. A cell holds
  its target and ramp as one unit, with a per-lane dirty mask. The inheritance rule carries over: a
  dirty predecessor cell is applied by the successor in its first block. The queue-capacity split
  the 2026-10-04 text required while a successor is pending disappears with the queue.

### P6. No crossfade between the two plans' outputs

Rendering both plans for a crossfade costs two plans' work in one callback, and each ring has one
render consumer (`PcmSourceConsumer`, `crates/source/src/lib.rs:993`). With P1-P5 every unchanged
path is bit-continuous, and D15-9's per-strip transitions treat the rest. A true crossfade (ghost
strips) is deferred by D15-9; it reopens only on a measured, audible dip in a listening test.

### P7. Latency and PDC across a swap (D15-8)

PDC delays each edge into a node by that node's latest incoming arrival minus the edge's own arrival
(`crates/graph-compiler/src/pdc.rs:59-89`), at **every** node with several inputs: the output, every
submix input, every effect with a sidechain. Compensation lines carry by `GraphEdgeId` (#1283).

- **No surviving node's arrival changes:** every unchanged edge keeps its length, its line carries,
  and every unchanged path is bit-continuous.
- **A node's arrival would drop:** floors. The successor is compiled with every surviving node's
  arrival floored at the predecessor's value (*Keep every node's latency from dropping during
  playback*, #1285). Floors and the accumulated read-ahead reset at a discontinuity the host
  declares: a stop, or a seek of every source (*Reset latency floors at a host-declared
  discontinuity*, #1323).
- **A node's arrival would grow by `P`: a primed warm successor** (*Grow latency during playback
  by adopting a primed warm successor*, #1287; D15-8 (round-5 amendment)). "History fill" is
  impossible: the samples needed are future processed samples, which the host already queues as
  raw source frames. The successor's nodes split into carried nodes (C), the nodes of the strips it
  restarts (R, #1324's `restarted_strips()`) and added nodes (N).
  1. Submit prepares the successor with floors at `a(n) + P` on carried nodes only (R keeps
     #1285's floors, N has none). `Δ` is the largest arrival growth over C, `P = q·⌈Δ/q⌉`, and the
     source-read offset is the predecessor's plus `P` (*Give a plan a source-read clock that leads
     its render clock*, #1396). Preparation checks that every carried node arrives at exactly
     `a(n) + P`; a node that does not is `WarmUnavailable::Misaligned`, its strip is restarted whole
     and `Δ` recomputed, and if it cannot be restarted the edit takes the transition (*Prepare a warm
     successor whose carried nodes lead the predecessor by P*, #1354).
  2. Submit publishes it as `Primed { not_before, lead_blocks }` (#1311).
  3. Render checks readiness on the **active** plan's consumers before it claims: S is at or after
     `not_before`, every carried source has its next `P/q + 1` blocks queued and playable, and no
     command or held seek is anchored in the prime window (*Let a source consumer check and replay
     its next blocks for a prime*, #1320). Once ready, it claims and adopts in move mode in the
     same block, then replays `P/q` blocks per carried consumer (`prime_block_at`) into the claim
     lines (*Adopt a warm successor with a raw-frame prime at the first ready block*, #1355).
  4. Live edits while it is pending are ordinary pending-candidate edits (#1053 D7). A structural
     edit supersedes it (#1310). A host-declared stop supersedes it by a plain rebuild (#1323).
  5. `ΣP` is bounded by `P_MAX`, and the prime by `PRIME_BYTES_MAX` (#1286).
  6. The C ABI checks the deadline in `miso_engine_v1_service` (#1348, *Check the warm-successor
     deadline in miso_engine_v1_service and report its outcome*, #1360); the browser checks it in
     the Worker's service loop (*Check the warm-successor deadline in the browser Worker's service
     loop and report its outcome*, #1361).
- **Fallback, counted and reported** through the watermark's outcome flags (#1314): the transition
  only, the duck-swap of the strips whose arrival grows (*Duck-swap the strips a latency growth
  restarts, and fall back to the transition when a warm successor cannot adopt*, #1397), counted
  `TRANSITION_FALLBACK`. It applies on `WarmUnavailable` at submit, or when readiness is still
  unmet `PRIME_DEADLINE_SAMPLES` of render after publication (*Fall back to the transition when a
  warm successor is not ready by its deadline*, #1358). The deadline is counted in render samples,
  so a paused host never falls back (D15-17). There is no render-thread pre-roll. #1286 derives
  `P_MAX` and `PRIME_BYTES_MAX`, with the browser row from the spike (#1331).
- **No permanent latency reserve.** A reserve is a permanent cost on every session.

VST3 states that a plug-in's latency change may interrupt playback, because the host must recompute
its delay compensation (`IAudioProcessor::getLatencySamples`,
https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html).
Fixed-latency consoles avoid the case by giving every path the same latency. The engine keeps
latency exact without that fixed cost.

### P8. Retirement, deferral and supersession

The successor's reservation holds a retirement credit (`reserve_replacement`, `plan_exchange.rs:265`).
A deferred swap leaves the candidate pending and the predecessor rendering; the carry runs only in
the block that applies the swap, so it copies the latest state. A predecessor mutated by the carry
is retired and reclaimed off the render thread. A structural edit while a candidate is unadopted
replaces it by compare-and-swap (#1310; withdrawal #1343, preparation across a withdrawn candidate
#1344), which removes the refusal at `control.rs:959-961`. If
render has already taken the candidate, the newer candidate's carry program is retargeted to the
adopted plan.

### P9. Acknowledgements: can an ack ever precede a drop? (D15-17)

- **Structural transaction.** Submit validates, classifies, prepares the successor and reserves its
  publication slot and retirement credit (`control.rs:962`), then commits (`:1007-1009`) and writes
  the response (`:1033`). Preparation stays in submit because it can fail; nothing is acked before
  it succeeds. Submit never waits for render, a swap or a warm adoption.
- **Completion.** The response carries `{revision, path}` (#1313). A committed revision is pending
  until the applied-revision watermark covers it (#1314; browser status #1349). It completes as
  `exact`, with a counted fallback (`transition_fallback`), or as `superseded`
  into a later revision whose committed model contains it. Never as nothing.
- **A planned transition completes as `exact`.** An added strip's fade-in (#1288), an edited strip's
  duck-swap (#1324), a two-phase removal (#1325) and a route ramp (#1363) are the designed outcome
  of their edit, so their revision completes `exact` with no fallback flag. `transition_fallback`
  is set only when a warm successor cannot adopt (`WarmUnavailable`, or a missed deadline), and the
  fallback transition, the duck-swap (#1324), runs instead (#1397, #1358).
- **Source PCM.** Accepted PCM for a persisting source stays in the carried ring. Accepted PCM for a
  removed source is discarded with phase 2 of its removal (P4).
- **Live records.** P5. None is dropped; a superseded cell value is counted (D15-2).

### P10. Meters and observation (D15-14)

- C ABI: the render-peak telemetry reads the output (`SharedPlanState`,
  `crates/capi/src/runtime/plan.rs:5`); it is continuous.
- Meter, observation and spectrum state of an unchanged owner carries under P1, on both hosts (*Carry
  meter and effect observation state across a plan swap*, #1327). This replaces the 2026-10-04
  plan, which restarted the browser's windows at every swap. Only a new or restarted owner's windows
  restart, and a window open at that owner's swap is counted as a loss, never relabelled.

### P11. A wrong predecessor cannot happen, and is harmless if it does

Each bound graph plan gets a process-unique identity at bind, and a carry program names its
predecessor's identity. With supersession (#1310) the program is retargeted to the adopted plan. If
the identities ever differ, the hook moves nothing, the successor's non-source owners start at rest,
its vacant sources render `+0.0`, and `carry_mismatched` counts it (`plan_exchange.rs:425`).

### P12. The browser: one control plane, off the audio thread (D15-10, D15-11)

- Wasm threads on one shared `WebAssembly.Memory`. A Worker instance runs the control plane (model,
  classifier, preparation, the warm-successor deadline check, retirement, disposal); the AudioWorklet instance only renders
  and swaps, and never allocates or frees after boot (*Run the browser control plane in a Worker and
  keep the AudioWorklet render-only*, #1332). The module imports one shared memory at every
  instantiation site (#1380); plans swap and retire through the Worker's service loop (#1381); live
  edits are admitted in the Worker through the committed model (#1382). The control plane is the C
  ABI's, extracted into one portable crate (*Extract the C ABI control plane into a portable crate
  both hosts call*, #1309).
- The allocation rule is enforced by a runtime counter and static checks (*Gate AudioWorklet render
  against allocation statically and at runtime*, #1333). The artifact builds on a pinned dated
  nightly with `-Zbuild-std` (#1334). The spike (#1331) proves two instances on one shared memory in
  three browser engines and on iOS before #1332 starts.
- Cross-origin isolation is required for structural edits. A non-isolated page keeps the same API
  and artifact; its structural edit runs the blocking rebuild, reported and counted. Warm latency
  growth needs no isolation: both pages adopt a `Primed` successor and report it `exact` (#1361).
- **One edit API.** The SDK gains `engine.apply(transaction) -> {revision, path}` plus the
  watermark. `replaceSession(document)` exists only as a convenience that diffs the document
  against the committed model into one transaction (#1386). The SDK builds and encodes
  transactions (#1383, #1385). The worklet message protocol stays internal. #1290 and #1293-#1297
  are rewritten on `engine.apply`.
- #1291 and #1292 close as not planned: the committed model replaces their three-way merge.

### P13. One implementation shape

The carry is portable core code (`engine`, `graph`, `rack`, `source`, `effect-contract`, the effect
crates, `builtins-compiler`, `host-core`, the control-plane crate). The C ABI and the browser adapter
only call it. No target-specific path. The per-node builtin processors are test-only, so builtins
carry only between bank lanes.

## State owners and their slices

| Owner | Where | Mechanism | Slice |
|---|---|---|---|
| Source consumer and producer | `PcmSourceConsumer` (`source/src/lib.rs:993`), `SourceControlSet` (`host-core/src/source.rs:127`) | move | #1271-#1273 (closed) |
| Strip input section | `BuiltinBankProcessor` (`builtins-compiler/src/lib.rs:428`) | drain, lane copy | #1276 (closed) |
| Fader/mute and pan/matrix ramps | `FaderBankProcessor`, `MatrixBankProcessor`, `FaderMatrixBankProcessor` (`:715`, `:778`, `:831`) | drain, lane copy | #1277 |
| Console effect lanes | `EffectBankStage` (`rack/src/lib.rs:750`) | lane copy (payload) | #1279 |
| Live-controlled effect lanes | `LiveControlEffectBankStage` (`rack/src/lib.rs:937`) | lane copy, inherited controls | #1280 |
| Insert lanes that change representation | bank lane and `NodeKind::Effect` / `LiveControlEffect` | lane copy (payload) | #1281 |
| Per-node effects | `NodeKind::LiveControlEffect` (`graph/src/runtime.rs:1143`, `:1184`) | move | #1282 |
| Compensation lines | `CompensationDelay` (`graph/src/runtime.rs:940`) | move, or a head-aligned copy for a changed length | #1283 |
| Strip and submix delay lines, live send ramps | `TrackDelayLine` (`:1021`), `LiveRoute` (`:861`) | move (delay lines); drain, then copy (send ramps) | #1284 |
| Meters, observation taps, spectrum | builtins meters, observation lanes | copy | #1327 |
| Source claim lines (after a warm growth) | `pdc_delay_block` claim lines, keyed by claiming node and source | move; a grown line is filled from pending frames and the prime (D15-8 L2, L3) | #1287 |

## Slices

Each slice is its own issue with its spec in `.github/ISSUE_SPECS/`. Rows within a stream are in
merge order. Streams are decision 15's (A swap carry, B control plane, C latency growth, D
transitions, E smoothness, H browser control plane).

**Closed (phase 1):** #1270, #1271, #1272, #1273, #1274, #1275, #1276, #1278, #1289 and #1071.
**Close as not planned (D15-11):** *Keep browser live strip state across a session replacement*
(#1291) and *Keep browser live effect edits across a session replacement* (#1292).

Each row's "Depends on" is the slice spec's own "Dependencies" section; the spec governs.

| Issue | Title | Stream | Depends on |
|---|---|---|---|
| #1300 | *Let soft-clip restore its own non-finite history* | A | none |
| #1277 | *Carry fader, mute and pan ramps across a plan swap* | A | #1312 |
| #1279 | *Carry console effect lanes across a plan swap* | A | #1277 |
| #1280 | *Carry live-controlled effect lanes across a plan swap* | A | #1279, #1312, #1345 |
| #1281 | *Carry an insert lane that moves between a bank and a per-node instance* | A | #1280 |
| #1282 | *Carry per-node effect instances across a plan swap* | A | #1281 |
| #1283 | *Carry compensation lines across a plan swap* | A | #1282 |
| #1284 | *Carry strip delay lines and live send ramps across a plan swap* | A | #1283 |
| #1285 | *Keep every node's latency from dropping during playback* | A | #1284 |
| #1323 | *Reset latency floors at a host-declared discontinuity* | A | #1285, #1309, #1310, #1311, #1314 |
| #1286 | *Record the swap block's cost on the 64-track console* | A | #1284, #1331, #1354, #1355 |
| #1327 | *Carry meter and effect observation state across a plan swap* | A | #1284 |
| #1395 | *Carry spectrum capture state across a plan swap* | A | #1327, #1401 |
| #1343 | *Let the control thread withdraw an unadopted candidate plan* | B | #1309 |
| #1344 | *Prepare a successor across a withdrawn candidate plan* | B | #1277 |
| #1398 | *Size the C ABI's plan capacities and resource admission for a superseding candidate* | B | #1309 |
| #1310 | *Supersede an unadopted candidate plan by compare-and-swap* | B | #1309, #1314, #1343, #1344, #1398 |
| #1311 | *Adopt a successor plan no earlier than a scheduled sample* | B | #1309, #1314, #1343 |
| #1348 | *Add miso_engine_v1_service for bounded control work between edits* | B | #1309, #1311, #1314 |
| #1349 | *Publish the applied-revision watermark in the browser status* | B | #1309, #1314, #1348, #1381, #1399 |
| #1287 | *Grow latency during playback by adopting a primed warm successor* | C | #1283, #1285 |
| #1396 | *Give a plan a source-read clock that leads its render clock* | C | #1316, #1323 |
| #1320 | *Let a source consumer check and replay its next blocks for a prime* | C | #1316, #1318, #1319 |
| #1354 | *Prepare a warm successor whose carried nodes lead the predecessor by P* | C | #1277, #1285, #1286, #1287, #1324, #1396 |
| #1355 | *Adopt a warm successor with a raw-frame prime at the first ready block* | C | #1277, #1287, #1310, #1311, #1314, #1320, #1323, #1327, #1343, #1344, #1354, #1395, #1396 |
| #1397 | *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm successor cannot adopt* | C | #1288, #1311, #1314, #1324, #1325, #1343, #1344, #1354, #1355, #1396, #1398 |
| #1358 | *Fall back to the transition when a warm successor is not ready by its deadline* | C | #1286, #1314, #1343, #1354, #1355, #1396, #1397 |
| #1360 | *Check the warm-successor deadline in miso_engine_v1_service and report its outcome* | C | #1309, #1311, #1313, #1314, #1323, #1348, #1351, #1354, #1355, #1358, #1397, #1398 |
| #1361 | *Check the warm-successor deadline in the browser Worker's service loop and report its outcome* | C | #1290, #1293, #1294, #1331, #1332, #1333, #1349, #1355, #1360, #1381 |
| #1326 | *Give every browser plan live strip fader and mute lanes* | D | none |
| #1288 | *Fade in a strip that a swap adds during playback* | D | #1054 |
| #1325 | *Remove a strip in two phases: ramp out, then a scheduled swap* | D | #1054, #1288, #1309, #1310, #1311, #1312, #1313, #1314, #1347, #1363, #1391 |
| #1324 | *Duck-swap a strip whose state cannot continue across a plan swap* | D | #1277, #1288, #1310, #1325, #1363, #1391 |
| #1363 | *Ramp a route that a plan swap adds to or removes from a surviving strip* | D | #1054, #1283, #1284, #1285, #1288, #1310, #1314 |
| #1391 | *Give every route whose tap precedes its strip's fader a live lane on every plan* | D | #1225, #1326, #1347 |
| #1392 | *Keep an added strip's pending fade-in across a later plan swap* | D | #1277, #1283, #1284, #1288, #1363 |
| #1380 | *Ship the browser module with one imported shared memory at every instantiation site* | H | #1331, #1333, #1334 |
| #1332 | *Run the browser control plane in a Worker and keep the AudioWorklet render-only* | H | #1057, #1331, #1333, #1334, #1380 |
| #1387 | *Move browser source submission and seeks into the Worker* | H | #1316, #1318, #1332 |
| #1401 | *Prepare every browser preparation branch concurrently, as a successor too, in host-core* | H | #1326 |
| #1400 | *Prepare through an adapter-supplied preparer in the control-plane crate* | H | #1309, #1326, #1401 |
| #1381 | *Swap and retire browser plans through the Worker's service loop* | H | #1309, #1314, #1327, #1332, #1348, #1387, #1395, #1400 |
| #1382 | *Admit browser live edits in the Worker through the committed model* | H | #1054, #1057, #1225, #1226, #1247, #1261, #1262, #1267, #1312, #1313, #1332, #1345, #1346, #1347, #1364, #1381, #1390, #1394 |
| #1290 | *Replace the running browser session in the Rust host* | H | #1277, #1309, #1310, #1313, #1314, #1326, #1327, #1332, #1348, #1349, #1381, #1382, #1387, #1395, #1400, #1401 |
| #1293 | *Export transaction apply and anchored seek from the browser engine module* | H | #1290, #1309, #1313, #1316, #1319, #1332, #1381, #1387 |
| #1294 | *Send a session transaction to the browser control plane* | H | #1293, #1310, #1332, #1348, #1349, #1381, #1382, #1386, #1387 |
| #1295 | *Qualify a structural browser edit in real browsers* | H | #1290, #1294, #1332, #1333, #1386 |
| #1383 | *Build and encode session transactions in the SDK* | H | #1394 |
| #1385 | *Encode the session, submix, output, route, automation and VCA edits in the SDK* | H | #1335, #1383, #1394 |
| #1386 | *Diff a replacement document against the committed model and export replace from the browser engine module* | H | #1290, #1293 |
| #1296 | *Apply session transactions from the browser SDK* | H | #1294, #1295, #1312, #1313, #1314, #1325, #1326, #1349, #1382, #1383, #1385, #1386 |
| #1297 | *Feed and retire the sources a browser edit adds or removes* | H | #1293, #1296, #1316, #1325, #1332, #1381, #1387 |
| #1389 | *Apply session transactions from the headless SDK engine* | H | #1293, #1296, #1332, #1381, #1382, #1383, #1385, #1386 |

**Prerequisites owned elsewhere.** Under #1053: *Extract the C ABI control plane into a portable
crate both hosts call* (#1309), *Hold live values in latest-target cells on both hosts* (#1312),
*Report each transaction's edit path in its response* (#1313), *Publish an applied-revision
watermark and complete edits asynchronously* (#1314), and the stream F slices. Seek contract
(stream B): *Anchor every seek on the plan's source-read clock* (#1316), *Report held source
blocks apart from underruns* (#1318), *Test held seeks across swaps and supersession, and add a
seek to audit capi* (#1319), *Tighten the seek entry points: source.id.invalid, a typed held
preparation, timed reads only* (#1350). Browser toolchain (stream H(a)): *Prove two Wasm instances
on one shared memory in three browser engines and on iOS* (#1331), *Gate AudioWorklet render
against allocation statically and at runtime* (#1333), *Build the browser artifact on a pinned
nightly toolchain* (#1334). Ramps (stream E): #1054, *Carry an optional per-edit ramp length on live
session edits* (#1394), *Resolve an absent live ramp to the session default on the browser and in
the SDK* (#1364). Automation guard (stream I): *Refuse automation on
effect parameters that are not block-rate* (#1335). Design note: *Design: one edit API on every host
over the core's committed session model* (#1057).

**First product slice of phase 2.** #1277: on the C ABI, a structural edit keeps every unchanged
strip's fader and pan ramps. #1279-#1284 extend that to every state family, one family per slice.

**Closing.** The umbrella closes when every slice in the table has closed with a Sol PASS and its
evidence is upstream, and #1291 and #1292 are closed as not planned.

## Coordination

- **#1053** (*Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes*). Its core slices are closed; #1257's newest-candidate rule is P5's second
  half. Its remaining slices (#1225, #1226, #1247, #1261, #1262, #1267) are built on cells (#1312),
  and #1261 must route input records to the newest plan, as `commit_live` does.
- **Decision 14** (#1259). Its C1 (one edit API) is met on the browser by D15-11 (P12). Its F4 (the
  delay's and the multiband compressor's prepared bypass) is answered by D15-13 E4: *Give the delay
  a live bypass shunt* (#1339) and *Give the multiband compressor a live bypass shunt* (#1340); until
  then lifting that bypass is a rebuild with a D15-9 transition, and #1282 asserts it.
- **#1057** (*Design: one edit API on every host over the core's committed session model*) is the
  D15-11 design note; its personal-mix question blocks nothing here.
- **#1054 and #1055** (ramp lengths, D15-1) give #1288, #1324, #1325 and #1363 their lengths.
- **#888, #889** (partial insert cohorts bank) and **#1107** (silent-lane skips in console banks)
  change bank shapes and bank-level state; whichever lands after #1279-#1281 keeps their gates.
- **#1074** (*Walk the engine plan's retained bytes so capi's plan check is exact*): the double-live
  peak changed with #1273; whichever lands second updates the other's numbers.
- Merge order in hot files (`crates/host-core/src/prepare.rs`, `crates/builtins-compiler/src/lib.rs`,
  `crates/effect-contract/src/live.rs`, `crates/engine/src/realtime/plan_exchange.rs`,
  `crates/source/src/lib.rs`) is root's, per decision 15's stream table.

## Answered by decision 15

The 2026-10-04 owner questions are answered; none blocks a slice.

- **Q1** (value edits inside a structural transaction): carry, then retarget for live values; a
  changed prepared value restarts behind a transition (D15-7, P1).
- **Q2** (latency growth during playback): a primed warm successor adopted with a raw-frame prime,
  with the transition as its one counted fallback and no permanent reserve (D15-8 (round-5 amendment),
  P7, #1287).
- **Q3** (fade in an added strip): yes, over the session mute ramp from its first played block
  (D15-9, #1288).
- **Q4** (browser rebuild cost on the audio thread): the control plane moves to a Worker on shared
  memory; the worklet only renders (D15-10, P12).
- **Q5** (a removed strip): two-phase removal, ramp then a scheduled swap (D15-9, #1325).
- **Q6** (`replaceSession` as a second edit API): one edit API, `engine.apply`; `replaceSession`
  only as a diffing convenience (D15-11, P12).

## Gates every slice inherits

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
- `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh` (new `unsafe`
  is refused outside its fixed file list; render code sits in `REALTIME_POLICY` regions)
- `bash scripts/check-cross-targets.sh`
- the slice's focused `cargo test --locked -p <crate>` runs, at both shipped bank widths
  (`Backend::Simd8` and `Backend::Simd4`) where banks are involved, through host-core's
  `test-support` entries `test_only_prepare_host_runtime_with_live_controls_on` and
  `test_only_prepare_host_runtime_with_live_controls_successor_on`
  (`crates/host-core/src/prepare.rs:948`, `:969`).

A slice that changes the shipped Wasm module reports `ARTIFACT CHANGED` and follows
`docs/RELEASE.md` for the pin. Every new or rewritten test states the defect only it catches
(AGENTS.md). A superseded test is deleted in the same PR.

## Phase 1 status

Every phase-1 slice passed review on `codex/seamless-swap` and merged in #1299. Each slice's spec
(in git history) carries its verdict table and its "Phase-1 follow-ups".

| Slice | Passing commit | Follow-ups commit |
|---|---|---|
| #1270 hand-over at the swap block | `22c9bd5a1` | `46bc191af` |
| #1271 move a source consumer | `5488fb2f5`, `1a911e31f` | `bbf4d8626` |
| #1272 successor whose unchanged sources play | `2a6f2c10c` | `bf2b788e8`, `1d40c488d` |
| #1273 C ABI structural transaction | `448baae85`, `1338b063c` | `41ed93566` |
| #1274 anchored seek | `0297efa8c`, `c14fde0ce` | `2330610e6` |
| #1275 added C ABI source at an exact sample | `55690373d` | `86ac2f359` |
| #1276 strip input section | `e6302d8a8` (attempt 2) | `63432c193`, `25954f771` |
| #1071 soft-clip own snapshots | `983ac85bd`, `d6217a79d` | (records only, `41ed93566`) |
| #1278 allocation-free effect restore | `251113c8f` (attempt 3) | `a3561163a` |
| #1289 browser rebuild measurement | `d169ef5f8` and its review follow-ups | (its own worker) |

The batch's final gates and its `ARTIFACT CHANGED` record (`29d5e78e...5a16` to
`9c8f4f68...7e76`, not re-pinned) are in this spec's git history.
`audit capi`'s `pcm_digest` moved `7281b6c931e05dcc` to `c60671f6593fa603` with #1276's carried
input filters (`807b48547`).

**Phase-1 items that remain, and where they went:**

- The delay refuses its own edge-ramp snapshots (feedback, mix, cross feedback); #1282 moves rather
  than restores it.
- Soft-clip's two open non-finite history cases: #1300. Soft-clip validating an in-flight current by
  its line rather than by `ramp_path_within` (#1278): not filed.
- The shared edge-ramp restore probe's cost per PR: #1301.
- The in-place C ABI amendment `miso_engine_v1_source_seek_at` and
  `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (feature bit 32, mask 63, ABI version `0x00010000`,
  #1275): D15-12's ABI-growth rule covers it (*Document the seek contract and the C ABI growth rule
  in the header*, #1317).
- Records admitted to a predecessor's input queue after the swap-block drain are never applied
  (#1276 attempt-1 NIT-4). The #1269 merge verdict found no defect on `main`: `commit_live` targets
  the newest candidate, and the C ABI attaches no input lane yet. The rule binds #1261 on the C ABI
  and #1332 on the browser.
- Smaller open review items, each recorded in its slice spec in git history: #1270 NIT-4, #1271 NIT
  2 and 3, #1272 NIT 2, #1273 NIT-3 and NIT-4, #1276 NIT-2 and NIT-3, #1274 attempt-1 NIT-2.

## Deferred (designed, not filed)

- **True crossfade (ghost strips)** (D15-9): reopens on a measured, audible dip in a listening test.
- **Whole-bank move:** when a bank's lane-to-strip map is identical in both plans, swap the bank
  instead of copying each lane. Only if #1286 shows lane copies dominate the swap block.
