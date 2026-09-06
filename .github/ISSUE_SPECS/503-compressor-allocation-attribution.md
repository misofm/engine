# 503: Attribute compressor render allocation proof to the render thread

Post-main qualification 34014528298 failed with actual exit101. /tmp/502-postmain-dsp-failure.log lines412–415 show conformance.rs122, `render allocated`, left4/right0; the child reports one failing test and the parent correctly propagates failure at line31. This is a new discovered qualification defect after delivered #475/#499, not permission for a fourth #475 attempt. Preserve the raw failed job and earlier successful evidence; do not rerun simply to obtain green.

## Diagnosis and bounded decision

Current conformance.rs35–124 uses bench_support::alloc process-global atomics. The --exact child removes the other conformance test, but libtest still has its coordinating thread alongside the test thread. Any allocator call on that thread enters the same delta. The four calls are therefore not attributable to compressor render from this log alone; thread contamination is a credible source-supported explanation, not a proven allocation stack trace. Do not assert production is exonerated solely by this diagnosis.

Reuse the existing allocator and engine::realtime::audit thread-local scope/counters, already wired by bench-support's allocator: alloc/alloc_zeroed/realloc record Allocation and dealloc records Deallocation only when the current thread is armed. This directly tests the real render thread without accepting unrelated harness traffic. No new allocator, global counter behavior change, thread-ID lookup, warmup-discard loophole, allowed allocation threshold or production correction is authorized.

## Exact scope and implementation route

Only crates/compressor/tests/conformance.rs, crates/compressor/Cargo.toml (one dev-dependency `engine = { workspace = true, features = ["realtime-audit"] }` to use the already-existing API explicitly), and the mechanically required Cargo.lock compressor dependency-list addition of the existing engine edge only, plus numbered spec/evidence. The engine dependency/feature already exists transitively through bench-support/conformance; no new package/version or normal production dependency. No shared allocator/engine/conformance framework edit, production compressor change, Cargo profile, workflow, environment marker or corpus change. Preserve the existing exact child and count-mode isolation.

Warm up/reset the existing audit outside observation. Replace process-global render-delta acceptance with thread-local snapshot of an armed interval containing the same 32 actual staged-uniform, D=0-uniform and native ragged renders, with all preparation/buffers/results/assertion formatting outside. Keep first render inside the interval; no discarded production render. Assert zero Allocation and Deallocation (and no other audit violations). Realloc remains forbidden because the existing realloc hook records Allocation; state this explicitly rather than claiming a new separate thread-local realloc field.

Prove the exact same scoped counter path is live before resetting: actual opaque heap allocation, forced capacity growth/reallocation, and drop under a deliberate counted probe must record allocations and deallocation; capacity growth must actually occur. Existing installed-allocator liveness remains useful but process-wide liveness alone is insufficient.

One compact deterministic attribution control: pre-create a worker and coordination objects outside the observed probe; make that worker actually allocate/grow/free while the test thread's audit scope is armed, then verify the test-thread snapshot stays zero. Synchronization is confined to this deliberate test-control probe, never the production render interval; join outside it. Separately demonstrate the same heap operations on the armed test thread produce positive counters. Do not rely on sleeps, probabilistic concurrent test scheduling, process-global subtraction, or disabling the allocator. No production mutation campaign is needed: positive same-thread and negative other-thread controls discriminate the changed attribution.

## Finite gates and delivery

Freeze one named local attribution-control test plus the existing exact render test. Run both exact debug/release with nonempty results, then the complete compressor conformance target debug/release with both original tests retained. Run the original failed DSP command once as integration qualification after focused acceptance, using the exact command in the retained CI log (including lane feature unification), with independent command/status records. Strict affected all-targets/all-features Clippy, fmt/diff, realtime policy and existing policy mutation suite remain required. Check production/Cargo normal-feature and shipped-artifact inputs remain unchanged; do not regenerate an artifact absent a real relevant change/mismatch ruling. Required CI and actual exact-head Astra PR review govern delivery.

If thread-local render counters actually report a production allocation, stop and report the concrete path; do not widen this proof successor into DSP repair or weaken zero. Root must number/synchronize this stateless scope before fresh Luna1, then Sol2/3 if needed. #496 remains held until this failed qualification is addressed per root's sequencing. No builds/tests/timing or source/Git/GitHub mutations performed for this diagnosis.

## Numbered scope checkpoint

GitHub #503 is the matching successor. Parent #475 and child #499 remain delivered; this issue owns the newly observed post-main qualification defect. Astra supplied this brief; root owns Git/GitHub and checkpoints, fresh Luna implements attempt 1, Astra adversarially reviews, and Sol receives attempts 2/3 only after consolidated failure. #496 remains queued.

## Astra numbered review and activation

# Astra #503 numbered scope/base review — one finite amendment required

Reviewed clean exact head 523b4ada938b8b1f4c3d58179356f4d5903faf0c in engine-compressor-allocation-proof, based on delivered ad00d16b. Only numbered #503 and #475/#499 closure documents differ; production, tests and dependency inputs are unchanged. The entire supplied successor body is preserved verbatim under the matching numbered title. Root reports matching synchronized remote number/title/body. The attribution design, same-thread positive and deterministic other-thread negative controls, unchanged 32 first-and-repeated real renders, and preserved parent history are sound and ready for fresh Luna1 after the mechanical scope amendment below.

