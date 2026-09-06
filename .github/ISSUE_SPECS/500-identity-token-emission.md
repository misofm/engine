# 500: Share graph node and edge identity token emission

Active fresh Luna attempt1 under the actual-base approval below.

Queued maintenance after #498, not an implementation assignment. Reviewed delivered396a97119583704888d7b2a20830c5720e16b189, graph-compiler/src/canonical.rs blobfe196f84338c0d209bd401e5a73bd8a9ede4d52a. Read-only open-title/local scope checks found no dedicated node/edge token-emission successor. #99 established allocation-free length calculation and on-demand canonical output; neither it nor #338 canonical Session JSON authorizes changing those contracts. This closes this precise CP20 duplication, not all encoders in349.

## Smallest product and frozen implementation choice

node_text versus node_text_len independently match six node variants; edge_text versus edge_text_len independently match four edge variants with identical prefixes/separators/field order. CompensationDelay recursively emits an edge; TrackMain recursively emits a node. Introduce two private mutually recursive borrowed-piece visitors, one per identity type, taking a mutable generic FnMut(&str) sink by reference. Each variant/token ordering appears once. Recursive calls pass the SAME sink type/reference; do not recursively wrap sinks into new closure types. Literal pieces and existing borrowed ID/port slices require no piece Vec, boxed iterator, trait-object allocation or intermediate String.

Keep current pub(crate) node_text/edge_text signatures; each collects pieces with String::push_str. Keep current node_text_len/edge_text_len signatures; each sums str::len into usize through the same visitor without constructing String or invoking format!/fmt::Write. Do not route length through text().len(), add checked/saturating resource arithmetic, pre-count and allocate twice, or introduce a general formatting framework. Existing stage_token/rack_token remain authoritative shared tokens. Preserve recursive output order and all delimiters exactly. Length is UTF8 byte count, not character count.

Only crates/graph-compiler/src/canonical.rs production code, and existing inline tests in crates/graph-compiler/src/lib.rs (or a compact cfg(test) module in canonical.rs), plus numbered spec/evidence. Do not change port_text, write_canonical, Sha256Writer, Graphviz writer, resource-estimate consumers, IDs/schema, compiler validation/order, graph runtime, Cargo or public APIs. String allocation strategy may naturally differ; no measured speedup or total canonical allocation-elimination claim follows. Allocation-free length computation is mandatory; source inspection must show only borrowed callbacks/stack integer accumulation through every recursive path.

## Independent finite proof

The existing node_text_len_matches_node_text_for_every_variant test remains but becomes correlated once text and length share a visitor. Add ONE compact literal identity fixture with fixed expected strings independent of every formatter/token/length helper: all seven TrackStage spellings; Effect in all three RackId values; Route, Submix and Output; each of the four edge variants; and a recursive CompensationDelay→TrackMain→CompensationDelay→RouteDestination chain with its exact literal prefix/separator order. Use distinct short stable IDs so field swaps are visible. Include one EffectSidechain with an actual non-ASCII port String (e.g. `é/鼓`, permitted by the identity type) and assert UTF8 byte length against the literal's bytes; do not invent invalid Unicode StableGraphIds. For each table row assert text equals the literal AND length equals that independent literal byte length. This catches a shared prefix/separator/field regression that the parity test alone would miss. No generated snapshots, expected strings assembled with stage_token/rack_token, new corpus or mandatory mutation campaign.

Retain canonical_artifacts_are_complete_and_repeatable_100_times, which exercises current canonical bytes/streaming SHA and deterministic graph outputs; do not claim it is an independent historical golden. Retain current graph-compiler resource/cap tests including live_scalar_owner_bytes_are_published_and_capped_before_binding and post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs. Since their metadata byte inputs depend on these lengths, no changed numerical expectation is accepted. The new literal fixture is the independent identity gate; existing parity/integration tests provide coverage around it.

Run the new exact private literal test debug/release (one nonempty result each), existing length parity and canonical repeatability filters, then graph-compiler library tests debug/release, affected strict all-targets/all-features Clippy, fmt/diff and existing graph policy. Prefer --lib for the release test suite, following the already recorded test-artifact collision ruling; preserve any real inherited failure rather than editing panic/profile/Cargo here. Capture actual argv/cwd/source/log/numeric statuses individually. No scale65k run, new allocation harness, timing, or wider target matrix is needed for source acceptance. Directly inspect both length call paths for absence of heap-producing operations; retained #99 no-string contract is not waived by passing byte tests.

## Delivery boundary

