# Issue 880 MQ-1 timing evidence

One timing invocation completed at candidate `2a8977f5f0fb9b3384e2d71632f21c7f9896dce`, with the
effect source tree unchanged from E1 checkpoint
`6f662fee7b47a5eb38b67e0ddc6d007edd438cfa`. The harness ran one warmup and two measured rounds
for each arm over the frozen four-second, 48 kHz, 128-frame workload:

| Arm | Round 1 | Round 2 |
|---|---:|---:|
| Simd8 bank | 6.281854 ns/lane-sample | 6.281717 ns/lane-sample |
| Scalar | 40.841292 ns/lane-sample | 40.803620 ns/lane-sample |

A lane-sample is one track frame; both dual-mono channels are processed for each lane-sample. The
programme uses eight tracks, dual-mono link mode, attack 0.75, sustain −0.5, mix 1.0, seeded
impulses and decays. Fixture SHA-256:
`85645b77376e39c43934476b5e5c5c37f96ae6abff7cc66b2b3d28a827275b6f`.

The timed test completed and passed. The runner then failed to extract the measurement line: the
Rust test harness prefixed `MQ1_RESULT` with `test mq1_transient_shaper_ns_per_lane_sample ...`,
while the runner expected the marker at the start of a line. The run's exact stdout/stderr and the
runner's original failure disposition are preserved in [the raw log](../artifacts/issue880/mq1-baseline/mq1-baseline.raw.log)
and [runner failure record](../artifacts/issue880/mq1-baseline/mq1-baseline.runner-failure.json).
[The expanded failure record](../artifacts/issue880/mq1-baseline/mq1-baseline.failure.json) includes
the observed rounds and host metadata. These numbers remain descriptive observations and are not
promoted as a validated record. The parser repair and record promotion belong to [successor issue
#902](https://github.com/misofm/engine/issues/902). No timing retry was made.

Host: AMD EPYC 7313P, x86_64, Linux 6.8.0-139-generic; Rust 1.97.1, LLVM 22.1.6,
`x86_64-unknown-linux-gnu`. `blast-cetus` and `clickhouse` were active on the shared host, so the
measurements are descriptive and may include background-load effects.
