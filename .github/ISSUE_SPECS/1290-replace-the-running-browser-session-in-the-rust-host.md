# Replace the running browser session in the Rust host

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Slice B2 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

Rewritten for decision 15. The old body replaced the whole document with
`replace_session(document)` on the audio thread. Its live values carried only through B3/B3b's
three-way merge. Decision 15 supersedes that design:
- D15-11: one edit API, the transaction;
- D15-10: preparation in the Worker;
- D15-17: asynchronous completion.

*Keep browser live strip state across a session replacement* (#1291) and *Keep browser live
effect edits across a session replacement* (#1292) close as not planned. The committed model
replaces their merge.

## Product outcome

The browser engine's Rust host applies a structural transaction while it plays. The Worker-side
control half validates the transaction, classifies it as `rebuild`, and prepares the successor
from the committed model. It commits and returns `{revision, path: "rebuild"}` at once, without
waiting for render. The render half (the worklet) adopts the successor at a block boundary, with
the C ABI's guarantees:
- unchanged sources keep their rings;
- unchanged nodes keep their state;
- the clock continues;
- a refused transaction leaves the running engine and the model untouched.

The watermark reports when the revision is in effect. On a page that is not isolated, the same
call runs the blocking rebuild on the audio thread. It is reported and counted. This slice is the
Rust host, tested natively. The Wasm export, worklet wiring and SDK are the rewritten slices
#1293-#1297.

## Context

- `AudioWorkletEngineHost` (`hosts/host-web/src/lib.rs:1990`) boots once
  (`boot_with_spectrum_config`, `:2043`). Boot runs:
  - the document size and parse-projection budgets;
  - `compile_host_model`;
  - the rate and quantum shape check;
  - the VCA bound;
  - then `compile_ready` (`:6617`). It prepares through one of three branches: a single
    spectrum capture, a spectrum collection, or no spectrum through
    `prepare_host_runtime_with_selected_meters_between_render_calls` (`:6644-6681`).

  The host owns one `ReadyOwnership` (`:1509`) and a `host_generation` (`:2004`). Prepared
  companions are bound to that generation and address strips by index.
- `render_next` (`:3212`) renders `ready.host.plan` directly, with `RenderTime { absolute_sample:
  status.next_absolute_sample }` (`:3234-3239`). It has no plan exchange.
- **The C ABI's shape, which this slice reuses.**
  - The control thread moves the persisting source producers into the candidate at commit
    (`crates/capi/src/runtime/control.rs:1010-1019`).
  - It publishes through `PlanPublisher::reserve_replacement`
    (`crates/engine/src/realtime/plan_exchange.rs:265`).
  - Render adopts in `RealtimePlanOwner::enter_block` (`:375`). That step continues the clock,
    runs `carry_from` (`crates/engine/src/realtime/plan.rs:917`) and sends the old plan to the
    retirement queue. The control side reclaims it with `PlanRetirer::try_reclaim` (`:512`).
- **Successor preparation.**
  - `SuccessorBase { inventory, committed }` (`crates/host-core/src/prepare.rs:641`).
  - `prepare_host_runtime_with_live_controls_successor` (`:926`).
  - `SourceControlSet::adopt_persisting` (`crates/host-core/src/source.rs:286`).
  - No successor wrapper exists yet for the two spectrum branches.
- **Cost.** A 64-track boot is 23.9 ms (V8 p50) against a 2.667 ms quantum (round-1 C4;
  *Measure a session rebuild on the browser's audio thread*, #1289, closed). In Worker mode this
  cost leaves the audio thread. In `single` mode it is the blocking rebuild that D15-10 accepts,
  reported and counted.

## Decisions frozen for this slice

- **D1. Entry point.** The control half's `apply(transaction) -> Result<{revision, path},
  refusal>` is `control_plane::SessionState`'s transaction apply (`crates/control-plane`,
  #1309). It runs in the Worker on the `SessionState` that #1381 constructs. It prepares through
  the adapter preparation hook that #1381 adds.
  - A `rebuild` delta prepares through host-core successor wrappers for each of `compile_ready`'s
    three branches. This slice adds the two spectrum wrappers beside
    `prepare_host_runtime_with_live_controls_successor`.
  - The wrappers use the concurrent preparation variants, never `_between_render_calls`. The
    producers live in the Worker, and render runs at the same time.
  - The current spectrum configuration is kept. A transaction that removes the strip a spectrum
    capture observes is refused as `web.apply.spectrum_target`.
  - A change of rate or quantum is refused with `RESULT_REPREPARE_REQUIRED`.
  - Any refusal returns before commit and changes nothing.
- **D2. Base.** The successor is prepared from the committed model, which includes every live edit
  already committed (D15-7, P1.4). `SuccessorBase.committed` is the committed model of the
  predecessor plan plus every record pushed to it. The old D4 rule ("an owner that received a
  live record is not carried") is superseded by D15-7.
- **D3. Budget.** The projection charges the running plan, every unadopted candidate and the new
  plan together. Each carried ring counts once. The parse transient is included. The total is
  checked against `maximum_memory_bytes`.
- **D4. Swap.**
  - At commit, the control half:
    1. moves the persisting producers into the candidate (`adopt_persisting`);
    2. publishes the candidate through the reserved replacement;
    3. advances `host_generation`, so a companion bound to the old strip indices is refused.
  - The render half adopts the candidate at block entry through `RealtimePlanOwner`, and the
    Worker's service loop reclaims the retired plan. Both are wired by *Swap and retire browser
    plans through the Worker's service loop* (#1381). The carry is stream A's;
    this slice adds nothing to it.
  - A second structural transaction while a candidate is unadopted supersedes it by
    compare-and-swap (#1310). The displaced revision completes as `superseded`.
- **D5. Completion.** `apply` never waits for render.
  - The revision is pending until #1314's watermark covers it. The browser reads it through
    *Publish the applied-revision watermark in the browser status* (#1349). The watermark's
    first-sample-in-effect is the block at which `enter_block` returned `Applied` for that
    candidate.
  - Live edits submitted while a candidate is pending are written to the newest candidate's lanes
    (D15-17).
- **D6. Single mode.** With no Worker, the same `apply` runs on the worklet thread between two
  render calls, and the same render half adopts at the next block.
  - Path stays `rebuild`.
  - Each such apply increments the host status's saturating `blockingRebuilds` counter, which
    #1332 D1 defines.
  - The service step (#1348) runs after the render call, never inside it.
- **D7. Observation.** Meter, observation and spectrum state of unchanged owners carry across the
  swap as *Carry meter, observation and spectrum state across a plan swap* (#1327) specifies
  (D15-14). The meter lease carries.

## Deliverables

1. D1-D6 in `hosts/host-web/src/lib.rs`: the control half's `apply` with the browser's
   successor preparation, publication, and the `blockingRebuilds` increment.
2. The two spectrum successor wrappers in `crates/host-core/src/prepare.rs`.
3. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`, `hosts/host-web/Cargo.toml`
  (dev-dependency `bench-support` only)
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`: the two spectrum successor
  wrappers only. Root orders this after stream A's open edits to `prepare.rs`.

## Non-goals

- No Wasm export, no worklet or SDK change. The rewritten #1293-#1297 cover those.
- No document convenience. Diffing a document into one transaction, `apply_document` and the
  `replace` export are *Diff a replacement document against the committed model and export
  replace from the browser engine module* (#1386), which calls this slice's `apply`.
- No SDK surface. `replaceSession` and `engine.apply` are *Apply session transactions from the
  browser SDK* (#1296), over the exports of *Export transaction apply and anchored seek from the
  browser engine module* (#1293) and #1386.
- No transitions of its own. Fade-in, duck-swap and two-phase removal come from #1288, #1324 and
  #1325.
- No warm-successor catch-up: *Run the browser catch-up in the Worker's service loop* (#1361).

## Objective gates

1. **Gap-free acceptance.** Native host-web test. The control half and the render half run on two
   threads, with live controls on.
   - Boot A: nine tracks with enabled high-pass and low-pass filters and non-centre pans.
   - Render 6 blocks, feeding sources through `submit_source`.
   - Apply a transaction that adds a muted track whose ID sorts first.
   - Render 6 more blocks.
   - The response is `{revision: 1, path: "rebuild"}` and returns before the render thread
     adopts.
   - From the adoption block on, every block equals a fresh boot of the committed model fed the
     same PCM from frame 0.
   - The watermark reaches revision 1 with outcome `exact` and the adoption block's first sample.
   - A response capture after the swap is stamped with the continued clock.
2. **Live edits are part of the base.** Apply a live fader edit to track 3, then a structural
   transaction. After adoption, track 3 renders at the edited fader value. Its ramp state carries
   (#1277). The output equals a fresh boot of the committed model.
3. **Refusals change nothing.** These are each refused with their typed result:
   - a malformed transaction;
   - a rate change;
   - a transaction over the memory budget;
   - the removal of a spectrum-observed strip.

   Revision and model are unchanged, no candidate is published, and the next blocks are
   bit-identical to a run without the call.
4. **Supersession.** Apply two structural transactions before the render thread enters a block.
   Only the second is adopted. Revision 1 completes as `superseded`, revision 2 as `exact`. No
   BACKPRESSURE is returned.
5. **Each preparation branch.** Gate 1 passes with a single spectrum capture and with a spectrum
   collection configured.
6. **Companions.** A prepared companion bound before the apply is refused after it.
7. **Realtime.** Across gates 1, 2 and 4, the render thread counts `allocations == 0 && frees == 0`
   around every render call (`bench_support::alloc` thread-scoped counters, after warm-up). Every
   retired plan is dropped on the control thread.
8. **Single mode.** Gate 1 driven on one thread (apply between render calls) gives the same blocks
   and the same response. `blockingRebuilds` is exactly 1.
9. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-web-audioworklet.sh`
   - `bash scripts/test-web-audioworklet.sh`
   - the browser legs: `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`
   - The shipped artifact changes: report its digest (`artifact-identity`).

## Test value

- Gate 1: turns red if the apply reboots internally (fresh rings or state), if the successor's
  clock restarts at 0, or if `apply` blocks until adoption.
- Gate 2: turns red if the successor is prepared from the boot document instead of the committed
  model, which would lose live edits. That is the defect B3/B3b's merge existed to patch.
- Gate 3: turns red if a refusal runs after the producers moved or after publication.
- Gate 4: turns red if a pending candidate still causes BACKPRESSURE, or if a displaced revision
  never completes.
- Gate 6: turns red if a companion addressed by an old strip index reaches the new plan.
- Gate 7: turns red if the render half drops a retired plan, or allocates on adoption.
- Gate 8: turns red if single mode takes a different code path that diverges in bits, or is not
  counted.
- Superseded tests: none. The old body's tests were never written.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Admit browser live edits in the Worker through the committed model* (#1382).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Publish the applied-revision watermark in the browser status* (#1349).
- *Give every browser plan live strip fader and mute lanes* (#1326): the transitions on the
  browser need a live strip mute.
- *Carry fader, mute and pan ramps across a plan swap* (#1277), for gates 1 and 2.
- *Carry meter, observation and spectrum state across a plan swap* (#1327), for D7.
