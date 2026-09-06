# Repair metering timing fixture final-chunk admission

## Problem and evidence

The sole descriptive timing invocation for #519/#520 at frozen `11cb3c2e` failed during poll-mode warmup before measured rounds. `timed_poll_mode` prepares exactly 8192 quanta of source content, but its shared `feed_and_render` helper submits every chunk with `end_of_region=false`. The final chunk ends exactly at the prepared region boundary, so `crates/host-core/src/source.rs:169` correctly returns `EndOfRegionMismatch`; the test fails at `hosts/host-web/src/tests.rs:1748` with result 1 instead of 0.

The failed log and JSONL contain the header only; no timing figures are available. The failed log and header are preserved in `docs/evidence/metering-519-520/timing-failed.log` and `timing-failed.jsonl`. Do not retry that invocation or claim a measured speedup. #519/#520's independently tested work avoidance and numerical contracts are not weakened by this tooling failure.

## Smallest useful correction

Correct the timing fixture's final source chunk to mark the actual region boundary, without changing engine admission semantics or general render behavior. Add a small non-timed fixture admission check for an ordinary chunk and the final chunk; the latter must be accepted with its end marker and rejected without it. Extend preflight to exercise this finite fixture boundary setup without launching the timed workload.

Keep the existing frozen 9-stream/128-frame/32-block-period/256-window comparison, shared JSONL serializer, output creation/overwrite refusal, immediate record persistence, and peak/poll payload checks. No generic benchmark framework, artifact promotion subsystem, DSP changes, new performance target or gate weakening.

## Acceptance and one new invocation

- Focused non-timed boundary regression and serialization/persistence preflight pass before any timed work.
- Reviewer freezes the repaired fixture and validator. Execute one new successor-owned invocation, exactly one warmup per mode and two measured rounds, with no retries or tuning.
- Preserve each measured mode immediately; verify comparable peak hashes, poll hashes and emitted window counts before reporting results.
- Report compiler/target, exact source revision, workload, raw records, units and limitations. Values describe isolated accumulator/Rust poll work, including clock/test-counter overhead; they do not establish end-to-end browser callback performance.
- If the repaired runner fails, retain raw evidence and stop under the bounded tooling-attempt rule. Do not block already qualified engine capabilities or weaken their correctness gates.