Root numbers/synchronizes and confirms actual current source/ownership before Luna1. One coherent attempt and consolidated Astra verdict, Sol2/3 if needed then hardstop/rescope. This is independent control-plane maintenance, not overlap with #475/#499 or queued runtime work. After source PASS integrate current main once and perform proportional ordinary artifact verification; any actual digest mismatch needs a bounded current-pin/current-consumer qualification ruling with unchanged canonical/PCM/resource expectations. Exact-head PR review and required CI remain merge gates. No implementation, build/test, benchmark or timing is authorized by this draft.

## Numbered queue checkpoint

GitHub500 title/number match this stateless spec. Branch codex/cp20-identity-token-emission begins at delivered396a97119583704888d7b2a20830c5720e16b189. Queued after498 delivery; root must integrate then-current main and obtain Astra actual-base approval before fresh Luna1. No source implementation, artifact build or timing is authorized by this queue checkpoint.

## Current implementation-base readiness

Root integrated delivered main95abdd015e28823905800d051d03837255d91612 after PR501/498 closure and carried the498 closure record. Graph-compiler, Cargo and target config are byte-identical to the originally scoped396a9711. Parent475/child499 are source accepted and in immutable delivery qualification, not overlapping these paths. Await Astra numbered actual-base review before fresh Luna1; no implementation has started.

## Astra actual-base approval and assignment

# Astra #500 numbered actual implementation-base review — PASS

Exact clean headd22431500df514530269c243271716e3031657e9 in engine-cp20-identity-plan, integrated delivered main95abdd015e28823905800d051d03837255d91612. Only500 numbered scope and498 closure documentation differ from main. Graph-compiler/Cargo/config are byte-identical to scoped396a9711. The entire approved CP20 brief body is preserved under the numbered title and queue/current-base records. Root reports matching remote identity/body and498 delivery closure.

Approve fresh Luna1 for the frozen private borrowed node/edge piece-emission slice. Keep current text/length interfaces, one shared token/variant definition, allocation-free length accumulation, original UTF8 bytes and recursive delay/edge ordering. Preserve stage/rack tokens, public canonical/hash/port writers, resource estimates, schemas and validation. No generic formatting framework, heap-producing length route, public API, Cargo or runtime change.

The independent literal fixture remains essential because text-versus-length parity becomes correlated: all staged/rack/identity variants, recursive chain and UTF8 sidechain port must compare against fixed independent literals. Existing parity, canonical/hash repeatability and resource/cap tests plus affected library/Clippy/fmt/policy gates remain the finite acceptance scope. No added scale run, allocator framework, new matrix or timing requirement.

#475/#499 immutable qualification does not share these source paths. Root may assign this independent maintenance pass with separate target directories and recoverable checkpoints. Later source acceptance, current-main integration, proportional ordinary artifact verification and any concrete mismatch ruling, exact-head PR review and required CI remain separate delivery gates.

Read-only source/Git comparison; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root assigns fresh Luna1 within the frozen scope, owns all Git/GitHub checkpoints, and defers artifact qualification until source acceptance. This independent maintenance does not modify the frozen compressor qualification worktree. No timing is authorized.

## Luna attempt1 candidate and evidence

Source4e2fc87d preserves the shared borrowed visitors introduced atfcb8703c, completes the independent literal fixture and updates the length helper documentation. Retained edge_text_len now has a function-local non-test dead-code annotation because production recursion moved to the shared visitor while the scope retains its signature. Graph-compiler library63 debug/release, final literal1 debug/release, existing parity/repeatability, strict workspace Clippy and fmt/diff/graph policy pass. Authentic initial zero-filter and Clippy failures remain in artifacts/issue500-identity-token-emission with final source identities and candid source-attribution notes. No artifact/timing or numerical claim; consolidated Astra review pending.

## Astra Luna attempt1 PASS and current integration

# Astra #500 Luna attempt1 consolidated review — PASS

Reviewed clean exact head 6f421d674846c4745140f79fe495e546a814d2b5 in engine-cp20-identity-plan; implementation source 4e2fc87dcc44243df84618101c10ed06a71cbe45. This is source acceptance, not artifact or PR delivery approval.

The cumulative production change is confined to canonical.rs. Its two private visitors preserve all six node/four edge variants, prefixes, separators, borrowed IDs/ports, stage/rack tokens and recursive delay/TrackMain ordering. Recursive calls pass the same generic sink reference. Both length entrypoints perform only borrowed callbacks and stack usize additions, including recursion: no intermediate String, piece collection, allocation, formatting, checked/saturating policy change or second traversal. Text entrypoints collect directly with push_str. The four required pub(crate) signatures remain. Port/canonical/hash/Graphviz writers, validation, resource consumers, Cargo and graph runtime remain untouched.

