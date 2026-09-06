# Express typed route-node mapping as total functions

CP-17 remains applicable on delivered c075e44e. Read current graph-compiler/session/graph identity source (unchanged in the inspected main-derived checkout), original349 row277 and CP reconciliation. A bounded open-issue title search found no existing total-route/CP-17/graph.port.unknown helper correction; related historical graph-compiler audit99 is not proof this cleanup shipped. Root must perform the normal issue-boundary identity check before numbering. No source edits, tests/builds, timing or Git/GitHub mutations performed.

## Smallest product and invariant ruling

ids.rs:120 route_source_node exhaustively maps typed RouteSource::Track/SubmixOutput and always returns Some. ids.rs:128 route_destination_node exhaustively maps SubmixInput/OutputInput and always returns Some. compile.rs:372/379/405 consequently contains three impossible None branches emitting graph.port.unknown for route source, route destination and routed sidechain source. Replace their Option return types with GraphNodeId, unwrap only their Some constructors, and replace exactly those three let-Some/else blocks with ordinary let bindings. This is an honest control-plane API simplification; no measured compile-time or runtime gain is established.

Do not sweep expects into this change. session::StableId has a private String and parse enforces exactly the same1..127-byte lowercase ASCII grammar as graph::StableGraphId. These route inputs contain StableId, not arbitrary strings. gid's expect atids.rs:112 is therefore a validated invariant for these typed mappings, not a demonstrated reachable malformed-ID panic. CompiledSession also keeps its normalized model private. topo's expect after cycle_witnesses and the diagnostics-return barrier is a different invariant and remains unchanged. If any actual accepted-input counterexample is found, preserve it and request a separate precise ruling before repair; do not assert that the audit's generic “real failure is panic” wording proves reachability.

Allowed implementation paths: crates/graph-compiler/src/ids.rs (two helpers only), src/compile.rs (three caller blocks only), existing src/lib.rs cfg(test) module for a compact direct mapping case, numbered spec/evidence. No session/graph type changes, validation removal, diagnostic sorting changes, extra public API, generic helper/framework or caller rewrites outside the exact three sites. Keep route_transform nonfinite checks before mapping; keep resource/cycle/prepared-effect checks and error return order exactly where they are. Do not delete graph.port.unknown from any external vocabulary or other source solely because these three producers vanish.

## Finite proof

Add one small direct helper test with handwritten expected node identities: all seven SendTap values map to their exact TrackStage for a typed track; SubmixOutput maps to Submix; SubmixInput and OutputInput map to their respective variants. Use valid stable IDs including one maximum-length identifier to substantiate the shared grammar boundary. Compare actual GraphNodeId values directly, not another mapping helper. No invalid-ID injection through unsafe/private-field bypass and no artificial None/panic control.

Retain actual canonical graph/session and diagnostic behavior through existing fixtures:
- graph-compiler library tests `tests::accepted_session_compiles_binds_and_renders_direct_route`, `tests::canonical_artifacts_are_complete_and_repeatable_100_times`, `tests::route_transform_bits_participate_in_semantic_hash`, `tests::route_transform_uses_the_canonical_db_to_gain_conversion`;
- `tests::a_sidechain_lifted_chain_slot_falls_back_instead_of_failing_the_compile`, `tests::cycle_witness_skips_acyclic_residual_nodes_downstream_of_cycle`, `tests::every_cyclic_scc_has_one_closed_sorted_witness_and_edge_paths`;
- existing session integration `diagnostic_parity`, specifically `parse_canonical_and_compile_diagnostics_have_code_path_and_span_parity`, plus `invalid_matrix` unchanged. These retain actual missing-reference/typed validation and diagnostic boundaries; do not invent unreachable graph.port.unknown inputs.

Run `cargo test --locked -p graph-compiler --lib` debug/release (includes named existing cases and new mapping test), and `cargo test --locked -p session --test diagnostic_parity --test invalid_matrix`; retain nonempty counts and exact commands/statuses. Reuse the already ruled release --lib test-harness execution approach if the documented abort/unwind graph artifact collision recurs; no production panic-profile change. Run fmt/diff and affected strict graph-compiler Clippy with its actual feature configuration. No benchmark, worklet repin, target matrix or second diagnostic corpus is justified by these total-return/callsite changes. Any inherited gate failure needs classification rather than blanket suppression.

Before/after evidence should establish only these two signatures/three callers changed and existing canonical/diagnostic fixtures remain exact. No extra mutation campaign is necessary for removing impossible branches. Parent349 retains other CP findings, including separate cycle/scheduler/invariant concerns. Number/synchronize before fresh Luna1; Astra scope/review, Sol2/3 only after failed verdict, hardstop after3. Root handles exact-head PR/requiredCI/remote closure. #463 remains sole runtime feature; this small independent control-plane maintenance slice can be queued without overlapping lane/runtime work.

## Numbered baseline

GitHub490 has the matching number/title, on deliveredmainc075e44e81d7be978bb6a7420a9724db134c46cf. Root boundary audit found no missing local numbered issue identity. Pending numbered Astra approval before Luna1. This control-plane maintenance slice does not overlap463 lane/runtime or488 graph/program tests.

## Numbered approval and Luna attempt 1

# Astra #490 numbered scope/base review — PASS

Exact head641966ff68e87f181a58265c19316202385fb871, engine-490-route-mapping, deliveredbasec075e44e81d7be978bb6a7420a9724db134c46cf. Independently checked only numbered spec differs from base; entire approved CP17 draft body is retained with matching title/numbered baseline. Root supplies synchronized GitHub490 number/title/body and clean boundary audit; no remote query was performed for this review.

Approve fresh Luna1 for the exact two total route helper signatures and three unreachable caller branches, plus the compact existing-module mapping test and numbered evidence. Preserve all genuine validation, error order, canonical results and the specifically excluded gid/topo expects. Those invariant classifications are not authority to suppress a demonstrated accepted-input panic. No session/graph API, diagnostic vocabulary or generic framework change.

Full frozen finite helper mapping/identity and current graph/session diagnostic fixtures remain binding, including all taps and maximum valid ID. Root owns checkpoint/synchronization and any inherited gate-failure ruling. No timing, speculative speedup, target/artifact expansion or overlap with463 runtime work. Actual-head PR/requiredCI and remote closure remain delivery gates.

Read-only filesystem/Git inspection; no source/spec changes, tests/builds/timing or Git/GitHub mutations performed.

Root adopts scope PASS and assigns fresh Luna1. Pause at the first compiling/focused-green exact-path tranche, retaining actual command/log/numericstatus before reporting it. Root owns Git/GitHub checkpoint and delivery.

## Luna attempt 1 source and finite gates

Source20eeb26a implements exactly the two total helper signatures, three caller bindings and direct mapping fixture;9d069b8b changes only formatting in that fixture. /tmp/490-luna1-{focused,debug,release,session,clippy,diff,fmt}/command.{command,log,status} retains actual runs: focused1, graph compiler62 in each profile, session parity1 plus invalid-matrix14, strict affected Clippy and diff all return0. Initial fmt returns1; formatting is applied and root independently captures restored fmt0 in /tmp/490-root-fmt-restored.*. Earlier tests precede this formatting-only correction; no semantic test rerun is claimed.

Pending one consolidated Astra Luna1 verdict. Validation order, transforms and validated-invariant expects remain unchanged. No timing or performance gain is claimed.
