# Adversary round 1: challenge of the root draft of 2026-10-05

Read against `main` at `6fb211594`, `AGENTS.md`, the owner memory files (binding:
`no-shortcuts-correctness-first.md`), decision 14 with its amendment, decision 13, and the open
specs. Four opus-xhigh investigations ran in parallel (seek_at review, input-filter tail, live
queues, browser off-thread preparation); their findings are folded in where cited as [inv-…].

Verdict up front: the draft's direction is right, but five positions are either infeasible as
written or are themselves shortcuts that the owner principle forbids, and the plan misses one
architectural decision that changes the order of half the work.

1. **C2 "history fill" cannot work** (proof below). Latency growth has exactly three exact
   remedies: compute the missing samples (pre-roll), never let the timing move (a reserve), or
   accept a transition. The draft must pick a combination, not a fourth option.
2. **C6 + C4 + #1057 are one decision, not three.** The browser B-series (#1290-#1297) is built
   around a document replace with a three-way merge (B3/B3b) *because the browser has no committed
   model*. Under C6 and the owner principle that merge is throwaway work. The correct shape is one
   portable control plane (session store, classifier, preparation, plan publication) used by both
   hosts, with the browser running it off the audio thread. That reshapes B2-B8.
3. **B3 "report the render sample in the response" is infeasible synchronously.** The control
   thread cannot know the block at which render pops a record or adopts a plan. Report the path
   synchronously and the application sample asynchronously.
4. **C5 two-phase removal is right but incomplete**; it needs a scheduled swap, candidate
   supersession, and a source lifetime rule, and the same mechanism should also cover an edited
   strip whose state cannot continue (prepared values, latent inserts, `delay_samples`).
5. **B5 (#1306) as drafted is a shortcut**: it would resize the span window now and again when
   #1058 renders stored automation. Size it once, from the plan's real producers.
6. **Missing**: #1058 (stored automation) and #1057 (browser model) are not in the plan, yet E1,
   B5, the classifier's automation mask and `AUTOMATION_ENQUEUE` depend on them; structural edits
   still get BACKPRESSURE while a candidate is pending or the host is paused (the same defect B2
   fixes for values); floors ratchet latency up for the life of a session.

---

## A. Seamless swap phase 2

**A1: agree, with three amendments.**

- **#1287 and #1288 must be rewritten**, not just unblocked: their designs assumed Q2/Q3
  answers the draft now changes (see C2, C3, C5).
- **#1290-#1297 must be re-planned after the C4/C6 decision** (see C6). B2's Rust replace method
  is still useful as the browser's rebuild primitive, but B3 and B3b (three-way merge of document
  and live overlays, `.github/ISSUE_SPECS/1291-…md`) exist only because the browser has no
  committed model. Under one control plane the committed model *is* the merge result, exactly as
  P1 already argues for the C ABI (`1269-…md` P1, "Why the committed model"). Doing B3/B3b now and
  #1057 later is the "fix it later" pattern the owner ruled out.
- **P10 "meters restart at the swap" on the browser is a shortcut too.** Meter, observation and
  spectrum windows of unchanged strips are state owners like any other; a producer watching a
  level or spectrum sees it reset at every structural edit. Carry them under P1 like DSP state
  (they are plain data), or record a rule-3 reason. Add as a slice.

## B. Mobile live updates

### B1 (ramps everywhere, #1054/#1055): agree, with one correction

