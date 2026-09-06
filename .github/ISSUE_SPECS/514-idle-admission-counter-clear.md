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

## Luna attempt 1 focused source checkpoint

The two authorized host files implement the private pending-command flag and physical-clear test counters with the frozen product test. Luna reported an initial successful filtered run but retained no raw capture and did not use the exact invocation; it receives no frozen-gate evidence credit. Root directly captured the required full-name --lib --exact debug command with contemporaneous source hashes, stdout/stderr and status0 (one selected test) before this checkpoint. Remaining release/full suites, actual mutation/restoration, candidate native layout and static gates are pending.

## Luna attempt 1 local proof

Source b0d9e087 plus the two-cast-only correction022e2f2b passes the captured frozen focused debug/release tests, full host suites63passed/1ignored each, corrected strict Clippy and static policies. The actual unconditional-fill mutant fails the same idle assertion at(3,9) versus(0,0); final exact restoration passes. The initial uncaptured filtered run and failed first restoration are explicitly excluded/preserved. Candidate native host/ReadyOwnership sizes and bridge totals grow8 bytes; largest allocations stay unchanged. No numeric expectations/pins were changed. Artifacts/issue514-luna-attempt1 holds raw evidence; consolidated Astra source review and post-#511 integrated qualification remain pending.

## Consolidated Luna attempt 1 FAIL and Sol attempt 2 brief

# #514 Luna attempt 1 — consolidated Astra review: FAIL

Reviewed clean `d22ea0177ad52c2a904e0b5f294c8db3604cd3e2` in `/home/bl/misofm/engine-idle-admission-clear`, including AGENTS.md, the complete numbered spec and its scheduling/base/fixture amendments, production/test diff, and both captured evidence packages. This is one consolidated source-qualification verdict. The implementation is sound on inspection, but the frozen discriminating test is incomplete in two bounded areas. Sol attempt 2 can correct those areas within the existing test; no production redesign or scope expansion is needed.

## Required corrections

1. **Finish the pending-prefix admission witness** (`hosts/host-web/src/tests.rs:1740–1939`). The initial three-record scenario is one successful submission, not multiple successful submissions before a render. More materially, the full-queue scenario asserts the pending flag before the extra-record backpressure refusal, then checks only `in_flight[0]` after it and abandons that host. The separate `refused` host checks flag preservation after INVALID_ARGUMENT, not after BACKPRESSURE with accepted records. Thus clearing the flag on that backpressure path could escape the new test, leaving stale nonzero counts after the following successful render and denying valid future admissions. Complete the already prescribed witness: accept at least two submissions in the same interval (repeated destination plus another destination), assert exact accumulated counts and the unchanged application sample/report, assert both pending flag and counters after the excess-record refusal, then successfully render that host and assert exactly one dense clear, false flag, all-zero counts and renewed capacity. Reuse the existing host/staging helpers; the extra staged record at `QUEUE_DEPTH` is not the record read by `submit_commands(1)`, which reads slot zero, so stage the intended extra command in slot zero for clarity. Preserve the subsequent idle-no-clear check. These are frozen admission cases, not a request for a second queue oracle or another mutation framework.

2. **Complete the explicitly amended pending-command failure assertions** (`hosts/host-web/src/tests.rs:1957–1989`). The new console host correctly admits a record, uses real TimeOverflow, checks retained ownership/counters/flag and proves WRONG_STATE re-entry. It does not seed nonzero output or assert STATE_FAILED, positive-zero output, or `web.render.rejected\t$\n`. Those assertions were explicitly required on this pending-command host by the numbered scope amendment. The old `render_failure_retains_ownership_and_silences` still passes, but uses a separate `ready_host` without the pending console command and cannot discriminate a failure path conditional on pending work. Copy its small nonzero-output setup and exact state/silence/diagnostic assertions into the new failure section; retain the existing counter/flag checks after both failure and re-entry. Do not simulate recovery or add an error injector.

Keep both corrections in the same named compact test. Its zero-emission solo case already checks false pending state and zero physical clears; the unchanged named solo regression supplies the positive-zero PCM/solo-state oracle. The existing named ack/paired/effect/flood regressions remain useful and passed. No replacement PCM oracle, corpus, native allocator, benchmark, or new production field is justified.

## Accepted implementation and evidence

