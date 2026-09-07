# Finish builtin batch endpoint cancellation and qualification

GitHub: #579 (https://github.com/misofm/engine/issues/579)

Bounded successor to hard-stopped #576 under parent #444 and audit lane A #559.
#576 reached its three-attempt limit at source head
`bb839fcc57651695b35276d578d8f6215a313817`. Astra LOW accepted the endpoint's
private preparation, fixed payload, ordinary collection gating, allocator hook,
thread ownership, allocation-cap enforcement and ordinary PCM reference, but found
five remaining contract/evidence gaps. This successor owns only those gaps and
delivery of the preserved source. It does not relabel or repeat #576's attempts.

## Frozen input and product

Start from #576 branch checkpoint `775722c1`, which preserves source head `bb839fcc`,
all three reviews and the hard-stop ruling. Keep the endpoint payload, public
constructor, private raw producers, fixed revision/rate/quantum, singleton FIFO claim,
on-time/late rule, separate Concurrent builtin owners, thread-affine render wrapper,
sticky-fault policy and host-core isolation. Do not redesign the endpoint or enable
pairing.

## Bounded corrections

### Cancellation join

`poll_cancel_boundary` may consume and retain the generic boundary acknowledgement,
but it must return `None` until every captured endpoint ticket is reconciled through
the ordinary endpoint collection path. The render owner must publish endpoint outcome
metadata before it publishes an Applied generic terminal, and must publish all such
outcomes and terminals before the generic cancel acknowledgement. Acquiring that
acknowledgement therefore makes every captured Applied outcome visible to control; a
missing outcome after acknowledgement denotes a never-injected Canceled terminal.
Test staging both before acknowledgement and after acknowledgement while an outcome is
visible but not yet consumed. An acknowledgement-before-Applied-outcome schedule is
invalid and must not be used as a fixture.

The generic API has no non-consuming terminal inspection: `collect` is the sole
release authority and clears its slot as it returns `CoreCompletion`. Within that
constraint, endpoint collection must first validate and stage the outcome expectation
from endpoint metadata, acknowledged cancellation, FIFO identity and the render
publication-order invariant, then invoke generic FIFO collection exactly once, verify
the returned completion and retire endpoint metadata. No second ticket/credit ledger
or protocol change is permitted. The proof and direct mutations must establish that
the staged expectation cannot disagree in production; do not claim recoverability
from an impossible post-release core corruption. Only after all captured tickets are
collected may a later endpoint poll return the retained cancellation completion. No
second generic poll is made after its one-shot acknowledgement.

This keeps caller terminal consumption and credit release coupled: cancellation poll
does not collect tickets for the caller, create a second terminal authority, or make
storage reusable early. A ticket applied before the boundary remains Applied with
exact requested/actual/late metadata. A never-injected ticket is Canceled at the
acknowledged sample with no fabricated actual application. A sticky uncertain graph
fault refuses cancellation success and credit reuse.

### Truthful resources

Rename or split every ambiguous report field. A builtin-only host value must say
`host_builtin_retained_payload_bytes`; it may not claim total host retention. Report
endpoint-owned retained heap separately from inline owners. Report the prepared and
started render wrapper inline sizes separately and include their plan/fault fields in
the correct concrete type. `largest_endpoint_heap_allocation_bytes` considers actual
endpoint heap allocations only; inline objects are not allocations. The unchanged
`HostPrepareReport` remains the authority for graph/source/effect/builtin host rows.
Compose totals only where all rows are present and prove no producer vector or live
queue bytes are counted twice.

Use an independent retained-allocation oracle, separate from the production report
arithmetic, for endpoint heap bytes/count/layout and largest allocation. It must
observe the actual prepared endpoint after construction. Gross transient constructor
traffic is not equated with retained storage. Repeat prepare/render/cancel/collect
reuse cycles and retain the real allocator positive control plus zero allocation/free
render assertions.

### Direct discriminators

Add a private test-only deterministic seam immediately after the single generic claim
and before injection/graph completion. It may exist only under `cfg(test)`, expose no
public callback or production synchronization, and must allow a unit test to publish
a second ticket while the first render is still held after claim. Prove the second
ticket cannot be claimed or applied in that block and becomes late on the next
eligible boundary. All rendezvous endpoints belong inside `thread::scope`; a
pre-release control panic must drop its sender and let render join without sleeps or
timeouts.

Extend endpoint-specific audio evidence to include both an addressed real bank lane
and a deliberately nonbanked scalar owner. Compare PCM with `to_bits`, builtin target
and ramp state, and PostFader observations against independently prepared existing
separate-owner execution using identical sources and records. Read a direct dispatch
witness proving paired bank and paired scalar processors remain unselected; effect
bank scratch or nonzero PCM is not a substitute.

Run five concrete source mutations against the exact claims:

1. make one render claim a second newly published ticket;
2. allow a ticket published at the post-claim seam to apply in the same block;
3. inject only a strict prefix of an otherwise valid typed batch;
4. release generic credit before endpoint outcome/terminal reconciliation;
5. label a potentially applied sticky-fault ticket Canceled or permit cancellation
   success after that fault.

Each mutation must change the named production/test-only operation, fail its intended
assertion without hanging, be recorded candidly, and be restored. Prefix-terminal
corruption or an assertion-triggered abort is not evidence for partial injection.

## Exact ownership

Allowed implementation paths:

- `crates/host-core/src/builtin_batch_endpoint.rs`
- `crates/host-core/tests/builtin_batch_endpoint.rs`
- this numbered spec
- #576's local spec only for final successor/delivery linkage
- focused successor and final review evidence under `docs/audits/`

