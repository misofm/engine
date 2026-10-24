# Issue #163 phase 2 step 1 — the wasm console baseline

Subject: the phase-3 tree (`e883bbf`) with the console benchmark's subject ported to
`wasm32-unknown-unknown`. No contract change, no kernel change, no fma call site touched. This is
the instrument and the number it reads **before** phase 2 acts, so that phase 2's confirmation has
something to be confirmed against.

The owner's ruling on decision 1 gave GO on the unfused multiply-add contract on the condition
that the win be confirmed *at console level*. Nothing in this tree could measure a console under
wasm. `docs/rulings/wasm-kernel-timing-interim.md` recorded what such an arm would need; all three
of its requirements are met here, and the first one turned out not to be a blocker at all.

## Runner invocations against this phase

| attempt | files | status | workloads launched | rounds |
|---|---|---|---|---|
| 1 | `wasm-console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 | 0 |
| 2 | `wasm-console-benchmark.{raw,accepted,disposition}` | PASS `complete`, `measurement_control: controlled` | 3 | 2 |

Attempt 1 refused before launching anything — the one-minute load average was still decaying from
the candidate's own build. It timed nothing (`raw_sha256: null`), so freeing the canonical names
for attempt 2 discarded no measurement.

Attempt 2: `candidate_commit 1b3543a`, cpu affinity 15, SMT sibling idle, cooldown honoured, two
measured rounds after one warmup, `raw` and `accepted` byte-identical. Round-to-round spread on the
wasm leg is at most **0.20%** on any row.

## What each leg is

Three legs of **one subject**, interleaved observation by observation (#104), so the ratio between
them is a paired distribution rather than a quotient of two summaries taken minutes apart.

| leg | target | width | what it is |
|---|---|---|---|
| `native_simd8` | native | 8 | the production backend every recorded console number was taken at |
| `native_simd4` | native | 4 | the same source at the lane width `simd128` offers |
| `wasm_simd128` | `wasm32-unknown-unknown` `+simd128` | 4 | the artifact the browser ships, under wasmtime 47.0.3 |

`native_simd4` exists so the comparison can separate *which target executed the code* from *how
wide its vectors were*. A wasm/Simd8 ratio confounds the two.

## The baseline, p50 µs/block, minimum of the two rounds

48 kHz, 128-frame quantum, 1 000 observations per leg per row. One block is **2 666.67 µs** of
audio, so "% of one core" is the number that matters for a browser console.

| row | tracks | `native_simd8` | `native_simd4` | **`wasm_simd128`** | wasm % of a core |
|---|---|---|---|---|---|
| **console, `eq+compressor`, real fixture** | 64 | 92.27 | 312.83 | **969.58** | **36.4%** |
| console, `eq+compressor`, synthetic | 128 | 185.05 | 630.72 | **1 941.10** | **72.8%** |
| decomposition: `eq` | 64 | 43.31 | 147.40 | 811.62 | 30.4% |
| decomposition: `compressor` | 64 | 71.86 | 198.59 | 461.59 | 17.3% |
| decomposition: `builtins` | 64 | 23.14 | 31.26 | 303.30 | 11.4% |
| decomposition: `identity` (control) | 64 | 23.15 | 31.53 | 303.11 | 11.4% |
| **idle (silence input)** | 64 | 38.28 | 36.87 | **328.59** | **12.3%** |
| `parametric-eq-nine-track` fixture | 9 | 8.39 | 21.14 | 125.06 | 4.7% |
| nine-track ragged strip | 9 | 17.17 | 44.35 | 148.61 | 5.6% |

## The ratio table — the device floor delta #149 asked for

Paired medians of the per-observation quotient. The `ratio_of_p50` column in the records agrees
with these to three decimals; the validator recomputes it from the legs rather than trusting it.

| row | wasm / `native_simd8` | wasm / `native_simd4` | wasm / phase-3 record |
|---|---|---|---|
| **console 64** | **10.49×** | **3.10×** | **10.65×** |
| console 128 | 10.46× | 3.08× | 10.54× |
| `eq` | 18.71× | 5.50× | 19.53× |
| `compressor` | 6.42× | 2.32× | 6.47× |
| `builtins` | 13.04× | 9.63× | 13.98× |
| `identity` | 13.10× | 9.58× | 13.99× |
| idle | 8.49× | 8.88× | 9.35× |
| nine-track | 14.87× | 5.90× | 15.30× |
| nine-track ragged | 8.62× | 3.35× | 8.79× |

The last column is against `artifacts/issue163-phase3/console-benchmark.accepted.jsonl` — the
authority that says 91.03 µs. It is close to the `native_simd8` column because this arm's own
native leg reproduces that record: **92.27 against 91.03, +1.4%** on the headline row, with this
arm's extra 64-block warmup and three-leg interleave accounting for the difference. That agreement
is the evidence that the wasm leg is being divided by the right denominator.

## One subject, three targets — and the finding that nearly wasn't

**Every one of the nine rows renders byte-identical output on all three legs**, over 1 000 blocks,
in both rounds. The validator asserts it per record and across the set. Cross-backend `to_bits`
identity, which the engine claims and which phase 2 must preserve, holds at *console* level and not
merely at kernel level.

That result took one correction to earn. The arm's first run reported `divergent` on four of the
nine rows. The cause was not the engine: the benchmark's input tone is `f32::sin`, `sin` is a libm
call, and libm is not the same implementation on `x86_64-unknown-linux-gnu` as on
`wasm32-unknown-unknown`. The two targets were being asked to render *different audio*. Anyone who
had not looked would have published a cross-target numeric divergence that does not exist.

The host now computes the tone and injects it into the guest before preparation, so both targets
render the identical input. The native tone is untouched: the nine native console digests still
match the phase-3 record to the byte.

## Reading the decomposition

The rows are subtractable — same fixture, same parameters, same track count, part of the strip
removed in code — and they are **additive on wasm to 0.03%**:

```
builtins 303.30  +  EQ increment 508.33  +  compressor increment 158.29  =  969.91
                                                    measured console block  969.58
