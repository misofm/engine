# 072 Separate builtin benchmark render timing from meter evidence collection

## Sol briefing checkpoint — 2026-08-22

**SOL XHIGH FAIL / TERMINAL STOP / NOT COMPLETE; ALL ISSUE-072 AUTHORITIES ARE CONSUMED.**
The authoritative brief is
`BRIEFS/072-separate-builtin-benchmark-render-timing-from-meter-evidence-collection.md`. This
stateless successor permits one Sol High implementation pass and, only after a Sol XHigh HOLD, one
bounded Sol High correction pass. Sol XHigh adversarially verifies both. A second HOLD exhausts the
two-total-pass budget and stops the issue. At rebrief, Issue-072 counters are
`preflight_invocations=0`, `runner_invocations=0`, `workload_invocations=0`, and
`timed_benchmark_invocations=0`. The consumed Issue-058 run remains historical evidence and is not
reset or reclassified.

The refreshed clean baseline is `main` commit
`c0eb5c5cb438e7a3b3cdaea30370812e09d8dffb`, tree
`43c6fd994d798fa93576abcda9a9a2eda3198bfe`, with `Cargo.lock` SHA-256
`4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`.
`target/issue72` is absent. This docs checkpoint authorizes no preflight, benchmark main, runner,
workload or timing command.

Sol High pass 1 implemented the render-only audit/timer boundary and successor lifecycle. Sol XHigh
returned one bounded HOLD because the runner did not independently compare every preflight
source/lock/input authority or validate the complete nonbenchmark seal. Sol High pass 2 corrected
that boundary before raw creation and added direct plus tandem replacement mutations with zero fake
launches. At the focused checkpoint, the untimed audited regression passed for all four render
workloads, both frozen rates and both independently prepared/warmed round states; package, locked,
warning-denied, validator/lifecycle, fixture and policy gates passed. The five retained Issue-058
files were exact, `target/issue72` was absent and counters were `0/0/0/0`. Sol XHigh returned a
strict focused PASS and authorized only the exact checkpoint commit. That focused verdict did not
constitute overall PASS or authorize the later seal, preflight, main, runner, workload or timing.

## Terminal runner evidence — 2026-08-22

The focused product/timing correction remains valid, but Issue 072 has no overall PASS. The clean
immutable candidate was HEAD `9dc95a5fb4d8e65c582b84320c84b22f2d780eba`, tree
`7e99e5fafa130e572d421156037b36f7f59232d7`. Its sole zero-workload preflight and sole separately
authorized runner both exited zero. Final Issue-072 preflight/runner/workload/timed counters are
`1/1/1/1`, with one warmup, two measured rounds and 20 records. Runner stdout contained exactly the
accepted-artifact path. Both one-shot authorities are consumed; no preflight, runner, direct binary,
alternate invocation, retry, tuning or additional timing is authorized.

All seven files in `target/issue72` are regular one-link files. The prelaunch disposition is absent:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `nonbenchmark.seal.json` | 2,109 | `7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d` |
| `miso_engine_builtins_bench` | 3,200,296 | `a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912` |
| `builtins-benchmark.preflight.json` | 1,525 | `f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf` |
| `builtins-benchmark.raw.jsonl` | 40,136 | `c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a` |
| `builtins-benchmark.jsonl` | 40,136 | `c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a` |
| `builtins-benchmark.validator.stderr` | 211 | `7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396` |
| `builtins-benchmark.disposition.json` | 1,252 | `b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e` |

Raw and accepted bytes are identical and their inodes are distinct. Stderr contains exactly the
five ordered `workload_started`, `warmup_complete`, `timed_started`, `round_1_complete` and
`round_2_complete` phase lines. The disposition is `PASS` / `complete`, binds every terminal
identity above and reports counters `1/1/1/1`, warmup `1`, rounds `2`, workload exit `0`.

The frozen aggregate validator and independent Sol XHigh checks accept the exact order: five
workloads at 48 kHz for round 1, five at 96 kHz for round 1, then the same rate/workload order for
round 2. Every workload/rate output hash is stable across rounds. All 16 render records report zero
errors and zero for all nine forbidden-operation categories and their total; the four preparation
records correctly use `not_applicable` and make no render claim. Median times were descriptive
only: identity about `1.028`–`1.034` microseconds, matrix about `1.441`–`1.461` microseconds, full
chain about `2.285`–`2.301` microseconds, meter success/full about `25.369`–`25.586` microseconds,
and 256-track preparation about `898.919`–`907.797` microseconds per operation across the two rates
and rounds. These are neither thresholds nor comparative or release claims.

