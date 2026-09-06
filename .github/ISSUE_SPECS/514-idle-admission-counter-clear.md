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
- An actual early render refusal preserves pending counters/flag and no clear; corrected inputs followed by successful render clears once. Use the existing host fixture failure seam, not a new production error injector.
- Compare PCM and command report/application samples against the existing expectations. Retain `command_ack_names_the_exact_application_sample`, `paired_fader_and_matrix_commands_share_the_acknowledged_application_sample`, `an_effect_parameter_command_names_the_exact_application_sample`, `command_flood_is_typed_backpressure_and_leaves_the_render_untouched`, `a_refused_solo_submission_leaves_the_console_untouched`, and `a_solo_that_changes_nothing_emits_nothing` as representative product regressions. No second queue oracle or new corpus.

ONE actual mutation restores unconditional filling at the production render site while retaining the SAME clear-site instrument and assertions. The exact test must fail at idle-clear count (not an unrelated assertion); save real diff/raw failure/status and restored success. Counts describe work avoided, not elapsed time. Installed host render allocation/free proof remains required; no new allocator framework.

Run the exact new test plus host-web lib suite in debug/release, strict affected Clippy/fmt/diff and existing realtime/workspace policy. Freeze exact test count/source identity in evidence. Existing native command-timeline identity and current scalar/SIMD Wasm builds apply at delivery. Any normal shipped artifact mismatch requires the usual observed-pin and existing current-consumer qualification, with unchanged PCM. No benchmark runner, listening campaign, broad target matrix or fresh performance threshold.

## Closure

The child closes when successful command-free render blocks demonstrably skip the dense admission reset and accepted command/ack/backpressure behavior remains exact, followed by normal actual-PR/required-CI delivery. IO7 can then be marked delivered for its unconditional-clear finding; it does not settle automation application IO5, concurrent admission444, master-meter cost203 or shared native console IO18. This is one bounded product change with a real render-path effect, not another formatting or harness-only issue.

Astra scope/base approval, Luna1, consolidated review, then Sol2/3 only if needed. No issue was created, no source was edited, and no tests/builds/timing or Git/GitHub mutations were performed during this read-only scoping.

## Numbered queue boundary

GitHub #514 is the matching child of #137 for #349 IO7 only; #137/#140/#444 remain open. Based on delivered maina6a59030 plus #512 closure record. Implementation is queued while #511 is active; no attempt is consumed. Astra must approve this actual numbered base, freeze the current layout measurement and evidence seams before Luna activation.
