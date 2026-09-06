# 498: Collect graph source claim sets directly from borrowed claims

Active Luna attempt 1; actual-base approval below supersedes historical queue wording.

Queued control-plane maintenance after delivered #495. Source reviewed explicitly at396a97119583704888d7b2a20830c5720e16b189, graph/src/lib.rs blobc81d68d3779659a830ed596733f6c51e60ea71ef. Root must number/synchronize and verify actual implementation base before fresh Luna1. #475 remains the sole runtime feature. No implementation, tests/builds/timing or Git/GitHub mutations performed.

## Actual remaining product

Private GraphPreparedSourceSet::claimed_nodes creates an owned Vec of cloned node IDs. Exactly two callers immediately collect it into BTreeSets: bind_optional_source_set first clones the Vec's keys again into source_claim_set; GraphExecutor::new consumes the Vec into source_inputs. Neither retains or needs the intermediate Vec. Public claims() already exposes the immutable slice. This is a concrete independent residual explicitly excluded by495, not a second attempt at its combined-set change. Targeted local scope search and read-only open issue title query for claim/binding/CP4 found no separate dedicated open implementation; root's normal issue/body boundary must confirm before numbering.

Smallest implementation choice: in bind_optional_source_set, make source_claims a borrowed &[GraphSourceInputClaim] through source_set.as_ref().map(GraphPreparedSourceSet::claims).unwrap_or_default(); collect source_claim_set with source_claims.iter().map(|claim| claim.node.clone()).collect(). Retain the existing length comparison against source_claims.len(). In GraphExecutor::new, retain the current Option map/default structure but collect directly from set.claims().iter().map(|claim| claim.node.clone()). Remove only the now-unused private claimed_nodes helper. No public signature or new helper abstraction is needed.

This eliminates both intermediate vectors and one extra node-key clone population during bind validation. Executor already moved cloned keys out of its Vec, so its change removes a Vec allocation, not an additional node-key clone. Keep both resulting owned BTreeSets and every other set. For successful source binding, the two transient vectors disappear; for absent sources, old empty Vecs generally did not allocate, so no allocation saving is claimed there. No measured speedup, allocation-free binding, persistent resource-layout change or render optimization is asserted.

## Unchanged contracts and exact paths

Allowed production/test source is crates/graph/src/lib.rs only. Numbered spec/evidence allowed. Existing crates/source/src/lib.rs render/transactional fixture is run unchanged. Do not edit runtime.rs, program.rs, source implementation, Cargo, public claims()/driver interfaces, source_input_buffers construction or source render loops.

Retain source validity evaluation and short-circuit order exactly: envelope, resource consistency/driver.claim_count, strict ordered-unique claims and Input-stage classification, then the existing set/count condition. Do not sort or deduplicate the original claims slice, and do not validate using only the collected set; duplicates/order are meaningful rejection conditions. Keep495's borrowed union coverage, source overlap, duplicate binding, observer/envelope priorities, structural rejection and returned owners unchanged. Source indices and buffer mappings still enumerate ORIGINAL claim order; no change to begin_block/copy_track_input order, channel mappings, source recycling or PCM reduction. The borrowed slice's last use is in validation; let the borrow end naturally before returning/moving the source owner. No earlier driver/processor/observer destruction, new ownership transfer, or retained reference into moved storage.

## Finite proof and gates

Source-mechanism gate is exact removal of claimed_nodes and both transient collect-to-Vec paths, with exactly one node clone per direct set insertion. No runtime instrumentation or allocator harness is needed to prove that source transformation. Review all remaining claimed_nodes references are absent and both direct collectors are real production callsites; do not replace them with another allocating adapter.

