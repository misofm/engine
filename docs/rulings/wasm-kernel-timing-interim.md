# The interim wasm device-floor datapoint, and what full 0b still needs

This is not a null ruling. It is recorded here because it is the same kind of thing the rulings
directory exists for: a measurement whose *boundary* matters more than its number, kept in one
place so nobody quotes it beyond what it measured.

## The claim it was taken against

Issue [#163](https://github.com/misofm/engine-v2/issues/163) opens with an uncomfortable headline:
every recorded number in this repository is native AVX2 while the product ships wasm `simd128`,
where one fused multiply-add costs roughly fifty-four instructions (master plan §3.5's software
FMA), and it cites a measured 5.5x on the SVF kernel. Phase 0b exists to replace "plausibly over
100% of a phone core" with a number.

## Why the console benchmark is not that number

Full 0b is the console workload under a wasm runtime. It is not reachable without porting the
bench tool, and the reasons are structural rather than a missing flag:

1. `tools/miso-engine-bench/src/console.rs` is `#[cfg(not(target_arch = "wasm32"))]`, and every
   compiler it drives -- session, builtins, effect and graph -- is a
   `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` entry of the bench manifest. The
   whole subject is absent from a wasm build of the crate.
2. `wasm32-unknown-unknown` has no clock. `std::time::Instant` cannot be constructed in the guest,
   so `timing::timed` -- the shared timed region every subject measures through -- cannot run
   inside the module at all. Timing has to move to the host, around an exported call.
3. The guest takes no arguments and reads no environment, by design (`miso-engine-wasm-gate-guest`
   exports `u32`-in/`u32`-out and imports nothing). The runner's round marker and the eleven host
   metadata names have nowhere to go, so a wasm console record could not carry the provenance the
   validators require of every other console record.

**What full 0b needs**, concretely: a `wasm32` build of the bench crate's console subject with the
four compilers available on that target; a host-side driver that owns the clock and passes the
round marker and metadata across the ABI; an exported entry that prepares a plan once and renders
one block per call; and a record family whose validator knows those records are `wasm-simd128` and
refuses to let them be compared with the native ones. None of that is a flag, and none of it is in
scope for phase 0.

## What was measured instead

The frozen gate-G5 lane corpus, built for `wasm32-unknown-unknown` with `+simd128` and executed
under the pinned wasmtime 47.0.3, against the same corpus run natively in the same process, arms
alternated per observation (#104), 500 observations per arm, one warmup and two measured rounds.

The guest exports only a digest, and a digest is a kernel followed by a SHA-256 far larger than
it. The arm therefore publishes a **difference**, never a total. `corpus::digest_case` routes every
lane case and every element-wise case through one identical
`digest_lanes(&lane_values(index, width, true))` path, so every arm in this measurement hashes
exactly the same 32,768 bytes; the SHA-256, the eight host-to-guest crossings that read the digest
words, and the guest's memoisation bookkeeping are all common terms and cancel in the
per-observation delta from the `gain_block/noise` baseline. The validator asserts that every leg
produced the *same digest* for the same case, so a timing difference is a difference in how a
target executed one pinned computation and never a difference in what it computed.

Runner: `scripts/run-wasm-kernel-timing.sh`. Record:
`artifacts/issue163-phase0/wasm-kernel-timing.accepted.jsonl`. Controlled under the #163 phase 0a
preconditions: pinned to one core, SMT sibling measured quiet, load average under the ceiling.

### Result, nanoseconds per case, median of the per-observation paired delta (round 1 / round 2)

| kernel | native Simd4 | native Simd8 | wasm-simd128 @ W4 | wasm/native at W4 | wasm@W4 / native Simd8 |
|---|---|---|---|---|---|
| `svf_block_ramped/noise` | 7714 / 7706 | 3728 / 3776 | 59283 / 59292 | **7.69x** | **15.80x** |
| `one_pole_block/noise` | 4508 / 4508 | 2114 / 2122 | 20700 / 20707 | **4.59x** | **9.77x** |
| `lane_fma` | 22402 / 22394 | 43222 / 43302 | 41550 / 41607 | 1.86x | 0.96x |

The SVF is the kernel the parametric EQ and every builtin filter section ride; the one-pole is the
follower a compressor's detector rides.

## What may be quoted from this, and what may not

**May be quoted.** At equal lane width, the shipped `simd128` target executes the SVF block kernel
**7.7x** slower than the same source natively, and the compressor's one-pole follower **4.6x**
slower. Against the production native backend the engine actually records on -- `Simd8` -- those
become **15.8x** and **9.8x**. Issue #163's cited 5.5x for the SVF is, on this host and this
corpus, an underestimate.

**May not be quoted.** These are not device numbers. wasmtime with Cranelift on an x86-64-v3
desktop core is not a phone's browser: it is a different JIT, a different microarchitecture and a
different thermal envelope. Nothing here licenses multiplying a native console block by 7.7 and
calling the result a phone budget -- the console block is not one SVF kernel, and 9.4% of it
(21.1 us of 223.97 us, `sixty_four_track_dispatch_only`) is not kernel arithmetic at all. Issue
#26's target device and budget remain an owner decision, and real mobile-device measurement is
owner hardware and explicitly out of scope.

## The anomaly this run surfaced, unresolved

Native `lane_fma` is **1.9x slower at Simd8 than at Simd4** (43.2 us against 22.4 us), which is the
opposite of what doubling the lane width should do, and it makes the wasm/native ratio for that one
case (1.86x) meaningless as a software-FMA measurement: if the native `f32x8` path is not reaching
a hardware FMA either, then neither side of that row is the baseline it looks like.

This is recorded and not diagnosed. Diagnosing it means reading `miso-engine-lane`'s `f32x8`
`mul_add` lowering, which is a DSP crate and outside phase 0's surface. It matters directly to
[#163](https://github.com/misofm/engine-v2/issues/163) phase 2, whose whole subject is the numeric
contract around fused multiply-add, and phase 2 should re-derive it rather than inherit this row.

## Reopening

Supersede this note when full 0b lands, or when a native `lane_fma` width anomaly is explained.
Do not supersede it with a re-run on different hardware: add a row.
