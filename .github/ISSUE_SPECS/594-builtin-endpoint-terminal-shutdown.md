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
duplicate begin/poll calls return the specified stale/terminal error and cannot create another
boundary message or completion.

`poll_shutdown` validates the exact token. Before render acknowledgement it returns Pending. After
acknowledgement but while any accepted terminal is uncollected it remains Pending. Only after exact
terminal collection does it return one completion containing the matched token, frozen frontier, and
acknowledged sample. Completion is not inferred from `outstanding == 0` alone and cannot precede the
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
workflows, or lane B #593 paths. Reuse the existing generic cancellation storage; no new queue,
ledger, heap allocation, manifest dependency, target-handle mapping, generation clear, plan exchange,
or public raw producer is authorized. Any added inline endpoint state must be reported truthfully by
the existing inline-owner resource fields. If a new retained heap allocation or generic delivery
change appears necessary, stop and rebrief rather than widening this child.

Direct red mutations may temporarily change one named shutdown predicate/order in the endpoint
module, run its focused discriminator, and restore the source. Mutations are never staged or
committed.

## Objective gates

1. Use deterministic control/render rendezvous around the actual generic cancellation publication
   and acknowledgement. Prove shutdown cannot complete before the render boundary; after the exact
   acknowledgement but before final terminal collection it still cannot complete; after collection
   it returns the matching token, frontier, and acknowledged sample exactly once.
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
6. Run the positive allocation/free audit plus repeated shutdown acknowledgement and terminal
   quiescent calls with zero allocations/frees. Stop only after render quiescence; prove the returned
   owner keeps all coupled storage alive until off-render drop and that teardown releases it there.
7. Run and restore four behavioral red mutations: publish completion before render acknowledgement;
   publish before final ticket collection; reopen admission after shutdown; permit graph execution
   after acknowledgement. Each must fail the assertion that directly proves the altered behavior,
   and the final worktree must be clean.
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
