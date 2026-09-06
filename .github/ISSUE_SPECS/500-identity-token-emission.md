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
