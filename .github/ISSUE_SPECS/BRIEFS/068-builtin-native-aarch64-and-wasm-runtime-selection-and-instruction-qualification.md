# Sol implementation brief — issue 068 builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Decision and attempt budget

**READY only after exact-title PASS of Quiescent builtin graph retirement-worker trace closure.**
Consume that exact clean candidate and its preserved Issue-069 direct/functional evidence. Permit one Terra implementation attempt and one bounded Sol
correction/review; a second failure stops. This issue performs no audit million-run, functional
workload, benchmark, timing, device run, browser run or listening. Both
`workload_invocations` and `timed_benchmark_invocations` start and remain zero.

## Frozen candidate and artifact isolation

Before any build, require a clean candidate and record its commit identity. Create a canonical,
sorted source manifest whose rows are `path<TAB>byte_length<TAB>sha256` and hash those bytes. It
includes root `Cargo.toml`, `Cargo.lock`, any `.cargo` configuration, both target/instruction
scripts, and every manifest/Rust source under these exact packages:

- `miso-engine-core`;
- `miso-engine-builtins`;
- `miso-engine-builtins-compiler`;
- `miso-engine-graph`; and
- `miso-engine-graph-compiler`.

Record `Cargo.lock` separately. Repeat the source and lock seals after all gates and require byte
identity. Bind the read-only corpus to these exact SHA-256 values:

- `fixtures/builtins/v1/MANIFEST.tsv`:
  `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
- `fixtures/builtins/v1/pcm/graph-taps.f32le`:
  `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
