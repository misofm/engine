# Publish terminal shutdown for the prepared builtin batch endpoint

GitHub: #594 (https://github.com/misofm/engine/issues/594)

Bounded final child of #444 and audit lane A #559 on delivered main
`e16cea23e05a69c00f9d2826670c95514b4c259d`. #571/#572 delivered the generic
boundary-cancellation ledger, #576/#579/#580 delivered the prepared fixed-revision Rust builtin
batch endpoint through separate owners, and #587/PR #592 enabled its existing qualified native bank
and scalar pair implementations. This issue owns one remaining lifecycle operation: a terminal
shutdown publication that permanently closes admission, obtains a matched render-boundary
acknowledgement, reconciles every accepted ticket, and only then reports shutdown complete.

Luna HIGH or XHIGH implements each attempt. Astra LOW performs every scope, source, artifact, and
exact-head verification. The Sol HIGH coordinator owns the brief, exact-path checkpoints, GitHub
synchronization, and delivery. Lane B owns AudioWorklet artifact qualification and pinning if the
source changes the shipped six-file artifact.

## Smallest closable outcome

Add explicit terminal shutdown to the already prepared `BuiltinBatchControl` and
`StartedBuiltinBatchRender` owners. `begin_shutdown` uses the existing generic cancellation
message, frontier, acknowledgement, ticket ledger, and reserved terminal credits. Once accepted it
permanently refuses later batch publication. At the next valid render boundary, before builtin
injection or graph execution, the render owner acknowledges the shutdown cancellation and becomes
permanently quiescent. Applied tickets remain Applied; claimed-future and queued tickets become
Canceled exactly once through the existing terminal authority.

Control-side shutdown completion stays pending until both conditions hold: the matching render
acknowledgement was observed and every accepted ticket through the frozen frontier was collected.
The one returned shutdown completion carries the exact existing cancellation token, frontier, and
acknowledged sample. `stop()` remains the sole operation that transfers the coupled plan, delivery,
outcome, raw-control, pending, and fault storage for control-thread reclamation; shutdown completion
certifies quiescence and terminal reconciliation, not reclamation.

This explicit fixed-revision Rust endpoint outcome completes #444's adopted bounded lifecycle
requirement and may close #444/RT4 after delivery. Whole-platform plan replacement, locate,
revision publication, provider loss, C ABI, browser activation, and automatic host scheduling remain
unsupported future scope. The existing engine `PlanPublisher` cannot publish this endpoint as a
plain plan because the endpoint's private delivery and console owners must remain coupled.

## Frozen contract

Add public endpoint-local shutdown vocabulary with stable unversioned Rust names. The control API
must distinguish reusable cancellation from terminal shutdown. `begin_shutdown` succeeds at most
once, returns or retains the existing opaque `CoreCancelToken`, and permanently changes admission to
a typed refusal before publishing the boundary message. If generic cancellation is already pending,
shutdown refuses without changing that cancellation; if terminal shutdown has begun or completed,
duplicate begin calls return the specified terminal error and cannot create another boundary message
or completion.

`poll_shutdown` validates the exact token. Repeated correctly tokened polls before render
acknowledgement return Pending. After acknowledgement but while any accepted terminal is uncollected,
repeated correctly tokened polls remain Pending. Only after exact terminal collection does one poll
return one completion containing the matched token, frozen frontier, and acknowledged sample; later
polls fail as stale. Completion is not inferred from `outstanding == 0` alone and cannot precede the
generic acknowledgement. Ordinary `begin_cancel`/`poll_cancel_boundary` retains its existing
reusable behavior and reopens admission after its matched completion.

The render owner recognizes terminal intent using bounded prepared state; it performs no control-side
publication. The acknowledging render call returns a typed shutdown/cancellation-only report without
touching caller PCM, advancing the plan clock, draining raw builtin queues, or executing graph/DSP.
Every later render call remains quiescent and returns the same stable terminal status without another
acknowledgement, terminal, allocation, free, lock, syscall, or data-dependent loop. Invalid shape or
sample calls before the acknowledgement retain the existing validation and cannot fabricate shutdown
completion. The acknowledged boundary sample is the exact valid `first` supplied to the accepting
call.

A pre-existing sticky endpoint fault cannot be converted into successful shutdown or cancellation.
This child preserves the existing fault and ownership contract and refuses shutdown completion after
any irreversible application uncertainty. Do not add a fault recovery protocol.

