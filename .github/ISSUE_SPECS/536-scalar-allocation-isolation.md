# Isolate scalar Point allocation probes from libtest bookkeeping

Status: scope preparation for a bounded test-only correction blocking PR #535. Gate issue #534 retains its conditional source PASS; no allocation assertion or CI gate may be weakened. Work begins from PR head cc5f04fb3bd98052fc4d3f8edbb814718259a543 on codex/scalar-allocation-isolation, a separate checkpoint branch. Root owns Git/GitHub. Implementation uses Luna high/xhigh; this allocation/realtime evidence requires Astra medium review. One consolidated verdict per attempt, maximum three attempts.

## Problem and bounded outcome

Required qualification34080063161 workspace job101613609720 failed in preparation_resources_and_success_path_are_bounded at host-core/tests/scalar_point_endpoint.rs:1369: endpoint31 allocations versus direct27. Bytes/reallocations/frees were not reached. The original job output is retained; no rerun-to-green or historical-cause claim is permitted.

The fixture counts process-wide allocations in an isolated libtest child. Other test processes cannot pollute that child, but its harness thread can. The pinned Rust libtest implementation spawns a test worker and then updates running_tests/timeout_queue on the concurrent scheduling branch even for one exact-filtered test. Explicit --test-threads=1 avoids that branch but still creates a test thread. This is a concrete opportunity for contamination, not proof of which thread made the historical four allocations. Host/compressor/protocol/allocator/build inputs are unchanged by the gate feature.

Smallest closable outcome: substantiate the measurement boundary and remove the identified child scheduling interference while preserving strict preparation allocation/byte equality, the combined-controller +2 allocation/+2 free/+9 byte authority, counter liveness, and zero-allocation/free realtime assertions. This is separate qualification/tooling work, not a gate DSP expansion.

## Exact scope and existing mechanisms

Only implementation path: crates/host-core/tests/scalar_point_endpoint.rs. Use its two existing child launchers and existing bench_support::alloc process counters plus engine::realtime::audit thread-local counters. Compact test-only diagnostics/control may live in this same file. No second allocator, generic counter framework, new production API, Cargo/dependency, environment vocabulary, CI/policy, numerical/corpus/resource expectation or effect code change. Existing admission, cancellation, PCM and current/target gates remain intact.

Relevant existing tests: preparation_resources_and_success_path_are_bounded and controller_scalar_preflight_rejections_and_single_allocation_authority. Preserve all current assertions. Do not replace global equality with a looser bound or subtract unexplained allocations. Do not claim preparation becomes realtime-safe: a warmed thread-local auditor in Count mode is diagnostic attribution only, with preparation still on the control path.

## Frozen discriminating sequence

First read this numbered/current-base brief and obtain Astra approval. Luna then adds only the compact diagnostics/control tranche before any child-scheduling correction. Warm/reset auditor state outside measurement windows; retain owners throughout; capture full global deltas and corresponding same-thread audit allocation/free counts for each direct and wrapped preparation, printing only after both windows. Verify that instrumentation preserves the actual preparation path; if a render-scope diagnostic changes control behavior, stop for a bounded ruling rather than change production.

Capture one fixed debug comparison for each of the two exact children at explicit inherited child concurrency2 versus1 (four invocations total), retaining all commands/source identities/deltas/statuses. Merely passing does not establish cause or correction. Parent Cargo test options alone do not alter internally constructed child arguments. Before the fix, use the existing Rust test-thread setting inherited by the child, with exact argv/environment recorded; do not introduce a new environment key or a retry loop.

Add one deterministic control using a prestarted, explicitly synchronized foreign-thread allocation while measuring the current thread. Allocate all synchronization setup before marks, preserve the controlled allocation through a real black-box use, and synchronize completion before reading deltas. Global totals must move while measured-thread audit allocation/free counts remain zero. Existing own-thread positive probes must still move the auditor. This proves contamination sensitivity, not attribution of the historical four allocations. No sleeps, probabilistic race test or new harness.

Pause at a coherent compiling diagnostic checkpoint for root commit/push. If a useful diagnostic checkpoint intentionally retains a reproduced failure, document it candidly; never hide it. If same-thread excess is found, or no bounded evidence supports the proposed isolation, stop and rebrief; do not quietly apply the scheduling fix.

Only with the diagnostic evidence and a bounded Astra decision, add --test-threads=1 to BOTH existing child commands. This removes the identified libtest scheduling branch; it does not claim an entirely single-threaded process or immunize arbitrary future worker creation. Retain strict original global counts and all other gates. No numerical budget or tolerance changes.

## Finite gates and evidence