- Do #1055 first (research is cheap and its result is a table #1054 consumes), then #1054.
- **No step may remain anywhere**, so #1054 must also cover the rows #1054's text leaves open:
  input trim/polarity (`LiveRamps::input_samples`, 0 today), send gain/mute/matrix (decision 13
  P6: caller-chosen, SDK default 0), VCA offset/mute, and pan with model `smoothing_samples == 0`
  (#1054 "Coordination", which leaves it undecided). Rule: a ramp length of 0 is legal only when a
  caller asks for it explicitly; an absent length means the session's researched default.
- The `-0.0` artefact #1054 records ("a fader move on a lane that stays muted") is acceptable and
  correct; gates compare after the ramp.

### B2 (no BACKPRESSURE for value edits): ruling — compatible, with four conditions

**Ruling: coalescing superseded value targets is compatible with "commands are never silently
lost", and it is the better design.** Evidence [inv-queues]:

- **No live value record carries a sample time.** `TrackFaderRecord` and `TrackControlRecord`
  (`crates/builtins-compiler/src/lib.rs:105-148`), `EffectControlRecord`
  (`crates/effect-contract/src/live.rs:53-100`) and the browser's 48-byte records
  (`hosts/host-web/src/lib.rs:4113-4180`) all apply at the first sample of the block that drains
  them (`live.rs:17-24`). Every record popped at one block entry therefore lasts zero samples except
  the last per address: a FIFO and a latest-target cell are observationally identical, except that
  FIFO is *worse* for "step then ramp" (it snaps to the never-heard intermediate and ramps from
  there, `crates/builtins/src/lib.rs:2731-2750`).
- **The engine already coalesces in three places:** the effect lane's `stage` collapses records
  last-wins per `(parameter, channel)` within one drain (`live.rs:301-345`); the browser merges solo
  and VCA mutes in one submission (`hosts/host-web/src/lib.rs:4687-4699`); the SDK's
  `LiveControlWriter` is latest-wins with a `coalesced` counter (`sdk/src/core/writer.ts:21-26`,
  `:136-150`) — exported but wired to nothing.
- **The model is the authority** (#1053 D9); render converges to it. The v1 bug was the ledger and
  render diverging for good, not an intermediate value going unheard.

**Conditions (all four are gates):**

1. **Levels only.** Absolute value assignments on value lanes. Never automation
   (`AutomationRecord` has absolute `start`/`end`, `crates/protocol/src/queue.rs:56-71`), never
   Observe records (keep a small FIFO beside the cells), never structural edits.
2. **Same boundary only.** A value is superseded only by a later commit to the same address
   before that address's next drain.
3. **Validate before any write.** #1053 D6's order is kept: every fallible check, then the
   infallible cell writes, then the commit. With cells a partial write followed by a refusal would
   *destroy* an acked value (worse than FIFO, where a partial push only adds), so a gate must turn
   red if a write moves before the predicate.
4. **Counted.** An exact saturating `live_values_superseded` counter (the dirty bit's swap tells
   whether the old value was unread). This is a counter-registry addition, so #1053 D10's "no new
   field" is amended.

**Design.** Preallocated cells per address — `(strip, stage, channel, kind)`, `(effect instance,
parameter, channel)` plus bypass, `(EQ instance, section, channel)` with `Both` written to both —
each holding target words plus ramp length as one unit; a per-lane dirty mask so a drain is
O(dirty); a fixed canonical drain order (fader before mute, so a mute ramp in flight is never cut
short by a fader move on a muted lane). Multi-word cells (matrix, EQ target words) under concurrent
C ABI delivery use a sequence word read **once**: on a torn read the dirty bit stays set for the
next block; render never spins. Ship on both hosts in one slice through shared builtins,
effect-contract and host-core code, or #1054's browser-versus-C ABI bit-identity gate breaks for
any script with two edits in one block. The channel-symmetry witness folds from final cell
contents.

**The draft overclaims:** "never refused for capacity" holds for live-lane capacity only. Still
typed and atomic: live admission bytes (#1053 D8), the replay cache, and a pending rebuild
candidate (`crates/capi/src/runtime/control.rs:959` on the current tree) — the last one is fixed by
candidate supersession (C5.2), the structural twin of this item.

**Ack semantics** (documented, bytes unchanged): "committed at revision r+1; render converges to
the committed value no later than the first block whose render call begins after the submit
returns." No superseded-by request ID (no consumer); B3's watermark answers "since when".

### B3 (report the outcome of every edit): agree on the goal, the draft's shape is infeasible

- **Path, synchronous.** The response to `SESSION_TRANSACTION_APPLY` gains one field: `live`,
  `rebuild` or `model_only`. The classifier already decides it before the commit
  (`crates/host-core/src/live_delta.rs`), so this is exact. This reverses #1053 Q3 ("not now");
  it is a protocol change and must land before launch freezes the response shape.
- **Application sample, asynchronous.** The control thread cannot know it:
  - a live record is applied at whatever block boundary the render thread pops it (concurrent
    SPSC; no stamp exists today);
  - a rebuild is applied when `enter_block` adopts the candidate
    (`crates/engine/src/realtime/plan_exchange.rs:358-404`), which for an unreserved candidate can
    be deferred (`DeferredRetirementFull`, `:375-399`) and while paused never happens.
  So report it as a **monotone watermark** the render thread publishes: `(revision, first sample at
  which that revision is fully in effect)`, two atomics written once per block with a change. It is
  lossless for the question a host asks ("is revision R audible yet, and since when?") because
  revisions are monotone, it needs no event queue, and it can never back-pressure an edit. A live
  record's "in effect" means "its ramp has started"; the response may also return the ramp length
  so the host can compute the end.
  - C ABI: a new query (or a field of the existing render-peak telemetry,
    `crates/capi/src/runtime/plan.rs:5`). Browser: the worklet status block.
  - The browser already acks with `applied_at_sample = next_absolute_sample`
    (`hosts/host-web/src/lib.rs:3337`, `:3388`): exact there only because admission runs on the
    audio thread between two renders. The watermark gives both hosts one meaning.
  - Do **not** put the application sample in the reliable event lane: its capacity is 2
    (`crates/capi/src/runtime/compile.rs:98`) and an undrained lane would then refuse edits — a new
    backpressure source, against B2's intent.

### B4 (bounded tail for live input filters): ruling

**Ruling: the draft's B4 is right in spirit but wrong in scope and definition. A finite tail is
computable only against a relative floor, never against exact zero; and it must be one
engine-wide contract, not an input-section exception.** Evidence [inv-tail] (scratch crate linking
the real `lane` kernels and a copy of `SvfSection::design`):

- **Nothing reads the tail.** `TailSamples::{Finite(u64), Infinite}`
  (`crates/effect-contract/src/lib.rs:131-134`) propagates through PDC with `Infinite` winning
  (`crates/graph-compiler/src/pdc.rs:121-136`); no read in `crates/graph/src/runtime.rs` or
  host-web; silence elision uses exact state checks (`section_is_identity`,
  `crates/lane/src/kernels/builtins.rs:1146-1163`). It is a report.
- **Exact-zero tail is genuinely infinite at the top of the cutoff domain.** Where `c1` rounds to
  `1.0f32` (from 22,047.6 Hz at 44.1 kHz, 23,997.4 Hz at 48 kHz, 44,095.2 Hz at 88.2 kHz,
  47,994.7 Hz at 96 kHz, up to each maximum), the TPT SVF state locks into a period-2 cycle
  (`ic1 = ±1e-20…1.35e-16`, output ≈ ±2e-21 forever) created by the 1e-20 flush
  (`crates/lane/src/kernels.rs:643-651`, `crates/lane/src/lib.rs:177-188`). Elsewhere exact zero
  is finite but useless (up to 44.4 M samples; ~30 M at 10 Hz/96 kHz).
- **The time-varying case is bounded:** with zero input each step is the bilinear-transform matrix
  of a passive `M` (`M + Mᵀ ≤ 0`), linear in the coefficient words, so the 64-step ramp cannot
  grow the state norm (f32 excess at most 1 + 2.2e-6).
- **Relative floor, worst case over the reachable domain** (HPF one ulp below max into LPF at
  max; -144 dB re input peak): 383,571 samples at 44.1 kHz (8.70 s), 381,428 at 48 kHz, 383,571
  at 88.2 kHz, 381,428 at 96 kHz; plus 64 for a ramp in flight and a ≥1 % margin. Trim and
  polarity are memoryless: tail 0. The slowest pole is at the *maximum* cutoff (radius
  1 − 5.2e-5), not at 10 Hz.
- **The current rule is "IIR → Infinite"** (EQ `crates/parametric-eq/src/lib.rs:653`, compressor
  `crates/compressor/src/lib.rs:296`, multiband `:349`, limiter `crates/true-peak-limiter/src/lib.rs:243`,
  delay `crates/delay/src/lib.rs:264`), and it is already uneven: the compressor and limiter only
  multiply a delayed input, the argument the transient shaper used for `Finite(0)`. A finite input
  tail alone would contradict the EQ (same SVF family, longer worst case).

**Decision for the plan:**

1. **#1261/#1262 proceed now** with `Infinite` for strips with a live input lane (Q4 "yes"). It
   changes no rendering and matches the current contract; it is not a shortcut because nothing
   reads the tail, and item 2 replaces the definition for every node at once.
2. **New engine-wide issue: a tail contract.** Definition: the smallest `T` such that for every
   input with peak `P` ending at 0 and no control event after it (a ramp in flight allowed),
   `|y[n]| < P·10^(-144/20)` for all `n ≥ T` (-144 dB ≈ 2⁻²⁴, also the fader/trim floor; matches
   Web Audio's tail-time note for IIR nodes). Computed per rate from the designer (a recomputing
   test, never a digest), on the control thread. Apply to EQ, multiband, delay, input section;
   gain-only processors (compressor, limiter: a gain times a delayed input) report `Finite(0)`
   beyond their latency.
3. **New DSP defect issue (owner ruling, class B): the input filter's top-of-domain limit cycle.**
   A filter that never reaches its fixed point defeats the owner's skip-work-on-silence rule after
   any real audio. Candidate fixes change bits: cap the domain maximum below the `c1 = 1.0f32`
   band, or flush `ic1` when `ic2` flushes. Check the EQ's SVF sections for the same band.
4. **#1268's premise looks stale:** the kernel clears the integrators when a ramp completes onto
   identity (`crates/lane/src/kernels/builtins.rs:863-884`, `:979-981`). Run its D1 check first; it
   may close as not needed.

### B5 (#1306 window by lane depth): disagree as drafted

- #1306 itself says that when #1058 renders stored automation "that issue must size the capacity
  for those spans too, and pay the window again" (`1306-…md`, "What yes commits to"). That is a
  known later fix — exactly what the owner principle forbids.
- **Correct design, once:** size each effect's span window at preparation from the producers the
  plan actually has: `LIVE_QUEUE_DEPTH` if the effect has a live lane, plus the stored-automation
  span bound per block that preparation computes from the session's compiled automation for that
  effect (known at preparation, independent of song length if segments are compiled to bounded
  per-block spans). The caller's S then bounds only protocol `AUTOMATION_ENQUEUE` density.
- That needs #1058's design first. So: #1058 research now; #1306 rewritten to this sizing and
  implemented with the first #1058 slice. Until then nothing changes (the cost is a documented
  memory overcount, not a defect).

### B6 (#1225, #1226, #1247): agree

- Do F3 (`follows_mute` live) and F9 (VCA membership reason or live) inside them, not after:
  #1226 is the C ABI's `follows_mute` composition and #1247 D1 is the membership rule; rewriting
  them later is rework.
- #1247 D1: on the C ABI membership needs no new render memory (decision 14 F9). Make it live;
  "reach drift" is not a rule-3 reason.

## C. Seamless swap decisions

### C1 (carry state when layout is compatible, also when values changed): agree, with a limit

- "Carry, then retarget" is correct **for live values only**: carry the state, then emit the same
  ramped live records a value edit would emit (the classifier already computes them, D1 step 6 of
  #1053). The result is bit-identical to "live edit, then structural edit", which is the
  definition we want.
- **A changed prepared (rule-3) value cannot be retargeted**: by decision 14 rule 3 it is
  prepared *because* a live change glitches (limiter lookahead, `delay_samples`, link mode,
  quality). Carrying state with a stepped prepared value reproduces exactly that glitch. Those
  owners restart, and the strip needs the transition of C5 (below). The draft's "no restart at
  rest" must say this.
- P1.4's comparison base must be **the committed model the predecessor plan was prepared from plus
  every record pushed to it**, not "the model before this transaction". With candidate
  supersession (C5 below) the transaction before this one may never have rendered.

### C2 (latency growth): "history fill" is impossible; recommended design

**Why history fill cannot be seamless.** Let an unchanged path `y` reach a summing node with
compensation `c`. Before the swap the node emits `y[n - c]`. Growth of `Δ` elsewhere at that node
lengthens this edge to `c + Δ`, so after the swap the node emits `y[n - c - Δ]`. At the swap
sample `s` the stream goes `… y[s-1-c], y[s-c-Δ], …`: the last `Δ` samples repeat (if the line was
filled from history) or are zeros (front pad, #1285 gate 5). Neither is continuous. The only way
to emit `y[s-c]` next is for the graph to have computed `Δ` more samples of `y` than the host
clock — that is pre-roll. "History fill" turns a gap into a repeat; it is not a third option.

The same holds for filling a *new* latent effect's lookahead from history: the plan keeps no
sample history at an arbitrary insert boundary, and keeping one means a ring per insert boundary
per strip written every sample (memory and a render copy on every strip, against
`no-unnecessary-copies`).

**What each exact remedy costs.**

- **Global pre-roll** (#1287 as drafted): `k = ceil(Δ / quantum)` extra blocks of the *whole*
  graph in one callback. Limiter at 48 kHz/128: `k = 4` (5x block cost); at 96 kHz/128: latency
  `966`, `k = 8` (9x). On a loaded phone or browser this is the dropout the owner wants to avoid,
  and it is worst exactly when the session is heaviest. Spreading it needs the predecessor to keep
  rendering while the successor catches up: ≥2x for the catch-up window plus a source tee (P6's
  rejected costs).
- **Latency reserve, per strip** (fixed-latency console design): compile PDC as if each strip's
  insert section had latency `max(actual, R)`, where `R` is a session setting. Any insert edit on
  a strip that stays within `R` changes no arrival outside that strip: only the edited strip's
  own compensation edges shorten. Zero CPU spike, exact for every unchanged path, deterministic.
  Cost: `R` samples of output latency always, and a compensation line of `R - actual` on edges
  from post-insert taps of strips with less latency (the lines are the existing PDC lines,
  `crates/graph-compiler/src/pdc.rs:60-89`, longer; no new mechanism). For stem playback the
  latency is acceptable (`product-goal-browser-daw.md`: a few ms fader-to-ear is fine; no live
  input). A reserve applied only at the output does *not* work: growth inside a submix still moves
  the submix's other inputs.
- **Topology growth** (a route that adds a bus level, a latent effect on a submix that raises the
  output's max) is not covered by a per-strip reserve.

**Recommendation (for owner record):**

1. Floors for decreases (#1285), plus a rule to *drop* floors at the next discontinuity the host
   declares (stop, or a seek of every source), so latency does not ratchet up for the life of a
   session. Today #1285 D2 keeps floors "for the session's life".
2. A per-strip latency reserve `R` as a session setting (`controlSmoothing`-style: one schema
   default, researched), default sized to cover the launch latent effects at 96 kHz (true-peak
   limiter `Fs/100 + 6`, plus the next largest). It removes the common case (insert a limiter on a
   playing strip) from pre-roll entirely.
3. Bounded global pre-roll (#1287) for growth beyond `R` or from topology, with its spike measured
   by #1286 on the real paths (browser V8 and AVX2 rows); above the bound, the documented
   transition and a counter.
4. The edited strip's own discontinuity (its new effect starts at rest; its compensation edge
   shortened) is handled by C5's transition, so it is never a click.

### C3 (added strip ramps in from silence): agree

- #1288's design is right (trigger on first played block, not the swap). Length: the session's
  mute ramp (#1054), not a separate constant.
- Keyed on "strip ID absent from the predecessor", as #1288 D1 says; never on "not carried".

### C4 (browser preparation off the audio thread): ruling

**Ruling: Wasm threads with one shared `WebAssembly.Memory` — a Worker instance runs the control
plane (session model, classifier, preparation, retirement), the worklet instance only renders and
swaps through the existing lock-free exchange. This is the C ABI's shape, so it is also C6's
"one control plane". Time-slicing and artifact transfer are rejected.** Evidence [inv-browser]
(profiles and Playwright probes in Chromium 151, Firefox 153, WebKit 26.5; scratch only):

- **Cost.** #1289's boot p50 under V8: 9-track 2.69 ms, 64-track console 23.9 ms, app shape
  23.4 ms, sends 39.1 ms, against 2.667 ms per quantum (9x-15x); Chromium in a real worklet 47 ms
  warm, 141 ms cold. A phase profile of the current module: ~63-67 % is pure computation (parse
  19-22 %, graph topology 20-22 %, schedule/PDC 9 %, per-instance descriptor validation 8-9 %,
  identity sha256 3 %), ~30-33 % builds heap objects that must live in the rendering instance
  (bind/lower/executor 17-18 %, effect/builtin prepare 11-12 %).
- **Why not (b)/(d), a second instance shipping an artifact:** the 30-33 % that builds objects is
  still 7-13 ms (V8) on the audio thread, so it misses the budget without slicing too; and a
  transferable plan is either a new versioned compiled-plan contract with a second construction
  path, or a same-address arena image where one stray pointer corrupts memory on the audio thread.
- **Why not (c), time-slicing:** the worklet has no usable clock (`performance` undefined in all
  three engines; Chromium `Date.now` 1 ms), parse (4.5-8.5 ms) and graph compile (7-12 ms) are
  single calls that would have to become resumable state machines in shared core code (a
  browser-only constraint on the C ABI's code), `memory.grow` detaches views, a rebuild would take
  25-80+ quanta, and it extends audio-thread allocation into playback against `AGENTS.md:13`.
- **Isolation cost is already paid by the first-party app:** `misofm/app/public/_headers:18-20`
  sends COOP `same-origin` + COEP `require-corp` (WebKit lacks `credentialless`). Verified in all
  three engines: with isolation a Worker and a worklet instance share one memory both ways; a
  shared Memory reaches the worklet via `processorOptions` and the port; without isolation posting
  it is a `DataCloneError`, but a worklet can still create a local shared memory and run the same
  atomics module single-threaded — **one artifact serves both**.

**Requirements and risks (each becomes a gate or an owner question):**

1. **Toolchain (owner question).** Stable 1.97.1 refuses `-Zbuild-std`; `+atomics` on stable
   links against a non-atomic std and produces unshared memory. A pinned nightly
   (`nightly-2026-08-20`, used today only for fuzzing, `.github/workflows/fuzz.yml:99`) with
   `-Zbuild-std=std,panic_abort` and `--shared-memory --import-memory --max-memory` produced the
   right module shape. Shipping a nightly-built artifact is a new release policy, and browser
   bit-identity with AArch64 must be re-qualified.
2. **Allocator spin lock.** With atomics, std's Wasm dlmalloc takes a global spin lock
   (`library/std/src/sys/alloc/wasm.rs:61-160`). Any worklet allocation or free while the worker
   holds it is unbounded priority inversion. Hard rule: **the worklet never allocates or frees
   after boot**; disposal and plan retirement move to the worker (which is what `AGENTS.md`
   already wants). Today's allocation gate checks only three exports and only direct `call` edges
   (`scripts/check-web-audioworklet.sh:474-500`, `check-web-audioworklet-callgraph.py:294-313`);
   it must cover every worklet-reachable export including indirect calls before (a) ships.
3. **Glue the repo owns** (no wasm-bindgen): per-instance stack and TLS (`__wasm_init_tls`), the
   `thread_local!` staging in `hosts/host-web/src/ffi.rs:503-519` becomes per-instance, browser
   preparation switches from `prepare_*_between_render_calls` to the concurrent variant
   (`crates/host-core/src/prepare.rs:1007-1013`), and the worklet's seven
   `memory.buffer !== this.memoryBuffer` checks (e.g. `miso-engine-v1-audio-worklet.js:1823`) must
   accept growth by the worker instead of tripping a sticky `REPREPARE`.
4. **Policy gates** that ban imports, shared memory, atomics, `SharedArrayBuffer`, `Worker` and
   `memory.grow` (`check-web-audioworklet.sh:363-380`, `:509-520`) are rewritten to the new
   invariants, not deleted.
5. **Platform limits:** shared memory needs a declared maximum (budget default 512 MiB,
   `hosts/host-web/src/lib.rs:115`; two plans live while a replacement is pending); mobile Safari's
   reservation for a large maximum and iOS isolation on a real device are unverified
   (`docs/mixer/isolation-audit.md:295-297` in the app repo). The worklet cannot read
   `crossOriginIsolated`; the main thread passes it.
6. **Non-isolated pages (owner question).** Recommended: same API; playback and live edits work;
   a structural edit runs the blocking rebuild on the audio thread, reported as such through B3's
   path field and counted — never silent. The editing product requires isolation.

**Independent wins that help every option:** validate static effect descriptors once per effect
type, not per instance (~9 %), and stop comparing `GraphNodeId` strings in the compiler hot path
(11-12 % inclusive). File them as a bounded performance issue.

**First step is a spike, not a feature:** one issue that proves, in all three engines plus a real
iOS device, a two-instance engine (worker prepares, worklet renders) with the nightly build, the
allocation rule and growth handling, and re-runs AArch64/browser bit-identity. Its result gates
the browser stream.

### C5 (removed strip fades out, two-phase): agree, and generalize it

Two-phase removal needs four things the draft does not list:

1. **A scheduled swap**: "adopt no earlier than render sample `X`" on the candidate
   (`enter_block` checks it; one comparison). Phase 1 is the strip's live mute ramp to 0, phase 2
   is a swap scheduled at the ramp's end.
2. **Candidate supersession.** Today a second structural edit while a candidate is pending is
   refused with BACKPRESSURE (`crates/capi/src/runtime/control.rs:959` on `6fb211594`; publication and
   retirement capacity 1, `compile.rs:170-171`). A scheduled swap holds a candidate pending for
   the whole ramp, and a paused host never adopts one, so the user's next structural edit would be
   refused. Fix: publish through a single-slot mailbox the control thread can **CAS-replace**
   while the render thread has not yet taken it; the displaced unrendered candidate is dropped on
   the control thread; if the CAS fails (render took it), re-target the new candidate's carry
   program to the adopted plan's inventory and publish again. Render stays wait-free (one swap).
   This is the structural twin of B2 and belongs in the same workstream.
3. **Source lifetime.** A removed strip's source must keep feeding until phase 2 (the ramp reads
   it). P4 refuses submits for a removed source immediately; the ring's ~100 ms of queued PCM
   (`default_source_ring_frames`, `crates/host-core/src/prepare.rs:60-79`) covers a 10-20 ms ramp,
   but the rule must say the source retires with phase 2, not with the commit.
4. **Live strip lanes on every plan.** Phase 1 needs a live mute. The browser prepares live lanes
   only when the app opts in at boot (decision 14, "Each host on `main`"); fan playback has none.
   Under C6 every plan, on both hosts, prepares the strip fader/mute lanes (the C ABI already does,
   #1256). Measure the memory once and accept it.

**Generalization (same mechanism, no new primitive):** an edited strip whose state cannot
continue — a changed prepared value (C1), an insert added/removed/reordered, a quality or link
mode change, a `delay_samples` change, the shortened compensation edge of C2.4 — uses
**duck-swap**: live mute ramp down on the old plan, swap scheduled at ramp end, fade-in (C3
mechanism, keyed on "strip restarted") on the new plan. It costs no double render (P6 stands) and
reuses only C3 + C5 parts. A true crossfade (render old and new strip versions together) is
better audio but needs ghost strips in the successor and a cleanup rebuild; record it as the
reopening condition, not now.

### C6 (one public edit API on every host): agree, and it forces the order

- **C ABI**: done in shape (`miso_engine_v1_submit_command` + `SESSION_TRANSACTION_APPLY`,
  #1053). Add B3's path field.
- **Browser**: the same transaction semantics, the same classifier, the same committed model. That
  is #1057's question 1. The control plane today lives in `crates/capi/src/runtime/control.rs`
  (1,584 lines) and `compile.rs` (994 lines), adapter code; host-web has no `protocol` dependency.
  **Move the control plane into a portable crate** (session store + `classify_live_delta` +
  preparation + publication + supersession), and make capi and host-web thin adapters over it
  (`product-goal-browser-daw.md`: "The portable core … holds shared behaviour; the browser
  adapter … and the C ABI adapter stay thin").
- **Public API**: browser SDK `engine.apply(transaction)` (or `edit`) returning
  `{revision, path}` with the watermark of B3 for the application sample; C ABI unchanged.
  `replaceSession(document)` exists only as a convenience that diffs the document against the
  committed model into one transaction (so it is the same API, not a second one). No public
  `miso.replace.v1` message; the worklet protocol is internal.
- The SDK's live-control handles become thin builders of transactions (or stay as a
  low-latency path that commits to the same model; decide in #1057 design).
- Consequence: B3/B3b are deleted, B2/B4-B8 are rewritten on the transaction path, and #1057
  becomes a design-plus-implementation umbrella, not research.

## D. `miso_engine_v1_source_seek_at` (feature bit 32)

**Verdict [inv-seek]: the primitive is right long-term (anchored on a render-block sample,
lateness compensated, allocation-free, no new queue, typed refusals). The clock contract is not
yet right, and two of its gaps must close before launch freezes the header.** No blocker.

- **MAJOR D1-a — the anchor clock contradicts pre-roll.** The header pins the anchor to "the clock
  `miso_engine_v1_render_f32_planar` takes" (`crates/capi/include/miso_engine_v1.h:98-99`) and F to
  "the source frame the playing stems read at A" (`:104-105`). #1287's "Input clock" makes the
  graph read sources `P` ahead of that clock and asks hosts to add `P` themselves — a host written
  against today's header silently drifts by `P` after any pre-roll. **Fix:** define the anchor on
  the plan's **source-read clock** ("equal to the render clock in this version"); then a host that
  never learns `P` stays aligned (the seek simply applies at host block `A − P`). Amend #1287's
  input-clock bullet and gate 4. This also makes C2's reserve-versus-pre-roll choice invisible to
  hosts.
- **MAJOR D1-b — after a plain seek during playback the host cannot compute F.** A plain seek
  applies at the first block the render thread begins after the call; nothing reports which
  (`active_generation` flips at the call, `crates/source/src/lib.rs:857`, `crates/capi/src/ffi.rs:499`;
  no applied-seek event). Scrub with `seek`, then add a stem with `seek_at`: the stem is off by one
  or two quanta — comb filtering on correlated drum mics. **Fix (long-term):** make every seek
  anchored. A plain seek during playback is defined as "anchored at the next block the render
  begins", and that block is reported through B3's watermark (per source generation). The header
  states the precondition until then.
- **MINOR:** document held behaviour for a source that is already playing (queued PCM plays until
  A, then silence), replacement of a held seek, the one-seek-in-flight BACKPRESSURE string
  `source.seek.backpressure` (absent from the header), displacement by a transaction, far-past
  anchors past the region end, the clock origin (0, quantum multiples,
  `crates/engine/src/realtime/plan.rs:582,875`), and choosing A to cover decode-and-submit latency.
- **MINOR — ABI growth rule.** `docs/C_ABI_V1_QUALIFICATION.md:112-113` implies the feature bit
  protects directly linked hosts; it does not (link failure). Write the rule: `ABI_VERSION` changes
  only on a break; additions get a feature bit; hosts test bits singly and tolerate unknown bits;
  `FEATURE_MASK` is never compared with `==` (`abi_smoke.c:124` models that). Bit 32 and mask 63
  are correct; the in-place amendment matches #1063/#1206/#1243.
- **MINOR — "those blocks count as source underruns"** (`miso_engine_v1.h:106-107`): capi exposes no
  underrun counter, and in the browser a held block sets the session-wide `source_underrun` bit
  (`crates/source/src/lib.rs:1722`), so a planned wait looks like starvation. Report "held"
  separately in `SourceReadReport`; drop the clause.
- **MINOR — untested ack path.** No committed test holds a `seek_at` across a plan swap
  (`crates/host-core/tests/successor_swap.rs:635` asserts `seek_before >= SWAP_BLOCK`). A swap
  change that resets `held_seek` would drop an acknowledged seek. Commit the verifier's probe P5.
  With candidate supersession (C5.2), the held seek of an added source must also transfer from the
  displaced candidate's producer to the newer one: add that case.
- **MINOR — stale diagnostic** on malformed source IDs (`crates/capi/src/ffi.rs:593-601`): set
  `source.id.invalid`.
- **MINOR — #1293 D4 trap:** copying the browser's plain seek export would report every accepted
  future anchor as `RESULT_INTERNAL` (`hosts/host-web/src/lib.rs:3193-3203` vs
  `prepare_seek` returning `false` for a held seek, `crates/source/src/lib.rs:1043-1065`).
- **NITs:** off-grid lateness starves a source (`crates/source/src/lib.rs:1390`; add a debug assert
  or discard behind-position blocks); untimed `begin_block`/`read_block` ignore the anchor
  (test-only them); `audit capi` never calls `seek_at` (add one); note that adding lateness to F
  assumes source rate equals render rate. Naming is acceptable; do not rename.

## E. Decision-14 findings

- **E1 (F1): agree.** Put the check in a crate that sees both session and descriptors (the
  effect-compiler preparation, or the validator tool) and in the SDK's `resolveAutomationTarget`.
  Must land before #1058's first rendering slice.
- **E2 (F2): make all of them live; no rule-3 reason holds.**
  - Gate attack/hold/release: the compressor's are live with the same ballistics shape.
  - Parametric EQ `enabled` and `kind`: the EQ is a cascade of Simper TPT SVF sections whose
    output mix is `m0, m1, m2` (`crates/parametric-eq/src/lib.rs:1-18`, `:678-706`); kind and
    enable are just other `m`/`k` words on the same state, so they ride the existing
    prepared-target ramp (the HPF/LPF `enabled` already does).
  - Multiband crossover: the LR4 is two TPT sections designed per frequency
    (`crates/multiband-compressor/src/lib.rs:510-570`); a ramp that moves both bands' shared
    coefficients together keeps the complementary sum per sample. Needs a test that the band sum
    stays allpass-continuous during a sweep.
  - Each change alters the effect's live-parameter set, its carry payload (#1279-#1282) and its
    bank contract; schedule each after the carry slice that touches the same crate.
- **E3 (F3): agree**, inside #1226 (C ABI) and as one browser slice.
- **E4 (F4): agree**, and extend it:
  - Owner question: may a **correctness** reason keep a value prepared? Recommend yes, recorded
    with its reopening condition, as for glitch and optimisation.
  - For the delay and the multiband compressor bypass: give them the live shunt (MBC after #1069's
    per-lane D7 recovery; the delay needs an owner). Until then a lift of their bypass is a
    rebuild with the C5 transition, never an acked no-op.
  - **`AUTOMATION_ENQUEUE`** is admitted and acked for `Block`-rate parameters and reaches no PCM on
    any host (decision 14, "Each host", last bullet; #349 IO-5). That is the acked-batch question
    answered "yes". Refuse it typed now; wire it with #1058.
- **E5 (F7): agree.** Crossfade the bypass switch over the session's ramp: dry and wet are both
  present every block (`crates/effect-contract/src/live.rs:820-842`, the wet path keeps running),
  so the crossfade is a per-sample mix with no new state beyond a ramp position. Bit-identical
  after the ramp. It touches `live.rs`, which #1280 (inherited queue) also edits: sequence after
  #1280.
- **E6 (F9): make live** (see B6).

## F. Reliability follow-ups

Agree to do all, with these notes:

- **#1236** (per-strip link mode) changes P1.3's layout key and the compressor bank contract;
  schedule after #1282, and make the per-strip change live (decision 14's note) or give the
  optimisation reason with a measurement.
- **#1237** (route bounds) touches session validation and route lanes; independent of the carry
  chain; can run early.
- **#1300** (soft-clip non-finite history) is code-independent of #1279 (its own spec, line 253),
  but without it #1279 D4's fallback restarts such a soft-clip lane at a swap, which is a
  glitch. Under the owner principle land it before #1279 is declared done.
- **#1301, #1302, #1251, #1248, #1232, #1234, #1235, #1303, #1304** are tooling and test hygiene;
  file-disjoint from the feature chains; run them in a parallel hygiene stream.
- **#1305** (linear scans) touches `commit_live`; put it in the control-plane stream.
- **#1268** (elide a settled input filter): run its D1 check first; its premise looks stale (B4).

## Missing items

1. **#1058 stored-automation rendering** (owner ruling 2026-09-28). Dependencies: E1, #1306,
   `classify_live_delta`'s automation mask (`live_delta.rs:234-236`), `AUTOMATION_ENQUEUE`, and
   #1226's follow-mute question. Run the research now.
2. **#1057** becomes the browser control-plane umbrella (C6).
3. **Candidate supersession** for structural edits (C5.2).
4. **Floors reset** at a host-declared discontinuity (C2.1).
5. **Live strip lanes on every browser plan** (C5.4).
6. **Meters/observation carry** (A1).
7. **Skip-on-silence hazard** (#1053 D11): every future silence skip must still run bounded
   drains, ramps and scheduled swaps; put it in the #1107 brief.
8. **`delay_samples`**: decision 14 keeps it a rebuild; with duck-swap the change becomes
   click-free without new memory. Record that as its transition, not as "live".
9. **Tail contract** (engine-wide, B4.2) and the **top-of-domain limit cycle** (B4.3).
10. **Preparation performance** (C4): descriptor validation once per type, no `GraphNodeId` string
    compares in the compiler hot path. Helps both hosts' rebuild latency.
11. **Seek contract** (D1-a, D1-b): source-read clock anchor; every seek anchored and reported.
12. **Owner Q5 of decision 13** (keep bus trim/polarity?) is still open; it blocks nothing here but
   should be asked in the same owner round.

## Owner questions this plan needs (one round)

- **OQ1 (C2).** Per-strip latency reserve `R` as a session setting with a researched default
  (always-paid output latency), plus bounded pre-roll beyond it. Alternative: pre-roll only.
- **OQ2 (C5).** Duck-swap (fade out, swap, fade in) for edited strips whose state cannot continue;
  true crossfade (ghost strips) deferred with a reopening condition.
- **OQ3 (C4).** Ship the browser artifact from a pinned nightly with `-Zbuild-std` (shared memory),
  re-qualifying bit-identity; require cross-origin isolation for structural edits; non-isolated
  pages rebuild blocking, reported and counted.
- **OQ4 (E4).** A correctness reason may keep a value prepared (recorded with its reopening
  condition).
- **OQ5 (B4.3).** Class-B fix of the input filter's top-of-domain limit cycle (bits move).
- **OQ6 (C2.1).** Floors drop at a host-declared discontinuity (stop / global seek).
- **OQ7 (WIP).** `AGENTS.md` allows one launch-critical implementation at a time. The streams
  below need two (S1 and S2) to finish in reasonable time; ask the owner to amend the rule to "one
  per disjoint file ownership", or run them serially.
- Pending from decision 13: Q5 (bus trim/polarity).

## Workstreams

Each stream = one opus-high coordinator with opus-medium workers in their own worktrees and fresh
opus-xhigh verifiers. "Files" is the ownership that keeps streams conflict-free; a stream may not
edit another stream's files without a hand-off.

**S0. Decisions and specs (root, before any new code).** Record the owner principle and the rulings
above as decision 15 (amending 14's Q3/Q4 answers, #1269's P4/P7/P10/P12 and Q1-Q6, #1053's Q2-Q4).
Rewrite #1287, #1288, #1290-#1297, #1306; file the new issues (cells, path+watermark, candidate
supersession, scheduled swap, duck-swap, removal, meter carry, floors reset, reserve, control-plane
extraction, browser spike, tail contract, limit cycle, seek contract, prep performance, live strip
lanes on every browser plan). Issue-first: no stream starts a slice without its spec. In-flight
slices (#1277 onward) continue meanwhile.

**S1. Seamless swap, C ABI core (launch-critical, serial).** #1300 → #1277 → #1279 → #1280 → #1281 →
#1282 → #1283 → #1284 → #1285 (+ floors reset) → #1286 → reserve → #1287 (rewritten) → meter carry.
Files: `crates/builtins*`, `crates/rack`, `crates/graph`, `crates/graph-compiler`, effect crates'
payload code, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`.

**S2. Control plane and protocol (launch-critical; parallel with S1 only under OQ7).**
(1) extract capi's control plane (`crates/capi/src/runtime/control.rs`, `compile.rs`) into a
portable crate both adapters use — done first so everything after it lands once for both hosts;
(2) candidate supersession + scheduled swap (`crates/engine/src/realtime/plan_exchange.rs`);
(3) B2 latest-target cells on both hosts (builtins-compiler lanes, `effect-contract/src/live.rs`
after #1280 lands, host-web admission); (4) B3 path field + watermark; (5) E4 typed refusals incl.
`AUTOMATION_ENQUEUE`; (6) #1305; (7) D1 seek contract and header text. Files: the new crate,
`crates/capi`, `crates/protocol`, `crates/engine/src/realtime`, `crates/host-core/src/live_delta.rs`.
Hidden dependency: (3) and S1's #1280 both edit `live.rs`; #1280 first.

**S3. C ABI live completeness (after S2(1)-(3)).** #1261, #1262 (tail `Infinite`), #1225, #1226
(with F3), #1247 (membership live, F9), #1267. Built as cells from the start. Files: shared
control-plane crate's classifier, host-core lane attachment.

**S4. Browser one control plane.** (a) spike now (no product code; scratch + a prototype branch):
two-instance shared-memory engine in Chromium, Firefox, WebKit and a real iOS device, nightly
build, allocation rule, growth handling, AArch64/browser bit-identity. (b) after S2(1) and the
spike: worker control plane, worklet render-only, allocation gate over every worklet-reachable
export including indirect calls, policy scripts rewritten. (c) B2 (#1290) reshaped, B4-B8
rewritten on the transaction API, B3/B3b deleted, live strip lanes on every plan, SDK
`engine.apply`. Files: `hosts/host-web`, `sdk/`, `scripts/check-web-audioworklet*`.

**S5. Smoothness.** #1055 research now (docs only) → #1054 (session schema, `LiveRamps`, SDK
defaults; every live row incl. trim, sends, VCA) → E5 bypass crossfade (after #1280) → C3 fade-in
(#1288 rewritten) and C5 duck-swap/removal (after S2(2) scheduled swap and S1 #1277). Files:
`crates/session`, `crates/host-core/src/*ramps*`, `sdk/src/core/live-controls.ts`,
`effect-contract/src/live.rs` (bypass, sequenced).

**S6. Effect liveness.** E1 validator + SDK check (now; effect-compiler/tools and
`sdk/src/core/session.ts`); E2 gate/EQ/multiband live, F4 delay/multiband live shunt (after
#1069), #1236 per-strip link mode — each after the S1 carry slice that touches the same effect
crate (#1279/#1282).

**S7. Hygiene and contracts (bounded, file-disjoint, any time).** #1232, #1234, #1235, #1237,
#1248, #1251, #1301, #1302, #1304; #1303 after S1's #1277 (both edit `crates/builtins`); #1268 D1
check; tail contract; limit-cycle fix after OQ5; prep performance (graph-compiler; coordinate with
S1's #1285).

**S8. Research (docs only, start now).** #1058 (feeds #1306, E1, the classifier's automation mask,
`AUTOMATION_ENQUEUE`); #1057 becomes S4's design note (personal-mix owner question first).

**Order on the critical path:** S0 → {S1 continuing, S2(1)} → S2(2)-(3) → {S3, S4(b), S5's
duck-swap} → S4(c). Start now in parallel without conflicts: S1's next slice, S4(a) spike, S5's
#1055, S6's E1, S7, S8.