## Exact ownership

Allowed implementation paths:

- `crates/host-core/src/builtin_batch_endpoint.rs`
- `crates/host-core/tests/builtin_batch_endpoint.rs`
- this numbered spec
- focused implementation and review records under `docs/audits/`
- #444/#559/#560 handoff specs for concise synchronized status

Do not edit protocol, engine, graph, builtins, builtins-compiler, other host-core modules,
`crates/host-core/Cargo.toml`, `Cargo.lock`, hosts, C ABI, browser, SDK, artifacts, policies,
workflows, or lane B #593 paths. Reuse the existing generic cancellation storage. One prepared shared
`Arc<AtomicU8>` lifecycle handshake is authorized because the generic cancellation payload
intentionally carries only its token and frontier. Allocate it during endpoint preparation, clone it
once into the control/render owners, and charge its exact allocation plus both inline handles through
the existing endpoint resource report before host preparation. Prove checked overflow, exact-cap
acceptance, one-below-cap refusal before any allocation, positive lifetime accounting, and off-render
final reclamation.

Freeze four internal states: `Idle`, `OrdinaryPending`, `ShutdownPending`, and
`ShutdownAcknowledged`. Control `begin_cancel` changes `Idle -> OrdinaryPending` before publishing
the generic cancellation message; `begin_shutdown` changes `Idle -> ShutdownPending`. If generic
`begin_cancel` fails for any reason, the initiating method restores `Idle` and every control lifecycle
field before returning the error. Use acquire/release ordering for every state transition.

After `cancel_boundary(first)` succeeds, render must classify that exact boundary before returning:
`OrdinaryPending -> Idle` returns the existing reusable cancellation-only report, while
`ShutdownPending -> ShutdownAcknowledged` returns the terminal shutdown report and permanently
quiesces this render owner. Any other state is an endpoint lifecycle fault. Control may observe the
generic acknowledgement before render completes this transition, so ordinary
`poll_cancel_boundary` must retain its cached generic completion and return Pending until it observes
`Idle`; it cannot finalize/reopen admission earlier. Therefore a new shutdown begin cannot overtake an
ordinary boundary whose generic acknowledgement has been published but whose endpoint classification
is still in progress. `poll_shutdown` likewise requires `ShutdownAcknowledged` in addition to its
matched cached generic completion and exact ticket reconciliation.

The handshake stays `ShutdownAcknowledged` after successful shutdown. Observing any state without a
successfully consumed generic cancellation message can never quiesce render. No new queue, ledger,
other heap allocation, protocol payload, manifest dependency, target-handle mapping, generation
clear, plan exchange, or public raw producer is authorized. If this four-state handshake is
insufficient, stop and rebrief rather than widening this child.

Direct red mutations may temporarily change one named shutdown predicate/order in the endpoint
module, run its focused discriminator, and restore the source. Mutations are never staged or
committed.

## Objective gates

1. Use deterministic control/render rendezvous around the actual generic cancellation publication
   and acknowledgement. Prove shutdown cannot complete before the render boundary; after the exact
   acknowledgement but before final terminal collection it still cannot complete; after collection
   it returns the matching token, frontier, and acknowledged sample exactly once.
   Add a separate rendezvous after an ordinary generic acknowledgement is published but before
   endpoint lifecycle classification: a concurrent shutdown begin must refuse until the ordinary
   render call reaches `Idle`, then its own message must be consumed and acknowledged at the next
   boundary without stranding or attaching terminal intent to the prior cancellation.
2. Exercise applied-but-uncollected, claimed-future, and queued tickets together. Prove exact Applied
   versus Canceled dispositions, record counts, prefixes, requested/application/acknowledged samples,
   collection order, outstanding counts, credit reuse, and no duplicate terminal. Explicitly answer
   the standing question: an acknowledgement cannot precede a later drop.
3. Cover empty and already-collected endpoints, stale tokens, duplicate begin/poll, shutdown while
   ordinary cancellation is pending, ordinary cancellation followed by resumed admission, and
   permanent admission refusal after shutdown acceptance and completion. Rejected shutdown calls
   leave the active lifecycle unchanged.
4. Through public native paired-bank and private forced-scalar preparations, render nontrivial PCM
   before shutdown and capture addressed state, plan time, output bits, pair process/member counters,
   and raw record drains. The acknowledging call and repeated later calls leave PCM bits, state, time,
   drains, and process counters unchanged; no graph report is produced.
