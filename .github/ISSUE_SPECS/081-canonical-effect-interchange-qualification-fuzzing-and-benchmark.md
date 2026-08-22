# 081 Canonical effect interchange qualification, fuzzing, and benchmark

## Outcome and status

Qualify the accepted descriptor, package/CID, state and migration products as one portable
interchange boundary without changing their bytes, APIs or product implementation. **SOL XHIGH
BRIEF PASS / READY FOR SOL HIGH ATTEMPT 1.** This is a qualification/tooling issue, not another
product feature.

Use one coherent Sol High implementation attempt and at most one bounded Sol High correction, each
adversarially reviewed by Sol XHigh. A second failed implementation pass stops and requires a
stateless rescope/rebrief; gates are never weakened. At briefing,
`reference_process_invocations=0`, `mutation_campaign_invocations=0`,
`cross_target_invocations=0`, `benchmark_preflight_invocations=0`, `benchmark_runner_invocations=0`,
`benchmark_workload_invocations=0` and `timed_benchmark_invocations=0`.

Remote Issue 81 was read-only verified open with the exact title and no comments on 2026-08-22.
Its original body has the correct outcome but leaves the matrices and benchmark lifecycle
underspecified. Root synchronizes this corrected record only at the eventual CI-conscious batch
boundary; this local briefing does not claim remote synchronization.

## Readiness correction and smallest closable slice

Issues 078 and 080 are accepted. Issue 002 supplies deterministic seeds, allocation instrumentation
and a descriptive benchmark schema precedent, but its existing conformance benchmark measures PCM
fixture decode/comparison and has no no-clobber or zero-launch preflight. Reusing or extending that
runner would change an accepted dependency boundary and would not measure this issue's interchange
workload. Issue 081 therefore owns one new bounded interchange qualification harness and one new
benchmark lifecycle; the Issue-002 runner/tool remains byte-for-byte unchanged.

The issue is closable only in two authorized phases:

1. build and review the read-only qualification harness and benchmark lifecycle without launching
   the real 100-process matrix, cross-target matrix or benchmark; then run and seal every
   nonbenchmark gate, including exactly one 100-process reference invocation; and
2. on that clean immutable candidate, run a separate preflight that launches no workload. Only a
   subsequent explicit Sol XHigh authorization permits root to invoke the public benchmark runner
   once. That invocation owns one untimed warmup pass and exactly two measured rounds.

The process runner, benchmark runner and real benchmark binary are never used as implementation
smoke tests. Their lifecycle tests use inert fakes in temporary directories. If the actual
100-process runner fails after launch, or if the benchmark runner fails after launching the timed
workload, preserve the evidence and stop; do not retry. A bounded correction may repair a defect
found before either one-shot launch. A post-workload runner/validator defect becomes a successor
tooling issue and cannot consume a second timing invocation.

## Accepted boundary and immutable baseline

The accepted Issue-078 96-byte package header, 72-byte artifact record, descriptor identity,
36-byte CID binary, 59-byte CID text, diagnostics and selection rules remain unchanged. The accepted
Issue-079 224-byte state header, 16-byte initial record, digest, diagnostics and transactional
restore remain unchanged. The accepted Issue-080 selector, compatible-edge token, 56-byte migration
diagnostic, registry/resolution/workspace and scalar/bank restore APIs remain unchanged. The
Issue-082 descriptor wire/header/C ABI and Issue-011 runtime traits remain unchanged.

Add a sorted `fixtures/effect-interchange/v1/ACCEPTED.sha256` baseline manifest. It covers every
accepted descriptor/package/state fixture and manifest, the three independent Python references,
the descriptor C header, and the exact product source files that implement descriptor wire/FFI,
package/CID/state and migration/restore. The manifest is authored from clean briefing base
`8d78ea3d4ad42d36831b4d0267908a7b98f16167` and checked before and after every qualification
runner. It may not contain `target/` paths, absolute paths, timestamps or addresses. Qualification
may add tests, tools and evidence, but any accepted-baseline hash change is a STOP, not a fixture
refresh.

