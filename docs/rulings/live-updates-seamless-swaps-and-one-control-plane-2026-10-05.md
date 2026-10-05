# Live updates, seamless swaps and one control plane: owner-delegated decisions of 2026-10-05

Follows decision 14 (`live-update-versus-rebuild-2026-10-04.md`). Recorded on 2026-10-05 against
`main` at `6fb211594`. Each point names whose authority it carries, on decision 13's model
(`submix-strips-sends-and-vca-2026-10-02.md`, "Authority").

## Decision 15: every live value ramps, every swap is seamless, one control plane on both hosts

**Context.** Decision 14 named two ways to change a running mix (live update and plan rebuild) and
left open questions on four fronts: how live values behave on a paused host (#1053 Q2), whether a
host learns which path an edit took (#1053 Q3), the tail a live input filter reports (#1053 Q4),
and how a rebuilt plan replaces the running one without a gap (#1269 Q1-Q6). #1269 phase 1 (PR #1299)
shipped the carry program, source continuity across a structural swap and the first carried
owners. Decision 14's findings F1-F9 asked for further changes.
This decision answers all of them at once.

**The owner's words.** On 2026-10-05 the owner wrote, verbatim:

> "Please make the decisions based on maximizing long-term reliability and correctness. We
> shouldn't take any shortcuts that need to be fixed or worked around in the future. We are
> building this out with agents so the usual pre-AI benefit of taking shortcuts now is not worth
> it. We can just spend resources now to do things correctly from the beginning. Please keep that
> in context, spin up an adversarial opus 5.5 high agent and talk with it to come up with a plan
> that encompasses all of these items. Then coordinate and implement."

The first four sentences are the **no-shortcuts principle**. It is binding on every point below
and on every issue filed from it: no interim placeholder that a later issue must undo; defer only
when a dependency forces the order, and then sequence the correct solution.

**Authority.**

- **Owner decision:** the no-shortcuts principle, and the instruction to plan with an adversary,
  then coordinate and implement.
- **Owner-delegated decisions (root and adversary):** D15-0 to D15-17 below. They are root's
  decisions, made under the owner's explicit delegation above and converged with a fresh
  adversarial Opus 5.5 agent over two rounds. They are **not** the owner's own words and are
  subject to owner review. Where one of them reverses an earlier recommendation, the earlier text
  is superseded (see "Earlier open questions, answered").
- **Evidence.** The root draft, the adversary's two rounds and the agreed plan are copied into
  `docs/handoffs/decision-15-2026-10-05/` (`PLAN-2026-10-05-root-draft.md`,
  `PLAN-2026-10-05-adversary-round1.md`, `PLAN-2026-10-05-adversary-round2.md`,
  `PLAN-2026-10-05-agreed.md`). The round files carry the file:line anchors and measurements each
  rationale below cites. The streams, issue numbers and merge order are in
  `docs/handoffs/decision-15-2026-10-05/STREAMS.md`.
- **No code changes.** This record and the `AGENTS.md` amendment change no behaviour. Each decision
  is delivered by the issues it names; `main` keeps its behaviour until they land.

### The decisions

**D15-0. Principle and delivery.**
- The no-shortcuts principle governs every decision below.
- Parallel coordinators may run in separate worktrees, each owning a declared file set, with merges
  into `main` sequenced by root. This replaces `AGENTS.md`'s one-launch-critical WIP rule; the
  amendment lands with this record.
- Every batch merge runs the full gate set once on the merged tree.
- *Rationale:* the plan has eleven streams with disjoint file sets; run serially it would take weeks.
  Every past batch's final full-gate run caught a blocker, so the merged-tree run stays mandatory.
- *Issue:* *Record decision 15: live updates, seamless swaps and one control plane* (#1308).

**D15-1. Every live value ramps.**
- #1055 researches the default ramp lengths; #1054 then applies them.
- A record with no ramp length uses the session's researched `controlSmoothing` default. An
  explicit 0 stays legal.
- #1054 covers every live row: fader, mute, pan or matrix (a model `smoothing_samples` of 0 means
  "session default"), input trim and polarity, sends (gain, mute, matrix), VCA offset and mute.
- SDK defaults stop being 0 (decision 14 F7, SDK half).
- *Recorded resolution:* a live edit carries an optional per-edit ramp length end to end, through
  the one edit API (absent means the session default, an explicit 0 stays legal). #1054 owns the
  field, #1364 resolves an absent length, #1382 carries it from browser records. The bypass
  crossfade alone always uses the session mute ramp (#1341). Default values come from #1055's
  cited and measured research; #1388 runs the blinded listening session, and a listening result
  changes only the default values.
- *Rationale:* a step is a click (#1053 A2 D3). The caller must not decide smoothness by omission.
- *Issues:* #1055, #1054, #1394, #1364, #1365, #1388 (stream E).

**D15-2. Live value lanes become latest-target cells, on both hosts in one slice.**
- Gated conditions:
  1. levels only;
  2. a value is superseded only by a later commit before the same drain;
  3. every fallible check runs before the first cell write;
  4. an exact `live_values_superseded` counter.
- Automation, Observe records, structural edits and every time-stamped record stay FIFO.
- Design: each cell holds its target words and ramp as one unit; a per-lane dirty mask; a canonical
  drain order, fader before mute; a multi-word cell is published so that render reads a whole value
  without spinning and never skips the latest committed value (the agreed plan's single sequence
  word could skip a torn read; #1312 fixes the mechanism).
- The ack bytes are unchanged. Their meaning is: render converges to the committed value no later
  than the first block whose render begins after the submit returns, or, while a successor is
  pending, at its adoption, which the watermark reports (D15-17).
- `AGENTS.md`'s "never silently lost" gains one sentence that defines supersession.
- *Rationale:* live records carry no sample time and apply at block entry, so FIFO and cells are
  observationally identical, except that FIFO is worse for "step then ramp". The engine already
  coalesces in three places. The committed model is the authority (#1053 D9). Cells remove capacity
  BACKPRESSURE for live values on a paused host. Acked-batch question: a superseded value is never
  dropped silently; the committed model holds it, render converges to it, and the counter is exact.
- *Issues:* *Hold live values in latest-target cells on both hosts* (#1312, strip fader, mute and
  matrix lanes; its browser status counter is #1399) and its sibling slices #1345 (effect parameter, bypass and EQ-target), #1346 (strip
  input lane) and #1347 (route lanes), all stream B.

**D15-3. Edit outcome reporting.**
- The response to every transaction carries its path. D15-17 fixes the set: `live`, `model_only` or
  `rebuild`.
- The application sample comes from a monotone watermark published by render, completed by D15-17
  as `(revision, first sample fully in effect, outcome flags)`. It is a C ABI query and a browser
  status field. It never uses the reliable event lane (capacity 2).
- This reverses #1053 Q3's "not now".
- *Rationale:* the control thread cannot know the block at which render pops a record or adopts a
  plan. Revisions are monotone, so a watermark is lossless for the host's question and cannot
  back-pressure.
- *Recorded resolution:* the agreed plan's D15-3 also named a fourth path, `rebuild_with_transition`,
  for a counted fallback. D15-17 later moved every fallback to the watermark's outcome flags,
  because a fallback is decided after submit returns. D15-17 governs: there is no fourth path value.
- *Recorded resolution:* the path is an in-place V1 amendment of the transaction response bytes. No
  C symbol or struct changes, so it takes no C ABI feature bit (D15-12's growth rule covers the C
  ABI surface; the protocol carries its own schema identity).
- *Issues:* #1313, #1314 (stream B); the browser status field #1349.

**D15-4. Tail and rest.**
- (a) **Fix the SVF limit cycles:** a joint flush in `svf_step` (`REST_EPS = 1e-14`), keeping the
  per-word 1e-20 flush. This is class B (bits move); root accepts it under the owner's delegation
  (`effect-floor-accounting.md` flags class B for owner ruling; the delegation is that ruling). Two
  regression tests, both red on revert: the input LPF period-2 cycle and the EQ shelf fixed point.
  The digest of the single cross-target corpus owner is re-baselined, with its reason.
- (b) **An engine-wide tail contract:** the smallest `T` with `|y[n]| < P·10^(−144/20)` for every
  input of peak `P` and no control event after it (a ramp in flight is allowed). Each effect and
  builtin computes it from its designer, per rate, on the control thread, with a test that
  recomputes it (never a digest). Every node also states an exact-rest bound. Gain-only processors
  report 0 beyond their latency.
- (c) #1261 and #1262 then report the bounded tail, never `Infinite`.
- *Rationale:* exact zero was provably never reached at the top of the cutoff domain, and the EQ
  had a second trap (round 1, B4.3; round 2, "Bits moved"). With the fix, exact rest is reached
  within a stated bound (input section ≤1.0M samples at +24 dBFS), so silence skipping can rely on
  it.
- *Recorded resolution:* with f32 rounding and ramps in flight the smallest `T` is not computable
  exactly, so each node reports a certified upper bound on it, checked against a brute-force
  recompute.
- *Issues:* #1328, #1329 and its per-node slices #1372-#1379 (stream G); #1261, #1262 (stream F).

**D15-5. Size the live effect span window once (#1306).**
- The size comes from the producers the plan really has: the live lane depth plus the
  stored-automation spans per block that preparation computes, after #1058's design.
- The caller's `S` bounds only `AUTOMATION_ENQUEUE` density.
- *Issue:* #1306 (stream F), blocked on #1058's design.

**D15-6. C ABI live completeness.**
- #1225, #1226 (with F3: `follows_mute` live), #1247 (F9: VCA membership live on the C ABI), #1261,
  #1262 and #1267, all built as cells (D15-2).
- `follows_mute` is also made live in the browser.
- *Issues:* stream F; *Make a send's follows_mute live in the browser* (#1342, stream H).

**D15-7. Carry across a swap.**
- Carry, then retarget, for live values: carry the state, then emit the classifier's ramped records.
- A changed prepared (rule-3) value restarts its owner behind a D15-9 transition.
- P1.4 compares against the committed model of the predecessor plan plus every record pushed to it,
  not against "the model before this transaction".
- *Rationale:* restarting at rest is a click; comparing against the pre-transaction model restarts
  owners whose live state already matches (round 1, C1).
- *Issues:* the carry slices of stream A (#1277-#1284).

**D15-8. Latency.**
- **Decrease:** floors (#1285). Floors and the accumulated read-ahead reset at a discontinuity the
  host declares (a stop, or a seek of every source) (#1323).
- **Growth: a warm successor (off-thread catch-up).**
  1. The successor is prepared with floors at the predecessor plus `P`.
  2. At block B, render copies state into it (copy-mode carry: bounded memcpy and allocation-free
     payload calls) and returns it through a capacity-1 queue.
  3. The control thread (C ABI: in bounded `miso_engine_v1_service` slices, D15-17; browser: the
     Worker) renders it forward with the FP environment pinned. It reads sources through a
     read-only peek cursor, whose position gates the ring's release.
  4. Once it leads render by `P`, it publishes "adopt exactly at S, else return". At S, render
     adopts by pointer swap and moves the consumers with their read index at `S + P`.
  5. Live edits during the window are held and apply at S. A structural edit during the window
     supersedes the catch-up.
  6. `ΣP` is bounded by ring headroom (`P_max`).
- **Fallbacks, each counted and reported:** render-thread pre-roll bounded by `k_max`, then the
  transition: the D15-9 duck-swap of the strips whose arrival grows. They apply on a deadline
  counted in render samples, or on a non-isolated browser page. (The agreed plan also listed "a host
  that renders nothing"; D15-17 removes it, see below.)
- **No permanent latency reserve.**
- *Recorded resolution:* "a host that renders nothing" is not a fallback trigger. D15-17 counts the
  deadline in render samples, so a paused host's edit stays pending and never falls back; D15-17
  governs. A catch-up abandoned at a host-declared stop completes its revision as `exact`, because a
  declared stop owes no continuity.
- *Rationale:* "history fill" is impossible, because the needed samples are future processed
  samples. A reserve is a permanent cost. Render-thread pre-roll is a 5x-9x spike in one callback.
  The catch-up is exact for every path, the edited one included, and render pays one bounded copy
  (round 1, C2; round 2).
- *Issues:* #1285, #1323, #1322, #1362 (stream A); #1287 and its slices #1320, #1321, #1353-#1361,
  #1396, #1397 (stream C); #1311 (stream B).

**D15-9. Transitions for strips whose state cannot continue.**
- **Added strip:** fades in from its first played block, over the session mute ramp (#1288).
- **Edited strip** (a changed prepared value; an insert added, removed or reordered; quality; link
  mode; `delay_samples`): a **duck-swap**. The live mute ramps down on the predecessor, the swap is
  scheduled at the ramp's end, and the successor fades in (#1324).
- **Removed strip:** two-phase: a ramp, then a scheduled swap. Its source retires with phase 2, not
  with the commit (#1325).
- **Mechanisms:**
  - a scheduled swap ("adopt no earlier than S") and exact-sample adoption (#1311);
  - **CAS candidate supersession:** the control thread replaces an unadopted candidate. If render
    has already taken it, the newer candidate's carry program is re-targeted to the adopted plan. A
    held `seek_at` moves with the producer (#1310).
    *Recorded resolution:* the control thread first withdraws the unadopted candidate by
    compare-and-swap (#1343), then prepares the newer one against the plan render is running, and
    moves the withdrawn candidate's host-fed sources, acked PCM and held seeks into it (#1344), so no
    acked submission is dropped (#1398 sizes the capacities and admission). Three plans coexist during that preparation. The C ABI has no
    default caps (every limit is the caller's), so the header states the sizing rule and the
    repository's reference limits admit three plans; a host that configures smaller caps sees the
    typed resource refusal of whichever cap the peak exceeds, never BACKPRESSURE;
  - **live strip fader and mute lanes on every plan, on both hosts.** Every C ABI plan already has
    them (`crates/capi/src/runtime/compile.rs:572-600`); #1326 gives them to every browser plan.
- *Recorded qualifier:* "link mode" triggers a duck-swap only where the link mode stays prepared.
  Once #1371 lands, a link-mode change on the four linked console effects is live (#1236).
- A planned transition is not a fallback: its revision completes as `exact` (or `superseded`) and
  sets no fallback flag.
- A route that a swap adds to or removes from a surviving strip ramps in or out (#1363). A re-pointed
  route is a removal plus an addition, ramped at route level; it never ducks its source strip.
  Sends whose tap precedes a transitioning strip's fader ramp with that strip's transition, so every
  such route gets a live lane on every plan (#1391). This narrows decision 13's O9 (routes into the
  output stay folded) for those routes only; every other route keeps the fold. An added strip's
  pending fade-in survives a later swap (#1392).
- A true crossfade (ghost strips) is deferred. It reopens on a measured, audible dip in a listening
  test.
- *Rationale:* no click and no double render. Supersession removes the structural BACKPRESSURE that
  a pending candidate causes today (`crates/capi/src/runtime/control.rs:959`), permanently so while
  paused.

**D15-10. Browser: one control plane, off the audio thread.**
- Wasm threads with one shared `WebAssembly.Memory`. A Worker instance runs the control plane
  (model, classifier, preparation, catch-up, retirement, disposal). The AudioWorklet instance only
  renders and swaps, and **never allocates or frees after boot**.
- Cross-origin isolation is required for structural edits; the first-party app already sends
  COOP/COEP.
- A non-isolated page keeps the same API and the same artifact (a local shared memory, single
  instance). Its structural edit runs the blocking rebuild, reported and counted.
- *Recorded resolution (single mode):* with no Worker, the one instance runs the same control plane
  in the worklet's message handler, outside `process()`. Its control allocations are counted and
  reported; the render-locked allocation count stays exactly 0 in both modes. This is the one
  exception to "compile plans only on control/worker threads", and `AGENTS.md` records it.
- **Toolchain:** a pinned dated nightly with `-Zbuild-std` on `wasm32-unknown-unknown`, for the
  browser artifact only; everything else stays on the stable pin. A nightly bump re-records the
  three-browser matrix and the AArch64/native parity gates. The stable `wasm32-wasip1-threads`
  route was rejected: it needs wasi-libc internals driven by hand (the internal `__wasi_init_tp`,
  and a stray write into another instance's stack), render reaches a futex wait that Chromium traps
  in a worklet, and the target does not cover browsers.
- **The allocation rule** is enforced by three gates:
  1. a runtime `GlobalAlloc` counter on a const, instance-local render-locked flag, asserted at
     exactly 0 in browser qualification, with a mutation self-test;
  2. static checks for thread_local destructor registration, `memory.atomic.wait` and a pinned set
     of render `call_indirect` sites (today's direct-call gate cannot see the executor behind
     `Box<dyn PreparedPlanExecutor>`);
  3. the native counters, unchanged.

  `LIVE_HOST` and `BOOT_STAGING` (`hosts/host-web/src/ffi.rs:503-512`) become const and
  destructor-free.
- *Rationale:* this is the C ABI's shape, so there is one implementation. Time-slicing on the audio
  thread and prepared-artifact transfer were measured and fail (round 1, C4).
- *Issues:* #1331, #1380, #1332, #1381, #1382, #1387, #1333, #1334 (stream H). #1331 tests iOS Safari
  before #1332 starts.

**D15-11. One edit API.**
- capi's control plane (`crates/capi/src/runtime/control.rs` and `compile.rs`) is extracted into a
  portable crate that both adapters call (#1309).
- The C ABI keeps `SESSION_TRANSACTION_APPLY`. The browser SDK gains
  `engine.apply(transaction) -> {revision, path}` plus the watermark. `replaceSession(document)`
  exists only as a convenience that diffs the document against the committed model into one
  transaction.
- The worklet message protocol stays internal.
- #1057 becomes the design note for this. Its personal-mix owner question stays open and blocks
  nothing here.
- #1291 and #1292 close as not planned: the committed model replaces their three-way merge.
- *Issues:* #1309, #1400 (streams B, H); #1290, #1401, #1293-#1297, #1383, #1385, #1386, #1389 (stream H);
  #1057 (stream K).
- *Rationale:* decision 14 C1 (one edit API) and decision 2 of `engine-footprint-2026-09-28.md`
  (the core engine owns the edited session). A merge in the browser would be throwaway work.

**D15-12. Seek contract.**
- `seek_at` anchors on the plan's source-read clock ("equal to the render clock in this version").
- Every seek during playback is anchored: a plain seek means "the next block render begins", and
  the block it applied at is reported through the watermark.
- The C header states held behaviour, replacement, the in-flight BACKPRESSURE string, displacement,
  far-past anchors, the clock origin and the source rate = render rate assumption.
- The ABI-growth rule is written down: a feature bit per addition, `FEATURE_MASK` is never compared
  with `==`, and the bit does not protect directly linked hosts.
- "Held" is reported apart from "underrun".
- Tests and tooling: commit the verifier's probe P5 (a held seek across a swap); add the
  supersession case; set a `source.id.invalid` diagnostic; fix the #1293 D4 trap; add a debug
  assertion for off-grid lateness; make the untimed reads test-only; add one `seek_at` call to
  `audit capi`.
- *Recorded resolution:* seeks are not session revisions, so a plain seek's landing is reported
  through a per-source seek report published by render on the watermark's primitive (#1316), not
  through the revision watermark itself.
- *Issues:* #1316-#1319 and #1350 (stream B).

**D15-13. Decision-14 findings.**
- **E1 (F1):** a crate that sees descriptors refuses automation on non-`Block` parameters, and so
  does the SDK. It lands before #1058's first rendering slice (#1335, stream I).
- **E2 (F2):** make live the gate-expander's attack, hold and release (#1336); the parametric EQ
  band's `enabled` and `kind` (other `m`/`k` words on the same SVF state, via prepared targets,
  #1337); the multiband crossover, with a test that the band sum stays continuous (#1366, #1338).
  Under the same finding, link mode becomes live: a strip's console link mode (#1236 and its slices
  #1368-#1371) and the multiband compressor's link mode (#1367).
- **E3 (F3):** `follows_mute` live (D15-6).
- **E4 (F4), and rule 3 extended:** a **correctness reason may keep a value prepared**, recorded
  with its reopening condition. This adds a third reason to decision 14's rule 3 (glitch,
  optimisation, correctness). Every command that would be acked with no effect is refused with a
  type, `AUTOMATION_ENQUEUE` included until #1058 wires it (#1315). The delay and the multiband
  compressor get a live bypass shunt (#1339; #1340 after #1069). Until those land, lifting their
  bypass is a rebuild with a D15-9 transition.
- **E5 (F7):** the bypass switch crossfades over the session ramp, and is bit-identical after the
  ramp (#1341; the browser command #1393; stream E).
- **F9:** VCA membership is live on the C ABI (D15-6, #1247).

**D15-14. Meter, observation and spectrum state carry** across swaps for unchanged owners, under
P1, on both hosts (#1327 meters and observation taps, #1395 spectrum; stream A).

**D15-15. Preparation performance**, both hosts: validate static effect descriptors once per type
(about 9% of preparation, #1330); no `GraphNodeId` string compares in the compiler hot path (11-12%
inclusive, #1384); stream J. No rendered bit moves.

**D15-16. Research now.** #1058 (stored automation, stream K). It feeds D15-5, E1, the classifier's
automation mask and `AUTOMATION_ENQUEUE`.

**D15-17. Edits never block on render: an asynchronous completion contract, on both hosts.**
- **Submit is synchronous only for what can fail.** `SESSION_TRANSACTION_APPLY` (and the browser's
  `engine.apply`) does these steps synchronously, then returns at once:
  1. validate and classify;
  2. for a rebuild, prepare the successor and reserve its publication and retirement credit;
  3. commit.

  Preparation stays inside submit because it can fail (budgets, ceilings). Committing only after it
  succeeds keeps the acked-batch rule: no ack before a drop. Submit never waits for render, a swap
  or a catch-up.
- **The response** carries `{revision, path}`, where path is `live`, `model_only` or `rebuild`.
  Every committed revision is *pending* until the watermark covers it.
- **Completion is guaranteed and observed, never awaited.**
  - The watermark is `(revision, first sample in effect, outcome flags)`. The revision is the
    highest one that is in effect together with every revision before it.
  - The outcome flags are the OR over every revision the watermark advance covered: `exact`,
    `preroll_fallback`, `transition_fallback`, `superseded`. Saturating counters per outcome back
    them up.
- **The engine owns no thread, so the host drives control work.**
  - A new C ABI entry point, `miso_engine_v1_service`, does bounded work per call: catch-up
    slices, the deadline check, publication, and reclaiming retired plans. It takes the session
    handle, because the engine handle owns no session state (`crates/capi/src/abi.rs:364-369`;
    #1348).
  - The host calls it from any non-realtime thread. Every other control call also services.
  - A pending edit progresses only while the host calls control functions, the same duty as
    draining events today.
  - The browser's Worker runs the same service loop continuously.
- **Deadline and fallback.** The catch-up deadline is counted in render samples, so a paused host
  never triggers a fallback; its edit stays pending until render resumes. When the deadline passes,
  the next service call publishes the successor in fallback mode: render-thread pre-roll bounded by
  `k_max`, else the transition. The outcome flag reports it.
- **Edits submitted during a pending window:**
  - **Live:** committed at once. If a rebuild candidate is pending but not catching up, the edit
    goes to the newest candidate's cells and applies at adoption (#1053 D7). If a catch-up is
    running, the edit is held in the control plane, because the successor already leads render in
    graph time and no exact replay exists; it is written to the successor's cells at publication
    and applies at S. Its revision completes with S. The hold is bounded by the deadline.
  - **Model-only:** committed at once. Its revision completes when every earlier revision has.
  - **Structural:** CAS supersession (D15-9). The displaced candidate's revision completes as
    `superseded`, and its content is part of the newer committed model. The catch-up restarts from
    a new B. Held live edits are already in the committed model the newer candidate is prepared
    from.
  - **Seeks:** anchored on the source-read clock (D15-12). A held `seek_at` moves with its
    producer.
- **A host-declared stop** turns any pending catch-up into a plain rebuild with no continuity
  constraint, applied at the next render.
- *Rationale:* a blocking submit would be a deferred shortcut (round 1, risk 6). Every host thread
  would inherit a dependency on render progress, and a paused host would block. Completion through
  the watermark keeps ack-before-drop impossible: every committed revision completes as `exact`,
  with a counted fallback, or as `superseded` into a later revision, never as nothing.
- *Issues:* #1313, #1314, #1348, #1349 (stream B); #1360 and #1361 (stream C); the Worker in #1332
  and #1381 (stream H).

### Earlier open questions, answered

Each earlier question is answered by the decision named. The earlier recommendation is superseded
where it differs; the issue bodies are rewritten to match.

- **#1053 (C ABI live updates).**
  - Q1 "steps first?" (recommended: ship steps) — superseded by D15-1: every live value ramps.
  - Q2 "BACKPRESSURE while paused acceptable?" (recommended: yes) — superseded by D15-2: cells, no
    capacity BACKPRESSURE for live values.
  - Q3 "tell the host which path ran?" (recommended: not now) — reversed by D15-3 and D15-17.
  - Q4 "infinite tail for live input filters?" (recommended: yes) — superseded by D15-4: a bounded
    tail, never `Infinite`.
  - Q5 (#1306, windows by lane depth) — answered by D15-5.
- **#1269 (seamless swap).**
  - Q1 value edits inside a structural transaction — D15-7: carry, then retarget.
  - Q2 latency growth during playback — D15-8: the warm successor; no reserve.
  - Q3 fade in an added strip — D15-9: yes, over the session mute ramp.
  - Q4 a browser rebuild that blocks the audio thread — D15-10: preparation runs in a Worker.
  - Q5 a removed strip stops at the swap block — D15-9: two-phase removal.
  - Q6 a public browser `replaceSession` — D15-11: one edit API; `replaceSession` only as a
    diffing convenience.
- **The adversary's round-1 owner questions** (`PLAN-2026-10-05-adversary-round1.md`, "Owner
  questions this plan needs").
  - OQ1 (a per-strip latency reserve) — rejected by D15-8: no permanent reserve.
  - OQ2 (duck-swap, crossfade deferred) — adopted by D15-9.
  - OQ3 (nightly browser artifact, isolation) — adopted by D15-10.
  - OQ4 (a correctness reason) — adopted by D15-13 E4.
  - OQ5 (class-B limit-cycle fix) — adopted by D15-4(a).
  - OQ6 (floors drop at a declared discontinuity) — adopted by D15-8.
  - OQ7 (the WIP rule) — adopted by D15-0.
  - "Pending from decision 13: Q5 (bus trim/polarity)" is stale: the owner answered decision 13 Q5
    on 2026-10-04 and kept both (`submix-strips-sends-and-vca-2026-10-02.md`, Q5).
- **Decision 14's findings.** F1 → D15-13 E1. F2 → E2. F3 → D15-6/E3. F4 → E4. F5 is filed
  (#1261-#1267). F6 → D15-11. F7 → D15-1 (ramps) and E5 (bypass crossfade). F8 was closed by #1260.
  F9 → D15-6.
- **The owner's earlier answers stand.** The 2026-10-04 approval of decision 14's four rules, the
  2026-10-04 direction on #1269 ("I've never used a DAW where adding a track would cause an audio
  dropout."), and decision 13's Q1-Q5 answers are unchanged. Decision 15 extends rule 3 (D15-13 E4)
  and delivers rule C1 (D15-11); it reverses none of the owner's own words.
- **Still open:** #1057's personal-mix owner question. It blocks nothing in this decision.

### Issues closed by this decision

- #1291 *Keep browser live strip state across a session replacement* and #1292 *Keep browser live
  effect edits across a session replacement*: not planned (D15-11).
- #1020 *Research: can the C ABI deliver live parameter changes through the browser's live-control
  lane?*: answered by #1053's delivered slices (#1255, #1257) and D15-11 (one portable control plane, #1309).

### Defects found while writing the specs

Filed under the no-shortcuts principle (stream B), outside D15-1 to D15-17:

- #1351 *Report each configured counter's own value in the C ABI counter snapshot*: the periodic
  snapshot reports the render sequence number as every counter's value
  (`crates/capi/src/runtime/control.rs:564`).
- #1352 *Report each configured meter handle's own meter in the C ABI meter batch*: the batch reports
  the master output peak for every configured handle (`control.rs:552`).

### Relation to earlier rulings

- **`AGENTS.md`.** Amended with this record: the WIP rule (D15-0), the supersession sentence
  (D15-2), the browser control plane and allocation rule (D15-10), the browser toolchain (D15-10)
  and the non-blocking submit (D15-17).
- **Decision 14.** Its four rules stand. Rule 3 gains the correctness reason (E4). Its F1-F9 are
  answered above. Its record gains a pointer to this one.
- **Decision 13.** DESIGN 5.7's structural `follows_mute` is reversed (E3). #1247 D1's structural
  VCA membership is reversed on the C ABI (F9). O9's fold of routes into the output is narrowed for
  routes whose tap precedes the fader (D15-9, #1391).
- **`engine-footprint-2026-09-28.md`.** Decision 1 (researched ramp defaults) is delivered by
  D15-1. Decision 2 (the core engine owns the edited session) is delivered by D15-11. Ruling R3
  (no sidecar or WebSocket transport) is untouched: the Worker is an in-process instance on shared
  memory, not a transport.
- **`effect-floor-accounting.md`.** D15-4(a) is a class-B change; the owner's delegation is its
  ruling.
- **`AGENTS.md` "single render-thread".** Unchanged: the Worker runs the control plane, never render.

### Verification

Pending: a fresh Opus 5.5 adversarial verifier checks this record, the `AGENTS.md` amendment and
every spec filed from it.
