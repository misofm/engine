# 081 Canonical effect interchange qualification, fuzzing, and benchmark

## Outcome and status

Qualify the accepted descriptor, package/CID, state and migration products as one portable
interchange boundary without changing their bytes, APIs or product implementation. **SOL XHIGH
TERMINAL BENCHMARK FAIL / STOP; SOLE RUNNER INVOCATION CONSUMED; NO RETRY OR OVERALL PASS.** The
nonbenchmark qualification remains valid, but Issue 081 cannot close successfully. This is a
qualification/tooling failure, not a product failure.

Use one coherent Sol High implementation attempt and at most one bounded Sol High correction, each
adversarially reviewed by Sol XHigh. A second failed implementation pass stops and requires a
stateless rescope/rebrief; gates are never weakened. At briefing,
`reference_process_invocations=0`, `mutation_campaign_invocations=0`,
`cross_target_invocations=0`, `benchmark_preflight_invocations=0`, `benchmark_runner_invocations=0`,
`benchmark_workload_invocations=0` and `timed_benchmark_invocations=0`.

Qualification execution first recorded `cross_target_invocations=1` on candidate `709b3d2ccc6d`.
The native execution row and Android and iOS compile rows completed. The scalar Wasm check, package
`rustc`, object creation and `wasm-objdump -x` completed, then the harness stopped because its
export selector also treated the module/name `<miso_engine_effect_package.wasm>` as an export. The
scalar opcode inspection and entire SIMD Wasm row did not run. This was a harness false positive,
not a product or target failure. The bounded correction committed as `4cb3b5c` admits only explicit
function exports inside the Wasm Export section and has a synthetic missing/extra/duplicate/
wrong-kind/reference regression. Sol XHigh authorized one corrected full rerun; target invocation
2 passed all five rows. The accepted product and target artifacts remain unchanged.

## Qualification and nonbenchmark evidence

The completed clean nonbenchmark candidate is commit
`4cb3b5c3a97361218f474700751653c4400dc08d`, tree
`9aec9ade2645057cf2c93986a0d0eb47658df7d1`. Sol XHigh independently confirmed that HEAD/tree,
clean index/worktree, the absent benchmark artifacts and these identities:

- `Cargo.lock`: `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`;
- `fuzz/Cargo.lock`: `af4547d5bae367e4249c6fcf482b249ff8af0ae29b9a933957d34b36ec36e5d5`;
- accepted baseline: `6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5`;
- qualification checker: `bde208b34413dd4e7e10fc27c2a85019300d61860c5055d5b081a949a704f970`;
- target matrix: `3edeacbbf6571bacfb87807ab6cf9d15612babf895c5215928fff1b3b0d3bae9`;
- reference runner: `026aa241b5146480fc393279f0fea4326c1b3172da81cadbf5750d186268014e`;
- benchmark runner: `4aca5153928bfee583cf5ea403483b63f848e4fb6a83045800424bc855a80429`;
  and
- zero-workload preflight: `3957a02b8e5d45efd3e3637c60fc04157180c555fb46b0aa0eee4157afa3029c`.

The sole reference-runner invocation launched exact children `0..99`; every child exited zero and
emitted one record. Raw and accepted evidence are byte-identical 100-line files with SHA-256
`0946cb00a980d7c94bdc37043d4384392d62e57994f7c5efcbb7e5bb4b924bb3`; the 100-row status file has
SHA-256 `f4033cc066e7239664498a034d43ce05b3cb30581f9c2fb0ec3d749e5ab9ca51`.
The exact 30,000-trial mutation campaign passed once, with ordered normalized-outcome hashes:

- descriptor: `02d88fc02583926a1e53ffe56ae08d17bffe9039f8e75cefef70fefb07c34155`;
- package: `fc8ea16692695dac08b29b64b5d7394c53ca70448ad3abc7c5c7994d289f7714`;
  and
- state: `1a153e0fe665d837deec13e014d442baeac49658baf8d3f927b5ddaef34a6ca2`.