Approve the exact function-local #[cfg_attr(not(test), allow(dead_code))] on edge_text_len. Its former production recursive caller now correctly calls the shared visitor; the frozen scope explicitly retains the entrypoint, exercised by tests. This annotation is limited to the genuinely unused non-test function, with no global lint suppression or hidden behavior change. The final followup otherwise updates stale documentation and replaces literal.as_bytes().len() with the equivalent UTF-8 byte count literal.len().

The independent literal fixture covers all seven stages, all three effect racks, Route/Submix/Output, four edge variants, the exact nested delay→TrackMain→delay→RouteDestination sequence, distinct field identities, and the non-ASCII sidechain port é/鼓. Every row compares both text and length directly with fixed independent literals; shared formatter parity alone is not being credited. Existing 10,000-case parity, canonical repeatability and both named resource/cap tests remain unchanged and execute successfully.

Retained execution records supply library debug/release 63 passes each, final exact literal debug/release one pass each, parity/repeatability one each, strict workspace all-targets/all-features Clippy, fmt/diff and graph policy/mutations exit 0. Broader Clippy execution does not create a new recurring scope requirement. Earlier zero-filter/format/Clippy failures remain candidly retained; final focused logs identify the actual dirty source blob/SHA rather than misrepresenting the base commit as final execution. The reviewed final followup has no visitor semantics delta, so prior full-library evidence remains applicable. No fresh tests/builds were run during this review.

Independently verified all 24 manifest payload sizes/SHA256 values and exact tracked coverage of 25 package files, including the manifest. No missing or extra manifested payload; final source identities match the reviewed change. No numerical expectation, measured performance, new allocation framework or extended matrix claim is accepted or needed.

Root reports PR502 has now merged as ad00d16b. Integrate that delivered main once, preserve accepted source identity, and perform proportional ordinary artifact verification. Any actual digest mismatch requires its own bounded pin/current-consumer ruling with unchanged numerical expectations. Exact-head actual PR review and required CI success remain mandatory before delivery/closure.

Root is integrating delivered main ad00d16b8ef8e3aa5ba4c406d00db4c62ff311b5; the 475/499 closure record follows in a separate checkpoint. The entire graph-compiler crate remains byte-identical to accepted6f421d67. The next pushed checkpoint freezes current combined source for ordinary artifact verification.

Recovery record: the first integration command stopped at a prose-only conflict in the #498 post-main closure note. A subsequent orchestration step incorrectly continued and ran the ordinary builder before the merge commit. Its raw record labels HEAD 6f421d67, but the worktree included delivered compressor source from main; this is not clean-head qualification evidence. The compiler succeeded and the digest comparison failed (expected fa78dc8d3f0d391b94419f5252504eee2aafbbf13853884157e24935ed092cc3, observed bed7d77cbd55e91f38679d4ed1ea6d99684ad12d8a1a0f785049d788630f17ee). Preserve these raw records; final qualification must execute on the resolved committed source. The conflict resolution retains the successful #498 post-main note. Separately, #502 post-main qualification 34014528298 failed its compressor allocation test; investigation is active and no green post-main claim is made.

## Clean artifact ruling

# Astra #500 current artifact integration ruling — approved, bounded

Reviewed clean frozen be055b4c602ffc8dcca0653815e303f3001bdd69 in engine-cp20-identity-plan. Graph-compiler is byte-identical to accepted 4e2fc87d/6f421d67; compressor/Cargo/config match delivered ad00d16b. The original builder executed during a documentation merge conflict and remains historical, explicitly not clean-head qualification evidence.

The separate clean ordinary invocation in /tmp/500-clean-delivery-builder.command.json identifies this exact source, explicit output /tmp/engine-500-clean-qualified and target /tmp/engine-500-artifact-target. Its authentic log records successful release compilation followed by builder exit1: expected fa78dc8d3f0d391b94419f5252504eee2aafbbf13853884157e24935ed092cc3, observed bed7d77cbd55e91f38679d4ed1ea6d99684ad12d8a1a0f785049d788630f17ee. This is an actual supported-build mismatch, not authorization to choose a digest from an unqualified file or alter numerical expectations.

Approve an exact observed-pin checkpoint followed by the ordinary verified rebuild and existing static/object/ABI, resource gate with its 26 rejection controls, hermetic worklet, current three-browser record/check plus self-tests and matrix qualification. Independently compare the successfully published module bytes/SHA to the exact pin and all current consumer records. Generated candidate/hash identities may change; canonical/PCM/resource numerical expectations, schemas, corpus pins, CI/lints and builders remain unchanged. Any unexpected numerical or further source discrepancy stops promotion for a concrete ruling. Preserve both initial dirty-merge and clean mismatch logs with honest provenance; do not overwrite failure evidence.

