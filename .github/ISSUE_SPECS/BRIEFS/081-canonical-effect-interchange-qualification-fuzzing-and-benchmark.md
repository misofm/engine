# Sol implementation brief — issue 081 canonical interchange qualification

## Decision

**SOL XHIGH BRIEF PASS / READY FOR SOL HIGH ATTEMPT 1.** Implement qualification tooling only.
Accepted descriptor/package/CID/state/migration bytes, APIs, product source, fixtures and C ABI are
read-only. Use one Sol High implementation pass and at most one bounded Sol High correction, with a
Sol XHigh adversarial verdict after each. A second failed pass stops and rescopes.

Current counters are all zero: real reference-process, mutation-campaign, cross-target, benchmark
preflight, benchmark runner, benchmark workload and timed benchmark invocations. Do not run any of
them while implementing the harness.

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
