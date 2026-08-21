# Sol implementation brief — issue 053 launch soft-clip bank and graph closure

## Authority and stop boundary

**READY FOR TERRA ATTEMPT 1.** Issue 053 and this brief are authoritative. There are exactly two
total attempts: Terra attempt 1 and, if needed, one bounded Sol correction/review. A second failure
stops. Never inspect V1 or run Issue-052 corpus, audit, target/object, benchmark, timing or listening
work. The timed benchmark invocation count is zero and remains zero.

Issue 019 is stopped, not PASS. Its accepted scalar checkpoint `e674d5e` is immutable technical
input. Do not change its descriptor, coefficient bits, operation order, domains, automation,
latency/tail, state payload, reset/recovery behavior or scalar output. Issue 053 owns only bank,
scalar-tail, one representative alias-claim row, registry/effect-compiler and graph/PDC closure.

## Frozen scalar input

```text
effect / contract / state  miso.soft-clip / 1.0 / layout 1
quality / ports            Normal / required dual-mono main-in and main-out
link / sidechain           DualMono / none
rates                      44100, 48000, 88200, 96000 Hz
latency / tail / support   31 / Finite(29) / final base sample 60
lane / track state         676 / 1352 bytes
retained defaults          24 bytes per track
```

The Issue-019 brief remains the sole source for the 63-tap coefficient literals, ascending retained
tap order, fixed 2x interpolate/cubic/decimate equations, separate multiply/add points, exact dry/
wet selection, three 64-update ramps and 169-word lane payload. The bank must serialize each track
through that unchanged layout and validation.

## Bank binding and exact ownership

Use the accepted `NativeEffectFactory::bind_homogeneous_bank` and
`PreparedNativeEffectBank` contracts. Validate, in order:

1. exact member count for W4/W8 and compatible requested backend/width;
2. every member's scalar descriptor, metadata, rate, quality, quantum, ports, initial values and
   preparation limits in ascending track order;
3. one immutable Normal/no-sidechain program signature while allowing legal per-track/lane values;
4. complete checked effect and graph capacity; and only then
5. backend availability, where a legal unavailable request returns `Ok(None)` and any malformed
   request remains `Err`.

Construction is control-plane-only and transactional. Failure publishes no bank and consumes no
caller ownership or compiler capacity. Each track owns two complete scalar lanes and its own six
retained default values; no lane/history/ramp/report state is shared or padded:

```text
bank_effect_bytes(W) = W * (1352 + 24)
W4 = 5504 bytes
W8 = 11008 bytes
```

Account width/member metadata, sample-major AoSoA histories, runtime member buffers and bank
scratch with checked arithmetic through the accepted post-bank graph resource estimator. Exact caps
pass; one byte below every directly owned category and the final plan cap rejects before publish.

## Exact bank operation graph

At each base sample, advance each track/lane's three accepted ramps once in descriptor order. For
each of the two high-rate phases and each channel independently:

1. write the accepted doubled input or inserted zero into that track's interpolation lane;
2. traverse the retained taps in ascending index order, computing vector `product=h[k]*history`
   followed by vector `sum=sum+product`;
3. apply the cubic with the exact scalar threshold masks and `p0=u*u; p1=p0*u; p2=p1/3; y=u-p2`;
4. write the nonlinear value to the decimation lane, repeat the same ascending FIR graph, and retain
   only the accepted even-phase result.

AoSoA vectors pack the same L or R lane across tracks. There is no horizontal operation or foreign
state warming. Base Wasm/NEON W4 and AVX2 W8 use the same explicit operation graph; AVX2+FMA exposes
zero contraction sites and must alias the same result. Gather/scatter, masks and tail ordering are
stable. Scalar tails run the unchanged scalar instance.

Bypass still executes and warms every wet lane, then selects the 31-sample delayed dry bits. Mix
zero/unity output uses the same identity selection. Sanitation and computed-fault recovery remain
lane-local and match the scalar payload/report exactly. Snapshot/restore is all-member transactional;
both reset kinds delegate the accepted scalar semantics independently for every track.

## Minimum direct evidence

- Exact W4/W8 retained bytes and caps at all four rates; wrong width/backend/count, malformed
  metadata/program/ports/sidechain and cap mutations prove validation-before-fallback.
- Available native bank versus scalar-peer PCM, full state and report equality over consecutive
  blocks with representative unequal track/lane values, active automation, bypass warming, active
  restore, both resets, signed-zero identity, sanitation and one injected lane recovery. One row may
  cover several properties; do not expand into a corpus.
- Legal unavailable W4 fallback on a host without that executable backend, plus compile-time/source
  coverage of its operation graph; scalar tails cover all remainders.

## Frozen alias-claim row

Run exactly the Issue-019 representative row: `N=16384`, bin 3001 unit sine, drive `+18 dB`, output
`0 dB`, mix `1`, three complete warm periods, rectangular DFT. Compare all non-DC/non-fundamental
energy against fundamental energy for the fixed-2x output and the independent f64 naive-1x cubic.
Record both ratios and require at least 2.0 dB improvement. No window, normalization, post-filter,
extra tone/rate/drive or retry is permitted. This is a correctness/product-claim gate, not a timed
benchmark.

## Registry and ten-track graph vertical

Register `miso.soft-clip` beside accepted launch native effects and add exactly its approved direct
effect-compiler dependency and policy mutations. Do not change registry/program-key/public APIs.

Freeze one homogeneous, unconnected ten-track fixture at 48 kHz and quantum 128:

| selected backend | retained shape |
|---|---|
| W8 | one eight-track bank plus two scalar tails |
| W4 | two four-track banks plus two scalar tails |
| scalar/unavailable | ten scalar instances |

Require ascending stable membership with no padding, consecutive-block scalar-delegate PCM/state,
latency 31, tail 29 and final support 60. Enabled and bypassed plans retain identical latency,
integer PDC, bank membership, schedule and canonical bytes. Independently derive the corrected
post-bank resource delta; a final-plan cap one byte below returns all ten prepared inputs and
publishes no graph.

## Final command/evidence boundary

After focused soft-clip/core/effect-compiler/registry/graph tests, run formatting, warning-denied
Clippy and one locked workspace check/test/Clippy/rustdoc seal plus applicable workspace, realtime,
effect-runtime, rack and graph policy/mutation scripts. Run no functional audit main, cross-target,
object-inspection, benchmark/preflight, timing or listening command.

Record the accepted scalar identity; attempt number; unchanged scalar-contract assertion; exact
bank resources; request/fallback/parity/state/recovery rows; alias ratios; graph shape/PDC/cap
report; focused/final/policy outputs; explicit Terra and final Sol PASS/FAIL; and
`timed_benchmark_invocations=0`. Overall PASS requires every Issue-053 gate.
