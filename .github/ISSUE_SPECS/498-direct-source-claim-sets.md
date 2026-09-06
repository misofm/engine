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

## Luna attempt1 candidate

Source735dc5d5 implements only the two direct collectors and removes their private Vec helper. Existing graph debug/release57 each, exact real source fanout debug/release1 each, strict graph Clippy, fmt/diff/graph policy all pass. The first graph-debug source/PATH metadata limitation is preserved and explained in artifacts/issue498-direct-source-claims; subsequent logs identify the committed source and effective environment. No tests, production behavior contracts, timing or delivery gates were added or changed. Pending consolidated Astra source review.

## Astra Luna attempt1 PASS

# Astra #498 Luna attempt 1 — PASS

Exact clean headfa9d86653112637a7e350545166cc12fb72bb0bd, source735dc5d5aea127687517927aec70ccda0db67185, engine-cp4-source-claims. One consolidated source verdict against the complete numbered scope. Read-only source/Git/log inspection; no tests/builds/timing or source/Git/GitHub mutations performed.

The transformation exactly follows the approved two-caller design. bind_optional_source_set borrows claims(), clones each node directly into its existing owned set, and compares the resulting length against the original slice length. GraphExecutor::new collects source_inputs directly from the borrowed claims while retaining its original Option/default shape. The private Vec-producing claimed_nodes helper is removed and has no remaining graph references. No other source/test path changes, public API, helper abstraction or retained collection is introduced.

Source validity and evaluation order are preserved: source envelope/resource/driver count/order/type checks still precede the same count comparison; original claims are not sorted or deduplicated in place. #495 coverage/overlap/duplicate/observer/envelope priorities and returned ownership remain untouched. Source-input buffer mappings still enumerate the original claims separately, with identical indices and render driver call order. Borrowing ends before source ownership moves; no caller-owned plan/driver/processor is dropped earlier. The saved mechanism is two intermediate vectors and one bind-time extra key-clone population, not elimination of the owned sets or measured runtime work.

Retained graph library debug/release each execute57 tests including #495's exact binding/priority/reuse fixture. The actual source-fanout test executes once in each profile, exercising missing/extra/overlap source claims and the real executor's four-channel-to-three-input mapping with analytic PCM. These existing tests plus the exact preserved production boundaries satisfy the frozen finite proof; no new fixture or mutation is required. Strict graph all-targets/all-features Clippy argv includes -D warnings and status0; configuration warnings do not justify relabeling the invocation. Fmt, diff and graph policy records also return0.

All9 manifest payload hashes/sizes independently verified, exact10-file tracked coverage including manifest. The first debug record's base-plus-dirty-source and inherited-PATH metadata mistake is disclosed, not silently rewritten; the exact graph diff was checkpointed without subsequent code changes, and later committed-source release/fanout/Clippy records have the effective prefix. There is no crates/hosts/tools/Cargo/config delta after735dc5d5. No invented contemporaneous provenance or new delivery run is claimed.

No remaining source/evidence blocker. Root may proceed with proportional delivery on actual current main and ordinary artifact verification; preserve any real mismatch for a bounded current-pin/current-consumer ruling before promotion. Source PASS does not itself approve automatic repinning, artifact qualification or merge. Actual-head PR review and required CI SUCCESS remain mandatory; broader CP4/349 work is not closed by this slice.

Root verified remote main remains396a97119583704888d7b2a20830c5720e16b189, already integrated. Accepted graph source is frozen for ordinary artifact verification.

## Current artifact integration amendment

# Astra #498 actual artifact mismatch — approve bounded delivery amendment

Reviewed source5d8d893cdf47b46a7ada6761307cffc5814183b7 on delivered396a9711. Graph source remains byte-identical to accepted735dc5d5. Actual command JSON names the dedicated engine-498 target/output and frozen source; retained log completes release compilation then builder status1 rejects expected87007c1aaad418f44074132b3c73876d77fcff0b778256e55c05a377a5277f6d versus observed8a42eb47c36ff8053b66054d72a06dee48b3c3577943f21119e0709fe64d0d0c. This is an actual artifact identity result, not a numerical mismatch or new source defect.

Root may synchronize the delivery amendment, preserve the authentic failed invocation/log/status, and checkpoint only that exact observed pin with evidence. Then ordinary verified rebuild on unchanged production/build source must establish the published module identity, followed by existing static/object/ABI, resources with26 red controls, separate hermetic, current Chromium/Firefox/WebKit qualification with self-test mutations and generated matrix check. Record actual qualified candidate/hash in current generated consumers only after those executions succeed.

No automatic promotion, numerical/resource expectation change, corpus repin, lint/CI/policy change, production revision, new framework/matrix or timing authority is granted. The observed digest above is from the builder record; final independent output identity belongs to verified rebuild evidence. Any other discrepancy needs its own concrete ruling. Source PASS remains subject to delivery qualification, exact-head actual PR review and required CI SUCCESS before merge/remote closure.

Read-only source/log inspection; no tests/builds/timing or source/spec/Git/GitHub mutations performed.

## Qualified current artifact

Candidatea928a42d passes verified builder/static/resources26/hermetic/npm/current3browser+self-tests/matrix, all7 numeric exits0. Final module SHA2568a42eb47c36ff8053b66054d72a06dee48b3c3577943f21119e0709fe64d0d0c agrees with the pin and generated records; only candidate/hash identity changed at5ff72a70. Full immutable raw records and original mismatch1 are retained. No numerical expectation or production source changed after acceptance. Actual-head PR review and required qualification remain before merge.
