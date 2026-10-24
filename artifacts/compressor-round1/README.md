# Compressor round 1 — the effect-optimisation round, measured

Two class-A changes to `crates/miso-engine-compressor/src/kernel.rs` and to nothing else, measured
as a **paired pair**: the same tooling, the same fixtures, the same runner and the same host, with
origin/main's kernel in one arm and the patched kernel in the other.

| question | answer |
|---|---|
| Did any rendered bit move? | **No.** Every `output_sha256` on every row of all five legs is byte-identical between the two arms, and still matches the pins in `artifacts/issue175/`. |
| What does the strip cost now? | **−1.9% native** on the intended console strip (137.31 → 134.69 µs), −2.2% on the compressor-only row. |
| And on wasm? | **Nothing.** ±0.6%, no consistent sign. A measured null, ruled and recorded. |
| Did the ramp guard land? | Kept, and **under-resolved** by this row — see below. Nothing is claimed for it here. |

**The authority does not move.** `artifacts/issue175/` remains the standing qualification record
for *what* the strip renders. This directory measures *what it costs*, and the digest equality is
the round's null detector: a class-A change that moved a digest would not be a faster compressor,
it would be a different one.

## Attempts

| arm | attempt | status | launches |
|---|---|---|---|
| native patched | 1 | PASS `controlled` | 3 |
| native baseline | 1 | FAIL `precondition_loadavg_above_ceiling` (launched nothing) | 0 |
| native baseline | 2 | PASS `controlled` | 3 |
| wasm patched | 1 | FAIL `precondition_loadavg_above_ceiling` (`wasm-console-benchmark.attempt-1-refused.*`) | 0 |
| wasm patched | 2 | PASS `controlled` | 3 |
| wasm baseline | 1 | PASS `controlled` | 3 |

Both refusals are kept and launched nothing (`raw_sha256: null`). The cause is the one
`artifacts/issue182/README.md` records, and this round found it applies twice over: **each runner
builds its subject before it evaluates admissibility**, and the wasm runner exports its own frozen
release profile (opt-level 3, LTO off, 16 codegen units) that differs from the preflight's — so its
builds are cold however warm the tree is. Both retries warm the build under the runner's own
profile *first* and only then wait for the one-minute average to decay.

## What changed

| | change | mechanism |
|---|---|---|
| **S2** | hoist the step-8 identity masks | `mix == 1`, `mix == 0` and `makeup == 0` are functions of coefficient words alone, and the idle body loads `Coef` once and never redesigns. They move into `Coef::load` as `wet_identity`, `dry_mix_zero`, `makeup_zero`. Only `smoothed == 0`, a function of the recursive word, stays per frame. |
| **S3** | idle-lane guard in `advance_ramps` | A ramping *block* is not a ramping *lane*. A per-lane `is_ramping` early-out; a lane with nothing in flight reads `remaining` seven times and does no more. |

Neither is a re-tuning. S2's masks are built from the same words by the same compares in the same
order, and the ramping body reloads `Coef` every frame so a mask is never staler than the words it
came from. S3 is the identity because `next_value` on a finished ramp returns `current` and mutates
nothing — calling it and not calling it are the same operation.

## The measured table

48 kHz, 128-frame quantum, 1 000 observations, p50 µs/block, minimum of the two rounds. Rows
carrying no compressor are the controls and are expected not to move.

### native `Simd8` (console runner)

| row | baseline | patched | Δ | Δ% |
|---|---|---|---|---|
| **compressor only** | 74.69 | **73.05** | −1.64 | **−2.2%** |
| **console — the intended strip** | 137.31 | **134.69** | −2.62 | **−1.9%** |
| **console legacy** | 94.75 | **92.86** | −1.89 | **−2.0%** |
| **eq+comp on simd1** | 96.53 | **94.30** | −2.23 | **−2.3%** |
| console, synthetic, 128 tracks | 274.51 | 271.96 | −2.56 | −0.9% |
| nine-track ragged strip | 26.74 | 26.46 | −0.28 | −1.1% |
| idle (silence) — *control* | 45.66 | 45.72 | +0.06 | +0.1% |
| eq only — *control* | 43.78 | 43.73 | −0.05 | −0.1% |
| builtins only — *control* | 22.38 | 22.33 | −0.05 | −0.2% |
| dispatch only — *control* | 22.59 | 22.65 | +0.06 | +0.3% |
| nine-track eq fixture — *control* | 8.96 | 8.97 | +0.01 | +0.1% |

