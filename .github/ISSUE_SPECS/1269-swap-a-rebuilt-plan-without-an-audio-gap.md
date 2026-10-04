# Swap a rebuilt plan without an audio gap

Umbrella. Planner brief of 2026-10-04 (Sol), written against `main` at `24029badb`. The code
anchors were read on `54b0a1bf8`; `24029badb` adds only owner decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`) and one AGENTS.md sentence, so every code
anchor still holds. One adversarial verification (FAIL: three blockers, fifteen majors) is folded
in; see "Verification".

## Owner direction (2026-10-04)

> "I've never used a DAW where adding a track would cause an audio dropout."

A structural edit (add a track, add an effect, reroute) rebuilds the plan. A DAW keeps the playback
position and the state of every unchanged node across that rebuild. The engine must do the same.
Decision 14 keeps value changes on the live path even after swaps become seamless (its C2) and lists
"plan swaps are not seamless on either host" as a known gap. This umbrella closes that gap for
structural edits only.

## Problem (verified)

- **The C ABI drops the audio at every structural swap.**
  - Every structural transaction prepares a whole new runtime from nothing: the `Structural` arm of
    `SessionState::command` (`crates/capi/src/runtime/control.rs:718`) calls `prepare_runtime`
    (`:748`; `crates/capi/src/runtime/compile.rs:412`), which calls `prepare_host_runtime`.
  - The new plan gets new, empty source rings. The policy is frozen as
    `StructuralSourceStatePolicy::ResetAtReplacementBoundary` (`control.rs:46-55`). The test
    `structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic`
    (`crates/capi/src/runtime/tests.rs:600`) pins the swap block as all zeros (`:726-741`). The host
    must seek a new generation and refill every ring before audio resumes.
  - The new plan's DSP state starts at rest: filter integrators, compressor and gate envelopes,
    limiter look-ahead lines, delay lines, fader and pan ramps, PDC lines. The only thing the swap
    carries is the clock (`RealtimePlanOwner::enter_block`,
    `crates/engine/src/realtime/plan_exchange.rs:358-404`, `adopt_absolute_sample` at `:395`).
- **The browser has no structural edit path.** `AudioWorkletEngineHost` owns one `PreparedHost` for
  its life (`ReadyOwnership`, `hosts/host-web/src/lib.rs:1509`). A structural change boots a new
  engine.

## Outcome

On both hosts, a structural edit while audio plays has no gap:

- every unchanged node keeps its exact state, so its output continues bit for bit;
- every unchanged source keeps its ring, its generation and its read position;
- a new node starts at rest; a new source starts at an exact render sample the host chooses;
- no acknowledged edit is lost: the swap never drops a live record, a source chunk for a source the
  session still has, or a committed transaction;
- the swap block allocates nothing, frees nothing, takes no lock and makes no syscall.

The acceptance shape every slice uses: render the old plan, apply the structural edit, render on.
Compare every block with a reference: the post-edit session compiled fresh and fed the same PCM from
sample zero, with each added strip contributing exact zeros before the swap (muted, or fed zeros).
The streams must be bit-identical. The test signal has no exact-zero sample, so `+0.0`/`-0.0` sums
stay out of the comparison. The master reduction sums its inputs left to right in edge-ID order, so
a muted strip's `+0.0` term moves no bit.

## Design (planner decisions P1-P13, subject to owner review)

### P1. What carries: the "unchanged" rule

A state owner carries from the predecessor plan to the successor when all of these hold:

1. **Same stable key.** Sources by source ID. Graph nodes by `GraphNodeId`
   (`crates/graph/src/lib.rs:272`): strip stages by `(strip ID, TrackStage)`, effects by
   `(strip ID, rack, effect ID)`, compensation lines by `GraphEdgeId`, routes by route ID. Never by
   index, lane or unit position.
2. **Same envelope.** Rate, quantum and output shape. The exchange already refuses a different
   envelope (`PlanPublisher::reserve_replacement`, `plan_exchange.rs:259`).
3. **Same prepared layout.** Native effect identity, quality, link mode, prepared bypass, prepared
   latency, `state_sizes` and `state_layout_version` (`PreparedEffectMetadata`,
   `crates/effect-contract/src/lib.rs:1112`); a source's `PcmSourceRingConfig`. These facts come from
   the predecessor's **state inventory**, recorded when it was prepared.
4. **The transaction did not change its values.** Compare the owner's values in the **committed
   model before the transaction** with the model the successor is prepared from: every parameter
   value and the bypass of an effect; the strip's input section, effective fader (VCA offsets
   included) and pan or matrix; a source's declaration; a route's coefficients. Compare `f32` by
   bits.

Why the committed model and not the predecessor's prepared words: on the C ABI every acknowledged
live edit is committed to the model (#1257, #1261-#1266), so the committed model is what the user
hears or is about to hear. A live edit makes the predecessor's state differ from its own prepared
words, and comparing with those words would restart every owner the user ever touched (the
verification's blocker X1). The predecessor's state plus its pending live records (P5) converge to
the committed values. In the browser, which has no committed model yet (#1057), the predecessor's
effective model is its document overlaid with every admitted live record, and the successor is
prepared from a three-way merge: where the new document keeps the old document's value, the
effective (live) value wins (*Keep browser live strip state across a session replacement*, B3, and
its effect counterpart, B3b).

Placement does not matter. A lane may move to another bank, or between a bank and a per-node
instance; that changes no bit (AGENTS.md: banking "may couple lanes' cost, never their bits").

An owner that fails the rule starts at rest, exactly as a fresh plan would. In particular, a node
whose values changed **in the same structural transaction** starts at rest; carrying its old state
would also carry old targets. "Carry, then retarget" is deferred (Q1).

### P2. Where state moves

- **Off the render thread:** the successor is prepared from the committed model as today. A join of
  the predecessor's inventory, the two models (P1) and the successor's own inventory builds a
  **carry program**: a flat list of `(owner kind, successor location, predecessor location)` moves.
  It is installed in the successor before publication. All allocation, string comparison and
  validation happen here.
- **On the render thread, at the swap block:** `RealtimePlanOwner::enter_block` owns both plans
  between the swap and the retirement push (`plan_exchange.rs:392-396`). A new hook runs the carry
  program there, inside the render scope, before the successor renders. A synchronous host (the
  browser) calls the same hook directly.
- **Mechanisms, cheapest first:**
  - **Move:** swap owned storage between the plans (`core::mem::swap`). A few words, whatever the
    state size. Used for source consumers, per-node effect instances and delay lines of equal
    length.
  - **Lane copy:** a bank lane, or a lane that changes between a bank and a per-node instance, goes
    through the payload methods every native effect implements
    (`PreparedNativeEffectBank::snapshot_track_state_payload` and `restore_track_state_payload`,
    `effect-contract/src/lib.rs:2046-2056`; the per-node pair at `:1913` and `:1955`) and one scratch
    buffer the successor preallocates. Those methods were written as control-path calls ("no engine
    path snapshots a bank at all since #1037", `:2036-2042`), and the true-peak limiter's restore
    allocates (`LaneRestore`, `crates/true-peak-limiter/src/lib.rs:3932-3947`, built at
    `:4089-4114`), and the compressor and transient shaper re-derive a ramp's step on restore, so a
    lane copied mid-ramp is not bit-exact (`crates/compressor/src/state.rs:96-111`; the differential
    says so, `crates/conformance/src/randomized.rs:393-395`). *Make every banked effect's state
    restore allocation-free* makes them render-safe and exact mid-ramp first; the payload is not
    persisted, so its layout may change. Builtins copy a fixed-size plain-data lane state between two instances of the
    same concrete type; that needs `as_any_mut` on `GraphPreparedBuiltinBankProcessor`, which today
    exposes only `as_any` and `into_any` (`graph/src/lib.rs:1285-1286`).
- **Bounded work:** the program's length and copy bytes are known when it is built. The swap block's
  extra cost is measured on the 64-track console (*Record the swap block's cost on the 64-track
  console*).

R6b is not involved: "in-memory state the engine hands across a plan replacement is not persisted
state" (AGENTS.md). Nothing is persisted, versioned or migrated.

### P3. Sources keep playing

- A source that passes P1 keeps its ring. The successor is prepared with a **vacant** entry for it
  (no ring allocated), and the carry moves the predecessor's `PcmSourceConsumer` into that entry,
  with the driver's pending generation-change flag. The ring, its queued blocks, its generation and
  its read position all continue.
- The control side moves the matching producer into the successor's `SourceControlSet` right after
  the transaction's protocol commit has succeeded. That move is infallible. From then on, a submit
  or seek goes to the newest set.
- The double-live peak holds no second ring for a persisting source. A single plan's source cap
  counts the rings it allocated plus the rings it carries.
- `StructuralSourceStatePolicy` and the test that pins the silent swap block are deleted.

### P4. Added and removed sources and strips

- **Added source:** a new ring at generation 1. To align it with the playing stems, the host calls a
  new **anchored seek**: "source frame `F` enters the graph in the block that starts at absolute
  render sample `A`". If that block has passed when the seek is observed, the consumer starts at
  `F + (block start - A)`. While the seek is held, the consumer keeps the held generation's first
  block instead of discarding it (*Hold an anchored source seek until its render sample*).
- **Removed source:** a later submit is refused with `source.id.unknown`. PCM already accepted for it
  is discarded when its plan retires. The committed session no longer has that source, so no
  acknowledged edit is lost.
- **Added strip:** its nodes start at rest; a fade-in is offered (Q3).
- **Removed strip:** its contribution stops at the swap block, as when a DAW deletes a playing track.
  A fade-out needs a scheduled swap (Q5).

### P5. Live controls: no admitted record is lost

- **Builtins and live sends** apply a drained record to lane state directly. The carry drains the
  predecessor's lane (bounded, #1253) and then copies it. The record takes effect on the swap block's
  first sample, as it would have without the swap.
- **Effects** only stage a drained record as an automation span or a prepared target; `process`
  applies it (`EffectControlLane::stage`, `crates/effect-contract/src/live.rs:124`; banks:
  `LiveControlEffectBankStage::drain`, `crates/rack/src/lib.rs:1247-1276`, applied in
  `process_inner`, `:1286-1302`). Draining at the carry would stage records into a plan that never
  renders again (blocker X3). So the successor's lane **inherits** the predecessor lane's queue
  consumer, live bypass and live symmetry terms, and drains the inherited queue before its own in its
  first block (a prepared target is never retained across a block boundary, so none is pending). The inherited consumer is freed when the successor retires, never on render.
- A record admitted while the successor is pending goes to the successor's queue (#1257's rule).
- **Capacity.** In its first block an inheriting lane drains both queues into one window that holds
  one queue's capacity (`EffectControlLane::stage`, `live.rs:300-312`; the target FIFO,
  `:193-205`). So while a successor is pending, a host admits to a successor lane only the room left
  after the predecessor lane's unconsumed records, and refuses the rest with typed backpressure.
  The C ABI applies it in its effect admission (#1264, #1265, #1266), the browser in B3b.

### P6. No crossfade between the two plans' outputs

Rendering both plans for a crossfade was evaluated and rejected:

- **CPU:** the crossfade blocks cost two plans' work in one callback. A session at 60 % load would
  need 120 % in that block, which is itself a dropout.
- **Sources:** each ring has exactly one render consumer (`PcmSourceConsumer`,
  `crates/source/src/lib.rs:955`). Feeding both plans needs a source tee across two plans.
- **Need:** with P1-P5 every unchanged path is bit-continuous. What remains is local to the edited
  strip, and a per-strip fade (Q3) treats it at the cost of one ramp.

### P7. Latency and PDC across a swap

PDC delays each edge into a node by that node's latest incoming arrival minus the edge's own arrival
(`crates/graph-compiler/src/pdc.rs:60-89`), at **every** node with several inputs: the output, every
submix input, every effect with a sidechain. Compensation lines carry by `GraphEdgeId`.

- **No surviving node's arrival changes** (add a track, add a zero-latency effect, add a latent effect
  on a path that stays below every maximum it joins): every unchanged edge keeps its length, its line
  carries, and every unchanged path is bit-continuous. An edited path has a local transition of its
  own latency change.
- **A node's arrival would drop** (a latent effect or its track removed, also inside a submix): keep
  it. The successor is compiled with every surviving node's arrival floored at the predecessor's
  value (*Keep every node's latency from dropping during playback*). Nothing downstream moves in
  time. The floors persist for the session's life; a host that wants the lower latency compiles the
  session again while stopped.
- **A node's arrival would grow** by `Δ`: the history the longer lines need was never stored. Exact
  continuity needs the successor to render ahead ("pre-roll") `k = ceil(Δ / quantum)` extra blocks at
  the swap. Example: a true-peak limiter (latency `Fs/100 + 6` = 486 samples at 48 kHz) costs `k = 4`
  extra blocks in one callback at a 128-frame quantum. The alternatives are a gap of `Δ` samples on
  the affected paths (a 10 ms dropout for that limiter), or a fixed latency reserve in the session.
  This is owner question Q2.

VST3 states that a plug-in's latency change may interrupt playback, because the host must recompute
its delay compensation (`IAudioProcessor::getLatencySamples`,
https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html).
Fixed-latency large-format consoles avoid the case by giving every path the same latency. The floors
and the reserve are the engine's form of that design.

### P8. Retirement and deferral

Unchanged. The successor's reservation already holds a retirement credit (`reserve_replacement`,
`plan_exchange.rs:259-290`). A deferred swap leaves the candidate pending and the predecessor
rendering. The carry runs only in the block that applies the swap, so it copies the latest state. A
predecessor mutated by the carry is retired and reclaimed off the render thread.

### P9. Acknowledgements: can an ack ever precede a drop?

- **Structural transaction.** The C ABI writes its response and `SESSION_COMMITTED` after the
  candidate's publication slot and retirement credit are reserved and the protocol commit succeeded
  (`control.rs:795-856`). The render thread never refuses a reserved candidate, and the carry is
  infallible.
- **Source PCM.** Accepted PCM for a persisting source stays in the carried ring. Accepted PCM for a
  removed source is discarded with its plan (P4).
- **Live records.** P5. None is dropped.

### P10. Meters and observation

- C ABI: the render-peak telemetry reads the output (`SharedPlanState`,
  `crates/capi/src/runtime/plan.rs:5`); it is continuous.
- Browser: meter, observation and spectrum windows restart at the swap, and the meter generation
  advances. A window open at the swap is dropped and counted as a loss, never relabelled.

### P11. A wrong predecessor cannot happen, and is harmless if it does

Each bound graph plan gets a process-unique identity at bind, and a carry program names its
predecessor's identity. The carry is built against the newest committed plan, and that is always
the plan it displaces: the C ABI allows one pending candidate (`control.rs:792`), and the browser
swaps synchronously. If the identities ever differ, the hook moves nothing, the successor's
non-source owners start at rest, its vacant sources render `+0.0` (their producers have already
moved), and a counter records it. A forced-mismatch unit test pins that outcome.

### P12. The browser prepares on its audio thread

The browser engine is one Wasm instance created inside the `AudioWorkletProcessor`
(`hosts/host-web/web/miso-engine-v1-audio-worklet.js:286`), and the Web Audio specification runs
`AudioWorkletGlobalScope` on the rendering thread. The engine's memory is not shared:
`scripts/check-web-audioworklet.sh` refuses shared memory and atomics in the engine module
(`:372-382`; the SDK's PCM feed worklet already uses SharedArrayBuffer and Atomics in its own
scope). So a browser successor is prepared on the audio thread, between two `process()` calls. B1
measures what that costs; owner question Q4 chooses the remedy if it is too slow.

### P13. One implementation shape

The carry is portable core code (`engine`, `graph`, `rack`, `source`, `effect-contract`, the effect
crates, `builtins-compiler`, `host-core`). The C ABI and the browser adapter only call it. No
target-specific path, and no code for modes production never runs: the per-node builtin processors
are a test-only oracle (`crates/builtins-compiler/src/lib.rs:4012-4019`), so builtins carry only
between bank lanes.

## State owners and their slices

| Owner | Where | Mechanism | Slice |
|---|---|---|---|
| Source consumer and producer | `PcmSourceConsumer` (`source/src/lib.rs:955`), `SourceControlSet` (`host-core/src/source.rs:118`) | move | 2-4 |
| Strip input section | `BuiltinBankProcessor` (`builtins-compiler/src/lib.rs:418`) | drain, lane copy | 7 |
| Fader/mute and pan/matrix ramps | `FaderBankProcessor`, `MatrixBankProcessor`, `FaderMatrixBankProcessor` (`:622`, `:685`, `:738`) | drain, lane copy | 8 |
| Console effect lanes | `EffectBankStage` (`rack/src/lib.rs:743`) | lane copy (payload) | 9-10 |
| Live-controlled effect lanes | `LiveControlEffectBankStage` (`rack/src/lib.rs:930`) | lane copy, inherited queue | 11 |
| Insert lanes that change representation | bank lane ↔ `NodeKind::Effect` / `LiveControlEffect` | lane copy (payload) | 11b |
| Per-node effects | `NodeKind::Effect`, `NodeKind::LiveControlEffect` with `BypassShunt` (`graph/src/runtime.rs:1135-1143`, `:1184`) | move, inherited queue | 12 |
| Compensation lines | `CompensationDelay` (`graph/src/runtime.rs:940`) | move or head-aligned copy | 13 |
| Strip and submix delay lines, live send ramps | `TrackDelayLine` (`:1021`), `LiveRoute` (`:861`) | move, drain, copy | 14 |
| Meters, observation taps, spectrum | builtins meters, `ObservationLane` | restart (P10) | B2 |

## Slices

Each slice is its own issue with its spec in `.github/ISSUE_SPECS/`. Dependencies name exact titles.

| # | Issue | Title | Depends on |
|---|---|---|---|
| 1 | #1270 | Hand the outgoing plan to its successor at the swap block | none |
| 2 | #1271 | Move a source consumer into a successor graph plan | 1 |
| 3 | #1272 | Prepare a successor plan whose unchanged sources keep playing | 2 |
| 4 | #1273 | Keep sources playing across a C ABI structural transaction | 3 |
| 5 | #1274 | Hold an anchored source seek until its render sample | 3 |
| 6 | #1275 | Start a newly added C ABI source at an exact render sample | 4, 5 |
| 7 | #1276 | Carry the strip input section across a plan swap | 3 |
| 8 | #1277 | Carry fader, mute and pan ramps across a plan swap | 7, #1253 |
| 9 | #1278 | Make every banked effect's state restore allocation-free | #1071 |
| 10 | #1279 | Carry console effect lanes across a plan swap | 8, 9 |
| 11 | #1280 | Carry live-controlled effect lanes across a plan swap | 10 |
| 11b | #1281 | Carry an insert lane that moves between a bank and a per-node instance | 11 |
| 12 | #1282 | Carry per-node effect instances across a plan swap | 11b |
| 13 | #1283 | Carry compensation lines across a plan swap | 12 |
| 14 | #1284 | Carry strip delay lines and live send ramps across a plan swap | 13 |
| 15 | #1285 | Keep every node's latency from dropping during playback | 14 |
| 16 | #1286 | Record the swap block's cost on the 64-track console | 14 |
| 17 | #1287 | Pre-roll a successor whose latency grows | 15, 16, owner Q2 |
| 18 | #1288 | Fade in a strip that a swap adds during playback | 6, 8, owner Q3 |
| B1 | #1289 | Measure a session rebuild on the browser's audio thread | none |
| B2 | #1290 | Replace the running browser session in the Rust host | 1, 3, 8, B1 (and owner Q4 if B1 exceeds the budget) |
| B3 | #1291 | Keep browser live strip state across a session replacement | B2 |
| B3b | #1292 | Keep browser live effect edits across a session replacement | B3, 11 |
| B4 | #1293 | Export session replacement from the browser engine module | B3b, 5 |
| B5 | #1294 | Send a replacement session to the AudioWorklet | B4 |
| B6 | #1295 | Qualify a browser session replacement in real browsers | B5 |
| B7 | #1296 | Replace the session from the browser SDK | B6, owner Q6 |
| B8 | #1297 | Feed a source that a browser replacement adds | B7 |

**First product slice.** Slice 4: on the C ABI, adding a track to a playing session no longer drops
the audio, for any session whose unchanged paths hold no DSP state. Slices 7-14 extend that to every
state family, one family per slice; slice 9 is an independent prerequisite.

**Order.** AGENTS.md allows one launch-critical implementation at a time. Run the slices in the
table's order. Slice 9 (effect crates only), B1 (measurement only) and slice 16 (tooling) are
independent bounded issues and may run beside the current feature slice. The umbrella closes when
slices 1-16 (11b included) and B1-B6 (B3b included) have closed, and slices 17, 18, B7 and B8 have
closed or been withdrawn after the owner's rulings on Q2, Q3 and Q6.

## Coordination

- **#1053** (*Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes*) and its slices, open and planned in parallel:
  - *Bound the builtin fader and matrix drains to the records present at block entry* (#1253): the
    bounded drain that slices 7, 8 and 14 call at the carry.
  - *Prepare C ABI plans with live track fader and matrix lanes* (#1256) and *Prepare C ABI plans
    with live effect lanes* (#1263): they change `prepare_runtime` (`compile.rs:412`) and
    `ProviderEpoch` (`control.rs:9`), as slice 4 does.
  - *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257): its
    live branch sits in the same `Structural` arm (`control.rs:718-857`) that slice 4 changes, and its
    pending-candidate rule is the second half of P5.
  - *Commit model-only C ABI transactions without a plan rebuild* (#1260): it rewrites the
    `SetSessionId` rebuild trigger in `tests.rs:627` (the test slice 4 rewrites) and the `command()`
    helper in `crates/capi/tests/resource_lifecycle.rs:192-209` (which slice 4's gates use). Its new
    trigger, a `SetSourceContent` with changed content, is not carried under P1, so slice 4's race
    gate uses an `UpsertTrack` trigger instead.
  - #1261-#1266 commit more values live, which is why P1 compares committed models. *Apply
    value-only effect parameter edits to the running C ABI plan* (#1264), *Apply value-only
    parametric EQ parameter edits to the running C ABI plan through prepared targets* (#1265) and
    *Apply value-only effect bypass edits to the running C ABI plan* (#1266) admit to effect lanes:
    they must apply P5's capacity rule while a successor is pending.

  Whichever of two overlapping slices lands second rebases onto the other and keeps both behaviours.
  #1053's amendment A0 (the swap-window race) is on `main` (#1042, `95ddc0eb0`;
  `synchronize_plan_epochs`, `control.rs:634-676`).
- **Decision 14** (#1259). Its C1 says callers see one edit API and the engine decides. The browser
  has none yet (its F6), so a public browser `replaceSession` is a second edit API: owner question
  Q6, and B7 waits for it.
- **Decision 14 F4.** A session bypass on the delay or the multiband compressor is prepared; a
  committed change of it makes the effect "not carried" (P1.3). Slice 12 asserts that and leaves F4
  to its owner question.
- **#1057** (the browser's session model). B2-B6 build the replacement mechanism a later browser
  session-transaction path calls. They do not decide #1057's design.
- **#1054 and #1055** (ramp lengths). Slice 18's fade length comes from them, or from #1053's ramp
  until they land.
- **#1071** (*Soft-clip refuses its own subnormal snapshot on restore*). Slice 9 depends on it.
- **#888, #889** (partial insert cohorts bank) and **#1107** (silent-lane skips in console banks)
  change bank shapes and bank-level state; whichever lands after slices 10-11b must keep their gates.
- **#1074** (*Walk the engine plan's retained bytes so capi's plan check is exact*). Slice 4 changes
  the double-live peak; whichever lands second updates the other's numbers.

## Owner questions

- **Q1. Value edits inside a structural transaction.** P1 starts a node at rest when the same
  transaction changed its values. Acceptable for now, with "carry, then retarget" later?
  Recommended: yes. It is rare and safe.
- **Q2. A node's latency that grows during playback.** (a) Pre-roll `ceil(Δ / quantum)` extra blocks
  in the swap callback: exact, a one-time CPU spike (4 extra blocks for a limiter at 48 kHz / 128).
  (b) A fixed latency reserve declared in the session: exact, no spike, latency always paid. (c)
  Accept a gap of `Δ` samples on the affected paths. Recommended: (a), decided on slice 16's measured
  spike; (b) costs little once slice 15 exists.
- **Q3.** Should a strip that a swap adds fade in, and over what length? Recommended: yes, with the
  live mute ramp length (#1053, then #1054).
- **Q4.** If B1 shows that a browser rebuild blocks the audio thread longer than the quantum budget
  for real sessions: time-sliced preparation across quanta, or Wasm threads with shared memory for
  the engine module (this lifts the `check-web-audioworklet.sh` ban and needs a nightly `build-std`)?
  No recommendation until B1 has numbers.
- **Q5.** A removed strip stops at the swap block (P4). Acceptable, as in a DAW? A fade-out needs a
  scheduled swap and live mute. Recommended: accept for now.
- **Q6.** Decision 14 C1 wants one edit API. Should the browser SDK expose `replaceSession`, and the
  worklet its `miso.replace.v1` message (B5), publicly now, or keep them internal until #1057 gives
  the browser session transactions? Recommended: expose it
  as the browser's structural edit API now, documented as the path #1057 will wrap.

## Gates every slice inherits

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
- `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh` (new `unsafe`
  is refused outside its fixed file list; render code sits in `REALTIME_POLICY` regions)
- `bash scripts/check-cross-targets.sh`
- the slice's focused `cargo test --locked -p <crate>` runs, at both shipped bank widths
  (`Backend::Simd8` and `Backend::Simd4`) where banks are involved. The width seam is
  `#[cfg(test)]` inside `crates/host-core/src` (`prepare.rs:657-672`, `lib.rs:113-116`), so width
  tests either live in a `#[cfg(test)]` module under `crates/host-core/src` or use the
  `test-support` successor entry with a backend that slice 3 adds.

