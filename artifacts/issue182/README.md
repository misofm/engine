# Issue #182 round 1 — the true-peak limiter's effect-optimisation round, measured

The #175 strip, re-measured on the same fixture at the same settings, after two class-A changes to
`miso-engine-true-peak-limiter` and to nothing else.

#175 named the limiter the single most expensive element of the intended strip — **+88.15 µs
native (×1.92), +196.18 µs wasm (×2.14)** — and recorded that its idle cost was 129.39 µs
"because no gate exists to skip it". This round vectorises the kernel's scalar sections and builds
the gate.

| question | answer |
|---|---|
| Did any rendered bit move? | **No.** Every `output_sha256` on every row, on all three legs, is byte-identical to the #175 arm. |
| What does the limiter cost on signal now? | **+39.25 µs native (was +88.15, −55.5%)**, **+126.25 µs wasm (was +196.18, −35.6%)**. |
| What does it cost on silence? | **−84.06 µs native, −183.77 µs wasm.** Nothing but the limiter changed, so that is the limiter's idle cost, removed. |

**The authority does not move.** `artifacts/issue175/` remains the standing 64-track qualification
record for *what the strip renders*. This directory re-measures *what it costs*, and the digest
equality below is the null detector for the whole round: a class-A change that moved a digest would
not be a faster limiter, it would be a different one.

## Attempts

| arm | attempt | files | status | launches |
|---|---|---|---|---|
| native | 1 | `console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| native | 2 | `console-benchmark.{raw,accepted,disposition}` | PASS `controlled` | 3 |
| wasm | 1 | `wasm-console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| wasm | 2 | `wasm-console-benchmark.{raw,accepted,disposition}` | PASS `controlled` | 3 |

Both arms refused once and both refusals are kept: they launched nothing and timed nothing
(`raw_sha256: null`). The cause is the one #175 records for its wasm arm and which this round found
applies to the native runner too — each runner builds its subject with its own frozen release
settings *before* it evaluates admissibility, so a cold build puts the one-minute load average over
the 0.50 ceiling by itself. Attempt 2 on both arms: cpu affinity 15, SMT sibling quiet, `raw` and
`accepted` byte-identical.

## What changed

| | change | mechanism |
|---|---|---|
| **S1** | uniform-cohort vectorisation | `sliding_minimum` and the box-expiry gather ran per lane, because `LaneShape` is derived from lookahead and lookahead is a per-lane preparation parameter. `lanes_uniform` gates one whole-bank branch — bit compares on the shape *and* on the van Herk phase — under which both collapse to lane-wide row operations, the amortised backward suffix pass included. Anything the gate rejects takes the unchanged per-lane body. |
| **S2** | earned silence fixed point | A block of exact `+0.0` on an instance that has *observed itself* at `clear_runtime`'s documented rest state advances the two cursors and each lane's van Herk phase, and renders nothing. |

Neither is a re-tuning. S1's bit identity is structural: `Lane::min` is *defined* as
`select(self < b, self, b)` (decision D8) and `scalar_min` is `if a < b { a } else { b }`, so one
lane of `a.min(b)` **is** `scalar_min(a, b)`. S2 never skips a block whose every output word and
every state word it has not already proved.

## The measured table

`Simd8` native and `wasm_simd128` under wasmtime 47.0.3, 48 kHz, 128-frame quantum, 1 000
observations, p50 µs/block, minimum of the two rounds. Rows carrying no limiter are the controls
and are expected not to move.

| row | native #175 | native #182 | Δ | wasm #175 | wasm #182 | Δ | digest |
|---|---|---|---|---|---|---|---|
| **console — the intended strip** | 184.52 | **135.62** | **−26.5%** | 368.76 | **299.01** | **−18.9%** | `0e527225d5e7` unmoved |
| **idle (silence input)** | 129.39 | **45.33** | **−65.0%** | 271.20 | **87.43** | **−67.8%** | `7b331c02e313` unmoved |
| console, synthetic, 128 tracks | 372.28 | 273.81 | −26.5% | 742.87 | 600.82 | −19.1% | `0970171176c6` unmoved |
| nine-track ragged strip | 33.04 | 26.64 | −19.4% | 63.75 | 50.05 | −21.5% | `262cfca89298` unmoved |
| eq+comp on simd1 — *control* | 96.37 | 96.37 | +0.0% | 172.58 | 172.76 | +0.1% | `ede41bb7e6fd` unmoved |
| console_legacy — *control* | 94.52 | 94.34 | −0.2% | 172.31 | 171.85 | −0.3% | `ede41bb7e6fd` unmoved |
| eq only — *control* | 43.69 | 43.72 | +0.1% | 91.86 | 91.86 | −0.0% | `83a4b205c383` unmoved |
| compressor only — *control* | 74.61 | 74.01 | −0.8% | 128.21 | 127.74 | −0.4% | `35b1d89136c8` unmoved |
| builtins only — *control* | 22.34 | 22.46 | +0.5% | 47.77 | 47.22 | −1.2% | `5bf3c3772d4c` unmoved |
| dispatch only — *control* | 22.61 | 22.58 | −0.1% | 47.54 | 46.95 | −1.2% | `2b015145fd33` unmoved |
| nine-track eq fixture — *control* | 9.02 | 8.99 | −0.3% | 16.12 | 16.14 | +0.1% | `d5df5ebe109d` unmoved |

