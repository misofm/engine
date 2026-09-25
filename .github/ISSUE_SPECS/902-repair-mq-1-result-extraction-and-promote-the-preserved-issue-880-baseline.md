## Problem

Issue #880 MQ-1's single authorized benchmark completed successfully at `2a8977f5`, but record promotion failed: libtest printed `test mq1_transient_shaper_ns_per_lane_sample ... MQ1_RESULT {...}` on one line, while the shell parser expected `^MQ1_RESULT`. The exact raw measurements and failure disposition are preserved with #880. No timed retry is authorized or needed.

## Smallest closable scope

Repair MQ-1's result extraction and promote the existing raw record with an explicit provenance/failure-recovery field. Use the committed raw log as a fixture. Do not change the benchmark workload, run new timings, change production DSP, or build another benchmark framework.

## Files

- `scripts/run-issue880-mq1-benchmark.sh` and its focused preflight/self-test helpers.
- Existing MQ-1 artifacts under `artifacts/issue880/mq1-baseline/` and a separately named recovered record (never overwrite originals).

## Gates

- Parse exactly one complete result from actual libtest-prefixed output and unprefixed output; reject zero, duplicate, malformed or nonfinite results.
- Exercise the extraction and complete promotion path using fixtures without launching the timed workload.
- Preserve original raw log and failed disposition byte-for-byte; validate recovered record, host/toolchain/candidate identity and the recorded one-warmup/two-round protocol.
- Existing preflight argument, schema, exit-propagation and overwrite-refusal tests pass, with zero workload launches.
- Shell syntax checks pass. No benchmark retry.

## Delivery and bounded attempts

Two attempts maximum: one implementation and one correction, each followed by independent review. Preserve a failing result rather than loosening gates. Implement separately from #880; the parent retains its raw descriptive numbers and candid failure record. Synchronize local spec/GitHub evidence and close only after reviewed delivery.
