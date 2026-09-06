# #514 — Skip browser admission-counter clearing on command-free render blocks

Queued independent host runtime slice, not implementation authorization. Inspected delivered0b8cf178 source in engine-dsp-test-hex (current changes are four test adapters/documentation only), original349 IO-7 and the after510 reconciliation. This is a real repeated render operation; no measured time or historical `(3T+E)` population is claimed without qualification.

## Actual remaining operation and ownership

`hosts/host-web/src/lib.rs:1260` clears the entire boxed `ReadyOwnership::in_flight` u32 array after EVERY successful render. The array counts records admitted since the last successful render (field573), is prepared as zero at2827, is consulted for capacity at2189, and has exactly one increment site after successful push at2203. Its length is prepared queue_count (2768); it is independent of whether any command arrived in this interval. A session with no current command traffic still incurs the whole write pass.

Original #349 IO7 remains OPEN. Local scope searches find #430/#444 discussing in_flight for admission/pairing, not owning the idle clear optimization. Read-only live open-title search found no dedicated idle-clear issue. #137 is the broad existing Web console ABI owner; its body and #203 meter-cost body contain no assignment for this counter clear. #444 concerns concurrent application-sample admission and must not be altered. Recommend a bounded child of #137, cross-referenced to349/IO7; keep broader137/140/444 open. This is host code, separate from511/478 and RT6 rack dispatch, but root still schedules runtime implementation deliberately rather than overlapping broad qualification.

## Minimum correction

Add one private `has_in_flight_commands: bool` to ReadyOwnership, initially false. At the existing successful push/count increment, set true immediately for every actually pushed record. Do not set it merely because a submission or decoded staging buffer is nonempty: solo coalescing can emit zero records. In the existing successful-render branch, clear the array ONLY if true, then set false. On render rejection preserve both counters and flag exactly as the current code preserves counters. An internal push failure after earlier successful pushes must leave true for that prefix, even though this impossible-after-preflight path returns an error; do not introduce false capacity credit.

Invariant: false implies every in_flight entry is zero. True means at least one admitted record has not yet been accounted as drained by a successful render. All admission/backpressure calculations, staging/solo changes, push order, queue drain order, command ack/application sample, status updates, leases and master-peak fold stay byte-for-byte structurally unchanged. This is not a sparse dirty-index list, queue protocol rewrite, lazy generation counter or new allocation. Retain dense clearing for intervals with admitted commands. It fixes the idle case only; do not claim cost proportional to the number of touched queues.

## Exact paths and resource boundary

Production only `hosts/host-web/src/lib.rs`: private field, initialization, existing successful push site, existing successful render site. Tests only `hosts/host-web/src/tests.rs`, with narrowly cfg(test) instrumentation in lib.rs if needed. Numbered spec/evidence otherwise. No graph/rack/protocol/CAPI production, JS transport, new feature/dependency, queue schema or allocator.

A boolean is not automatically free. The field may alter ReadyOwnership and thus AudioWorkletEngineHost layout, whose actual `size_of` is charged in the resource projection at2412. Independently establish the before/after size and exact affected existing report totals; preserve the automatic charge. Do not change queue allocation bytes or label an unmeasured padding assumption as proof. Any actual numeric expectation delta needs a bounded derived resource ruling; PCM expectations never change. No extra resource schema is needed.

## Finite discriminating proof

Add one compact actual-host test `idle_render_skips_admission_counter_clear_without_losing_queue_credit`, using existing console_host/effect_console_host, stage_command and feed_and_render:

- Fresh host and repeated successful idle renders: counter array remains zero; actual clear-site entries/cleared elements are zero. Use test-only instrumentation around the physical fill, not around the new predicate only.
- One and multiple accepted submissions before a render, including repeated same destination and a different destination: flag true and exact old counter values/capacity behavior. First successful render performs one full clear; next idle render performs none. Refill the queue to its exact existing capacity after render and require the existing typed refusal on one extra record.
- A rejected malformed/backpressure submission on an otherwise idle host must not create pending work. A refusal following accepted records must not erase existing pending work. Exercise the existing zero-emission solo case rather than treating all successful submissions as dirty.
- An actual early render refusal preserves pending counters/flag and no clear; the sticky failed state and subsequent WRONG_STATE preserve the pending ownership and clear nothing. A separate healthy host covers successful drain. Use the existing TimeOverflow seam, not a new production error injector.
- Compare PCM and command report/application samples against the existing expectations. Retain `command_ack_names_the_exact_application_sample`, `paired_fader_and_matrix_commands_share_the_acknowledged_application_sample`, `an_effect_parameter_command_names_the_exact_application_sample`, `command_flood_is_typed_backpressure_and_leaves_the_render_untouched`, `a_refused_solo_submission_leaves_the_console_untouched`, and `a_solo_that_changes_nothing_emits_nothing` as representative product regressions. No second queue oracle or new corpus.

ONE actual mutation restores unconditional filling at the production render site while retaining the SAME clear-site instrument and assertions. The exact test must fail at idle-clear count (not an unrelated assertion); save real diff/raw failure/status and restored success. Counts describe work avoided, not elapsed time. The existing shipped-Wasm render closure allocation/free gate remains required; no new native allocator framework.

Run the exact new test plus host-web lib suite in debug/release, strict affected Clippy/fmt/diff and existing realtime/workspace policy. Freeze exact test count/source identity in evidence. Existing native command-timeline identity and current scalar/SIMD Wasm builds apply at delivery. Any normal shipped artifact mismatch requires the usual observed-pin and existing current-consumer qualification, with unchanged PCM. No benchmark runner, listening campaign, broad target matrix or fresh performance threshold.

## Closure

The child closes when successful command-free render blocks demonstrably skip the dense admission reset and accepted command/ack/backpressure behavior remains exact, followed by normal actual-PR/required-CI delivery. IO7 can then be marked delivered for its unconditional-clear finding; it does not settle automation application IO5, concurrent admission444, master-meter cost203 or shared native console IO18. This is one bounded product change with a real render-path effect, not another formatting or harness-only issue.

Astra scope/base approval, Luna1, consolidated review, then Sol2/3 only if needed. No issue was created, no source was edited, and no tests/builds/timing or Git/GitHub mutations were performed during this read-only scoping.

## Numbered queue boundary

GitHub #514 is the matching child of #137 for #349 IO7 only; #137/#140/#444 remain open. Based on delivered maina6a59030 plus #512 closure record. Implementation is queued while #511 is active; no attempt is consumed. Astra must approve this actual numbered base, freeze the current layout measurement and evidence seams before Luna activation.

## Numbered Astra scope amendment

# #514 numbered scope/base review — finite amendment required before assignment

Inspected clean18bd58ba2c09466c3b2362a94128bfbf61e90c5e in engine-idle-admission-clear. Live514 is OPEN with matching title and exact local body. Delta from delivereda6a59030 is only512 closure and514 spec; the entire host-web crate, Cargo and .cargo inputs match the prior inspected0b8cf178 base. The product remains applicable and bounded. #511 stays active; this review does not authorize overlapping implementation.

The numbered scope faithfully carries the proposed dirty-flag outcome. Two inherited proof phrases in my draft need the following concrete applicability corrections before Luna1, not additional machinery.

1. **Render failure is sticky.** `render_failure_retains_ownership_and_silences` (tests.rs923) sets next_absolute_sample=u64::MAX and executes real render_next, obtaining RESULT_RENDER_REJECTED/STATE_FAILED; subsequent render returns WRONG_STATE until disposal. Reuse that exact TimeOverflow seam with a console host that first admits a record. Assert pending counters/flag and clear-site count remain unchanged, retained ready ownership survives, positive-zero silence/status/diagnostic remain exact, and subsequent WRONG_STATE also clears nothing. Do NOT reset private host state to fake public recovery. Delete the draft requirement “corrected inputs followed by successful render” for this failed host. A separate healthy host covers successful drain followed by renewed capacity. This preserves the real product contract rather than waiving it.

