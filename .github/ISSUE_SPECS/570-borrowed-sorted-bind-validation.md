# 570: Replace graph bind tree sets with borrowed sorted validation

Current assignment: Luna HIGH attempt 1 after the numbered-base review below. Root owns Git, GitHub, checkpoints, artifact qualification and pinning. Astra LOW performs every new adversarial verification under the user's current routing. No timing or performance-number capture is authorized.

## Problem and smallest closable product

Audit #349 CP4 found six string-keyed `BTreeSet` populations in `PreparedGraphPlan::bind_optional_source_set`. Delivered #495 removed the combined cloned `all_supplied` set by comparing the two existing sets through a borrowed union. Delivered #498 removed two intermediate source-claim vectors and one extra clone population. Current main `c8951bfe23164086ca1ce34b456ab5d600fd4a13` still builds five transient trees in this bind-validation block: supplied node IDs, builtin-bank members, required node IDs, source-claim node IDs, and observer `(node, handle)` pairs. Four clone owned `GraphNodeId` values, whose stable IDs own strings; the fifth clones observer node IDs. The collections exist only long enough to decide validation booleans and are dropped before executor construction.

Replace exactly those five transient bind-validation trees with pre-sized `Vec` collections of borrowed keys. Sort each vector by the existing `Ord` law and deduplicate where the old set did. Detect duplicate supplied bindings and source claims from the pre-deduplication length. Filter required bindings against a sorted unique borrowed builtin-member vector using binary search. Compare the sorted unique supplied/source union with the sorted unique required vector through a small allocation-free two-way merge over borrowed values; do not construct a combined collection and do not reintroduce #495's removed allocation. Validate observer uniqueness by sorting borrowed `(node, handle)` pairs and checking adjacent equality. Put the borrowed collections in one lexical block that returns only the existing booleans so every borrow ends before the plan, bindings, or source set can move into a failure or executor.

This is the remaining original CP4 bind-validation collection outcome. Acceptance means no `BTreeSet` construction and no `GraphNodeId::clone` in `bind_optional_source_set`, at most one pre-sized contiguous allocation for each nonempty validation family, and no combined coverage allocation. This source mechanism replaces per-entry tree-node allocation and owned string-bearing key clones; it is not a measured speedup, allocation-free binding, persistent resource reduction, or render-path optimization. The retained executor `source_inputs` ownership, structural-layout sets, bank-attachment sets, public types, and every `BTreeSet` outside this one validation block are outside this issue.

## Frozen behavior and paths

Allowed production and test source is `crates/graph/src/lib.rs` only. This numbered spec and durable evidence are allowed. Do not edit `runtime.rs`, `program.rs`, graph compiler, source, hosts, Cargo manifests, policies, allocators, or public APIs. A private comparison helper may live in `lib.rs` only when it operates on sorted borrowed slices and allocates nothing. Do not sort or mutate caller-owned bindings, plan requirements, source claims, bank members, or observer vectors.

Preserve the old set semantics for arbitrary public-plan input, including unsorted and duplicate `required_bindings` and builtin members before later structural validation. Preserve `GraphNodeId`'s full value ordering; pointer identity is never semantic. Preserve source `is_valid()` evaluation and `driver.claim_count()` exactly once in the same relative validation phase. Preserve the borrowed-union result for overlap, missing, extra, duplicate, and empty populations. Source overlap remains independently invalid even when the union equals required.

Keep rejection precedence and returned ownership unchanged: with a source set, invalid source claims, source overlap, or coverage mismatch yields `source.graph.binding_mismatch`; otherwise an invalid or duplicate observer yields `graph.plan.observer`; otherwise envelope mismatch yields `graph.plan.envelope_mismatch`; remaining coverage or duplicate binding failure yields `graph.plan.binding`; structural layout remains later. Rejected processors, observers, source driver and boxed plan must remain recoverable and reusable. Successful binding must retain processor/observer ordering and source claim-to-buffer ordering because only borrowed validation copies are sorted.