- Production changes are confined to the approved private field, initialization, successful-push site and successful-render site in `lib.rs`, plus cfg(test) thread-local counters. False is initialized with the zero counter array. Each successful push increments its counter and immediately marks pending before the next iteration can fail. Zero emitted records do not mark pending. Only successful render with pending work enters the physical fill and then resets the flag. Render failure does neither. Admission/preflight, push order, ack construction, solo transaction handling, leases and meter folding remain structurally unchanged. No allocation or additional heap owner is introduced in production render.
- The clear instrumentation is attached to the actual physical fill. It is thread-local test state, not an instance field, so it does not inflate the native layout probe. Fresh repeated idle renders, active clear, subsequent idle skip, zero-emission solo, malformed idle refusal and sticky retained ownership have meaningful existing assertions.
- Independently checked manifest sizes/SHA-256 and exact tracked-file coverage: baseline 8 payload files plus manifest (9 tracked); candidate 75 payload files plus manifest (76 tracked). Normal captured source SHA-256 values agree with their Git blobs. Both layout probes reconstruct exactly as their recorded HEAD test bytes plus the preserved probe; production hashes and exact clean restorations match. Baseline source applicability remains unchanged through the pre-implementation base.
- Independently reconstructed the unconditional-fill mutant from checkpoint bytes and verified both recorded SHA-256 and Git blob identity, unchanged test bytes, real status 101, and the intended assertion failure at `tests.rs:1725`: `(3, 9)` versus `(0, 0)`. Also reconstructed the malformed first restoration, which inserted the condition in the Observe arm, and verified its recorded identities and syntax-error status 101. Final `restored2` exactly matches checkpoint source and passes the one exact test. These are real captured outcomes, not inferred mutations. The uncaptured initial filtered run gets no frozen-gate credit.
- Captured focused debug/release runs select one test each (63 filtered); both full host suites report 63 passed/1 ignored. Every named required ack/effect/pair/flood/solo regression, plus the original failure-retention fixture, is explicitly present and passed in both full-suite outputs. Full/release evidence is from `b0d9e087`; `022e2f2b` changes only two redundant `u32` casts in the new test. Corrected exact debug and strict affected Clippy pass with final source bytes, and that cast-only difference does not invalidate the earlier suite evidence. Initial Clippy failure is preserved candidly. Captured fmt, diff, realtime/workspace policy and their existing controls pass.
- Native cfg(test) layout is ReadyOwnership/Option 1336 → 1344 and host 1864 → 1872, all alignment 8. Captured prepared-host bridge metadata is 6859 → 6867 and retained 29087 → 29095; largest bridge/named allocations remain 19238. `project_buffers` charges actual host size automatically. Unchanged `projected_retained_bytes`/`exact_retained_bytes` each include the bridge total once, so aggregate retained projections increase by the same 8 bytes; queue payload and other engine owners do not change. No numeric expectation or artifact pin was edited. These observations make no shipped-Wasm padding or timing claim.

## Next boundary

Root should record this single FAIL and assign the bounded test correction to Sol attempt 2 under the requested Astra → Luna → Sol workflow. Capture the affected frozen test/suite/static gates against the amended test identity and preserve prior evidence. The accepted baseline/layout observation need not be repeated for test-only assertions; keep the existing mutation/instrument/idle assertion intact and assess its applicability explicitly rather than inventing more negative-control work.

After source qualification, integrated artifacts/current consumers/native command-timeline identity/scalar and SIMD Wasm builds/the shipped render closure allocation-free gate and controls/actual PR qualification still wait for #511 delivery and default-branch integration with drift review. This verdict neither authorizes issue closure nor claims integrated delivery. Root retains all repository checkpoints and GitHub synchronization.

No builds, tests, benchmarks/timing, source edits, Git mutations or GitHub operations were performed during this review. Read-only inspection/hash reconstruction was used; the only file written is this requested `/tmp` report.

Root activates Sol attempt2 for only these bounded corrections inside the existing frozen host test. Production source, instrumentation, initial idle assertion, baseline/layout and actual accepted mutation remain unchanged. No new mutation or layout run is required merely for later test assertions; reassess applicability from the exact diff. Integrated delivery remains deferred until #511 is delivered and integrated.

## Sol attempt 2 focused checkpoint