The exact 48-row migration matrix passed once with aggregate hash
`f834c9447fb57e3e93408a69285e2a42b3bf94422ce7c4eb23dc205333849f46`. Native C11/Rust agreement
passed for all six accepted descriptor ABI records and comprehensive-A projections. Isolated
allocation evidence passed with these native measurements: descriptor/package publication `8/8`
allocations/frees and `1000/1000` bytes, peak `736`, live `0`; postverify/prebound state `0`;
registry construction/drop `4/4`, `864/864` bytes, peak `864`, live `0`; resolution and prebuilt
bank restore `3/3`, `88/88` bytes, peak `48`, live `0`; normalized scalar success delta `2/0`,
`160/0` bytes, peak/live `160`; and its drop delta `0/2`, `0/160` bytes, returning live bytes to
baseline. Static policies and mutations, tracked-shell syntax, conflict/trailing-whitespace,
artifact and diff checks passed. Target invocation 2 passed the native execution, Android/iOS
compile-only and scalar/SIMD Wasm compile/object-only rows; it makes no mobile, Wasm or cross-CPU
execution claim.

The broad nonbenchmark seal is candid: format and locked workspace all-target/all-feature check
passed. The first locked workspace nonbenchmark test process showed only passing output, but its
parent evidence stream was lost after process completion, so that invocation is mechanically
inconclusive and is neither PASS nor failure evidence. Root repeated exactly
`CARGO_BUILD_JOBS=1 cargo test --workspace --all-features --locked --lib --bins --tests --examples`
with a retained session; it exited `0` with only the expected ignored/manual rows. Workspace
doctests exited `0` with eight compile-fail doctests; warning-denied workspace all-target/all-feature
Clippy and warning-denied workspace all-feature rustdoc exited `0`. Final HEAD/tree, clean index/
worktree, conflict, whitespace, artifact and diff checks passed.

Final nonbenchmark counters are `reference_process_invocations=1`,
`mutation_campaign_invocations=1`, `migration_matrix_invocations=1`,
`cross_target_invocations=2`, `benchmark_preflight_invocations=0`,
`benchmark_runner_invocations=0`, `benchmark_workload_invocations=0` and
`timed_benchmark_invocations=0`. Those were the final counters before benchmark preflight.

## Terminal benchmark attempt evidence

Root committed the documentation evidence and created the required candidate-bound nonbenchmark
seal on clean commit `466b05cbf2bb61e0367d25aa6ca6a0da7643e83f`, tree
`2e1c5c12515e7b16d8a36846130cfe4cde42ad55`. The sole zero-workload preflight invocation exited
`0`; its output reported workload/timing counts `0`. The retained preflight artifacts are regular,
one-link files:

- nonbenchmark seal: 833 bytes, SHA-256
  `6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c`;
- sealed executable: 827,232 bytes, SHA-256
  `fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c`; and
- 22-key preflight seal: 1,577 bytes, SHA-256
  `da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3`.

Sol XHigh independently recomputed every transitive candidate/tool/source/fixture/lock/validator/
lifecycle/preflight/runner hash in that seal and confirmed the four frozen output identities,
`runner_invocations=0`, `workload_invocations=0`, `timed_benchmark_invocations=0`, planned warmup
`1`, rounds `2` and records `8`. No raw, accepted, stderr, disposition or prelaunch artifact existed.
Sol XHigh therefore authorized exactly one invocation of
`bash scripts/run-effect-interchange-benchmark.sh`, with no retry or alternate/direct invocation.

That sole authorized runner invocation exited `1` with zero command stdout. Its terminal artifacts
are preserved unchanged:

- raw JSONL: regular, one link, 0 bytes, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- stderr: regular, one link, 361 bytes, SHA-256
  `442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93`;
- disposition: regular, one link, 817 bytes, SHA-256
  `8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de`;
- accepted JSONL: absent; and
- prelaunch disposition: absent.

The disposition is an exact terminal `FAIL` with reason `workload_failed`, candidate/binary/
preflight/raw/stderr identities matching the retained files, and counters
`benchmark_runner_invocations=1`, `benchmark_workload_invocations=1`,
`timed_benchmark_invocations=0`, `warmup_passes_completed=0`,
`measured_rounds_completed=0`. Stderr contains only the `workload_started` phase followed by a panic
at `tools/miso-engine-effect-interchange-bench/src/main.rs:450` while obtaining descriptor wire
requirements: `EffectDescriptorWireDiagnosticV1 { code: Semantic, byte_offset: 0,
record_index: 4294967295, required_bytes: 0 }`.

