# Separate the worklet AudioContext clock stall on Playwright WebKit, Linux, GStreamer and a PulseAudio null sink

This is a diagnosis slice. It comes from the stream B batch-3 B2 diagnosis (#1345, "B2 diagnosis
and fix", the paragraph "What the click does not fix") and from root's stall-signature ruling of
2026-10-12. It changes no engine, SDK, host or gate code. Its one code deliverable is a minimal
repro with no engine code. Anchors were verified on `codex/d15-stream-b3` at `ebe706208`, in
`/tmp/claude-1002/b2diag/` and in `/tmp/claude-1002/b2r2/` on 2026-10-12.

## Product outcome

We know which layer stops a running WebKit `AudioContext` that has an `AudioWorkletNode`: our
render code, WebKit's AudioWorklet thread, WebKit's audio destination, GStreamer's audio sink, or
the PulseAudio null sink. The browser qualification then has a committed minimal repro that shows
the same stall signature without engine code. Root and the owner can then rule on the B3 gate's
exemption, on a CI change and on an upstream report from facts.

Root note (2026-10-13): the real-browser observation-unsubscribe gate was removed by #1345 Amendment 7 and moves to #1535. "The B3 gate" below means #1535's `observation-unsubscribe` gate.

## Context

- **The stack.** The CI browser leg (`.github/workflows/qualification.yml:376-445`) has these parts:
  - ubuntu-24.04;
  - Playwright 1.62.1 (`hosts/host-web/qualification/node_modules/playwright`);
  - WebKit 26.5 headless (Playwright revision 2336, `minibrowser-wpe`);
  - the system GStreamer (1.24.2 on the development box, `gstreamer1.0-plugins-{base,good,bad}`);
  - a private PulseAudio 16.1 server with `module-null-sink rate=48000 channels=2` (`:428-444`).

  The development box runs Ubuntu 24.04 and has 32 CPUs.
- **How the status reply is made.** The shipped processor answers `miso.status.v1` in its own port
  handler (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1076-1084`). It reads
  `renderedQuanta` and `lastResult` from the status view (`:944-961`). The port handler runs in the
  same JS agent as `process()`, so no reply can be made while `process()` is on the stack. A reply
  shows that the worklet agent is between calls and responsive.
- **Root's stall signature.** A stall has the signature only when the gate records all four items:
  - (i) `context.state === "running"`;
  - (ii) `currentTime` and `renderedQuanta` are equal at the resume and at the timeout;
  - (iii) at least 2 status replies are answered inside the window;
  - (iv) `lastResult === 0`.

  A failure without the full signature is ours and is never exempt.
- **An example with the full signature, with no load.** `/tmp/claude-1002/b2r2/logs/t1-webkit.log`
  (load average 1.2 at the start of the run, `b2r2/summary.txt`) shows this record:
  - the stall message is `stall timeout at the replayed region to render after 10000 ms`;
  - at the resume: `contextState: "running"`, `currentTime: 0.021333`, `renderedQuanta: "8"`,
    `lastResult: 0`, status `requestId` 39;
  - at the timeout: the same `running`, `0.021333`, `"8"` and `0`, `requestId` 2282.

  So the agent answered status requests 39 to 2282 for 10 s while it rendered nothing. This stall
  also occurs with no CPU load.
- **The earlier repro (`/tmp/claude-1002/b2diag/repro/repro.mjs`, `run.sh`).** It has no engine
  code. It creates fresh 48 kHz `AudioContext`s, each with a trivial `AudioWorkletNode` whose
  `process()` fills 0.001. It resumes each context twice through trusted clicks, with a suspend
  between the two resumes, and waits for `currentTime` to advance.
  - With 32 busy loops on 32 CPUs, 2 of 3 runs of up to 40 contexts stalled (one stall at context
    12, one at context 17). `resume()` settled and `state` was `running`, but `currentTime` stayed
    at 0.04 for more than 10 s.
  - The same loop with an `OscillatorNode` and no worklet ran 120 contexts in 3 runs with no stall
    (`drv4.out`).
  - A script `suspend()` followed by a click `resume()` hung the page in 3 of 8 runs (`drv5.out`).
  - The repro did not check whether the worklet still answered messages. It was never measured
    with no load.
- **What is known not to be the cause.** The B2 activation cause (WebKit resumes a context only
  inside a user activation) is fixed by the trusted click, and the stall comes after a settled
  `resume()`. The SDK closes take 0 to 8 ms (#1345 B2 record (a) and (e)).
- **The cross-origin state.** The qualification server sends no COOP/COEP headers
  (`hosts/host-web/qualification/server.mjs`). This slice does not change that.

## Decisions frozen for this slice

- **D1. One signature, and every other result is ours.** The repro classifies each failure with
  root's signature, in the repro's own terms:
  - (i) `state === "running"`;
  - (ii) `currentTime` and the processor's `currentFrame` (the `AudioWorkletGlobalScope` built-in,
    sent in its port reply) are equal at the resume and at the timeout;
  - (iii) at least 2 port pings are answered inside the window;
  - (iv) the repro has no `lastResult`, so no `processorerror` event fired.

  The classes are:
  - **S** (the full signature): a candidate external stall.
  - **N1**: fewer than 2 replies. The agent is not responsive (it could be stuck in `process()`
    or frozen).
  - **N2**: `processorerror` fired.
  - **N3**: `currentTime` and `currentFrame` disagree (one is frozen and the other moves).
  - **N4**: `state` is not `running`.

  N1 to N4 are never classed as a WebKit stall. The repro has no new counters, no
  SharedArrayBuffer, no cross-origin isolation and no wrapper around the shipped processor.
- **D2. The repro.** The repro is a trivial processor. Its `process()` fills 0.001 and returns
  `true`. Its port handler answers `{ ping, currentFrame }`. One run is one browser launch, one
  page and one 48 kHz `AudioContext` with that node. The run resumes the context in a trusted
  click, waits for 0.2 s of clock, suspends it, resumes it again in a second trusted click and
  waits for 0.2 s more. The window for each resume is 10 s. The repro sends a ping every 100 ms.
  An outer bound records a hung page as `HANG` with its last step. The repro never waits on a hung
  page. It runs under the same `run.sh` private PulseAudio null sink as CI.
- **D3. The cells.** Each cell changes one item from C1:
  - C1: the worklet node, no load, at least 60 runs (the baseline that root asked for).
  - C2: an oscillator only, no load, 60 runs (the control). The signature without (iii) and with
    `currentTime` alone.
  - C3: the worklet node and an oscillator in one context, no load, 60 runs. This shows whether
    the oscillator's output also stops when the worklet stalls.
  - C4: the worklet node with no PulseAudio server, and GStreamer's audio sink set to a
    non-PulseAudio sink that keeps clock time (for example, `GST_PLUGIN_FEATURE_RANK` set to
    prefer `fakeaudiosink`), 60 runs. First record which sink WebKit's audio destination uses in
    C1 (from the `GST_DEBUG` output). If WebKit does not let the sink be chosen without a rebuild,
    record that and skip C4.
  - C5: the worklet node with 32 busy loops on 32 CPUs (`/tmp/claude-1002/b2diag/load.sh start
    32`), 30 runs.
  - C6: the worklet node, with the browser and 4 busy loops pinned to CPUs 0-3 (`taskset -c 0-3`),
    30 runs.
  - C7: the same page in Chromium and Firefox, no load, 20 runs each.
- **D4. Thread states at each S.** When a run classifies S, the driver records `comm`, state and
  `wchan` from `/proc` for every thread of every WebKit process. It also keeps the `GST_DEBUG=2`
  output and `pulse.log`. Without these records, an S does not name a layer.
- **D5. Self-test of the classifier before the data.** On a quiet box, each planted condition
  must give its class:
  - P1: a script `ctx.suspend()` that the driver is not told about gives N4.
  - P2: a busy loop of 15 s inside `process()` after 200 calls gives N1, because the agent cannot
    answer while `process()` is on the stack.
  - P3: a port handler that ignores pings gives N1.
  - P4: a `process()` that throws after 200 calls gives N2.

  S has no plant: no page action stops WebKit's pull while the agent still answers. S is the class
  that remains when no planted condition holds.
- **D6. No upstream report.** Do not file a WebKit, GStreamer, PulseAudio or Playwright issue. If
  the layer is outside our code, the Attempt record holds a draft report (the stack, the versions,
  the repro and the counts) for the owner. Filing needs the owner's consent.
- **D7. No fix and no gate change here.** The result goes to root. Root rules on (a') and on any
  CI or gate change.

## Deliverables

1. The repro and its driver (D1, D2, D4 and D5), committed at
   `hosts/host-web/qualification/webkit-clock-stall/` (`repro.mjs`, `run.sh` and a short
   `README` with the command). The repro belongs to this slice, not to B3. One command then
   performs the (a') check "the minimal repro shows the same signature in the same session".
2. The Attempt record:
   - the self-test (D5);
   - a table of the cells (runs, and failures per class S and N1 to N4);
   - the thread states and the sink name for at least one S;
   - a statement of the layer, or "not separated" with what remains open.
3. If the layer is outside our code, the draft report of D6, in the Attempt record.
4. A short note to root on what the result means for (a') (see "How (a') depends on this").

## Authorized paths

- `hosts/host-web/qualification/webkit-clock-stall/` (new)
- This spec.
- Nothing else. Logs and scratch trees stay outside the repository.

## Non-goals

- No change to `qualification.js`, `run.mjs`, `server.mjs`, the SDK, the shipped worklet or
  the engine.
- No counters, wrappers or cross-origin isolation (D1).
- No retry, kick or longer wait in any gate.
- No upstream filing (D6).
- No change to CI's audio setup.
- No diagnosis of the spectrum gate failures (#1522).

## Objective gates

1. **Self-test of the classifier.** P1, P2, P3 and P4 give N4, N1, N1 and N2.
   *Test value:* this gate turns red if the classifier labels a hang inside `process()`, a mute
   port or a processor error as S. That error would let (a') exempt a fault of our own render as a
   browser stall.
2. **Baseline measured.** C1 has at least 60 runs, each classified. The record states the count
   of S. Zero S in 60 runs is a result, not a failure. The record then says that the trivial page
   does not show the stall with no load, and that the t1 stall needed something more.
   *Test value:* without this count, a later cell's "no stall" cannot be compared with anything.
3. **Every cell run and counted.** C1 to C7 each have a run count and the failures per class, with
   the load average before and after each run.
4. **Layer statement.** The record names the layer and the cells that separate it, or it says
   "not separated" and lists the cells that disagree. No claim goes beyond the counts.
5. **The committed repro runs.** `hosts/host-web/qualification/webkit-clock-stall/run.sh 5` on a
   quiet box finishes, classifies every run and prints its JSON summary.
6. `bash scripts/check-workspace-policy.sh` passes.

## How (a') depends on this

Under root's rule (a'), a WebKit leg-i failure of the B3 gate is exempt only if all three
conditions hold:

1. the failure has the full signature (i) to (iv);
2. the minimal repro shows the same signature in the same session;
3. every exempt failure is listed.

Root note (2026-10-13): the real-browser observation-unsubscribe gate was removed by #1345 Amendment 7 and moves to #1535. "The B3 gate" means #1535's `observation-unsubscribe` gate; #1535 R3 routes a full-signature stall there to this issue.

This slice affects the rule as follows:

- **Condition (2) needs this repro.** The B2 repro checks only `currentTime`. It cannot show (iii)
  (the worklet answers) or a frozen `currentFrame`, so it cannot show "the same signature". Until
  deliverable 1 lands, condition (2) cannot be met and no failure is exempt.
- **If C1 to C6 show S and the layer is outside our code** (WebKit, GStreamer or PulseAudio):
  (a') stands for a known external fault. Root and the owner decide on a CI change (for example
  C4's sink, if it removes S) or an upstream report (D6).
- **If S never appears without the engine worklet** (C1, C5 and C6 show none, while the gate shows
  S): condition (2) cannot be met. Each such gate failure is ours, and root rebriefs the cause
  into our worklet or host.
- **N1 to N4 in the gate are never exempt,** whatever this slice finds (root's ruling).

## Dependencies

- None to start: the repro has no engine code.
- Related: #1522. Its WebKit H1024 failures may share this stall (see #1522 Amendment 1).

## Placement

Recommended: **stream H**. The new directory is under `hosts/host-web`, which belongs to stream H
(`STREAMS.md:472`). The stall is in the browser host layer, and stream H owns the qualification.
The slice is independent of stream H's queue and can start at once. It touches no hot file.

## Attempt record

*(empty: the implementer fills it in)*

## Amendment 1 (root, 2026-10-12)

Corrected after a fresh verifier's verdict (`/home/bl/misofm/submix-verdicts/1531-diagnosis-verdict.md`) and extended with the step-1 result (root ruling 5). Evidence is in `/home/bl/misofm/submix-verdicts/1531-evidence/` (below: `E/`).

## What changed since the spec

1. **The trivial repro does not stall unloaded.** `/tmp/claude-1002/b3repro/` (copy:
   `E/b3repro-summary.txt`; scripts `E/one.mjs`, `E/run1.sh`, `E/sets.sh`) ran 150 fresh-browser
   runs (A plain, B with an SDK-like preface, C with a status poll like the gate's): 150 of 150
   passed. No run showed the full signature. So the spec's C1 baseline (60 runs, worklet, no load)
   is expected to show zero S, as gate 2 allows. The t1 stall needed something more.
2. **The hypotheses now include the first-segment length and our worklet host.** The two recorded
   stalls (`b2r2/logs/t1-webkit.log`, `b3sig/full-signature-webkit.log`) both came after a first
   segment of only 7 to 8 quanta (about 20 ms) before `suspend()`. The trivial repro did not vary
   that timing, and it did not use the shipped worklet host or the SDK phase.

## The one-time variant set (root, 2026-10-12; run once, no retry)

Protocol:

- WebKit 26.5 (Playwright 1.62.1), a fresh private PulseAudio null sink (`-vvvv` log) and a fresh
  browser per run. 540 runs in one shuffled order (`E/schedule.txt`; the verifier reproduced it
  from the seed in its first line).
- "Unloaded" means only the start rule: the 1-minute load average was below 4 at the start of
  every run (24 runs waited; largest start value 3.99). The box was not idle: 23 runs ended with a
  1-minute load of 4 or more, and V2 stall 095 ended at 38.9. Stalls do not follow load (median
  end load 2.53 in V2 stalls, 2.81 in V2 passes). A strict subset (end load below 4 and start
  5-minute load below 8) gives the same conclusions (verdict section 1).
- `GST_DEBUG=pulsesink:5,webkit*:4` plus `pulse:5,audiobasesink:5` (the `pulsesink` category
  matches nothing in this build; the sink logs under `pulse`).
- Tree: `374d347c2` exported read-only. Artifacts: `E/art/`. Its JavaScript is byte-equal to the
  tree. Its wasm (sha256 `e60b505c…`) was built at 09:07, after the last `crates/` commit
  (`ee6a8cc5b`, 09:02). The step-1 rebuild of `374d347c2` gives a byte-identical artifact set
  (below), so `art/` is the `374d347c2` artifact set. It is not the committed release pin
  (`6c952a2c…`, 2026-09-28).
- No D4 thread states (`/proc` `comm`/state/`wchan`) and no backtrace were taken at any S or at
  the run-494 hang. Under D4, no S here names a layer.

| Variant | What runs | k/60 full-signature stalls | one-sided 95% upper bound |
|---|---|---|---|
| V0 | the unmodified CI command (SDK source-bundle mode, `--check-matrix --self-test-mutations`) | 13 | 32.2% |
| V1 | shipped host and wasm, the gate's unsubscribe sequence, no SDK phase | 0 | 4.9% |
| V2 | V1 after the real SDK phase | 19 (+1 page hang, run 494) | 42.9% |
| V4 | trivial processor after the real SDK phase, no host (first segment ended at 7 process() calls) | 0 | 4.9% |
| V5 q2, q4, q7, q16 | trivial processor, no SDK, first segment suspended after 2, 4, 7, 16 process() calls | 0 each | 4.9% each |
| V5 t200 | same, first segment suspended 200 ms after the resume | 0 | 4.9% |
| V3 (meters off) | not run: V1 did not stall | -- | -- |

The bounds are exact one-sided Clopper-Pearson bounds. A rate below a bound is not excluded:
0 of 60 is consistent with a true rate of up to about 4.9%.

V0 differs from the diagnostic page in its context lifecycle: no V0 context starts a destination at
creation (60 of 60 runs). V0 records only the gate's own stall record, the GStreamer log and the
PulseAudio log. It has no page-side lifecycle events: no per-suspend counts, no resume or suspend
settle times, no intervals, no `baseLatency` or `outputLatency`.

Signature: all 32 stalls (13 in V0, 19 in V2) have the full signature (i) to (iv): context
`running` at the resume and at the timeout, `currentTime` and `renderedQuanta` equal at both, 39 to
40 status replies answered in the window, `lastResult` 0. All 32 are at the second resume ("the
replayed region to render"). There is no partial-signature stall.

One other failure, V2 run 494 (`E/runs/494-V2/`): 3 of 4 clicks returned. The 4th (replay) click
was dispatched and its `resume()` began to start the destination: the gate sink (`autoaudiosink2`)
began a third cycle at GStreamer t=4.180 s, and PulseAudio created stream 13 at the same instant.
Then the whole WebProcess stopped: its GStreamer log ends inside that cycle at `activate
ringbuffer`, stream 13 was never uncorked (freed at 317 s, when the run was killed), and the final
`page.evaluate` got no answer (`events: null`). This is a second failure mode at the same step as
the 19 stalls. It has no signature, so it is not exempt.

## Findings

- **The SDK phase is needed.** V2 19 of 60 against V1 0 of 60 (one-sided Fisher exact
  p = 3.5e-7). V0 13 of 60 against V1 (p = 5.9e-5). V2 and V0 do not differ (two-sided p = 0.30).
  This is the only clean component finding.
- **V4 is not a processor-only swap of V2, so the host's role is unproven.** V4 has the same SDK
  phase, click plan and context count. But it has no host, no wasm, no PCM submits and no meter
  lease (the shipped `process()` posts meter frames from the render thread,
  `E/art/miso-engine-v1-audio-worklet.js:1869`), and it has a different timing envelope: gate-context
  creation to the first resume takes median 159 ms in V4 against 710 ms in V2, and the pre-boot
  destination cycle in GStreamer lasts about 1 ms in V4 against 79 to 197 ms (median 160) in V2.
  V2 19/60 against V4 0/60 (p = 3.5e-7) shows that something in V2's package (engine, host,
  message traffic, timing) is needed against V4's. It does not show which element. "Needs the
  shipped host" and "not the trivial processor" are unproven.
- **The visible fault: the second `resume()` starts no destination.** In every stall the replay
  `resume()` produces no new gate-sink cycle (no "ringbuffer may start now", no state change) and
  PulseAudio creates no stream. In GStreamer the gate sink has 2 cycles in a V2 stall against 3 in a
  V2 pass, and 1 cycle in a V0 stall against 2 in a V0 pass (`E/cycles.json`; 19/19, 40/40, 13/13,
  47/47 runs). PulseAudio sees one stream and one uncork fewer than in a pass of the same variant
  (V0 3 streams against 4; V2 6 against 7). In the 19 V2 stalls the second `resume()` settled in
  0 to 1 ms with state `running`; V2 passes take 3 to 7 ms, V1 passes 3 to 12 ms. V0 records no
  settle times.
- **The gate's first segment never reaches a started sink.** In no V2 run, stall or pass, did the
  gate sink start its ringbuffer or uncork in the first segment. The page suspends 6 to 12 ms after
  the first resume, before the PulseAudio stream starts. The same holds in 339 non-stalling runs
  (V1 59, V2 passes 40, V4 60, V5-q2/q4/q7 180) and in 46 of 47 V0 passes. So this condition is
  present in every stall but it is not sufficient. The 2 uncorks counted in a V2 stall are the two
  SDK contexts'. V5-q16, V5-t200 and one V1 run started the sink in the first segment; none of
  those 121 runs stalled.
- **Within V2, an earlier first suspend goes with the stall.** `renderedQuanta` in the status reply
  taken at the moment of the first `suspend()` call: 16 of 25 runs at 6 quanta or fewer stalled,
  3 of 34 at 7 or more (two-sided Fisher p = 1.1e-5). By value: 4 of 4 at 4 or 5, 12 of 21 at 6,
  3 of 24 at 7, 0 of 10 at 8. The count once the suspend took effect is 1 to 4 higher. V4 and V5
  suspend at a similar clock point (6 to 7 process() calls) and do not stall. So the length
  matters inside V2 but does not decide alone.
- **A V2-only pre-boot fact, not necessary for the stall.** In V2 the page's pre-boot `suspend()`
  settles in 6 to 17 ms (median 8), but the GStreamer pipeline stops about 150 ms later (pre-boot
  cycle 79 to 197 ms, median 160), in passes and in stalls. In V4 the cycle lasts about 1 ms, in
  V1 4 to 12 ms. V0 starts no destination at context creation, and V0 still stalls. So this delayed
  stop is not necessary for the stall.
- `ctx.baseLatency` was 0.002667 s and `ctx.outputLatency` 0 in every V1, V2, V4 and V5 run, stalled
  or not. V0 records neither.

## Step 1: the stall is not a B3 regression (root ruling 5)

`E/step1/` (`results-table.txt`, `artifact-digests.txt`, `schedule.txt`, `runs/NNN-V2-{main,b3}/`):
V2 only, 120 runs in one shuffled order (seed 5387233579789162608, first line of `schedule.txt`),
the same start rule (largest start load 3.95; 12 runs waited; 11 runs ended at a load of 4 or more,
the largest 61.1).

| Side | Artifacts | full-signature stalls | one-sided 95% upper bound | other |
|---|---|---|---|---|
| V2-main | main `6cc1f8bc8` | 19/60 | 42.9% | 2 page hangs (runs 042, 079) |
| V2-b3 | B3 `374d347c2` | 22/60 | 48.1% | 0 |

- Fisher exact, two-sided p = 0.70.
- Both sides ran B3's gate page: `qualification.js` of `374d347c2` plus the one diagnostic export
  line, and the same `diag.js`. Each side used its own host JavaScript, wasm, SDK source and SDK
  entry (`sdk-response-entry.ts`).
- The rebuilt B3 artifact set is byte-identical to the #1531 set's `art/` (wasm `e60b505c…`,
  closure `3c917be8…`). Main's wasm is `ba702dce…` (closure `f3a6c0a1…`).
- Runs 042 and 079 have the run-494 pattern: 3 of 4 clicks returned, the WebProcess's GStreamer
  log ends in the gate sink's third cycle at `activate ringbuffer`, PulseAudio stream 13 was
  created and never uncorked, and no page events came back.
- No D4 thread states and no backtrace were taken at these hangs either.

So the stall and the hang are pre-existing with main's engine. They are not a B3 regression.

## What this means for (a')

Under #1531's rule ("How (a') depends on this"), S has appeared only with the engine worklet:
0 of 510 trivial-processor runs stalled (360 here, 150 in `b3repro`). Condition (2) cannot be met.
On the current data no B3 `observation-unsubscribe` stall is exempt. Each such gate failure is ours
until a no-engine variant reproduces S. The visible fault is in WebKit, but the data cannot say
"outside our code".

Root note (2026-10-13): the real-browser observation-unsubscribe gate was removed by #1345 Amendment 7 and moves to #1535. The finding applies to #1535's gate.

Note: `renderedQuanta` that matches the clock proves only that `process()` was not called. It does
not clear our code: our render-thread posts may trigger a WebKit race.

## Product risk

"an early suspend then resume after other contexts may leave Linux WebKit silent while reporting running; the cause may be ours (0 of 510 trivial runs stalled)"

## Product gate owned by #1531

#1531 owns a product gate for suspend-then-resume: after a resume of a suspended context, the clock
advances and audio renders. The gate is defined after the diagnosis. No gate covers that path now.
The SDK phase resumes each of its contexts once, from the pre-boot suspend, and never resumes a
context that has rendered and then been suspended (`hosts/host-web/qualification/sdk-response-entry.ts`
at `374d347c2`). B3's gate moves to one resume (#1345 Amendment 6).

## Next cells (run once, after B3's measurements)

- **V2m:** V2 with the meter lease off and the same first-suspend quanta as V2 (the first segment
  ends at the `renderedQuanta` value that V2 has at its first meter frame, through the same status
  poll).
- **V4m:** the trivial processor with one `process()` message per quantum, V2's suspend timing
  (suspend after one awaited round trip once the first message arrives) and V2's 710 ms
  context-to-resume.
- 60 runs each, interleaved in one shuffled order. Record D4 thread states
  (`/proc/*/task/*/{comm,stat,wchan}` of every WebKit process) at every S and every hang, and a
  `gdb -batch -ex 'thread apply all bt'` of the WebProcess at any hang.
- Then a fresh verifier.
- Reading: S in V4m gives a no-engine repro and opens condition (2). V2m at 0 while V2 stays high
  names render-thread messaging as the trigger. V2m stalling while V4m does not leaves an engine-
  or host-specific difference, which stays ours to separate.

## Narrowest stalling variant

V2: the real SDK phase, then the shipped host and the gate's unsubscribe sequence. The cause is not
isolated. The visible fault is in WebKit: a `resume()` that reports `running` and starts no
destination, and in runs 494, 042 and 079 a WebProcess that stops inside that start. The trivial
processor did not reach it in 60 runs after the SDK phase or in 300 runs without it.

## Evidence

`E/`: `schedule.txt`, `run-all.sh`, `run-one.sh`, `pulse-run.sh`, `drv.mjs`,
`trees/diag/hosts/host-web/qualification/{diag.js,diag.html,trivial-worklet.js}` (uncommitted
diagnostic page, plus one appended export line in the diag copy of `qualification.js`),
`runs/NNN-VARIANT/` (per run: `meta.json`, `result.json` or `out.log`, `gst.*.log`, `pulse.log`),
`analyze.py`, `feats.py`, `cycles.py`, `timeline.py`, `merge.py`, `analysis-rows.json`,
`cycles.json`, `results-table.txt`, `b3repro-summary.txt`, `art/`.
Step 1: `E/step1/` (`schedule.txt`, `run-one.sh`, `drv.mjs`, `analyze1.py`, `results-table.txt`,
`artifact-digests.txt`, `build-b3.log`, `build-main.log`, `runs/`, `trees/{main,b3}`).
Verdict: `/home/bl/misofm/submix-verdicts/1531-diagnosis-verdict.md`.