A slice that changes the shipped Wasm module reports `ARTIFACT CHANGED` and follows `docs/RELEASE.md`
for the pin. Every new or rewritten test states the defect only it catches (AGENTS.md). A superseded
test is deleted in the same PR.

## Verification

One opus-xhigh adversarial verifier reviewed the plan twice. The first draft returned FAIL with
three blockers and fifteen majors; all are folded in:

- X1 (P1 compared prepared words, so every live-touched owner restarted): P1.4 now compares
  committed models.
- X2 (payload restore allocates): slice 9 makes restore render-safe, and slice 10's realtime gate
  carries every bankable effect.
- X3 (draining an effect lane at the carry loses staged records): P5's inherited queue.
- The majors: PDC at every summing node (P7, slices 13 and 15); anchored-seek discard (slice 5);
  producer move after the protocol commit (slice 4); test-allocator and width seams (slices 1 and 3);
  synchronous clock adoption (slice 1); test-only per-node builtins dropped (P13); rack and `Any`
  access (slice 7); mono collapse kept after a swap (slices 7 and 10); prepared versus live bypass
  (slice 12); #1053's split issues (Coordination); browser staging, ABI-layout mirrors, generation,
  solo, memory growth and harnesses (B2-B8); and multi-day slices split (this table).

The re-verification returned PASS-WITH-FIXES, no blocker, eleven majors, all folded in: the
inherited queue's window capacity (P5, slice 11); exact mid-ramp payloads (slice 9, a quantum-32
gate in slice 11); slice 12's bypass case; authorized paths (slices 3 and 4); the browser's three-way
merge, solo composition, feed re-cut, commands during a pending replace, oracle files and the MSB1
version 2 control block (B3-B8); and the splits 11b and B3b.

