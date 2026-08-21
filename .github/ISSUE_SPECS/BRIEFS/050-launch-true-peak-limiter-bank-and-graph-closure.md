# Sol implementation brief — issue 050 launch true-peak limiter bank and graph closure

## Authority and stop boundary

**READY FOR TERRA ATTEMPT 1.** This brief and Issue 050 are authoritative. There are exactly two
total attempts: Terra attempt 1 and, if needed, one bounded Sol correction/review. A second failure
stops. Never inspect V1 or run Issue-049 qualification, audit, target/object, benchmark, timing or
listening work. The timed benchmark invocation count is zero and remains zero.

Issue 016 is stopped, not PASS. Its final scalar correction is nevertheless accepted technical
input. Do not change its descriptor, FIR, guard, gain/hold/release law, latency, automation,
snapshot bytes, reset/recovery behavior or existing scalar output. Issue 050 owns only W4/W8 bank,
scalar-tail, registry/effect-compiler and representative graph/PDC product closure.

## Frozen scalar input

```text
effect / contract / state    miso.true-peak-limiter / 1.0 / layout 1
quality / ports              Normal / required dual-mono main-in and main-out
sidechain                    none; no connected fallback exists
rates                         44100, 48000, 88200, 96000 Hz
N / F / T                    Fs/100 / 6 / N+6 samples
lookahead L / hold H         round(ms*Fs/1000), 0..N / L+6
tail                          Infinite
banking                       homogeneous W4/W8 plus exact scalar tails
```

The Issue-016 authoritative brief remains the source of the exact 48 Annex-2 coefficient bits,
separately rounded scalar operation order, required-gain ring, one-u32 hold correction, automation
and snapshot field order. This successor may cite those bytes but must not duplicate them into a
second mutable DSP contract.

| Fs | latency `T` | lane state bytes | dual-mono state bytes | fixed defaults |
|---:|---:|---:|---:|---:|
| 44,100 | 447 | 3,652 | 7,304 | 24 |
| 48,000 | 486 | 3,964 | 7,928 | 24 |
| 88,200 | 888 | 7,180 | 14,360 | 24 |
| 96,000 | 966 | 7,804 | 15,608 | 24 |

Prepared scalar state is exactly two lane payloads. The 24 fixed bytes are the two retained
three-`f32` default parameter tables. Scalar scratch per frame is zero. Per-track bank state bytes
are byte-for-byte the same scalar payload and use the same layout ID and restore validation.

## Bank binding and ownership

Implement the accepted `NativeEffectFactory::bind_homogeneous_bank` and
`PreparedNativeEffectBank` APIs without changing them. Validation order is normative:

1. Require `requests.len()==width.lanes()` and exact backend/width compatibility.
2. Run the existing scalar descriptor/prepared-metadata validation and initial-value validation for
   **every** member, in ascending track order.
3. Require identical immutable program signatures and Normal quality. Per-track/lane parameter
   values may differ.
4. Require exactly the descriptor's `main-in`/`main-out` topology and no sidechain. Any invented
   sidechain or topology mutation is `Err`, never legal fallback.
5. Only after steps 1–4, obtain `PreparedGateGainKernelV1`. A valid unavailable backend returns
   `Ok(None)`; every malformed request remains `Err` even on that host.
6. Allocate/construct the complete fixed-width bank off render and publish it only after success.
   Failure leaves all caller-owned request inputs and compiler capacity unchanged.

A bank owns exactly `W` independent complete scalar track states and two three-`f32` default tables
per track. Use fixed width-specialized arrays, not padded tracks or a compiled maximum. The exact
declared effect payload/default retention is:

```text
bank_effect_bytes(Fs,W) = W * (dual_mono_state_bytes(Fs) + 24)
```

Therefore:

| Fs | W4 effect bytes | W8 effect bytes |
|---:|---:|---:|
| 44,100 | 29,312 | 58,624 |
| 48,000 | 31,808 | 63,616 |
| 88,200 | 57,536 | 115,072 |
| 96,000 | 62,528 | 125,056 |

The prepared kernel token adds no effect state payload. Width/member metadata and graph-owned AoSoA
planes remain charged through the accepted checked graph accounting. Test exact caps and one byte
below each owned category before publication; do not relabel headers or metadata as DSP state.

## Exact render operation graph

