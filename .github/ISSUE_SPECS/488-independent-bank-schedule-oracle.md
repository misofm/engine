# Pin runtime bank-unit ordering with independent expected schedules

CP-15 remains a test-independence gap. Original349 row275 identifies the mirrored scheduling oracle. Current program.rs:1632 units_in_runtime_order copies first-member emission, group collection and member sorting from runtime.rs:1797 units_of. Its use by runs_in_runtime_order and bank-window property interpreters can agree with a shared mistake. Graph program/runtime source is byte-identical to delivered4587bfae at this inspection. No source edit, test, build, timing or Git/GitHub mutation performed.

Smallest closable outcome is an independent finite literal schedule contract against the ACTUAL pub(crate) runtime::units_of function, plus honest classification of the existing interpreter as a model. Supplement existing property tests; do not replace them with another generic scheduler or claim complete independent coverage of cohort_runs, redirects or arithmetic. Keep the actual runtime implementation unchanged.

Allowed paths: crates/graph/src/program.rs within existing cfg(test) module and its model comments; numbered issue/spec and retained command/mutation evidence only. Existing Membership, BankMembership and PlannedUnit are pub(crate) and derive the equality/debug needed by assertions. No public API, runtime.rs permanent change, dependency, new test harness or workflow is needed. A temporary one-site runtime mutation is an evidence action only and must be restored before checkpoint. Root numbers/synchronizes before Luna1; this queued test-only slice does not displace463 runtime priority.

## Exact finite construction and expected results

Add one named test `program::tests::runtime_units_match_literal_bank_schedules`, using a tiny fixed ExecutionProgram construction with unique, deliberately nonmonotonic node IDs so node ID, op position and member lane cannot be conflated. Fill existing Op and ExecutionProgram fields coherently; the unit grouper only reads ops and membership. This is a unit scheduling fixture, not a claim that its buffers render a production graph. Do not invoke another grouping algorithm to calculate expected results.

1. Empty ops/membership => literal empty list. An eight-op version with no memberships => literal eight `(None,[index])` units in0..7 order.
2. On that same eight-op program: op0 unbanked; op1 Effect(0),position1; op2 Builtin(0),position1; op3 unbanked; op4 Effect(0),position0; op5 Builtin(0),position0; op6 Effect(1),position0; op7 unbanked. Exact handwritten result:
   `[(None,[0]), (Some(Effect(0)),[4,1]), (Some(Builtin(0)),[5,2]), (None,[3]), (Some(Effect(1)),[6]), (None,[7])]`.
   This independently pins first encountered emission rather than lane0's later op; interleaved outsiders are deferred after the already-emitted bank; lane order differs from op order; later members do not emit twice; equal numeric effect/builtin IDs remain distinct; singleton groups and final outsiders remain present. Positions are unique and dense per membership, matching the valid caller contract. Do not add duplicate-lane/invalid-ID behavior or oversized malformed bank indices to scope.
3. For the existing effect-only interpreter helper, use the same fixed ops with only the Effect(0)/Effect(1) memberships and a separate handwritten expected nested list: `[[0],[4,1],[2],[3],[5],[6],[7]]`. Assert BOTH actual runtime units (projecting only membership tags away) and units_in_runtime_order against that literal. Never use either result as the other's expected value. This anchors the property interpreter to a genuinely independent schedule case without manufacturing a second scheduler.

Use explicit ordered equality, not sorted/flattened set equality. An optional assertion that all eight indices occur exactly once is redundant with this literal equality and is not an extra gate. Keep existing bank-window/chain random properties unchanged; comments must distinguish their modeled scheduling from this independent finite contract. No claim that this one case independently validates the other mirrored dataflow/chain/gather models.

## Discriminating evidence and proportional gates

