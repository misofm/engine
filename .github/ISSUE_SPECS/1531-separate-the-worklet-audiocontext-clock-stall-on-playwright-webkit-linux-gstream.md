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