No production crate may depend on a qualification tool. New tools may depend downward on
effect-package/effect-compiler/conformance. No package/compiler public export, feature, C symbol,
wire, diagnostic or runtime call path is added.

## Exact independent-reference 100-process matrix

Add one standard-library-only, import-safe aggregator
`scripts/effect-interchange-v1-reference.py`. One child invocation:

- runs the complete existing descriptor, package and state independent reference checks in the
  same fresh Python process without shelling to Rust;
- verifies exact fixture membership and all three accepted manifests read-only;
- verifies descriptor encode/verify/identity, package encode/verify/re-encode/CID/selection and
  state encode/verify/re-encode/digest/descriptor identity; and
- emits exactly one canonical JSON line with only these keys:
  `schema_version=1`, `issue=81`, `process_index`, the three fixture-manifest SHA-256 values and one
  combined SHA-256 over their length-prefixed raw manifest bytes. It emits no path outside the repo,
  timestamp, PID, address or machine-dependent value.

`scripts/run-effect-interchange-reference-processes.sh` is the sole real process-matrix entrypoint.
It refuses arguments and existing/symlink raw or accepted evidence, freezes the baseline manifest
before launch, and invokes
`PYTHONDONTWRITEBYTECODE=1 python3 -I -B scripts/effect-interchange-v1-reference.py
--process-index N` for every `N` in exact ascending order `0..99`. It attempts all 100 children even
if one fails, records each exit status, and launches no child twice. Success requires exactly 100
records, every index exactly once, all exit statuses zero, identical manifest/combined hashes and
unchanged accepted files. Raw evidence is preserved; validated evidence is published atomically and
without overwrite. One successful public-runner invocation is exactly 100 fresh reference
processes, not 100 per format and not 300 subprocesses. No accepted reference may spawn another
process.

Hermetic runner tests use a fake Python executable and temporary fixture tree to prove exact 100
launches, index order, continued collection after a child failure, malformed/missing/duplicate
record rejection, baseline mutation detection, raw preservation, atomic publication, no overwrite,
symlink/hardlink rejection and shell exit propagation. Those fakes do not count as real reference
processes. The actual runner is invoked once only after its checkpoint is committed and Sol XHigh
passes its source/lifecycle review.

## Exact deterministic mutation campaigns

Run exactly 10,000 deterministic trials for each parser: descriptor wire, package and state
selector/parser, for 30,000 total. Use the accepted Issue-002 SplitMix64 algorithm independently per
parser with seeds `0x081d_e5c0_0000_0001`, `0x081d_e5c0_0000_0002` and
`0x081d_e5c0_0000_0003`. Trial `i` starts from comprehensive A when `i` is even and comprehensive B
when it is odd; state uses its sole canonical vector. Consume one `u64`, choose
`offset = value % input_len` and `bit = 1 << ((value >> 32) & 7)`, and XOR exactly that one bit.
Inputs are rebuilt from the frozen fixture for every trial and therefore never accumulate changes.

Each candidate is parsed twice inside `catch_unwind`. A panic, nondeterministic acceptance or a
different complete diagnostic is failure. Rejection requires the exact canonical diagnostic shape,
including zero reserved fields and correct unavailable values. Acceptance is legal: descriptor
identity, package CID/artifact iteration and state selector must be deterministic, every borrowed
slice must remain inside the candidate, and a supported re-encode must equal the accepted candidate
byte-for-byte. Guard canaries and SHA-256 prove the source fixture and bytes outside the exact input
slice remain unchanged. The test reports exact trial/accept/reject/panic counts and a SHA-256 over
the ordered normalized outcomes; acceptance has no required count and no performance threshold.

This deterministic campaign is the bounded fuzz evidence for Issue 081. Existing libFuzzer targets
are locked-check/Clippy compiled only; no open-ended `cargo fuzz`, corpus minimization, crash retry or
time-based fuzzing is authorized. Adding a new parser or changing a seed/count is a rescope.

## Exact migration qualification matrix

Use only portable mock descriptors/factories and accepted Issue-080 APIs. The success matrix is the
Cartesian product of:

- launch rate: `44_100`, `48_000`, `88_200`, `96_000`;
- source layout/current layout: `3->3` (zero step), `2->3` (one step), `1->3` (two steps);
- historical owner: scalar or Wasm/NEON-width-four bank member; and
- destination: scalar or Wasm/NEON-width-four bank member.

That is exactly 48 rows. Every row uses quantum 128, Normal quality, nonempty unequal
common/left/right state, complete ordered initial values and saved request caps. Bypass alternates by
historical owner and link mode is DualMono/Maximum/Average for zero/one/two steps, so replay is not a
vacuous constant. Each row proves exact final canonical bytes, next snapshot/continuation, source
immutability, exact workspace-prefix use, oversized canaries, scalar/bank equality and unrelated
bank-lane isolation. Zero-step rows require zero-length migration prefixes while preserving supplied
oversized suffixes.

Run the existing complete Issue-080 registry and terminal suites unchanged for failure priority,
one-short buffers, provenance, partial writes and by-value disposal. Add only qualification
cross-checks that all 48 rows share the expected descriptor identities, chain count and output
digest. Do not add a third layout, graph search, production DSP case, W8-only product requirement or
new migration hook.

## Exact C/Rust and allocation matrices

C/Rust agreement is limited to the accepted descriptor inspection ABI; this issue adds no package,
state or migration C ABI. Native C11 and Rust must independently assert every size, alignment and
field offset for the six frozen records: parameter `80/4`, port `24/4`, quality `64/8`, enum choice
`16/4`, summary `64/4` and descriptor diagnostic `16/4`. The existing C inspection smoke and Rust
qualification consume comprehensive A and compare counts, identity and every projected record
field/float bit to the canonical wire; comprehensive B remains a Rust/reference vector. Package,
state and migration diagnostics retain their Rust `repr(C)` size/offset tests but are not described
as exported C records. Native exports remain exactly
`miso_engine_effect_descriptor_v1_inspect`.

Allocation instrumentation runs in isolated single-threaded test binaries and records allocations,
deallocations, allocated/deallocated bytes, peak live bytes and surviving bytes for these rows:

1. descriptor verify and identity: temporary heap is permitted, measured exactly on the native
   host, bounded by the accepted 4 MiB input cap and leaves zero live bytes;
2. package required-size, encode, verify and package CID: each equals one matched descriptor-pass
   baseline plus zero package-native allocations and leaves zero live bytes;
3. verified artifact iteration/selection, CID binary/text parsing and caller-buffer text output:
   zero allocations/deallocations and no retained state;
4. prebound state selector, verify, replay/current validation, requirements and encode into caller
   storage: zero allocations/deallocations;
5. migration registry construction and resolution: allocations are control-plane, bounded by the
   exact entry/chain/replay caps, measured and fully released on drop; and
6. bank migration/restore with prebuilt capability and caller workspaces: zero framework
   allocations/deallocations. Scalar success may retain exactly the required owned replay-initial
   slice plus the mock factory's one destination `Box`; dropping both returns live bytes to the
   exact baseline.

Native absolute counts/bytes are descriptive evidence, not portable constants. The zero/delta and
no-survivor contracts are gates. One-byte/count-below inputs, guard pages where supported, prefix/
suffix canaries and before/after SHA-256 cover read-only inputs and atomic caller outputs. Tests do
not invoke render, a benchmark or production DSP.

## Cross-target and static matrix

The exact target rows are:

| Row | Target | Action | Execution claim |
| --- | --- | --- | --- |
| native | host `x86_64` Linux | locked package/compiler tests, mutation, migration, allocation and C smoke | executed |
| Android | `aarch64-linux-android` | locked all-target check for effect-package, effect-compiler and conformance | compile only |
| iOS | `aarch64-apple-ios` | locked all-target check for the same three packages | compile only |
| Wasm scalar | `wasm32-unknown-unknown`, `-simd128` | locked library/object build and export/object inspection | compile/object only |
| Wasm SIMD | `wasm32-unknown-unknown`, `+simd128` | locked library/object build and export/object inspection | compile/object only |

