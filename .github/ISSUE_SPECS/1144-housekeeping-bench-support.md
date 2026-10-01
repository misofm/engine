# Housekeeping: bench-support

## Authorized scope and smallest closable slice

Review the complete `tools/bench-support` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `tools/bench-support` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 checkpoint and root adversarial verdict: REQUEST

Worker A read all nine package files and the related #349/#559/#560/#1039 ownership records. Baseline debug/release: 44 active tests passed. Product checkpoint `74e203e1`, integrated/pushed as `a61f34d3`, passed 41 active focused tests, formatting and diff checks. Four Rust paths add 51/remove 49 lines. The JSON array appends into its result through the unchanged escape body; CPU core keys borrow the input; identical counter subtraction is shared without changing loads, updates or failure semantics. Three standalone core-count functions consolidate into the same four exact inputs. No performance timing or new harness was run.

Root inspected all four diffs plus the JSON cases, allocator update-before-System path, topology parser, metadata and timer owners. The core-count matrix catches duplicate (core,socket) identity and empty/header-only fallback; all four original inputs remain. Those edits are sound, but the full crate cleanup still leaves avoidable private metadata copies: `missing` clones values merely to test Unicode presence, and `record_fields` clones values before escaping and builds an extra vector/join for missing names. The existing hashing-outside-timer test only hashes afterwards and checks digest length; pure-timer and digest owners already cover those observations, while preexisting-update exclusion is not exercised. Attempt 1 receives one REQUEST verdict for these bounded copy/test-value gaps; the useful compiling checkpoint is preserved.

### Attempt 2 Sol brief approval

Approved by root Sol, 2026-10-01. Finish this same package with a private borrowed snapshot projection for metadata presence and JSON fields, preserving public `var` ownership and exact missing/non-Unicode/empty handling and every output byte/field/order. Reuse `json_string_array` for the missing-name array; do not change the public format or metadata source/snapshot semantics. Rewrite the existing outside-timer test to update the sink both before and after the timed region, with `untimed` carrying the outside operations. Its purpose is to turn red if timing wrongly includes preexisting digest updates or refuses operations outside the interval; no existing pure-body or inside-timer negative test covers that boundary.

No counter, digest algorithm, percentile, public API, manifest, script or timer implementation changes. No new test or benchmark. First focused-green tranche pauses for root exact-path checkpoint. Then run focused package debug/release, strict lint/fmt/policies, supported compile checks and an existing downstream metadata/allocator owner proportional to these changes; reuse unchanged prior evidence candidly. Retain complete nine-file/five-axis/family evidence. Attempt 2 gets one root adversarial verdict within the maximum of five.
