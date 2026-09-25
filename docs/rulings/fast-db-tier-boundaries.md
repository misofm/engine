# The fast dB tier: where it wins, where there was nothing to win, and the instrument that lied

**Candidate.** Issue #144 item 5 / #149 workplan item 2 — "a sealed fast f32 `db_to_gain`/`log2`
tier confined to the dynamics gain path — exactly N named f32 crossings pinned by independent
restatement, token-scan seal refusing fast vocabulary elsewhere. v1 ran full dynamics at 16–17
cycles/ch-sample vs our ~120 cycles/lane-sample compressor; the scalar dB path is our recorded
#88/#89 cost center."

**Status.** Adopted, at eight named crossings, measuring 22.64% on the standing sixty-four-track
console fixture. Three boundaries are recorded here: one effect where the predicted win did not
exist at all, one design choice that was deliberately *not* taken, and one measuring instrument
that was wrong by a factor of seventeen and would have killed the optimisation if it had been
trusted.

## Boundary 1 — the premise was already half-spent, and the recorded cost centre had moved

The candidate cites #88 and #89, which measured the compressor at ~120 cycles per lane-sample and
found four libm calls (`log10f`, two `expf`, `powf`) on every one of them. That is not the engine
this tier landed in. Those findings were acted on before this sprint: the dynamics gain path
already ran `math::exp2_lane`/`log2_lane` — Cephes polynomials in `Lane` basic
operations, no libm, qualified at 2 ulp by gate M1 — through
`effect_runtime::dynamics::level_db`/`gain_from_db`.

So the win recorded below is **not** the win #88/#89 predicted. Their win (delete the libm calls,
recover cross-target bit identity) had already been banked. What remained was a second, smaller
question that neither issue asked: given that the conversion is already a portable polynomial, is
it the *right degree* for a detector? The answer was no, and that is worth 22%.

The lesson is about reading a banked finding before implementing it. A cost centre recorded at one
commit is a claim about that commit. This one had been half-resolved in the intervening work, and
the surviving half was a different problem with a different fix.

## Boundary 2 — the true-peak limiter has no dB crossing to make

The brief names four dynamics effects: compressor, gate/expander, limiter, multiband. The limiter
takes **zero** crossings, because it runs no per-sample decibel conversion to replace. Issue #90 F1
had already moved it to a linear-domain ramp: `limit` and the release coefficient are designed in
`f64` at event time, and the per-frame work is `select`, `div`, `floor`, `mul`, `add`, `sub`, `fma`,
`max` — no transcendental of any kind. Its observation tap publishes the linear reduction word
precisely so that no logarithm reaches the render thread.

This is a null, and it is the useful kind: the limiter was expected to pay and it had already
collected. Confirmed in the shipped wasm binary, where its kernel's arithmetic count is unchanged
at 124 vector / 0 scalar across this whole change.

The transient shaper is a separate case. Approved R2 admits its detector contrast and applied gain
as X7 and X8, after MA-5 proved their domains and exhaustive error bounds. Its `fast_level_db` and
`fast_gain_from_db` call sites now carry the same named-crossing `#[expect]` seal as X1–X6.

## Boundary 3 — the shared runtime helpers were deliberately not converted

The obvious implementation is a one-line change: point
`effect_runtime::dynamics::level_db`/`gain_from_db` at the fast tier and every dynamics effect
crosses at once. It was rejected, and the reason is worth keeping.

Those two helpers are shared, and they are their own pinned corpus cases (`D1_DIGESTS`, rows
`level_db` and `gain_from_db`). Converting them would have moved those nine runtime rows and,
through them, the parametric EQ, soft clip, builtins, delay and transient-shaper digests — five
effect families re-pinned, for no measured gain, because none of them is a dynamics gain path.
It would also have put the fast tier behind an exact-tier spelling, where the seal could not see
it: a future caller of `dynamics::level_db` would silently get the cheaper conversion.

Instead the crossings call `math::fast_db` directly and the shared helpers stay exact.
The evidence that this was the right cut is in the multiband corpus: five of its six cases —
`lr4_step/low`, `lr4_step/high`, `branching_smooth`, `link_levels/maximum`,
`link_levels/average` — are **byte-for-byte unmoved**, and only `band_amplitude` changed.

## Boundary 4 — the isolated microbenchmark was wrong by 17x, in the safe direction

This is the one to remember.

The first instrument pointed at this optimisation was a tight-loop throughput benchmark of the two
conversions on their own: load a vector, call `exp2_lane`, accumulate; then the same with a
candidate fast version. It measured a combined saving of **0.22 ns per lane-sample**, which over
the 16,384 lane-samples in a sixty-four-track block projects to 3.6 µs against a 287 µs block —
**1.3%**. Against the cost of the change (a sealed module, a token-scan seal, a corpus-wide re-pin,
a permanent accuracy trade) that is a clear reject, and the honest next step looked like writing a
null ruling.

The in-situ measurement is **22.64%**.

The microbenchmark was not noisy or badly written; it was measuring a different thing. Its loop
body was `load`, one polynomial, one accumulate, over independent chunks, so the out-of-order
engine had a full reorder window and no register pressure: every polynomial evaluation overlapped
its neighbours almost perfectly. Inside the real kernel the same polynomial competes for registers
with ring cursors, per-lane coefficients, link masks and the smoother's recurrence, and its fifteen
live constants have to be rematerialised or spilled. The exact tier's cost is not the throughput of
its instructions; it is what those instructions do to the kernel around them.