5. Preserve the complete #576/#579/#580/#587 endpoint suite: admission/cutoff/late behavior,
   cancellation, sticky fault, resource caps, allocator lifetime, selected bank/scalar pairing,
   PostFader observations, exact decline behavior, and bitwise reference equivalence.
6. Run the positive allocation/free audit plus exact/one-below resource-cap preparation and repeated
   shutdown acknowledgement and terminal quiescent calls with zero allocations/frees. Stop only
   after render quiescence; prove the returned owner keeps all coupled storage alive until off-render
   drop and that teardown releases it there.
7. Run and restore five behavioral red mutations: publish completion before render acknowledgement;
   publish before final ticket collection; finalize ordinary cancellation before lifecycle
   classification; reopen admission after shutdown; permit graph execution after acknowledgement.
   Each must fail the assertion that directly proves the altered behavior, and the final worktree
   must be clean.
8. Pass focused host-core tests in debug and release, strict Clippy and rustdoc, formatting and diff
   checks, workspace/host/realtime/CI-routing policies, native x86-64-v3 compilation, and Wasm
   scalar/simd128 compilation. Record exact commands, statuses, mutation output, and restored-tree
   proof.

## Realtime and correctness invariants

The callback remains allocation/free, lock, syscall, I/O, logging, and unbounded-loop free. Shutdown
uses one already reserved generic boundary message and terminal capacity; the render side only moves
bounded existing records and publishes the existing acknowledgement. No acknowledgement or shutdown
completion can orphan accepted work, release reservations early, execute a batch twice, or turn
partially applied faulty work into Canceled. Paired bank/scalar arithmetic and raw `Concurrent`
public preparation remain unchanged.

## Workflow and completion

Astra LOW must approve this stateless scope at a pushed checkpoint before Luna implementation begins.
Each coherent exact-path tranche is committed before another pass. Astra LOW adversarially reviews
every source checkpoint and the exact integrated head. At most three implementation attempts are
allowed; after attempt three fails, preserve the evidence and create a newly bounded successor rather
than retrying.

On source PASS, merge current main, run proportional gates, and ask lane B to probe the six-file
AudioWorklet artifact. Lane B alone qualifies and pins it if bytes change. Obtain Astra LOW exact
source/artifact-head PASS, push once, run required PR qualification, merge, and require successful
post-main qualification. Then synchronize and close #594 and #444, mark RT4 delivered in #559/#560,
verify remote state, and remove every clean delivered #594 worktree. RT5 remains the next partial
barrier; original open findings remain unauthorized until all partials are complete.

## Astra LOW scope review

The initial scope checkpoint `37ffd52f` failed because it authorized no communication mechanism for
terminal intent and ambiguously rejected repeated pending polls. Corrected checkpoint `48b99418`
authorized a charged atomic flag but failed a concrete generation race: a shutdown begin could attach
its one-bit intent to an ordinary cancellation whose generic acknowledgement was already visible while
render had not returned. These were scope corrections and consumed no implementation attempts.

Astra LOW returned **PASS** on pushed checkpoint
`6d857c6038beb8243454ad462b05c0eca6e964ef`. The four-state handshake prevents ordinary completion
and admission reopening until render classifies the acknowledged boundary as `Idle`; shutdown must
therefore consume its own cancellation message and publish `ShutdownAcknowledged`. The deterministic
post-ack/pre-classification rendezvous and fifth mutation discriminate the race. Exact paths,
preallocated resource charging, realtime behavior, terminal reconciliation, fault retention,
teardown, closure disposition, and lane-B disjointness are approved. Luna HIGH/XHIGH attempt 1 may
begin.

## Attempt 1 review

Luna HIGH source checkpoint `111c2b0e` received Astra LOW **FAIL** with two implementation attempts
remaining. The draft's four-state structure is usable, but it treats the legal pending-intent before
generic-message window as sticky Fault and lets shutdown overwrite a cached, not-yet-finalized
ordinary endpoint completion. Required deterministic race/rollback, combined ownership, sticky-fault,
paired bank/forced-scalar state and PCM, shutdown realtime/lifetime/resource, and five restored
mutation gates are missing. Independent debug integration 16/16, endpoint unit 6/6, strict Clippy,
formatting, and diff checks passed; release was not claimed. Full verdict:
`docs/audits/594-attempt1-review.md`.