This ruling grants no timing authority and does not resolve or waive the separately observed PR502 post-main allocation qualification failure. That regression must be handled under its bounded successor and required CI; no merger may rely on the historical premerge PASS to ignore the known failure. Final exact-head Astra PR review and required CI success remain delivery gates.

Read-only source/log comparison; no source/Git/GitHub edits, builds, tests or timing performed.

## Current artifact qualification complete; delivery held for #503

Clean pinned candidate 02c74cc264d5d2ed98dda446ab1adb511f05de6e passes all seven existing consumer steps (builder, static, resource, hermetic, npm-ci, three-browser qualification with self-tests, matrix), each terminal exit 0. Published module independently hashes to bed7d77cbd55e91f38679d4ed1ea6d99684ad12d8a1a0f785049d788630f17ee. Generated matrix/results differ only in candidate and hash identity. Raw records and both historical mismatch invocations are retained in artifacts/issue500-integrated-delivery (33 payloads plus manifest); no numerical expectations changed. PR delivery remains held until the known compressor post-main allocation proof defect is resolved under #503.

## Integrated evidence PASS and delivered #503 base

# Astra #500 integrated evidence review — PASS, delivery held

Reviewed clean exact head 7d964d7bd148ea6923ae1778601f00bf0587a470 in engine-cp20-identity-plan. This is an integrated source/evidence review, not actual-PR approval or a waiver of #503.

The complete source contract accepted at 4e2fc87d/6f421d67 remains satisfied: graph-compiler is byte-identical, the two borrowed-piece visitors preserve canonical identity bytes and stack-only length calculation, and the independent fixed literal fixture and retained semantic/resource tests remain applicable. Integration adds delivered compressor source, closure records and qualified current-consumer identity; no unexpected production, Cargo, schema, numerical expectation, policy or framework change appears. Qualified candidate 02c74cc264d5d2ed98dda446ab1adb511f05de6e to reviewed head has no crates/host source/Cargo/config/scripts delta.

The historical dirty-merge builder is explicitly limited, not credited as clean qualification. Clean be055b4c ordinary build compiled successfully and failed the actual fa78→bed7 pin comparison with exit1. Under the bounded ruling, exact pin checkpoint 02c74cc2 then passed all seven recorded steps with independent numeric exit0: ordinary verified builder, static/object/ABI checks, resources including 26 rejected mutants, hermetic worklet, npm installation, current three-browser qualification with self-tests, and matrix check. Browser logs identify Chromium151.0.7922.34, Firefox153.0 and WebKit26.5. Generated results/matrix differ only in candidate/hash; numerical resource/PCM expectations remain unchanged.

Independently read the published 2,686,410-byte simd128 module and verified SHA256 bed7d77cbd55e91f38679d4ed1ea6d99684ad12d8a1a0f785049d788630f17ee, matching the current pin, results and matrix. Independently verified every manifest size/hash and exact Git tracking: original package24 payloads+manifest=25; integrated package33+manifest=34. Historical failures remain intact. Authentic raw-log terminal whitespace is not a source defect and should not be trimmed or used to weaken a checker.

The known compressor post-main allocation proof failure remains unresolved under #503. #500 source and artifact evidence are complete for this reviewed integration, but delivery stays held for that dependency and successful required CI. Any subsequent #503 integration needs checked source/artifact applicability, then actual-PR exact-head Astra review before merge. This report cannot authorize merging a failed required check or claim #503 fixed. No timing/performance or new full-workspace execution is claimed here.

Read-only source/log/hash review; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

PR504 delivered #503 as main 5159da6ceee5903ff1b9534a55ea149c38970acb after exact-head Astra PASS and required qualification34015517216 SUCCESS. Root merged that main and its closure record. Comparison against qualified candidate02c74cc2 shows only the known compressor test, explicit dev-only engine dependency and existing lock edge, plus previously accepted generated candidate/hash identities. All production source, normal dependency declarations, artifact pin and consumer numerical expectations remain unchanged. Per Astra final PR504 ruling, the existing #500 artifact qualification remains applicable without an unconditional rebuild. Actual #500 PR review and required CI remain mandatory; #504 post-main34015812921 is monitored independently.

## Delivered and remotely closed

PR505 merged at2026-09-06T06:20:49Z as71059eab8b83a39d5b3c69fd2aba9ca285409e52. Astra exact-head PASS applies to3622d6250375418fdedf7d045b24ef42496d2788 and required qualification34015869156 completed SUCCESS on that same head before merge. GitHub #500 is verified CLOSED. This delivers shared node/edge token emission with allocation-free length calculation and unchanged canonical bytes; CP20 remains PARTIAL because concrete independent hex encoders remain. The historical count of fourteen encoders is not recertified. Post-main CI is monitored separately.
