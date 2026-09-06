# refresh_channel_symmetry is O(records×lanes) under sustained per-block record traffic

Status: OPEN, queued scope amendment for audit #349; no implementation assigned.

## Root priority and delivery ruling

The user authorized autonomous completion of all audit fixes, including work previously deferred. This amendment brings the existing #238 per-record bookkeeping repair into the active audit queue, preserving its original scope and historical evidence. It is queued behind the current #442 runtime feature. Reuse #238 rather than creating a duplicate. Astra must approve this synchronized numbered scope and the actual implementation base before Luna attempt 1.

The product slice is exactly the per-lane cached-bit repair and its correctness/mechanism proof. Full-bank refresh after ramp blocks/reset stays required and unchanged. Broader RT-5 ramp-block extraction and descriptive sustained-record measurement remain retained audit obligations requiring separate bounded briefs; this issue cannot close all RT-5. No benchmark invocation or projected performance gain is authorized.

## Historical issue body (preserved, not current measurements)

From the #235 fix verification (ledger on #235): `refresh_channel_symmetry` walks all lanes per drained record, so a dense automation ride (a record on every track every block) pays +51% on a synthetic 8-track probe (8.46 vs 5.60 µs/block); ~20 µs worst-case at 64 tracks. Digest-invisible, needs sustained per-block traffic to matter — non-blocking for launch. Repair shape (verifier's): per-lane refresh in `set_trim_signed` instead of the whole-bank walk. Post-launch item; standard implement→verify protocol.

## Current source reconciliation and binding implementation brief

# RT-5 / #238 current-source reconciliation

**Finding remains live, but part of the audit narrative is already addressed. Reuse and amend existing #238; do not open a duplicate implementation issue.** This is queued scoping only. #442 remains the sole feature and no code or timing is authorized here.

Inspected the clean delivered-#435 source in `/home/bl/misofm/engine-435-plan` (the reviewed PR #447 tree delivered as main `99df5cf6c639f0909f82e116eb776e95c172536c`) and read current GitHub #238, #235/#237 and #349 plus related issue searches. No build, test, benchmark, legacy source, repository/spec edit or Git/GitHub mutation.

## Existing ownership and already-delivered work

