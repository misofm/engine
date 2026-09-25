# Issue #880 MQ-2 benchmark evidence

The frozen ramp-spike benchmark ran once on the checkpointed harness at candidate commit
`e0c57d6ff959616bab23e6783a9a14585408020d`. The machine was an AMD EPYC 7313P, x86-64 Linux
(`6.8.0-139-generic`), with `rustc 1.97.1` / LLVM 22.1.6. The harness source SHA-256 was
`bf2d6604d641f7200bced4a86ce4ba41c5ba2eaf996ec1a66ec378264416e81c`; the runner SHA-256 was
`886ac84bcd431d39f737ae673be4db5112a9472930142964e950f9b2673a0949`.

The workload used eight `Simd8` compressor banks (64 tracks), 48 kHz, and 128-frame blocks. Each
arm had 32 untimed warmup blocks followed by two measured rounds of 32 blocks. The timed interval
covers the eight `process_bank` calls; input copying and event setup happen outside it. Release
targets alternate 800/1,600 ms. Attack/release targets alternate 20/80 ms and 600/1,800 ms. The
no-automation arm processes the same seeded stereo input. All automation points are at frame zero
on every block.

| Arm | Derived `exp` calls per block | Round 1 mean / max (µs) | Round 2 mean / max (µs) |
|---|---:|---:|---:|
| Release only | 8,192 | 169.448 / 175.995 | 169.313 / 174.312 |
| Attack and release | 16,384 | 215.761 / 220.029 | 216.534 / 220.700 |
| No automation | 0 | 40.739 / 45.918 | 40.833 / 45.236 |

Call counts are derived from ramp state, not production instrumentation:
`8 banks × 8 tracks × 2 channels × 64 ramp frames × ramping parameters`. This gives 1,024
`math::exp` calls per bank and ramping parameter, 8,192 for one parameter across eight banks, and
16,384 for two. The non-timed `mq2_preflight_payloads_prove_ramps_restart_on_each_block` test
checks targets and remaining-frame counters for every bank lane and both channels, verifies the
64-sample ramp completes, and verifies the next point restarts it. The payload assertions probe
all eight lanes of one prepared bank; the workload applies the same event pattern to each of its
eight independently prepared banks.

The observed difference from the no-automation arm includes automation/event handling as well as
coefficient recomputation; it is not an isolated cost for `exp`. Across the two rounds, the release
arm exceeded the corresponding no-automation mean by 128.709 and 128.480 µs/block; the attack plus
release arm exceeded it by 175.022 and 175.701 µs/block. These are descriptive baseline numbers,
not an acceptance threshold or a claim about another host.

The runner preflight passed before timing. It checked output refusal/persistence, shell exit-status
propagation, fixture acceptance and a wrong-call-count rejection, compiled the ignored target, and
ran a harmless marker test through actual libtest output to verify result extraction and `Simd8`
selection. The runner selected exactly one ignored benchmark test; its raw output contains one
`MQ2_RESULT`, and the final record passes the frozen jq validator. No timing retry was made.
The actual benign libtest marker output used to test prefix-tolerant extraction is preserved in
[`artifacts/issue880-mq2/preflight-marker.log`](../artifacts/issue880-mq2/preflight-marker.log).

Reproduce the non-timed checks and inspect the preserved run with:

```text
scripts/test-issue880-mq2-benchmark.sh
jq -e -f scripts/issue880-mq2-record-validator.jq artifacts/issue880-mq2/record.json
```

The complete structured record and raw libtest output are preserved in
[`artifacts/issue880-mq2/record.json`](../artifacts/issue880-mq2/record.json) and
[`artifacts/issue880-mq2/raw.log`](../artifacts/issue880-mq2/raw.log).
