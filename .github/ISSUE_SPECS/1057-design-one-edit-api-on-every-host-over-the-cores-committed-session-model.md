# Design: one edit API on every host over the core's committed session model

Stream K of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-11; also D15-3, D15-10, D15-17).
Code anchors verified on `main` at `6fb211594`.

This issue is now the **design note for one edit API** (D15-11). Decision 15 answered its former
design and live-edit questions. What remains is to write the design down, measure its costs, and
keep the personal-mix owner question open. No product code change.

## Product outcome

One document, `docs/handoffs/one-edit-api/1057-design-note.md`, that every implementer of the
control-plane crate, the browser Worker and the SDK reads instead of re-deriving the design. A
reader can answer from it alone: which call edits a session on each host, what the call returns,
how completion is observed, how a save works, and what each choice costs.

## Owner rulings this note serves

`docs/rulings/engine-footprint-2026-09-28.md`:

- The portable core owns the current, edited session on every platform, browser included. Every
  edit goes through the engine, which keeps the session model and a revision. Any app saves by
  asking for a canonical JSON snapshot and storing it itself (device, browser storage or server).
- A fan's edits are saved as a **personal mix**; the producer's original is untouched.

## Context (today)

- **C ABI.** The protocol controller owns a `SessionStore` (`crates/protocol/src/model.rs:872`) and
  a revision. `SessionTransactionApply` (`crates/protocol/src/controller.rs:892`) and
  `SessionSnapshotGet` (`:888`) are its edit and save commands. The header documents live and
  structural transactions (`crates/capi/include/miso_engine_v1.h:35`, `:85`).
- **The C ABI's control plane is adapter code.** `crates/control-plane/src/control.rs` (1,707
  lines; `commit` at `:437`; `commit_live` at `:1201` calls `host_core::classify_live_delta` at
  `:1207`) and `crates/control-plane/src/compile.rs` (837 lines; plan exchange with retirement capacity 1
  at `:198`). A structural edit while a candidate is pending is refused
  with BACKPRESSURE (`control.rs:1095-1097`).
- **Browser.** The TypeScript SDK owns the document. `BrowserEngine`
  (`sdk/src/browser/engine.ts:192`) has no edit or snapshot method. The live controls hold the
  booted document byte for byte (`assertBootedSession`, `sdk/src/core/live-controls.ts:276`). A
  canonical writer exists (`writeCanonicalSessionDocument`, `sdk/src/internal/session-json.ts:9`)
  but is not exported. `hosts/host-web` keeps compiled plans and live lanes but no session model.
  Its `[dependencies]` (`hosts/host-web/Cargo.toml`) include `host-core` and `session`, not
  `protocol`.

## Decisions already made (decision 15; the note records them and does not reopen them)

