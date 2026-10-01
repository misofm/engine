# Housekeeping: parameter-metadata

## Authorized scope and smallest closable slice

Review the complete `tools/parameter-metadata` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `tools/parameter-metadata` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 checkpoint and approved ownership completion

Worker B read all six Rust files (4,419 lines), manifest and consumer/issue boundaries. The bounded three-path product checkpoint e41c327f is pushed through 1a9479ef: remove temporary descriptor/probe containers, borrow test parser names and delete two repeated source-ring tail assertions. All ten native tests, formatting and diff checks pass; schemas, probes and test functions are unchanged. Root inspected the complete diff, actual ordered registry iterator and independent ring matrix. No final adversarial verdict is claimed yet.

Root identified remaining unnecessary nested output copies during review. Approved same-attempt completion on2026-10-01, confined to src/lib.rs and src/abi_layout.rs: make the seven private immediate-copy fragment emitters append to the final render-owned String; remove their local builders/returned fragments, and append the four maximumByRate pairs in their original order. Replace format-then-push ABI rows with direct fmt writes using the same literal templates. Keep number/optional_number and escaping authority, all tables/offsets/scenarios/field and whitespace order, output transaction/ownership, exported render API, manifests and dependencies fixed. This changes buffer ownership only, with no new serializer model, algorithm, schema, helper framework or test.

The saved67,855-byte metadata and34,247-byte ABI documents plus69 oracle rows are the one-time base/head evidence; they are not new permanent pins. Existing ten independent parameter/ABI/schema/consumer owners remain. At the first focused-green completion pause for exact root checkpoint before final evidence. Recheck only boundaries actually affected by this new writer ownership; reuse unchanged evidence candidly. No timing or broad matrix. One final root adversarial verdict covers the complete attempt within the original maximum of five.