Reuse unchanged #495 `tests::binding_coverage_preserves_validation_and_ownership`, plus `tests::binding_rejects_duplicates_and_returns_all_ownership` and `tests::structural_layout_rejection_is_shared_after_binding_validation_for_source_families`. Those cover set membership, missing/extra/overlap, exact error priority and returned source reuse. The exact source test `tests::one_four_channel_source_fans_out_to_three_inputs_in_the_sequential_executor` in crates/source/src/lib.rs builds the real four-channel source with three differently mapped inputs, rejects missing/extra/overlapping claims transactionally, binds through GraphExecutor and renders the analytic four-word [10,10,10,10] output. It exercises the second collector and current source-input setup; retain it unchanged. No new fixture matrix is required absent a concrete finding. Existing source driver claim-order/recycling tests `tests::graph_driver_last_claim_recycles_in_call_and_incomplete_paths_recycle_next_begin` and `tests::graph_driver_zero_claims_recycles_in_begin_and_retains_no_plane_class` can be retained by the normal source library suite; no new scheduler or source mock framework.

Commands: `cargo test --locked -p graph --lib` and release equivalent; `cargo test --locked -p source --lib tests::one_four_channel_source_fans_out_to_three_inputs_in_the_sequential_executor -- --exact` and release equivalent, each actual one-test result. Verify the actual module/filter before capture, never credit zero tests. Run affected strict `cargo clippy --locked -p graph --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, `git diff --check`, and existing `bash scripts/check-graph-policy.sh`. Record actual command/cwd/source/log/numeric status, not a combined shell's final status standing for earlier commands. Existing tests plus the tightly preserved call order are proportional proof; no mandatory production mutant, new allocation counters or timing invocation.

One coherent Luna1 pass, Astra review, Sol2/3 only after finite FAIL, then hardstop/rescope. After source PASS, integrate actual current main once and perform proportional ordinary artifact verification; any actual mismatch requires the narrow current-pin qualification ruling, never automatic PCM/resource repins. Actual-head PR review and required CI remain merge gates. Parent349 should count only elimination of these intermediate claim vectors; remaining owned supplied/member/required/claim/observer sets retain distinct semantics and are not silently declared optimized.

## Numbered actual-base checkpoint

GitHub498 number and title match this stateless spec. Branch codex/cp4-direct-source-claims starts from delivered main396a97119583704888d7b2a20830c5720e16b189 and carries the pushed495 closure decision. Graph/Cargo/config match the reviewed delivered base. Root previous boundary audit found331 remote issues and235 local numbered specs with no missing remote identity before498 creation. This is independent control-plane maintenance; #475 remains the sole runtime feature. Await Astra numbered actual-base approval before fresh Luna1 assignment. No implementation or artifact build is authorized by this queue checkpoint.

## Astra actual-base approval and assignment

# Astra #498 numbered actual-base review — PASS

Exact clean head2402100ef545d2e8882e24bcaba23623abb8b984 in engine-cp4-source-claims, based on delivered396a97119583704888d7b2a20830c5720e16b189. Complete approved source-claims brief body is preserved verbatim under the numbered title. Only498 spec and495 delivery/post-main-CI documentation differ from delivered main; crates/hosts/tools/Cargo/config/scripts source is identical. Root reports matching remote498 identity/body and495 required/post-main qualification success.

Approve fresh Luna1 for the frozen two-caller transformation: collect the existing owned sets directly from borrowed claims, remove only the now-unused private claimed_nodes Vec helper, preserve original claims order/count/driver validation, error precedence, source-index mapping and returned ownership. Retain all other sets and the495 borrowed-union result. Exact graph/lib source and current real source-fanout/transactional tests remain applicable. No public API, source runtime, scheduler, resource layout, allocator framework, new matrix or timing is introduced.

The unchanged numbered finite gates and actual command/source/status evidence requirements are sufficient. Root owns implementation assignment/checkpoints and later artifact applicability, actual-head PR review and required CI. #475's just-recorded final FAIL is preserved separately; #498 does not repair or overlap that runtime issue and may proceed as independent maintenance.

Read-only source/Git inspection; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root assigns fresh Luna attempt1 within this scope. Root owns all Git/GitHub checkpoints. No timing or artifact qualification until source acceptance.
