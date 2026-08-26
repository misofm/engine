# The unfused multiply-add: the nineteen sites, and the one question that turned out not to be a question

**Candidate.** Issue #163 phase 2 — "unfused multiply-add on the numeric contract (class B —
**owner ruling required**; deletes the browser's ~7× filter-kernel tax; §8 re-pins + exhaustive
bound + listening qualification)". Owner ruling 2026-08-26: "GO on phase 2 (unfused multiply-add
contract). Rationale: the product is leaning heavily into wasm — the upcoming in-browser console
app targets real music production and mixing — so the browser's filter-kernel penalty is the cost
that matters." Step 2 of the ruling's execution order: "the ~13-site fma audit with per-site
rulings (recurrence-stability sites may keep exact paths, documented)".

**Status.** Adopted at **all nineteen sites**. No site keeps an exact path, `softfma` is retired
rather than renamed, and no `fma_exact` exists. The ruling anticipated that recurrence stability
might force exact paths; boundary 1 records why that concern does not survive contact with the
state matrix, and it is the reason this audit has no exceptions to seal. Two things did come out of
the sweep that were not anticipated: a signed-zero carve-out that was already not what the frozen
contract claimed (boundary 3), and one genuine difference in *kind* rather than degree between the
two forms, which lies outside the operating domain (boundary 4).

## The criterion

For one site with `u = 2^-24`:

* fused: `r_f = fl(a·b + c) = (a·b + c)(1 + d)`, `|d| ≤ u`
* unfused: `r_u = fl(fl(a·b) + c) = (a·b(1 + e₁) + c)(1 + e₂)`, `|e₁|, |e₂| ≤ u`

Subtracting, `r_u − r_f = a·b·e₁ + (a·b + c)(e₂ − d) + O(u²)`, so

```text
|r_u − r_f|  ≤  u·|a·b|  +  2u·|a·b + c|  +  O(u²).
```

The second term is the ordinary rounding **both** forms already carry. The first term, `u·|a·b|`,
is the whole of the contract change. It is an absolute quantity in units of the *product*, so each
site is judged on two numbers:

* the **absolute divergence referred to full scale (dBFS)** — the audible quantity, because every
  render-path signal is bounded by a few units full scale; and
* the **cancellation ratio `|a·b| / |a·b + c|`** — the factor amplifying the change in *relative*
  terms. Bounded by a small constant over the operating domain ⇒ incidental. Unbounded ⇒ candidate
  load-bearing, and then the question becomes whether the amplified error survives to the output.

## Boundary 1 — the fma contract cannot move a pole, so "recurrence stability" was never at stake

This is the finding that decided the audit, and it is an argument rather than a measurement.

The TPT state update (`kernels.rs::svf_step`) is `x[n+1] = A·x[n] + B·u[n]` with, from
`ReferenceSvfStateSpace`'s documented substitution,

```text
A = [[1 − 2·c1,  −2·a2],
     [    2·a2,  1 − 2·a3]]
```

`A` is built **only** from the prepared coefficients `c1`, `a2`, `a3`. Those are designed in `f64`
on the control plane and rounded once on the way in; the fma contract does not appear in them and
does not appear in `A`. A per-step arithmetic perturbation `δ[n]` therefore propagates as
`Σₖ Aᵏ·δ[n−k]` — through an operator the contract change **cannot touch**. Two consequences:

1. **Stability is not a function of the fma contract.** The Jury test reads `A`; `A` is unchanged;
   a section that was strictly stable before is the same section after. Unfusing cannot move a
   pole, cannot change a settling time, and cannot turn a conditionally-stable design unstable.
   The concern the ruling flagged — that recurrence-stability sites might need exact paths — has no
   mechanism behind it.
