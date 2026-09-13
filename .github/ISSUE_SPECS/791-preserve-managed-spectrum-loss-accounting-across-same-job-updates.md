# Preserve managed spectrum loss accounting across same-job updates

Bounded SDK correctness successor required by engine #789; parent #763 remains
open. Scope by Astra xhigh. Root must create and synchronize the numbered local
spec and matching GitHub issue before implementation. Luna implements; fresh
Astra medium verifies concrete correctness. Maximum **two attempts**, each one
coherent correction and one adversarial verdict. #789 exhausted its five-attempt
budget and remains stopped/open, dependent on this successor; this is not a
sixth #789 attempt. No new feature or broader refactor is authorized.

## Problem and outcome

After one native capture drop, a subscriber receives a gap reporting loss 1 and
an older queued window reporting historical drops 0. Notification handling keeps
its seen-loss count monotonic, but `updateSpectrum` unconditionally resets that
count to the older metadata. A cadence-only update then makes the recovery
window report the same loss again: notification deltas **1,0,1** for one drop.

Correct only that update path so the sequence is **1,0,0**. A same-job update
preserves the consumer's already-accounted native epoch/count even when the
job's current snapshot is historical. A genuinely replaced job/history retains
the existing initialization/reset semantics; never compare its counter against
an unrelated old history. Preserve accepted cadence/callback changes,
publication cursor behavior, typed refusal, sharing and cleanup.

## Frozen baseline and allowed change

Correction source `97a66d0cd2b94e0aef46c4591eca79452d45b6c8`, pinned candidate head
`018b551437a9d4f691282ea817459a449cf754ff`, in
`/tmp/miso-engine-continuous-spectrum`. Unchanged candidate Wasm:
`/tmp/issue789-candidate3-artifact/miso-engine-v1-audio-worklet.simd128.wasm`,
SHA-256 `f41194f922ce09b31cfe004fcd263b0b13da2fc1213cf6b455fba1402a68490e`.

Change only the loss-baseline assignment in
`sdk/src/core/observation-subscriptions.ts::updateSpectrum` and extend the
existing regression in `sdk/test/spectrum-evals.mjs`. Reuse the existing job
identity and baseline fields. No new owner/state container, timer, capture/FFT,
ABI, Worker protocol, native code, artifact regeneration or dependency change.

Read the preserved verdict `/tmp/issue789-astra-medium-review-pass5.md` and actual
reproduction `/tmp/issue789-pass5-update-loss-probe.mjs` / `.log`. Root reports
227 headless tests, three-browser qualification and static/resource gates PASS
for the stopped candidate; those checks did not exercise this update sequence.

## Acceptance and delivery

1. Extend the existing monotonic-loss regression by inserting an accepted
   cadence-only update between gap -> older queued window -> recovery. Assert
   deltas 1,0,0, one total reported loss, unchanged job/capture history and the
   new cadence. Keep ordinary pending, genuine new-epoch reset and existing
   replacement/refused-update checks green. Callback-only updates follow the
   same preservation rule; no additional fixture framework.
2. Run the preserved real-Wasm reproduction against the corrected SDK and the
   **unchanged** candidate artifact; it must produce 1,0,0 rather than 1,0,1.
   Run the existing focused headless spectrum and browser spectrum suites plus
   SDK types/format/deletion checks. Reuse the stopped candidate's native,
   artifact/resource and three-browser evidence; no expanded target matrix or
   native rebuild for this SDK-only change. Root runs required proportional
   delivery/package checks on the corrected candidate.
3. Root checkpoints the exact focused-green paths and retains the regression,
   real-Wasm output and fresh independent verdict. On PASS, push/synchronize and
   close this successor, then record resolution on #789; #789 closes only after
   its accepted combined evidence is upstream and its normal delivery completes.
   If attempt 2 fails, stop, preserve evidence and rebrief; do not weaken the
   counting invariant. Focused-target switching and app integration stay separate.

Attempt 1 implementation: Luna max limits baseline reinitialization to actual job replacement and extends the existing loss regression with an accepted cadence update. Actual candidate3 Wasm reproduction now reports 1,0,0 (/tmp/issue791-update-loss-probe.log); focused headless 7/7, browser 7/7, SDK types/deletion and diff checks pass. Only the two authorized SDK files changed. Awaiting independent Astra medium verdict.