## Attempt 2 implementation

Luna HIGH corrected both attempt-1 races and root checkpointed the coherent source as `1a99a968`.
The complete attempt-2 evidence checkpoint is `c59b961d`. Debug integration 20/20, endpoint unit 9/9,
release integration 20/20, strict Clippy, rustdoc, formatting, and diff checks pass. Deterministic
intent-before-message/rollback and post-ack/pre-classification rendezvous tests pass, along with exact
combined ticket ownership, sticky-fault retention, stale/duplicate lifecycle calls, native paired-bank
and forced-scalar quiescence, zero render allocation/free, independent Arc/resource caps, and
`stop()` off-render reclamation. All five required direct behavior mutations failed their intended
assertions and were restored. The known generated `Cargo.lock` ordering drift was restored. Full
record: `docs/audits/594-terminal-shutdown-attempt2.md`.

## Attempt 2 review

Astra LOW returned **FAIL** at exact pushed evidence head `edb73387`; one implementation attempt
remains. Both production races and the principal lifecycle/ownership/quiescence behavior are accepted.
Attempt 3 is limited to three evidence corrections: make mutation five reach a valid-sample direct
graph/PCM/state/process discriminator; require exact independently observed Arc allocation and isolate
its retention through control drop and `stop()` until stopped-owner off-render drop; and make the
intent-before-message rendezvous failure-safe so an early assertion cannot hang scope join. Debug and
release each pass 20 integration plus nine unit tests, with strict Clippy, rustdoc, formatting, and
diff checks. Full verdict: `docs/audits/594-attempt2-review.md`.

## Attempt 3 implementation

Luna HIGH corrected only the three attempt-2 evidence blockers; root checkpointed the exact two-file
tranche as `d49fe6fa`. Thread-scoped counters now prove the independent Arc allocation and reported
bytes exactly, while a private unit test isolates real lifecycle ownership across control drop and
`render.stop()` through the final off-render deallocation. Both success and dropped-release
intent-before-publication rendezvous paths are failure-safe and prove rollback without hanging.
Restored mutation five now calls the unchanged valid sample, reaches graph execution, and fails on the
changed render report/advanced clock. Debug integration 20/20, unit 11/11, release integration 20/20,
strict Clippy, rustdoc, formatting, and diff checks pass; `Cargo.lock` is unchanged. Full record:
`docs/audits/594-terminal-shutdown-attempt3.md`.

## Attempt 3 review

Astra LOW returned **PASS** at exact evidence head `9e9ea3c6`. All three attempts were used; zero
remain. Exact Arc accounting/retention, failure-safe and dropped-release rendezvous behavior, and the
valid-sample graph-execution mutation now directly discriminate their claims. The previously accepted
lifecycle, ownership, paired-bank/forced-scalar quiescence, fault, cap, realtime, and mutations one
through four remain intact. Independent debug and release each pass 20 integration plus 11 unit tests,
with strict Clippy, rustdoc, workspace/host/realtime/CI-routing policies and mutations, native, Wasm
scalar, and Wasm simd128 checks. Full verdict: `docs/audits/594-attempt3-review.md`. Delivery remains
pending current-main integration, exact-head Astra LOW review, lane-B artifact disposition, required
CI, merge, post-main CI, and GitHub synchronization.

## Integrated head and artifact disposition

The accepted attempt-3 source was merged with current main `be17e3293fa7fabb425d1b7eb6edd20bd13c6867`
at checkpoint `cb4e4bed4f84ca5c76a0dd02384c235f2836ff24`. Astra LOW returned **PASS** on
that exact integrated source head: the incoming #593/#596 benchmark-metadata changes are disjoint,
the accepted endpoint source/tests and `Cargo.lock` are unchanged, and independent endpoint tests
pass 11 unit plus 20 integration cases.

Lane B then rebuilt the shipped six-file AudioWorklet artifact. Both the repin probe and ordinary
six-file build passed, every shipped file remained byte-identical to the canonical #587 artifact,
and the candidate digest remained the pinned
`39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`. No pin,
qualification result, deployment matrix, source, or consumer changed, so full qualification was
correctly skipped. Durable evidence is in `artifacts/issue594-artifact-qualification/`; the artifact
checkpoint was incorporated as `a1a2f89b`. Final exact combined-head Astra LOW review, required PR
qualification, merge, post-main qualification, and GitHub synchronization remain pending.
