# Preserve scalar error witnesses with lint-clean normal-feature propagation

Ready-to-number independent, bounded maintenance brief. Discovered by483's broader strict Clippy; existing production source is unchanged by483. Do not expand483 or relax its dependency gates. Root must number/synchronize before edits and serialize this single file with active builtins-compiler work; #238 input-stage work is a different file but root owns resource scheduling.

Problem: three `if let Err(error) { cfg(test/test-support) witness; return Err(error) }` sites become plain manual error propagation when test support is disabled, triggering question_mark. They are ConsoleMatrixProcessor::process (~3591), ConsoleFaderProcessor::process (~3648), and ScalarPairProcessor::process's FIRST fader drain (~3735). Preserve all test-support observations; do not apply ? blindly and drop them.

Allowed source: crates/builtins-compiler/src/lib.rs, those three blocks only, plus numbered evidence. Use Result::inspect_err with the existing cfg-gated observation followed by `?` (or equally small explicit cfg-separated propagation if needed). The witness must execute only on the existing error, before return, observe exactly the same state, and not cause a second drain. Normal builds must remain free of witness access. No global lint allow, new public API/helper framework, queue/arithmetic/resource/layout change or unrelated formatting.

Keep the later matrix-drain error branch in ScalarPairProcessor UNCHANGED: it intentionally processes fader arithmetic before returning the matrix error. Keep fader drain before envelope validation and envelope rejection before matrix drain. Preserve original errors and consumed queue prefixes.

Finite gates: strict `cargo clippy --locked -p builtins-compiler --all-targets -- -D warnings` with normal features AND `--features test-support` (before --), plus affected existing scalar error/FIFO/state tests in debug/release using the current nonempty exact names selected from the delivered source. Exact existing filters are tests::scalar_invalid_envelope_leaves_the_later_matrix_queue_untouched, tests::actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners, and tests::actual_scalar_graph_preserves_scheduled_matrix_prefix_error_and_queue_tail. Invoke each with `cargo test --locked -p builtins-compiler --features test-support --lib FILTER -- --exact` and matching --release; require one actual test each. Preserve those names in the numbered baseline, not a new test corpus. Check existing mutation witness still reports the same error-time state via unchanged fixture assertions; no new mutation campaign. Run fmt/diff and relevant realtime policy; retain initial full Clippy101 and corrected commands/statuses. No timing/workload/artifact repin; mechanically equivalent propagation does not justify a new benchmark or independent broad matrix.

Astra scopes/reviews, Luna1 then Sol2/3 on FAIL, hardstop after3. Root owns checkpoint, source-equivalence/applicable prior qualification, actual-head PR review, requiredCI and remote closure. This brief does not authorize implementation until numbered.

## Numbered baseline

GitHub485 has the matching number/title. Source base is delivered main024ad674789a96390bcc45a754931ef5119c8b59. This independent three-block maintenance slice does not overlap238 builtins source or483 floor tests. Pending numbered Astra approval before Luna1. No timing or broad optimization is authorized.

## Numbered scope approval and Luna attempt 1

# Astra #485 numbered scope review — PASS

Exact head8bd6e316738f29c0fd627db98d3a2eb63ef43b11, engine-485-error-propagation, based on delivered024ad674789a96390bcc45a754931ef5119c8b59. Only the numbered spec differs from base. The approved narrow brief body is retained with the matching numbered title/baseline. Live GitHub485 is OPEN with matching number/title; root reports matching body synchronization.

Approve fresh Luna1 for the three exact builtins-compiler error-propagation blocks. Preserve error-only state witnesses, single drain, queue/error order and normal/test-support behavior. The later matrix-error branch that finishes fader arithmetic is expressly excluded. Strict Clippy in both feature configurations and the three exact existing tests in debug/release are finite sufficient gates; no new mutant campaign, global suppression, production API or artifact/timing work enters scope.

Source base preserves the reviewed three sites and fixtures; #238 remains the active runtime feature and touches a distinct file. Root owns checkpoint/synchronization and later actual-PR/requiredCI. One Luna pass, Sol2/3 on failure and hardstop after3 apply. No tests/builds/timing or repository/Git/GitHub edits performed.