Freeze the literal test first. Run `cargo test --locked -p graph --lib program::tests::runtime_units_match_literal_bank_schedules -- --exact` in debug and release, one test each. Temporarily remove only `members.sort_unstable()` from actual runtime::units_of. The SAME unchanged literal equality must fail because actual Effect(0) members become[1,4] instead of[4,1] (and builtin members similarly). Record actual one-site diff, exact command,101 with the named equality/mismatch, then restored0. A compilation error, zero tests, changed expectation, or mutation of only the test model is not accepted. Exactly one bounded control; no mutation campaign.

Run the existing graph library suite debug/release to preserve its nonempty bank-window and cohort-chain properties, plus fmt/diff and affected graph policy. Existing names include a_bank_window_never_recycles_a_physical_slot, no_slot_is_recycled_inside_a_merged_bank_window, bank_window_hoisting_preserves_dataflow_on_random_graphs and cohort_chain_merging_preserves_dataflow_on_random_graphs under program::tests. No audio benchmark, timing, allocator/host fixture, target matrix or worklet repin is justified by a cfg(test)-only schedule assertion. Root later performs proportional delivery/source identity and exact-head PR/requiredCI/remote closure.

Astra scope/review, Luna1, Sol2/3 after failed verdicts; hardstop and explicit rebrief after three failures. CP-15 closure is the independent schedule-checking outcome, not a runtime performance gain or completion of CP-14/other mirrored-model findings. Do not create nineteen audit issues or broaden this into scheduler redesign.

## Numbered baseline

GitHub488 has the matching number/title; frozen baseline is currently deliveredmain4587bfae673adbb848cda66f473beecefeae8deb. Actual graph program/runtime inputs are unchanged by pending accepted238/485. Pending numbered Astra approval before Luna1. This independent cfg(test)-only slice may proceed alongside delivery qualification;463 retains next runtime priority.

## Numbered approval and Luna attempt 1

# Astra #488 numbered scope/base review — PASS

Exact headbd0d8d636d750043acf3f5f62017e6a4d090ad68 in engine-488-schedule-oracle, delivered base4587bfae673adbb848cda66f473beecefeae8deb. Independently checked only numbered spec differs from base; graph program/runtime are unchanged. Live GitHub488 is OPEN with exact matching title and body. The complete approved draft body is retained, with numbered title/baseline only.

Approve fresh Luna1 for the frozen cfg(test)-only scheduling assertion in program.rs. Literal expectations must independently check actual runtime::units_of, including interleaving, first-member emission, lane order, effect/builtin identity separation and singleton/unbanked/empty cases. The effect-only model is checked against its own handwritten literal, never used to generate the runtime expected result. Preserve all existing property tests; do not claim other mirrored models have become independent.

The one actual temporary members.sort_unstable removal must fail the SAME unchanged expected-schedule assertion, then restored source passes. No permanent runtime.rs change, public API, new scheduler/framework, benchmark or target/artifact expansion. Existing graph library debug/release, fmt/diff and graph policy are finite proportional gates. Root owns coherent checkpoints, synchronized evidence, exact-head PR/requiredCI and remote closure. #463 retains runtime priority; independent488 may start on this approved base.

Read-only source/Git/GitHub inspection; no tests/builds/timing or repository mutations performed.

Root adopts PASS and assigns fresh Luna1; pause for exact-path checkpoint at the first focused-green source tranche. Root owns Git/GitHub.

## Luna attempt 1 source and evidence, pending Astra

Source359548c9 adds the literal mixed-bank schedule and model classification comments. Retained /tmp/488-luna1-* PTY records show focused debug/release passing, intended runtime-sort mutation101 at the unchanged mixed literal assertion, and both full graph library profiles56 passing. First checkpoint debug output was not retained; the later records are recaptures, not retroactive provenance.

Root flags incomplete frozen evidence for review: the effect-only case currently compares only the model to its literal, without the independently required runtime projection; no actual mutation diff was retained among the reported files; and the fmt/diff/workspace-policy shell bundle captures only its final exit while the specifically affected graph policy has no retained run. These are not silently waived by the green full suites. Runtime source is restored and the worktree is clean. Pending one consolidated Astra Luna1 verdict before further implementation.
