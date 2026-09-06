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