## Deferred (designed, not filed)

- **Carry, then retarget** (Q1).
- **Fade-out of a removed strip** (Q5): needs a scheduled swap.
- **Spread pre-roll:** spread slice 17's catch-up over several callbacks with a ring peek, if its
  one-callback spike is too high.
- **Whole-bank move:** when a bank's lane-to-strip map is identical in both plans, swap the bank
  instead of copying each lane. Only if slice 16 shows lane copies dominate the swap block.

## Phase 1 status

Every phase-1 slice passed review on `codex/seamless-swap`; the batch follow-ups that close their
remaining review findings follow `fe37db10f` (the #1289 merge). Each slice's spec carries its
verdict table and a "Phase-1 follow-ups" section.

| Slice | Passing commit | Follow-ups commit |
|---|---|---|
| #1270 hand-over at the swap block | `22c9bd5a1` | `46bc191af` |
| #1271 move a source consumer | `5488fb2f5`, `1a911e31f` | `bbf4d8626` |
| #1272 successor whose unchanged sources play | `2a6f2c10c` | `bf2b788e8`, `1d40c488d` |
| #1273 C ABI structural transaction | `448baae85`, `1338b063c` (minors closed in #1275) | `41ed93566` (records, line widths) |
| #1274 anchored seek | `0297efa8c`, follow-ups `c14fde0ce` | `2330610e6` |
| #1275 added C ABI source at an exact sample | `55690373d` | `86ac2f359` |
| #1276 strip input section | `e6302d8a8` (attempt 2) | `63432c193`, `25954f771` |
| #1071 soft-clip own snapshots | `983ac85bd`, follow-ups `d6217a79d` | (records only, `41ed93566`) |
| #1278 allocation-free effect restore | `251113c8f` (attempt 3) | `a3561163a` |
| #1289 browser rebuild measurement | `d169ef5f8` and its review follow-ups | (handled by its own worker) |

Batch-follow-ups gates, on the final tree: fmt, workspace clippy `-D warnings` and rustdoc
`-D warnings` clean; workspace policy check and self-test ok; realtime policy ok (87 marked
regions in 25 files, the new floors) and its self-test ok; `check-capi-abi.sh` and `--self-test`
ok; `audit capi` 100,000 calls with 0 allocations, deallocations, locks and syscalls,
`pcm_digest` `c60671f6593fa603`; `check-cross-targets.sh` PASS with the ten #1018 iOS
`memset_pattern16` expected-failure rows; `cargo test` for engine (also `realtime-audit`),
source, graph, host-core, capi, host-web, builtins-compiler, rack, parametric-eq and compressor
with the test-support features, 1009 passed and 0 failed. Worklet chain (`--named-twin` build,
`check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`) green;
`bridgeRetainedBytes` 1167211 of 1181888. **ARTIFACT CHANGED**: the shipped module moved from
`29d5e78e...5a16` (2886763 B, at `fe37db10f`) to `9c8f4f68...7e76` (2886837 B): the browser
now charges the plan state inventory, and the engine envelope guard, the graph carry outcome and
`prepare_seek`'s re-check compile into it. Not re-pinned (`docs/RELEASE.md`, "Between releases").

**Successor items not done in phase 1:**

- The delay refuses its own edge-ramp snapshots (feedback, mix, cross feedback); slice 12 moves
  rather than restores it.
- Soft-clip's two open non-finite history cases (filed as #1300), and soft-clip validating an
  in-flight current by its line rather than by `ramp_path_within` (#1278; not filed).
- The shared edge-ramp restore probe costs about 70 s per PR in debug (#1278 delay follow-up
  MINOR-3; filed as #1301).
- Owner review of the in-place C ABI amendment: `miso_engine_v1_source_seek_at` and
  `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (feature bit 32, mask 63) added under ABI version
  `0x00010000` (#1275).
- Records admitted to a predecessor's input queue after the swap-block drain are never applied;
  when a host gains that path (P5, #1257), admission must route to the newest plan (#1276
  attempt-1 NIT-4). **Verified after the #1053 merge, no issue filed.** The #1269 merge verdict
  (section 3, "The newest-plan interaction") found no defect: the C ABI's `commit_live` targets
  `pending_providers.last_mut()`, the newest candidate; the structural arm prepares from the
  newest inventory and the committed model; a second structural transaction while one is pending
  is refused with backpressure, so a candidate holding acknowledged records is never displaced
  unrendered; and at the swap block the carry runs before the successor drains its own queues.
  The C ABI attaches no input lane today, so no live record shares state the carry overwrites. The
  rule stays a constraint on the first host path that adds one: #1261 on the C ABI and B3 (#1291)
  on the browser must route input records to the newest plan, as `commit_live` does.
- #1289's numbers and its reading against the budget are posted on this issue (the umbrella comment
  #1289's Deliverable 3 asked for); B2 waits for the owner's ruling on Q4.
- `audit capi`'s `pcm_digest` moved `7281b6c931e05dcc` to `c60671f6593fa603` with #1276's carried
  input filters (`807b48547`); the PR that lands this batch must say so (#1273 already moved it
  from `ff6cdcb96cdcdad5`).
- Smaller open review items, each recorded in its slice spec: #1270 NIT-4 (a missing predecessor
  executor reports `NotRequested`), #1271 NIT 2 and 3, #1272 NIT 2 (install an empty program so a
  wrong predecessor is counted even when nothing carries), #1273 NIT-3 (`ProtocolCommit` in the
  fault matrix) and NIT-4 (double-live oracle trigger, #1260), #1276 NIT-2 and NIT-3 (about 10 KB
  of sort instantiations), and #1274 attempt-1 NIT-2 (harness duplication).
