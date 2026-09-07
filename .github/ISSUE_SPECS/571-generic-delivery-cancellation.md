# Add boundary cancellation to generic prepared delivery

Child of #444 and follow-on to delivered #460. This is the smallest independently closable prerequisite for typed concurrent builtin admission. It exposes the already-proven cancellation cutoff through the generic prepared-delivery core without enabling builtin PCM, a production host, graph binding, plan publication, or native bank/scalar pairing.

## Problem

`PreparedDelivery<P: Copy + Send + 'static>` already owns bounded publication, render progress, terminal return, serial identity, and credit reuse. Its generic public surface only supports complete application. Partial terminal completion is private, and boundary cancellation lives inside `AutomationDeliveryControl`/`AutomationDeliveryRender`. A future #444 builtin payload therefore cannot reuse the delivered cancellation state machine without copying a second ledger or reaching automation-specific code.

The child must expose typed-payload cancellation through the same generic ticket authority. It must preserve the central correctness question: can an acknowledgement ever precede a drop? The answer must remain no.

## Product contract

Extend the existing generic `PreparedDelivery<P>`, `DeliveryCoreControl<P>`, and `DeliveryCoreRender<P>` owners. Keep `try_publish(payload, logical_count)` and `CoreTicket`; logical counts remain `1..=256`.

Add a typed cancellation lifecycle with conventional exact Rust names equivalent to:

- control `begin_cancel` captures the exact last committed ticket serial, blocks later publication until reconciliation, and returns a typed token;
- render `cancel_boundary(first_sample)` acknowledges the captured frontier at a render boundary and produces terminal dispositions for work that will no longer apply;
- control `poll_cancel_boundary` completes only after the boundary acknowledgement and after every ticket through the captured frontier has one terminal disposition;
- the public terminal result distinguishes `Applied` from `Canceled`, reports the exact applied prefix and remaining logical-record count, and carries the observed/acknowledged sample.

The existing private partial-completion mechanism may be refactored into this public typed core contract. Reuse the current slot, ticket, serial, publication, and terminal queues. Do not introduce another ticket ledger, sequence authority, callback, heap job, or payload-specific validator.

Cancellation establishes a cutoff only. The generic core does not schedule payload application, promise that `begin()` claims a closed application epoch, define late application, publish protocol notifications, mutate a session revision, or commit a locate/plan swap. Those decisions remain in later #444 children.

## Required semantics

- A cancel request freezes the exact committed publication frontier. Publication after `begin_cancel` is refused until control has collected and reconciled the cancellation result.
- A completed ticket is `Applied`. Cancellation never relabels already-completed work.
- A partly applied ticket reports the exact applied prefix and cancels only its unapplied remainder. A never-applied ticket reports prefix zero and its full logical count as remaining.
- The render acknowledgement certifies that no work through the captured frontier can apply later through this endpoint.
- Control does not report cancellation complete until the boundary acknowledgement and all frontier terminal dispositions are present.
- Slot and publication credit remain unavailable until control consumes the disposition. Queue movement, render completion, or boundary acknowledgement alone cannot release them.
- Stale tokens, duplicate terminal operations, regressing/overlong prefixes, duplicate polling, and ticket/generation/serial overflow refuse without releasing ownership or credit.
- Cancellation transport capacity is prepared independently of ordinary publication capacity, so saturation of all `B` tickets cannot prevent the cancel request or acknowledgement.
- Destruction and retained heap reclamation occur only when quiescent and off render.

## Bounds and realtime rules

Preparation configures `B` outstanding tickets. Each payload has a validated logical count of at most 256. Fixed storage must cover all ticket states and terminal dispositions plus an independent cancel request/acknowledgement path. Checked arithmetic must account for every retained payload copy, queue header, capacity-plus-one backing allocation, pending array, and largest single allocation.

After preparation, publish, begin, progress, finish, cancellation, polling, and collection allocate and free nothing. Render-side work is bounded by configured capacities and the captured frontier, independent of producer refill. It takes no locks, performs no syscall or I/O, invokes no callback, publishes no `Arc`-backed event reservation, and performs no reclamation.

The automation adapter must continue to own automation validation, reservations, cancellation reason/event publication, and its sole reliable-event sequence. Refactor it to use the generic cancellation core without changing its externally observable enqueue, application-progress, reliable-event, or sequence behavior.

## Exact ownership

Allowed implementation paths:

- `crates/protocol/src/delivery.rs`
- `crates/protocol/src/lib.rs`
- `crates/protocol/tests/delivery_ownership.rs`
- this numbered issue spec and focused evidence beneath the existing documentation/evidence conventions

The existing automation adapter may be refactored inside `delivery.rs`. Do not edit controller, queue grammar/admission law, SPSC implementation, builtins, graph, engine render, hosts, C ABI, artifacts, workflow, or dependency files. If a required correction crosses that boundary, stop and amend/split the issue first.

## Objective gates

1. Use actual separately owned control/render endpoints and deterministic two-thread barriers around publication, cancel request, render boundary acknowledgement, and control collection. No sleeps or timing assertions.
2. Exercise cancellation with zero, partial, and full application racing the request. Prove exact frontier coverage, exact applied prefix/remaining count, and that completed work stays applied.
3. Fill all `B` ordinary ticket slots and prove cancellation remains publishable. Prove `B+1` ordinary publication is refused atomically and no credit is reused before control consumes the disposition.
4. Reject duplicate and stale tokens/terminals, invalid prefixes, and checked identity overflow without freeing or applying twice.
5. Keep the automation adapter's current cancellation reason, request/revision/generation identity, reliable event count/order/sequence, reservation release, and legacy controller/protocol behavior unchanged.
6. Prove exact resource reporting and configured-cap/overflow refusal. Run a positive allocator-liveness control and repeated generic render cancellation operations with zero allocations and zero frees.
7. Mutation checks must make the same focused assertions fail if cancellation completes before render acknowledgement or if ticket credit is released before control collection. Restore source and pass the original tests afterward.
8. Run focused protocol tests, protocol library tests, formatting/lint checks proportional to touched paths, workspace policy, and the repository's required qualification routing before delivery.

## Delivery workflow

Luna HIGH owns implementation attempt 1. Astra LOW performs adversarial exact-head review, including API scope, ownership, realtime behavior, resources, mutations, and unchanged automation behavior. A failed attempt may receive at most two bounded implementation revisions followed by fresh Astra LOW reviews; after attempt three, preserve evidence and rebrief rather than weakening gates.

Root commits each coherent exact-path tranche before more implementation is layered on it, pushes checkpoints promptly, keeps the GitHub issue and this spec synchronized, obtains required CI on the reviewed head, merges only after PASS, verifies post-main qualification and issue closure, then removes the clean delivered worktree.

## Parent accounting

This child earns reusable generic cancellation and closes no #444 PCM or pairing outcome. After delivery, #444 still requires a prepared Rust typed builtin batch endpoint with an explicit requested block sample/revision, one closed claim/cutoff and late-outcome contract, application through separate bank/scalar owners, and lifecycle publication decisions. Bank pairing and scalar pairing then follow under that endpoint contract. Existing raw concurrent producer endpoints retain their present separate behavior.

## Attempt record

Attempt 1 source checkpoint `1716c5fb6dce228ef969411d6f7bc5c757203317` was focused-green but received Astra LOW **FAIL**. Automation cancellation could publish unsupported/control-retained batches before the cancel request became visible, and the adapter lost a pre-dequeue identity-overflow preflight. Required generic concurrency, mutation, resource/overflow and repeated allocation evidence was also incomplete; strict Clippy found one finite lint. The full exact-head review is preserved in `docs/audits/571-attempt1-review.md`. Attempt 2 remains within the same exact three protocol paths and must correct those findings before a fresh Astra LOW review.

Attempt 2 repairs the failed head within the same boundary. Unpublished automation entries remain control-owned and become canceled terminal dispositions without entering the render data ring; checked serial-plus-queued preflights run before reliable reservation or dequeue; threaded generic and staged-automation barriers cover publication, request, acknowledgement, and collection; and the focused evidence records both temporary production mutation failures. The implementation remains uncommitted pending the root checkpoint and fresh Astra LOW review.

Astra LOW reviewed exact attempt-2 head `607352f80205fe88986a7e736bf2cecf6784dbde` and returned **FAIL** on evidence completeness, while accepting both source corrections. The staged-automation schedule could not detect the former race; generic threaded coverage omitted partial/full application; and direct generic resource/lifecycle/overflow/stale-token/physical-credit proof remained incomplete. The full review is `docs/audits/571-attempt2-review.md`. Attempt 3 is the final allowed attempt and must add only the missing discriminating gates/evidence inside the existing ownership boundary. A third FAIL triggers the hard stop and rebrief.