Reuse the accepted bank block/automation representation. Automation spans for a track are selected
from its validated boundary pair; malformed boundaries reject before render. At each sample:

1. Traverse tracks `0..W` in ascending order. Apply each track's block-Point automation in stable
   descriptor order and run its two accepted scalar detector/link/gain/hold/delay lanes exactly
   once. Bypass does not skip this work.
2. Gather the left delayed dry samples, left gains and left identity masks into one width vector;
   call `PreparedGateGainKernelV1` once. Scatter left outputs in ascending track order.
3. Repeat gather/call/scatter for right. Do not share detector, gain, ring, hold, automation,
   recovery or report state across channel or track.

For each packed lane, the frozen kernel graph is:

```text
p = z * g
identity_mask = all-ones exactly when bypass || g == 1, else zero
y = bit_select(identity_mask, z, p)
```

This is exactly one multiply plus bit selection, with zero FMA sites. AVX2+FMA aliases the same
zero-contraction graph. The bank must not vectorize/reassociate the Annex-2 FIR, release recurrence
or state updates. On finite-normal/no-sanitation input this makes scalar/W4/W8 PCM, serialized
track state and reports bit-identical on the same target.

Recovery remains scalar and lane-local. A recovered lane emits delayed safe zero for that sample,
increments only its accepted report field and resets only its lane according to Issue 016. Other
tracks and the other channel continue unmodified. Full/discontinuity resets, active transactional
restore and bypass/identity warming use the accepted scalar methods for all `W` tracks.

## Registry, compiler and graph vertical

Register `miso.true-peak-limiter` beside the accepted launch-native factories and add exactly its
approved effect-compiler dependency/policy mutation coverage. Do not alter the generic registry,
effect ABI, program key or graph cohort algorithm.

Freeze one ten-track fixture at 48 kHz and quantum 128 with homogeneous Normal limiter programs,
legal per-track parameter differences, no sidechain and sufficient preparation capacity:

| selected backend | retained shape |
|---|---|
| W8 | one eight-track bank + two scalar tails |
| W4 | two four-track banks + two scalar tails |
| scalar/unavailable | ten scalar effects |

There is no connected-sidechain fallback row because the effect has no sidechain port. Require
stable ascending membership and no padding. Enabled and bypassed instances both report `T=486`;
exact integer PDC, graph/schedule/observer bytes and bank membership are unchanged by bypass.
Consecutive blocks must match ten independently prepared scalar instances in PCM, per-track state
and reports. A one-byte-below post-bank capacity failure publishes no graph and returns all owned
factory/effect/source inputs through the accepted transactional path.

## Minimum product-closing evidence

Keep tests representative and contained to the limiter, core gain kernel, registry/effect compiler
and graph fixture:

- all four rate descriptor/preparation rows, exact scalar and W4/W8 effect bytes, exact caps and
  one-byte-below rejection;
- width/backend/program/quality/port/initial-value mutation order, including malformed-before-
  unavailable behavior;
- scalar versus available W4/W8 consecutive-block PCM, complete state and report parity with
  per-track parameter differences and scalar tails;
- reset, active snapshot/restore, bypass/identity warming, signed-zero identity and injected
  scalar/bank lane recovery with L/R/track isolation;
- exact ten-track membership counts, `T=486` enabled/bypass latency/PDC, unchanged graph bytes and
  transactional cap ownership return.

Existing green Issue-016 scalar tests remain required regression input. Do not expand the
independent FIR/corpus matrix, long sequences, deterministic cohorts, realtime audit, target/
instruction evidence or listening; Issue 049 owns those surfaces.

## Final command/evidence boundary

After focused limiter/core/effect-compiler/registry/graph tests pass, run formatting, warning-denied
Clippy and one locked workspace check/test/Clippy/rustdoc seal plus the applicable workspace,
realtime, effect-runtime, rack and graph policy/mutation scripts. Run no functional audit main,
target/object command, benchmark/preflight, timing or listening command.

Record candidate identity; attempt number; unchanged scalar contract assertion; exact resource
rows; binding mutation/fallback results; parity/state/recovery rows; graph shape/PDC/cap report;
focused/full/policy outputs; explicit Terra and final Sol PASS/FAIL; and
`timed_benchmark_invocations=0`. Overall PASS requires every Issue-050 gate; otherwise preserve
evidence and stop after the second attempt without weakening the scalar or qualification gates.