**Every row that carries a compressor moved; every row that does not is inside ±0.3%.** The idle
row is the sharpest control of the set: it *does* carry a compressor, but renders silence, so
#182's silence fixed point skips the kernel entirely and it correctly does not move.

### wasm `simd128` and the wasm runner's native leg

| row | wasm baseline | wasm patched | Δ% | native8 baseline | native8 patched | Δ% |
|---|---|---|---|---|---|---|
| compressor only | 128.09 | 128.88 | **+0.6%** | 74.95 | 74.00 | −1.3% |
| console strip | 298.63 | 297.64 | **−0.3%** | 137.51 | 135.42 | −1.5% |
| console legacy | 171.98 | 173.05 | **+0.6%** | 95.34 | 93.93 | −1.5% |
| eq+comp on simd1 | 172.31 | 172.48 | **+0.1%** | 96.06 | 94.74 | −1.4% |
| eq only — *control* | 91.86 | 91.80 | −0.1% | 45.10 | 45.35 | +0.5% |

The wasm runner builds with a *different* frozen release profile from the console runner (LTO off,
16 codegen units), so its native leg is an independently-built second measurement of the same
change — and it reproduces the native win with the same sign on every compressor-carrying row.

## The automation-active row, and what it does not resolve

No row of the standing table could see S3 at all. `console_model` clears the fixture's automation
unconditionally, both fixture gates assert the standing sessions declare none, and the one arm that
delivers spans — `console_hoist` — drives banks of parametric EQs. **No compressor in this
benchmark had ever seen an automation span.**

`console_automation` closes that gap: one Point span per block, on one track (`ch00`, slot `comp`,
`threshold`, left channel), pushed through the same bounded live-console queue a host pushes
through, into a real prepared plan. Three arms carry the identical control channel and differ only
in what rides it. `quiet == restated` is asserted in-run — the first time the #144 stationary hoist
is stated about the *compressor* through a prepared plan — and `restated != automated` is the
honesty half.

| arm | round 1 | round 2 |
|---|---|---|
| baseline ramping surcharge | 2184 ns | 2155 ns |
| patched ramping surcharge | **1873 ns** | **2094 ns** |

**This is recorded as under-resolved, not as a result.** −14.2% on round 1 and −2.8% on round 2,
with the patched arm's round-to-round spread (11%) wider than the baseline's (1.3%). The direction
is right in both rounds and neither regressed, but the row cannot separate the effect from its own
noise, for two structural reasons: only one bank in eight ever enters the ramping body, and the
surcharge also contains the queue drain, span validation, `apply_automation` and the ramping lane's
own `design_lane`, none of which S3 touches.
`docs/rulings/compressor-idle-lane-guard-console-under-resolved.md` records what a later round
needs to resolve it — a more-lanes-ramping row variant, or a kernel-level paired arm — and why
neither was chased here.

The control-queue delta (`restated − quiet`) is −220 to +221 ns across four rounds: at this row's
resolution, delivering one Point span per block costs nothing measurable.

## Honest nulls and boundaries

* **S2 buys nothing on wasm.** Ruled and recorded in
  `docs/rulings/compressor-identity-mask-hoist-wasm-null.md`. The change is kept for its native
  saving; no wasm improvement may be quoted for it.
* **S3 has no console number.** See above.
* **Wasm numbers are not browser numbers.** wasmtime's Cranelift compiles ahead of time and does
  not tier. Every wasm record carries `browser_field_measurement: false`.
* **This is not a compressor qualification.** #088 owns that. This measures one placement of one
  parameter set on one fixture.
* **Out of scope by the researcher's decomposition, untouched:** wasm `Simd8` banks (a D4 question
  escalated separately), any fast-dB tier change (sealed), interleaving (a confirmed null), the
  uniform-delay gather (a session-level null), and the 20 ms fixed latency (contract-level).
