# Find and fix the intermittent SDK spectrum gate failures in Firefox and WebKit

Qualification fault, found by the stream A2 batch verdict (MINOR 1) and by the #1480 verifier. Code
anchors read on `codex/d15-stream-a2` at `43f39404a`. It touches no engine code until the diagnosis
names one.

## Product outcome

The Firefox leg of the `browser` qualification job stops failing at random in the SDK spectrum
gates. `browser` is a required leg of `qualification`, so each flake can turn a pull request or a
`main` run red for no reason in the change. The first deliverable is the root cause. A fix comes
only after it.

## Context

- Two gates fail, each on its own, only in Firefox, and a re-run passes:
  - `sdk-spectrum-collection`, `hosts/host-web/qualification/run.mjs:646`. The record had
    `firstCapturedSample` 2048, and the gate expects 0 (`collectionFirstCaptured === 0n` with
    `collectionFirstEnd === 2_048n`). Every other flag of the record was true. The exact failure
    text is `spectrum collection did not return distinct owned A/B/A known-signal spans while audio
    continued: <the collection record as JSON>`. The batch verdict kept the field values, but the
    failing log was not kept, so the full record is not on file.
  - `sdk-spectrum-hop`, `run.mjs:669-704`. The text is `firefox: sdk-spectrum-hop: actual browser
    H1024 did not preserve Worklet metadata, Worker results, overlap, smoothing, and ownership`.
    The gate carries no record, so the log does not say which of its conditions failed. The log
    is `/tmp/claude-1002/va2/evidence/logs/browser/firefox-rerun-7.log`.
- Observed rates:
  - Stream A2 batch, quiet box, HEAD: 29 passes in 31 Firefox runs (the first run failed
    `sdk-spectrum-collection`, and re-run 7 of 10 failed `sdk-spectrum-hop`). The first failure
    came while 30 other cargo, audit and browser jobs ran on the same box.
  - Stream A2 base: 20 passes in 20 alternating runs (`/tmp/claude-1002/va2/evidence/ffcmp/`).
    Fisher's exact test of 20/20 against 29/31 gives p about 0.5, so the data does not show that
    the batch raises the rate.
  - #1480 verification (`submix-verdicts/1480-attempt1.md:100-110`): the implementer saw each gate
    fail once in 50 pre-change Firefox runs. The verifier saw neither in 110 runs on the fix (50
    Firefox, 25 Chromium, 25 WebKit, and 10 Firefox with 24 CPU hogs running).
  - Chromium and WebKit: no known failure of these two gates.
- The batch did not touch `hosts/host-web`, `sdk/` or the spectrum code. The fault is older.
- The cause is **unknown**. The failure data are too few to say that load, the audio clock, the
  resume gesture or the Worker round trip is the cause, and this spec does not name one.
- The probes are in `hosts/host-web/qualification/sdk-response-entry.ts`: the collection probe
  returns `firstCapturedSample` at `:1259`, and `runConfiguredSpectrumHopQualification` starts at
  `:874`.
- #1106 covers a different
  symptom of the hop probe: `H1024 browser probe published 1 windows`, seen in Chromium, from a
  poll count that ends before the deadline. It does not explain a failure of `sdk-spectrum-hop`
  that has no such message, and it does not cover the collection gate. #1248 covers the continuous
  gate. Check both before the diagnosis, and record whether either one's fix has landed.

## Decisions frozen for this slice

- **D1. Diagnosis first.** The first deliverable is a root cause, with evidence, written in this
  spec's Attempt record before any fix. "Load" or "timing" without a mechanism is not a cause.
- **D2. Capture before guessing.** Add the failing record to the `sdk-spectrum-hop` gate's
  message (the way the collection gate already prints its record) so that the next failure says
  which condition failed. Keep the full log of every failing run.
