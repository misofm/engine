# Hoisting the compressor's step-8 identity masks — a native win and a wasm null

**Candidate.** Compressor effect-optimisation round 1, strategy S2. Three of the mask words step 8
of `crates/miso-engine-compressor/src/kernel.rs` builds per channel-frame — `mix == 1`,
`mix == 0` and `makeup == 0` — are functions of coefficient words alone. A coefficient word cannot
change inside the idle body, which loads `Coef` once and never redesigns, so the three compares
were being repeated for every frame of every block to produce the same masks each time.

**Claim under test.** That hoisting them into `Coef::load` is a measurable saving on the console
strip, and specifically that the release build's LTO had *not* already done it.

## What was built

`Coef` gained `wet_identity`, `dry_mix_zero` and `makeup_zero`, computed once per load and consumed
in `one_frame`. Bit identity is structural rather than tested-for: the masks are built from the
same words by the same compares in the same order, and the ramping body reloads `Coef` every frame
after `advance_ramps`, so a mask is never staler there than the words it came from. Only
`smoothed == 0`, a function of the recursive word, remains per frame.

## Measurement

Paired arms, `artifacts/compressor-round1/` against `artifacts/compressor-round1-baseline/`. Same
tooling, same fixtures, same runner, same host, same commit — only the kernel differs, swapped on a
throwaway branch so the runner's clean-tree precondition is met honestly. Both arms `controlled`.
p50 µs/block, minimum of the two rounds.

| leg | compressor-only | console strip | eq-only *(control)* |
|---|---|---|---|
| native `Simd8`, console runner | 74.69 → **73.05** (−2.2%) | 137.31 → **134.69** (−1.9%) | 43.78 → 43.73 (−0.1%) |
| native `Simd8`, wasm runner | 74.95 → **74.00** (−1.3%) | 137.51 → **135.42** (−1.5%) | 45.10 → 45.35 (+0.5%) |
| **wasm `simd128`** | 128.09 → **128.88 (+0.6%)** | 298.63 → **297.64 (−0.3%)** | 91.86 → 91.80 (−0.1%) |

**Native: a win, and reproduced by a second independently-built binary.** The console runner and
the wasm runner build with *different* frozen release profiles (the wasm runner sets LTO off and 16
codegen units), and both show the same effect with the same sign on every compressor-carrying row
while every row without a compressor stays inside ±0.5%. So the console build's LTO had not already
hoisted it, which is the specific thing this measurement was asked to settle.

**Wasm: a null.** Under wasmtime 47.0.3 the effect is inside ±0.6% with no consistent sign — the
compressor-only row moves the *wrong* way (+0.6%) and the console row the right way (−0.3%), which
is the signature of noise rather than of a small win. The change is kept because it is a real
native saving and costs nothing on wasm, but **no wasm improvement may be claimed for it.**

The likeliest reading is that Cranelift already sinks the compares, or that three lane compares are
too small a share of a wasm frame to surface. Neither was investigated; the null is recorded as
measured, not explained.

## Decision and boundary

S2 is kept. What is ruled null is **the wasm arm of this specific hoist**, not lane-mask hoisting in
general and not the native result. A future round that wants a wasm saving from the compressor's
step 8 needs a different mechanism, not a re-run of this one.

**Reopen if:** a wasm runtime other than wasmtime/Cranelift is measured (a browser engine that
tiers would be materially new evidence — every wasm record here carries
`browser_field_measurement: false`), or the step-8 identity structure itself changes.

## Evidence

* `artifacts/compressor-round1/README.md` — the round's record and full table.
* `artifacts/compressor-round1/{console,wasm-console}-benchmark.accepted.jsonl` — patched arm.
* `artifacts/compressor-round1-baseline/` — baseline arm.
* Digest identity: every `output_sha256` on every row of all five legs is byte-identical between
  the two arms, which is this round's class-A null detector.
