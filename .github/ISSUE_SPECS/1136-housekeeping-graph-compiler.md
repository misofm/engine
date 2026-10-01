# Housekeeping: graph-compiler

## Authorized scope and smallest closable slice

Review the complete `crates/graph-compiler` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

## Frozen boundaries

Existing public APIs, feature behavior, canonical/wire identities, arithmetic order and per-lane rendered bits, latency/tail, link modes, smoothing and NaN/denormal rules remain contractual. Zero render allocations/frees, locks, I/O or syscalls; no runtime native ISA dispatch. Retain scalar tails, 4-lane Wasm/NEON and x86 AVX2/FMA. Preserve all feature and target configurations. Do not expand another open issue or change DSP algorithms. Read current related issue bodies before touching their subject; discoveries that require architecture or product decisions become bounded follow-ups and final owner questions.

## Implementation and test-value decisions

Prefer deleting or consolidating repetition to adding abstractions that increase total complexity. Assess every existing test or a clearly named homogeneous family by its plausible unique defect; remove trivial, redundant or obsolete cases only after identifying the surviving behavioral gate. Keep independent numeric/oracle, fault, allocation, queue, boundary and target tests. Add/rewrite tests only for a concrete uncovered defect, and state which plausible defect no existing test catches. No prose/source-grep tests, new bit-digest pins or exact resource-byte pins. Copies needed for ownership, snapshots or atomic admission stay unless the same semantics are proved with less work. Data structure changes must preserve deterministic order and bounded realtime work. Inspect applicable hot loops and generated code before claiming additional SIMD; recursive/stateful dependencies alone do not justify changing arithmetic.

## Objective gates and evidence

- Read all production and test files in this package; record concise findings for each of the five requests, concrete changed/deferred locations, and load-bearing test families with retained coverage for deletions.
- Run focused locked package tests and affected feature configurations; use existing downstream/RT/differential gates proportional to the changed contract. Check formatting and package clippy with warnings denied. Relevant Wasm and AArch64 compile checks are required for changed product code; record limitations candidly.
- Changes to DSP arithmetic or hot state need existing independent numeric and scalar/SIMD gates plus one-time base/head evidence when needed; no permanent comparison against the old implementation. Existing research remains the algorithm authority; no new algorithm or listening claim is authorized.
- Benchmarks are optional and descriptive. Any timed measurement freezes its workload/validator, passes zero-workload preflight, and runs exactly one invocation with one warmup/two measured rounds. No timing optimization loop, performance percentage or unsupported sound-quality claim.
- Root conducts one adversarial verdict per coherent attempt, at most five total attempts. Every new/rewritten test gets its unique-defect sentence in that verdict. No-change audits require the same five-axis review, not manufactured edits.
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/graph-compiler` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

Root test-value decision after the complete read: retain current `MISO-GRAPH-V1` canonical-format identities in the reverse-submix and track-delay gates. Retire `the_merged_span_hold_costs_the_input_slots` and its measurement-only commentary: its only assertions pin the current 256/129 arena inefficiency, with no distinct dataflow or admission behavior. Independent lowering/dataflow and transactional resource-cap gates remain. Root reconciles the historical measurement/test reference in #931's spec and GitHub body; #931's colouring implementation and product objectives remain outside this issue.

## Attempt evidence

### Attempt 1 — focused checkpoint

Worker B read every production/test file, the fixture data and mutation notes before editing; related #888/#889/#931/#932/#967/#969 scopes stay separate. Temporary PDC, reduction, effect-level and bank-membership indexes now borrow IDs/strings; returned records, prepared owners and diagnostics retain their ownership and order. Three causal-effect/limiter PDC tests share one body with all compressor, multiband and gate cases and their bypass/486-sample assertions retained.

Removed the same-visitor random identity-length comparison (independent literal/UTF-8/nested-identity gate survives), historical executor-size comparison (compiled lowering/order/elision/protected-bank-storage assertions survive), seeded PCM transcript and optional long-run PCM hash (all generators, independent ordered scalar PCM, membership/callback/RT/pointer assertions survive), old-platform gain witness (current canonical compiler coefficient and independent gain-range oracle survive), and the ruled merged-span 256/129 measurement test (current lowering/window, scalar/vector dataflow and resource-cap owners survive; #931 retains the finding and optimization scope). Reverse-submix and track-delay `MISO-GRAPH-V1` format identities remain.

Focused locked library filters `identity_tokens compiled_plans_lower seeded_builtin mixed_causal effect_control_resource issue122 timing_`: **8 passed**, including every consolidated PDC case. Production and test code compile; package formatting applied and `git diff --check` passed. First local invocation exposed the now-unused `stages` import, removed before the clean rerun using `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b`. No timing claim or optional long sweep. Product edits paused for root's exact-path checkpoint; remaining package/feature/lint/target evidence follows afterward.