- **D1. One control plane.** capi's control plane (`control.rs`, `compile.rs`) moves into a
  portable crate that both adapters call: session store, `classify_live_delta`, preparation, plan
  publication and supersession (*Extract the C ABI control plane into a portable crate both hosts
  call*, #1309). capi and host-web stay thin adapters. No browser-only session logic in the core.
- **D2. The C ABI surface keeps its shape.** It keeps `SESSION_TRANSACTION_APPLY` and
  `SessionSnapshotGet`. The response gains the edit path (D15-3, *Report each transaction's edit
  path in its response*, #1313). The applied-revision watermark is a query (D15-17, *Publish an
  applied-revision watermark and complete edits asynchronously*, #1314).
- **D3. Browser SDK surface.** `engine.apply(transaction)` returns `{revision, path}`, where path
  is exactly one of `live`, `model_only` or `rebuild`.
  The watermark `(revision, first sample in effect, outcome flags)` is a status field. Submit is
  synchronous only for what can fail; completion is observed, never awaited (D15-17).
- **D4. `replaceSession(document)`** exists only as a convenience. It diffs the document against
  the committed model into one transaction and calls the same apply path. It is not a second API.
- **D5. The worklet message protocol stays internal.** No public replace message.
- **D6. Where the browser control plane runs.** In a Worker instance on one shared
  `WebAssembly.Memory`; the AudioWorklet only renders and swaps (D15-10, *Run the browser control
  plane in a Worker and keep the AudioWorklet render-only*, #1332). A non-isolated page keeps the
  same API; its structural edit is a blocking rebuild, reported and counted.
- **D7. B3 and B3b close as not planned.** *Keep browser live strip state across a session
  replacement* (#1291) and *Keep browser live effect edits across a session replacement* (#1292)
  existed only because the browser had no committed model. The committed model replaces their
  three-way merge. #1290 and #1293-#1297 are rewritten on `engine.apply` by stream H.
- Former question 1 (design) and question 3 (live edits) of this issue: superseded by decision 15,
  D15-2, D15-3, D15-11 and D15-17.

## Deliverables

The note at `docs/handoffs/one-edit-api/1057-design-note.md`, with these sections:

1. **The edit path on each host.** A sequence for a live, a model-only and a structural edit, on
   the C ABI and on the browser (main thread, Worker, worklet), naming the crate and function that
   owns each step. Answer the acked-batch question for every hop: *can an ack ever precede a drop?*
2. **The portable crate's boundary.** What the adapters pass in and get back (transaction,
   response, watermark, snapshot pages), stated so #1309 and #1332 implement the same thing. Name
   what stays in capi and what stays in host-web.
3. **The SDK surface.** TypeScript signatures for `engine.apply`, the watermark field,
   `replaceSession` and `engine.snapshot()`. Decide what the SDK live-control handles become: thin
   builders of transactions submitted through `engine.apply`, or a separate low-latency path that
   commits to the same model. One edit API is the default; a second path needs a measured reason
   (the live-edit hop of section 5).
4. **Saves.** The snapshot API on both adapters, its cost on the 64-track documents, and the
   argument that it does no render-thread work (it reads the committed model on the control thread
   or in the Worker).
5. **Costs, measured.** Each with its exact command, commit and host:
   - the shipped AudioWorklet artifact size with the control plane (and `protocol`) in the closure,
     from a scratch build that is not committed;
   - the crates that enter host-web's closure (`cargo tree -p host-web -e normal`);
   - the control-thread cost of a structural edit: cite `artifacts/steps/web-rebuild-base/report.md`
     (the #1289 boot times); re-run `bash scripts/run-web-mixing-automation-benchmark.sh` only if
     the base moved;
   - the live-edit hop of a Worker design (main thread to cells), p50 and max, from a scratch
     Chromium probe.
6. **The staged plan.** Map each step to its existing issue (#1309, #1312, #1313, #1314, #1331,
   #1332, and the rewritten #1290 and #1293-#1297). Name any gap the note finds as a proposed issue
   for the coordinator to create.
7. **The one open owner question** (below), with options and costs.
8. **Host-local monitoring state.** A ruling on solo, observe subscriptions and the meter lease.
   None of them has a transaction form: the session model (`crates/session`) and the protocol
   (`crates/protocol`) have no solo field or command. Solo lives in host-core's
   `LiveControlSoloState` (`crates/host-core/src/solo.rs:129`), reached through the SDK's track
   live edits (`sdk/src/core/live-controls.ts:728`). Meters and observations are leases on
   `BrowserEngine` (`subscribeMeters`, `sdk/src/browser/engine.ts:197`; `subscribeObservations`,
   `:205`). *Admit browser live edits in the Worker through the committed model* (#1382) assumes
   a host-local overlay in the Worker's control half.
   - **Position to evaluate (root's):** solo, observe subscriptions and the meter lease are
     host-local monitoring state, not session content. They stay outside the committed model,
     travel through the same control plane (the portable crate and the Worker), and never appear
     in a transaction's path or revision. A snapshot never contains them.
   - **The note rules on it** by answering, with anchors: is any of the three saved, shared with a
     personal mix or replayed after a rebuild (if so, it is session content and needs a
     transaction form); how the overlay is carried across a plan swap and a `replaceSession`; how
     its submits are refused or acknowledged (the acked-batch question); and whether the C ABI
     treats the same three the same way. It confirms the position or names the item that breaks
     it, with the proposed issue.

## The one open owner question: personal mixes

This stays open and **blocks nothing** in this issue or in decision 15's streams. The note
presents, with costs:

- how a personal mix is represented: a full session copy, or the producer's session plus the fan's
  changes (a transaction log or an overlay);
- what happens to a fan's personal mix when the producer publishes a new version of the session;
- how it interacts with stored automation (*Research: render stored session automation in the
  engine, identically on every platform*, #1058, question 2).

The coordinator asks the owner in the next owner round. The note must not assume an answer.

## Authorized paths

- `docs/handoffs/one-edit-api/1057-design-note.md` (new)
- This spec's evidence record.

## Non-goals

- Any product code, SDK code or committed benchmark artifact.
- Reopening D1-D7. A finding that contradicts one is written as a finding for the coordinator, not
  as a changed design.
- The extraction itself (#1309), the Worker (#1332), the cells (#1312), the path field (#1313) and
  the watermark (#1314).

## Objective gates

1. The note exists at the path above and has the eight sections listed in Deliverables.
2. Every `path:line` in it is checked on the commit it names.
3. Every measured number names its command, commit and host. The scratch builds and probes are not
   committed (`git status --short` shows only the note).
4. Section 1 states the acked-batch answer for every hop.
5. `bash scripts/check-workspace-policy.sh` passes.

## Test value

No test is added. The note is design evidence; the implementing issues' gates check its claims.

## Dependencies

- None to start. It reads the specs of *Extract the C ABI control plane into a portable crate both
  hosts call* (#1309) and *Run the browser control plane in a Worker and keep the AudioWorklet
  render-only* (#1332), and should land before #1332 starts implementation.

## Attempt record

**Attempt 1** (stream K research worker). The note is
`docs/handoffs/one-edit-api/1057-design-note.md`; every anchor in it is checked on `8be19c86e`.
Host: `Linux devbox 6.8.0-139-generic x86_64`, AMD EPYC 7313P, loaded (load average 18-48), so
every number is uncontrolled and descriptive. Scratch builds and probes stay in
`/tmp/claude-1002/w1057/` and are not committed.

- Module size: scratch tree from `git archive HEAD`; the cargo line of
  `scripts/build-web-audioworklet.sh:113-114` with `CARGO_TARGET_DIR=/tmp/claude-1002/w1057/target`,
  then `scripts/strip-wasm-names.py strip`. Baseline 2,894,202 bytes (digest equal to the official
  script's); with `capi` and `protocol` linked and reachable, 3,222,313 bytes (+328,111, +11.3 %);
  gzip -9 926,483 → 1,039,871.
- `cargo tree -p host-web -e normal`: 66 distinct crates; the proxy adds exactly `capi` and
  `protocol`.
- Structural edit (#1289 base moved): `run-web-mixing-automation-benchmark.sh prepare`,
  `rebuild-preflight`, `rebuild-run --step w1057-rebuild-head` with
  `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`; round 2 p50 23.851 ms (64-track console), 39.366 ms
  (sends), as at the base.
- Live-edit hop: Chromium 151 (Playwright 1.62.1), COOP/COEP, a Worker running the C ABI control
  plane in Wasm, 500 edits per round, one warmup and two rounds: hop to cell p50 2.715-7.460 ms,
  max 5.560-14.975 ms on the 64-track documents; the post is 0.020-0.045 ms (p50).
- Gates: `bash scripts/check-workspace-policy.sh` ok; `bash scripts/check-dsp-research.sh` ok (it
  checks the DSP research corpus, which this issue does not change).

**Attempt 2** (answers the attempt-1 verdict: 3 MAJOR, 8 MINOR, 5 NIT). Anchors still on
`8be19c86e`, the new ones included. No new build or timing; every number comes from the attempt-1
evidence in `/tmp/claude-1002/w1057/evidence/`.

- MAJOR-1: new section 8.4 and F12. Only render writes an observation tap's arm state, in both
  modes: the `Observe` record carries an arm tag from the control plane; `ObservationLane::arm`
  writes `armed`, `arm_sample` and `arm_tag` into the tap's observation slot in the drain; host-web's
  `observation_armed` and `observation_arm_samples` are deleted. The SDK's `appliedAtSample` on
  subscriptions becomes `armed(): Promise<bigint>`, resolved from the reported tag. New proposed
  issue P7; amendments of #1381 D3 and #1382.
- MAJOR-2: section 5.4 decomposes the apply: three whole canonical writes per value-only edit
  (one in the compile, two in the classifier, `live_delta.rs:228`, `:257-263`), about 97 % of the
  phase run; #1305 excludes the comparison (its non-goal). P2 becomes P2a (edit-bounded typed
  classification, canonical comparison kept only as test oracle), P2b (canonical text at the first
  snapshot of a revision, exact incremental length, resumable writer) and P2c (replay entry
  staged in place). The single-mode budget gate moves to #1382's single-mode leg. Section 4.3
  decides the single-mode snapshot: written in entity slices across handler calls and service
  ticks.
- MAJOR-3: section 3.4 never merges two calls that write the same address, so every acked value
  is committed and every supersession is a later commit counted by `live_values_superseded`; the
  W1 row of section 1.3 states it; P6 gains the gate.
- MINORs: section 5.5 lists every command (scratch tree, proxy edit, module builds, twiggy and
  grouping, native example builds and runs, Chromium probe); the uncited second `probe_phases`
  run is removed, not re-run. Option C states the field-level diff and setters it needs, and the
  snapshot claim names P3. Section 3.3 defines the `monitor` encoding; 3.4 states the cuts (edit
  count, frame bytes, repeated address, `apply`/`replaceSession`/`snapshot` barrier). F2/P4: the
  browser adapter passes only `SESSION_TRANSACTION_APPLY`, so no `transport_state` or
  `automation_canceled` event exists there. F11 and section 3.1: single-mode calls stay pending
  while the context is suspended. Section 5.4 limit (6): service tick and source drain not
  measured. Section 3.4 gives the browser's `ControlLimits` rule and values; section 2.1 states how
  replace enters the controller (typed entry, no frame, no replay entry; amends #1386).
- NITs: commit count and `prepare.rs` +597/-39; "a track" (a submix is solo-safe); the telemetry
  lease mention removed; worst-case latency includes resends; "approximate size", stable
  non-atomic build, not the nightly atomics artifact.
- Gate: `bash scripts/check-workspace-policy.sh` ok.

**Attempt 3** (answers the attempt-2 verdict: 2 MAJOR, 6 MINOR, 5 NIT). Work on `50fb23b5f`; no
file under `crates/`, `hosts/`, `sdk/`, `tools/` or `scripts/` changed since `8be19c86e`, so every
anchor holds on both. New measurement (note section 5.6): a scratch test of
`ProtocolCodec::encoded_session_transaction_len` and of record layouts, in a `git archive` export
under `/tmp/claude-1002/w1057b/` (tree and target deleted after; output kept in
`/tmp/claude-1002/w1057b/evidence/sizes.txt`, not committed).

- MAJOR-1: the browser `ControlLimits` are derived now, from the encoder. Largest live edit:
  `UpsertEffectParam`, 176 bytes at 3-byte IDs and 416 bytes at the 127-byte ID; 1,024 of them are
  426,032 bytes. Rule: 1,024 × the largest live edit at the 127-byte ID, rounded up to a power of
  two. Values: frame 524,288, response 524,288, one replay entry of 1,048,576. P1 gains the gate
  "every setter encodes to at most 416 bytes at the 127-byte ID", so no later issue rechecks the
  values. The pre-P2c staging cost at these limits is measured (43-54 µs p50 natively per edit).
- MAJOR-2: P2a has no exact-0 allocation gate, by the realtime rules: AGENTS.md allows counted
  control allocations in the single-mode message handler and keeps the render-locked count at 0
  (D15-10); the model clone in `prepare_transaction` allocates anyway, so a classifier-only 0 would
  not make the handler allocation-free. P2a's gate is now "allocation count and bytes are equal
  with and without 63 unnamed tracks" plus the time gate; the launch registry is built once. The
  audio-thread protection is #1382's single-mode time gate.
- MINORs: the single-mode snapshot writes at most 65,536 bytes per handler call with a writer
  resumable inside strings, continued by the SDK's message, not by a timer; bound 2S + 2 quanta,
  gated in P3 with `currentFrame`. The single-mode budget is measured in a cross-origin-isolated
  Chromium Worker on the same module and exports (5 µs clock). Arm tags are `u64` (no wrap
  refusal), carried as first and count in `CommandReply`, #1293's outcome record and #1294's
  reply; P7 changes the read record (96 to 104 bytes) and the SDK layout; `armed()` settlement is
  decided; #1280 D3 is cited. P2b keys on a value-setter predicate the compile can decide; P2c
  plans the eviction read-only and applies it at commit, and stages the response in one boot-time
  buffer. The SDK queue holds 64 calls and refuses with `backpressure`. New proposed issue P8 owns
  the typed commit path in `protocol`.
- NITs: per-call time is labelled an inference with its basis; the input-rate reliance is removed;
  cut 1 defines lane-set overlap; a merged call reports `rebuild`; P7 lists the resource rows.
- Correction found: route gain, mute and matrix already have setters, so P1 no longer lists them;
  `SetVcaFader` replaces `UpsertVca` in P1's list.
- Gate: `bash scripts/check-workspace-policy.sh` ok.

**Follow-ups after PASS** (the attempt-3 verdict: PASS, 6 MINOR, 4 NIT; all folded in one
commit). Anchors read on `a1a3fe87c`, which has the same code as `8be19c86e`. The three #1057
verdicts are copied to `docs/handoffs/decision-15-2026-10-05/verdicts/stream-k/`.

- MINOR-1: section 4.3 states the single-mode snapshot bound in system audio callbacks: at most
  S + 2 callbacks, (S + 2) × ceil(B / 128) quanta (8 callbacks, 32 quanta for the sends document
  at 480-frame callbacks). P3's gate is (S + 2) × Q quanta, with Q measured in the same run by a
  ping round trip; the evidence records Q and the observed B.
- MINOR-2: P2b's length update counts the `revision` field, rewritten by every commit
  (`model.rs:943`) and written as a decimal string (`visit.rs:220`); gate (1) crosses its digit
  boundaries.
- MINOR-3: P2b gate (4) uses a covering set of budgets: every budget on small documents; on large
  documents 1 to 64, powers of two and ±1, and budgets at each kind of stop point; about 150
  writes per large document.
- MINOR-4: the typed commit takes the `miso.replace.v1` message's own `requestId` (non-zero, SDK
  numbers from 1) as the `SESSION_COMMITTED` event's `origin_request_id`; P8's gate compares the
  whole event at the same request ID. Amendments of #1294 D1 and #1386 updated.
- MINOR-5: `CommandReply.removed_arm_tag_count` with `SessionState::removed_arm_tags()`; the
  #1293 outcome record's reserved `u32` becomes `removed_arm_tag_count` (still 40 bytes) with a new
  export `miso_engine_web_v1_removed_arm_tags_ptr`; `miso.edit.v1` gains `removedArmTags`. Solo
  entries are not listed (no pending promise). P5 gains a gate; amendment rows #1293, #1294 and
  #1382 updated.
- MINOR-6: option C's cost in the owner question names the setters that exist (effect bypass,
  route destination, route gain, mute and matrix, pan and matrix, effect parameters) and what P1
  adds (no route setter). The question stays open.
- NIT-1: P2c names the response buffers at `controller.rs:1916` and `:1934`, counts them in
  `RESPONSE_STAGING_VECS`, and gates them.
- NIT-2: the effect-parameter `smoothingSamples` option is removed, not renamed (section 3.1, P6).
- NIT-3: the 64-call bound applies to new calls only; resent calls of a refused merged message
  can raise `pendingCalls` above 64.
- NIT-4: `currentFrame` is cited as Web Audio 1.1 §1.32.3 (new source [S8]).
- Gate: `bash scripts/check-workspace-policy.sh` ok.
