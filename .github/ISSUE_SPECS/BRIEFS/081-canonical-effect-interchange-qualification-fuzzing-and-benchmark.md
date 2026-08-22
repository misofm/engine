# Sol implementation brief — issue 081 canonical interchange qualification

## Decision

**SOL XHIGH TERMINAL BENCHMARK FAIL / STOP; SOLE RUNNER INVOCATION CONSUMED; NO RETRY OR OVERALL
PASS.** Accepted descriptor/package/CID/state/migration bytes, APIs, product source, fixtures and C
ABI remain valid and read-only; the failure is confined to the qualification benchmark tool.

At briefing, all real counters were zero. Qualification later recorded target invocation 1 on
candidate `709b3d2ccc6d`: native and Android/iOS completed; scalar Wasm reached object creation and
`wasm-objdump -x`, where a harness-only module/name false positive stopped before scalar opcode
inspection and before the SIMD row. No product or target failure occurred. The exact function-
export parser correction committed as `4cb3b5c`; Sol XHigh authorized target invocation 2, which
passed all five rows.

## Completed nonbenchmark verdict

The clean qualification candidate is `4cb3b5c3a97361218f474700751653c4400dc08d`, tree
`9aec9ade2645057cf2c93986a0d0eb47658df7d1`. Reference qualification passed in one runner
invocation with 100 successful children `0..99`. The exact 30,000-trial mutation campaign passed
with descriptor/package/state hashes `02d88fc02583926a1e53ffe56ae08d17bffe9039f8e75cefef70fefb07c34155`,
`fc8ea16692695dac08b29b64b5d7394c53ca70448ad3abc7c5c7994d289f7714` and
`1a153e0fe665d837deec13e014d442baeac49658baf8d3f927b5ddaef34a6ca2`. The exact 48-row migration
matrix passed with hash `f834c9447fb57e3e93408a69285e2a42b3bf94422ce7c4eb23dc205333849f46`.
C/Rust ABI and allocation/read-only gates passed; the spec records the exact native allocation
counters. Target invocation 2 passed all five rows. Static policies/mutations, shell syntax, format,
locked workspace check, warning-denied Clippy/rustdoc and final clean/diff/artifact checks passed.

The first workspace nonbenchmark test invocation is mechanically inconclusive because its parent
stream was lost after process completion; it is not counted as PASS or failure. The authoritative
retained-session repeat was exactly
`CARGO_BUILD_JOBS=1 cargo test --workspace --all-features --locked --lib --bins --tests --examples`
and exited `0` with only expected ignored/manual rows. Doctests exited `0` with eight compile-fail
doctests.

Counters at the nonbenchmark seal were reference `1`, mutation `1`, migration `1`, cross-target `2`,
and benchmark preflight/runner/workload/timed all `0`. `Cargo.lock` is
`4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`; `fuzz/Cargo.lock` is
`af4547d5bae367e4249c6fcf482b249ff8af0ae29b9a933957d34b36ec36e5d5`; accepted baseline is
`6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5`; qualification/target/
reference/benchmark-runner/preflight scripts are respectively `bde208b34413dd4e7e10fc27c2a85019300d61860c5055d5b081a949a704f970`,
`3edeacbbf6571bacfb87807ab6cf9d15612babf895c5215928fff1b3b0d3bae9`,
`026aa241b5146480fc393279f0fea4326c1b3172da81cadbf5750d186268014e`,
`4aca5153928bfee583cf5ea403483b63f848e4fb6a83045800424bc855a80429` and
`3957a02b8e5d45efd3e3637c60fc04157180c555fb46b0aa0eee4157afa3029c`.

## Terminal runner verdict

On clean candidate `466b05cbf2bb61e0367d25aa6ca6a0da7643e83f`, tree
`2e1c5c12515e7b16d8a36846130cfe4cde42ad55`, the sole zero-workload preflight exited `0` and
produced a valid 833-byte nonbenchmark seal (`6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c`),
827,232-byte binary (`fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c`)
and 1,577-byte preflight seal (`da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3`).
Sol XHigh verified the complete seal and authorized one runner invocation.

That invocation exited `1`, stdout was empty, and no accepted JSONL or prelaunch disposition was
created. Preserved artifacts are empty raw SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, 361-byte stderr SHA-256
`442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93`, and 817-byte terminal
disposition SHA-256 `8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de`.
The disposition is `FAIL/workload_failed`, with runner/workload/timed/warmup/round counters
`1/1/0/0/0` and exact candidate/binary/seal/raw/stderr identities.

Terminal real counters are reference `1`, mutation `1`, migration `1`, cross-target `2`, benchmark
preflight `1`, runner `1`, workload `1` and timed `0`.