Terminal failure is metadata completeness. Every record honestly contains JSON null for all 16
host/build fields and lists all 16 in `missing_metadata`: `background_load_note`, `codegen_units`,
`cpu_architecture`, `cpu_model`, `governor_or_power_mode`, `kernel`, `llvm_version`,
`logical_core_count`, `lto`, `opt_level`, `os`, `physical_core_count`, `profile`, `rust_version`,
`target_features`, and `target_triple`. Several were available on the benchmark host and therefore
were required to be recorded. The benchmark reads fixed `MISO_ENGINE_BENCH_*` variables, but the
runner exported only candidate commit and binary SHA-256. The validator correctly enforced honest
null/list equivalence but did not and could not prove that the runner had attempted discovery.
This contradicts the frozen runner contract and gate 7's complete host-metadata requirement.

Sol XHigh therefore records **FAIL / TERMINAL STOP / NO OVERALL PASS**. Do not overwrite, delete,
truncate, rename, link from or otherwise reuse any `target/issue72` artifact. Successor issue 109,
**Repair builtin benchmark host metadata export and reauthorize one descriptive run**, owns only
runner-side metadata discovery/export, successor-specific fake/static authority, a new zero-launch
preflight and at most one separately authorized descriptive rerun under `target/issue109`.

## Outcome

Correct the proven builtin benchmark harness boundary so only product render is audited and timed,
while meter draining and all evidence hashing occur allocation-free or off-scope as frozen. Seal
the corrected candidate, then perform one new no-retry preflight and exactly one no-retry
one-warmup/two-measured-round descriptive run.

## Context

Issue 058 stopped after its sole authorized runner invocation aborted at the first
`meter_success_full` warmup. `RealMeterTapRuntime::render_one` called
`drain_all().collect::<Vec<_>>()` inside `audit::in_render_scope`; the audited global allocator
aborted with status 134. The same path placed queue draining and evidence hashing inside the timed
operation. Issue-058 checkpoint `bd17fc1` and its preserved artifacts are technical input, not a
PASS dependency.

This issue does not reopen the accepted product, corpus, audit, target, instruction, schema, or
workload decisions. It changes only the benchmark harness boundary and the successor-owned
artifact lifecycle needed because the failed Issue-058 paths cannot be overwritten.

## Scope

- Split every render operation into outside-timing input preparation, render-only audited/timed
  execution, and outside-timing evidence collection.
- Drain the seven success meter consumers directly without `Vec`, collection, allocation, or any
  other transient retained container. Keep the capacity-one full plan prefilled and record its
  final snapshot/drop evidence outside render timing.
- Hash all direct PCM, graph PCM, meter snapshots, tap identities, counters, and continuation
  evidence only after the elapsed interval and after the render audit is disarmed.
- Add a compact nonexecuting test that exercises every render workload with the audited allocator
  armed only around product render and proves zero audit counters, off-scope drain/hash, exact
  seven-tap behavior, and unchanged output identities across the two prepared round states.
- Retarget the existing preflight/runner lifecycle to new Issue-072 artifacts without changing the
  frozen Issue-035 record schema.
- After all nonexecuting gates pass, commit the exact implementation paths and create one
  successor-owned no-clobber nonbenchmark seal on that clean candidate. Only a separate Sol XHigh
  seal review may authorize exactly one zero-workload preflight; only a later independent Sol XHigh
  preflight review may authorize exactly one runner.

## Required public interfaces and artifact contract

No engine or DSP API changes are permitted. `miso_engine_builtins_bench` remains a private tooling
binary and the public runner remains a no-argument fixed entrypoint.

Measured JSONL remains byte-schema compatible with Issue 035:

- `schema_version=2`, `issue=35`, and `issue035.<kind>.<rate>hz.q128` IDs;
- five kinds × rates 48,000/96,000 × rounds 1/2 = exactly 20 records;
- one global untimed warmup and exactly two measured rounds;
- unchanged checked TOML/PCM inputs, output-hash definitions, audit fields, metadata mapping,
  nearest-rank percentiles, operation counts, and no threshold.