Terminal real counters are `reference_process_invocations=1`,
`mutation_campaign_invocations=1`, `migration_matrix_invocations=1`,
`cross_target_invocations=2`, `benchmark_preflight_invocations=1`,
`benchmark_runner_invocations=1`, `benchmark_workload_invocations=1` and
`timed_benchmark_invocations=0`.

The exact cause is benchmark-tool-local and deterministic. `MIGRATION_Q1`, `MIGRATION_Q2` and
`MIGRATION_Q3` each contain only `migration_quality(48_000, layout)`. The accepted
`validate_descriptor_v1` contract requires every advertised quality to contain all four launch
rates `44_100`, `48_000`, `88_200` and `96_000`; `effect_descriptor_wire_v1_required_size` maps that
semantic rejection to the observed offset-0/unavailable-index diagnostic. The first three frozen
untimed workloads completed in memory, but the program emits no stdout records until after both
measured rounds. Migration descriptor construction therefore panicked during the frozen untimed
correctness pass, before `warmup_complete` or `timed_started`, exactly explaining empty raw output
and the terminal counters. The product descriptor validator behaved correctly. The benchmark-only
fixture was invalid, and its frozen migration digest was unreachable in this binary.

Compile-only qualification and the hermetic fake lifecycle could not expose this invalid real-main
fixture; the zero-launch preflight deliberately built but did not execute it. Issue 081's rule is
unambiguous: any failure after the runner launches is final evidence. Do not delete or overwrite
these artifacts, rerun preflight/runner/binary, repair a record manually, or claim an Issue 081
benchmark PASS.

## Required successor/rescope recommendation

Create a new stateless issue, not an Issue 081 retry:

- number/title: **108 Repair effect-interchange benchmark migration fixture and reauthorize one
  descriptive run**;
- local path:
  `.github/ISSUE_SPECS/108-repair-effect-interchange-benchmark-migration-fixture-and-reauthorize-one-descriptive-run.md`;
- dependencies by exact title: **Canonical effect interchange qualification, fuzzing, and
  benchmark**; **Prepared effect state envelope and transactional current-layout restore**;
  **Effect state migration registry and bounded chains**; and **Close canonical effect descriptor
  wire, identity, and C inspection ABI**.

The successor's smallest slice is benchmark-tool repair only. Preserve every Issue 081 terminal
artifact and all accepted product/reference/fixture bytes. Give D1/D2/D3 complete sorted four-rate
quality tables for their respective layouts; add a focused nontimed executable regression that
validates all three descriptors and the exact two-step final envelope before any one-shot
authorization; independently recompute and bind the migration digest across tool/checker/preflight/
runner/lifecycle; and use a successor-specific no-clobber artifact namespace so Issue 081 evidence
cannot be overwritten. Run only proportional compile/lint/fake/static gates, then a new zero-launch
preflight and Sol XHigh review. A later descriptive invocation belongs solely to Issue 108 and
requires new explicit authorization; it is never described as an Issue 081 retry. Do not rerun the
100-process, 30,000-mutation, 48-row, target or broad nonbenchmark matrices merely to repair this
qualification tool.

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

## Retirement note (#104 phase A, 2026-08-24)

#104 phase A (#83 W4-D2): `fixtures/effect-interchange/v1/ACCEPTED.sha256` sealed twelve `crates/miso-engine-effect-{compiler,package}` source files alongside the interchange corpus. Waves 1-4 rewrote six of them (`effect-compiler/src/prepare.rs`, `effect-package/src/{diagnostic,ffi,lib,package,wire}.rs`), so the source half of the accepted baseline went permanently red and is not refreshable without re-running this qualification. The twelve source rows are retired; the manifest is now the 24 corpus/reference-script rows, all of which still verify byte-for-byte. Accepted manifest identity: `6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5` (36 rows, this issue) -> `1aaa96dc731c0da3dabb2f8ecd7c2bf803078b580a38cccfccf1ffe280c83588` (24 rows, #104). No corpus byte changed.
