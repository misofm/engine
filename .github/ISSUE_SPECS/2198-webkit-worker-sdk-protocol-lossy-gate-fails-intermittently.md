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