Root adopts scope PASS and assigns fresh Luna1. Pause at the first compiling/focused-green exact-path checkpoint. Root owns Git and remote delivery.

## Luna attempt 1 checkpoint and gate results

Source79a4da0a changes exactly the three approved propagation sites. All three exact existing tests execute once and pass in debug/release; feature-supported all-targets Clippy, formatting, diff and realtime policy plus its mutation suite pass. Initial malformed release commands incorrectly placed --release after the test separator; their101 records remain, and corrected command records show actual release tests passing.

Required normal-feature all-targets Clippy returns101 due seven inherited E0425 references from cfg(test) code to phase-two allocation helpers gated only on feature=test-support. `/tmp/485-luna1-clippy-normal.log` retains this distinct failure; it is not a gate PASS. Root normal-feature library-only strict Clippy has captured numeric0 in `/tmp/485-root-clippy-normal-lib.*`, distinguishing repaired propagation from the test-feature mismatch without substituting it for the frozen all-targets gate. Pending consolidated Astra verdict and explicit scope ruling before further edits.

## Astra attempt 1 verdict and bounded scope amendment

# Astra #485 Luna attempt 1 — FAIL (qualification blocker; three-site source accepted)

Exact reviewed head8565ad67af5bf4c67a394e215bc592b55ba9c645, source79a4da0a, engine-485-error-propagation. Read full numbered scope, cumulative diff and retained logs/statuses. No builds/tests or repository/Git/GitHub mutations performed.

The three approved inspect_err(...)? rewrites are semantically correct: each performs exactly one existing drain, observes the same state only on Err before returning that same error, and has no normal-feature witness access. ConsoleMatrix/ConsoleFader envelope and arithmetic order are unchanged. ScalarPair's fader drain still precedes envelope validation; its later matrix-error branch still performs the fader arithmetic before returning and is byte-unchanged. No production repair is requested for these blocks.

Retained debug and corrected release commands execute each of the three frozen exact tests once, all status0. Feature-supported all-targets strict Clippy, fmt/diff/realtime policy and its mutation suite also return0. Initial malformed release commands return101 and remain historical failures. Root's normal-feature library-only strict Clippy returns0; it usefully separates the repaired propagation from the inherited test configuration defect, but cannot replace the required all-targets gate.

The frozen normal-feature all-targets Clippy remains101 with seven E0425 errors. The unconditional cfg(test) helper prepared_pair_graph_variant_observed returns TestPhaseTwoAllocationSnapshot options and references begin/snapshot/reset around lines5734–5811, while that type and functions are feature=test-support only. These sites and their dependency gates predate this three-block change (confirmed by cumulative diff). This is a real missing configuration, not a new propagation defect or acceptable gate waiver. Therefore the attempt is FAIL as a complete delivery candidate.

## Explicit bounded repair ruling before Sol2

Root may amend existing485 to include this necessary test-compilation dependency closure in the SAME source file before assigning Sol2. It does not require a new product subsystem or framework; if root elects a separate numbered child,485 must remain open until its required gate passes. Do not silently broaden the current three-block permission.

Smallest coherent route: make the existing phase-two observation dependency closure available under cfg(any(test, feature = "test-support")), preserving normal non-test/non-feature exclusion. Exact allowed dependency scope is the AtomicBool/Ordering and Mutex imports; TEST_PHASE_TWO_ACTIVE and TEST_PHASE_TWO_LAYOUTS; TestPhaseTwoLayoutTable and its impl; TestPhaseTwoAllocationSnapshot; existing reset, record-allocation, record-deallocation, snapshot, begin functions; TestPhaseTwoAllocationGuard plus its begin/Drop impls. These are the existing cluster around1275–1463 and guard Drop around1480. No blanket replacement of every test-support gate in this file: scalar owner layout export, other resource APIs, allocator installation, observation call locations and feature-only owner fixtures stay unchanged unless a separately identified direct dependency is demonstrated before edit. Keep all function bodies, allocation/layout arithmetic and existing helper return shape unchanged. The record functions belong to the closed cluster, avoiding an orphaned private table-method configuration.