2. What *does* change is the amplitude of the injected noise, amplified by the propagation gain,
   which scales as `1/(1 − ρ(A))`. That gain is identical for both arms. So the honest question is
   not "does the recurrence diverge" but "does `G·|δ_u|` sit materially above the `G·|δ_f|` the
   frozen contract already accepts" — a ratio, and one that has to be measured rather than bounded,
   because the bound above is worst-case per step and the propagation is where the size lives.

Measured over 12 236 designs (7 kinds × 4 launch rates × 23 log-spaced cutoffs × 7 Q from 0.1 to
32 × gain where the design reads it), with `ρ(A)` up to 0.999979 and propagation gain up to
1.9 × 10⁵, the answer is that the oracle distance grows by at most **3.82 dB** — a factor of 1.55
on an error floor that is itself 66 to 140 dB below full scale. Under sustained rendering (1 000 000
frames, 20.8 s at 48 kHz, at the tightest corner of each kind) the growth is at most **3.06 dB** on
the output and **3.08 dB** on the retained state, and for three of the seven kinds it is negative —
the unfused arm lands *closer* to the `f64` oracle than the fused one, which is what an error term
that is noise rather than bias looks like.

**The rule this establishes.** A numeric-contract change is load-bearing for a recurrence only if
it changes the recurrence's state matrix. Rounding changes the injection; topology changes the
propagation; only the second can destabilise. Ask which one a proposal touches before arguing from
"but it is a feedback path".

## Boundary 2 — every site is incidental, which makes the seal an absence rather than a registry

The per-site table below is the audit's required output. Nineteen `.fma(` call sites (the ruling
estimated thirteen; phase 3's interleaved cascade and the multiband all-pass added the rest), in
six families.

| # | Site | Family | Expression | Worst in-domain divergence | Max cancellation ratio | Verdict |
|---|---|---|---|---|---|---|
| 1 | `lane/kernels.rs:276` | F1 | `d1 = fma(−c1, ic1, a2·v3)` | −103.5 dBFS | 4.8 × 10⁴ (via `G`) | **incidental** — boundary 1 |
| 2 | `lane/kernels.rs:278` | F1 | `d2 = fma(a3, v3, a2·ic1)` | −103.5 dBFS | 4.8 × 10⁴ (via `G`) | **incidental** — boundary 1 |
| 3 | `multiband/lib.rs:515` | F1 | `ap = fma(−2k, v1, x)` | −79.4 dBFS | 1.9 × 10⁵ | **incidental** — same recurrence, all-pass mix |
| 4 | `lane/kernels.rs:139` | F2 | `y = fma(m2, v2, fma(m1, v1, m0·v0))` | −79.4 dBFS | bounded by `Σ|mᵢ|` | **incidental** — feed-forward |
| 5 | `lane/kernels.rs:218` | F2 | same, ramped coefficients | −79.4 dBFS | bounded by `Σ|mᵢ|` | **incidental** — feed-forward |
| 6 | `lane/kernels.rs:310` | F2 | same, interleaved cascade | −79.4 dBFS | bounded by `Σ|mᵢ|` | **incidental** — feed-forward |
| 7 | `lane/kernels/builtins.rs:374` | F2 | same, fused builtins chain | −79.4 dBFS | bounded by `Σ|mᵢ|` | **incidental** — feed-forward |
| 8 | `lane/kernels.rs:365` | F3 | `y = fma(c, x − y, y)` | −138.6 dBFS | 5.3 × 10⁴ | **incidental** — first-order, `G = 1/c` |
| 9 | `effect-runtime/envelope.rs:80` | F3 | release path, `t = fma(c, d, x_abs)` | −138.6 dBFS | 5.3 × 10⁴ | **incidental** |
| 10 | `effect-runtime/envelope.rs:173` | F3 | mean-square follower | −164.7 dBFS | 2.0 × 10² | **incidental** |
| 11 | `delay/lib.rs:1146` | F3 | damping one-pole | −138.6 dBFS | 5.3 × 10⁴ | **incidental** |
| 12 | `gate-expander/kernel.rs:387` | F3 | gain-dB slew | −138.6 dBFS | 5.3 × 10⁴ | **incidental** |
| 13 | `true-peak-limiter/lib.rs:883` | F3 | release ramp | −139.5 dBFS | 1.5 × 10⁴ | **incidental** |
| 14 | `multiband/shim.rs:55` | F3 | `fma(c, y − target, target)` | −138.6 dBFS | 5.3 × 10⁴ | **incidental** — identity at `c = 0` preserved |
| 15 | `lane/kernels.rs:423` | F4 | `y = fma(mix, w − x, x)` | −114.4 dBFS | 1.7 × 10⁷ | **incidental** — bypass identity exhaustively preserved |
| 16 | `delay/lib.rs:1155` | F4 | wet/dry blend | −114.4 dBFS | 1.7 × 10⁷ | **incidental** |
| 17 | `lane/kernels.rs:578,579,586,587` | F5 | 2×2 matrix rows | −120.4 dBFS | 2.0 × 10⁸ | **incidental** — feed-forward |
| 18 | `delay/lib.rs:1081,1082` | F5 | ping-pong matrix | −120.4 dBFS | 2.0 × 10⁸ | **incidental** |
| 19 | `delay/lib.rs:1132` | F6 | parameter smoothing | −138.6 dBFS | 5.3 × 10⁴ | **incidental** — control rate, not audio |

