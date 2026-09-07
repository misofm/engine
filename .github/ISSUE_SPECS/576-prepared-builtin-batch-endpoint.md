# Add prepared builtin batch endpoint through separate owners

GitHub: #576 (https://github.com/misofm/engine/issues/576)

Status: numbered child of #444 and audit #349 RT4 on delivered main
`bf882a84ec630fb559690d010a65baf77dc45734`, after #571/#572 generic boundary
cancellation delivery. This is a remaining partial-finding slice; no original open
finding has started. Luna HIGH implements attempt 1. Astra LOW performs every
adversarial source and exact-head verification. The Sol HIGH coordinator owns the
brief, exact-path checkpoints, GitHub synchronization and delivery. Lane B retains
all artifact qualification and pinning.

## Smallest closable outcome

Add an opt-in Rust host-core endpoint that prepares a host and its live builtin
console channels in one transaction, keeps those raw track producers private, and
accepts bounded typed batches for fader/mute and matrix targets. At each render
boundary its render owner claims at most one FIFO batch, injects the complete batch
into the existing per-track fader and matrix queues, and invokes the existing graph
plan. The current `Concurrent` lowering remains selected, so bank lanes and nonbanked
scalar nodes execute through their original separate owners. No paired processor is
enabled by this child.

This endpoint establishes durable admission, a closed singleton claim, actual and
late application reporting, cancellation, and ownership through terminal collection.
It does not add a wire command, session mutation, automatic plan swap or locate,
native C ABI/browser activation, input builtin delivery, effect automation, bank
pairing, or scalar pairing.

## Frozen public contract

Define one fixed `Copy` batch whose header contains the exact prepared
`SessionRevision`, requested absolute block-start `SampleTime`, and a record count in
`1..=256`. Each record contains a prepared canonical `u32` track index and exactly one
of the existing `TrackFaderRecord` or `TrackControlRecord` values. Preserve record
order, lane selectors, whole 2x2 matrices and explicit smoothing lengths. Input trim
and polarity records, effect parameters, automation segments and opaque payloads are
outside this type.

The endpoint's constructor must call the existing concurrent console preparation
path itself and consume the matching plan and console handles before returning. It
must not accept an arbitrary public plan/handles pair. Every raw per-track producer,
including the unused input producer, remains private to the render owner; none can
race the endpoint or escape to a caller. The control-side result retains the existing
source/report/lifecycle parts needed by this bounded host slice without exposing a
second builtin admission path.

Admission validates the complete batch before publication: nonzero bounded length,
exact revision, in-range track indices, a requested sample aligned to the prepared
quantum, finite/domain-valid fader and matrix values, and legal smoothing. Rejection
returns the untouched batch. Success returns the existing `CoreTicket` and certifies
only retained acceptance. It never acknowledges PCM application. Full delivery
capacity and checked identity/resource overflow reject atomically. Ask on every path:
can an acknowledgement precede a drop? The answer must be no.

Admission order is FIFO. A future ticket at the FIFO head may be claimed and retained
as pending, and blocks later tickets until it is terminal. Multiple tickets may name
the same or earlier boundary; the one-batch-per-quantum bound makes later FIFO tickets
late in a defined way rather than dropping or reordering them.

Each valid render call makes exactly one `DeliveryCoreRender::begin` attempt when no
ticket is already pending. That successful dequeue is the claim linearization point
and closes a singleton population for that block. A publication completing after the
attempt remains owned for a later block. Never drain until empty, sample queue length,
or allow producer refill to enlarge the block's work.

Apply the whole claimed batch at the first render boundary whose first sample is at
or after its request. Populate every addressed fader/matrix queue before invoking the
exclusive graph render. The per-track queue depth must be prepared to hold all 256
records at one destination. A future requested batch stays pending. A missed request
applies at the first eligible later boundary and returns `actual_sample` plus
`late = actual_sample > requested_sample`. This explicit one-batch-per-quantum bound
is part of the contract.

