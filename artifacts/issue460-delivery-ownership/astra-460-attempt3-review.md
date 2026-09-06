# Astra #460 final Sol attempt 3 review

**PASS for source acceptance and remaining frozen delivery qualification.** Exact reviewed head `aa06c5eb7a140c84d43b698c6031712f41f7a809`, `/home/bl/misofm/engine-140-ownership`. Reviewed the full numbered contract, final cancellation amendment, retained accepted implementation and four finite attempt-2 findings. This does not authorize host enablement, claim DSP application, close #140/#444, or replace actual-head PR review and required CI.

## Shared ownership and exact capacity

The actual automation adapter continues to instantiate the same generic Copy-payload core. CoreMessage and the retained ticket entry now freeze logical_count at publication; 0 and257 reject before ownership changes. Generic mark_progress enforces monotonic bounded progress, full finish requires the frozen count and current prefix, and CoreCompletion returns exact ticket/payload/count/progress. Automation delegates progress and completion to this state rather than maintaining a second independent render progress ledger. The independent u32 payload test exercises rejected count, partial progression, premature finish and complete return.

B includes resident and core-owned batches; unsupported heads remain whole and FIFO-blocking. Reservation release remains control-side terminal/cancellation reconciliation, not handoff. Original admission helpers and legacy behavior are unchanged. Directed automation identity tests exercise stale generation/serial/slot, regressing and overlong prefix, premature and duplicate finish, and stale collection without credit loss. The actual256-record batch now crosses admission, handoff, pending, completion and collection. Generic slot identity above u16 remains covered without imposing a product capacity cap.

Cancellation totals are wide and checked: cancellation_headroom validates sequence addition and total*256 before publication; publication accumulates exact u64 totals over the frozen bounded population rather than u16 overflow/u32 saturation. Source ownership bounds justify the postvalidation checked additions. The finite arithmetic boundary test replaces any need to allocate huge automation fixtures.

## Cancellation and reliable publication

The retained token/frontier/actual-sample barrier remains ordered after committed handoffs. No new old-generation handoff is admitted during cancellation. Render returns pending or queued old work through the terminal core and cannot acknowledge while old pending work remains exposed. Actual matched acknowledgement precedes control reconciliation and reliable publication. Control owns the event reservations and sole sequence cursor; copied tokens cannot release them, and terminal collection cannot bypass pending cancellation.

The new distinct-thread test uses two real endpoint owners and deterministic barriers for queued, handed-off, partial and final-completion-race cases. It holds the worker before acknowledgement, proves Pending there, then completes at the chosen actual sample and verifies no old ticket remains accessible. This is a bounded ordering test, not a general stress or application-cutoff guarantee.

Physical slot reuse is exercised so cancellation publication order must follow admission order instead of slot index. Existing event inspection covers request/revision/reason/count/sample and consecutive sequence; production supplies the original queue's actual generation (the test's generation assertion is nonzero rather than a separately pinned constant). Applied-only cancellation emits zero and the next real cancellation reuses the unchanged sequence/returned credits. A genuinely occupied reliable queue refuses further cancellation without changing outstanding/resident/cursor state. Existing headroom, partial counts and stale/repeated token tests remain applicable.

## Physical/realtime evidence and two causal controls

Preparation allocation instrumentation now independently sums successful actual allocation Layout sizes and observes the largest allocation, comparing both with the public resource report. It no longer relies only on recomputing the production formula. The report includes current core payload copies, metadata ledger and physical ring backing/header allocations; inline endpoint state is not counted as fictitious heap storage. Actual teardown on a separate thread supplies positive free liveness.

The measured windows retain successful begin/mark/finish/terminal zero-allocation/free checks and add actual render cancellation boundaries across the four separate-owner cases, with synchronization outside measured windows. These are representative bounded realtime proofs, not an exhaustive scheduler/interleaving claim. No production allocator framework or dependency was introduced.

The first retained Sol2 mutation releases ownership at handoff and fails the same real B-capacity assertion. The refined second mutation suppresses actual reserved event commit after a real acknowledgement; the unchanged event-presence/sequence test fails with QueueEmpty. Retained `/tmp/460-sol3-mutant-premature-publication.status` is101 and restored status0. This now proves the required publication-before-Complete claim rather than merely early-ack refusal. Preserve the earlier unsuccessful control's history without crediting it as this proof.

## Verification and limits

Read retained `/tmp/460-sol3-final-{protocol,fmt,diffcheck}.{log,status}`: all statuses0. The protocol log includes the new separate-owner allocation integration and existing integrations/doctests. Exact four-path delta is service, exports, integration fixture and numbered evidence. No reviewer builds, tests, timing, source edits or Git/GitHub mutations were performed.

Root may freeze the immutable integrated delivery candidate and execute the already required proportional workspace/target/artifact qualification, then obtain Astra review of the actual pushed PR and wait for required qualification SUCCESS before merge. This remains an opt-in ownership service: no controller/host is enabled, no actual Point/segment DSP application is claimed, and #140/#444 plus their audit obligations remain open.