2. **Allocation safety is the existing shipped-Wasm closure proof.** scripts/check-web-audioworklet.sh217 explicitly documents no native audited allocator for this browser host. Its existing call-graph check of miso_engine_web_v1_render rejects allocator/deallocator/drop reachability. Freeze that normal current-artifact gate (and its existing controls) for delivery. Do not add a global allocator, native positive-counter framework or test-support dependency just to satisfy my imprecise “installed host render allocation/free proof” wording. Source proof remains one private boolean, no new heap owner, unchanged queues. Existing required consumer checks are retained.

## Concrete existing seams

Zero-emission: reuse `a_solo_that_changes_nothing_emits_nothing` (tests.rs4128). One-track console_host, settle an actual immediate mute, then stage_solo for the same track. Existing logic changes no effective mute and emits no record. Inspect the pending flag and actual clear-site count after that successful zero-emission submission and next render; success status alone does not imply pending records. Preserve its positive-zero PCM and solo state assertions.

Admission/capacity: existing stage_command, console_host/effect_console_host, feed_and_render, command_flood_is_typed_backpressure_and_leaves_the_render_untouched and named ack fixtures provide the actual queue seam. Set the flag at the single successful push/count site2203, not after the entire submission loop. No failure injector is needed to establish source ordering for an internal partial-push failure: each successful iteration must visibly set the flag before a later iteration can return.

Mechanism: use one cfg(test) thread-local pair of counters at the actual physical fill site (clear calls/cleared elements), reset outside render. Do not put them in ReadyOwnership or AudioWorkletEngineHost: that would contaminate the very layout observation. The one old-unconditional-fill mutation retains those counters and the SAME idle assertion. No new public helper, registry or allocator.

Layout: record `size_of` and `align_of` for ReadyOwnership, Option<ReadyOwnership>, AudioWorkletEngineHost in the existing private host test module on the untouched base, then on the candidate. A temporary test-only print in that module is sufficient for pre-edit evidence; it must not add instance fields and must be removed/restored before the production edit checkpoint. Capture exact source identity/status and do not call cfg(test) layout proof the shipped-Wasm layout. The existing host layout/resource test at tests.rs280–326 already compares fixed host-shell bytes and ready metadata. Extend it minimally if needed; project_buffers2412 computes host_shell_bytes from size_of::<AudioWorkletEngineHost>, subtracts options/status, then adds plane references. Therefore a real shell-size delta affects bridge_metadata_bytes, bridge_retained_bytes, and total retained; largest_bridge_allocation_bytes/largest_named_allocation_bytes change only if the changed shell or metadata row becomes/maximizes that existing max. Budget projection follows those totals. Queue arrays/payload and graph/source/effect estimates do not change from this flag.

At Wasm delivery, use the actual unchanged-resource oracle/current-consumer comparison to establish target-specific report deltas; no guessed native-to-Wasm padding conversion. If a numerical resource expectation changes, request the exact derived amendment before updating it. Preserve PCM and all unrelated rows. This uses existing static/resource/artifact tooling and requires no standalone layout target or new report field.

Retain the already named new test `idle_render_skips_admission_counter_clear_without_losing_queue_credit`, exact debug/release one-test gates, existing host lib suites and policies. After root appends/synchronizes these corrections, numbered scope is ready for fresh Luna1 at its eventual integrated implementation base. They resolve inherited fixture applicability only; no change to the actual dirty-flag product. No tests/builds/timing or repository/GitHub mutations performed in this review.

Root adopts these fixture applicability corrections. #514 remains queued, with no Luna attempt consumed, until its implementation slot and integrated-base review.

## Pre-edit native layout observation

Root ran one temporary test-only probe against production base58ef9b23 and restored the test file exactly (clean status). ReadyOwnership/Option sizes are1336 with alignment8, host size1864/alignment8; prepared_host(128) bridge metadata6859, retained29087 and largest bridge/named19238. Actual raw proof is in artifacts/issue514-native-layout-baseline. This is native cfg(test) evidence only; later implementation-base dependency changes require applicability review, and shipped-Wasm resource proof remains separate. #514 stays queued, no implementation attempt consumed.