The high cancellation ratios in F4 and F5 are real and are not a reason to keep an exact path. They
say that when a mix or a matrix row very nearly cancels, the *relative* error of the result is
large — in both forms. The result there is a number 114 dB or more below full scale whose low bits
carry no signal in either contract. Absolute divergence is the audible quantity and it is bounded.

Because no site is load-bearing, the seal this ruling installs is the opposite shape from the fast
dB tier's. That seal admits exactly six named crossings and refuses the vocabulary elsewhere; this
one admits **none**. `scripts/check-unfused-seal.sh` fails if a fused multiply-add appears anywhere
in workspace source, with a single registered exemption — the audit's own fused arm, which has to
keep a copy of the retired operation in order to compare against it.

`softfma::fma_f32_via_f64` and gate G3 are deleted rather than kept dormant. Keeping a gate that
pins "the software FMA equals the hardware FMA" would pin a property nothing in the engine relies
on any more, and would leave a 54-instruction-per-op emulation one call away from a future kernel.

## Boundary 3 — the bypass identities survive exactly, and one of them was never what the docs said

A bypassed slot must be an identity kernel, not a near-identity one, and that property is what a
large part of the pinned corpus rests on. Three identity sub-domains are enumerable, so they were
swept **exhaustively** — all 2³² `f32` bit patterns of the free operand, 15 sweeps, **64 424 509 440
cases**:

| sweep | fused-vs-unfused mismatches |
|---|---|
| F4 `gain_mix_step`, `mix = 0`, over 7 values of `g` | **0** |
| F3 one-pole, `c = 0`, over 4 values of `x` | **0** |
| F5 matrix, `lr = 0, ll = 1`, over 4 values of `r` | **0** |

Zero. Not "within tolerance" — the two contracts produce the same bits on every one of those cases,
which is why a bypassed slot's pinned output does not move even though the engine's arithmetic did.

The sweep also counts the cases where *neither* form returns the input, classified by cause:
negative zero, non-finite input, and an intermediate `x·g` that overflows. All three are
pre-existing properties of the **fused** contract, and `kernels.rs:415` already documented the
first — "It does **not** preserve the sign of a zero `x` when `d` is non-zero, which is why an
effect with a signed-zero identity contract selects the dry value with a mask instead of relying on
`mix`." The audit confirms the unfused form reproduces that carve-out exactly rather than widening
it, and the classification leaves **zero unexplained cases**.