The existing frozen test now covers repeated accepted submissions, pending flag and counter preservation through backpressure followed by drain/refill, and pending-command failed-state/positive-zero silence/exact diagnostic assertions. Only hosts/host-web/src/tests.rs changed. The captured exact debug test passes one test with contemporaneous source hashes verified by root; pre-checkpoint fmt/diff pass. Production, instrumentation, initial idle assertion and native layout are unchanged. Remaining release/full/static gates and consolidated review are pending.

## Sol attempt 2 local evidence

Sourceb12fbc3a passes frozenexactdebug/release1each, fullhost63passed/1ignored each, strictClippy/fmt/diff andexistingpolicycontrols. Multipleacceptedsubmission/capacityack andpendingfailure state/silence/diagnostic proofs are complete in the existingtest. Artifacts/issue514-sol-attempt2 preserves11capturedcommands andpriorconsolidatedFAIL. Production/initialidleassertion/layout remainunchanged; accepted prioractualmutation/layout apply. ConsolidatedAstra review andpost-#511 integrateddelivery arepending.

## Consolidated Astra Sol attempt 2 PASS

# #514 Sol attempt 2 — consolidated Astra review: PASS

Source qualification passes at clean `cf110abb209fcab58cbf2e1ed97b14486c00c44d` in `/home/bl/misofm/engine-idle-admission-clear`, with implementation/test source checkpoint `b12fbc3a54660c4f447792445ede3383d3d9b517`. Both blocking findings from the consolidated Luna attempt-1 review are resolved. No additional source correction is required. This is source acceptance only; integrated delivery remains deferred.

## Frozen contract and corrected witnesses

The attempt-2 source diff changes only later staging and assertions in the existing `idle_render_skips_admission_counter_clear_without_losing_queue_credit` test. Production `lib.rs`, its physical-fill instrumentation, the initial idle assertion, all other tests, dependencies and resource expectations remain unchanged.

The admission witness now makes two successful submissions before one render: two records to the matrix destination, then one record to the fader destination. It compares complete command reports against explicit expected fields and the shared application sample `3 * QUANTUM`, with exact intermediate counts `[2, 0, 0]` and accumulated counts `[2, 1, 0]`, and pending true throughout. Successful drain proves one full physical clear and zero counters/false pending; subsequent idle rendering adds no clear.

The full-queue witness now stages the excess record in the slot actually read, requires the complete typed BACKPRESSURE report with zero admitted records, and directly asserts the pending flag and full counters remain intact. That same host renders successfully, records `(1, 3)` physical-clear work, resets pending and zeros every counter. Reusing the retained matrix staging then successfully admits the full capacity at the next exact application sample, drains once again, and leaves the following idle render at `(0, 0)` clears. This closes the prior false-negative: a backpressure refusal that erased pending work would now fail directly and could not hide behind an abandoned host.

The real pending-command TimeOverflow witness now seeds output with `-1.0` before rejection and requires STATE_FAILED, bitwise positive-zero output and exact `web.render.rejected\t$\n`. It retains the ready owner, exact counters, pending flag and zero clear work. WRONG_STATE re-entry preserves failed state, diagnostic and pending ownership without clearing. No fake recovery or production error injector was added.

The previously accepted production invariant remains intact: initialize false beside the zero array; mark immediately after every successful push/count increment; clear and reset only following successful render with pending records; preserve ownership on failure. Zero-emission solo, idle malformed/backpressure rejection, and malformed rejection after accepted work remain covered. The unchanged named PCM/ack/fader-matrix/effect/flood/solo regressions supplement the mechanism assertions and pass in both full suites. Admission and ack construction, queue order, solo handling, meter/lease logic and production realtime operations have no further changes.

## Independently verified evidence

- Recomputed all manifest byte counts and SHA-256 values and compared exact tracked coverage: baseline 8 payload plus manifest (9 files), Luna attempt 1 75 plus manifest (76 files), Sol attempt 2 47 plus manifest (48 files). Both prior packages are byte-identical to the previously reviewed commit.
- All 11 Sol captures have status 0 and identify the expected worktree and final production/test SHA-256 and Git blob identities. The focused debug capture records the pre-checkpoint dirty test at HEAD `a4d16dd3`; its captured bytes exactly equal the eventual committed test. The other ten captures record clean `b12fbc3a`. Current source equals those captured bytes. The inherited `514-luna1-` filename prefix does not obscure the explicit `sol2` labels and report attribution.
- Exact focused debug/release each selected one test, with 63 filtered. Full host debug/release each report 63 passed, 1 ignored, no failures. The new test and every frozen named regression are explicitly present and passed in both outputs. Strict affected Clippy, fmt, diff, realtime/workspace policy and their existing controls pass. Directed-fault stderr in the workspace control script is consistent with its passing negative-control output, not a hidden candidate failure.
- Test SHA-256 is `d35a91c02b8b020997348583be0846d5270f70397250487c0614bba18650cf91`, blob `ea9b160972cb9be5bc2b169162ac857e00461abf`. Production remains SHA-256 `aaea4b3c344e3f948fe965e1e8ed0c8394b5fb3843c2a50974890e7be9391924`, blob `0400e53fab15ba4066057432cba11928431efe2a`.

