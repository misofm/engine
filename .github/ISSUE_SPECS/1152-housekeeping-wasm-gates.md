# Housekeeping: wasm-gates

## Authorized scope and smallest closable slice

Review the complete `tools/wasm-gates` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `tools/wasm-gates` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

Worker A read all four Rust files, manifest and mutation history; root independently read the host runner and G5/G6 owners. Approved exact attempt-1 proposal on 2026-10-01, confined to src/lib.rs, src/main.rs, tests/g5_native_corpus.rs and MUTATIONS.md. Append report mismatch rows into the final owned String with identical literal fields/order/name/width/hex spelling and no new escaping/refusal. Share identical native/Wasm outcome reporting while preserving JSON-first stdout, all mismatch labels/order and minmax/f64/meter diagnostic order; borrow the existing argument path instead of an owned PathBuf. Compare the existing digest arrays directly and compute each idle counterpart name once; append scalar-oracle diagnostic bytes with unchanged lowercase spelling/separators. Add an accurate current-owner preface while preserving historical mutation rows.

All seven G5 and two native G6 functions remain; no new test/corpus, numerical expression, manifest, dependency, runtime config, export, guest shape/admission or digest-word read changes. Root preserves independent native/guest loops and every G6 arm. Focused full debug tests pause for exact checkpoint, then existing release native G5/G6, actual unchanged SIMD guest comparisons on the modified host, strict lint/policies/fmt. Reuse #1148 guest opcode/detector evidence candidly: #1149 changed only prose. Wasmtime is a host qualification tool; no unsupported mobile/Wasm runner matrix or shipping portability claim is requested. Root reviews exact serializer literals and caller ownership; no new observer/harness or timing workload. #1173 remains a source-only future guest-refusal issue and is not implemented here.
