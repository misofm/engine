# Publish managed spectrum recovery after the bounded gap drain

## Trigger

Engine SDK 0.4.2 can repeatedly expose `gap` while `readLatest()` advances with valid recovered FFT frames. In automatic polling, `#refreshSpectrumJobs` publishes and notifies the native gap before performing its one bounded recovery read. That intermediate notification consumes the subscriber cadence. The recovery updates the canonical result, but the timer's normal completion notification is then suppressed until the next cadence; repeated queue pressure can repeat this forever.

A deterministic actual-source reproduction at H256 ran 60 `gap -> ready` recovery cycles over 1.2 seconds. The SDK canonical token advanced from 1 to 61 while the deployed app remained at token 1 and returned false after its bounded retention expired. Removing only the premature notification produced 61/61 ready notifications and accepted display reads while preserving `nativeMissedWindows = 1` for every recovered publication.

## Smallest closable product slice

For scheduler-driven spectrum polling, publish and account for the initial gap, perform the existing single bounded recovery read, then let the existing timer completion notify the final published state. Do not consume subscriber cadence on an intermediate gap that has already recovered synchronously.

If the recovery read rejects, notify the already-published gap/terminal state once, respecting cadence, then rethrow so existing error handling remains intact. Do not unconditionally notify at the end of refresh because a scheduler-backed manual pump must return its own final notification rather than having its cursor consumed internally.

Keep native loss accounting, maximum two sequential reads, publication identity, immutable publication metadata, strict availability, and all lifecycle/close fences unchanged. Do not change native DSP, H256, FFT window size, queue capacity, smoothing, app animation, Wasm/Worklet bytes, or package versions in this implementation issue.

## Exact path boundary

- `sdk/src/core/observation-subscriptions.ts`
- `sdk/test/spectrum-evals.mjs`
- this issue spec

Release/version metadata belongs in a separately briefed patch-release issue after this implementation passes review.

Attempt 1 is frozen to removing the successful recovery path's premature internal notification and adding only a narrow catch around the second `spectrumRead`: on rejection, notify the already-published gap once per handle subject to normal cadence, then rethrow the same error. Successful recovery uses the existing timer completion or `pump()` caller notification boundary. Do not add a helper, public API, type, native, browser-worker, artifact, package, or version change.

## Objective gates

- Repeated automatic `gap -> ready` recovery for at least one second, including cold start before any ready callback, publishes the recovered ready identity on every cycle.
- `readLatest()` identity and delivered ready identity agree; strict availability advances with the canonical result.
- Every recovered notification carries truthful accumulated `nativeMissedWindows`; cadence and `skippedPublications` remain bounded and deterministic.
- Gap followed by pending, another gap, failure, or rejected recovery remains unavailable and reports the terminal state without silently losing the first gap.
- At most two sequential native reads occur per scheduled capture; no loop or new allocation enters realtime processing.
- Scheduler-backed manual pump returns the final notification and does not lose it to an internal cursor update.
- Existing update, close, in-flight, epoch, publication immutability, types, formatting, and SDK test gates pass.
- Fresh adversarial review confirms the notification is emitted at one owner boundary and the ack/cursor cannot precede a dropped publication.

The focused proof must include:

- a deterministic simulated 1.2-second, 60-cycle cold-start H256 `gap -> ready` scheduler episode with exactly 120 reads, 60 `ready`/available callbacks, callback identity matching `readLatest()` every cycle, one accrued native miss and one coalesced skipped publication per callback;
- a scheduler-backed manual `pump()` whose `gap -> ready` result is returned and whose callback fires exactly once, proving refresh did not consume the cursor;
- a rejected second-read promise that exposes the first gap exactly once when cadence-eligible, remains unavailable, retains loss, and rejects `pump()` with the same sentinel, plus a cadence-blocked phase;
- second-read `pending`, `gap`, and `failed` cases whose final state is respectively the initial gap, second gap, and failure, always unavailable and never exceeding two reads; and
- corrected existing assertions that currently encode an intermediate gap after successful same-poll recovery.

Run the focused named Node tests, the full `node --test sdk/test/spectrum-evals.mjs`, `bash scripts/check-sdk-types.sh` when locked dependencies are present, and `git diff --check`. If the normal qualified SDK artifact directory is already available, also run `bash scripts/check-sdk-headless.sh <qualified-artifact-dir>` without rebuilding or repinning Wasm for this issue.

## Evidence

Fresh Astra xhigh reviewed native capture, SDK publication/cadence, app retention, animation, canvas lifecycle, and the prior EQ issue sequence. It rejected the app-side status bypass and blanket canvas decay as redundant and unsafe: the bypass fails cold start, while the canvas change masks real failure/capture-epoch lifecycle resets. The SDK ordering correction alone changed the exact pressured reproduction from 11/61 accepted app reads to 61/61, final app token 61 matching SDK token 61, with all loss counters preserved.

Sol high approved this as the smallest closable slice. The success path emits at the existing caller boundary; gap publication and loss accrual happen before recovery, so a recovered ready notification retains the loss and reports the coalesced gap. A rejected recovery cannot reach timer completion, which is why only that second await receives the fallback notification. Both reads remain sequential and bounded on the SDK control plane; render, DSP, native code, Wasm, realtime allocation/syscall behavior, and portability are unchanged.

## Implementation and review evidence

Attempt 1 checkpoint `63eb2214` removed the premature successful-recovery notification and added the rejected-recovery fallback. Focused, full spectrum and qualified headless gates passed, but Sol review returned **FAIL** because the catch also enclosed epoch assertion, publication validation and loss accounting. A fulfilled invalid recovery could therefore emit and consume the prior gap before its validation error escaped.

Attempt 2 checkpoint `e52e73ab` narrows the catch to only the second transport read. Epoch assertion, publication validation and loss accounting execute outside it. Its new regression supplies a fulfilled ready recovery whose result exceeds the capture bound, proves rejection emits no fallback callback and consumes no cursor, then proves the next valid ready publication recovers with the pending gap counted as one skipped publication.

Sol's attempt 2 adversarial verdict is **PASS**. The reviewed gates are: 7/7 focused tests; the full spectrum suite with 11 passes and 8 artifact-dependent skips; static SDK typecheck using the locked compiler/types; the qualified headless gate with 284/284 passes against `/tmp/issue855-artifacts.HD1jhN`; and clean diff checks. The review also rechecked the 60-cycle cold-start episode, automatic and manual success boundaries, rejected recovery cadence, pending/gap/failed terminal states, the two-read bound, close/in-flight/epoch behavior, publication immutability, exact-path scope, and the ack-before-drop question.