The sole stderr phase is `workload_started`; main then panics at line 450 with descriptor-wire
`Semantic`, offset `0`, unavailable index and required `0`. `MIGRATION_Q1..Q3` each advertise only
48 kHz, while the accepted descriptor validator requires all four launch rates for every quality.
The validator correctly rejects the benchmark-only fixture during the frozen untimed migration
pass, before warmup or timing. Compile-only and fake lifecycle gates did not execute this real-main
fixture. The frozen migration digest was unreachable. This is not a product failure.

Do not rerun or directly invoke the Issue 081 runner/binary, repeat preflight, tune, repair records,
or remove/overwrite terminal artifacts. Issue 081 remains STOP with no overall PASS.

## Successor recommendation

Open **108 Repair effect-interchange benchmark migration fixture and reauthorize one descriptive
run** at
`.github/ISSUE_SPECS/108-repair-effect-interchange-benchmark-migration-fixture-and-reauthorize-one-descriptive-run.md`.
Its exact dependencies are **Canonical effect interchange qualification, fuzzing, and benchmark**,
**Prepared effect state envelope and transactional current-layout restore**, **Effect state
migration registry and bounded chains**, and **Close canonical effect descriptor wire, identity,
and C inspection ABI**.

Issue 108 must preserve Issue 081 evidence and accepted product bytes, repair only the benchmark
fixture to complete sorted four-rate D1/D2/D3 descriptors, add a focused nontimed executable
descriptor/final-envelope regression, independently rebind the migration digest across every
authority, and use a successor-specific no-clobber artifact namespace. Proportional compile/lint/
fake/static gates and a new zero-launch preflight precede any new Sol authorization. Existing real
100-process/mutation/migration/target/broad matrices are not rerun. Any later descriptive launch is
an Issue 108 attempt, never an Issue 081 retry.

## Smallest closable vertical

Keep Issue 081 whole but enforce two barriers. First qualify the immutable interchange boundary and
seal a clean candidate. Second build and fake-test a separate benchmark lifecycle, run a
zero-workload preflight, and wait for explicit Sol XHigh authorization before root's sole timed
invocation. The accepted Issue-002 conformance benchmark is not reusable: it measures different
work and lacks the required no-clobber/preflight lifecycle.

If the real 100-process runner fails after launch, do not retry it. If anything fails after the
timed workload launches, preserve raw/disposition evidence and move repair to a successor. Only a
prelaunch defect may consume the one bounded correction.

## Implementation order

1. Author sorted `fixtures/effect-interchange/v1/ACCEPTED.sha256` from clean base `8d78ea3` over all
   accepted descriptor/package/state fixtures and manifests, three independent references,
   descriptor C header and exact descriptor/package/CID/state/migration/restore product sources.
   Add a checker that rejects any baseline change before and after a gate.
2. Add an import-safe stdlib reference aggregator. Add the sole no-argument process runner that
   launches exactly 100 children indexed `0..99`, attempts all children, preserves raw/status
   evidence and atomically publishes only 100 unique successful identical-hash records. Fake-test
   its complete failure/no-clobber lifecycle; do not invoke it yet.
3. Add an explicit ignored qualification campaign with exactly 10,000 SplitMix64 single-bit trials
   per descriptor/package/state parser using the spec's three seeds. Parse twice under
   `catch_unwind`; freeze complete deterministic outcomes, canaries and normalized-outcome digest.
   Compile existing libFuzzer targets only; do not run open-ended fuzz.
4. Add the exact 48 migration rows: four launch rates x zero/one/two steps x scalar/bank source x
   scalar/bank destination. Freeze unequal sections, nonvacuous replay, exact final/next bytes,
   workspace suffixes and unrelated bank lanes. Reuse the complete accepted Issue-080 failures.
5. Complete native C/Rust layout/record agreement for the six descriptor ABI records without adding
   a C symbol. Add isolated allocation rows for descriptor temporaries, one-pass package deltas,
   allocation-free postverify/state/bank execution, bounded registry/resolution and scalar's exact
   required owned replay-initial slice plus sole mock destination `Box`. Every row returns to its
   exact live-byte baseline after drop.
6. Add one target script for native x86-64 execution plus Android/iOS AArch64 compile-only and Wasm
   scalar/SIMD compile/object-only rows. Missing tools/targets HOLD. Add static dependency, unsafe,
   baseline, render-reachability and generated-artifact checks plus minimum policy mutations.
7. Add exact qualification docs and fake/focused tests. Pause for Sol XHigh checkpoint-1 review and
   root's exact-path local commit before benchmark lifecycle work.
