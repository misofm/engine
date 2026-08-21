# Sol implementation brief — issue 036 representable TPT cutoff domain and builtin contract acceptance

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** Use the landed issue-034 correction checkpoint as input. This issue
has exactly one Terra implementation/review attempt and at most one bounded Sol correction/review.
A second failure stops. Do not implement qualification tooling, SIMD, listening or benchmarks.
`timed_benchmark_invocations=0` is invariant.

## Frozen numerical contract

Keep the existing `f64` coefficient calculation, one-time `f32 c1/a2/a3/k` casts, strict Jury
checks, cast-state cutoff-response gate and incremental non-fused `f32` recurrence unchanged.
Change only the public/preparation cutoff domain to exact zero disabled or inclusive
`[10 Hz, maximum_hz(rate)]`:

| Rate | Maximum (`f32::to_bits`) | First excluded (`f32::to_bits`) |
| ---: | --- | --- |
| 44,100 | `22049.482421875` (`0x46ac42f7`) | `22049.484375` (`0x46ac42f8`) |
| 48,000 | `23999.43359375` (`0x46bb7ede`) | `23999.435546875` (`0x46bb7edf`) |
| 88,200 | `44098.96484375` (`0x472c42f7`) | `44098.96875` (`0x472c42f8`) |
| 96,000 | `47998.8671875` (`0x473b7ede`) | `47998.87109375` (`0x473b7edf`) |

The maximum is the greatest contiguous shared HPF/LPF boundary, not the last isolated value that
happens to pass after quantization creates a failure pocket. The numerical reason is that TPT
prewarping `tan(pi*f/Fs)` becomes unbounded at Nyquist while stored state-transition coefficients
are quantized to `f32`; see [RBJ-COOKBOOK], [SIMPER-SVF], [ZAVALISHIN-TPT], [SMITH-SASP] and
[ORFANIDIS-ISP]. Do not retain the impossible immediate-predecessor promise or alter coefficients
to force it through.

## Required implementation shape

1. Introduce a versioned rate-keyed cutoff-domain representation whose exact maximum bits are
   inspectable. Unsupported rates return the existing typed rate diagnostic.
2. Put cutoff-domain validation in one helper used by descriptor validation and preparation.
   Above-maximum values must return `FilterCutoff`/`builtin.filter.cutoff` before coefficient work.
3. Retain IDs, scope, mappings, defaults, reset/update behavior and exact-zero disable semantics.
4. Update only the cutoff seam of the composite matrix. Keep seed `0x000000034007c10`, exactly
   10,000 cases and every landed class; freeze a new transcript hash only after independent pass.
5. Append candid evidence to issue 036. Do not rewrite issue 034's failed record.

## Frozen boundary tests

For both HPF and LPF at all four rates, test zero, 10 Hz, maximum-minus-one-ULP, maximum, successor,
Nyquist predecessor, Nyquist, below-10 nonzero, NaN and infinities. Exact maximum bits pass.
Successor and every greater invalid boundary produce the exact cutoff diagnostic and parameter
path. Descriptor and `BuiltinChain::new` agree.

Enumerate ordered positive-`f32` bits from `0.45*Fs` through the table maximum for both sections;
all pass, and the listed successor is the first shared-domain exclusion. Retain the existing
lower-frequency analytic/impulse/sustained response gates. This is a bounded deterministic test,
not a benchmark.

## Exact ordered gates

1. `cargo fmt --check`
2. `cargo test -p miso-engine-builtins --lib descriptor`
3. `cargo test -p miso-engine-builtins --lib tpt`
4. `cargo test -p miso-engine-builtins-compiler --lib deterministic_builtin_compiler_mutation_matrix_has_exactly_ten_thousand_cases -- --nocapture`
5. `cargo test -p miso-engine-builtins-compiler --features test-support --test allocation_tracker`
6. `cargo test -p miso-engine-graph-compiler --lib each_forged_builtin_seal_tuple_is_rejected_before_graph_attachment`
7. `cargo test -p miso-engine-graph-compiler --doc`
8. `cargo check --workspace`
9. `cargo test --workspace --locked`
10. `RUSTFLAGS='-Dwarnings' cargo clippy --workspace --all-targets -- -D warnings`
11. `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`
12. `bash scripts/test-builtins-policy.sh`
13. `bash scripts/check-builtins-policy.sh`
14. `bash scripts/check-graph-policy.sh`
15. `bash scripts/check-realtime-policy.sh`
16. `bash scripts/check-workspace-policy.sh`
17. `bash scripts/check-graph-determinism.sh`
18. `bash scripts/check-builtins-targets.sh`

If an exact named filter matches no tests, correct the filter or run the containing lib suite and
record the substitution before review. Do not invoke `scripts/run-builtins-benchmark.sh` or any
timed binary. Final evidence must state `timed_benchmark_invocations=0` and identify no artifact.

## Sol review stop conditions

FAIL immediately if the implementation changes coefficient/recurrence/threshold semantics,
accepts an above-table value under the descriptor, rejects an in-domain seam value, weakens a
matrix/resource/opacity gate, changes the 10,000-case cardinality, or enters issue-035 scope. A
PASS unblocks issues 008 and 035 but is not machine qualification, listening evidence or launch
approval.
