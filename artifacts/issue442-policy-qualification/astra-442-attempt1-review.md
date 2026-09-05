# Astra #442 Luna attempt 1 review — FAIL

Exact reviewed checkpoint: `74ef8d5ed62c4562c1e9072a7d8de63c9c0c87e7`, `/home/bl/misofm/engine-430-plan`, against approved `b3185b15`. Source checkpoint `8394abab` plus the evidence correction comprise this attempt.

FAIL. Three finite blocking groups remain within the frozen scope. Luna attempt 1 is consumed; a bounded Sol attempt 2 may correct these together. No full workspace/artifact qualification should precede a coherent source PASS.

## 1. Restore accidentally damaged existing graph tests and qualify the actual checkpoint

`crates/graph/src/lib.rs` still defines GraphNodeId::TrackStage with required `track_id` AND `stage` fields (lines 73–77), but three existing test constructors at approximately 1984, 2028 and 2071 now omit `stage` entirely. The cumulative diff deletes `stage: TrackStage::PostInputBuiltins` from all three. This is a definite Rust missing-field compile error in the graph unit-test configuration, independent of any run environment. The same change broadens the existing stage-specific membership filter around 1975 to all TrackStage nodes. These edits are unrelated to policy metadata and must be restored exactly; do not weaken the graph fixture or enum to make them compile.

Root independently confirmed this on the exact checkpoint: `cargo check --locked -p graph --tests` exited 101; `/tmp/engine-442-exact-checkpoint-check.log` reports the missing stage fields at 1984/2026/2071. The retained `/tmp/luna-442-graph.log` says graph's 52 tests passed, but cannot qualify these actual malformed constructors. Preserve that record candidly as earlier-source evidence and regenerate proportional graph/fmt/check evidence on the final corrected source. This is not a request for a broad build or new graph tests.

## 2. Complete the frozen actual-host causal proof and controls

The only added policy test is builtins-compiler's `control_delivery_metadata_reaches_both_sealed_strip_bank_owners` around line 4337. It does lower real banks and observes owner-sourced metadata, which is useful accepted proof. However, all three preparations pass an empty control request slice, and the test directly selects builtin preparation functions. It never goes through host-core's policy selection or the actual WebEngine `compile_ready` call.

No new host policy assertion, private host construction observation or wrong-wrapper mutation evidence exists in the source/logs. Existing host-web behavior tests cannot discriminate the missing opt-in because #442 intentionally does not read this metadata during render. Replacing the WebEngine call with the old Concurrent wrapper leaves the new direct-builtin metadata test untouched. Thus the frozen mandatory host-to-owner assertion and same-assertion wrong-wrapper control are absent, not merely under-documented.

Finish the existing bounded proof: prepare actual default/raw and explicit live-console owners (populated control requests), retaining the no-console/default control; observe both PostFader and PostMatrix real sealed bank metadata. Add the actual private WebEngine construction assertion using a preparation-only observation before binding, as already authorized. Demonstrate that changing only its dedicated production entry call back to the old Concurrent wrapper makes that same assertion fail for the expected policy mismatch, while the original passes. Keep the mutation well-formed and verify its exact target/hunk; syntax or setup failures are not the control. Retain an appropriate default-path discrimination so changing the raw/general default does not silently opt exported producers in. No render report field, duplicate simulated policy, test-only request-value assertion or new generic framework is warranted.

## 3. State the explicit caller contract in the public dedicated entries

The new host-core entry at prepare.rs:488 and builtin entry at builtins-compiler/lib.rs:2324 have only one-line descriptions saying producers/control ownership are “exclusively retained between render calls.” That does not expressly require what the frozen contract depends on: the caller must retain the producer endpoints and admit records only outside exclusive render calls, with no concurrent enqueue during render. The enum variants also carry no explanation of that distinction.

Document those obligations directly on the dedicated public entry (and its builtin counterpart/reference), and state that this is a caller declaration rather than runtime synchronization/enforcement. Existing general APIs remain Concurrent. This is the previously required documented policy contract, not a request for locks, ownership API redesign, unsafe, scheduling enforcement or a native cutoff guarantee.

## Accepted implementation and preserved boundaries

The graph-owned unversioned enum/default query is dependency-safe. PreparedBuiltinsSession privately carries one policy to both concrete fader/matrix owners. GraphPreparedBuiltinBankInfo obtains the value from `bank.processor.control_delivery()`, not from a separate request/default; the added stage observation identifies the actual member stage. Host-core's private helper chooses existing/default versus explicit builtin preparation, and GraphCompiler gains no duplicate policy. The WebEngine production call currently selects the dedicated entry. Existing raw/general wrappers select Concurrent; no ordinary HostConsoleRequest field/default changes.

No production queue drain, record, acknowledgement, application sample, callback, arithmetic, factory/Any, scalar behavior or render policy query is changed. Both concrete owners merely store/query preparation metadata. Source resource accounting still uses real size_of, and capi changes are limited to matching owner mirrors and their resulting expectation deltas. No new capi runtime/ABI change appears. The supplied focused builtins/host/capi/policy records are useful, subject to accurate final-source provenance; immutable workspace, supported targets/current artifact and actual-head PR/required-CI gates remain pending.

Reviewed the full numbered contract/frozen-base approval, cumulative source changes and existing evidence. No source changes, builds/tests, benchmarks or Git/GitHub mutations were performed. Only this `/tmp` verdict was written. #442 remains the sole feature; #430/#443/#444/#431 retain their separate outcomes. This verdict is one consolidated adversarial review, not authorization for additional Luna sub-attempts.