This doubles as the phase-4 re-verification the plan asked for. Phase 4's earned-silence claims are
observations and re-earn automatically under new bits, but its `−0.0` *reasoning* had to be checked
against the new arithmetic: every signed-zero corner of `gain_mix_step` and of the one-pole agrees
between the two forms, including the four corners where the result is a negative zero.

## Boundary 4 — the one difference in kind, and it is outside the domain

Everywhere in the operating domain the two forms differ by a rounding. There is exactly one place
they differ in kind: `fma` computes `a·b` exactly, so when the product overflows `f32` while
`a·b + c` is representable, the fused form returns a finite number and the unfused form returns an
infinity. That is not a rounding difference and no error bound covers it.

Swept over 1 × 10⁻³⁰ to 1 × 10³⁰, the F5 matrix family shows **1 496 finiteness disagreements** and
the F4 mix family shows none. Over the operating domain — signals bounded by ±4.0 (full scale plus
12 dB of headroom), matrix coefficients on [−1.5, 1.5] — both families show **zero**, across
7.3 × 10⁸ cases.

This is recorded as a boundary rather than folded into a bound because it is the honest limit of
the change: the contract is now unfused *given* that the render path stays inside its domain, and
the thing that keeps it there is the builtins clamp, the limiter ceiling and the fader law, not the
arithmetic. A future kernel that multiplies two unbounded quantities and relies on the sum coming
back finite would be relying on a property the engine no longer has.

## Boundary 5 — three corpora that did not move are part of the evidence

