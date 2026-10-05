# Agreed plan, 2026-10-05: live updates, seamless swaps, one control plane

Root and adversary, converged over two rounds (`PLAN-2026-10-05-root-draft.md`,
`PLAN-2026-10-05-adversary-round1.md`, `PLAN-2026-10-05-adversary-round2.md`; the evidence and
file:line anchors are there). The owner delegated every decision below under the binding
no-shortcuts principle (`no-shortcuts-correctness-first.md`, 2026-10-05). Each decision is recorded
in a decision-15 ruling (`docs/rulings/`) with its rationale, on decision 13's authority model
("owner-delegated decision, root and adversary").

Read against `main` at `6fb211594`.

---

## 1. Decisions

**D15-0. Principle and delivery.**
- The owner principle governs: no interim placeholder that a later issue must undo.
- Parallel opus-high coordinators may run in separate worktrees, each owning a declared file set,
  with merges into `main` sequenced by root. `AGENTS.md`'s one-launch-critical WIP rule is amended
  in the same PR that records decision 15.
- Every batch merge runs the full gate set once on the merged tree (every past batch's final
  verification caught a blocker).

**D15-1. Every live value ramps.** #1055 (research) then #1054. A record with no ramp length uses
the session's researched `controlSmoothing` default; an explicit 0 stays legal. #1054 covers every
live row: fader, mute, pan or matrix (a model `smoothing_samples` of 0 means "session default"),
input trim and polarity, sends (gain, mute, matrix), VCA offset and mute. SDK defaults stop being 0.
*Rationale:* a step is a click (#1053 A2 D3); the caller must not decide smoothness by omission.

**D15-2. Live value lanes become latest-target cells, on both hosts in one slice.**
Gated conditions:
1. levels only;
2. a value is superseded only by a later commit before the same drain;
3. every fallible check runs before the first cell write;
4. an exact `live_values_superseded` counter.

Automation, Observe records, structural edits and any time-stamped record stay FIFO.

The design:
- each cell holds its target words and ramp as one unit;
- a per-lane dirty mask;
- a canonical drain order, fader before mute;
- multi-word cells use a sequence word that render reads once and never spins on.

The ack bytes are unchanged; their meaning is "render converges to the committed value no later
than the first block whose render begins after the submit returns". `AGENTS.md`'s "never silently
lost" gains one sentence that defines supersession.

*Rationale:* live records carry no sample time and apply at block entry, so FIFO and cells are
observationally identical, except that FIFO is worse for "step then ramp". The engine already
coalesces in three places. The model is the authority (#1053 D9). This removes capacity
BACKPRESSURE for live values on a paused host.

**D15-3. Edit outcome reporting.**
- The response to every transaction carries its path: `live`, `rebuild` or `model_only`, plus
  `rebuild_with_transition` for a counted fallback (D15-8).
- The application sample comes from a monotone watermark, `(revision, first sample fully in
  effect)`, published by render. It is a C ABI query and a browser status field. It never uses the
  reliable event lane (capacity 2).

This reverses #1053 Q3. *Rationale:* the control thread cannot know the block at which render pops
a record or adopts a plan. Revisions are monotone, so a watermark is lossless for the host's
question and cannot back-pressure.

**D15-4. Tail and rest.**
- (a) **Fix the SVF limit cycles**: a joint flush in `svf_step` (`REST_EPS = 1e-14`), with the
  per-word 1e-20 flush kept. This is class B, accepted. Two regression tests, both red on revert:
  the input LPF period-2 cycle and the EQ shelf fixed point. The digest of the one cross-target
  corpus owner is re-baselined.
- (b) **An engine-wide tail contract**: the smallest `T` with `|y[n]| < P·10^(−144/20)` for every
  input of peak `P`, with no control event after it (a ramp in flight is allowed). Each effect and
  builtin computes it from its designer, per rate, on the control thread, with a test that
  recomputes it (never a digest). Every node also states an exact-rest bound. Gain-only
  processors report 0 beyond their latency.
- (c) #1261 and #1262 then report the bounded tail, never `Infinite`.

*Rationale:* exact zero was provably infinite at the top of the cutoff domain, and the EQ had a
second trap. With the fix, exact rest is reached within a stated bound (input section ≤1.0M samples
at +24 dBFS), so silence skipping can rely on it.

**D15-5. #1306: size the span window once.** The size comes from the producers the plan really
has (the live lane depth plus the stored-automation spans per block that preparation computes),
after #1058's design. The caller's S bounds only `AUTOMATION_ENQUEUE` density.

**D15-6. C ABI live completeness.**
- #1225, #1226 (with F3: `follows_mute` live), #1247 (F9: VCA membership live on the C ABI),
  #1261, #1262 and #1267, all built as cells.
- `follows_mute` is also made live on the browser.

**D15-7. Carry across a swap.**
- Carry, then retarget, for live values: carry the state, then emit the classifier's ramped
  records.
- A changed prepared (rule-3) value restarts its owner behind a D15-9 transition.
- P1.4 compares against the committed model of the predecessor plan plus every record pushed to
  it, not against "the model before this transaction".

**D15-8. Latency.**
- **Decrease:** floors (#1285). Floors and the accumulated read-ahead reset at a discontinuity the
  host declares (stop, or a seek of every source).
- **Growth: a warm successor (off-thread catch-up).**
  1. The successor is prepared with floors at the predecessor plus `P`.
  2. At block B, render copies state into it (copy-mode carry, bounded memcpy and allocation-free
     payload calls) and returns it through a capacity-1 queue.
  3. The control thread (C ABI: in bounded `miso_engine_v1_service` slices, D15-17; browser: the Worker) renders it
     forward with the FP environment pinned. It reads sources through a read-only peek cursor,
     whose position gates the ring's release.
  4. Once it leads render by `P`, it publishes "adopt exactly at S, else return". At S, render
     adopts by pointer swap and moves the consumers with their read index at `S + P`.
  5. Live edits during the window are held and apply at S. A structural edit during the window
     supersedes the catch-up.
  6. `ΣP` is bounded by ring headroom (`P_max`).
- **Fallbacks, each counted and reported:** render-thread pre-roll bounded by `k_max`, then the
  documented transition. They apply on a timeout, a host that renders nothing, or a non-isolated
  browser.
- **No permanent latency reserve.**

*Rationale:* "history fill" is impossible, because the needed samples are future processed
samples. A reserve is a permanent cost. Render-thread pre-roll is a 5x–9x spike in one callback.
The catch-up is exact for every path, the edited one included, and render pays one bounded copy.

**D15-9. Transitions for strips whose state cannot continue.**
- **Added strip:** fades in from its first played block (#1288), over the session mute ramp.
- **Edited strip** (changed prepared value, insert added, removed or reordered, quality, link mode,
  `delay_samples`): a **duck-swap**. The live mute ramps down on the predecessor, the swap is
  scheduled at the ramp's end, and the successor fades in.
- **Removed strip:** two-phase (ramp, then a scheduled swap). Its source retires with phase 2, not
  with the commit.
- **Mechanisms:**
  - a scheduled swap ("adopt no earlier than S") and exact-sample adoption;
  - **CAS candidate supersession**: the control thread replaces an unadopted candidate. If render
    has already taken it, the newer candidate's carry program is re-targeted to the adopted plan.
    A held `seek_at` moves with the producer;
  - **live strip fader and mute lanes on every plan, on both hosts.**
- A true crossfade (ghost strips) is deferred. It reopens on a measured, audible dip in a listening
  test.

*Rationale:* no click and no double render. Supersession removes the structural BACKPRESSURE that
a pending candidate causes today (`crates/capi/src/runtime/control.rs:959`), permanently so while
paused.

**D15-10. Browser: one control plane, off the audio thread.**
- Wasm threads with one shared `WebAssembly.Memory`. A Worker instance runs the control plane
  (model, classifier, preparation, catch-up, retirement, disposal). The AudioWorklet instance only
  renders and swaps, and **never allocates or frees after boot**. That is enforced by a static gate
  and a runtime gate (see TOOLCHAIN below).
- Cross-origin isolation is required for structural edits; the first-party app already sends
  COOP/COEP.
- A non-isolated page keeps the same API and the same artifact (a local shared memory, single
  instance). Its structural edit runs the blocking rebuild, reported and counted.
- TOOLCHAIN: a pinned dated nightly with `-Zbuild-std` on `wasm32-unknown-unknown`, for the
  browser artifact only; everything else stays on stable. The stable `wasm32-wasip1-threads` route
  was rejected. It needs wasi-libc internals driven by hand: the internal `__wasi_init_tp`, and a
  stray write into another instance's stack. Render reaches a futex wait, which Chromium traps in a
  worklet. The target does not cover browsers.
- **The allocation rule** is enforced by three gates:
  1. a runtime `GlobalAlloc` counter on a const, instance-local render-locked flag, asserted at
     exactly 0 in browser qualification, with a mutation self-test;
  2. static checks for thread_local destructor registration, `memory.atomic.wait` and a pinned set
     of render `call_indirect` sites. Today's direct-call gate cannot see the executor behind
     `Box<dyn PreparedPlanExecutor>`;
  3. the native counters, unchanged.

  `LIVE_HOST` and `BOOT_STAGING` (`hosts/host-web/src/ffi.rs:503-512`) become const and
  destructor-free.

*Rationale:* this is the C ABI's shape, so there is one implementation. Time-slicing and artifact
transfer were measured and fail (`PLAN-…-round1.md`, C4).

**D15-11. One edit API.**
- capi's control plane (`crates/capi/src/runtime/control.rs` and `compile.rs`) is extracted into a
  portable crate that both adapters call.
- The C ABI keeps `SESSION_TRANSACTION_APPLY`. The browser SDK gains `engine.apply(transaction) ->
  {revision, path}` plus the watermark. `replaceSession(document)` exists only as a convenience
  that diffs the document against the committed model into one transaction.
- The worklet message protocol stays internal.
- #1057 becomes the design note for this. Its personal-mix owner question stays open and blocks
  nothing here.
- B3 (#1291) and B3b (#1292) close as not planned (the committed model replaces their three-way
  merge).

**D15-12. Seek contract.**
- `seek_at` anchors on the plan's source-read clock ("equal to the render clock in this version").
- Every seek during playback is anchored: a plain seek means "the next block render begins", and
  the block it applied at is reported through the watermark.
- Header text covers held behaviour, replacement, the in-flight BACKPRESSURE string, displacement,
  far-past anchors, the clock origin and the source rate = render rate assumption.
- The ABI-growth rule is written down: a feature bit per addition, `FEATURE_MASK` is never compared
  with `==`, and the bit does not protect directly linked hosts.
- "Held" is reported apart from "underrun".
- Test and tooling fixes:
  - commit the verifier's probe P5 (a held seek across a swap);
  - add the supersession case;
  - set a `source.id.invalid` diagnostic;
  - fix the #1293 D4 trap;
  - add a debug assertion for off-grid lateness;
  - make the untimed reads test-only;
  - add one `seek_at` call to `audit capi`.

**D15-13. Decision-14 findings.**
- E1 (F1): a crate that sees descriptors refuses automation on non-`Block` parameters, and so does
  the SDK. It lands before #1058's first rendering slice.
- E2 (F2): make all of these live:
  - the gate-expander's attack, hold and release;
  - the parametric EQ band's `enabled` and `kind` (other `m`/`k` words on the same SVF state, via
    prepared targets);
  - the multiband crossover, with a test that the band sum stays continuous.
- E3: `follows_mute` live (D15-6).
- E4: a **correctness reason may keep a value prepared**, recorded with its reopening condition.
  Every command that would be acked with no effect is refused with a type, `AUTOMATION_ENQUEUE`
  included until #1058 wires it. The delay and the multiband compressor get a live bypass shunt
  (the multiband after #1069). Until then, lifting their bypass is a rebuild with a D15-9
  transition.
- E5: the bypass switch crossfades over the session ramp, and is bit-identical after the ramp.

**D15-14. Meter, observation and spectrum state carry** across swaps for unchanged owners, under
P1, on both hosts.

**D15-15. Preparation performance**, both hosts:
- validate static effect descriptors once per type (~9%);
- no `GraphNodeId` string compares in the compiler hot path (11–12% inclusive).

**D15-17. Edits never block on render: an asynchronous completion contract, on both hosts.**
- **Submit is synchronous only for what can fail.** `SESSION_TRANSACTION_APPLY` (and the browser's
  `engine.apply`) does these steps synchronously, then returns at once:
  1. validate and classify;
  2. for a rebuild, prepare the successor and reserve its publication and retirement credit;
  3. commit.

  Preparation stays inside submit because it can fail (budgets, ceilings). Committing only after it
  succeeds keeps the acked-batch rule: no ack before a drop. Submit never waits for render, a swap or
  a catch-up.
- **The response** carries `{revision, path}`, where path is `live`, `model_only` or `rebuild`.
  Every committed revision is *pending* until the watermark covers it.
- **Completion is guaranteed and observed, never awaited.**
  - The B(4) watermark is `(revision, first sample in effect, outcome flags)`. The revision is the
    highest one that is in effect together with every revision before it.
  - The outcome flags are the OR over every revision the watermark advance covered: `exact`,
    `preroll_fallback`, `transition_fallback`, `superseded`. Saturating counters per outcome back
    them up.
  - This is lossless for the host's questions and needs no event queue that could back-pressure.
- **The engine owns no thread, so the host drives control work.**
  - A new C ABI entry point, `miso_engine_v1_service(engine)`, does bounded work per call:
    catch-up slices, the deadline check, publication, and reclaiming retired plans.
  - The host calls it from any non-realtime thread. Every other control call also services.
  - A pending edit progresses only while the host calls control functions, the same duty as
    draining events today.
  - The browser's Worker runs the same service loop continuously.
- **Deadline and fallback.** The catch-up deadline is counted in render samples, so a paused host
  never triggers a fallback; its edit stays pending until render resumes. When the deadline passes,
  the next service call publishes the successor in fallback mode: render-thread pre-roll bounded by
  `k_max`, else the transition. The outcome flag reports it.
- **Edits submitted during a pending window:**
  - **Live:** committed at once. A rebuild candidate is pending but not catching up: as today
    (#1053 D7), the edit goes to the newest candidate's cells and applies at adoption. A catch-up is
    running: the edit is held in the control plane, because the successor already leads render in
    graph time and no exact replay exists. It is written to the successor's cells at publication and
    applies at S. Its revision completes with S. The hold is bounded by the deadline.
  - **Model-only:** committed at once. Its revision completes when every earlier revision has.
  - **Structural:** CAS supersession (D15-9). The displaced candidate's revision completes as
    `superseded`, and its content is part of the newer committed model. The catch-up restarts from
    a new B. Held live edits are already in the committed model the newer candidate is prepared
    from.
  - **Seeks:** anchored on the source-read clock (D15-12). A held `seek_at` moves with its
    producer.
- **A host-declared stop** turns any pending catch-up into a plain rebuild with no continuity
  constraint, applied at the next render.

*Rationale:* blocking submit was a deferred shortcut (round-1 risk 6). Every host thread would
inherit a dependency on render progress, and a paused host would block. Completion through the
watermark keeps ack-before-drop impossible: every committed revision completes as `exact`, with a
counted fallback, or as `superseded` into a later revision, never as nothing.

**D15-16. Research now.** #1058 (stored automation). It feeds D15-5, E1, the classifier's
automation mask and `AUTOMATION_ENQUEUE`.

---

## 2. Workstreams

Each is one opus-high coordinator with opus-medium workers in their own worktrees and fresh
opus-xhigh verifiers. "Owns" is the file set the stream may edit. A file shared between streams
names its order. The merge order into `main` is root's.

| ID | Stream | Scope (issues) | Owns | Depends on | Parallel-safe with |
|---|---|---|---|---|---|
| S0 | Decisions and specs (root) | decision-15 ruling + AGENTS.md amendment; every rewrite and new spec in §3; GitHub sync | `docs/rulings/`, `AGENTS.md`, `.github/ISSUE_SPECS/` | none | all |
| A | Swap carry, C ABI core | #1300 → #1277 → #1279 → #1280 → #1281 → #1282 → #1283 → #1284 (all with copy mode) → #1285 (+ floor reset) → #1286 → meter carry (D15-14) | `crates/builtins*`, `crates/rack`, `crates/graph`, `crates/graph-compiler` (except perf), effect crates' payload code, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs` | S0 rewrites of #1282, #1284, #1285 | B (until B3), G, J, K, H(a) |
| B | Control plane and protocol | (1) control-plane crate extraction; (2) CAS supersession, scheduled and exact-sample adoption, return queue; (3) latest-target cells, both hosts; (4) path field + watermark with outcome flags and the asynchronous completion contract (D15-17), `miso_engine_v1_service`, browser service loop hook; (5) E4 typed refusals; (6) #1305; (7) seek contract D15-12 | new crate, `crates/capi`, `crates/protocol`, `crates/engine/src/realtime`, `crates/host-core/src/live_delta.rs`, `crates/source` (seek parts) | S0; for (3), A's #1280 must have landed (`effect-contract/src/live.rs`) | A, G, J, K, H(a) |
| C | Latency growth | ring peek cursor and gated release; off-thread plan executor; #1287 rewritten as the warm successor, run in bounded service slices (D15-17); held live edits; deadline in render samples; fallbacks with outcome flags | `crates/source` (after B7), `crates/engine/src/realtime` (after B2), host-core successor code (after A) | A #1284/#1285, B(2), B(7) | E, F, G, J |
| D | Transitions | #1288 fade-in; duck-swap; two-phase removal; live strip lanes on every plan | builtins fade flag, host-core successor, host-web lane request | A #1277, B(2), E #1054 | F, G, J |
| E | Smoothness | #1055 → #1054 → bypass crossfade (E5) | `crates/session`, host-core ramps, `sdk/src/core/live-controls.ts`, `effect-contract/src/live.rs` (after A #1280, B3) | S0 | A, B, G, J, K |
| F | C ABI live completeness | #1261, #1262, #1225, #1226, #1247, #1267, #1268 (D1 check first) | the control-plane crate's classifier, host-core lane attachment | B(1)–(3), G(b) for the tail | C, D, H |
| G | DSP contracts | (a) SVF joint flush; (b) tail contract engine-wide; E2 (gate, EQ, multiband live); F4 live shunts; #1236 per-strip link mode | `crates/lane`, `crates/dsp-reference`, effect crates' parameter code | (a) none; E2/#1236 after A's carry slice for that crate | A (different files in effect crates: coordinate), B, E, J |
| H | Browser control plane | (a) spike now: two instances on shared memory in 3 engines + real iOS, nightly build-std artifact, runtime + static allocation gates (also closes today's executor blind spot), const destructor-free thread_locals, growth, repo parity gates; (b) worker control plane, render-only worklet, policy scripts rewritten; (c) #1290 reshaped, #1293–#1297 rewritten on `engine.apply`, `follows_mute` live, SDK | `hosts/host-web`, `sdk/`, `scripts/check-web-audioworklet*` | (a) none; (b) B(1) and the spike; (c) B(3), B(4), D's live lanes | A, G, J |
| I | Effect automation guard | E1 (validator + SDK) | effect-compiler validation, `sdk/src/core/session.ts` | S0 | all except H(c) (sdk) |
| J | Hygiene | #1232, #1234, #1235, #1237, #1248, #1251, #1301, #1302, #1304; #1303 after A #1277; D15-15 prep performance (coordinate with A's #1285 on graph-compiler) | scripts, tests, listed files | S0 | all |
| K | Research | #1058; #1057 as the D15-11 design note | `docs/handoffs/` | none | all |

**Critical path:** S0 → {A continues, B(1)} → B(2)/(3) → {C, D, F, H(b)} → H(c).

**Start immediately** (no file conflicts):
- A's next slice;
- B(1);
- E's #1055;
- G(a);
- H(a);
- I;
- J;
- K.

That is eight streams; cap concurrent *implementation* coordinators at five (A, B, G, J, and one
of E/I/H(a)) so verification capacity keeps pace.

## 3. S0 spec work

**Rewrite (frozen decisions change):**
- #1269 umbrella: P1.4 base, P4 removal, P7 latency, P10 meters, P12 browser, and the answered
  Q1–Q6.
- #1053 umbrella: Q2 → cells, Q3 → path field, Q4 → bounded tail.
- #1282 and #1284: copy mode alongside move.
- #1285: floor reset at a declared discontinuity; gate 5 is replaced by #1287.
- #1287: the warm successor (D15-8), keeping its proof obligation; its input-clock bullet and gate
  4 follow D15-12.
- #1288: trigger and length per D15-9.
- #1290 and #1293–#1297 on the transaction API; #1291 and #1292 close as not planned.
- #1306 per D15-5, blocked on #1058's design.
- #1054: every live row (D15-1).
- #1261 and #1262: the bounded tail, depending on the tail contract.
- #1226: F3 inside.
- #1247: membership live.
- #1236: make the per-strip change live, or measure the optimisation reason.
- #1268: run its D1 check first.
- #1057: becomes the D15-11 design note.
- Decision-14 ruling: add a pointer to decision 15 where Q3, Q4, F3, F4 and F9 are answered.

**New issues** (one outcome each, half-day slices):
1. Decision-15 ruling + the AGENTS.md amendments (WIP rule, supersession sentence).
2. Extract the control plane into a portable crate.
3. CAS candidate supersession.
4. Scheduled and exact-sample adoption, with a return queue.
5. Latest-target cells on live value lanes (both hosts).
6. The edit path field in the transaction response.
7. The applied-revision watermark with outcome flags, and the asynchronous completion contract
   (D15-17), including `miso_engine_v1_service` and the browser service loop.
8. Refuse acked-but-inert commands (`AUTOMATION_ENQUEUE`, any other no-op found).
9. Seek contract: source-read-clock anchor and anchored plain seeks.
10. Seek header text and the ABI-growth rule.
11. Report held source blocks apart from underruns.
12. Seek tests and tooling: P5, supersession case, diagnostic, `audit capi`.
13. Ring peek cursor with gated release.
14. Off-thread plan executor with a pinned FP environment.
15. Copy-mode carry program.
16. Floors reset at a declared discontinuity.
17. Duck-swap for an edited strip.
18. Two-phase strip removal.
19. Live strip lanes on every browser plan.
20. Carry meter, observation and spectrum state.
21. SVF joint flush.
22. Engine-wide tail contract.
23. Preparation performance.
24. Browser shared-memory spike.
25. Worker control plane.
26. Worklet allocation gates (static and runtime). This also closes today's static blind spot over
    the executor, and makes the render-reachable thread_locals const and destructor-free.
27. The browser artifact's nightly toolchain entry: `rust-toolchain.toml`, `qualification.yml`,
    `npm-publish.yml` and `docs/RELEASE.md`.
28. E1 automation-rate validation.
29. Gate-expander ballistics live.
30. EQ band `enabled` and `kind` live.
31. Multiband crossover live.
32. Delay live bypass shunt.
33. Multiband live bypass shunt (after #1069).
34. Bypass crossfade.
35. `follows_mute` live in the browser.

Every new spec follows `issue-briefs-for-small-models.md`: outcome, anchors, authorized paths,
non-goals, objective gates, test value, and dependencies by exact title. Each is created on GitHub in
the same checkpoint.

## 4. Risks

1. **The nightly toolchain for the browser artifact** (D15-10):
   - A second pinned toolchain enters the release identity, through `rust-toolchain.toml`,
     `qualification.yml`, `npm-publish.yml` and `docs/RELEASE.md`.
   - `-Zbuild-std` may change between nightlies.
   - Mitigation: pin a dated nightly and bump it only together with a re-recorded three-browser
     matrix and the AArch64/native parity gates.
   - The probe showed byte-identical PCM across LLVM 22 and 23, but the repo's parity gates have not
     been run on it yet; H(a) runs them.
2. **iOS Safari:** isolation and a large shared-memory maximum are unverified on a real device.
   H(a) must test them before H(b) starts.
3. **Merge conflicts in hot files:** `crates/host-core/src/prepare.rs` (A, C, D, F),
   `crates/builtins-compiler/src/lib.rs` (A, B(3), D), `crates/effect-contract/src/live.rs` (A
   #1280, B(3), E5), `crates/engine/src/realtime/plan_exchange.rs` (B(2), C),
   `crates/source/src/lib.rs` (B(7), C). The orders are in §2; root merges one stream at a time and
   rebases the others.
4. **Digest churn:** the joint flush, #1054's non-zero defaults, cells ("step then ramp" changes)
   and the bypass crossfade each move digests. Sequence the re-pins and give each its reason; never
   re-pin in bulk.
5. **Catch-up on slow devices:** a device rendering near 100% of a core cannot lead render. The
   timeout and fallback are counted; #1286 and an H(a) measurement size `k_max`, the timeout and
   `P_max`.
6. **Hosts that do not service** (D15-17): a pending edit progresses only while the host makes
   control calls. The header and the qualification doc state the duty, and the C ABI and browser
   harnesses test it. The deadline is in render samples, so a host that renders but never services
   holds live edits until its next control call. That is documented and observable through the
   watermark.
7. **Verification capacity:** five concurrent coordinators produce more verdicts than one root can
   integrate. Cap concurrency, and keep a batch-level full-gate verification per merge.
8. **Scope:** about 35 new issues and 20 rewrites. Hold the half-day slice rule. After two
   checkpoints without advancing, a stream stops and re-scopes (AGENTS.md).