## Prior-proof applicability and delivery boundary

The prior actual unconditional-fill mutant, physical counter instrumentation and original initial `(0, 0)` assertion are unchanged. The accepted recorded failure at `(3, 9)` still discriminates the same production regression before any attempt-2 edit executes. Its recorded failure, malformed first restoration and exact successful final restoration remain preserved. There is no need to repeat this accepted mutation merely for later assertions.

No production type, layout or resource calculation changed in attempt 2. The accepted native cfg(test) baseline/candidate observations therefore remain applicable: ReadyOwnership/Option 1336 → 1344, host 1864 → 1872 (all align 8), bridge metadata 6859 → 6867, bridge retained 29087 → 29095, largest bridge/named 19238 unchanged. Aggregate retained projections include the bridge delta once. This does not establish shipped-Wasm layout or a timing improvement, and no numeric pins were changed.

Root may record this source PASS. Keep #514 open until #511 lands, the default branch is integrated and source/dependency drift is reviewed. The normal integrated current-artifact/current-consumer checks, native command-timeline identity, scalar/SIMD Wasm builds, shipped render-closure allocation/deallocation/drop gate and its controls, unchanged PCM, actual PR/required CI and remote issue synchronization remain delivery requirements. Root owns all Git/GitHub actions.

No builds, tests, benchmarks/timing, source edits, Git mutations or GitHub operations were performed during this review. Inspection and hash reconstruction were read-only; the only file written is this requested `/tmp` report.


## Delivered prerequisite integrated — ready for qualification