Capture complete source/cwd/argv/environment/stdout/stderr/numeric status for the frozen comparison, deterministic control, and final commands. After the bounded correction run both exact parent/child selectors in debug and release, including an explicit inherited concurrency2 check to demonstrate the child argument overrides it; run the complete nine-test scalar_point_endpoint binary once. Use --locked and --features control-provider. Run affected strict Clippy and format checks. No full workspace repetition, browser rebuild, benchmark, new target matrix or generic harness suite is part of this issue; required PR CI remains mandatory integration evidence. Extra diagnostics are a stop/split condition, not an unlimited investigation.

Astra medium performs one consolidated review of the complete attempt against measurement attribution, preserved contracts and objective evidence. No source acceptance or flaky-test disposition may rest solely on a later passing run. Preserve the original CI failure and all local failures.

## Delivery and recovery

Keep PR535 head fixed while this package accumulates; root commits and promptly pushes coherent checkpoints on the dedicated branch (ordinary feature pushes do not trigger required CI). After this issue's review PASS, root may fast-forward the existing gate PR branch to the complete combined package once, update its description/evidence around both final outcomes, and obtain exact-head/current-base review plus required qualification SUCCESS. No source history is rewritten. Assert live main immediately before exact-head merge, verify actual parents, and close/synchronize both issues only when their accepted evidence is upstream in that delivery workflow. No failed gate is waived and no issue is counted delivered while PR535 is blocked.

## Primary evidence

- Original CI failure: https://github.com/misofm/engine/actions/runs/34080063161/job/101613609720
- Pinned libtest implementation: https://raw.githubusercontent.com/rust-lang/rust/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/test/src/lib.rs
- Existing local allocator/auditor contracts and the two current fixture implementations are authoritative; this issue changes no DSP algorithm, coefficients, latency, tail, smoothing or listening claim.

## Numbered/current-base approval

**PASS — #536 authorizes only the diagnostic first tranche.** No brief blockers or unnecessary expansion of the retained triage.

Verified clean local HEAD/upstream `ca6229bd6e75c0a072927d4ddff07abf98e4aee5`, based on unchanged PR535 head `cc5f04fb3bd98052fc4d3f8edbb814718259a543`. GitHub #536 is OPEN with matching title and exact local-spec body. The retained failure stdout matches the original log byte-for-byte. Required run34080063161 remains failed.

The brief correctly bounds implementation to `crates/host-core/tests/scalar_point_endpoint.rs` using existing allocation counters, TLS auditor and child helpers:

- Capture exactly four debug comparisons: two existing children × inherited concurrency1/2, **before** changing child arguments.
- Pair global and same-thread diagnostics without changing preparation behavior; keep setup, printing and owner destruction outside measurement windows.
- Use deterministic foreign-thread allocation to demonstrate counter contamination sensitivity, preserving own-thread counter liveness.
- Preserve every original allocation/byte, realtime, resource and admission assertion.

The diagnostic checkpoint must receive the specified bounded review before adding `--test-threads=1` to both launchers. Passing comparisons or the synthetic control alone cannot establish the historical cause. Same-thread excess or insufficient supporting evidence requires stopping and rebriefing.

Finite gates, the three-attempt limit, Luna high/xhigh implementation, Astra medium verification and one completed-package integration into PR535 are appropriate. No production, Cargo, CI, policy or expectation changes are authorized.

This is scope approval only—not implementation acceptance, historical-cause attribution or merge authorization.

## Attempt 1 diagnostic checkpoint (Luna xhigh)

The frozen four debug comparisons each ran once and passed before any child argument change. Resource preparation: direct and wrapped each 27 allocations, 0 frees/reallocations, 102496 bytes; same-thread counts each 27/0 under inherited concurrency 1 and 2. Controller preparation: direct 31 allocations, 0 frees/reallocations, 120280 bytes; wrapped 33 allocations, 2 frees, 0 reallocations, 120289 bytes; corresponding same-thread counts agree under both settings. Original strict assertions remain.

The synchronized foreign-thread control recorded process-global 1 allocation, 2 frees and 4096 bytes while measured-thread counts remained 0/0. The extra foreign deallocation is retained honestly. This proves global-counter contamination sensitivity; passing comparisons do not reproduce or attribute the historical four extra allocations. No child scheduling change has been made. A preliminary invocation used a nonexistent worktree path and launched no test; all four actual comparisons are retained exactly once.

Source SHA-256: eafda5a261d900b89f463c91bbf12a050195c83f891755fd57aad0d4064add90. Full commands, inherited environments, dirty-source identity, numeric status and stdout/stderr are in artifacts/issue536-luna-attempt1. Implementation is paused for the required bounded Astra medium diagnostic decision.

## Bounded diagnostic decision — Astra medium

**Approve the bounded isolation correction:** add `.arg("--test-threads=1")` to **both existing child commands**. No stop/rebrief is required by this diagnostic evidence.

Verified clean HEAD/local upstream/remote branch at `44f9e9c73abeaaf778507db9ce83d0eaa7b99e53`. GitHub #536 is OPEN with matching title and exact spec body. The current test-source SHA-256 matches all four captures: `eafda5a261d900b89f463c91bbf12a050195c83f891755fd57aad0d4064add90`.

