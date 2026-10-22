# Add a console benchmark row that rides a fader and a pan

Found in #890 (2026-10-22). Qualification work only.

## Problem

`console_automation` (`tools/bench/src/console.rs`, near `:129-140` and `:2021-2022`) automates one compressor threshold. Every fader, mute and matrix stays settled in its timed region, so no benchmark row reaches the ramping output section. #890's fused ramping pass (`fader_matrix_ramp_block`) therefore has no row that can show its cost, and neither will later work on that path.

## Smallest closable slice

One native row, `sixty_four_track_console_fader_ride`, on the standing 64-track console: per block, a live fader move on one track per bank and a pan (matrix) move on one other track per bank, sent as ordinary control values so each opens a ramp. Inside the timed region at least one lane of every bank has a fader or matrix ramp in flight.

## Authorized paths

- `tools/bench/src/console.rs`, `tools/console-workload/` (fixtures or helpers it needs)
- `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-lib.jq`, `scripts/test-console-benchmark.sh` (only to accept the new row)
- this spec

## Objective gates

1. The bench preflight passes with the new row and does not launch the timed workload.
2. In-run checks: a test-support witness or counter shows the ramping output path ran in every timed block; the row's digest differs from `sixty_four_track_console`'s; push counts are exact.
3. One descriptive run (one warmup, two measured rounds), attached. No pass threshold.

## Non-goals

No engine change. No V8 row (a later issue may add one).

## Dependencies

#890 merged, so the in-run witness can name the fused ramping path.

## Evidence and decision record

- 2026-10-22, attempt 1 (implementer), stopped before code: #890's fused ramping pass is reachable only from `FaderMatrixBankProcessor::process` (`crates/builtins-compiler/src/lib.rs`, the one caller of `try_process_ramping_with_matrix(_and_operations)`). That pair exists only when `make_fader_matrix` sees both banks at `BuiltinControlDelivery::BetweenRenderCalls`; it declines `Concurrent`. No production host prepares `BetweenRenderCalls`: the C ABI calls `prepare_host_runtime_with_live_lanes(_successor)`, which passes `between_render_calls = false`, and the browser's `prepare_web_host` calls no `_between_render_calls` entry since #1400b (13fd9b491). Only tests call the host-core `_between_render_calls` entries. So a row whose gate 2 needs the fused ramping witness must prepare a delivery no product runs, and a row on the production (`Concurrent`) delivery takes the split fader/matrix ramp path and cannot see #890. The two conflict with "benchmark real paths only"; the choice is root's. The metered row's doc (`tools/bench/src/console.rs` near `:103-118`) still calls `BetweenRenderCalls` the browser's delivery, which is stale for the same reason.
- 2026-10-22, root ruling (option a): the row is prepared on the product's `Concurrent` delivery and measures today's split ramping path; it is the baseline for #1754, which moves the fused pass onto product plans and re-runs it.
- Gate 2 rescoped: prove a fader or matrix ramp is in flight in every timed block by a production means read outside the clock; the digest-differs and exact-push checks stay.
- Attempt 1 implementation (3f53b4890): `console_fader_ride` record, `workload_kind` `sixty_four_track_console_fader_ride`, emitted last. `SessionRuntime::build_with_strip_controls` prepares through `prepare_session_builtins_with_live_controls` (`Concurrent`, no input lane, as `HostLiveLanes::FADER_AND_MATRIX`). `console_workload::fader_ride` reads the bank chains off `unit_eligibility()` (banked units past the fader/matrix seam: 8 at Simd8) and rides lane 0's fader (Left and Right records, `fader_ms` default 960 samples) and lane 1's pan (lowered 2x2, the pan's own 16-sample window), one transaction per block, values alternating base +/- step.
- Gate 2 witness: no production counter reports a fader ramp, so a stepped twin (same plan, same values as exact sets) renders untimed after every timed block; every timed block must differ from it. The pre-roll asserts the first block after a move differs and the block after the ramps end agrees bit for bit. In-run asserts: in-flight blocks, fader and matrix records, transactions (gate revisions) all exact, `live_values_superseded == 0`, digest differs from `sixty_four_track_console`'s.
- The runner pins the record count outside this spec's paths: `scripts/operator/run-console-benchmark.sh` (`wc -l == 62`, header) and `scripts/operator/preflight-console-benchmark.sh` (`records_required: 62`). The new row makes it 64; gate 3 waits for root to authorize those lines.
- 2026-10-22, root authorized the record-count pin edit outside the spec's paths: 62 -> 64 in `scripts/operator/run-console-benchmark.sh` (header and count check) and `scripts/operator/preflight-console-benchmark.sh` (`records_required`), nothing else in those files.
- Gate 3, descriptive (`run-console-benchmark.sh --step opt2-1750` at c39b198a7, one warmup and two rounds, exclusive lock, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` for load above the ceiling, cpu 31): `artifacts/steps/opt2-1750/`. `sixty_four_track_console_fader_ride` p50 108.24 / 109.63 us per block (rounds 1 / 2); `sixty_four_track_console` in the same run 102.15 / 102.64 us. All 64 records validated; the ride's digest is equal across rounds and differs from the standing row's.
- Attempt 1 verdict (fresh verifier): FAIL, one blocker: the stepped twin rendered a second 64-track plan between every two timed blocks, evicting the riding plan's state from cache, which the standing row does not suffer. Attempt 1's step records (`artifacts/steps/opt2-1750/`, at c39b198a7) are invalid for that reason and are removed; attempt 2 replaces them.
- Attempt 2 (root rulings): the twin is replayed untimed after the timed loop against the riding arm's blocks, kept in storage allocated before the clock; both arms use the product lane set, `controls::live_lanes` of the session as the C ABI and the browser choose it (here fader/matrix, strip input and effects, no observation, no meters; no route lanes, as the session has no submix); a `quiet` arm of the same plan takes no write and alternates per observation with `riding`, so `riding - quiet` is the ride's cost. The witness proves only that the riding arm did not hold a timed block's targets from that block's first sample: not which bank ramped, and not that a value was not applied late (a riding arm writing the previous block's values as exact sets passes it; the applied-revision watermark tests own timing).
- Attempt 2 gate 3, descriptive (`run-console-benchmark.sh --step opt2-1750` at ff9143153, one warmup and two rounds, exclusive lock, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` for load above the ceiling, cpu 31): `artifacts/steps/opt2-1750/`. Rounds 1 / 2: quiet p50 112.14 / 112.27 us, riding p50 116.50 / 116.67 us, paired riding - quiet 4367 / 4317 ns per block (68.2 / 67.5 ns per track); `sixty_four_track_console` in the same run 102.42 / 102.08 us. All 64 records validated. Observed, not investigated: the quiet arm's digest (21d04f12...) differs from `sixty_four_track_console`'s (6bae1dd0...), so attaching the product lanes with no write moves rendered bits on this session.