Unavailable installed targets/tools are a HOLD, never a silently skipped PASS. Android/iOS/Wasm do
not claim execution or cross-CPU byte identity. Both Wasm rows retain the single descriptor inspect
export and no other `miso_engine_` export; parser/CID/state/migration Rust symbols may exist in the
object but are not new C exports. Scalar output contains no SIMD opcode. SIMD output may contain
SIMD, but correctness does not depend on it.

Static gates prove the baseline manifest, exact dependency direction, no new unsafe production
surface, no serialization of migration registries, no package/state/migration reference from
core/session/graph/rack/builtins render-owned sources, no qualification tool dependency from a
production crate, no stale API, no untracked generated corpus, and no artifact under a tracked path.
Run workspace, realtime, effect-runtime, rack, graph and builtins policies plus available mutation
suites. No browser runtime, mobile runtime, network, repository, trust or third-party executor is
invoked.

## Frozen benchmark workload and record

Add a separate `miso-engine-effect-interchange-bench` tool. It depends only on accepted
effect-package/effect-compiler/conformance and their already required contract/core dependencies.
It embeds no product implementation and changes no accepted benchmark tool. Its four address-free
workload IDs are:

1. `descriptor_verify_identity_a`: comprehensive-A descriptor verify plus identity;
2. `package_verify_cid_select_a`: comprehensive-A package verify, CID and frozen AVX2/FMA selection;
3. `state_verify_reencode_current`: canonical state bind, verify, replay validation and exact
   re-encode; and
4. `migration_two_step_bank_restore`: portable layout-1 state through two accepted steps into one
   width-four bank member and exact final snapshot.

The no-argument binary reads only the accepted fixture paths, verifies all input hashes before
timing, performs one untimed warmup pass containing each workload once, then measured rounds 1 and
2. Each measured round contains exactly 256 observations per workload; each observation is one
complete operation and inputs/destinations are recreated outside its timed interval where required.
Use `std::time::Instant`, `black_box` and nearest-rank p50/p95/p99/p99.9/min/max in
`ns_per_operation`. There is no threshold, comparison claim or optimization response.

The binary emits exactly eight JSONL records: four workload IDs times rounds 1 and 2, no warmup
record. Every record has an exact closed key set covering schema/issue/workload/round/count/unit,
candidate commit/tree, binary/tool/source/fixture hashes, output digest, Rust/LLVM/target/profile,
CPU/core/OS/kernel/power/governor/background metadata, timer/percentile method, total/min/max and
four percentiles, `descriptive_only=true`, `metadata_incomplete` and sorted unique
`missing_metadata`. Records contain no pointer/address, PID or absolute path. Missing honest machine
metadata is allowed only when named; candidate, binary, source and fixture identities may never be
unknown. Every workload/round output digest must equal its frozen untimed preflight digest.

## Zero-launch preflight and exactly-once lifecycle

Add one validator, one hermetic lifecycle test, one preflight and one public runner. The validator
rejects missing/extra keys, duplicate workload/round rows, nonfinite/nonpositive timing, unordered
percentiles, wrong identities/digests/counts, dishonest missing metadata or anything except eight
records with rounds `[1,2]` for all four workloads.

Lifecycle tests use an inert fake executable and prove: no arguments; exact validator schema;
runner/preflight shell status propagation; absent dependency/tool rejection; existing regular,
symlink and hardlink output rejection; raw partial preservation on nonzero workload exit; malformed,
truncated, extra and duplicate record rejection; validator failure; accepted-artifact atomic
publication; no-clobber after success; and disposition evidence for prelaunch, workload and
postlaunch failures. They never execute the real benchmark main.

On a clean committed candidate, the no-argument preflight:

- requires exact branch/HEAD/tree, clean index/worktree and the accepted/source/fixture/lock hashes;
- runs every nonbenchmark qualification gate or verifies its candidate-bound seal;
- warning-denied release-builds a dedicated benchmark binary into a temporary target directory,
  atomically publishes it under `target/issue081/`, and seals its SHA-256;
- validates a frozen synthetic record through the real validator and reruns hermetic lifecycle
  tests;
- refuses all raw/accepted/disposition artifacts or symlinks that would be overwritten; and
- writes an atomic no-clobber preflight seal declaring `runner_invocations=0`,
  `workload_invocations=0`, `timed_benchmark_invocations=0`, `warmup_passes=1`,
  `measured_rounds=2`, `records_required=8`.

