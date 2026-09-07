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

Factor actual endpoint queue preparation into the same helper used by production.
After warming allocator TLS, measure current-thread counters around that helper alone,
excluding host preparation, while every returned owner stays alive. Require exact
requested bytes and allocation count with zero frees and reallocations, compare bytes
and count with the independent concrete layouts, and then require the same allocation
count to be reclaimed off render. The allocator does not report freed bytes or largest
allocation, so validate largest allocation only from the independent individual
queue/header/ledger layouts and do not claim it was directly measured.

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
- `crates/host-core/src/prepare.rs`, only for one crate-private `cfg(test)` backend
  selection seam used by endpoint unit tests; production remains pinned to
  `Backend::current()` and exposes no override
- `crates/host-core/Cargo.toml`, only to enable the existing
  `builtins-compiler/test-support` feature on the existing dev-dependency; the normal
  dependency and production feature graph remain unchanged
- this numbered spec
- #576's local spec only for final successor/delivery linkage
- focused successor and final review evidence under `docs/audits/`

Minimal correction of #576's existing `crates/host-core/src/lib.rs` export is allowed
only if a renamed public resource type requires it; otherwise keep it byte-identical.
Do not edit protocol, scalar Point, graph, engine, builtins, builtins-compiler, any
other host-core module, any other manifest, lockfile, hosts, C ABI, browser, SDK,
artifacts, policies or workflows. Active lane B #578 retains its controller,
controller-delivery, controller-test and scalar Point test paths and explicitly
excludes every production host-core file. If a required proof needs a broader
production seam beyond that one amended test seam, stop and rebrief instead of
widening.

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

## Attempt 2 implementation record

Attempt 2 corrected cached-token identity, retained cancellation completion gating, actual
allocation/reuse observation, and PostFader endpoint comparison in the exact endpoint/test paths.
The endpoint rejects publication and another cancellation from the last collection through the
single retained completion report, then permits next-generation reuse. The focused integration
suite passes 13 tests; the private sticky-fault and post-claim tests pass; strict Clippy,
formatting, and diff checks pass with `builtins-compiler/test-support` selected.

Endpoint and separately prepared console PCM and eq8 PostFader peaks match bitwise. The native
x86-64-v3 target banks every fixture track, so it has no nonbanked scalar owner. The existing
`builtins_compiler` witness APIs are dependency-feature gated: they are unavailable to the normal
host-core integration command and, when the support feature is selected, the native fixture still
reports an empty scalar trace. The calls therefore cannot persist within the exact frozen paths
without a manifest or render-plan seam. A scalar-target execution or a smallest allowed seam is
required for the explicitly nonbanked scalar target/ramp gate; this attempt does not claim it.
Full target/policy and GitHub synchronization remain root-owned.

Mutation 5 was changed to clear the sticky post-graph fault and render the cancellation boundary.
The clean `Err(Empty)` collection assertion then failed with an actual `Ok(BuiltinBatchCompletion
{ disposition: Canceled, acknowledged_sample: Some(SampleTime(128)), .. })`; the mutation was
restored. Cargo.lock was restored after the gates.

## Attempt 2 review and final-attempt amendment

Luna HIGH correction checkpoint `62b034c83251c3ea2e37092a81a26f9659b9e948`
passed its focused and proportional gates. Astra LOW returned **FAIL**: immediate empty
or already-collected cancellation reports its completion twice and keeps publication
blocked; the required scalar execution is structurally unavailable through pinned
native preparation; proof-only PostFader meters changed the public constructor's cap
behavior; and allocator liveness still does not compare actual retained endpoint
bytes/count/largest allocation with the report. The full verdict is
`docs/audits/579-attempt2-review.md`.

Attempt 3 is final. It may add only the crate-private `cfg(test)` backend-selection
seam in `crates/host-core/src/prepare.rs` and dev-dependency-only test-support feature
amendment in `crates/host-core/Cargo.toml` recorded above, keep production pinned to
`Backend::current()`, revert proof-only public telemetry, unify exact-once cancellation
finalization, and finish actual-retention plus native-bank/forced-scalar evidence. No
other path or product expansion is authorized. A third FAIL hard-stops #579; gates may
not be weakened.

## Attempt 3 final implementation record

The final amended pass unifies immediate and cached cancellation completion through one exact-once
finalizer. Empty and already-collected cancellation each return one completion, then reopen
publication; subsequent polling is stale. Production PostFader preparation and the public endpoint
meter accessor were removed, preserving low-meter-cap constructors. A private unit-only meter
setup remains for evidence.

The crate-private `cfg(test)` backend seam prepares both native `Backend::current()` and forced
`Backend::Scalar` endpoints; production remains pinned to `Backend::current()`. Matching ordinary
console references and endpoints receive identical source and target/ramp/matrix records. PCM,
scalar state traces, and PostFader peaks match bitwise, and the builtins-compiler witness reports
zero paired factory/process/fused/fallback/member execution. The existing dev dependency alone
selects `builtins-compiler/test-support`.

Actual queue preparation is factored into `prepare_endpoint_queues`. Current-thread allocator
measurement observes exact retained layout bytes with zero frees/reallocations while owners live,
then exact allocation-count reclamation after off-render drop. The final mutation record clears
the sticky fault and observes false `Canceled` success, failing the clean `Err(Empty)` assertion;
the mutation is restored.

Final proportional host-core tests, strict Clippy, formatting, and diff checks pass. Cargo.lock is
restored. Root must complete release/rustdoc/target/policy/routing checks and the final review
synchronization; this is the final #579 attempt and no fourth pass is authorized.

## Attempt 3 hard stop

Final source checkpoint `d67becc3d4dd7100faf3b172c6d18e87d963bfaa`
passed all proportional gates. Astra LOW returned **FAIL**: actual endpoint queue
allocation now precedes the retained/largest cap checks, regressing preflight refusal;
and the two new native-bank/forced-scalar PCM comparisons use `f32` equality rather
than the required `to_bits` arrays. The complete verdict is
`docs/audits/579-attempt3-review.md`.

The three-attempt hard stop is reached. No fourth #579 revision is authorized. Freeze
and preserve `d67becc3` plus all three reviews. A newly numbered bounded successor may
edit only the same endpoint/spec/evidence paths needed to restore report-only cap
projection before queue allocation, add the allocator-backed zero-endpoint-allocation
refusal discriminator, and replace the two PCM comparisons with mapped bits. It may
not reopen the accepted cancellation, backend, meter, retained-layout, state,
PostFader, pairing, protocol, graph, artifact, or lane-B work. #579 remains open and
undelivered until that successor earns exact-head PASS and delivers the preserved
source.

## Successor #580 linkage

Issue #580 attempt 1 preserves the accepted #579 source and closes the two residual defects: report-only endpoint retained/largest preflight before host/queue allocation, and mapped bitwise PCM plus signed-zero discrimination. Its focused evidence is `docs/audits/580-builtin-endpoint-cap-preflight-bitwise.md`.