## Finite proof and gates

Retain and run `tests::binding_coverage_preserves_validation_and_ownership`, `tests::binding_rejects_duplicates_and_returns_all_ownership`, `tests::structural_layout_rejection_is_shared_after_binding_validation_for_source_families`, `tests::id_ordered_bank_plan_rejects_transactionally_and_returned_ownership_is_reusable`, and `tests::level_major_w4_builtin_bank_is_analytic_for_three_blocks`. Add one compact inline test only if current coverage does not distinguish sorted/deduplicated borrowed behavior for unsorted requirements, duplicate requirements/bank exclusions, or duplicate observer pairs across plan-owned and caller-owned lists. Assert exact codes and successful repair using returned owners. Reuse existing fixture types; do not add an allocator harness, benchmark framework, broad mutation matrix, or production instrumentation.

The source-mechanism review must inspect the exact production diff and prove: all five bind-local `BTreeSet` populations are absent; validation keys are borrowed; each vector is capacity-sized from its source upper bound; sorted uniqueness matches old set semantics; the coverage merge consumes equal keys once from both inputs, rejects missing/extra tails, and allocates nothing; the observer pair comparison uses value ordering; and no borrowed value survives the validation block.

Run the focused tests with exact nonzero test counts in debug and release, then `cargo test --locked -p graph --lib` and release equivalent, `cargo clippy --locked -p graph --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, `git diff --check`, and `bash scripts/check-graph-policy.sh`. Record each command, cwd, exact source, stdout/stderr and numeric status separately in durable evidence. Existing analytic graph/source behavior is proportional proof; no timing invocation is allowed.

After Astra LOW source PASS, integrate current main once if needed and run the ordinary AudioWorklet builder against the frozen source. Preserve any real expected/observed mismatch. Do not change a pin or generated consumer until an Astra LOW review confirms that the only discrepancy is artifact identity and explicitly authorizes the bounded qualification. Root then owns the verified rebuild, static/object/ABI checks, resource equivalence with negative controls, hermetic tests, pinned SDK install, current Chromium/Firefox/WebKit qualification with self-test mutations, generated matrix verification, exact artifact hashing, and the final pin checkpoint. Numeric resource or PCM changes require a separate concrete ruling; no automatic expectation update is allowed.

One coherent Luna HIGH implementation pass and one Astra LOW adversarial verdict form attempt 1. A finite FAIL may authorize Luna attempt 2 and then attempt 3, each with a fresh Astra LOW verdict; after three failures stop and rescope. Exact-head Astra LOW PR review, live base/head assertions, required `qualification` CI success, merge, post-main qualification, GitHub evidence/closure verification, tracker synchronization, and clean worktree removal are mandatory. Delivering this issue closes CP4 only; it does not advance CP1/2/3 or any original open finding.

## Numbered base and coordination

GitHub #570 and this title match the local stateless spec. Branch `codex/cp4-borrowed-sorted-bind` and isolated worktree `/home/bl/misofm/engine-cp4-borrowed-sorted-bind` start from clean delivered main `c8951bfe23164086ca1ce34b456ab5d600fd4a13`. The issue-boundary inventory found no local numbered spec lacking a GitHub issue; historical GitHub issues without local specs predate or sit outside this child creation and were not relabeled. Open draft PR #465 also touches `crates/graph/src/lib.rs` on a non-main stacked base; this issue does not integrate or rewrite it. Lane B claims only the current-main bind-validation block for #570 and has notified #559 before implementation. The paused lane-A limiter worktree and historical failed hex worktree remain untouched.

Root reviewed current main, delivered #495/#498 specs and evidence, the exact remaining bind code, current open PR file overlap, and the #559 coordination boundary. The scope above is the smallest complete CP4 outcome and is approved for Luna HIGH attempt 1 after this spec/body checkpoint is upstream. Astra LOW is the sole new verifier.

## Luna HIGH attempt 1 and Astra LOW source decision

Luna HIGH replaced the five transient bind-validation trees with five capacity-sized vectors of
borrowed keys. The vectors use the existing value `Ord`, sort and deduplicate without changing
caller-owned order, compare supplied/source coverage through an allocation-free two-cursor union,
check overlap through a second two-cursor walk, and validate observer pairs by adjacent value
equality. The lexical block returns only booleans, so no reference survives into a returned failure
or executor. The final production checkpoint is `f347ec06b58b004749633ba602b599f4011aa7c1`;
later source changes add test assertions only.

Root caught and corrected one duplicate-count expression before the first source commit. The first
retained qualification then passed five focused tests in both profiles, 60/60 graph tests in both
profiles, and strict Clippy, but stopped on one rustfmt line join. That authentic failure is
preserved under `artifacts/issue570-attempt1/`. The formatting-only checkpoint and green bounded
gates are under `artifacts/issue570-attempt2/`. Root and Astra LOW then found that existing tests did
not distinguish duplicate public requirements or value-equal observer pairs across ownership
lists. The one scoped fixture was added, passed 61/61 in both profiles with all source gates, and
was finally extended to repair and rebind the returned observer owners. Exact-head repair evidence
is under `artifacts/issue570-observer-repair/`. The malformed first Clippy wrapper, over-escaped
first census command, and over-strict ad hoc diff assertion remain candid non-credit records.

Astra LOW gave source PASS at clean pushed `e99818a289019839cd34e9d1c2b2664286f8c4f6`,
including the final test/source checkpoint `6913563c661d9faaffd6c9b25d57fe04a8d37179`.
The bind block has zero `BTreeSet` and `.clone()` tokens, five pre-sized vector families, preserved
set/error/ownership semantics, and the required discriminating repair fixture. This is a
source-mechanism allocation reduction only; no timing, speedup, allocation-free binding, persistent
memory, or render optimization claim is made.

## Qualified artifact and pre-PR delivery decision

The single Astra-authorized frozen-source probe observed Wasm SHA-256
`0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`; its immutable record is
`artifacts/issue570-artifact-probe/`. Astra LOW verified the actual identity-only mismatch and
authorized a detached exact-source qualification with a provisional pin. The ordinary builder
produced the exact six-file artifact set. Static/object/ABI, unchanged browser resources with 26
red mutations, SDK generation/deletion/type/headless/package, qualification install, session
identity, and generated-matrix gates passed.

The first all-browser invocation correctly failed before browser launch because checked
`results.json` still carried the prior artifact identity; hermetic testing was skipped. Root
preserved that failure at `6ba0c6ca` before Astra LOW authorized detached-only `results.json` and
generated matrix lineage overlays. Exactly one corrected run then passed Chromium
`151.0.7922.34`, Firefox `153.0`, and WebKit `26.5` with self-test mutations, followed by the green
hermetic AudioWorklet gate and its negative controls. Numeric resource and PCM expectations did not
change.

Astra LOW authorized exactly three qualified repository updates: the artifact pin, the
`results.json` candidate/hash fields, and the generated matrix lineage line. Root checkpointed
those bytes at `bdb9586d32acc8b4e11c0846470ea4089267e58b`. A fresh ordinary post-pin build
reproduced all six retained files byte-for-byte; static/ABI, frozen resources, matrix, pinned SDK
install, and package gates passed, with manifests and lockfiles unchanged. Complete preflight,
qualified, and post-pin records are under `artifacts/issue570-artifact-qualification/`; all 87
manifest entries verify.

Astra LOW gave pre-PR delivery PASS at clean pushed `93c34cad1c8ec673882393f2eba68245e95ea13a`.
Root may record this decision and open the PR. Exact-head Astra LOW review, live main/base/head
assertions, required qualification success, merge, post-main qualification, remote issue closure,
#560/#559 synchronization, and clean worktree removal remain mandatory.