## Independent implementation scheduling and actual-base approval

Root supersedes the earlier queued status: #511 remains the sole launch-critical resource implementation; #514 is independent bounded host work in its own worktree with disjoint source/test paths. Integrated artifact/current-consumer/PR qualification stays deferred until #511 delivery and base integration. Fresh Luna attempt1 is activated only for the frozen host correction and local proof.

# #514 actual implementation base — PASS

Approve fresh Luna attempt 1 for the already frozen #514 host-only correction at clean `6ba60c46ff5af6187d6a8ddf44190a8aced1793f` in `/home/bl/misofm/engine-idle-admission-clear`, after root records the scheduling amendment in the numbered issue. This is implementation readiness at the actual source base, not integrated delivery qualification. Root can supersede the earlier queue wording without changing the product or evidence contract.

Current inspection confirms:

- The #514 tree is clean. Its delta from `a6a59030` is the #512 closure record, #514 spec, and native baseline artifacts only. There are no production/test/dependency changes from the reviewed native baseline at `58ef9b23`.
- #514's allowed source paths are exactly `hosts/host-web/src/lib.rs` and `hosts/host-web/src/tests.rs`, plus its own spec/evidence. Active #511 work is confined to the six graph, graph-compiler and builtins-compiler source/test paths previously reviewed; its current worktree edits contain no host source. There is no source/test edit overlap.
- The repository's one-launch-critical-issue rule explicitly permits genuinely independent bounded work without overlapping edits or unreliable broad gates. Keeping #511 as the sole launch-critical resource issue while implementing this separate host idle-clear optimization in its own worktree is consistent with that rule. #514 does not depend on the new slot reservation to implement or discriminate its private dirty flag.
- The native pre-edit baseline remains applicable to #514's present source: ReadyOwnership and Option<ReadyOwnership> are 1,336 bytes/alignment 8, and AudioWorkletEngineHost is 1,864 bytes/alignment 8 in the recorded native debug cfg(test) run. Its hashes, exact probe reconstruction, restoration and one-test exit 0 were verified in `/tmp/astra-514-baseline-review.md`; this inspection finds no intervening source drift. Do not rerun the unchanged baseline merely because scheduling changed.

Preserve the exact small implementation: one private boolean, initialization, immediate marking at each successful push/count increment, and guarded clearing at the existing successful-render fill site. Preserve admission, ack/application sample, queue drain order, backpressure and retained ownership. Use only the existing private host test module and a cfg(test) thread-local pair of actual fill-site counters. No new test framework, allocator, public helper, instance instrumentation field, feature, dependency or error injector is authorized.

The existing frozen test and regressions remain mandatory. In particular, use the actual TimeOverflow seam: failed render retains the ready owner and pending counters/flag, produces the existing silence/status/diagnostic, and subsequent WRONG_STATE clears nothing. Do not fake recovery by resetting private state; a separate healthy host proves drain/refill. Preserve actual zero-emission solo coverage and the single unconditional-fill mutation against the same idle assertion. Keep native layout/resource comparison honest and obtain the existing bounded derived ruling before changing any numeric expectation.

Defer #514's integrated artifact/current-consumer/PR qualification until #511 is delivered and root integrates the resulting default branch. At that point assess dependency/source drift and distinguish any #511 resource delta from the host flag's size/projection delta. The pre-#511 resource observation is not an oracle for unrelated post-integration owners. Retain the shipped-Wasm render closure allocation/deallocation/drop gate and its controls, native command-timeline identity, scalar/SIMD Wasm builds, unchanged PCM and ordinary required CI. No native layout result substitutes for that target-specific evidence, and no issue closure or integrated PASS follows from this readiness decision.

Root owns spec/GitHub synchronization and exact-path checkpoints. The two worktrees must retain isolated source identities and output attribution; neither may qualify a candidate while its own sources are changing. Checkpoint each coherent green tranche before layering more implementation, as required. Shared delivery/artifact work stays serialized through root even while these independent source edits proceed.

No builds, tests, timing, repository edits, Git mutations or GitHub mutations were performed. Only this requested temporary decision report was written.