- `fixtures/builtins/v1/meters/graph-taps.jsonl`:
  `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.

The target driver creates one fresh empty directory with
`mktemp -d "${TMPDIR:-/tmp}/miso-engine-issue068.XXXXXX"`, installs a cleanup trap, and gives every
matrix leg its own child `CARGO_TARGET_DIR`. It may not read or select an object from repository
`target/` or a prior leg. For each object gate, enumerate matching current-package objects and
require exactly one before hashing it; extract the named symbol and require exactly one matching
symbol body. Zero or multiple matches fail.

## Exact runtime-selection matrix

Run exactly all 16 tuples of
`TargetCapabilities::from_detected(wasm_simd128, aarch64_neon, x86_avx2, x86_fma)`. Both
`KernelBackendV1::select` and `KernelDispatch::select` must implement this precedence:

1. `x86_avx2 && x86_fma` selects `X86Avx2Fma`, width 8;
2. otherwise `x86_avx2` selects `X86Avx2`, width 8;
3. otherwise `aarch64_neon` selects `Aarch64Neon`, width 4;
4. otherwise `wasm_simd128` selects `WasmSimd128`, width 4; and
5. otherwise select `Scalar`, with no bank width.

Thus FMA without AVX2 never selects an x86 FMA backend. Record all four input booleans, selected
backend and width for each row. `target_capabilities()` and executable-backend validation occur
only during preparation; prepared render retains the selection and makes no capability query.
Scalar preparation always succeeds. An unavailable backend request returns the existing typed
`BackendUnavailable`; it must not silently execute a different backend. On the native x86
qualification host, actual capability selection must equal the detected tuple. AVX2 and
AVX2+FMA execution rows are mandatory for this native qualification; lack of either facility is a
reported host-precondition STOP, not a skipped PASS.

## Four-rate correctness

Preserve exactly the launch rates 44,100, 48,000, 88,200 and 96,000 Hz. Use the accepted read-only
corpus and current public builtin/bank path; do not generate fixtures. At every rate, compare
representative nonidentity HPF+LPF dual-mono bank PCM, carried state and report values against the
scalar path over consecutive blocks. Native AVX2 non-FMA finite-normal/no-sanitation output and
state are bit-identical to scalar. AVX2+FMA uses the already accepted bound
`abs(error) <= 1e-6 + 2e-5 * abs(scalar)` and may not change production operations, coefficients,
fixtures or tolerance. Keep lane/state isolation and scalar-tail behavior represented. Record the
four deterministic result-row hashes. Cross-target builds do not claim numerical execution.

## Build matrix

Use `--locked --release` throughout and compile the exact five-package closure plus transitive
dependencies:

- native scalar baseline with `-C target-feature=-avx2,-fma`;
- native x86 object legs with `+avx2,-fma` and `+avx2,+fma`;
- `aarch64-linux-android` and `aarch64-apple-ios` release checks;
- `wasm32-unknown-unknown` scalar with `-simd128`; and
- `wasm32-unknown-unknown` base SIMD with `+simd128`.

Do not add Cargo features named `simd128`, `neon`, `avx2` or `fma`, a global target feature, target
CPU setting or relaxed-SIMD requirement. Android/iOS/Wasm evidence is compile/object inspection
only. It is not device, browser, AudioWorklet, realtime or listening evidence.

## Named TPT instruction contract

Use the current `scripts/check-rack-instructions.sh` structure, strengthened by the isolated
artifact rules above. Inspect only these frozen TPT symbols and operation contracts:

| Leg | Exact symbol suffix | Required | Forbidden |
| --- | --- | --- | --- |
| native scalar | `scalar::process_tpt_scalar` | one symbol body | packed AVX/FMA |
| AVX2 | `x86::process_tpt_x86_avx2_inner` | `vmulps`, `vaddps`, `vsubps`, `%ymm` | any fused op |
| AVX2+FMA | `x86::process_tpt_x86_avx2_fma_inner` | exactly three sites: one each `vfmsub`, `vfmadd`, `vfnmadd` | any fourth fused site |
| AArch64 NEON | `process_tpt_aarch64_neon_inner` | `fmul`, `fadd`, `fsub` on `.4s` vectors | `fmla`/`fmls` |
| Wasm scalar | no SIMD TPT symbol | scalar opcodes only | `f32x4`, `v128`, relaxed SIMD |
| Wasm simd128 | `process_tpt_wasm_simd128_inner` | `f32x4.mul`, `.add`, `.sub` | relaxed SIMD |

Hash the selected object and extracted symbol body for every leg. Inspection of the whole object
cannot substitute for named-symbol extraction, and a symbol-name grep cannot substitute for
opcode validation.

## Ordered gates and stop conditions

1. Candidate/source/lock and three frozen fixture hashes pass before any build.
2. The 16-row selection/width matrix, actual native tuple and unavailable-backend tests pass.
3. The native four-rate scalar/AVX2/FMA rows pass without tolerance or DSP changes.
4. The isolated five-package target matrix and all six named instruction/opcode rows pass.
5. Run read-only checked-corpus validation; format; locked check/tests for the exact package
   closure; warning-denied all-target/all-feature Clippy and rustdoc for that closure; applicable
   workspace, realtime, rack and builtin policy checks/mutations; shell syntax; and static
   no-artifact/no-workload scans.
6. Recompute candidate/source/lock/corpus hashes and require exact equality with step 1.

Stop on a dirty or changing candidate, stale/ambiguous object, missing target/toolchain,
unavailable mandatory native AVX2/FMA host, selection mismatch, four-rate numerical mismatch,
unexpected opcode/fusion, cross-target execution claim, corpus change or second failed attempt.
Do not repair DSP, regenerate fixtures, add a target abstraction, run #69/#70 audits, touch benchmark
or listening tools, or broaden into host adapters.

## Required evidence and verdict

Record the Issue-070 PASS commit and preserved Issue-069 audit identities; candidate commit; before/after source-manifest
and `Cargo.lock` hashes; unique scratch basename; the three frozen corpus hashes; 16 selection rows;
four correctness-row hashes; target triples/features/tool versions; per-leg object and symbol hashes;
exact command outcomes; Terra/Sol attempt count; strict PASS/FAIL; `workload_invocations=0`; and
`timed_benchmark_invocations=0`.