Endpoint-specific outcome storage may associate requested/actual/late metadata with
the existing `CoreTicket`, but it is not another ticket, credit, sequence, or terminal
authority. Capacity covers every outstanding ticket. Control releases generic
delivery credit only after it reconciles the matching outcome and generic terminal.
Duplicate/stale collection and token use refuse without reuse.

## Failure and lifecycle rules

Validate render shape, exact contiguous time and sample overflow before injecting any
record. These ordinary envelope rejections change no target, ticket, queue or clock.
Only publish successful application after the valid graph block completes.

If the graph returns an unexpected error after injection, the endpoint cannot infer
which opaque separate owners consumed their queues. Latch one sticky typed fault,
preserve all ticket and outcome ownership, and report the application state as
unknown. Do not fabricate Applied or Canceled, release credit, reuse storage, or
claim successful cancellation after that fault. Later valid render calls return the
same fault without injecting more records or advancing the endpoint clock. Teardown
occurs only after the render owner is quiescent and off the render thread.

Healthy cancellation uses delivered generic boundary cancellation. A ticket already
completed stays Applied. A not-yet-injected ticket is canceled at the acknowledged
render boundary and cannot later enter a builtin queue. Cancellation completion waits
for the boundary acknowledgement, all captured terminal dispositions and matching
endpoint outcome reconciliation before reporting success or releasing credit.

A healthy render boundary checks for and services a cancellation request before it
injects an already pending batch or claims new work. If cancellation is present, that
call is a cancellation-only boundary: the supplied sample must equal the next required
render sample, generic cancellation is acknowledged there, no builtin queue or DSP is
touched, and the render clock does not advance. The next ordinary render still starts
at that sample. Applied tickets reconcile endpoint metadata carrying requested sample,
actual sample and `late` with the matching generic Applied terminal. Never-injected
canceled tickets derive their public canceled outcome from the generic Canceled
terminal and its acknowledged sample; they carry no fabricated actual-application
metadata.

The prepared revision, sample rate, quantum and contiguous clock are fixed and start
at sample zero. The endpoint is an ephemeral live overlay and never edits the
canonical session model or revision. Preserve a transferable prepared render owner,
a thread-affine started owner, render-thread floating-point attestation/guarding and
off-render destruction. Automatic plan replacement, locate, clock publication and
provider shutdown integration remain later lifecycle work.

## Exact ownership

Allowed implementation paths:

- `crates/host-core/src/builtin_batch_endpoint.rs`
- minimal module/export additions in `crates/host-core/src/lib.rs`
- `crates/host-core/tests/builtin_batch_endpoint.rs`
- this numbered spec and focused evidence beneath `docs/audits/`

Use the existing `control-provider` feature and dependencies. Do not edit protocol,
`scalar_point_endpoint`, graph, engine, builtins, builtins-compiler, other host-core
modules, manifests/lockfile, hosts, C ABI, browser, SDK, artifacts, policies or
workflows. If a missing seam forces one of those changes, stop and split or amend the
issue before implementation. Lane B #575 owns
`crates/protocol/src/controller.rs`, `controller_delivery.rs`, its focused protocol
test, and `crates/host-core/tests/scalar_point_endpoint.rs`; these paths must remain
untouched.

## Bounds, resources and realtime

Preparation configures a nonzero generic ticket bound and fixed 256-record payloads.
Account with checked arithmetic for every generic payload copy and queue backing,
endpoint outcome slot/queue, canonical track mapping, privately retained producer,
inline control/render owner and largest allocation. Include the existing live-console
ring storage and enforce the existing host caps plus any explicit endpoint cap. An
independent observation of retained endpoint storage and its largest retained
allocation must equal the endpoint report. Compose the unchanged host report without
double-counting its producer vectors or queue storage; transient allocations inside
existing host preparation are not claimed to equal retained bytes. A one-below cap and
arithmetic overflow reject before publication while returning owned inputs as the
established host preparation API permits.