Add Cargo.lock to the exact allowed files, restricted solely to adding the existing `engine` dependency edge in the `compressor` package dependencies list. Current Cargo.lock279–290 contains no direct engine edge for compressor; the explicit dev-dependency required by this very brief therefore needs that lockfile update for the frozen --locked gates. No package/version/checksum, unrelated dependency or normal production dependency change is authorized. This omission originated in my draft's exact-path enumeration, not in the numbered adoption or implementation. Record the narrow correction before assignment; it needs no new architecture, additional test or redesign.

All other scope and gates remain unchanged. Existing engine/bench-support audit code must stay untouched; the negative worker control does not prove render clean by itself, and real armed render violations require stopping for a concrete ruling. No discarded first render, threshold, new allocator/framework, workflow/corpus adjustment or benchmark invocation is allowed. Root retains the one coherent Luna1 verdict and Sol2/3 escalation, actual PR/CI delivery gates, and #496 hold.

Read-only comparison; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root adopts the exact lockfile amendment above and activates fresh Luna attempt 1. No package/version/checksum changes are authorized.

## Attempt 1 source and root verification

Pushed source ce88bf12 implements the existing thread-local audit and discriminating allocation/growth/free/isolation controls. Root directly captured all frozen exact debug/release tests, full conformance debug/release, strict Clippy, fmt/diff, realtime policy/mutations and the original failed DSP command on clean source; all exit 0. The complete conformance target runs three tests plus the child result. Artifact package artifacts/issue503-allocation-attribution retains authentic root records and the original failed CI job. Luna-supplied .log files are candidly classified as reconstructed summaries, not raw evidence, and retained unchanged; root recapture supplies authoritative proof. Production sources and normal dependency declarations are unchanged; only the test dependency and its existing lock edge change. Consolidated Astra review pending.

## Astra consolidated attempt 1 PASS

# Astra #503 Luna attempt1 consolidated review — PASS

Reviewed clean exact head 030f65db5df06cf919502806b7aa5814853fde01 in engine-compressor-allocation-proof, source ce88bf12e0a0c0473aa743115c80260327180d9d. No source delta follows that checkpoint. This accepts the bounded allocation-attribution correction; actual-PR review and required CI remain pending.

The change stays within amended scope: compressor conformance test, explicit test-only engine/realtime-audit dependency, and the single existing engine edge in Cargo.lock's compressor list. No production compressor, shared allocator/audit implementation, normal dependency, package/version/checksum, profile, workflow, environment marker or numerical change. Production artifacts are not regenerated or claimed here.

The same first and subsequent 32 staged-uniform, D=0-uniform and native ragged renders now run inside the existing thread-local armed scope. Preparation, buffers and output inspection remain outside; no initial render is discarded. Snapshot assertions require zero allocations, deallocations and total violations. The existing allocator's realloc hook records Allocation, so zero allocations also rejects realloc; no fictitious separate scoped realloc counter is claimed. The exact child remains, preserving count-mode isolation and parent failure propagation.

The positive control observes initial allocation, then a strictly larger allocation counter after actual Vec capacity growth, then a strictly larger deallocation counter after drop. The vector has a concrete nonzero u8 element and forced reserve, with opaque use before drop. This proves the armed hook path, not just process-global installation. The negative control creates worker/barriers outside observation, releases the worker only after the test thread is armed, and waits until the worker's actual allocation/growth/drop has completed before leaving that scope. Worker join and assertions are outside. Its synchronization belongs only to the deliberate attribution control, never to production render. Zero thread-local snapshot under that coordinated activity discriminates unrelated-thread contamination; the paired positive control prevents an inactive audit from passing both.

Authoritative evidence is the root's direct subprocess capture, not Luna's reconstructed .log summaries. Those summaries are explicitly disqualified as raw evidence and preserved unchanged with the attribution note. Root command/source/environment JSON and independent numeric statuses cover exact control and render debug/release, full conformance three tests each (plus the exact child result), affected strict all-targets/all-features Clippy, fmt/diff and realtime policy/mutations, all exit0. The independently retained DSP integration argv matches the originally failing package roster, --locked --all-targets and --features math/lane; it exits0 on the final source. No fresh execution was performed for this review.

Independently verified all 52 payload sizes/SHA256 values and exact tracked coverage of 53 files including the manifest. Original post-main exit101/four allocations and all report-attribution limitations remain intact. The new evidence proves zero scoped violations for these real renders and fixes their attribution; it does not retrospectively identify the allocation stacks responsible for the original four process-wide calls.

Root may package/open the actual PR for exact-head review. Required CI must succeed before delivery; neither the earlier #475 premerge success nor this source PASS substitutes for that. The successor remains distinct from a fourth #475 attempt. Subsequent #500 integration must preserve source identity and explicitly retain this dependency's delivery outcome.
