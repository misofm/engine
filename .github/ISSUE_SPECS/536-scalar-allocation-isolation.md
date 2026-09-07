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