- **D3. No retry, no wider tolerance as a fix.** The gate may not pass a failing run by a retry,
  a longer wait or a looser expectation (`0n`, `2_048n`, the hop and span checks) unless the
  diagnosis shows the expectation itself was wrong, and then the spec says why. A wait bound by a
  deadline that replaces a poll count is a fix only when the diagnosis shows the count was the
  cause (#1106).
- **D4. Reproduce, then fix.** The diagnosis tries to make each failure happen on demand (a planted
  delay, a throttled CPU, a slowed Worker). A cause with no reproduction is stated as unproven,
  and the fix then waits for a rebrief.
- **D5. The fix goes where the cause is.** It may be in the probe, in the SDK or in the engine. If
  the cause is in the engine or the SDK, the spec is amended and rebriefed before that edit.

## Deliverables

1. The root cause of each of the two failures, with evidence, or a statement that it is unproven.
2. D2's capture in the gate.
3. The fix and its reproducer, after the cause is accepted.

## Authorized paths

- `hosts/host-web/qualification/run.mjs`, `hosts/host-web/qualification/sdk-response-entry.ts`,
  and any sibling file in `hosts/host-web/qualification/`.
- This spec.
- Nothing else. The paths of a fix outside these are named by the diagnosis and added by an
  amendment before the edit.

## Non-goals

- No change to Chromium or WebKit legs.
- No change to the continuous-spectrum gate (#1248) or to #1106's `published 1 windows` symptom,
  except where the diagnosis shows one cause for all.
- No change to the gate's required status, and no `continue-on-error` on `browser`.
- No engine change without an amendment.

## Objective gates

1. **Cause stated.** The Attempt record names the mechanism of each failure and the evidence (a
   captured record, a trace or a reproduction), or says that it is unproven.
2. **Reproduction.** For each stated cause, a planted condition makes the gate fail in Firefox
   on the unfixed code and pass on the fixed code. Record both runs. The command is the one in
   `.github/workflows/qualification.yml` (`npm run qualify -- --check-matrix
   --self-test-mutations`, one browser).
3. **Soak.** 100 consecutive Firefox runs of the full leg pass on a quiet box, and 30 more pass
   with 24 CPU hogs running. Report the counts. At the observed rate of 2 in 31 the chance that
   100 runs all pass by luck is about 0.1 percent; at 1 in 50 it is about 13 percent, so the
   reproduction of gate 2 is the stronger gate.
4. **Capture.** A forced failure of `sdk-spectrum-hop` (for example the mutation of one of its
   conditions) prints the record and names the failed condition.
5. **Other browsers.** `--check-matrix --self-test-mutations` passes in Chromium and WebKit.
6. Commands: `bash scripts/check-workspace-policy.sh`, `python3 scripts/check-ci-path-routing.py`
   only if a CI file changes. If the fix is in browser-compiled code, the worklet chain.

## Test value

- Gate 2: a fix that only widens a wait or retries turns the planted-condition run green without
  removing the cause. The reproduction runs the same planted condition on both codes, so a fix
  that does not remove the cause stays red.
- Gate 3: a cause that appears at 2 in 31 and stays hidden at 100 clean runs, because another
  condition fixes only the soak's box. The loaded soak asks the question again under load.
- Gate 4: a gate that fails with no record leaves the next failure as blind as this one.

## Dependencies

- None to start. #1106 and #1248 are checked first (see Context). The fix may depend on the
  diagnosis, and then on an amendment.
- Related: *Bound the browser spectrum hop probe by its deadline, not a poll count* (#1106) and
  *Name the failed predicate and bound the waits of the browser continuous-spectrum gate by a
  deadline* (#1248).

## Amendment 1 (root, 2026-10-12): WebKit `sdk-spectrum-hop` H1024 failures under CPU load, and a new title

**New title (root, 2026-10-12).** *Find and fix the intermittent SDK spectrum gate failures in
Firefox and WebKit*. The spec file keeps its number. When this amendment lands, the spec file is
renamed to `1522-find-and-fix-the-intermittent-sdk-spectrum-gate-failures-in-firefox-and-webkit.md`,
and the GitHub issue title is changed to match in the same checkpoint.

*Anchors verified on `codex/d15-stream-b3` at `ebe706208` and on `main` at `6cc1f8bc8`.*

**What is new.** The Context line "Chromium and WebKit: no known failure of these two gates" is no
longer true. The stream B batch-3 B2 diagnosis (#1345, "B2 diagnosis and fix", load bar) saw the
same `sdk-spectrum-hop` failure in WebKit, only under CPU load:

- Text: `webkit: sdk-spectrum-hop: actual browser H1024 did not preserve Worklet metadata, Worker
  results, overlap, smoothing, and ownership` (the gate is `run.mjs:690-725` on the branch,
  `:669-704` on `main`). As in Firefox, the gate prints no record, so the failed condition is not
  known. The failure is a returned record that fails the predicate, not the probe's own
  `H1024 browser probe published N windows` throw (`sdk-response-entry.ts:946-948`), so #1106's
  symptom is not this one.
- Runs on the batch-3 tree (`/tmp/claude-1002/b2diag/bar-final.txt`; logs in
  `/tmp/claude-1002/b2diag/logs/`; CI command, SDK source-bundle mode, a private PulseAudio null
  sink per run):

  | WebKit 26.5 | runs | reached validation | `sdk-spectrum-hop` failures | logs |
  |---|---|---|---|---|
  | no load | 5 | 5 | 0 | `bar-u1..5-webkit.log` |
  | leg i: 32 busy loops on 32 CPUs, unpinned (load average 32 to 163) | 10 | 8 | 1 | `bar-i10-webkit.log` |
  | leg ii: browser and 4 busy loops pinned to CPUs 0-3 | 10 | 10 | 2 | `bar-ii7-webkit.log`, `bar-ii10-webkit.log` |

  In leg i, `bar-i1` and `bar-i8` hung after the SDK phase at the unsubscribe gate's resume
  control, so their spectrum results were never validated; they are not counted.
- Six more, all under 32 busy loops, on B2 diagnostic trees (scratch copies of the batch-3 tree
  with diagnostic edits; the trees and their diffs were not kept): `f-l4`, `g-l1`, `q-l9`,
  `v8-l11`, `v8-l12`, `x-l10` (`logs/<label>-webkit.log`, rows in `summary.txt`). They show the
  failure is not rare under that load; they are not rate data.
- In the same bar, Firefox failed `sdk-spectrum-hop` once in leg i (`bar-i1-firefox.log`) and
  0 times in leg ii and unloaded. Chromium: 0 of 25.
- No `main` tree ran under this load, so the data do not say whether batch 3 raises the WebKit
  rate. The #1480 verification ran WebKit 25 times with no failure
  (`submix-verdicts/1480-attempt1.md:99`; its CPU hogs ran only with Firefox, `:100`). No
  WebKit run under full load is on record before batch 3.
- The same diagnosis found a stall in which a running WebKit worklet `AudioContext` renders
  nothing while its worklet still answers status requests. It happened in a qualification run
  with no load (`/tmp/claude-1002/b2r2/logs/t1-webkit.log`), and a page with no engine code
  stalled under load (`/tmp/claude-1002/b2diag/drv4.out`). Both are in the WebKit clock-stall
  issue. A hop probe whose context stalls for part of its 10 s deadline could publish windows
  that are not one hop apart. That is a hypothesis to test, not a cause.
- In the same bar, Firefox failed `sdk-observation-subscription` 2 times in 10 runs of leg i
  (`bar-i2-firefox.log`, `bar-i8-firefox.log`), and 0 times in leg ii or with no load. That gate is
  not a spectrum gate. The paired measurement below decides who owns it.

**Scope (ruling).**

- **In scope:** D2's capture also covers the WebKit failures, because the gate is shared. The
  diagnosis (D1) states the mechanism of the WebKit failures as well as the Firefox ones, or says
  that the mechanism is unproven.
- **In scope, conditionally:** the fix, when the diagnosis shows one mechanism for both browsers.
- **Out of scope:** a WebKit failure whose captured record has root's full stall signature. That
  signature has four items: the hop context's `state` is `running`; its `currentTime` and the
  worklet status's `renderedQuanta` are equal at the resume and at the failure; at least 2 status
  replies were answered in between; and `lastResult` is `0`. Such a failure is listed in the
  Attempt record with its log and goes to the WebKit clock-stall issue; #1522 does not fix it. A
  failure without the full signature stays in scope.
- The Non-goal "No change to Chromium or WebKit legs" now reads: "No change specific to the
  Chromium leg. A WebKit change is made only for a mechanism that this amendment brings into
  scope."
- To capture the signature, D2's record adds these fields for the hop probe's context, read at the
  resume, at each `pump()` and at the end:
  - `state`;
  - `currentTime`;
  - the status reply's `requestId`, `renderedQuanta` and `lastResult`.

**How it is measured: paired `main` and branch runs.**

1. **Trees.** Export two scratch trees with `git archive`:
   - M is the `main` commit that batch 3's branch is based on (at merge time, the merge's first
     parent).
   - B is the branch head (at merge time, the merge commit).

   Each tree builds its own artifact and passes its own `--sdk-root`. Put the same D2 capture
   patch into both, so that every failure has a record.
2. **Command.** Use the CI command (`--check-matrix --self-test-mutations`, one browser) with a
   private PulseAudio null sink for each run (the B2 `qa.sh` method).
3. **Counts per side (M and B):**

   | browser | no load | leg i: 32 busy loops on 32 CPUs, unpinned | leg ii: browser and 4 busy loops pinned to CPUs 0-3 |
   |---|---|---|---|
   | WebKit | at least 60 | 40 | 40 |
   | Firefox | — | 40 | 40 |
   | Chromium | — | 10 | 10 |

   Run M and B in the order M, B, B, M, repeated, so that a change in the box's load does not
   favour one tree. Record the load average before and after each run.
4. **Classes.** Classify each run by its SDK-phase signature:
   - pass;
   - `sdk-spectrum-hop`;
   - `sdk-spectrum-collection`;
   - `sdk-observation-subscription`;
   - any other `sdk-*` gate, by its name;
   - a page that hung during the SDK phase;
   - root's full stall signature (from the D2 record).

   Runs that fail after the SDK phase (for example `observation-unsubscribe`) are counted apart,
   because their SDK results were still validated.
5. **Statistics.** For each browser, leg and signature, report the 2x2 table (M against B) and
   Fisher's exact test (two-sided), as the Context does for Firefox. "B raises the rate" is
   claimed only at p < 0.05; otherwise the record says "not shown". Suppose WebKit fails at the
   observed rate (3 in 18) and M has no failures. Then 40 pairs give p of about 0.01 for 7 failures
   against 0, and about 0.05 for 5 against 0; 20 pairs would not separate the trees.
6. **Ownership (root, 2026-10-12).**
   - If a signature rises only on B (p < 0.05 and absent on M), those failures are a batch-3
     defect. They go back to stream B, and #1522 does not fix them.
   - If `sdk-observation-subscription` (Firefox) also fails on M, it is an older fault that #1522
     does not own. Root files a new issue for it, and a draft is written only then.
   - If it does not fail on M, it is a batch-3 defect, like any branch-only signature.
   - A spectrum signature that fails on both M and B stays in #1522.

**Gates, added.**

- Gate 3 (soak) adds 30 WebKit runs of the full leg with 32 busy loops after the fix. A failure
  with root's full stall signature is counted apart and listed. Any other `sdk-spectrum-*` failure
  fails the soak.
- Gate 4 (capture) also holds in WebKit: a forced failure prints the record with the fields added
  above.
- Gate 5 now reads: `--check-matrix --self-test-mutations` passes in Chromium, and in WebKit with
  no load.
- *Test value of the paired measurement:* it is PR evidence, not a committed test. It turns a
  "WebKit under load" observation into a statement about the change. Without it, a batch-3
  defect in the SDK path and an older browser fault look the same.

**Dependencies, added.** The WebKit clock-stall issue gives the out-of-scope rule its committed
repro. #1345 does not need to be on `main` before the work starts, because the measurement uses
exported trees.

## Amendment 2 (root, 2026-10-24): narrowed to D2, then closed

*Verified on `main` at `506196ce5`.*

- **Facts.** #1106 (deadline-bound hop wait) and #1248 (named continuous predicates) have landed.
  Of the last 60 `qualification` runs on `main`, no Firefox or WebKit leg failed; the one red
  `main` run since 2026-10-09 was Chromium (37879401215). The one branch run with red browser legs
  since 2026-10-22 (40082172520) failed all three browsers and the workspace job together, a batch
  defect, not this flake. Every recorded spectrum failure came from the 32-CPU development box,
  most under artificial load.
- **R1 (root).** The slice is D2 only. After it lands, #1522 closes (the close text was rewritten
  when the verifier found a candidate cause; see the Attempt record). A browser reproduction (D4, gate 2) and the
  soak (gate 3) are not done, and the collection failure has no named cause; they come back with
  the next red `sdk-spectrum-*` log.
- **R2 (root).** Amendment 1's paired M-vs-B matrix is dropped as moot: #1345's batch 3 merged
  2026-10-13 and `main` has not flaked since.

## Attempt record

**Attempt 1 (2026-10-24, D2).**

- `run.mjs`: the `sdk-spectrum-hop` gate's predicate is split into twelve named conditions
  (`requestHop`, `boundsHop`, `nativeStart`, `nativeRead`, `publicationCount`, `notificationHops`,
  `notificationSmoothing`, `publications`, `startDelta`, `spans`, `resultMetadata`, `ownedArrays`;
  their conjunction is the old predicate). A failure prints the failed names and the
  `configuredHop` record. The record's publications already carry scalars only (no bin arrays);
  the per-notification hop and smoothing lists are reduced to `{length, distinct}`.
- `sdk-response-entry.ts`: Amendment 1's signature fields are added as `clock.atResume` (sampled
  when `resume()` resolves, not awaited by the wait) and `clock.atEnd` (after the wait): the
  context's `state` and `currentTime` and the status reply's `requestId`, `renderedQuanta` and
  `lastResult`. The probe's own `published N windows` throw also prints `clock`. They are **not**
  sampled at each `pump()`: a status round trip per poll would change the wait that
  `sdk-spectrum-hop-wait` measures.
- No new test (gate 4 is the self-test mutation). **Correction to the brief:** the mutation
  `sdk-spectrum-hop-wrong` sets `continuous.first.hopFrames`, so it is caught by
  `sdk-spectrum-continuous`, not by this gate. The mutation that reaches `sdk-spectrum-hop` is
  `sdk-spectrum-hop-missing`. Its failure text, captured once in Chromium on `08ea327e0` with a
  print-only patch that was then removed:

  ```
  chromium: sdk-spectrum-hop: actual browser H1024 did not preserve Worklet metadata, Worker results, overlap, smoothing, and ownership: failed ["nativeStart"] in {"requestHopFrames":1024,"smoothingMs":37.5,"boundsHopFrames":1024,"nativeStart":{"status":1,"sampleRateHz":48000,"quantumFrames":128,"smoothingMs":37.5,"capturedSample":"0","endSample":"0","sequence":"0","windows":"0","droppedCaptures":"0"},"nativeRead":[...4 entries, hopFrames 1024, smoothingMs 37.5...],"publicationCount":12,"wait":{"startDelayMs":750,"deadlineMs":10000,"pollMs":5,"firstPublicationAfterMs":806.5,"secondPublicationAfterMs":833.3,"elapsedMs":838.3,"polls":162,"neverPublishes":{"count":0,"polls":50,"elapsedMs":253.8,"deadlineMs":250}},"notificationHops":{"length":12,"distinct":[1024]},"notificationSmoothingMs":{"length":12,"distinct":[37.5]},"publications":[{"metadata":{"status":"ready","hopFrames":1024,...,"capturedSample":"37504","endSample":"39552","sequence":"0","windows":"1"},"result":{"capturedSample":"37504","endSample":"39552","windowFrames":2048,"binCount":1025,"finite":true,"ownedArrays":true}},{...,"capturedSample":"38528","endSample":"40576",...}],"startDelta":"1024","firstSpan":"2048","secondSpan":"2048","resultHopFrames":1024,"resultSmoothingMs":37.5,"ownedArrays":true,"clock":{"atResume":{"contextState":"running","currentTime":0.0133,"requestId":129,"renderedQuanta":"5","lastResult":0},"atEnd":{"contextState":"running","currentTime":0.8507,"requestId":145,"renderedQuanta":"319","lastResult":0}}}
  ```

  (`...` elides repeated fields of the one-line log; the names and the failed condition are
  verbatim.)
- Real-browser legs on `08ea327e0`, CI command (`--check-matrix --self-test-mutations`, private
  PulseAudio null sink), each run once: Firefox 153.0 passed; WebKit 26.5 passed.

**Attempt 1, verifier MAJOR folded in (root, option b; `41428c6df`).**

- **Candidate cause.** The `publications` condition compared `metadata.endSample >
  metadata.capturedSample`, two decimal strings from `streamMetadataFields`. A string compare is
  lexicographic, so it is false whenever `endSample` has one more digit than `capturedSample`:
  `capturedSample` in 97953..99999, the first consecutive pair landing about 2.04-2.08 s into the
  context at 48 kHz. The probe's audio start is delayed 750 ms, and under load the first pair can
  land that late, which fits the loaded Firefox and WebKit failures and the old message's lack of a
  named condition. Not reproduced in a browser; the defect is shown on the record.
- **Fix.** The clause compares `canonicalU64` values. The same string compare in the
  `sdk-observation` window check (`observations.windows[0]`) now uses `validResidentWindow`, whose
  width-crossing proof already exists. No other ordered compare of string sample fields remains in
  `run.mjs`; the probe in `sdk-response-entry.ts` compares `bigint`s.
- **Proof (in `mutationProofs`).** `sdk-spectrum-hop-width` moves the real record's pair to
  98304..100352 and 99328..101376 and asserts the gate accepts it. Red on the string compare
  (Chromium, `41428c6df` with the clause reverted): `Got unwanted exception: chromium:
  sdk-spectrum-hop-width: valid decimal-width-crossing hop pair was refused`, actual
  `sdk-spectrum-hop: ... failed ["publications"]`. Test value: a sample-field ordering compare that
  goes back to strings turns it red; the existing mutations all keep the pair below 5 digits.
- **Clock reads bounded.** Each `status()` read and the resume sample race a 2 s timer
  (`HOP_PROBE_CLOCK_TIMEOUT_MS`); a timed-out read records `status: "timeout"` instead of hanging.
- Legs on `41428c6df`, each run once: Firefox 153.0 passed; WebKit 26.5 passed.
- **Close record (R1, rewritten).** Candidate cause found and fixed: a lexicographic compare of
  decimal sample strings fails when the first pair lands between about 2.04 and 2.08 s (under
  load); capture in place; reopen on the next red log.
