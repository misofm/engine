# 109 Repair builtin benchmark host metadata export and reauthorize one descriptive run

## Outcome and briefing status

Repair only the builtin benchmark runner's host/build metadata discovery and fixed environment
export, prove that repair without timing, and establish a successor-owned authorization barrier for
at most one fresh descriptive run. Preserve the accepted Issue-072 render-only timer/audit boundary,
binary record schema, inputs, workloads, rates, output hashes and all terminal artifacts.

**SOL XHIGH BRIEF PASS / READY FOR SOL HIGH PASS 1; ZERO PREFLIGHT OR WORKLOAD AUTHORIZATION.**
Sol High implements one focused pass. If Sol XHigh returns one HOLD, Sol High may make one bounded
correction; a second HOLD stops the issue. Sol XHigh remains briefer and adversarial verifier.
At briefing, Issue-109 preflight/runner/workload/timed counters are `0/0/0/0`, `target/issue109` is
absent, and no preflight, benchmark main, runner, workload or timing invocation is authorized.

Remote issue 109 was read-only verified available on 2026-08-22. Root must create the matching
GitHub issue with this exact H1 title before implementation and later owns synchronization and
closure. This local brief claims no GitHub mutation.

## Terminal Issue-072 input

Issue 072 stopped without overall PASS after its only runner produced correct render/timing evidence
but omitted discoverable host/build metadata. Its clean candidate was
`9dc95a5fb4d8e65c582b84320c84b22f2d780eba`, tree
`7e99e5fafa130e572d421156037b36f7f59232d7`. Historical Issue-072 preflight/runner/workload/timed
counters are permanently `1/1/1/1`, warmup `1`, rounds `2`, records `20`. Do not reclassify or reset
them.

The seven files below are immutable regular one-link evidence under `target/issue72`; the prelaunch
disposition remains absent:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `nonbenchmark.seal.json` | 2,109 | `7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d` |
| `miso_engine_builtins_bench` | 3,200,296 | `a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912` |
| `builtins-benchmark.preflight.json` | 1,525 | `f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf` |
| `builtins-benchmark.raw.jsonl` | 40,136 | `c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a` |
| `builtins-benchmark.jsonl` | 40,136 | `c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a` |
| `builtins-benchmark.validator.stderr` | 211 | `7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396` |
| `builtins-benchmark.disposition.json` | 1,252 | `b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e` |

Raw and accepted are byte-identical distinct-inode files. Their matrix, timing relations,
input/output identities and zero render-audit counts remain positive technical evidence. The
failure is exact: all 20 records contained null for every one of the 16 metadata fields and named
all 16 in `missing_metadata`. The tool reads fixed `MISO_ENGINE_BENCH_*` variables, while the
runner exported only candidate commit and binary SHA-256. Honest null/list equivalence did not
prove that discovery was attempted.

Never create, delete, truncate, replace, rename, link from or write beneath `target/issue72`.
Issue-109 scripts may verify those identities and absences read-only. All successor artifacts use
only `target/issue109`.

## Dependencies by exact issue title

- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification
- Separate builtin benchmark render timing from meter evidence collection

Issue 068 is the accepted product/target dependency. Issue 072 is stopped terminal technical input,
not an accepted overall PASS. This issue neither reopens nor weakens either record.

## Smallest closable correction

Do not change the benchmark Rust source. Add successor-only scripts that collect metadata on the
control plane immediately before the single binary launch, remove ambient values for all 16
`MISO_ENGINE_BENCH_*` keys, and export only freshly derived values. Collection, validation and
serialization occur before `workload_started` and outside every measured interval.

The metadata fields and environment names remain exactly those already consumed by
`Metadata::collect`:

| Record field | Environment variable | Required source rule |
| --- | --- | --- |
| `cpu_model` | `MISO_ENGINE_BENCH_CPU_MODEL` | `/proc/cpuinfo` first model-name value when readable and usable; otherwise null/missing |
| `cpu_architecture` | `MISO_ENGINE_BENCH_CPU_ARCHITECTURE` | required `uname -m` value |
| `logical_core_count` | `MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT` | required positive `getconf _NPROCESSORS_ONLN` value |
| `physical_core_count` | `MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT` | unique core/socket pairs from `lscpu -p=CORE,SOCKET` when available; otherwise null/missing |
| `os` | `MISO_ENGINE_BENCH_OS` | required `uname -s` value |
| `kernel` | `MISO_ENGINE_BENCH_KERNEL` | required `uname -r` value |
| `governor_or_power_mode` | `MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE` | scaling governor when readable and usable; otherwise null/missing |
| `rust_version` | `MISO_ENGINE_BENCH_RUST_VERSION` | required exact `rustc -V` output |
| `llvm_version` | `MISO_ENGINE_BENCH_LLVM_VERSION` | required LLVM value from `rustc -vV` |
| `target_triple` | `MISO_ENGINE_BENCH_TARGET_TRIPLE` | required host value from `rustc -vV` |
| `target_features` | `MISO_ENGINE_BENCH_TARGET_FEATURES` | required sorted comma-separated `target_feature` values from `rustc --print cfg` for that target |
| `profile` | `MISO_ENGINE_BENCH_PROFILE` | fixed `release` |
| `opt_level` | `MISO_ENGINE_BENCH_OPT_LEVEL` | fixed `3` for the sealed Cargo release build |
| `lto` | `MISO_ENGINE_BENCH_LTO` | fixed `false` for the unchanged release profile |
| `codegen_units` | `MISO_ENGINE_BENCH_CODEGEN_UNITS` | fixed `16` for the unchanged release profile |
| `background_load_note` | `MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE` | prelaunch `/proc/loadavg` triple plus an explicit `not-controlled` label when readable; otherwise null/missing |

Required-source failure is a prelaunch STOP, not a null. Optional-source absence is represented by
an unset/empty environment value, JSON null, and the exact sorted field name in `missing_metadata`.
An available usable value must never be discarded as missing. Reject empty/control-containing,
`unknown` or `default` values, invalid/nonpositive counts, malformed tool output, duplicate target
features, ambient overrides and disagreement between the captured projection and any record.

The runner builds an exact expected metadata projection before launch. After the workload returns,
it requires all 20 records to share that projection and requires `missing_metadata` to equal exactly
the unavailable optional fields. The final disposition binds a SHA-256 of that canonical projection
in addition to the existing candidate, binary, seal, raw, accepted and stderr identities.

## Frozen benchmark and product boundary

Preserve byte-for-byte:

- `tools/miso-engine-builtins-bench/src/main.rs` at
  `b520e3d14bd4fa2985d18f273e515261a53b4ea69ac1a2a38aba9bc77bf6e7fe` and its Cargo manifest;
- the existing Issue-072 preflight, runner and lifecycle scripts at respectively
  `216cdd879a02b350279619066a28be7f3ef5fa9f05ec26641dd6d3bac634cfe8`,
  `17968dfbdc502ecf8f708e4d99db199848a153d08e6dbc25ef46a4bf9a02669f`, and
  `19ecf0ed6c0b6dacbbd2ebf1417fff0bd1207d2cfd567d3a731f735805704b0c`;
- both existing record/aggregate validators and their hashes
  `c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467` and
  `6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63`;