8. Add a new `miso-engine-effect-interchange-bench` tool with four fixed address-free workloads:
   descriptor verify/identity, package verify/CID/select, current state verify/re-encode and
   two-step bank migration/restore. It owns one untimed all-workload warmup and two measured rounds,
   256 observations per workload/round, and emits exactly eight closed-schema JSONL records.
9. Add one strict validator, hermetic fake lifecycle test, no-argument zero-launch preflight and
   no-argument public runner. Test arguments/schema/status/raw preservation/atomic publication/
   regular-symlink-hardlink overwrite refusal without executing benchmark main. Compile only, then
   pause for Sol XHigh checkpoint-2 review and root commit.
10. Run the ordered nonbenchmark qualification once. Commit its candidate-bound evidence. Run the
    preflight with workload/timing counts still zero. Sol XHigh independently reviews its seal and
    alone may authorize root's one runner invocation. Never tune or retry.

## Exact frozen matrices

- Reference: one real runner invocation, exactly 100 fresh Python processes total, indexes `0..99`,
  each running all three independent references in-process and emitting one address-free hash row.
- Mutation: exactly 10,000 trials each for descriptor/package/state, one SplitMix64-selected bit per
  fresh canonical input, seeds `0x081d_e5c0_0000_0001..3`, deterministic accept/diagnostic replay,
  no panic and read-only canaries.
- Migration: exactly 48 successes from rates `{44100,48000,88200,96000}` x step counts `{0,1,2}` x
  source `{scalar,bank}` x destination `{scalar,bank}`, plus unchanged accepted failure suites.
- C/Rust: parameter 80/4, port 24/4, quality 64/8, choice 16/4, summary 64/4 and diagnostic 16/4;
  every field offset and comprehensive-A projected value, with the sole existing inspect export.
- Allocation: descriptor; package publication; package postverify/CID; prebound state; registry/
  resolution; bank execution and scalar destination. Absolute native measurements are descriptive;
  zero native deltas/no survivors are gates where frozen by the spec.
- Targets: native x86-64 execute; Android `aarch64-linux-android` and iOS `aarch64-apple-ios`
  compile-only; Wasm scalar `-simd128` and Wasm SIMD `+simd128` compile/object-only.

## Benchmark lifecycle freeze

The tool emits four workload IDs x rounds 1 and 2. One warmup pass emits no record. Each measured
record has 256 one-operation observations, nearest-rank p50/p95/p99/p99.9/min/max in
`ns_per_operation`, exact candidate/binary/source/fixture/output hashes, complete honest machine/
toolchain metadata and no address/PID/absolute path. There is no threshold.

Preflight requires a clean exact HEAD/tree, all nonbenchmark seals and hashes; builds but never
executes the binary; validates synthetic records and the fake lifecycle; refuses existing/symlink/
hardlink artifacts; and atomically records zero runner/workload/timed invocations plus one planned
warmup, two rounds and eight records. Sol XHigh authorization must name that seal and candidate.

The public runner invokes the sealed binary once, streams stdout to an exclusively created raw file,
preserves every partial/rejected result, strictly validates, and atomically/no-clobber publishes
accepted/disposition evidence. Success is exactly runner=1, workload=1, timed=1, warmup=1,
measured-rounds=2. Postlaunch failure is not retryable.

## Checkpoints and verdicts

Checkpoint 1 contains only immutable-baseline/reference/mutation/migration/C-allocation/target/static
harness and docs. Focused fakes/smoke may run; real process/target/benchmark counts remain zero.

Checkpoint 2 contains only benchmark tool/validator/fake lifecycle/preflight/runner. It compiles and
fake-tests with real runner/workload/timing counts zero.

Qualification then runs in the spec's exact order and ends in one clean locked nonbenchmark
workspace seal. Root commits candidate-bound evidence. Preflight is a separate zero-launch action.
The sole benchmark occurs only after an explicit Sol XHigh PASS/authorization. Final evidence
records all hashes/counters and no broad/timed rerun follows it.

## Stop conditions and allowed surface

Allowed: #81 spec/brief and one qualification doc; qualification-only package/compiler/C tests;
`fixtures/effect-interchange/v1/`; one reference aggregator/process runner; one target/static checker
and minimum policy mutations; one new benchmark tool; one validator, fake lifecycle test, preflight
and runner; mechanical workspace manifest/lock entries without a new dependency.

STOP for any accepted product/reference/fixture/C-header/Issue-002 benchmark edit; public API/wire/
diagnostic/export change; production reverse dependency or render reachability; package/state/
migration C ABI; open-ended/time-based fuzzing; network/trust/repository/third-party/DSP/session/
graph scope; skipped target presented as PASS; real workload during implementation; or a repeated
100-process/timed invocation.
