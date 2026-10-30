# WebKit worker: sdk-protocol-lossy gate fails intermittently

GitHub #2198.

## Problem

The browser gate `sdk-protocol-lossy` (`hosts/host-web/qualification/run.mjs`,
`validateSdkProtocol`) failed on the WebKit worker row two times, and passed on rerun each time:

- #1387 qualification: one lossy-lane event (sample 384) was missing.
- #2191 qualification: a lossy event was out of the headless engine's order. Four reruns passed.

The lossy lane drops events by design. The gate requires that, for each lossy kind, the last
received event carries the headless engine's last sample. On WebKit worker mode, the worker can read
the last block's telemetry after a later write replaced it, or the gate reads before the last
block's events arrive.

## Scope

1. Find the cause from the gate's failure detail (add the received and expected rows to the failure
   message if they are not there).
2. Decide which is wrong: the gate's claim (the last block's lossy telemetry always arrives) or the
   worker drain (it can read late or skip the last block).
3. Fix the wrong side. Do not add retries and do not loosen the claim without a reason that the spec
   records.

## Gate

- `sdk-protocol-lossy` passes on 10 consecutive WebKit worker runs, and its mutants
  (`sdk-protocol-lossy`, `-none`, `-first-block`) still turn it red.

## Decision record (attempt 1, 2026-10-30)

### Cause

The failure message already printed both full row lists. Local base runs (7ef7567bd, a null
PulseAudio sink): WebKit worker failed 2 of 16, Chromium worker 0 of 10. Both failures had the same
shape (`d` = diagnostic, `c` = counter snapshot, `@` the sample):

```
browser:  d@256 d@384 c@256 c@384 d@512 c@512
browser:  d@384 d@512 c@384 c@512
headless: d@128 c@128 d@256 c@256 d@384 c@384 d@512 c@512
```

Every event is the headless engine's, each kind is in the headless order, and each kind's last
event carries the last sample (512). The gate failed only on the order *between* the two kinds.

The two kinds travel in different lanes: the render diagnostic in the reliable lane (lane 0), the
counter snapshot in the lossy lane (lane 1). The control Worker's `drainControlEvents` empties lane
0 and then lane 1, and every `control_event` call runs the core's service step first
(`SessionState::dequeue_event`). In `worker` mode the worklet renders meanwhile, so a service step
inside the lane-0 loop observes a new block and stages its diagnostic, which lane 0 posts before lane
1 posts the earlier block's counter snapshot. The headless engine renders synchronously between its
drains, so it never interleaves.

### Which side was wrong

1. The gate overclaimed. The C ABI's event lanes are two independent FIFOs (#1570b D1); there is no
   order between them, and no host can give one without a global sequence in every event. An app
   orders lossy telemetry by its sample stamp. The real contract of each lossy kind is: every event
   that arrives is the engine's event of its kind and block, the events of a kind keep the engine's
   order, a block's event can be replaced by a later one, and the newest arrives. The gate now
   holds exactly that per kind (each kind's events are a subsequence of the headless events of that
   kind, the last carries the headless last sample, and the headless run has one event per block of
   each kind). This is the recorded reason for the narrower order claim: the cross-kind order was
   never a lane contract. The browser SDK's `onEvent` doc now says the order is per lane.
2. The engine also had a defect in "the newest arrives" for diagnostics, which the reported
   "missing lossy event at sample 384" fits. `collect_render_activity` had two render-diagnostic
   slots and, when both were full, skipped the block's diagnostic *and* marked the block observed,
   so that block was lost for good, even when it was the last block. Three service steps with no
   reliable-lane read in between fill the slots, and the Worker's lane-1 loop is exactly that (each
   lossy read runs a service step while render runs). Now a block observed while both slots are full
   stays owed, and the next service step with a free slot stages the newest observed block's
   diagnostic (its sample, sequence and revision at observation). A refused reliable-event
   reservation is also retried instead of lost. The diagnostic rides the reliable lane; it coalesces
   only through its two slots. Each owed block that a newer block replaces counts in the existing
   `TELEMETRY_COALESCED` counter (5), beside the controller's own same-key telemetry coalescing, as
   AGENTS.md requires for coalesced telemetry; no counter ID was added. Not reproduced in the browser
   locally; the unit test
   `a_block_observed_while_the_diagnostic_slots_are_full_is_staged_when_one_frees`
   (`crates/control-plane`) renders four blocks and expects `[128, 256, 512]` and one coalesced
   count. It is red on the old code (`[128, 256]`), on a mutant that keeps the first waiting block
   (`[128, 256, 384]`) and on a mutant that moves no counter (`0`, not `1`).

No retry was added. The Wasm module changes (the control-plane fix).

### Mutants

`sdk-protocol-lossy` (a counter snapshot of no headless block), `-none` (no lossy event) and
`-first-block` (only the first block's telemetry) each still turn the gate red: the first is in no
headless kind list, the second has an empty kind, the third's last sample is the first block's.