**Every control row is inside 1.2% on both arms.** That is the measurement's own sanity check: six
rows that contain no limiter did not move, and the five that do, moved a lot.

The `native_simd4` and `native_simd8` legs of the wasm runner agree with the same signs
(console 186.16 → 137.47, idle 130.30 → 47.48 at `native_simd8`), so the effect is not an artifact
of one backend.

## The limiter's own cost row

`console − eq+comp on simd1`, the only difference being the `simd2` true-peak limiter:

| arm | #175 increment | #182 increment | change | factor over the strip without it |
|---|---|---|---|---|
| native `Simd8` | +88.15 µs | **+39.25 µs** | **−55.5%** | ×1.92 → **×1.41** |
| wasm `simd128` | +196.18 µs | **+126.25 µs** | **−35.6%** | ×2.14 → **×1.73** |

At #175 the limiter cost more than the EQ and the compressor together. It no longer does on
native: 39.25 µs against 21.1 + 52.7 = 73.8 µs measured as increments over the identity control.

## Honest nulls, and where the round fell short of its projection

* **The wasm arm gained less than projected.** The probe that decomposed the kernel projected the
  limiter's strip increment down by about half on both targets. Native delivered −55.5%, ahead of
  that; wasm delivered −35.6%, well behind it. The projection was made from a kernel-level timing
  harness, and the strip increment carries per-block work the kernel harness does not — the AoSoA
  round-trip, the bank dispatch, the boundary check. On the native arm that overhead is a small
  share of a large saving; under wasmtime it is a larger share of a smaller one. **The projected
  wasm console figure of ≈272 µs was not met: the measured figure is 299.01 µs.**
* **No engagement-rate field was added to the console record.** The brief asked for engagement-rate
  instrumentation on the idle row. `silent_engagements` is a `#[cfg(test)]` counter on
  `LimiterCore`, in the same position and for the same reason as `nonfinite_report` —
  instrumentation is not render state — and it is pinned by
  `a_settled_silent_limiter_renders_exactly_the_never_fast_path` at **35 engagements over 40
  consecutive silent blocks**, identically at `f32`, `Simd4` and `Simd8`. Adding a field to the
  frozen `console_session` record instead would have been a schema change to a sealed record, and
  its two validators and their mutation suites, in service of a number the idle row already
  reports as time. The 35/40 is arithmetic and not a tolerance: the main delay line is
  `B = N + 6 = 486` samples, so the claim cannot be earned until both the line and the output block
  it produces have gone entirely `+0.0`, which first happens at block 5.
* **The idle row did not reach ≈37 µs.** It reached 45.33. The residual is the admission test
  itself — two `block_is_positive_zero` scans of the input planes per block, which is the price of
  refusing to trust a claim without re-checking the caller's buffers — plus whatever the rest of
  the strip costs on silence, which this fixture has no row to isolate. The row fell 65.0%; the
  84.06 µs it fell is attributable to the limiter alone, because nothing else in the tree changed.
* **The Annex-2 oversampler was not touched.** It remains the detector's cost centre and is out of
  scope for this round by the researcher's decomposition; the compliance gate (E4) is green and
  untouched.

## Measurement boundary — what may not be quoted from this

* **Wasm numbers are not browser numbers.** wasmtime's Cranelift compiles ahead of time and does
  not tier, deoptimise or recompile on feedback, on hardware that is not a phone. Every wasm record
  carries `browser_field_measurement: false` and `comparable_with_console_records: false`.
* **This is not a limiter qualification.** #049 owns that. This measures one placement of one
  parameter set on one fixture.
* **The console row remains incomparable with any pre-#175 `sixty_four_track_console` row**, for
  the reason #175 states: the name is the same and the workload is not.