```

Per-component, at equal lane width:

| component | wasm | `native_simd4` | ratio | `native_simd4` / `native_simd8` |
|---|---|---|---|---|
| builtins base (SVF-bearing) | 303.30 | 31.26 | **9.70×** | 1.35× |
| EQ increment (SVF-bearing) | 508.33 | 116.14 | **4.38×** | 5.76× |
| compressor increment | 158.29 | 167.33 | **0.95×** | 3.44× |

Two things to read here.

* **The wasm penalty is concentrated in SVF-bearing code**, which is exactly where `mul_add` lives.
  The compressor increment — the row with the least SVF content — shows *no* wasm penalty at all.
* **Phase 0b's kernel number is corroborated.** It measured the SVF block kernel at 7.69× wasm over
  native at W4. The two SVF-bearing console components bracket it (9.70× and 4.38×), which is what
  a console-level measurement of a kernel-level effect should look like: diluted by the plumbing
  around it in the EQ increment, concentrated in the builtins base.

## The projection: what unfused multiply-add would recover

**There is no harness-level substitution available.** `miso-engine-lane` declares no Cargo features
at all, and the fused/unfused choice is made inside `wide_impl.rs`'s macro — contract code, which
this brief does not touch. So the softfma share cannot be *measured* by this instrument; it can
only be *bounded* by the decomposition. Stating that plainly is part of the deliverable.

The bound, with its basis:

* SVF-bearing code is **811.62 µs of the 969.58 µs wasm block — 83.7%.** (Basis: builtins base plus
  EQ increment, both measured rows of an additive decomposition.)
* Its native same-width counterpart is **147.40 µs**, so the excess is **664.22 µs, 68.5% of the
  block.** (Basis: the `native_simd4` leg of the same paired observations.)
* **Ceiling.** If softfma were the *only* difference between the two targets on that code, unfused
  multiply-add recovers all 664.22 µs: the console block falls **969.58 → ~305 µs/block**, 36.4% →
  **11.5% of one core**, a 3.18× improvement.
* **Floor.** Allow a 2× general non-fma wasm penalty on that code — Cranelift codegen, linear-memory
  bounds checks, register pressure — and the recoverable part is 516.82 µs: the console block falls
  to **~453 µs/block**, **17.0% of one core**, a 2.14× improvement.

**So: unfused multiply-add is projected to recover 53–69% of the wasm console block, landing 64
tracks somewhere between ~305 and ~453 µs against 969.58 today.** Phase 2 step 4's in-situ paired
measurement on this same arm is what replaces that bracket with a number.

Two things push the true value toward the ceiling rather than the floor. The compressor increment's
0.95× says the general non-fma wasm penalty on this engine's code is near *zero*, not 2×. And the
`native_simd4` denominators are inflated: halving the lane width should cost about 2×, and it costs
3.39× on the console row — the same class of width anomaly phase 0b recorded for `lane_fma` and
left undiagnosed. An inflated denominator makes the measured excess an **under**statement.

## Caveats — what may not be quoted from this

* **These are not device numbers, and not browser numbers.** wasmtime's Cranelift compiles a module
  ahead of time: the code running at observation 1 is the code running at observation 1000. A
  browser JIT tiers, deoptimises and recompiles on feedback, on a different microarchitecture in a
  different thermal envelope. Every record carries `browser_field_measurement: false`. This is the
  determinism-pinned reference; the browser pass remains the owner's.
* **Not comparable with native console records.** Every record carries
  `comparable_with_console_records: false`. The wasm/native ratio column above is the *only*
  sanctioned way to read the two families against each other.
* **The host-side clock includes the crossing.** Each `wasm_simd128` observation is timed around one
  wasmtime call, which contains the trampoline and stack switch the native legs do not pay. It is
  measured rather than assumed and reported as `guest_call_overhead_p50_ns` — **20 ns**, against a
  969 582 ns block: **0.002%**. It is not subtracted.
* **The allocation audit cannot see inside the guest.** `render_total_forbidden_operations` counts
  this process's heap. The guest allocates in its own linear memory. Every wasm leg says so in
  `audit_scope` rather than reporting a zero that would look like a finding.
* **Leg order within an observation is fixed**, not rotated, as the console benchmark's own facility
  arms do it. Position residue is inside the ratio. It is bounded by the native pair, which sits in
  positions 0 and 1 running the same code on the same target.
* **This arm's native leg is not a substitute for the console record.** It warms 64 blocks on every
  row where the console runner warms none, and it interleaves three legs. It is the ratio's
  denominator, and its agreement with the phase-3 record (+1.4%) is what qualifies it as one.

## For #26 and the app

The uncomfortable headline of #163 was "plausibly over 100% of a phone core". On this desktop
x86-64-v3 core under a pinned ahead-of-time runtime, the shipped wasm artifact renders **64 tracks
at 36.4% of one core and 128 tracks at 72.8%** — before any phone, any browser JIT, and any thermal
envelope enters the picture. And **idle costs 328.59 µs, 12.3% of a core**, against 38.28 µs
natively: phase 4's silence path is class A and rides the same softfma tax as everything else.
