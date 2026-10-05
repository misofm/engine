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
- **The C ABI's control plane is adapter code.** `crates/capi/src/runtime/control.rs` (1,584
  lines; `commit` at `:353`; `commit_live` at `:1065` calls `host_core::classify_live_delta` at
  `:1071`) and `crates/capi/src/runtime/compile.rs` (994 lines; plan exchange with publication and
  retirement capacity 1 at `:170-171`). A structural edit while a candidate is pending is refused
  with BACKPRESSURE (`control.rs:959-961`).
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