Preflight must never execute the sealed benchmark binary. Sol XHigh independently verifies the
seal and clean candidate, then may explicitly authorize root to run the no-argument public runner
once. The runner revalidates the seal/candidate/hashes, invokes the sealed binary exactly once,
streams stdout directly to a newly and exclusively created raw JSONL file, preserves raw output on
every postlaunch failure, validates it, and atomically/no-clobber publishes the accepted JSONL plus
a disposition record. Success counters are exactly `benchmark_runner_invocations=1`,
`benchmark_workload_invocations=1`, `warmup_passes=1`, `measured_rounds_completed=2` and
`timed_benchmark_invocations=1`.

No retry, tuning, alternate binary, manual record repair, second invocation or performance gate is
permitted. A prelaunch failure may be corrected before workload launch. Any failure after launch is
final evidence for this attempt; preserve the raw/disposition files and open a tooling successor.

## Checkpoints and ordered gates

Checkpoint 1 is the immutable baseline, 100-process/mutation/migration/C-Rust/allocation harness,
target/static scripts and exact docs. Focused tests may use one vector, a tiny mutation count and
fake children only; they must not invoke the real 100-process runner, cross-target script or
benchmark. Sol XHigh reviews, then root commits the exact-path checkpoint.

Checkpoint 2 is the benchmark tool, closed validator, hermetic lifecycle tests, preflight and public
runner. Compile and fake-test only; real runner/workload counts stay zero. Sol XHigh reviews, then
root commits before qualification execution.

Root then runs the real nonbenchmark gates once in this order: baseline/reference 100-process
matrix; exact 30,000 mutation campaign; 48-row migration plus accepted Issue-078/079/080 tests;
C/Rust and allocation/read-only matrix; five target rows; static policies/mutations; format, locked
workspace all-target/all-feature check, locked nonbenchmark tests/doctests, warning-denied Clippy and
rustdoc; artifact/diff/clean-candidate seal. Any failure stops for the one bounded correction or
rescope. Benchmark/timing remains zero.

Only after every nonbenchmark result is committed/evidence-bound does root run the zero-launch
preflight. Only after Sol XHigh PASS on that seal may root perform the single benchmark invocation.
Final docs record exact commit/tree, attempt, process/mutation/matrix counts, C/allocation/target
results, every corpus/tool/source/lock/preflight/binary/raw/accepted/disposition hash, machine
metadata omissions, benchmark records, counters and strict Sol verdict. After the benchmark, only
docs/status/diff/hash sanity may run; no broad or timed gate is repeated.

## Allowed paths and stop conditions

Allowed implementation paths are limited to:

- this spec and its tracked brief plus one exact qualification document;
- new qualification-only tests under effect-package/effect-compiler and a bounded addition to the
  existing descriptor C smoke or its runner if exact agreement requires it;
- `fixtures/effect-interchange/v1/` containing only the sorted baseline/evidence manifest;
- import-safe aggregation around the three existing independent references without changing their
  accepted algorithms or emitted fixture bytes;
- one reference-process runner, one qualification/target/static checker and minimal direct policy
  mutations;
- one new `tools/miso-engine-effect-interchange-bench/` package, one validator, hermetic lifecycle
  test, preflight and public runner; and
- mechanical workspace manifest/lock entries for that tool, with no new third-party dependency.

STOP rather than edit accepted descriptor/package/CID/state/migration product source, fixtures,
wires, APIs, diagnostics, C header or existing Issue-002 benchmark; add a product/runtime
dependency; add package/state/migration C exports; loosen a limit or diagnostic; introduce network,
repository, trust/signature, third-party execution, production DSP, session/graph/render behavior;
claim Android/iOS/Wasm execution; add open-ended/time-based fuzzing; run a process/target/timed
workload during briefing; or repeat the real 100-process or benchmark invocation.

## Dependencies by exact issue title

- Canonical effect package, CID, and artifact selection
- Effect state migration registry and bounded chains
- DSP research corpus and conformance harness