Evidence supporting this decision:

- All four pre-correction comparisons returned status0 and executed their child tests. Scalar direct/wrapped both recorded 27 allocations and 102496 bytes; controller preserved the exact +2 allocations/+2 frees/+9 bytes. TLS allocation/free counts agree with global counts.
- Warm/reset occurs before measurement; owners survive the windows; printing occurs afterward. Count mode records allocations while retaining the same System allocator and preparation calls. The audited scope does not select a different preparation control path. Original strict global, realtime, resource and admission assertions remain intact.
- The synchronized foreign-thread control recorded global **1 allocation/2 frees/4096 bytes**, with measured-thread TLS **0/0**. Worker teardown can overlap the snapshot; the extra free is correctly retained without subtraction or exact attribution.
- Pinned libtest starts the worker before concurrent-branch map/queue bookkeeping. Explicit concurrency1 bypasses that branch, providing a concrete reduction in interference opportunity.

The comparisons did **not** reproduce or explain the historical 31-versus-27 failure. The synthetic control demonstrates susceptibility, not historical causation. The correction also does not make the child process entirely single-threaded.

Luna high/xhigh may make only the two argument additions and necessary formatting. Complete the spec’s finite gates: both exact selectors through parent/child in debug and release, inherited concurrency2 override verification, the full nine-test binary once, affected strict Clippy and formatting, using `--locked --features control-provider`. Preserve complete captures and every failure.

This is the diagnostic decision only; consolidated attempt review and required PR qualification remain outstanding.

## Attempt 1 correction checkpoint

Luna xhigh added --test-threads=1 to both existing child launchers and applied formatting. The first corrected resource selector passed in debug with inherited RUST_TEST_THREADS=2, parent and child each 1 pass. Direct/wrapped counts remain 27 allocations, 0 frees/reallocations and 102496 bytes; measured-thread counts agree. Source SHA-256: 15dedf93497aacf0304b756ed39ea30383c2e8b037b4a4811e1fc155d3fab246. Remaining frozen gates and consolidated review are pending.

## Attempt 1 finite final gates

All seven corrected invocations passed: both exact allocation selectors in debug and release, each with inherited RUST_TEST_THREADS=2 and explicit child concurrency1; the full nine-test scalar_point_endpoint binary once; affected strict Clippy; and cargo fmt --all --check. Each exact invocation executed one parent and one child. Resource equality remained 27 allocations/0 frees/0 reallocations/102496 bytes. Controller direct 31/0/0/120280 versus wrapped 33/2/0/120289 preserved the original +2 allocations/+2 frees/+9 bytes. The foreign-thread control retained global 1 allocation/2 frees/4096 bytes versus measured-thread 0/0. Full captures use source SHA-256 15dedf93497aacf0304b756ed39ea30383c2e8b037b4a4811e1fc155d3fab246. No further source change or test repetition followed these passing gates. Consolidated Astra medium attempt 1 review and required PR CI remain pending.

## Consolidated attempt 1 verdict — Astra medium

**PASS — consolidated attempt 1 for issue #536.** No remaining implementation blockers within the approved scope.

Verified clean local HEAD, upstream and remote branch at `6b9cf41a566e59a986a8129035ce7591a1f1690e`. GitHub #536 remains OPEN with matching title and exact spec body. Changes from `cc5f04fb` are limited to the existing test file, numbered spec and evidence; gate534 and all production/build/policy sources remain unchanged.

- Every original strict allocation/byte assertion remains, including controller **+2 allocations/+2 frees/+9 bytes**, counter liveness, zero realtime work, resource and admission contracts.
- Diagnostics preserve preparation calls and owner lifetimes. Warm/reset and setup precede measurements; printing and owner destruction follow them. Count-mode auditing does not select another control path.
- The synchronized foreign-thread control demonstrates global-counter sensitivity with TLS **0/0**. Its observed **1 allocation/2 frees/4096 bytes** is retained honestly; worker teardown can overlap the snapshot.
- Both child launchers explicitly set `--test-threads=1`, overriding inherited `RUST_TEST_THREADS=2`.

All **seven corrected captures returned status0**: both exact selectors through parent/child in debug and release, the full **9-test** binary once, strict Clippy and formatting. All match current source SHA-256 `15dedf93497aacf0304b756ed39ea30383c2e8b037b4a4811e1fc155d3fab246`; later commits add only records/spec updates. The four original diagnostic comparisons and historical CI failure remain preserved.

The evidence supports removing the identified concurrent libtest bookkeeping opportunity. It does **not** establish the historical failure’s cause or guarantee immunity from arbitrary future worker activity.

The complete accepted package may now be integrated into existing PR535. A new exact-head/current-base review and actual required qualification **SUCCESS** remain mandatory before merge; this PASS is not merge authorization.
