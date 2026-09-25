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

## Attempt 1 implementation evidence

- The result extractor recognizes one `MQ1_RESULT` marker at line start or after libtest's whitespace-delimited test prefix. It requires one complete JSON object with the exact result keys, two positive finite observations per arm, and a valid fixture digest; zero, duplicate, malformed, and nonfinite records fail.
- `--recover-preserved` runs the same extractor and promotion validator against the committed raw log and failed-disposition JSON. Recovery checks the archived SHA-256, confirms the extracted numbers match the failed disposition, and writes only a separately named recovered record. It records the original failure, candidate and runner identity, both source paths and hashes, and zero recovery workload invocations.
- The recovered MQ-1 observation remains tied to candidate `2a8977f5f0fb9b3384e2d71632f21c7f9896dce4` and E1 commit `6f662fee7b47a5eb38b67e0ddc6d007edd438cfa`: bank `[6.281854, 6.281717]`, scalar `[40.841292, 40.803620]` ns per lane sample. These are the original one-warmup/two-round measurements, not new timing evidence.
- Existing artifacts were preserved: raw log SHA-256 `f77b1db698c8248d82cf73a433032c8cb1482bff98886a7eeca9c5c3e9adb565`, failed disposition SHA-256 `725b7aaff22970a96771023aa4fa4b86ac5685b53d1551bb59e3a164dd67a3cf`, and runner failure report SHA-256 `1c88fb8cfb1b0bbf6caa765eaba4883c15b566eaa2757f676cb8de1d110fd3d4`.
- Validation: `bash -n` passed for the runner, helper, and self-test; `bash scripts/test-issue880-mq1-benchmark.sh` passed, including prefixed/unprefixed extraction, invalid-result rejection, full fixture promotion, existing preflight/schema/exit/overwrite checks, and source-artifact hash preservation. Its cargo step was `--no-run`; timed workload launches: 0. The committed recovered record passed the MQ-1 record validator.
- Attempt 1 is ready for independent adversarial review. MB2 candidate-specific preflight remains outside this bounded MQ-1 repair; its existing E1 source pin will need the separately approved minimal adaptation before any MB2 post-change benchmark.

## Independent review and integration

Astra xhigh recorded **attempt-1 PASS** with no blocking findings in
`docs/issue880-class-b-astra-review.md`. It independently checked selected-source
preflight, complete offline promotion, original artifact preservation, record/source
hashes and additional malformed/nonfinite/duplicate parser probes. No timing was
rerun. The separately approved #880 MB-2 adapter now passes an explicit source/revision
through the self-test, so the combined fast-tier source is no longer tested as E1.
The original E1 record and provenance remain unchanged. Reviewed implementation is
complete; coherent batch delivery and GitHub synchronization remain pending.
