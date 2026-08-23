# 036 Representable TPT cutoff domain and builtin contract acceptance

## Outcome

Replace the impossible open-Nyquist builtin-filter domain with one truthful representable domain
at each launch rate, then accept the already-landed issue-034 contract corrections through their
final deterministic compiler and nonbenchmark gates.

## Context

Engine V2 is greenfield and must not inspect or inherit V1. The render thread remains allocation,
lock, I/O, syscall, logging and structural-mutation free. Audio and retained TPT coefficients,
state and operations are `f32`; coefficient calculation and validation happen off render.

Issue 034 exhausted its two attempts without PASS. Its bounded correction checkpoint `9c57af8`
landed the metadata types, sealed-only graph attachment, checked exact resource accounting and the
composite 10,000-case compiler matrix. That matrix exposed one remaining contradiction: the public
domain accepted every finite `f32` below Nyquist, but the retained cast-coefficient Jury and cutoff-
response gates reject representable values before Nyquist. Issue 034 is stopped; this issue alone
owns that numerical boundary and the final acceptance of its landed corrections.

This issue has at most **two total attempts**: Terra attempt 1 and one bounded Sol correction and
review. A second failure stops and requires a new stateless rescope. No timed workload or benchmark
is in scope; `timed_benchmark_invocations=0` must remain unchanged.

## Scope

- Replace `DisabledOrRateBoundedHertz` with a versioned domain whose inclusive maximum is an exact
  rate-keyed `f32` value, and use one shared helper for descriptor and preparation validation.
- Preserve numeric parameter IDs, mappings, defaults, update/reset behavior and exact-zero disable.
- Preserve the accepted conditioned incremental non-fused all-`f32` TPT coefficients, recurrence,
  strict cast-state Jury test and `-3.0102999566 +/- 0.005 dB` preparation gate.
- Update the composite deterministic 10,000-case matrix at the frozen cutoff seam and run the full
  issue-034 nonbenchmark acceptance gates against one candidate.

## Required public interfaces/contracts

HPF/LPF cutoff is exactly `0 Hz` (disabled), or finite `10 Hz <= f <= maximum_hz(rate)`. The
inclusive launch-rate maxima and first excluded representable values are:

| Rate (Hz) | Inclusive maximum Hz (`f32` bits) | First excluded Hz (`f32` bits) |
| ---: | --- | --- |
| 44,100 | `22049.482421875` (`0x46ac42f7`) | `22049.484375` (`0x46ac42f8`) |
| 48,000 | `23999.43359375` (`0x46bb7ede`) | `23999.435546875` (`0x46bb7edf`) |
| 88,200 | `44098.96484375` (`0x472c42f7`) | `44098.96875` (`0x472c42f8`) |
| 96,000 | `47998.8671875` (`0x473b7ede`) | `47998.87109375` (`0x473b7edf`) |

These are the greatest **contiguous shared** HPF/LPF maxima: both sections prepare for every
representable near-Nyquist value through the listed maximum, and the next `f32` is the first value
at which HPF preparation fails. Later isolated pass/fail pockets caused by coefficient
quantization do not enlarge a truthful interval domain. Unsupported rates remain rejected by the
launch-rate authority. Values above the table maximum, including the immediate predecessor of
Nyquist, return `builtin.filter.cutoff` at the exact parameter path before coefficient preparation;
they are not promised and then rejected as `builtin.filter.coefficients`.

## Numerical decision and evidence basis

The bilinear/TPT prewarp is `g=tan(pi*f/Fs)`; `g` grows without bound as `f` approaches Nyquist.
The retained implementation casts `c1/a2/a3/k` to `f32`, then applies strict Jury inequalities and
checks cast-state cutoff response. Finite coefficient quantization therefore makes “every `f32`
strictly below Nyquist” a stronger and false promise. This follows the bilinear prewarp and TPT
derivations in [RBJ-COOKBOOK], [SIMPER-SVF] and [ZAVALISHIN-TPT], with finite-precision stability
cross-checked against [SMITH-SASP] and [ORFANIDIS-ISP].

The chosen correction is metadata/domain truthfulness, not a DSP change. Exact production-code
diagnostics found the first shared rejection by ordered positive-`f32` bits from `0.45*Fs` toward
Nyquist; the table records the preceding value. Acceptance tests must repeat that exhaustive seam
scan for both HPF and LPF, prove the exact bits, and retain the existing below-seam response suite.