Successor-owned paths are exactly:

- `target/issue72/nonbenchmark.seal.json`;
- `target/issue72/miso_engine_builtins_bench`;
- `target/issue72/builtins-benchmark.preflight.json`;
- `target/issue72/builtins-benchmark.raw.jsonl`;
- `target/issue72/builtins-benchmark.jsonl`;
- `target/issue72/builtins-benchmark.validator.stderr`;
- `target/issue72/builtins-benchmark.prelaunch.disposition.json`; and
- `target/issue72/builtins-benchmark.disposition.json`.

Every persistent successor file is regular, one-link, atomically published and never overwritten.
The Issue-072 seals and dispositions use `issue=72`; measured records retain `issue=35`. The runner
accepts no arguments, path overrides, environment-selected binary, retry, or resume. It refuses a
preexisting final or prelaunch disposition before scratch creation, records any prelaunch failure
in the prelaunch disposition, launches the sealed binary at most once, and preserves raw, stderr
and final disposition after launch. Either disposition consumes the sole runner authorization.

## Preserved Issue-058 failure evidence

Do not delete, rewrite, move, truncate, link from or reuse any `target/issue35` path. Before and
after every Issue-072 seal/preflight/run, verify all retained files are regular and one-link:

- binary 3,191,104 bytes, SHA-256
  `242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944`;
- preflight seal 2,211 bytes, SHA-256
  `85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d`;
- raw and validator stderr each 0 bytes, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- accepted output and `builtins-benchmark.prelaunch.disposition.json` absent; and
- FAIL disposition 974 bytes, SHA-256
  `e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce`.

## Refreshed source, lock and immutable dependency identities

Preserve the Issue-058 frozen manifest, ten benchmark-input hashes, graph PCM/meter, accepted
audit/trace/target identities, record and aggregate validators, launch-rate scope, and historical
benchmark-only lock transition. Current preimplementation identities are:

- benchmark source
  `3e68bf52e76e7a3ecb4b07c97056cce0e85aa6648ac7267a0dc79c89e84b1100`;
- preflight script
  `76ff9959b96e20ae2fd21e46ce8f12359a6b77ca317ba90fc13e998761d1df40`;
- runner script
  `7e429ca2df3c24494e9d3c9bcc03b4e0713851d34b8831eb449d1c5f41cccabc`;
- lifecycle test
  `cb26f6f05c963ff1b56d8a8ce3aeb82e4d6b1561d60f6bd5a58240d3ef9cf74d`;
- record validator
  `c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467`;
- aggregate validator
  `6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63`; and
- current `Cargo.lock`
  `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`.

The source and three lifecycle scripts are permitted implementation inputs, so the clean
postimplementation nonbenchmark seal binds their final hashes. The two validators and
`Cargo.lock` are read-only and must retain the identities above. Frozen content identities remain:

- manifest `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
- graph PCM `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
- graph meter `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- accepted Issue-068 source manifest
  `0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`.

Issue 058 historically based its benchmark-only lock transition at commit
`265109f300f58e005ac7a68a56298d167c5ae809`, pretransition lock SHA-256
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`, and binary-diff SHA-256
`5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e`; the original Issue-072
briefing later recorded lock SHA-256
`da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`.
These values remain candid historical provenance, not active Issue-072 gates. The Issue-072
preflight must bind the exact clean postimplementation `Cargo.lock`, which retains the refreshed
hash above, and must not compare the current workspace against the old Issue-058 base/diff. No
dependency, version or lock change is allowed in Issue 072.

No dependency, version, source, checksum, fixture, input, record schema, workload or product code
may change. Any required change beyond the benchmark crate and its direct scripts is a STOP.

## Deliverables

- corrected render/timing/evidence boundary in the existing benchmark tool;
- focused armed-render and off-scope evidence tests;
- successor-owned no-clobber preflight/runner lifecycle and hermetic stub coverage;
- one clean-candidate `target/issue72/nonbenchmark.seal.json` with all real invocation counters
  `0/0/0/0`;
- one sealed Issue-072 preflight record with zero runner/workload/timed counters; and
- only after separate Sol authorization, one raw/accepted/stderr/disposition set for the exact
  20-record descriptive run.