Do not gate out the ordinary scalar tests/helper to make Clippy pass, return fake empty snapshots, enable test-support as a default feature, install an allocator in normal builds, add lint allows, or turn feature-only owner tests into uninstrumented tests. Ordinary no-feature unit fixtures currently pass observe_binding=false; enabling declarations does not itself claim allocator observation there. Feature-supported owner/allocation evidence must retain its existing real instrumentation.

Finite Sol2 gates: both original normal and test-support all-targets strict Clippy must return0; run the original three exact tests in debug/release with test-support, and the same three in normal-feature unit mode to demonstrate the formerly broken configuration actually compiles and executes (one test each). Retain fmt/diff/realtime checks. Check the final diff is only the accepted three rewrites plus enumerated cfg dependency changes/spec; no further mutant campaign, workspace matrix, benchmark or artifact regeneration is required by this repair. Root still owns later exact-head PR/requiredCI and proportional qualification. Luna1 ends here; Sol2 follows only after the amendment is synchronized.

Root adopts this explicit dependency-closure amendment and assigns Sol attempt2 after synchronization. Accepted three propagation rewrites remain unchanged. The enumerated cfg declarations and twelve exact existing test executions across two feature modes/two profiles are binding; no other scope expansion is authorized.

## Sol attempt 2 acceptance and evidence

# Astra #485 Sol attempt 2 — PASS

Exact reviewed head d0bafe8bd669ddbba659236ae988f8d4755f9cca in engine-485-error-propagation. Read amended numbered scope, prior FAIL, cumulative source diff and actual retained command/log/numeric-status evidence. No builds/tests/timing or repository/Git/GitHub writes.

The previously accepted three inspect_err propagation blocks remain unchanged. Sol2 adds exactly15 cfg substitutions in the enumerated phase-two observation dependency closure: two imports, two statics, table type/impl, snapshot type, reset/record-allocation/record-deallocation/snapshot functions, guard type/impl/begin function and guard Drop impl. All now use any(test, feature="test-support"); function bodies, arithmetic and allocation semantics are unchanged. The normal non-test/non-feature build still excludes this observation machinery. No blanket cfg replacement, default feature, allocator installation, fake snapshot, suppressed test or public production API change occurred. Feature-only scalar owner layout/other resource APIs and owner instrumentation retain their existing gates.

Both original all-targets strict Clippy modes now return0, including the formerly broken normal-feature lib-test configuration. Twelve retained exact executions cover the three frozen error/FIFO/state tests across normal/test-support and debug/release; every log reports one passing test, zero failures. Fmt, diff-check, realtime policy and its existing mutation suite return0. These numeric records primarily use the .actualnumericstatus suffix, not uniformly .status; preserve their actual filenames/attribution. Successful Clippy logs still contain inherited invalid-path warnings; no warning-free claim is made.

The content-aware scope audit returns0 and its15-cfg/3-propagation counts agree with independent diff inspection. The earlier source-diff command returned1 because it asked for no differences across the intended source-change commit; it is retained as an inapplicable check, not relabelled PASS. Preserve the reported shell quoting/EOF failure without inventing a captured status, and preserve Luna's earlier feature mismatch and malformed release invocations.

The original error-only witness timing, single drain, queue prefix/error identity, envelope order and later matrix-error fader processing remain accepted. The required gate was repaired rather than replaced by library-only Clippy. Source PASS permits root's proportional delivery packaging and exact-head actual PR review, then required CI/remote closure. No new benchmark, artifact repin, broad matrix or runtime optimization is authorized by this maintenance result.

Raw history and captured final gates are retained under artifacts/issue485-scalar-error-propagation with hash/size coverage. Root integrates delivered main before actual PR review; no timing or broad runtime change is claimed. Issue485 remains OPEN pending required CI and verified remote delivery.
