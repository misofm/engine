# Engine environment and marker vocabulary

Every environment variable and every process marker this repository defines is spelled
`MISO_ENGINE_<SUBJECT>_<FACT>`. There is no second prefix and no second name for one fact. This
file is the vocabulary; `scripts/check-env-vocabulary.sh` enforces it and
`scripts/test-env-vocabulary.sh` proves that enforcement red.

Two rules, both mechanical:

1. **One prefix.** No tracked file may contain a `MISO_`-prefixed identifier that does not continue
   `MISO_ENGINE_`. Two paths are exempt and both have to be: this file, which has to be able to
   name a retired prefix in order to say it is retired, and `.github/ISSUE_SPECS/`, whose job is to
   record what a name used to be. No source, configuration or script file is exempt. Before #104 phase C there were seven other prefixes (`MISO_RT_`, `MISO_GRAPH_`,
   `MISO_ISSUE069_`, `MISO_039_`, `MISO_INTERCHANGE_`, `MISO_CAPI_`, `MISO_TEST_`, …) across 91
   distinct names.
2. **One name per fact, and it is listed below.** Every `MISO_ENGINE_*` identifier that appears
   under `tools/` or `scripts/` must be a row of one of these tables, and every row must appear
   under `tools/` or `scripts/`. A row that nothing uses is as much a defect as a name that is not
   a row: the second copy of a name is what let a runner and its binary disagree (#104 F2).

Names that appear only under `crates/` or `hosts/` -- the C ABI macro families
`MISO_ENGINE_V2_*` and `MISO_ENGINE_EFFECT_*_V1`, and the crate-local test hooks
`MISO_ENGINE_MATH_PIN`, `MISO_ENGINE_REPIN_MULTIBAND_CORPUS`, `MISO_ENGINE_TRANSCRIPT_031`,
`MISO_ENGINE_TRANSCRIPT_045`, `MISO_ENGINE_AUDIT_008`, `MISO_ENGINE_AUDIT_037`,
`MISO_ENGINE_WEB_ORACLE_PRINT` -- are bound by rule 1 but are not part of the tool/script
vocabulary and are not listed here.


## Benchmark identities

Set by `scripts/run-*-benchmark.sh` before the single launch; read by the bench binary and copied into its record. Every benchmark subject uses the same names.

| name | meaning |
|---|---|
| `MISO_ENGINE_BENCH_CANDIDATE_COMMIT` | 40-hex commit the candidate binary was built from. |
| `MISO_ENGINE_BENCH_CANDIDATE_TREE` | 40-hex tree of that commit. |
| `MISO_ENGINE_BENCH_CANDIDATE_SHA256` | sha256 of the candidate commit string (rack, scheduler). |
| `MISO_ENGINE_BENCH_BINARY_SHA256` | sha256 of the launched binary. |
| `MISO_ENGINE_BENCH_ROUND` | `warmup`, `1` or `2` for the runners that launch per round. |
| `MISO_ENGINE_BENCH_TOOL_SOURCE_SHA256` | sha256 of the bench tool source (interchange). |
| `MISO_ENGINE_BENCH_TOOL_MANIFEST_SHA256` | sha256 of the bench package manifest (interchange). |
| `MISO_ENGINE_BENCH_FIXTURE_MANIFEST_SHA256` | sha256 of the accepted fixture manifest (interchange). |
| `MISO_ENGINE_BENCH_HERMETIC_CHILD` | set by a test harness so a re-entered runner refuses to launch. |


## Benchmark host and toolchain metadata

One name per fact. Set by the runner, read by the bench binary; a name the runner does not set is reported in the record's `missing_metadata`.

| name | meaning |
|---|---|
| `MISO_ENGINE_BENCH_CPU_MODEL` | CPU model string. |
| `MISO_ENGINE_BENCH_CPU_ARCHITECTURE` | CPU architecture. |
| `MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT` | logical cores. |
| `MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT` | physical cores. |
| `MISO_ENGINE_BENCH_OS` | operating system. |
| `MISO_ENGINE_BENCH_KERNEL` | kernel release. |
| `MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE` | cpufreq governor or platform power mode. |
| `MISO_ENGINE_BENCH_POWER_SOURCE` | mains or battery. |
| `MISO_ENGINE_BENCH_RUST_VERSION` | `rustc -V`. |
| `MISO_ENGINE_BENCH_LLVM_VERSION` | LLVM version behind that rustc. |
| `MISO_ENGINE_BENCH_TARGET_TRIPLE` | build target triple. |
| `MISO_ENGINE_BENCH_TARGET_CPU` | `-C target-cpu` in effect. |
| `MISO_ENGINE_BENCH_TARGET_FEATURES` | enabled target features. |
| `MISO_ENGINE_BENCH_PROFILE` | cargo profile. |
| `MISO_ENGINE_BENCH_OPT_LEVEL` | opt-level. |
| `MISO_ENGINE_BENCH_LTO` | lto setting. |
| `MISO_ENGINE_BENCH_CODEGEN_UNITS` | codegen-units. |
| `MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE` | operator's declared background load. |
| `MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER` | Wasm runtime or browser identity. |
| `MISO_ENGINE_BENCH_WASM_HOST` | Wasm host name. |
| `MISO_ENGINE_BENCH_WASM_HOST_VERSION` | Wasm host version. |
| `MISO_ENGINE_BENCH_WASM_SCALAR_BYTES` | size of the scalar Wasm artifact. |
| `MISO_ENGINE_BENCH_WASM_SIMD_BYTES` | size of the simd128 Wasm artifact. |


## Benchmark phase marker

Written to stderr by every bench binary, counted by the runner. Not an environment variable. Payload words: `workload_started`, `warmup_complete`, `timed_started`, `round_<n>_complete`.

| name | meaning |
|---|---|
| `MISO_ENGINE_BENCH_PHASE` | the one bench phase marker. |


## Realtime trace markers

Written to stdout by an audit binary immediately outside its armed render scope, and matched by `scripts/validate-realtime-trace.sh` against the strace timestamps. Not environment variables.

| name | meaning |
|---|---|
| `MISO_ENGINE_RT_BEGIN` | realtime audit: armed. |
| `MISO_ENGINE_RT_END` | realtime audit: disarmed. |
| `MISO_ENGINE_GRAPH_RT_BEGIN` | graph audit: armed. |
| `MISO_ENGINE_GRAPH_RT_END` | graph audit: disarmed. |
| `MISO_ENGINE_BUILTINS_RT_BEGIN` | builtin direct-chain audit: armed. |
| `MISO_ENGINE_BUILTINS_RT_END` | builtin direct-chain audit: disarmed. |
| `MISO_ENGINE_BUILTINS_GRAPH_RT_BEGIN` | builtin graph audit: armed. |
| `MISO_ENGINE_BUILTINS_GRAPH_RT_END` | builtin graph audit: disarmed. |
| `MISO_ENGINE_SOURCE_RT_BEGIN` | source audit: armed. |
| `MISO_ENGINE_SOURCE_RT_END` | source audit: disarmed. |
| `MISO_ENGINE_EFFECT_RT_BEGIN` | effect-contract audit: armed. |
| `MISO_ENGINE_EFFECT_RT_END` | effect-contract audit: disarmed. |
| `MISO_ENGINE_PARAMETRIC_EQ_RT_BEGIN` | parametric-EQ audit: armed. |
| `MISO_ENGINE_PARAMETRIC_EQ_RT_END` | parametric-EQ audit: disarmed. |
| `MISO_ENGINE_COMPRESSOR_RT_BEGIN` | compressor audit: armed. |
| `MISO_ENGINE_COMPRESSOR_RT_END` | compressor audit: disarmed. |
| `MISO_ENGINE_DELAY_RT_BEGIN` | delay audit: armed. |
| `MISO_ENGINE_DELAY_RT_END` | delay audit: disarmed. |
| `MISO_ENGINE_GATE_EXPANDER_RT_BEGIN` | gate/expander audit: armed. |
| `MISO_ENGINE_GATE_EXPANDER_RT_END` | gate/expander audit: disarmed. |
| `MISO_ENGINE_SCHEDULER_PHASE_PREPARED` | scheduler audit: plan prepared. |
| `MISO_ENGINE_SCHEDULER_PHASE_ARMED` | scheduler audit: armed. |
| `MISO_ENGINE_SCHEDULER_PHASE_DISARMED` | scheduler audit: disarmed. |
| `MISO_ENGINE_SCHEDULER_PHASE_RETIRED` | scheduler audit: plan retired off-render. |


## Subject switches

Read by one subject each.

| name | meaning |
|---|---|
| `MISO_ENGINE_BUILTINS_SKIP_METADATA` | `check-builtins-policy.sh`: skip the `cargo metadata` smoke. |
| `MISO_ENGINE_SCHEDULER_AUDIT_PACED` | scheduler audit: run the paced arrival pattern. |
| `MISO_ENGINE_SCHEDULER_TRACE_ROOT` | scheduler trace gate: where the strace files go. |
| `MISO_ENGINE_CAPI_HEADER` | C-ABI tests: path of `miso_engine_v2.h`. |
| `MISO_ENGINE_CAPI_LIBRARY` | C-ABI tests: path of the built library. |
| `MISO_ENGINE_CAPI_C_FIXTURE` | C-ABI tests: path of the C consumer. |
| `MISO_ENGINE_CAPI_CPP_FIXTURE` | C-ABI tests: path of the C++ consumer. |
| `MISO_ENGINE_CAPI_SKIP_BUILD` | C-ABI tests: use a prebuilt library. |
| `MISO_ENGINE_CHROMIUM_BINARY` | browser gate: Chromium path. |
| `MISO_ENGINE_CHROMEDRIVER_BINARY` | browser gate: chromedriver path. |
| `MISO_ENGINE_WEB_STRIP` | AudioWorklet build: the `wasm-strip` binary. |
| `MISO_ENGINE_WEB_WORKLET_TEST_MODULE` | Hermetic worklet test: override module path for the bootstrap-under-test (#132). |
| `MISO_ENGINE_PRINT_HELPER_MANIFEST` | native PCM runner portability gate: helper manifest path. |
| `MISO_ENGINE_EFFECT_CONTRACT_V1_H` | the C include guard `check-effect-contract.sh` asserts. Not an environment variable. |
| `MISO_ENGINE_VECTORIZATION_AARCH64_TARGET` | native vectorization report: which AArch64 target to build the probe object for. |


## Test harness hooks

Read only by a `scripts/test-*.sh` fake, never by a real run. A runner that reads one of these outside a `MISO_ENGINE_TEST_`-guarded branch is a defect.

| name | meaning |
|---|---|
| `MISO_ENGINE_TEST_MODE` | which scripted outcome the stub produces. |
| `MISO_ENGINE_TEST_BENCH_MODE` | which scripted outcome the fake bench produces. |
| `MISO_ENGINE_TEST_FAKE_BENCH` | path of a fake bench binary to launch instead of building. |
| `MISO_ENGINE_TEST_LAUNCH_LOG` | file the stub appends one line to per launch. |
| `MISO_ENGINE_TEST_RECORDS` | records the fake bench emits. |
| `MISO_ENGINE_TEST_FROZEN_RAW` | raw payload the stub replays. |
| `MISO_ENGINE_TEST_CANDIDATE` | candidate identity the stub reports. |
| `MISO_ENGINE_TEST_GIT_DIRTY` | make the runner see a dirty tree. |
| `MISO_ENGINE_TEST_CARGO_FAIL` | make the cargo stub fail. |
| `MISO_ENGINE_TEST_PREFLIGHT_ROOT` | root the preflight stub inspects. |
| `MISO_ENGINE_TEST_PREFLIGHT_LAUNCH_LOG` | preflight launch log. |
| `MISO_ENGINE_TEST_PREFLIGHT_CARGO_LOG` | preflight cargo log. |
| `MISO_ENGINE_TEST_PREFLIGHT_DRIFT` | make the preflight see authority drift. |
| `MISO_ENGINE_TEST_REAL_RUSTC` | real rustc behind the stub. |
| `MISO_ENGINE_TEST_REAL_SHA256SUM` | real sha256sum behind the stub. |
| `MISO_ENGINE_TEST_RUSTC_PIPE_FAIL` | make the rustc stub break the pipe. |

