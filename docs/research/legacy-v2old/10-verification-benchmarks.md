<!--
Provenance: copied from misofm/engine-v2-old docs/research/10-verification-benchmarks.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Verification and benchmarks

The exactness corpus covers finite accepted input, layouts, cohort occupancy/tails, event offsets, recursive state, and partitions 1, 7, 64, 128, 512. Native AVX2 and browser Wasm SIMD compare output and declared serializable state bit-for-bit at 44.1, 47.999, 48, 88.2, and 96 kHz. The awkward 47,999 Hz fixture catches accidental 48 kHz assumptions. Rate-dependent coefficients, time conversion, latency, tail, and automation are included.

Admission/regression goals—not class-leading claims—use a real mounted session at 48 kHz/128: 32 stereo full chain ≤250 µs median native AVX2 and ≤500 µs browser Wasm SIMD; core-only ≤40/80 µs; 64-track full ≤500/900 µs; EQ (four stereo tracks) ≤20 µs, compressor ≤15 µs, master limiter ≤15 µs native. Report p50/p95/p99/deadline for 8/32/64/128 tracks and 512-frame native blocks.

Benchmarks require quiet host, pinned physical core with SMT sibling idle, governor/config receipt, warmup, and at least 15 independent alternating samples. Statistical reporting follows [Georges et al.](https://doi.org/10.1145/1297027.1297033); affinity mechanisms are documented by [sched_setaffinity(2)](https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html). The current host has 8 physical/16 logical CPUs, no isolated/nohz_full cores, and powersave governor: its results are non-authoritative until shielding is configured in its benchmark issue.

E2-020 begins in W0: its first deliverable is a host receipt plus pin/shield/governor protocol and empty/synthetic-overhead baseline. It extends to real sessions/effects only as their implementations land. The verification harness also audits process allocation, locks/waits/logging paths, control-plane entry, mount overflow, non-finite rejection, explicit copy counters, capability refusals, and reproducible environment receipts.
