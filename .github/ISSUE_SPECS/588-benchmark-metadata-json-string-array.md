# Share JSON string-array serialization in benchmark metadata

Status: proposed numbered TOOL9 child of audit #349 and lane-B handoff #560, based on delivered main `6984c61aea0a56ea03071a2480c0806aef4b7740` after #585 / PR #586 and successful post-main qualification `34168555795`. This advances an original partial finding, not an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH implements. Astra LOW performs every scope, source and exact-head/current-base verification under the user's latest routing. No audio, browser or artifact qualification is indicated.

## Smallest closable outcome

Move the identical JSON string-array assembly used for `missing_metadata` by the session and conformance benchmarks into `bench_support::json`. Migrate only those two production callers. Preserve exact bytes, input order, duplicate entries, empty strings, Unicode and the existing shared string escaping. Do not sort, normalize or change either benchmark record schema.

This removes one concrete formatting-law duplicate. TOOL9 remains partial afterward; other metadata structures, formatters and record templates require separate review.

## Exact ownership

Allowed implementation paths are:

- `tools/bench-support/src/json.rs`
- `tools/bench/src/session.rs`
- `tools/bench/src/conformance.rs`
- this numbered spec and bounded issue evidence

Exclude every other benchmark consumer, metadata acquisition and environment policy, statistics, timing/workloads/corpora, manifests and lockfiles, policies/workflows, runtime/host/DSP/session-compiler/control code, SDK/browser code, generated artifacts and pins. Active lane-A #587 owns only its documented `host-core` preparation/endpoint/test and bounded evidence paths; the paths do not overlap.

## Objective gates

1. The shared serializer produces exact JSON bytes for an empty array, an empty element, ordered duplicate elements, quotes, backslashes, control characters and Unicode.
2. Session and conformance production record paths both use the shared serializer for `missing_metadata`; their private duplicate array-assembly bodies disappear.
3. Existing session and conformance synthetic record oracles pass unchanged, proving no schema, field-order, sentinel or escaping drift.
4. Focused session/conformance and bench-support tests pass in debug and release; complete affected bench debug and proportional release tests pass. Strict affected Clippy and rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass.
5. The previously established unrelated release allocator disposition remains excluded. No timed benchmark, browser run or artifact output receives credit.

## Stop and split triggers

Stop before changing a schema, metadata acquisition, another consumer, an escaping rule, a manifest/lock, a policy/workflow, a workload/timing path or any runtime/audio/browser/artifact path. Stop rather than introducing a general serialization framework. Preserve the checkpoint and brief an independent successor if an existing record oracle must change.

One Luna HIGH attempt receives one Astra LOW adversarial verdict. The implementer pauses at the first coherent focused-green tranche for root checkpointing. After three failed attempts, preserve evidence and rebrief without weakening gates. Historical #543/#555 → #558/#552 and #542 → #567 delivery order and verdict provenance remain unchanged.

## Preliminary residual audit

Astra LOW reviewed current main `6984c61aea0a56ea03071a2480c0806aef4b7740` and found identical `json_string_array` implementations in `tools/bench/src/session.rs` and `tools/bench/src/conformance.rs`. Their production callers serialize `missing_metadata`, and both implementations already delegate per-element escaping to `bench_support::json::escape`; only array assembly remains duplicated.

TOOL11 requires its separately excluded declarative-rule contract plus fail-closed parser and producer controls. IO5 requires broader host activation, live-state publication, graph rollout or segment contracts. Neither is a smaller next slice. This TOOL9 child is disjoint from #587 and the live open-PR roster. Astra LOW is sufficient for all review. Activation still requires exact local/GitHub numbered identity, a pushed clean brief, current-base and ownership checks, and an Astra LOW scope PASS.

## Numbered current-base scope review

Astra LOW returned **PASS** for exact pushed brief and upstream `282aaa67a7b2086e29b04428a23610f7128336e3`, with current main and merge-base `6984c61aea0a56ea03071a2480c0806aef4b7740`. The clean worktree's sole delta is this spec, diff checks pass, GitHub #588 has exact open number/title/body identity, and base qualification `34168555795` succeeded.

The two private implementations are byte-identical. The three-file ownership and exact-byte plus unchanged-record gates fully bound the extraction. #560 records the activation, #559/#587 and the live PR/worktree roster do not overlap, and historical provenance remains intact. No correction is required; Astra LOW is sufficient. Luna HIGH attempt 1 may begin from this exact brief/base and must pause at the first focused-green tranche. No audio, browser, artifact or timed benchmark work is authorized.

## Attempt 1 implementation checkpoint

Luna HIGH delivered source checkpoint `47a43a24db18c42cfb8637867da263cb6c95a0ba`, changing exactly the three authorized tooling files. `bench_support::json::json_string_array` now owns the array assembly, both session and conformance production callers use it for `missing_metadata`, and both private duplicate bodies are removed. Shared-helper tests cover empty arrays and elements, ordered duplicates, quotes, backslashes, control characters and Unicode. Existing session and conformance record oracles remain unchanged.

Shared-helper debug/release tests pass 3 tests each, session debug/release pass 3 tests each, conformance debug/release pass 4 tests each, and complete bench debug passes 43 tests. Strict bench and bench-support Clippy/rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass. A transient dependency-order-only lockfile change was restored; the final tree contains no excluded path. No timed benchmark, browser or artifact work ran. Astra LOW attempt-1 adversarial source review remains pending.

## Attempt 1 source verdict

Astra LOW returned **PASS** at exact pushed review head and upstream `a737700f23fc3783b8c0902516ac08bd4c041a1e`, source `47a43a24db18c42cfb8637867da263cb6c95a0ba`, and current main/merge-base `6984c61aea0a56ea03071a2480c0806aef4b7740`. The shared helper preserves exact bytes; tests cover empty arrays and elements, order, duplicates, escaping, controls and Unicode. Both production callers use it, both duplicate bodies are gone, and the existing record oracles are unchanged.

Independent full debug bench-support/bench tests pass 43 bench tests, release JSON tests pass 7 tests, release session/conformance tests pass 3 and 4 tests, and strict Clippy/rustdoc, formatting/diff, workspace policy, bench policy and mutation gates pass. The tree is clean and pushed, GitHub identity matches, and only the three authorized files plus this spec changed. There is no #587 overlap or excluded-path drift; the unrelated release allocator disposition remains untouched. No correction or current-main integration is presently needed. Astra LOW is sufficient; final exact-head delivery review remains required before PR authorization.