[Issue #238](https://github.com/misofm/engine/issues/238), “refresh_channel_symmetry is O(records×lanes) under sustained per-block record traffic”, is OPEN, bug-labelled, unassigned, and has no comments. The current issue search found no second titled refresh repair or assigned competing owner. Its original body explicitly calls it a post-launch, nonblocking follow-up; it is not implementation authority for a launch feature without an explicit priority/scope amendment. No numbered local #238 spec was found in the inspected spec catalog. Root should synchronize a stateless #238 spec and its existing remote body before assignment, preserving the old evidence and changing the queued priority explicitly if desired.

[Issue #235](https://github.com/misofm/engine/issues/235) is CLOSED. [PR #237](https://github.com/misofm/engine/pull/237) is MERGED at `8c2f588200e746d7b7119ef7cafd2315f8e7ea84`. It restored the collapse-dispatch short circuit and introduced the held symmetry byte with writer-side refresh. Its final closure comment explicitly names #238 as the remaining dense-ride follow-up. Those changes are still in current source: rack/lib.rs:1827 uses `armed && self.all_lanes_symmetric()`, and builtins/lib.rs:1576 reads the cached bit, with the full definition recomputed only by its debug assertion. Thus a settled no-record release dispatch does not unconditionally recompute the old input-word witness. Do not redo #235 or claim that this stationary cost is still open.

The historical #235/#238 timings establish the origin of the follow-up, not a measured estimate for this delivered tree. No current cycle, microsecond, register-spill count or speedup is established by this review.

## Exact remaining mechanism

In `crates/builtins/src/lib.rs`:

- `lane_read` (:817) materializes a lane value via `Lane::store` into an eight-word scalar array.
- `refresh_channel_symmetry` (:1036) iterates all eight maximum bank positions and reconstructs the entire cached mask using `compute_lane_channel_symmetry` (:1636). Padding/out-of-width positions return false before extraction. Active positions compare exact f32 bit patterns for current trim, target and step; integer countdown equality; and the twelve HPF/LPF coefficient words per channel. It is not an integrator-state comparison and does not redefine other witness terms.
- `set_trim_signed` (:1058–1095) changes one lane's selected channels and then invokes that whole-bank refresh. `set_trim_db` and `set_polarity_invert` share this writer. Consequently R admitted retargets cause R whole-bank comparisons although only one lane's compared words change per record.
- The other whole refresh calls are preparation (:1027), dual ramp-block settle (:1257), collapsed ramp-block settle/mirror (:1340), and reset (:1520). These maintain broader writes. They are separate from the excess per-record cross-lane comparison.

The actual banked production path is builtins-compiler's `BuiltinBankProcessor::begin_block` (:390–430): it drains each lane's real input queue, admits the record into that lane's LIVE witness before dispatch, then calls BuiltinInputBank::set_trim_db/set_polarity_invert. The public setters (:2674/:2705) validate populated membership, dispatch the existing W4/W8 stage and reach the shared single-lane writer. Scalar ConsoleInputProcessor::process (:3025–3051) drains its real queue and calls InputBuiltins' scalar setters (:2446/:2458), reaching the same generic writer at lane zero. InputStage process/process_mono and reset supply the remaining refresh calls through the established public scalar/bank processing/reset surfaces. Do not change any drain location, LIVE admission, collapse dispatch or public parameter validation to repair this bookkeeping.

The audit's broader RT-5 wording includes per-ramping-block SIMD-to-scalar extraction. A single-lane retarget fix removes the R×active-lanes amplification; it does not eliminate all refresh extraction, especially after a ramp block or reset. The old source-level “up to 240 SIMD registers” wording is not a current codegen measurement and must not become an acceptance threshold. If block-refresh extraction is pursued later, retain it explicitly as a separate unbriefed RT-5 residual; do not make that optimizer work a hidden condition for closing #238.

## Recommended smallest closable #238 amendment

Product contract: after a valid single-lane trim/polarity retarget, update exactly that lane's cached channel-symmetry bit from the unchanged definition; preserve every other cached bit. Use an ordinary clear-and-set bit update, capable of both true-to-false and false-to-true transitions. Retain complete refresh at preparation, both ramping process arms after settle/mirror, and reset. Leave the full comparison/debug oracle, parameter arithmetic, ramp ordering, recovery, elision plans, state copying and all LIVE/AGREEING/DESIGNED/collapse semantics unchanged.

Allowed implementation scope: builtins/src/lib.rs, its existing focused input-liveness/mono tests and mutation record, plus the numbered #238 spec. Existing host-core/builtins-compiler/graph tests may be run unchanged. No lane kernel rewrite, intrinsic/unsafe addition, extra cached coefficient copy, runtime dispatch, queue protocol, rack implementation change, allocation/layout optimization or general benchmark/test framework. A small private unit test can inspect the internal stage without introducing a new public evidence API.

Objective gates:

1. Prove mask correctness against the unchanged full comparison for scalar, W4 and W8, including a partial bank. Retarget a nonzero lane while other lanes have a mixture of true/false bits; exercise Left, Right and Both, trim and polarity, immediate and positive-window changes, re-equalization and repeated records on the same/different lanes. After each retarget the addressed bit must equal the definition, every unaddressed bit must retain its prior value and padding remains false. Include false-to-true as well as true-to-false; merely OR-ing the bit or overwriting the whole mask must fail.
2. Prove the actual reduced mechanism, not only PCM identity: one accepted retarget evaluates the full predicate for only the addressed active lane; restoring the old full-refresh call must contradict that claim. A bounded source observation plus a private test-local predicate-call count/disposable probe is sufficient; freeze the observation/assertion before the mutation, exclude debug reader-oracle calls from the measured interval, and require the same specific excess-comparison assertion for the restored-old-call control. Do not add production counters or claim unchanged-output tests detect the old extra work.
3. Preserve the existing full-refresh invalidation after dual ramps, collapsed settle/mirror and reset. Reuse builtins input_liveness/input_liveness_mono and host-core input_liveness_console proofs for admission-block asymmetry, retarget between collapse and disengage, re-equalization, symmetric rides and never-collapsed PCM/state comparison. Verify PCM and exposed retained state words with exact bits, including signed zero and existing recovery cases; do not replace bit equality with a tolerance. Keep #235's C-1/C-2 causal stale-cache controls. C-3 remains the documented equivalent mutation under total mirroring, not a required false red or permission to delete the refresh.
4. Focused debug and release checks must pass: debug retains the independent cache-definition assertion; release exercises actual held-bit behavior without that assertion. Existing public malformed-lane/parameter refusal stays unchanged. Preserve the scalar/banked operation order, recovery counters and collapse census. No new broad corpus is required when the existing liveness and collapse suites cover these semantics.
5. Apply proportional realtime/lane/resource/workspace and supported-target/current-artifact qualification under the immutable candidate workflow after source PASS, then actual-head Astra/required CI delivery. Resource/layout change is not intended or required by a bit-update-only repair; any new need should be reviewed rather than silently expanding scope.

This is class-A bookkeeping: the compared words and answers are unchanged and no rendered arithmetic is touched. A proposal to approximate comparisons, ignore signed-zero bits, change ramp convergence, alter recovery or relax collapse eligibility would move into class-B/correctness territory and must stop for an owner ruling. No projected gain is justified without measurement.

## Closure accounting and sequencing

Amend #238, preserve its historical link to #235, and add a reciprocal #349 note: stationary dispatch caching/short-circuit was delivered by #235; #238 owns only excess full-bank refresh per single-lane record; any separate ramp-block extraction work remains explicitly open/unbriefed. No duplicate owner or new implementation child is needed for this slice. If root wants separate block-refresh work, number and brief it independently rather than expanding #238; do not mark all RT-5 closed merely because the per-record slice ships.

Separate measurement from feature proof. #349's historical descriptive-measurement request can be retained in a named successor if root wants it, with a frozen genuine sustained-record workload after source acceptance. This review authorizes neither a timing run nor a historical extrapolation. #238's usable mechanism/identity repair should not wait for a new benchmark framework or broad research corpus.

After #442 delivery, root may choose this queued slice, sync the full amendment/local spec and request Astra numbered/frozen-base approval before Luna attempt 1. Sol retries only after FAIL, at most three attempts total. This report itself does not assign implementation or supersede the current sole feature.


## Root adoption

Root adopts the finite product contract, allowed paths, five objective gates, class-A limits, and closure accounting above. The inspected runtime source remains unchanged by tooling-only PR #449, delivered as `39da065507beb822ef70a1552ff5dcc363938dd4`, the base of this planning branch. Historical timing is retained solely as provenance; a current microseconds/block claim requires separately authorized measurement. Luna receives one implementation attempt only after Astra scope/base approval, Sol receives attempts 2/3 only after FAIL, and a third FAIL requires a preserved checkpoint and explicit rescope. Actual PR-head Astra PASS and required qualification CI precede merge and verified remote closure.

## Astra numbered scope approval

# Astra #238 numbered scope review — PASS

Planning checkpoint supplied by root: `d5ecad07ab184a106c190a29072b9c508288dc35`, `/home/bl/misofm/engine-238-plan`, on delivered main `39da065507beb822ef70a1552ff5dcc363938dd4`.

PASS for queued scope. Read the full numbered amendment against `/tmp/astra-349-rt5-current-scope.md`. GitHub issue #238 remains OPEN with its original exact title, “refresh_channel_symmetry is O(records×lanes) under sustained per-block record traffic”; its body matches the local spec. No duplicate issue identity or implementation owner is introduced.

The root priority ruling explicitly brings the previously deferred repair into the audit queue while preserving the historical post-launch wording as provenance. Root adoption makes the embedded recommendation binding. It keeps #442 as the sole runtime feature and requires Astra approval of the actual implementation base before Luna assignment. This scope approval does not authorize immediate implementation or timing.

The product remains exactly one lane's clear-and-set cached-bit update after a valid trim/polarity retarget. The unchanged comparison definition, all other mask bits, preparation/dual-ramp/collapsed-ramp/reset full refreshes, ramp/state/recovery arithmetic, LIVE/AGREEING/DESIGNED terms and collapse semantics are retained. The allowed builtins implementation/tests/mutation/spec paths and no-new-framework/no-public-probe/no-runtime-counter boundaries are faithful to the brief.

All five finite gates remain intact: scalar/W4/W8 and partial-bank exact masks with both bit transitions and unaddressed-lane preservation; an actual addressed-lane-only predicate evaluation proof with the restored old full-refresh call failing the same frozen excess-comparison assertion; existing exact PCM/state/collapse/admission proofs and C-1/C-2 controls; honest C-3 equivalence and retained full refresh; debug/release and proportional immutable delivery qualification. No approximate arithmetic, relaxed bit equality, generic SIMD optimizer or speculative resource change enters this issue.

Closure accounting is explicit: #235 already delivered stationary dispatch caching/short-circuit; #238 owns excess per-record cross-lane refresh only. Broader ramp-block extraction and descriptive sustained-record measurement remain retained audit obligations requiring their own bounded briefs. Historical timing is not promoted to a current estimate; #238 cannot close all RT-5/#349. No amendment is needed.

After #442 delivery and root selection of this queued slice, recheck the actual integrated source base before Luna attempt 1, particularly if builtins or its production caller paths changed. Preserve the one-Luna/two-Sol-retries maximum and actual-head Astra/required-CI/remote-closure workflow.

Read-only spec and remote identity/body review. No source/spec changes, Git/GitHub mutations, builds, tests or timing; only this `/tmp` review file was written. Supplied checkpoint/base identities were not independently re-queried with Git.

## Current implementation base and Luna attempt 1

# #238 current accepted-base readiness — PASS for scope; delivery-base freeze still required

Read full queued238 spec at engine-238-plan e5df49fc and compared relevant source with accepted PR482 head2546cc2992fcb56bed493b9acb6f48ccf6fb39af in engine-479-delivery. No implementation, tests, builds, timing or Git/GitHub mutations.

The smallest product remains valid without expansion: `InputStage::set_trim_signed` at builtins/lib.rs:1061–1095 still retargets one populated lane and calls full refresh. `refresh_channel_symmetry` at1036 still walks the entire maximum mask; `compute_lane_channel_symmetry` at1636 still defines exact trim/target/step bits, countdown and coefficient equality. Its definition and writer section are unchanged from the queued source. Preparation1027, dual-ramp1257, collapsed settle/mirror1340 and reset1520 still require the full refresh. Preserve the debug reader oracle at1580.

Delivered changes since the old planning base add fader/matrix settled pairing and scalar resource/binding machinery elsewhere in builtins/compiler/graph. They do not alter this input-stage writer/definition. The actual bank queue path still calls the same trim/polarity setters at builtins-compiler/lib.rs:424/431; scalar input processing reaches them at3541/3549. Broader compiler line numbers in the old brief are stale, not a new mechanism. Existing input_liveness.rs, input_liveness_mono.rs and host-core input_liveness_console.rs have no delta between the compared bases. Current scalar pairing must remain untouched.

Before assignment, root should append only the current delivered-base/sequence record: replace active “behind442” scheduling with “after482 is merged and actual main is frozen.” Preserve old442/base statements as historical records rather than implementation instructions. No new issue or substantive product amendment is necessary.

Exact implementation/test paths remain:
- crates/builtins/src/lib.rs: private addressed-lane clear-and-set update plus private cfg(test) correctness/predicate-call evidence. Update only comments that currently say the definition is called exclusively by full refresh/debug reader.
- crates/builtins/tests/input_liveness.rs and input_liveness_mono.rs: existing public correctness fixtures, modify only if needed to expose the frozen bit/state transitions.
- .github/ISSUE_SPECS/238-single-lane-channel-symmetry-refresh.md. Mutation commands/diff/output belong to this decision/evidence record. No new production probe API, counter, helper crate or harness.
- host-core/tests/input_liveness_console.rs is an unchanged execution gate, not an added implementation path.

Finite source gates remain exactly those already frozen: W1/W4/W8 including a partial bank, nonzero addressed member, mixed other bits, both bit transitions, Left/Right/Both, trim/polarity and immediate/ramped/repeated records. Count only the setter's predicate interval, outside debug reader/oracle assertions; the restored old full-refresh call must fail the SAME named excess-comparison assertion. One scalar lane is not by itself a discriminating old/full control—use a multi-member width for that control. Preserve padding/unaddressed bits and the complete refresh invalidation sites. Keep C-1/C-2 evidence and candid equivalent C-3; do not manufacture a C-3 failure.

Existing focused commands, each debug and --release: `cargo test --locked -p builtins --lib` (including the new private case), `cargo test --locked -p builtins --test input_liveness --test input_liveness_mono`, and `cargo test --locked -p host-core --test input_liveness_console`. Default host-core features suffice for its existing console fixture; no protocol enablement is required. Preserve actual nonempty names/counts, fmt and proportional strict affected Clippy/policies. After source PASS, root freezes the candidate and applies existing supported-target/artifact/resource/workspace/actual-PR/CI qualification; do not add a new matrix or assume artifacts remain byte-identical before checking.

No layout/resource change is intended. Broader RT5 ramp-block extraction and descriptive sustained-record measurement remain separate retained obligations; #238 closes neither all RT5 nor349. Historical timings remain provenance only. Root may assign Luna1 only after482 delivery, main integration, synchronization and a runtime-equivalence check; Sol2/3 follow failed verdicts with the standing hard stop.

Root adopts this current-base scope and replaces the old active behind442 sequencing with delivered PR482. Actual main is024ad674789a96390bcc45a754931ef5119c8b59; integrated planning head556324c5609bb48ecff08854cd9c43f433f9903f is byte-identical to that main for crates/hosts/tools/Cargo/.cargo/scripts, and those paths are also identical to Astra-reviewed2546cc29. GitHub238 retains its original number/title and the finite single-lane contract. Root assigns fresh Luna attempt1 after this checkpoint; pause at the first compiling/focused-green tranche for exact-path commit/push. No timing or broader ramp-block rewrite is authorized.

## Luna attempt 1 first recoverable checkpoint

The writer now clears/sets only the addressed cached bit, while preparation/ramp/reset full refreshes remain. A private test covers two transitions on nonzero members of partial W4/W8 banks, with predicate-count and unaddressed/padding assertions. `/tmp/238-luna1-debug-builtins-lib.log` executes3 passing library tests; its retained `.status` records status=pass/tests=3 and the exact locked command.

This is partial proof, not source acceptance. Remaining frozen W1/selector/trim-polarity/ramp/repeated-record cases, unchanged full comparison checks, same-assertion old-full-refresh control and complete debug/release/liveness/lint/policy gates remain. Root notes the process-global test predicate counter can receive calls from unrelated parallel tests and needs attribution confined to the measured setter interval; passing once does not establish isolation.

## Luna attempt 1 final source checkpoint, pending Astra verdict

The final tranche replaces the global predicate counter with cfg(test) thread-local observation and adds W1 and selector/ramp/polarity coverage. Retained /tmp/238-luna1-* logs and descriptive status files show debug/release library3, liveness13+8, host console10, formatting, affected Clippy and builtins/realtime policies passing.

Root inspection does not yet substantiate Luna's claim of the complete frozen proof: the added selector loop uses the private signed setter, does not compare the full predicate oracle after each operation, and no retained old-full-refresh mutation run was found in the reported evidence. These are explicitly pending adversarial assessment, not accepted or silently waived. Source is checkpointed before any further work; Astra supplies the consolidated attempt1 verdict. No timing was run.

## Astra Luna attempt 1 review and Sol attempt 2 assignment

# Astra #238 Luna1 verdict — FAIL

Exact ab4dd39f5f19d75745c723060aec17e4d69d2133, engine-238-plan. Read full numbered/current-base scope, cumulative source and retained descriptive command/status evidence. No tests/builds/timing or source/Git/GitHub writes. One coherent Luna verdict; root may assign Sol2, not further Luna changes.

Production change is the intended bounded clear/set update: it evaluates the unchanged predicate for the addressed lane, preserves other bits and leaves preparation/dual-ramp/collapsed-ramp/reset refreshes intact. The counter is now cfg(test), thread-local and measured outside debug-reader calls. No arithmetic, queue, layout or public-validation change is identified. Existing liveness/mono/host console debug/release results are useful retained regression evidence.

Two finite original proof groups remain:

1. Complete the frozen per-operation mask oracle in the existing private test. The initial retarget checks count/unaddressed bits/padding and an expected true bit; the selector/ramp loop then checks ONLY count, and the final polarity call checks one false bit. It never compares the entire resulting cache to the unchanged full predicate after EACH operation. Most operations invoke set_trim_signed directly rather than the actual trim/polarity paths; only one lane per width is addressed, with no repeated different-lane sequence. Keep W1 and partialW4/W8, add a full populated W4/W8 case if needed by the existing frozen full/partial matrix, and run a compact deterministic sequence through set_trim_db/set_polarity_invert covering Left/Right/Both, immediate/positive-window and both symmetry transitions. After each operation: capture its one-call interval BEFORE oracle reads, compare every active bit to the unchanged predicate, preserve all unaddressed prior bits, reject padding. Reuse existing public liveness cases for public validation/PCM/state; no new public probe or corpus. Explicitly exercise repeated same/different active lanes on the multi-member cases. Do not add an unbounded Cartesian matrix.

2. Execute the required actual old-full-refresh counter-control. No retained mutation diff/command/failing output/restored result demonstrates it. Freeze the SAME named excess-comparison assertion, temporarily replace only the setter's new single-lane refresh call with the original full-refresh call, and show that exact unmodified assertion fails for a multi-member case with excess comparisons. Generic compilation failure or an unrelated bit mismatch is not proof. Preserve actual diff/log/status and restored passing execution; no new production mutant beyond this one. W1 alone is not the discriminating control. Keep C-1/C-2 history and C-3's equivalent disposition; do not manufacture a C-3 red.

After those test-only completions, retain exact named debug/release results and affected liveness/host/fmt/Clippy/policy evidence. Existing reported status files are descriptive `status=pass`, not captured numeric exit files; report their authorship/form honestly rather than invent terminal values. No parent full workspace/artifact/timing before source acceptance. Full refresh after ramp/reset and broader RT5 residual/measurement separation remain unchanged. No further production repair is requested by this verdict.

Root adopts this consolidated FAIL and assigns Sol attempt2 for these two original proof groups only. Pause at a coherent green checkpoint; no further production algorithm changes or timing are authorized.

## Sol attempt 2 source and finite evidence

Source checkpoints0c20debe and70a6ff03 complete the compact per-operation oracle in the existing private test without changing accepted production code. The first case is partialW4 so the old-full-refresh mutation discriminates a multi-member bank. `/tmp/238-sol2-old-full-refresh-multimember.{diff,command,log,status}` retains the actual single-call mutation and numeric101 at the SAME assertion `one retarget evaluates only the addressed lane`, left8/right1. Restored exact execution has numeric0 in old-full-refresh-restored records.

Final debug/release builtins library, input liveness/mono and host console liveness, affected all-targets Clippy, fmt and builtins/realtime policy records are under /tmp/238-sol2-* with captured numeric0. Earlier formatting failure, W8 shift correction, zero-test filter and W1-first mutation are retained as failed or nondiscriminating iterations; they are not final evidence. Pending one consolidated Astra Sol2 verdict. No broader qualification or timing has been claimed.

## Astra Sol attempt 2 acceptance

# Astra #238 Sol attempt 2 — PASS

Reviewed exact pushed head 8bd081fb71634dba0e98d7d1691d46eecec8ded4 in engine-238-plan, source70a6ff03, full numbered contract and prior Luna1 FAIL. Read-only source/diff and retained evidence review; no tests, builds, timing or repository/Git/GitHub mutations performed.

Both finite correction groups are satisfied. The existing private test now uses the actual trim/polarity operations, captures the setter-only predicate interval before oracle calls, reconstructs the complete active mask with the unchanged predicate after EVERY operation, preserves every unaddressed bit and rejects padding. Its compact sequence covers all selectors, immediate/positive-window changes, repeated same/different members, and explicitly asserts both symmetry transitions. W1, partial/full W4 (3/4) and W8 (5/8) execute, with mixed initial masks and nonzero addressed members in multi-member cases. This is the frozen representative sequence, not an unclaimed full Cartesian matrix. Existing public liveness/mono/console fixtures retain the public validation, PCM/state, ramp and recovery regression role.

The retained old-full-refresh-multimember.diff changes only the production setter call back to refresh_channel_symmetry(). The exact named test executes once and fails status101 at the unchanged “one retarget evaluates only the addressed lane” assertion, left8/right1. Eight is the old maximum-mask walk's predicate-entry count (including padded positions), not eight active lanes in the partial-W4 fixture. Restored exact execution passes one test/status0. Earlier zero-test and W1-first controls are correctly not credited. Current source contains the restored single-lane call; Sol2 changes only the private test and issue record, preserving the accepted product implementation.

Inspected captured numeric0 final records: debug/release builtins library3, input liveness13 plus mono8, host console10; affected all-targets Clippy, fmt, builtins/realtime policies. Clippy's retained log includes warnings despite successful invocation; this review claims its actual exit0, not warning-free output. Earlier shift/format/control failures remain candidly retained. The unchanged definition, preparation, both ramp refresh sites, reset, queue/admission/arithmetic and production layout remain intact. The thread-local counter exists only under cfg(test).

Source PASS permits root to freeze and complete the existing proportional immutable delivery qualification: workspace, supported targets and artifact/current-consumer applicability, then exact-head actual PR review and required CI before merge/remote closure. No full qualification or timing is claimed here; no new benchmark authority or matrix is added. #238 repairs addressed-retarget bookkeeping only: broader RT5 ramp-block extraction and descriptive measurement remain separate, and #349 stays open.

Root integrates delivered main4587bfae (PR484 cfg(test)-only floor parity) before freezing immutable qualification. The sole add/add conflict in483 documentation retains its full accepted body plus verified closure record. Builtins source is unchanged from accepted70a6ff03.

## Shipped artifact qualification checkpoint

On immutable add3ca11, scalar/SIMD Wasm, ordinary release native C API, resource lifecycle and shared/static ABI gates return numeric0. Normal worklet build correctly rejects the previous a22f42fe pin and reports new da36c7503d9d4e1994cec6f22abd3fd97a41ede0551e3355adacdf9068bbead1 from the accepted production input-stage change. The actual failing command/log/status and expected/observed digests are retained under /tmp/engine-238-worklet-current.*.

Root records the observed digest as the new artifact pin on this separate delivery branch before rebuilding and running existing static/resource/browser qualification. Runtime source remains byte-identical to add3ca11; no builder flags or qualification gates change. Browser results are still pending and the prior matrix is not evidence for the new bytes.

## Immutable delivery qualification complete

Full workspace at add3ca11:276 result blocks,1628 passed,0 failed,24 ignored. Compared with the delivered479 test population there are exactly two additions (floor parity and addressed symmetry), no removals. Supported scalar/SIMD Wasm checks, native C API build/resource lifecycle/shared+static ABI, new-pin worklet build/static object checks, expected-resource26 negative controls, all three actual browser engines and generated-matrix check passed. Root environment/lane/unfused/workspace policies also passed.

All actual command/log/status records, initial pin rejection, test-population delta and source-equivalence record are in artifacts/issue238-addressed-symmetry. Browser-generated changes at719232c9 preserve all prior gates/version floors and update only candidate/digest. Actual PR-head Astra review and required qualification remain before merge; broader RT5 extraction and measurement are still retained separate obligations.
