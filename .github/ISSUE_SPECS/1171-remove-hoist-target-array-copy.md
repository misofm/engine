# Remove the materialized target-array copy from the hoist benchmark subject

## Finding and bounded product slice

The complete bench housekeeping audit #1145 found an actual per-lane target-array copy in HoistArm::render. In the unchanged source at tools/bench/src/console.rs, targets selects one of two precomputed immutable target tables, then `let (prepared, count) = targets[index * self.lanes + lane]` copies the twelve-target tuple before iterating the valid prefix. The pinned AVX2/FMA shipping-profile generated assembly from checkpoint `753fb5c7` has a 936-byte frame and memcpy length672 at that tuple load, followed by the count load. Root independently inspected the source and worker B's /tmp/engine-housekeeping-b-1145/hoist-render.s.txt. No timed invocation, cycle estimate, projected speedup or production-engine copy claim is made.

Smallest closable slice: borrow the selected tuple and its valid prepared-target prefix while retaining the exact same bank/lane/application order. Confirm that synchronous target application cannot retain or mutate that input. This is a bounded tooling/optimization successor to the fixed untimed result-ownership slice #1145; it does not reopen a DSP algorithm or pursue a descriptive timing number.

## Frozen contract and scope

Only the bench target-selection/copy statements and their accurate comment, plus this spec. Existing workload construction, two target tables, active counts, parity selection, target words, bank/lane order, application/process calls, resource/audit boundaries, warmup/observation schedule, output/digest/record fields and class-A floors remain unchanged. No manifest/lock/policy edits, new benchmark runner, test framework, permanent generated-code/digest pin, SIMD algorithm, API or other crate change. Four unused direct dependency edges reported by #1145 are a separate configuration question and are outside this slice.

## Gates and test value

- Review immutable tuple ownership and target-application lifetime semantics; no reference escapes or crosses a target mutation.
- Locked existing bench tests and strict package lint/format/policy gates; existing untimed preflight/console fixture as proportional. No new test for this local borrowing substitution.
- One-time current shipping-profile code inspection must show that the identified full-array memcpy is gone; retain the before/after excerpts as review evidence, not a committed source-grep test. Do not claim every remaining copy gone.
- Do not launch a descriptive benchmark or timing optimization loop. Existing short record qualification tests may execute their timers without interpreting/quoting the values. If later timing is desired, it belongs to the frozen weekly workload/validator and one invocation with one warmup/two measured rounds.
- First focused-green exact-path product checkpoint pauses for root commit/push before evidence layering. Root adversarial review against this brief gets one verdict per coherent attempt, maximum two attempts; preserve any failed evidence. PASS and evidence upstream precede GitHub closure and required qualification precedes merge.

## Sol brief approval

Approved by root Sol, 2026-10-01 within the user's practical copy-efficiency request. Implementation starts only after #1145 is remotely synchronized. Two requested GPT-6.1 Sol xhigh workers remain the team; worker B implements and root reviews. No owner decision, measured gain or implementation is claimed yet.
