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