`miso_engine_math`'s Horner chains have been deliberately unfused since they were written
(`lane_math.rs`: "an `fma` Horner improves `exp2_lane` from 1.462 to 1.191 ulp and leaves
`log2_lane` unchanged, against a gate of 2 ulp"). The crate reached this ruling's conclusion first,
on the same reasoning, for one module. Soft clip has no `fma` at all — "the frozen graph has none,
and adding one would change every pinned bit" (`soft-clip/kernel.rs:34`).

Their digests are therefore **controls**: `M3_DIGESTS` (32 cases), `EXP2_DIGEST`/`LOG2_DIGEST`, and
`SOFT_CLIP_DIGESTS` (6 cases) must come through the re-pin byte-for-byte unmoved. A change there
would mean the contract change had reached code it has no business reaching, and it is a stronger
check than any of the digests that *do* move, because it cannot be satisfied by re-deriving.

## Per-family bounds and their red mutations

Every bound is measured on a model of the site written out in `tools/miso-engine-audit`, because the
fused arm no longer exists in the tree to measure. A model is only evidence if it is the thing it
models: the `conformance` pass runs the production kernels at `Scalar` width over the same input
and compares `to_bits`, and reports **0 mismatching words** for `svf_block`, `one_pole_block`,
`gain_mix_block` and `mix2x2_block`.

| family | bound (worst in operating domain) | red mutation | detected |
|---|---|---|---|
| F1 SVF state | oracle distance +3.06 dB sustained, +3.82 dB swept | `d1 + 1 ulp`; `d2 + 1 ulp` | RED, RED |
| F2 SVF mix | −79.4 dBFS between arms | output mix `+ 1 ulp` | RED |
| F3 one-pole | −138.6 dBFS; growth ≤ 3.89 dB | `y' + 1 ulp` | RED |
| F4 mix | −114.4 dBFS, 0 finiteness disagreements | result `+ 1 ulp` (general path only) | RED |
| F4 identity | 0 mismatches / 2³² × 7 | break `mix = 0` only | RED |
| F5 matrix | −120.4 dBFS, 0 finiteness disagreements in domain | result `+ 1 ulp` | RED |
| F3 identity | 0 mismatches / 2³² × 4 | break `c = 0` only | RED |

The two identity mutations are deliberately orthogonal to the divergence mutations: they perturb
**only** the `mix = 0` / `c = 0` path, so they leave the quoted divergence bound unmoved and are
caught by the identity check alone. That separation is what proves the two claims are independently
gated rather than one claim counted twice.

Detection is by digest over the whole trajectory, not by comparing maxima. An earlier form of this
harness compared maxima and reported the F2 mutation as GREEN: shifting every sample by one step
moves the extremum by one step too, and the dominant sample happened to lie on the side that made
the reported maximum *smaller*. A maximum is not a detector.

**Measurement boundary.** All numbers are `f32` against the crate's `f64` oracles
(`ReferenceSvf`, and the same recurrence restated in `f64` for the one-pole), which use no
multiply-add primitive and are therefore unaffected by the contract. Inputs are SplitMix64
deterministic bipolar noise at full scale — broadband and worst-case for a rounding study, but not
music; the listening qualification is the arm that speaks to programme material. The SVF sweep runs
20 000 frames per design, which is several settling times for every design in it except the very
lowest-frequency, highest-Q corners; those corners are covered separately by the 1 000 000-frame
sustained arm, which is why both are reported. The F4/F5 grids are stratified dense sweeps, not
exhaustive — only the identity sub-domains are exhaustive, and the ruling does not claim otherwise.
The `f64` unfused oracle `softfma::unfused_mul_add_via_f64`, used by the graph and graph-compiler
matrix tests, is exact for normal results only; double rounding into the subnormal band can differ,
and what keeps those tests out of that band is the D7 flush law, not the oracle.

## What would justify reopening

* **An `fma_exact` for a specific site**: only on evidence that some site's *output* — not its
  intermediate — carries an error that matters at its audible level. Boundary 1 rules out the
  stability argument permanently; a reopening has to be an audibility argument with a measurement
  behind it. The emulation and its round-to-odd proof are recoverable from history
  (`crates/miso-engine-lane/src/softfma.rs` before this change) and would return as a separately
  named operation with a registry-style seal, not by un-retiring `Lane::fma`.
* **The finiteness boundary** (boundary 4): if a future kernel is written whose product can
  legitimately overflow while its sum cannot, that kernel needs either a domain argument or an
  exact path, and this ruling does not cover it.
* **A per-backend split**: never. The cross-backend `to_bits` identity across
  Scalar/Simd4/Simd8/wasm-scalar/wasm-simd128 is the property the lane crate exists to hold, and
  reintroducing hardware fusion "only where it is free" is exactly the split that made the wasm leg
  a different engine.
* **The control groups moving** (boundary 5): if `M3_DIGESTS`, `EXP2`/`LOG2` or `SOFT_CLIP_DIGESTS`
  ever move under a change that claims to be confined to the fma contract, the claim is wrong.

## Links

* Ruling: issue #163 phase 2, owner GO 2026-08-26 and the confirmation of the same date; the
  wasm baseline it rests on, `artifacts/issue163-phase2-wasm-baseline/`.
* Audit instrument: `tools/miso-engine-audit/src/unfused_fma.rs` (subject `unfused-fma`, modes
  `dense`, `exhaustive`, `mutations`, `conformance`, `all`).
* Contract: `crates/miso-engine-lane/src/wide_impl.rs` and `crates/miso-engine-lane/src/scalar.rs`
  (the two dispatch points), `crates/miso-engine-lane/src/lib.rs` (`Lane::fma`),
  `crates/miso-engine-lane/src/softfma.rs` (what survives, and why it kept its name).
* Independent restatement: `crates/miso-engine-dsp-reference/src/tpt.rs`
  (`ReferenceRetainedTptF32`), moved in lockstep; `softfma::unfused_mul_add_via_f64`.
* Seal: `scripts/check-unfused-seal.sh` and `scripts/test-unfused-seal.sh`.
* Measurements: `artifacts/issue163-phase2/`.
* Precedent: `crates/miso-engine-math/src/lane_math.rs` (unfused Horner, with its measurement);
  ceremony modelled on `docs/rulings/fast-db-tier-boundaries.md`.