Minimal correction of #576's existing `crates/host-core/src/lib.rs` export is allowed
only if a renamed public resource type requires it; otherwise keep it byte-identical.
Do not edit protocol, scalar Point, graph, engine, builtins, builtins-compiler, any
other host-core module, manifests/lockfile, hosts, C ABI, browser, SDK, artifacts,
policies or workflows. Active lane B #578 retains its controller,
controller-delivery, controller-test and scalar Point test paths and explicitly
excludes every production host-core file. If a required proof needs a broader
production seam, stop and rebrief instead of widening.

## Objective gates

1. Deterministically stage control before acknowledgement and after acknowledgement
   with Applied outcome metadata visible but unconsumed. Exercise an Applied prefix
   followed by a Canceled suffix within one captured cancellation frontier. In a
   separate scenario, fully reconcile a Canceled generation, then admit and apply new
   work in the next generation. An out-of-order collection attempt must refuse without
   mutation, followed by successful FIFO collection. Prove poll returns completion
   only after every captured ticket is collected. Missing metadata before
   acknowledgement, duplicate/stale collection and early reuse attempts preserve
   ownership; after acknowledgement a missing outcome is the proven never-injected
   Canceled case.
2. Use the private post-claim seam with real control/render owners to publish the
   second ticket while render is held. Prove exactly one claim/injection/application in
   the current block and late FIFO application in the next. Prove assertion failure
   drops all inside-scope senders and joins.
3. Compare exact PCM bits, target/ramp state and PostFader observations for addressed
   bank and scalar owners against separate execution, and directly prove paired
   dispatch counts remain zero.
4. After the private post-graph/pre-terminal fault, attempt cancellation and collection
   and prove no Canceled label, completion, release, reuse, reinjection or clock
   advance. Off-render teardown retains the possibly applied owner.
5. Independently observe retained endpoint heap layouts/bytes/count/largest allocation;
   verify each renamed field, host-report composition, exact and one-below caps, and no
   double count. Repeat healthy application and empty/nonempty cancellation reuse with
   allocator liveness and zero render allocation/free.
6. Preserve exact outputs for all five direct mutations, restore source, and pass the
   original discriminators afterward.
7. Rerun complete #576 endpoint and host-core suites in debug/release, strict
   Clippy/rustdoc, formatting/diff, workspace/host policies, CI routing, native x86-64-v3
   and Wasm scalar/simd128 compilation. No benchmark or performance claim is authorized.

## Workflow and completion

Luna HIGH implements successor attempt 1. Astra LOW performs every adversarial source
and exact-head review. The successor has its own maximum of three attempts; #576's
history remains immutable. Root checkpoints each coherent exact-path tranche, pushes
promptly, synchronizes both issues and #444/#559/#560, and obtains required CI on an
exact reviewed head. On PASS, deliver the complete preserved #576 source with this
successor, close both issues after upstream evidence, verify post-main qualification,
and remove both clean delivered worktrees. Delivery advances #444 but does not close
its bank pairing, scalar pairing or automatic lifecycle children.

## Attempt 1 review

Luna HIGH implementation checkpoint `2744cc7ba0d344553878fdd5ed590527f491ae57`
passed its focused and proportional gates. Astra LOW returned **FAIL**: cached polling
does not validate token identity; the endpoint admits or starts another cancellation
after captured collection but before reporting the retained completion; the audio gate
lacks direct existing bank/scalar pairing witnesses, addressed scalar state and
PostFader evidence; the resource oracle repeats layout arithmetic without observing
actual retained endpoint allocation; and the sticky-fault mutation targets fault
reporting rather than false cancellation. The full verdict is
`docs/audits/579-attempt1-review.md`.

Attempt 2 must correct only those five findings in the existing exact paths. Existing
builtins-compiler test-support witnesses and same-module stopped-plan inspection are
sufficient; no render-session, protocol, manifest, artifact, pairing or lane-B path is
added.

## Attempt 1 implementation record

Attempt 1 preserves the #576 product surface and closes the bounded qualification gaps. The
endpoint now retains the generic cancellation acknowledgement until all captured endpoint
tickets are collected, stages Applied outcome metadata before generic collection, and leaves a
missing post-ack outcome as the never-injected Canceled case. It reports separately named host
payload, endpoint heap, composed heap, largest endpoint/host/composed heap, and prepared/started
render inline rows. The focused tests include an independent concrete SPSC/ledger layout oracle,
exact and one-below cap checks, allocator liveness, and zero render allocation/free cycles.

The private cfg(test) post-claim hold uses scoped control/render owners to publish a second ticket
while the first is held; the second is shown to apply only at the next boundary and to be late.
The ordinary endpoint PCM is compared bitwise against a separately prepared console owner with
identical source and records. The post-graph fault test refuses cancellation success and retains
the outstanding ticket. Five direct source mutations were run, recorded, and restored; see
[`docs/audits/579-builtin-batch-endpoint-qualification.md`](../../docs/audits/579-builtin-batch-endpoint-qualification.md).

Focused result: 12 integration tests and the private post-claim source test pass. Full debug,
release, strict Clippy/rustdoc, target, policy, and CI-routing results remain to be attached at
the root checkpoint. The paired bank/scalar dispatch-counter witness remains an explicit blocker:
the exact allowed paths cannot read the private `StartedRenderSession` plan, and the report's bank
scratch row plus PCM equality do not prove post-render paired dispatch counts. A smallest
test-only accessor in `render_session.rs` or a rebriefed successor is required before claiming
this gate.