After preparation, admission, claim, injection, render, completion, cancellation,
polling and collection allocate and free nothing. Render work is bounded by one
ticket, 256 records and the prepared graph; it performs no locks, syscalls, I/O,
logging, callbacks, reclamation, model/controller access or producer-refill loop.

## Objective gates

1. Use actual separately owned control and render threads with failure-safe
   rendezvous around publication and the single claim. Prove zero/future/on-time/late
   cases, two tickets, and a publication completing after claim remaining retained
   for a later block without timing assumptions or sleeps.
2. Make an invalid final record, wrong revision, bad alignment, out-of-range track
   and full-capacity publication reject the whole
   batch unchanged. No addressed owner changes state and no successful admission or
   application result exists. Equal or regressing requested samples retain FIFO order
   and produce the documented late result when their turn reaches a later boundary.
3. Exercise cancellation before claim and while a future ticket is pending. Verify
   exact captured frontier and acknowledged sample, completed work remaining Applied,
   canceled work never entering a builtin queue, and no ticket/outcome reuse before
   collection. Reject stale and duplicate operations.
4. Render nontrivial PCM through both actual bank lanes and actual nonbanked scalar
   owners using asymmetric fader/mute, crossfeed matrix and nonzero smoothing. Compare
   PCM, state and post-fader observations against independently prepared existing
   separate-owner execution. Assert current paired bank/scalar dispatch remains
   unselected.
5. Prove pre-render envelope/time rejection is a no-op. A private `cfg(test)` seam in
   the new module may inject failure immediately after actual graph execution and
   before terminal publication; it must expose no public callback or graph hook. Use
   it to prove the sticky unknown-application fault never
   fabricates cancellation/completion, releases credit, reuses storage, injects again
   or advances time.
6. Mutation checks must fail if the claim drains more than one new ticket, a
   post-claim publication applies in the same block, only part of a batch is injected,
   credit releases before both outcome and terminal collection, or a possibly applied
   fault is labeled canceled. Restore source and pass the original tests afterward.
7. Use a positive allocator-liveness control and prove repeated healthy render and
   cancellation operations have zero allocations and frees. Verify exact resources,
   FP restoration, Send/Sync ownership, off-render reclamation, debug/release affected
   suites, strict Clippy/rustdoc, formatting, workspace and host policy gates, and
   supported native plus Wasm scalar/simd128 compilation.

## Attempt 1 evidence

The host-core-only endpoint and private producer ownership are implemented in the
allowed source paths. Its fixed typed batches validate atomically, claim one FIFO
ticket per render block, retain future work, report actual/late application after
graph completion, and service cancellation before any builtin or graph work. The
focused endpoint suite passes 3 tests, the full host-core test-support target is
green, and strict host-core Clippy is green. Details are in
`docs/audits/576-prepared-builtin-batch-endpoint.md`.

No benchmark or performance claim is authorized. Artifact qualification is lane B's
responsibility: this source slice must first reach a frozen reviewed checkpoint. Lane
B then decides whether consumer/artifact qualification is required and owns every pin
or browser evidence change.

## Stop and delivery rules

Stop before widening if the endpoint requires a protocol change, shared runtime hook,
new decoder, public graph/builtin API, automatic lifecycle integration, paired
dispatch, effect/input control, another fixture corpus, benchmark framework, or
artifact edit. Preserve evidence and brief the independent outcome.

This child receives at most three implementation attempts. Luna HIGH owns attempt 1;
Astra LOW gives one adversarial verdict per attempt. Root checkpoints each coherent
exact-path tranche before further implementation, pushes promptly, keeps this spec
and GitHub synchronized, obtains exact-head PASS and required CI, verifies the live
base before merge, verifies post-main qualification and closure, and removes the
clean delivered worktree. Delivery advances #444 but does not close it: concurrent
bank pairing, scalar pairing and automatic lifecycle integration remain separately
numbered outcomes.