## Deliverables

- versioned rate-keyed cutoff domain and one shared typed validation path;
- exact table/boundary tests for all four rates and both filter kinds;
- passing final 10,000-case composite compiler matrix with its new transcript hash;
- passing inherited descriptor, opacity, corruption, resource, target and policy gates; and
- a checksummed nonbenchmark evidence record with `timed_benchmark_invocations=0`.

## Explicit non-goals

Changing TPT coefficient equations, cast precision, response/Jury thresholds, render operations,
matrix/pan/meter semantics, graph topology/PDC, resource formulas, fixtures, million-call audits,
SIMD kernels, human listening, performance work, or any benchmark invocation.

## Dependencies by exact issue title

- Issue-007 launch-critical builtin contract closure
- Dual-mono builtins and metering
- DSP research corpus and conformance harness
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/036-representable-tpt-cutoff-domain-and-builtin-contract-acceptance.md`.
It freezes the table, two-attempt budget, shared validation behavior, final matrix and zero-
benchmark scope.

## Hazards/decisions

Do not compute a platform-dependent maximum at runtime, publish Nyquist minus an arbitrary epsilon,
or define the maximum as the last isolated accepted value. Store/compare exact rate-keyed bits.
Descriptor validation and `BuiltinChain` preparation must not duplicate different inequalities.
The boundary change must not reopen issue-034 opacity/resource work or issue-035 qualification.

## Acceptance gates with objective measurements

At every launch rate, exact zero and `10 Hz` pass; both sections pass at the exact listed maximum;
the next `f32`, Nyquist predecessor, Nyquist, infinities, NaN and below-10 nonzero values reject as
`builtin.filter.cutoff` at exact paths. An ordered-bit scan proves both sections accept every
representable value from `0.45*Fs` through the maximum and that the listed successor is the first
shared-domain failure. Descriptor and preparation results agree at every boundary.

The deterministic compiler mutation test executes exactly 10,000 cases with seed
`0x000000034007c10`, retains all frozen composite classes, replaces the impossible predecessor-
acceptance case with maximum/successor checks, and passes with a frozen updated transcript hash.
All issue-034 compile-fail opacity, eight-category seal corruption, exact 3x3 resource layout/cap,
workspace, target and policy gates pass. No timed command runs and no benchmark artifact is made.

## Target matrix

Native scalar test; compile checks for `aarch64-linux-android`, `aarch64-apple-ios`, and
`wasm32-unknown-unknown` with `-simd128` and `+simd128` through the inherited target gate.

## Required evidence

Exact descriptor dump and boundary bits; exhaustive seam result; final matrix seed/count/hash and
diagnostic summary; inherited opacity/corruption/resource logs; target/workspace/policy logs;
candidate hashes; explicit `timed_benchmark_invocations=0`; and a Sol PASS/FAIL verdict.

## Terra attempt 1 evidence (2026-08-21)

Candidate input was `00a59db`. `BuiltinParameterDomain` now uses
`DisabledOrRateKeyedHertzV1`; `builtin_filter_cutoff_maximum_hz_v1` exposes the frozen launch
table, and `validate_builtin_filter_cutoff_v1` is shared by descriptor containment, `BuiltinChain`
preparation, and compiler diagnostic-path selection. The exact descriptor/preparation maxima are
44,100: `0x46ac42f7`, 48,000: `0x46bb7ede`, 88,200: `0x472c42f7`, and 96,000:
`0x473b7ede`. Their immediate ordered-`f32` successors reject as `builtin.filter.cutoff` before
coefficient preparation, at the exact HPF or LPF parameter path. Exact positive zero remains the
only disabled encoding.

`representable_cutoff_domain_is_shared_by_descriptors_and_preparation` covers zero, 10 Hz,
maximum-minus-one ULP, maximum, successor, Nyquist predecessor, Nyquist, below-10 nonzero, NaN
and infinities for both sections at every launch rate. The deterministic
`representable_cutoff_seam_is_contiguous_for_both_tpt_sections` scan enumerates every ordered
positive `f32` from `0.45 * Fs` through the maximum for each HPF and LPF; all prepare under the
retained coefficients, and each listed successor is the first shared-domain exclusion. It passed
in 1.68 s as a correctness test, not a timed benchmark.

The composite matrix retained seed `0x000000034007c10`, all 49 classes and exactly 10,000 cases.
It now exercises isolated HPF/LPF maximum acceptance and successor rejection; its frozen
transcript hash is `17626955350904343931`. The inherited descriptor, TPT response, allocation,
seal-opacity, compile-fail, workspace, locked-workspace, warning, documentation, policy,
fresh-process determinism (100/100), and target matrix gates all passed. No TPT coefficient,
recurrence, Jury/response threshold, render operation, resource formula, fixture/audit, SIMD,
listening, or benchmark change was made.

Source-file candidate SHA-256 values before the root checkpoint are
`db8cd8cadd59f2626d77254149b80fa37b2f3a2447e1ea0afaeba60d2288f000`
(`crates/miso-engine-builtins/src/lib.rs`) and
`3cc6917b586eee6d6f10d40ae722a4d575bbdf60d9c1c8e22caefdf62942602c`
(`crates/miso-engine-builtins-compiler/src/lib.rs`). `timed_benchmark_invocations=0`; no benchmark
command or artifact was created. **Terra verdict: PASS; Sol review pending.**

## Sol bounded correction and final verdict (2026-08-21) — PASS

Sol reviewed candidate input `14286c8` and used the one allowed bounded correction. The fallback
in `validate_builtin_filter_cutoff_v1` had admitted every nonlaunch rate despite being documented
only for the four issue-032 extended compatibility rates. It now delegates that classification to
the core extended-rate predicate: all four informational compatibility rates retain their prior
direct TPT evidence, while `0`, `32,000` and `192,001` reject. The seam test now proves directly
that each listed successor is the first underlying HPF `FilterCoefficients` failure after every
preceding representable value passed for both sections. A compiler test also proves maximum
acceptance and successor `builtin.filter.cutoff` rejection at the exact HPF/LPF path for all four
launch rates. No coefficient, cast, Jury, response, recurrence or render operation changed.

All 18 ordered nonbenchmark gates in the brief pass on Rust/Cargo 1.97.1, including the unchanged
10,000-case seed, 49 classes and transcript hash `17626955350904343931`; the full locked workspace,
allocation grid, seal corruption/opacity, warning-denied Clippy/rustdoc, policies, 100/100 fresh-
process determinism and target matrix also pass. Final source SHA-256 values are
`f93da3ca6d904d72286845704ac14259a21f9b2a7a4db47d22900453c5d02fa9`
(`crates/miso-engine-builtins/src/lib.rs`) and
`c0a3dd80c4ddd4e27304e5e5b83832aa98f16004211ed270fd10ddf395dc3027`
(`crates/miso-engine-builtins-compiler/src/lib.rs`). `timed_benchmark_invocations=0`; no benchmark
command ran and no benchmark artifact was created. **Sol verdict: PASS.**

## Note (2026-08-23, issue #85 / master plan #83)

Superseded **in part**. The table itself — `builtin_filter_cutoff_maximum_hz_v1`,
`validate_builtin_filter_cutoff_v1` and the four launch-rate maxima — is a contract fixture under
master plan §8.2 and is unchanged: the same values prepare, the same successors are rejected, and
`representable_cutoff_domain_is_shared_by_descriptors_and_preparation` still passes verbatim.

What is superseded is the *derivation of the seam*. The pre-#83 maximum was the first cutoff whose
cast coefficients failed a preparation-time Jury stability check, and that seam was set by the
`1 - c1` quantisation the check was fed. The kernel now stores `c1` directly and casts
`a2 = g / (1 + t)` and `a3 = g * g / (1 + t)` (master plan §4.2, amendment A1), so it is better
conditioned near Nyquist and every value through the table maximum — and beyond it — prepares. The
Jury check and the preparation-time cutoff-response gate are deleted; the public domain is enforced
by the table alone.

The replacement derivation is `representable_cutoff_domain_prepares_everywhere_and_rejects_successor`
in `crates/miso-engine-builtins/tests/response.rs`: every representable cutoff from `0.45 * fs`
through the maximum designs a section, its cast state-space transfer is `-3.0103 dB` at its own
cutoff to the frozen 0.005 dB tolerance, and the successor is rejected with `FilterCutoff` by the
domain. The table is not widened, no new maximum is computed, and no stability gate is re-added.
