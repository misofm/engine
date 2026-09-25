# Issue #880 MC-2 — compressor coefficient-domain ramps

## Implementation

Attack and release retain their existing linear-millisecond parameter ramps and public current/
target values. A Point designs its exact target coefficient with the existing `f64` law; the current
coefficient word is the start endpoint. A preallocated `LinearRamp` then interpolates between
those values for 64 samples, with the usual exact target snap on sample 64. Retargeting starts from
the live coefficient word, so it is continuous. The render loop no longer evaluates `exp` while
those ramps advance.

A Point back to the current milliseconds can cancel a moving parameter while its coefficient still
needs to return to that time's exact design. In that case the parameter's current and target stay
equal and its step stays zero; its remaining count carries the bounded 64-sample coefficient
return through the existing ramping path. If the coefficient is already at the endpoint, the
parameter settles as before.

The effect retains state-layout version 1 and its 22-word (88-byte) per-channel payload. Restore
reconstructs the coefficient ramp from the serialized current/target/remaining triple: it designs
the coefficient at the current and target times, then interpolates for the remaining count. As in
the existing payload contract, an active restore is deterministic and partition invariant but does
not promise bit-identical continuation of an auxiliary ramp state that the payload does not store.
Full and discontinuity resets reseed both coefficient ramps from the designed words. Mono-collapse
copies and symmetry checks include the auxiliary ramps.

## Evidence

- Only the scalar `dual_mono_ramping` corpus row moved. Its C1 SHA-256 changed from
  `b0bf75abf8795696987c08cb72619dc90801bd6cde413ad42636e428f8f2866d` to
  `6a9ca596a0a37f97b1d980229473d84674ec50afac60196565a6d7fbdb90ade2`. The other three rows
  stayed fixed, and the corpus test confirms scalar, Simd4 and Simd8 agreement.
- The focused tests check attack and release retarget continuity, exact 64-sample endpoints,
  cancel-to-current behavior, reconstructive restore, reset behavior, mono-collapse, and every
  frozen block partition. The payload remains 22 words per channel.
- MQ-2's frozen workload is unchanged. Its event-rate coefficient count is now derived as 16 calls
  per bank/parameter/block, or 128 for release only and 256 for attack plus release across eight
  banks; the no-automation arm remains 0. The baseline record and fixture remain schema 1 with
  their original 1,024 / 8,192 / 16,384 counts. The validator accepts that preserved baseline and
  the new schema-2 profile separately.

The post-change MQ-2 workload ran exactly once, with one 32-block warmup and two 32-block measured
rounds per arm, at frozen candidate commit `ffe9d233ea16105a1d985665d53997ec1b2e585a` (tree
`4243eafea57778a3c2e16a3a1c61adb74bfb37cd`). It used the same AMD EPYC 7313P, x86-64 Linux
machine, `rustc 1.97.1` / LLVM 22.1.6, eight Simd8 banks, 64 tracks, 48 kHz, and 128-frame blocks
as the baseline. The unchanged baseline artifact remains in `record.json` / `raw.log`; the MC-2
record and raw output are preserved separately. There was no retry.

| Arm | Coefficient calls/block | Baseline round means (µs/block) | MC-2 round means (µs/block) | MC-2 round max (µs/block) |
|---|---:|---:|---:|---:|
| Release only | 128 | 169.448 / 169.313 | 87.583 / 87.501 | 91.013 / 92.035 |
| Attack and release | 256 | 215.761 / 216.534 | 94.228 / 94.609 | 100.481 / 113.656 |
| No automation | 0 | 40.739 / 40.833 | 39.697 / 39.630 | 43.583 / 44.475 |

The automated arms include coefficient design, event handling, and coefficient interpolation; their
timings do not isolate `exp`. These descriptive results are not an acceptance threshold. Full
per-block measurements and provenance are in [`mc2-record.json`](../artifacts/issue880-mq2/mc2-record.json)
and [`mc2-raw.txt`](../artifacts/issue880-mq2/mc2-raw.txt).

Validation: debug and release `cargo test --locked -p compressor`, package clippy with warnings
denied, format check, the MQ-2 preflight/self-test, and `scripts/run-wasm-gates.sh` all pass. The
preflight launched zero timed workloads; the separate authorized run above launched exactly one.
