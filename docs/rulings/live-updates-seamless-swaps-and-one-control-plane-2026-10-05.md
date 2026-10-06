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
  the one edit API (absent means the session default, an explicit 0 stays legal). #1394 owns the
  field (#1054 owns the session key and defaults), #1364 resolves an absent length, #1382 carries it from browser records. The bypass
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
  Each node states two tail values (root, 2026-10-06; #1329 Amendment 3). `T_decay` is the bound
  above for every peak at or above the node's flush floor `P*` (below it the `f32` output near the
  1e-20 per-word flush is no longer relative to `P`); tail reporting uses it (#1261, #1262, PDC).
  `T_rest = max(T_decay, R(P*))`, at least `N_SILENCE` for an enabled filter section, is the bound
  over every peak: from it on the output is below `P·10^(−144/20)` for `P ≥ P*` and exactly zero
  for `P < P*`. Silence skipping (#1107) uses the exact-rest bound (`RestSamples`), which holds for
  every input; `T_rest` is its low-peak case (root, 2026-10-06; #1329 Amendment 3 addendum). An absolute output floor in place of the
  `P < P*` branch is refused: it would make the tail a fixed-level one, which #1328's A8 and A9
  removed. A two-level `P*` is a candidate for #1433.
  A live input-filter retarget (#1407) moves the recursion only through designs and their
  mixtures: every recursion word the kernel loads is a design, the identity at rest with +0.0
  integrators, or within a proven f32 rounding allowance (64 half-ulps plus u·D per word) of the
  convex hull of the designs its history used, so the section's zero-input step stays a
  contraction, ||A(w)||_V ≤ q_design + 1.419e-5 ≤ 1 − 3.79e-5, at every launch rate and block
  size (with the `f32` state rounding, the certified margin is 3.673e-5 at 44.1 and 88.2 kHz and
  3.701e-5 at 48 and 96 kHz; #1329 Amendment 3); a collapsed strip decides every record over
  channel 0's state and renders the bits of the same strip rendered dual. (Root, 2026-10-05; #1407 Amendment 1.)
- (c) #1261 and #1262 then report the bounded tail, never `Infinite`.
- *Rationale:* exact zero was provably never reached at the top of the cutoff domain, and the EQ
  had a second trap (round 1, B4.3; round 2, "Bits moved"). With the fix, exact rest is reached
  within a stated bound, so silence skipping can rely on it. For the input section with a live
  input lane the certified bounds, each including `2·N_SILENCE`, are about 1.27M samples at
  +24 dBFS and 2.59M for any sanitized input (44.1 / 48 / 88.2 / 96 kHz: 1,264,736 / 1,257,840 /
  1,268,585 / 1,262,029 and 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156; #1329 Amendment 3,
  which supersedes the earlier ≤1.0M and Amendment 2's 1.19M / 2.5M). #1433 may tighten them.
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
- **Growth: a primed warm successor (prime adoption; round-5 amendment, below).** The successor's
  nodes split into carried nodes (C), the nodes of the strips it restarts (R: #1324 D1's
  `restarted_strips()`) and added nodes (N).
  1. Submit prepares the successor with floors at `a(n) + P` on carried nodes only; restarted nodes
     keep #1285's floors `a(n)`, added nodes have none. `Δ` is the largest arrival growth over the
     carried nodes, and `P = q·⌈Δ/q⌉`. Its source-read offset is the predecessor's plus `P` (#1396).
  2. Preparation checks alignment: every carried node arrives at exactly `a(n) + P`. If one is
     late, preparation takes the first late carried node in schedule order and walks back from
     its late inputs through restarted and added nodes, along every edge kind (route, added
     route, sidechain; PDC takes its maximum over sidechain edges too). It restarts whole every
     carried strip that holds that path at its lead (a route counts as its source strip), and
     no other strip (#1324 D2). The walk always reaches a carried node, since the two compiles
     differ only in the carried nodes' floors; reaching none is an invariant error. `Δ` is
     recomputed and the check repeats (#1354 D2 step 6). Preparation returns
     `WarmUnavailable::Misaligned` only when every predecessor strip that reaches the output is
     restarted whole, and the edit takes the transition, which gives the same audio. The walk is
     exact but not minimal: in the round-7 review's model about 5% of random sessions duck 1-4
     more strips than the smallest exact set, recorded as known behaviour (#1354 D2 step 6). So
     a latent insert on a bus restarts the bus's carried feeders, a kick that keys a bass
     compressor restarts the kick when the bass gains a limiter, and every other path, the
     output included, stays exact. If the restarts leave `Δ = 0`, the edit is an ordinary
     rebuild that duck-swaps those strips.
  3. Submit publishes the candidate as `Primed { not_before, lead_blocks }` (#1311).
  4. Render checks readiness (C4 below) on the **active** plan's consumers before it claims the
     candidate. Once ready, it claims and adopts in move mode in the same block, and fills the
     claim lines from the predecessor's pending frames and a raw-frame prime of the next
     `P/q` blocks of each carried source (`prime_block_at`, #1320). Nothing is rendered ahead, and
     no state is copied.
  5. A live edit committed while the candidate is pending is an ordinary pending-candidate edit: it
     goes to the newest candidate's cells and applies at adoption (#1053 D7). A structural edit
     supersedes the candidate by compare-and-swap (#1310).
  6. `ΣP` is bounded by `P_MAX`, and the prime by `PRIME_BYTES_MAX` (#1286).
- **Fallback, counted and reported:** the transition only, the D15-9 duck-swap of every strip whose
  content timing moves (#1397 D2's `grown_strips`: a strip with a node whose arrival grows, or with
  an outgoing route, send or output path whose compensation line changes length, unedited strips
  included), closed under #1324 D1's sidechain rule over the joined strips. That set is computed
  from the ordinary compile before the carry join and passed to preparation as
  `SuccessorBase::forced_restart`, so each of its strips is restarted before the fader and armed
  (#1324 D2-D3) and joins #1324's restarted strips. `S` is counted with #1324 D4's `C` (each
  sidechain line from a ducked strip's `post_fader` or `post_pan` tap included), and the edit is
  counted `TRANSITION_FALLBACK`. It applies when preparation returns
  `WarmUnavailable`, or when readiness is still unmet `prime_deadline_samples` of render after
  `not_before` (counted in render samples: one stall tolerance plus `P_MAX` plus one quantum,
  #1358 D1). There is no render-thread pre-roll. While render waits for readiness, the
  predecessor keeps playing exactly. A host that never queues `P + q` frames ahead gets the
  transition, because any exact mechanism needs those future frames.
- **No permanent latency reserve.**
- *Recorded resolution:* "a host that renders nothing" is not a fallback trigger. D15-17 counts the
  deadline in render samples, so a paused host's edit stays pending and never falls back; D15-17
  governs. *Superseded by the round-5 amendment:* "A catch-up abandoned at a host-declared stop
  completes its revision as `exact`". Now a pending `Primed` candidate at a declared stop is
  superseded by a plain rebuild through #1310 (#1323 D2); its revision completes `exact` or
  `superseded`, because a declared stop owes no continuity.
- *Recorded amendment (round 5): prime adoption.* Root, under the owner's delegation, replaces the
  off-thread catch-up of steps 2-5 of the earlier text (in git history at `6b8bc7c96`: render
  copied state into the successor at a block B and returned it; the control thread rendered it
  forward to lead render by `P`; render adopted it exactly at S) with prime adoption.
  - *Lemma.* Predecessor A has source-read offset `O`, recorded floored arrivals `a(n)` (#1285 D2)
    and claim lines of length `λ`; successor W has floors `a(n) + P` on C, `a(n)` on R, none on N,
    and offset `O + P`. Conditions, each exact or the edit takes the transition:
    1. C1, alignment: `a'(n) = a(n) + P` for every carried node, checked at preparation.
    2. C2, isolation: every edge from R or N into C carries exact `+0.0` from S until its fade
       fires (fader and post-fader paths, ducked routes, armed fades). A sidechain edge has no gain
       lane, so the tap decides, by #1324 D1's rule: a sidechain from an R strip's `post_input`,
       `insert_send`, `insert_return` or `pre_fader` tap makes the consuming strip join R
       (repeated until none joins). A track's `input` tap (raw source) and a `post_fader` or
       `post_pan` tap (exact `+0.0` from the duck's end to the fire) keep the consumer in C. A
       restarted submix's `input` tap (the sum of its incoming routes) keeps it in C if every line
       into that submix's `Input` stage comes from a restarted or added strip (exact `+0.0` from
       the duck's end to the fire, with `S` counting the sidechain line and the longest line into
       the stage, #1324 D4, for every duck-swap), and otherwise only if that stage arrives at
       exactly `a(n) + P`, checked with C1 (with `P = 0` for a rebuild that grows nothing, #1354
       D2 step 4); otherwise the lines into that stage and out of it change length and the
       consuming strip joins R.
    3. C3, bounds: `ΣP + P ≤ P_MAX` and prime bytes `≤ PRIME_BYTES_MAX`.
    4. C4, readiness, checked by render on the active plan before it claims: S is at or after
       `not_before`; every source W carries has its next `P/q + 1` blocks queued and playable; no
       command is queued and no held seek is anchored below `S + O + P + q` (#1320 D2: the window
       `[S + O, S + O + P + q)` and an anchor in `(S + O − q, S + O)`, which applies late at
       `S + O`).

    Then:
    1. L1, edges: a C-to-C edge keeps its length, so its line moves unchanged.
    2. L2, claim lines: a W line of length `λ'` is filled with the last `λ'` samples of A's pending
       frames followed by the prime, the next `P/q` consumer blocks on A's own schedule. A grown
       line (`λ' = λ + P`, on C) then emits exactly what A's line would; a restarted line takes the
       shifted fill; an added line has zeros only at its head.
    3. L3, an Input-tap line from R into C grows by `P` and is filled with A's line pending, A's
       claim pending and the prime, which yields A's content.
    4. L4, induction: under C1-C4, for every carried node and every render sample from S up to the
       first fade that fires upstream of it, W's state and input equal A continued without a swap.
       State moves by move-mode carry; inputs are equal by L1-L3 and C2; banking moves no bit.

    Against an uninterrupted render of the new graph, carried nodes that no restarted or added
    node feeds are equal. Downstream of a restarted or added node no mechanism can equal it,
    because the new chain has no history; there the contract is D15-9's fade-in and duck-swap,
    whose references (#1288, #1324) prime adoption meets exactly.
  - *Evidence.* The round-4 verifier's finding M6: with B1 fixed, a carried node in the successor
    processes at each render sample exactly what the predecessor's node processes, so the
    catch-up re-derives state that move-mode carry gives directly. A round-5 analysis (Opus 5.5,
    extra-high effort) confirmed it with a standalone model of prime adoption (not engine code),
    bit-exact over 60-80 blocks for #1397 gate 1's shape plus an Input-tap sidechain from the
    restarted track into a carried compressor, and for growth, then an ordinary rebuild, then a
    second growth. Three mutants diverge exactly at S: a zeros-first grown claim line, claim lines
    not carried, and a sidechain line filled with zeros (the third only in the sidechain case).
    The model is `docs/handoffs/decision-15-2026-10-05/m6/model.py` and `mutants.py`
    (`python3 model.py`; `python3 mutants.py`); it is design evidence, not a gate.
  - *Why it replaces the catch-up.* The catch-up computes nothing for a carried node that the
    predecessor does not. Where it differs it is worse: its zeros-first claim-line fill leaves a
    detector hole in a carried effect sidechained from a restarted strip's Input tap, so the
    effect's state jumps at S; and restarted or added chains warm for `S − B` samples, a length
    that depends on wall time, is not reproducible and does not match #1324's reference (a fresh
    plan from S). Prime adoption removes the off-thread executor, copy-mode carry, the peek pool,
    the return queue, held edits, catch-up supersession and render-thread pre-roll. Ring headroom is
    `P_MAX` plus one quantum, with no deadline term.
- *Rationale:* "history fill" is impossible, because the needed samples are future processed
  samples, and the host already queues them as raw source frames. A reserve is a permanent cost.
  Render-thread pre-roll is a 5x-9x spike in one callback. Prime adoption is exact on every
  carried path, and render pays one bounded raw-frame fill (round 1, C2; round 2; round 5). An
  edited or restarted strip takes its D15-9 transition, which no mechanism can avoid.
- *Issues:* #1285, #1323 (stream A); #1287 and its slices #1320, #1354, #1355, #1358, #1360, #1361,
  #1396, #1397, #1402, #1403, #1406 (stream C); #1311 (stream B). Retired by the round-5 amendment (closed as not
  planned): #1321, #1322, #1353, #1356, #1357, #1359, #1362.

**D15-9. Transitions for strips whose state cannot continue.**
- **Added strip:** fades in from its first played block, over the session mute ramp (#1288).
- **Edited strip** (a changed prepared value; an insert added, removed or reordered; quality; link
  mode; `delay_samples`): a **duck-swap**. The live mute ramps down on the predecessor, the swap is
  scheduled at the ramp's end, and the successor fades in (#1324).
- **Removed strip:** two-phase: a ramp, then a scheduled swap. Its source retires with phase 2, not
  with the commit (#1325).
- **Mechanisms:**
  - a scheduled swap ("adopt no earlier than S", #1311). *Superseded by the D15-8 round-5
    amendment:* exact-sample adoption ("adopt exactly at S, else return") and #1311's return queue;
    a warm successor is published `Primed` and adopts at the first ready block;
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
  (model, classifier, preparation, the warm-successor deadline check, retirement, disposal). The AudioWorklet instance only
  renders and swaps, and **never allocates or frees after boot**.
- Cross-origin isolation is required for structural edits; the first-party app already sends
  COOP/COEP.
- A non-isolated page keeps the same API and the same artifact (a local shared memory, single
  instance). Its structural edit runs the blocking rebuild, reported and counted. Warm latency
  growth needs no isolation: render adopts a `Primed` successor (D15-8) on either page, so both
  report it `exact` (#1361).
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
- After a warm adoption the source-read clock leads the render clock by `ΣP` (#1396), and a held
  `seek_at` anchored inside a prime window waits for the adoption, then applies exactly (D15-8
  C4).
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
  or a warm adoption.
- **The response** carries `{revision, path}`, where path is `live`, `model_only` or `rebuild`.
  Every committed revision is *pending* until the watermark covers it.
- **Completion is guaranteed and observed, never awaited.**
  - The watermark is `(revision, first sample in effect, outcome flags)`. The revision is the
    highest one that is in effect together with every revision before it.
  - The outcome flags are the OR over every revision the watermark advance covered: `exact`,
    `transition_fallback`, `superseded`. Saturating counters per outcome back them up. A failed
    re-preparation for the transition is counted by `transition_reprepare_refusals`, surfaced in
    the C ABI counter snapshot (#1351). (*Superseded by the D15-8 round-5 amendment:* the
    `preroll_fallback` flag and `catch_up_reprepare_refusals`.)
- **The engine owns no thread, so the host drives control work.**
  - A new C ABI entry point, `miso_engine_v1_service`, does bounded work per call: the
    warm-successor deadline check (with the transition's re-preparation when it fires), publication,
    and reclaiming retired plans. It takes the session
    handle, because the engine handle owns no session state (`crates/capi/src/abi.rs:364-369`;
    #1348).
  - The host calls it from any non-realtime thread. Every other control call also services.
  - A pending edit progresses only while the host calls control functions, the same duty as
    draining events today.
  - The browser's Worker runs the same service loop continuously.
- **Deadline and fallback.** The warm-successor deadline (`prime_deadline_samples`, #1358 D1) is
  counted in render samples from the candidate's `not_before`, so a paused host never triggers a
  fallback; its edit stays pending until render resumes. When the deadline passes, the next control
  call withdraws the `Primed` candidate (a `Taken` result means render adopted it exactly),
  re-prepares with the withdrawn candidate as the donor and publishes the transition (#1358, #1397).
  `transition_fallback` reports it. If that re-preparation is refused (a defect, counted), the
  donor is republished as it was and the next call retries, so a candidate is never held outside
  the mailbox.
- **Edits submitted during a pending window:**
  - **Live:** committed at once. If a rebuild candidate is pending, the edit goes to the newest
    candidate's cells and applies at adoption (#1053 D7). A pending warm candidate is an ordinary
    pending candidate (#1053 D7).
    *Superseded by the D15-8 round-5 amendment:* the hold of live edits during a catch-up (written
    to the successor's cells at publication, #1356).
  - **Model-only:** committed at once. Its revision completes when every earlier revision has.
  - **Structural:** CAS supersession (D15-9, #1310, including its step that returns persisting
    producers to the running plan). The displaced candidate's revision completes as `superseded`,
    and its content is part of the newer committed model. A displaced `Primed` candidate is
    superseded the same way; the newer candidate is prepared, warm or not, against the plan render
    runs.
  - **Seeks:** anchored on the source-read clock (D15-12). A held `seek_at` moves with its
    producer.
- **A host-declared stop** supersedes a pending `Primed` candidate by a plain rebuild with no
  continuity constraint, through #1310, applied at the next render (#1323 D2). Its revision
  completes `exact` or `superseded`.
- *Rationale:* a blocking submit would be a deferred shortcut (round 1, risk 6). Every host thread
  would inherit a dependency on render progress, and a paused host would block. Completion through
  the watermark keeps ack-before-drop impossible: every committed revision completes as `exact`,
  with a counted fallback, or as `superseded` into a later revision, never as nothing.
- *Recorded resolution (round 4, M4):* the round-4 verifier found that this point and #1287 W5
  still described the pre-N3 mechanism (retargets written before every publication, live edits
  held and written at publication), which an implementer could follow back into N3 (a stale
  retarget overwriting an acked live value). The D15-8 round-5 amendment resolves it: a warm
  successor's retargets are written once, at preparation (#1277 D5, move mode), and a live edit
  goes to the pending candidate's cells like any other (#1053 D7), so no later write can overwrite
  it.
- *Issues:* #1313, #1314, #1348, #1349 (stream B); #1403, #1360 and #1361 (stream C); the Worker
  in #1332 and #1381 (stream H).

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
  - Q2 latency growth during playback — D15-8: a primed warm successor (round-5 amendment); no
    reserve.
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
- **Stream C's own earlier specs (round-5 amendment).** Seven slices of the off-thread catch-up
  have no role under prime adoption (D15-8, "Recorded amendment (round 5)") and close as not
  planned: #1321, #1322, #1353, #1356, #1357, #1359 and #1362 (listed under "Issues closed by this
  decision"). A general move-mode rule that one of them held moves into the spec that uses it.
- **Still open:** #1057's personal-mix owner question. It blocks nothing in this decision.

### Issues closed by this decision

- #1291 *Keep browser live strip state across a session replacement* and #1292 *Keep browser live
  effect edits across a session replacement*: not planned (D15-11).
- #1020 *Research: can the C ABI deliver live parameter changes through the browser's live-control
  lane?*: answered by #1053's delivered slices (#1255, #1257) and D15-11 (one portable control plane, #1309).
- Closed as not planned by the D15-8 round-5 amendment (prime adoption has no off-thread render,
  no copy-mode carry and no catch-up):
  - #1321 *Render a successor plan off the render thread with a pinned floating-point environment*;
  - #1322 *Carry plan state by copy as well as by move* (copy mode has no consumer);
  - #1353 *Keep source transfer blocks in a shared pool, immutable from publication to release*;
  - #1356 *Hold live edits during a catch-up and apply them at the adoption sample* (#1053 D7
    applies);
  - #1357 *Supersede a running catch-up by a structural edit* (#1310 applies, its step 5
    included);
  - #1359 *Turn a pending catch-up into a plain rebuild at a host-declared stop* (absorbed by
    #1323 D2);
  - #1362 *Copy a per-node effect's state into a same-layout instance in one pass* (copy mode
    only).

### Defects found while writing the specs

Filed under the no-shortcuts principle (stream B), outside D15-1 to D15-17:

- #1351 *Report each configured counter's own value in the C ABI counter snapshot*: the periodic
  snapshot reports the render sequence number as every counter's value
  (`crates/capi/src/runtime/control.rs:564`).
- #1352 *Report each configured meter handle's own meter in the C ABI meter batch*: the batch reports
  the master output peak for every configured handle (`control.rs:552`).

### Root decisions after S0

Made by root under the owner's no-shortcuts delegation while stream G implemented D15-4. Each
decision lives in its issue's GitHub body (the issue's own branch carries the spec file):

- **#1328** *Flush the SVF jointly so builtin and EQ filters reach exact rest*, Amendment 1
  (A1-A6): the V8 spill gate's select classifier is corrected, not weakened; the EQ's per-frame
  output-limit flag is restructured so the dual depth-1 tail carries no stack slot; the masked
  mono depth-2 pair's `ic1` spill is eliminated (A6 chose elimination, not A3's exception path:
  each section's dry mask is kept in state); D6 (class B) is restated by change size.
- **#1328 A8 (supersedes the A4/A7 rest threshold), itself superseded by A9:** A8 armed the joint
  flush on a sample whose section input is exactly zero. The attempt-4 verdict showed that this
  still loses a sparse signal's boost (non-zero samples below the rest limit with exact zeros
  between them: about −120 dBFS at four +24 dB shelves) and moves bits inside a live EQ where a
  section's own input cancels to zero.
- **#1328 A9 (supersedes A8):** silence is a time property. Each effect input channel (the builtin
  input stage, the parametric EQ, the multiband compressor) keeps one counter word per lane, the
  run of exactly-zero input samples, and a section may apply the joint flush only once its effect
  input has been zero for `N_SILENCE` samples, a time of `4096 / 48000` s (3,764 to 8,192 samples
  by rate; 1,024 samples left a sparse-input residual above one tail's worth at every rate). While
  the effect input is live, sparse or tiny, no bit moves; in tails the change is at most one tail's
  worth (analytic bound `1.95e-13` at one +24 dB shelf, largest found `3.5e-10` at four, −189 dBFS),
  and on a block of live audio the builtin chain and the EQ run their unarmed form, with no
  counter, no threshold and no joint term in any frame loop: what remains is one armability test
  per channel per block and one compare of the block's last frame (the multiband compressor runs
  the counter and the joint flush on every frame). A block in which some lane ends on an exact
  zero, common in quiet 16-bit sources, adds a backward scan from the last frame that stops at
  each such lane's first non-zero frame: within noise natively (an eight-lane builtin block
  1.005-1.008 of all-live, an eight-lane EQ block 0.98-1.02; before the backward scan they paid
  13 % to 21 % and 7 % to 9 %). A bank with a silent or padding lane beside live ones scans that
  lane's whole block (about 3 % of an eight-lane builtin block; the EQ's within noise). In the
  browser the builtins' dual loop costs about 2 % to 5 % p50 on the 64-track documents against
  the per-word build; the cause is the loop's structure since #1328, not the chain constants, and
  #1454 owns it (#1451 removed the iOS memset cause). #1328's follow-up record has the numbers.
  The counter rides the state payload, the carry and the mono-collapse disengage copy. Details:
  #1328 Amendment 1 A9 and `dsp-research/filters.md`.
- **#1329** *State a bounded tail and an exact-rest bound for every node*, Amendment 1: option (m),
  the live filter retarget law, is *Retarget a live input filter only through its designs and their
  mixtures* (#1407); D11's endpoint clamp is *Keep every trim, fader and matrix ramp inside its
  endpoints* (#1408); the effect-parameter counterpart is *Keep every effect parameter ramp inside
  its endpoints* (#1409). #1407 and #1408 are prerequisites of #1261 and #1262 (#1053 D13).

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

Fresh Opus 5.5 adversarial verifiers, none of whom wrote the record, checked it eight times:

- **Round 1 (whole record, about 900 anchors): FAIL**, five blockers (an ack lost on the pre-roll
  fallback, a watermark that aliased under supersession, BACKPRESSURE for a live link value, an
  unmeetable exact-rest definition, no browser service export) and about forty majors. All were
  folded in; fourteen over-size slices were split (#1388-#1401).
- **Round 2 (the fixes, about 400 anchors): FAIL**, two new blockers on the warm-successor path (a
  revision reported in effect while its candidate was held by the control thread; donation from a
  successor the catch-up had rendered) and ten majors. All were folded in.
- **Round 3 (focused on the round-2 fixes, about 270 anchors): FAIL**, every round-2 finding
  resolved, one new blocker (the render pre-roll could not read an added source) and ten majors,
  nine of them on the warm successor and the browser carry. All were folded in.
- **Round 4 (the round-3 fold, the stream C lemma and 15 random non-C specs): FAIL**, one blocker
  B1 in the warm-successor lemma (lead floors on every surviving node broke the equality the proof
  needed whenever a grown path started at a surviving source), six majors M1-M6 at stream C's seams
  (claim lines without a carry owner, a candidate returned after render claimed it, a missing
  supersession step, the pre-N3 text in #1287 W5 and D15-17, an unpassable #1280 gate, and M6:
  the catch-up computes nothing the predecessor does not), and majors in non-C specs. All were
  folded in.
- **Round 5 design analysis (M6, not a verifier round):** an extra-high-effort analysis
  confirmed M6 with a standalone model and three mutants. Root amended D15-8 to prime adoption
  (D15-8, "Recorded amendment (round 5)"), which also resolves B1 (floors on carried nodes only,
  with the alignment check), M1 (claim lines carry and fill by L2 and L3, #1402), M2 (no candidate
  is returned), M3 (a warm candidate is superseded through #1310, its step 5 included), M4 (D15-17's
  recorded resolution) and M5 (the copy-mode gate goes with copy mode), and retires seven stream C
  slices.
- **Round 5 (the prime-adoption fold): PASS-WITH-FIXES**, no blocker. It confirmed M6 and found six
  majors: an `input`-tap sidechain from a restarted submix whose `Input` stage misses `a(n) + P`
  (M1), a C1 iteration that sent a limiter on a bus to the whole-mix transition (M2), a ring
  headroom baseline measured against a default that #1358 grows (M3), a control-plane warm path
  with no owner or lifecycle (M4), four unpassable gates (M5) and an over-size #1355 (M6), plus
  minors. All were folded in before merge.
- **Round 6: PASS-WITH-FIXES**, no blocker. Five majors: a C1 iteration that restarted feeders
  only through a held-late `Input` stage and had no rule at the output (M1; now the walk of step
  2), an unpassable `g < P` gate (M2), a warm growth that also removes a strip published without
  the ramp-out (M3; now routed on #1324 D4's duck set), a refused deadline re-preparation with no
  holder (M4; now republished, #1358 D3), and a default ring that missed quanta other than 128 and
  pins outside its slice (M5; `p_max_samples(fs, q)`, split to #1406), plus minors. All were folded
  in before merge.
- **Round 7: PASS-WITH-FIXES**, no blocker. Three majors: the walk's feeder recursion (M1), the
  #1406 pins (M2) and the `S` term for sidechain lines (M3). All were folded in.
- **Round 8: PASS-WITH-FIXES**, no blocker. One major: the transition's grown strips fell outside
  #1324 D4's `C` and #1397 D2's duck set for sidechain lines (M1). It was folded in, with the
  minors.
- **Round 9: PASS-WITH-FIXES**, no blocker. One major: the transition computed its grown strips
  after preparation, so a grown strip that preparation carried was never restarted or armed (M1;
  now `SuccessorBase::forced_restart`, #1397 D2 step 1). It was folded in, with the minors.

The authority statement, the decision coverage, the dependency graph (acyclic) and the GitHub titles
passed every round. Every blocker from round 2 on was in stream C (the warm successor) or its
seams with streams B, D and H. Stream C is now #1287, #1320, #1354, #1355, #1358, #1360, #1361,
#1396, #1397, #1402, #1403 and #1406. Its coordinator still runs one fresh design verification of those specs
before the first implementation slice (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`).
