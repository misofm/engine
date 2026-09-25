# Repair the console benchmark's mono symmetry witness premise after the census drift

## Defect

`scripts/run-console-benchmark.sh` cannot complete on `main` (a16bc5b6, 2026-09-25): the warmup launch aborts in `tools/bench/src/console.rs` at the `sixty_four_track_console_mono` premise `symmetry[index][0] == tracks` with `left: 65, right: 64`. The last sealed record (`artifacts/issue420-rt3`, candidate `1fc6ed1e`, 2026-09-05) carried `symmetric_lanes 64, lanes 129` for that row, so a change since then added one channel-symmetric op unit to the census (`PreparedPlanExecutor::symmetry_counters` sums `[eligible, 1]` per `RuntimeUnit::Op` plus each bank chain's counters) without the benchmark's premise following it. No CI job runs this benchmark, so it drifted silently.

Reproduce: on a clean checkout of `main`, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 bash scripts/run-console-benchmark.sh --pure-path-baseline` (any arm); see `console-benchmark.stderr.log`. The refused attempt is preserved in `artifacts/pure-path-baseline/` on branch `codex/pure-path-measurement`.

## Smallest closable slice

Tooling only. Authorized paths: `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs` (evidence accessors only), their tests, `scripts/test-console-benchmark.sh`, and this spec.

1. Identify which unit gained eligibility (dump `unit_eligibility()` for the mono fixture; name the op kind and the commit that changed `channel_symmetry()` or the lowering).
2. Decide whether the extra symmetric unit is a legitimate census change (an op that is genuinely channel-symmetric and was previously mis-classified, or a new op) or an engine regression. If legitimate, restate the premise so it counts the mono fixture's *bank lanes* (the quantity the assertion is about) rather than the whole census, or subtract the named non-track units explicitly with a comment naming them. If it is an engine regression, stop and report; do not paper over it.
3. Add a test in `tools/bench` or `tools/console-workload` that pins the mono fixture's per-unit eligibility rows so the next census drift fails at `cargo test`, not at the next benchmark run.

## Non-goals

No engine change, no benchmark run, no fixture change, no new rows.

## Objective gates

1. `cargo test -p bench -p console-workload` green, including the new pin.
2. `bash scripts/test-console-benchmark.sh` and `bash scripts/check-bench-policy.sh` green.
3. A warmup launch of `scripts/run-console-benchmark.sh` on the fixed tree gets past the mono premise (run with the uncontrolled escape hatch into a throwaway arm directory and delete the artifact directory afterwards, or use the runner's preflight if one exists); no record is claimed.

## Dependencies

Blocks the paired measurement of the pure-path batch (#885, #886, #898, #900).