- `Cargo.lock` at `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`;
- builtin manifest `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`,
  graph PCM `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`
  and graph meter `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- accepted Issue-068 source identity
  `0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`;
- schema version 2 with `issue=35`, five workload kinds, 48/96 kHz, quantum 128, one warmup,
  two rounds, 20 records, nearest-rank percentiles, operation counts, input and output hash rules;
  and
- the Issue-072 render-only audit/timer boundary, direct meter drain, evidence ordering and all
  product APIs, sources, fixtures and policies.

The successor rerun is descriptive only. No threshold, comparison, tuning, optimization, capacity,
quality, listening or release claim follows from its numbers.

## Successor namespace and one-shot lifecycle

Persistent Issue-109 paths are exactly:

- `target/issue109/metadata-repair.seal.json`;
- `target/issue109/miso_engine_builtins_bench`;
- `target/issue109/builtins-benchmark.preflight.json`;
- `target/issue109/builtins-benchmark.raw.jsonl`;
- `target/issue109/builtins-benchmark.jsonl`;
- `target/issue109/builtins-benchmark.validator.stderr`;
- `target/issue109/builtins-benchmark.prelaunch.disposition.json`; and
- `target/issue109/builtins-benchmark.disposition.json`.

Every persistent file is regular, one-link, atomically published and never overwritten. A
preexisting prelaunch or final disposition blocks before scratch creation. Any public runner call,
including arguments, missing metadata tools/sources, dirty candidate, seal mismatch or other
prelaunch failure, publishes the prelaunch disposition and consumes the authority. Postlaunch
failure preserves raw/stderr and publishes the final FAIL disposition. There is no retry, resume,
path override, environment-selected binary or direct/alternate invocation.

## Non-timed proof and static mutation matrix

The hermetic lifecycle uses only temporary fake commands, metadata sources and a fake benchmark
binary. It must prove, without invoking real main or a clock:

- every required and optional available field is exported with its exact type/value;
- optional missing sources produce exact null/sorted-missing output, while missing required sources
  stop before launch;
- ambient metadata variables cannot override derived values;
- malformed, duplicate, empty, sentinel, control-containing and invalid numeric discovery fails;
- all 16 record fields match the captured projection on every row, including mixed-row and dishonest
  null/missing mutations;
- direct and tandem seal/authority mutations, dirty candidate, arguments and missing tools reject;
- zero fake launch for every prelaunch failure and exact phase-derived counters after launch;
- regular/symlink/hardlink no-clobber, partial-output preservation, distinct-inode accepted copy,
  raw mutation isolation, exact five-line phase handshake and second-call refusal.

A static checker pins the allowed paths, Issue-072 artifact identities, unchanged tool/validators/
lock/fixtures, all 16 environment mappings, required build constants, `issue=109` authority files,
`target/issue109`, one launch site and initial counters `0/0/0/0`. Its synthetic mutation suite must
make each invariant fail. These fakes and scans are nonbenchmark evidence.

## Checkpoints and authorization barriers

### Checkpoint 1 — runner repair and fake proof

Sol High adds only successor scripts and docs, runs shell syntax, the hermetic fake lifecycle,
static checker/mutations, frozen validator self-tests or read-only validation, and text/diff/artifact
scans. A compile-only locked check of the unchanged benchmark package is allowed if needed; no
benchmark main, preflight, runner, workload or timing path is executed. No broad, target, audit,
trace, listening or inherited Issue-072 gate is rerun. Sol XHigh must return focused PASS before
root commits the exact paths.

On that clean commit, root no-clobber creates only
`target/issue109/metadata-repair.seal.json`. Its closed schema binds branch/HEAD/tree, current lock,
unchanged tool/validators/fixtures, all five successor scripts, exact Issue-072 artifacts and
absence, metadata-regression count `1`, and Issue-109 preflight/runner/workload/timed counters
`0/0/0/0`. Sol XHigh independently validates it before any preflight authorization.

### Checkpoint 2 — sole zero-workload preflight

Only a strict Sol XHigh GO may authorize exactly one:

`bash scripts/preflight-builtins-benchmark-109.sh`

It accepts no arguments, validates the clean candidate and repair seal, executes only fake/static/
compile gates, builds but never executes the unchanged benchmark binary, and publishes only the
binary and preflight seal. The seal binds every authority and planned warmup `1`, rounds `2`, records
`20`, with counters `1/0/0/0`. Failure is STOP; no repeat or alternate preflight exists.

Sol XHigh then independently validates every seal/hash, Issue-072 preservation, regular one-link
state, exact target/issue109 membership and absent raw/accepted/stderr/dispositions. Only a separate
strict GO may authorize exactly one:

`bash scripts/run-builtins-benchmark-109.sh`

The runner gathers metadata once before launch, launches only the sealed binary once, and permits
one untimed warmup plus two measured rounds. Success requires 20 frozen-validator-valid records,
exact metadata-projection equality, all render audit counts zero, byte-identical distinct-inode
raw/accepted files, exact five-phase stderr and a PASS/complete disposition with counters
`1/1/1/1`. Any failure is terminal STOP. No further benchmark activity is authorized.

## Exact allowed paths

- this spec and its tracked brief;
- the minimal Issue-072 spec/brief and `ISSUE_SPECS/README.md` terminal/successor routing;
- `scripts/run-builtins-benchmark-109.sh`;
- `scripts/preflight-builtins-benchmark-109.sh`;
- `scripts/test-builtins-benchmark-109.sh`;
- `scripts/check-builtins-benchmark-109.sh`; and
- `scripts/test-builtins-benchmark-109-policy.sh`.

No Rust, Cargo manifest/lock, existing validator, existing benchmark script, production, fixture,
reference, target, audit, trace, C ABI or policy source edit is allowed. If the repair requires a
new schema, workload, timer, product change, second benchmark framework or any Issue-072 artifact
mutation, STOP and rebrief.

## PASS boundary and evidence

PASS requires Sol High pass count and Sol XHigh verdicts; clean candidate and repair/preflight
seals; all authority hashes; exact before/after Issue-072 identities; fake/static transcript;
metadata projection and missing-only-when-unavailable proof; one authorized preflight and one
separately authorized runner; 20 exact records; raw/accepted/stderr/disposition sizes, hashes and
inode counts; phase-derived counters `1/1/1/1`; warmup `1`; rounds `2`; zero render violations; and
an explicit descriptive-only/no-threshold statement.

Only then may Issue 109 be marked `SOL XHIGH PASS / COMPLETE / READY TO CLOSE`. Root owns commit,
GitHub synchronization and closure. Issue 109 does not close human listening or release readiness.
