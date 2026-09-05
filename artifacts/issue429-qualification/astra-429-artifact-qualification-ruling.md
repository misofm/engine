# #429 conditional artifact qualification ruling

**Approved conditional reuse of prior browser/static/resource/hermetic evidence, ONLY after complete byte and applicability proof. No unchanged-artifact result is presumed.** Frozen source candidate: `e4bcaa2feae13c9f016bb7b2e1eaff8bd7314547`; accepted semantic source: `74128241f91b396cdf798e287c451f9a0505cc14`.

The frozen #429 scope says supported target/artifact/static/browser requirements “remain applicable if current Rust reaches the artifact.” It requires mandatory correctness and artifact identity, not a gratuitous new browser run for identical delivered bytes and unchanged consumers. This ruling defines sufficient applicability evidence; it does not waive a gate on changed product bytes.

## Source classification

Current builtin compiler production uses BuiltinChain construction to validate parameters/compute tail and then splits it via into_sections, with an independently prepared bank input. The public process_dual_mono call in builtin compiler is in its test region. Other located calls are builtin tests and audit/bench tools; no current host production call was found in the inspected source. Thus the new public full-chain seam is an independently useful Rust capability, while its reachability in the shipped worklet is not established by the crate dependency alone. This source inventory supports checking byte identity; it is not itself a linker/whole-program proof. Independent actual builder output remains decisive.

The integration delta after source acceptance consists of #411/#438 gate/evidence and issue records, not another runtime implementation. New shell-gate behavior must retain its own accepted focused/CI proof and cannot borrow old results merely because the worklet is identical.

## If all six actual outputs and their consumers are unchanged

Root may reuse the prior executed browser/static/resource/hermetic results when ALL of the following hold:

1. The existing normal builder completes successfully from the frozen source with the normal supported production profile/toolchain/configuration; retain its full command/status, output manifest and source provenance. This is the new actual artifact identity evidence, not reuse of the old binary without a build.
2. Enumerate the full expected six-file builder output with checked commands and compare the actual relative-file population, byte lengths and hashes/bytes against the previously accepted delivered bundle. Every file, including Wasm, JavaScript wrappers and metadata, must match; not just the Wasm digest. Expected old Wasm identity is `24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe` only if that matches the actual accepted manifest—use the authoritative existing manifest rather than copying this prose.
3. Independently reconcile the actual accepted pin, publisher expectation, current browser matrix/results, ABI/deployment documentation and the prior executed browser artifact identity. Check the relevant runtime wrappers, configuration, validators/harness and resource/static/hermetic inputs have not changed since their retained results, or separately qualify any changed tool. Same binary alone does not prove applicability of a changed host adapter/harness or resource policy.
4. Preserve the old generated browser records and their OLD executed source-candidate/revision/browser versions. Add a separate #429 equivalence record: newly built candidate reproduces that previously qualified bundle; browser runs were NOT repeated. Do not relabel/rewrite old browser source lineage, manufacture a current-candidate browser record or repin unchanged bytes.
5. Retain current #429 mandatory native/full-workspace correctness, realtime/audit, supported scalar/SIMD builds and supported Wasm inspection evidence. Public Rust capability changed even if not linked into the shipped worklet; unchanged worklet does not substitute for its direct tests/targets. Actual-head PR review and required CI remain mandatory.

With those proofs, no redundant second normal artifact rebuild, new three-browser execution, unchanged static/resource mutation suite or unchanged hermetic mutation sweep is required solely to restate the identical delivered product. Use existing normal verification behavior for pin/output consistency; if its command performs a needed existing check, record it accurately.

## If any byte, output population or relevant consumer differs

No reuse disposition on the basis of “probably dead code.” Complete the normal independent verify and current pin/publication consistency work, static/callgraph/resource/hermetic checks, browser record and required per-browser checks against the immutable current source/output. Derive changed generated identities honestly and preserve historical artifacts. If the change is only source-dependent metadata, it still fails the exact-six-file condition proposed here: classify explicitly and use the normal changed-output path rather than silently dropping that file from the comparison.

A failed build/comparison, missing prior bundle/provenance, unverified population, changed resource consumer, or unexplained digest mismatch is not unchanged-output evidence. Preserve it and resolve the specific gap before claiming reuse.

Known accepted Wasm digest: `24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe`. Prefer machine-read actual prior pin/manifest for comparison. No builder result or browser execution was observed or newly performed by this review; the decision remains conditional until root supplies the complete byte proof.
