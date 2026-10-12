# Find and fix the intermittent Firefox SDK spectrum gate failures

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
- #1106 (`.github/ISSUE_SPECS/1106-spectrum-hop-probe-waits-on-the-clock.md`) covers a different
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