## Explicit non-goals

Product DSP/core/graph/runtime/session changes; corpus, benchmark-input, schema, rate, workload,
operation-count, tolerance, audit, target, instruction, metadata, or percentile changes; a general
meter API; performance tuning; thresholds; retries; resume; direct binary execution; listening;
or altering Issue-058 artifacts.

## Dependencies by exact issue title

- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

Stopped Issue 058 is consumed only as the exact technical checkpoint and failure evidence above;
it is deliberately not a PASS dependency.

## Ordered acceptance gates — gate 7 not met

1. Static review proves the Issue-058 allocator-abort path is removed without changing product
   render, inputs, schema, validators, rates, operation counts, or output-hash content/order.
2. Focused tests execute all four render workloads under the audited allocator without timing or
   benchmark main: product render alone is armed, every audit category is zero, the success queue
   drains seven exact taps outside scope, the full queue stays full/drops exactly, evidence hashing
   is outside scope, both warmup states remain identical, and no allocation is hidden by fallback.
3. Format, locked benchmark-package check/tests, warning-denied Clippy/rustdoc, read-only fixture
   validation, complete synthetic validator/lifecycle suite, locked workspace check/tests, and
   applicable workspace/realtime/builtins/graph/rack policy and mutation gates pass. Static scans
   prove benchmark main, public runner, preflight, audit/trace/target workload and timing counts are
   zero during implementation/review. Sol High pauses for Sol XHigh review; one HOLD permits only
   the bounded pass 2, while a second HOLD is terminal.
4. Root commits the exact focused product/lifecycle paths. On that clean candidate, with
   `target/issue72` absent and Issue-058 artifacts unchanged, root no-clobber publishes only
   `target/issue72/nonbenchmark.seal.json`. Its closed schema binds branch/HEAD/tree, current lock,
   final benchmark source and three lifecycle scripts, both frozen validators, builtin manifest,
   graph PCM/meter, Issue-068 source identity, all Issue-058 file identities/absence facts, focused
   regression count `1`, and preflight/runner/workload/timed counters `0/0/0/0`. Creating and
   validating this seal launches no benchmark code and grants no execution authority.
5. Sol XHigh independently validates the clean candidate, seal, one-link/no-clobber state,
   Issue-058 evidence and absence of every other Issue-072 path. A strict GO may authorize exactly
   one invocation of `bash scripts/preflight-builtins-benchmark.sh`. That preflight accepts no
   arguments, executes zero benchmark workloads, builds the release binary, and atomically
   publishes only the binary and preflight seal. Failure consumes the preflight authorization and
   stops; there is no alternate or repeat preflight.
6. Sol XHigh independently validates the exact candidate/source/lock/tool/input/nonbenchmark/binary/
   preflight seals, preflight/runner/workload/timed counters `1/0/0/0`, and absence of raw,
   accepted, stderr and both disposition paths. Only a separate strict GO may authorize exactly
   one invocation of `bash scripts/run-builtins-benchmark.sh`.
7. The sole runner invocation must emit exactly 20 validator-valid rows, one warmup and two rounds;
   preserve raw bytes; atomically publish a byte-identical distinct-inode accepted copy; record
   complete host and audit metadata; and write a PASS/complete final disposition with total
   preflight/runner/workload/timed counters `1/1/1/1`. A prelaunch failure instead publishes only
   the prelaunch disposition with runner `1`, workload/timed `0/0`; a postlaunch failure preserves
   raw/stderr and writes the final FAIL disposition with phase-derived counters. Any failure
   consumes authorization and is final STOP.

## Required evidence and disposition

Record Sol High pass and Sol XHigh verdict counts; clean candidate/source/lock/tool/nonbenchmark/
binary/preflight seals; all immutable hashes; before/after Issue-058 artifact hashes; focused
armed-render transcript; package/workspace/policy gates; each separate authorization; exact
artifact sizes/hashes/inode counts; 20-row cardinality; one warmup/two rounds; phase-derived
counters; zero render violations; no-threshold statement; and strict PASS/FAIL. PASS produces the
machine-qualified candidate for Issue 033 but does not claim human listening, release readiness,
capacity, or performance superiority. Root owns the evidence commit, remote body synchronization
and closure after Sol XHigh PASS; this rebrief claims none of those future actions.