Main107b9ed1 (#511/PR515) merged without production conflicts at5ce91031, followed by its remote closure record e789c777. The sole #512 documentation conflict retained its post-main SUCCESS paragraph. Both accepted #514 source hashes are unchanged.

# #514 integrated-base drift review — PASS to qualify

Approve the existing immutable and current-artifact delivery route on clean `e789c777bc45590e252cb75678c6e055468dc263` in `/home/bl/misofm/engine-idle-admission-clear`, following integration merge `5ce91031`. This is integrated-base readiness, not a claim that integrated gates or CI have passed. Sol attempt-2 source acceptance remains valid; no new implementation attempt is consumed or authorized.

## Drift assessed

Delivered main `107b9ed1803b8313e434821ec5bf49a178b6bb2f` is an ancestor. Relative to accepted #514 checkpoint `841e7125e97b6c8df1339fc4e3d22ca60feae091`, the 12 changed dependency/delivery paths are byte-identical to that main. They comprise the delivered #511 graph/builtins/compiler reservation and physical-capacity preparation, its feature-gated test support, C API resource-test correction, and qualified browser resource/artifact/matrix records. Cargo.lock, workspace Cargo/.cargo, qualification workflow, gate scripts and AGENTS.md have no intervening change. Relative to delivered main, the only production/test differences are the two already accepted #514 host files.

Both accepted host identities match exactly:

- `hosts/host-web/src/lib.rs`: SHA-256 `aaea4b3c344e3f948fe965e1e8ed0c8394b5fb3843c2a50974890e7be9391924`.
- `hosts/host-web/src/tests.rs`: SHA-256 `d35a91c02b8b020997348583be0846d5270f70397250487c0614bba18650cf91`.

The #514 spec and three accepted evidence packages have no drift from `841e7125`. The #512 conflict resolution exactly matches delivered main, including its post-main success paragraph. The final cherry-pick adds #511's closure record, not another implementation change.

The #511 changes are off-render preparation/resource-accounting changes: conservative runtime bank-slot reservation enters graph estimates and caps, while `bank_chain` explicitly prepares its existing slot population at requested capacity. They do not modify host admission, command queues/acks, drain order, the #514 pending flag, physical clear instrument or failure seam. Their graph-report effects are independent of the host shell charge. The inherited browser expectation changes are exactly the three already qualified +204 graph rows (session-plus-plan 29294 → 29498, incremental 29294 → 29498, metadata 3455 → 3659); no #514 bridge-row repin has occurred.

The current inherited worklet pin is `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`, the #511 artifact identity, not evidence about the module that will be built from this integrated host change. The accepted pre-integration native +8-byte host observation remains historical native evidence. It does not establish a Wasm padding delta or freeze post-integration totals that now include #511 owners. Keep graph reservation and host-shell effects separately attributable.

## Smallest existing delivery route

1. Freeze the integrated source identity and retain contemporaneous commands, relevant environment, stdout/stderr and exit status. Run the already frozen proportional host/static checks and existing immutable workspace/supported-target/native sequence: locked workspace tests; scalar Wasm checks for target-smoke, protocol, host-web and builtins-compiler; SIMD target-smoke/protocol checks; release native C API build and its existing ABI check. Use issue-specific output/target paths rather than another worktree's captures. The ordinary worklet builder below supplies the real release SIMD host build. Confirm the existing `native_command_timeline_digest_pins_the_wasm_parity` actually executes and passes in the integrated host/workspace evidence; preserve its native/wasm digest expectations. No fresh digest-generation mode is needed.
2. Invoke unchanged `bash scripts/build-web-audioworklet.sh EMPTY_OUTPUT_DIRECTORY` normally, with the inherited pin and no REPIN bypass. The script requires an existing empty nonsymlink output directory and refuses overwrite. If it reports a digest mismatch, preserve its exact expected/observed values and terminal failure for a separate bounded artifact ruling. This review authorizes no guessed digest, numeric repin, alternate build recipe or publication of unchecked module bytes.
3. Once an ordinary verified builder succeeds, use that identified published artifact directory for existing consumers: `bash scripts/check-web-audioworklet.sh ARTIFACTS`; `python3 -B scripts/check-browser-expected-resources.py --artifacts ARTIFACTS`; `bash scripts/test-web-audioworklet.sh`; then the pinned qualification npm install and all-three-browser `npm run qualify -- --artifacts ARTIFACTS --browser all --record-matrix --candidate-commit ACCEPTED_SOURCE --self-test-mutations`, followed by `npm run matrix -- --check`. This is the existing delivery path: static shipped render closure allocation/deallocation/drop and trap checks, their hermetic controls, resource/native-row and PCM/direct-oracle checks, and current browser qualification. It does not require a new harness, full historical preflight, benchmark, listening campaign or target matrix.
4. If any ordinary consumer exposes an actual numerical discrepancy, preserve its raw failure and the complete observed oracle/resource values, module identity and expected-document source. Request a separate ruling deriving the exact affected rows from this integrated target and existing ownership formulas. Preserve PCM and all unrelated rows. The native +8 observation supplies no speculative Wasm bridge delta. Likewise, #511's earlier +204 graph ruling cannot authorize additional graph changes. After a bounded correction, rerun the affected existing gates with honest source attribution; do not silently repin until green.

Keep tracked sources immutable while any qualification job using them is active; collect terminal status before pin/spec/test edits or browser-generated record writes. Generated candidate/module matrix records belong to their normal final checkpoint, once the actual consumers have passed. Reuse a qualified artifact for remaining consumers when its inputs and bytes remain unchanged. Retain prior actual mutation/layout captures: the host code, instrument and initial idle assertion are unchanged, so integration does not justify repeating those local mechanisms merely for ceremony. New integration failures must be assessed on their evidence.

## Remaining acceptance boundary

The supplied #511 delivery record states PR #515 merged after required run `34023823998` succeeded, and explicitly leaves post-main run `34024175768` running. This read-only review does not convert that pending run into success or independently certify live GitHub state. Root retains CI tracking and all Git/GitHub synchronization.

#514 remains open until terminal integrated evidence, final actual-PR review, required CI and normal delivery synchronization. This PASS permits executing the established qualification route; it is not a final artifact/PR/delivery PASS or a blanket permission to change expectations.

No builds, tests, benchmarks/timing, source edits, Git mutations or GitHub operations were performed. Only read-only source/history inspection and hash comparisons were used; the sole output written is this requested `/tmp` report.