**The rule this establishes.** A per-sample kernel component may not be sized by an isolated
throughput loop. The arm has to be the real kernel, on the real fixture, alternated against the
real alternative. Had the 1.3% projection been trusted, a 22% win on the standing benchmark fixture
would have been written up as a null and closed.

## What was adopted, and what it measures

Eight named crossings — compressor (X1, X2), gate/expander (X3, X4), multiband compressor (X5, X6),
and transient shaper (X7, X8) — onto `math::fast_db`, whose two polynomials are fresh minimax fits of degree 4 (`exp2`)
and 5 (`log2`) with no range-reduction fold, replacing Cephes degree 6 and 9 with folds.

| workload | exact tier | fast tier | delta |
|---|---|---|---|
| nine-track EQ-only baseline (control) | 13.245 | 13.260 | −0.11% |
| nine-track ragged strip | 44.950 | 36.114 | **19.66%** |
| sixty-four-track console | 287.521 | 222.438 | **22.64%** |
| 128-track stretch | 578.443 | 449.205 | **22.34%** |

**Measurement boundary.** Fixtures `fixtures/session/v1/parametric-eq-nine-track.toml` and
`fixtures/session/v1/console-sixty-four-track.toml`, 48 kHz, 128-frame quantum, 1000 observations,
one warmup pass and two measured rounds, descriptive only, no threshold. Runner
`scripts/run-console-benchmark.sh --phase2`; records
`artifacts/issue149/console-benchmark.accepted.jsonl` (exact) and
`artifacts/issue149-phase2/console-benchmark.accepted.jsonl` (fast). These are two single-arm runs,
not a paired alternation — the exact tier is no longer in the tree by phase 2. The paired arm was
run before the crossings landed, with two binaries differing only in `dynamics.rs`, alternated
launch by launch, eight rounds each: 282.087 → 219.582 µs (22.16%), decomposing to 11.65% for
`log2` alone and 9.80% for `exp2` alone against a combined 21.48%.

The accuracy traded for it, proven exhaustively by gate F1 rather than argued: at most `2.810e-5` dB
on a detector level and `7.431e-6` dB on an applied gain, which is 1.83x and 1.06x the *exact*
tier's own measured error over the same domains. Under a factor of two, everywhere the dynamics
path can reach.

## What would justify reopening the rejected parts

* **The limiter** (boundary 2): only if a future limiter topology puts a decibel conversion back on
  its per-sample path. As long as its ramp is linear-domain there is nothing here for it.
* **The transient shaper**: R2 approved X7 (detector contrast) and X8 (applied gain) after MA-5
  established both domains and showed a maximum fast-versus-exact applied-gain change of
  `1.654115e-5` dB. The source digest pins move once for those crossings.
* **The shared runtime helpers** (boundary 3): only if a measurement shows a non-dynamics consumer
  of `dynamics::level_db` that is both hot and tolerant of detector-grade accuracy. None exists
  today.
* **A lower degree still** (`exp2` degree 3, `log2` degree 4): F1 exhaustively remeasured the
  refitted candidates over its operating domains. With P degree 3 coefficients, highest order
  first, `[0x3c5bf2e2, 0x3d55ffe6, 0x3e7711ca, 0x3f316b63]`, the applied-gain error is
  `3.210375e-5` dB over `[-160, -0]` and `2.662959e-5` dB over `[0, 24]`, above the `1.0e-5` dB
  gate. With Q degree 4 coefficients `[0x3d3e0145, 0xbe48fcca, 0x3ed5d00c, 0xbf35aca2,
  0x3fb89252]`, the level error is `1.005557e-4` dB over `[1e-8, 16]`, above the `4.0e-5` dB
  gate. These are measurements from the same oracle and full-domain sweep as the shipped rows.
  Separately, the real-coefficient LP minimax lower bounds, derived by the research harness, are
  `2.52e-5` dB for P degree 3 and `8.65e-5` dB for Q degree 4. Those derivations show that no f32
  refit can meet either gate. The degree is not slack.

## Links

* Optimisation: issue #144 item 5, issue #149 workplan item 2; the folded-in cost-centre findings
  #88 F1 and #89 F1.
* Implementation: `crates/math/src/fast_db.rs` (the sealed tier),
  `crates/math/tests/f1_fast_db_bounds.rs` (gate F1, exhaustive),
  `clippy.toml`'s `disallowed-methods` plus a per-crossing `#[expect(clippy::disallowed_methods)]`
  (the container; formerly `scripts/check-fast-db-seal.sh` and `scripts/test-fast-db-seal.sh`,
  retired once the migration was mutation-proven).
* Crossings: `compressor/src/kernel.rs`, `gate-expander/src/kernel.rs`,
  `multiband-compressor/src/lib.rs`, `transient-shaper/src/lib.rs`.
* Gates: `scripts/run-wasm-gates.sh` (133 cases, 331 comparisons, 0 mismatches on all three legs).
* Bench protocol: `AGENTS.md` "Benchmarks are descriptive during feature development"; issue #104;
  `tools/bench/src/console.rs` "Paired alternation".
* Prior ruling in this sprint: `docs/rulings/stationary-smoother-hoist-boundary.md`.
