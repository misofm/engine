# Issue #579 attempt 1 qualification evidence

This record qualifies the preserved #576 builtin batch endpoint at the #579 successor head.
The implementation remains limited to `builtin_batch_endpoint.rs` and its focused integration
test. No protocol, graph, engine, manifest, lockfile, or lane-B path was changed.

## Cancellation and ownership

The endpoint retains the one generic cancellation acknowledgement and returns `None` from
`poll_cancel_boundary` until every captured endpoint ticket has passed through `collect`. Applied
metadata is staged before generic collection; after acknowledgement, a missing metadata row is
the never-injected canceled case. An applied prefix followed by a canceled suffix, out-of-order
collection, duplicate collection, stale generation rejection, and next-generation reuse are
covered by the focused integration tests. A private scoped test holds the first generic claim,
publishes a second ticket while the claim is held, then proves the second application is at sample
128 and is late. The control/render rendezvous channels are created inside `thread::scope`, and a
sender-drop panic path joins without a sleep or timeout.

The post-graph fault seam remains sticky. The focused source test attempts cancellation and
collection after the fault and observes `Ok(None)`, `Empty`, and an unchanged outstanding ticket;
the ticket cannot be labeled canceled or reused.

## Audio and resources

`endpoint_drives_nonzero_pcm_through_the_prepared_bank_and_scalar_plan` feeds identical source
planes and fader/matrix records to the endpoint and an independently prepared ordinary console
owner. It compares the complete PCM arrays bitwise and requires nonzero output. The endpoint
report exposes the bank scratch witness; the existing host preparation remains the separate-owner
reference. The private claim test separately proves direct FIFO dispatch ownership.

The requested paired-dispatch counter witness remains blocked by the exact ownership boundary:
`StartedRenderSession` keeps its `PreparedRenderPlan` private and exposes no dispatch-counter
reader, while the #579 allowed paths exclude `crates/host-core/src/render_session.rs` and the
engine/graph production crates. `HostPrepareReport::effect_bank_scratch_bytes` and PCM equality
cannot truthfully substitute for a post-render paired bank/scalar dispatch count. This gate is
therefore reported as an explicit rebrief blocker rather than claimed as satisfied; obtaining it
requires a smallest test-only accessor in the frozen render-session boundary or a separately
authorized successor path.

Resource rows are named by ownership: host builtin payload, endpoint heap, composed heap, largest
endpoint heap, largest host engine allocation, largest composed heap, and prepared/started render
inline sizes. The integration test independently mirrors the concrete SPSC layouts and ledger
array, comparing bytes and largest allocation against the endpoint's report. Exact and one-below
aggregate/largest cap cases pass. The workspace audited allocator supplies the positive
allocation/free liveness control; render application and cancellation snapshots report zero
allocations and frees. A local second global allocator was deliberately not installed because
`bench_support` already owns the workspace global allocator.

## Focused gates

```
cargo test -p host-core --test builtin_batch_endpoint --features control-provider
12 passed; 0 failed
cargo test -p host-core --lib --features control-provider \
  builtin_batch_endpoint::tests::private_post_claim_hold_rejects_same_block_second_claim
1 passed; 0 failed
```

## Direct mutation evidence

Each mutation was applied to the named operation, run with a 30-second safety bound, and restored
from the clean source immediately afterward.

1. Forcing a second `DeliveryCoreRender::begin` while the first claim was pending failed the
   private claim test at `render: Delivery(AlreadyPending)` and the report rendezvous returned
   `RecvError`.
2. Adding a second claim/injection/terminal after the first finish, before the render call
   returned, failed the private claim test at `second application`; the second ticket was
   incorrectly consumed in the current block.
3. Changing `for record in batch.records()` to `batch.records().take(1)` failed the endpoint PCM
   reference at the exact `assert_eq!(endpoint_samples, baseline_samples)` assertion, with the
   left endpoint array missing the matrix transform and the right reference retaining it.
4. Calling generic `delivery.collect(ticket)` before endpoint outcome staging failed the FIFO
   test at `first terminal: StaleTicket`; generic ownership had already been released before
   endpoint reconciliation.
5. Replacing the post-graph sticky `Err(Fault)` with a healthy report failed the source test at
   the exact assertion expecting `Err(Fault)`, showing the observed value as
   `Ok(BuiltinBatchRenderReport { ... applied: None })`.

All five mutations were removed before the green rerun. The mutation records are negative
evidence only; no mutation output is used as a product result.

## Attempt 2 implementation and evidence

Attempt 2 corrected the five findings within the existing endpoint and focused-test paths. Cached
cancellation polling now validates the exact token before reading a cached completion. Publication
and a second `begin_cancel` remain closed after the render-side acknowledgement and after the last
collection until the retained completion is reported once; only then are the token and generation
released. The integration interval test asserts both refusals and then proves next-generation
reuse. The applied-prefix/future-suffix scenario also polls an old token while a later generation
is awaiting collection and requires `StaleTicket`.

The endpoint now owns bounded PostFader meter consumers for the focused endpoint comparison.
Addressed right and left fader and matrix records are compared against a separately prepared
console owner, PCM is bitwise equal, and the eq8 PostFader sample peaks are bitwise equal. The
existing native host target selects a SIMD backend for every track; there is no nonbanked scalar
owner in this target. An attempted support-enabled integration call to the existing
`builtins_compiler` witness APIs compiled only with `--features 'builtins-compiler/test-support'`;
the ordinary focused command cannot resolve those dependency-gated symbols. The support-enabled
run still produced an empty scalar trace because the native fixture is fully banked. The witness
calls were removed from the default integration test so the normal host-core suite remains
compiling. This is recorded as an explicit qualification blocker: a scalar-target run or a smallest
allowed render-plan seam is required; no production or manifest path was widened to fake it.

`actual_endpoint_allocations_and_nonempty_cancellation_reuse_are_live` observes the workspace
allocator around actual endpoint preparation and teardown, then repeats healthy application,
nonempty cancellation, final completion reporting, and next-generation reuse twice. Render
application and cancellation remain covered by the zero-allocation audit. The endpoint's meter
caps in the private source fixture were raised to the concrete bounded meter request used by the
endpoint; no runtime queue or resource cap was weakened.

Direct mutation 5 was rerun against the post-fault cancellation path: clearing the sticky fault
and rendering the cancellation boundary made `poll_cancel_boundary` return an actual
`BuiltinBatchCompletion` with `Canceled`, while the clean assertion expected `Err(Empty)`.
The exact failing assertion was:

```
assertion `left == right` failed
left: Ok(BuiltinBatchCompletion { ticket: CoreTicket { generation: 1, slot: 0, serial: 1 }, ... disposition: Canceled, ... acknowledged_sample: Some(SampleTime(128)) })
right: Err(Empty)
```

The mutation was restored immediately. Focused integration (13 tests), both private source tests,
strict Clippy with the support feature, formatting, and diff checks pass. `Cargo.lock` is restored
before handoff. Full target/policy qualification remains for the root checkpoint; the scalar
witness blocker is not claimed green.

## Attempt 3 final implementation and evidence

Attempt 3 closed the remaining cancellation, backend, telemetry, and retention findings within the
amended scope. Empty cancellation and the all-Applied-collected-before-acknowledgement case now
share one exact-once finalization path. The path clears the retained token only when the completion
is returned, so publication reopens after that return and every later poll is stale. The new
integration test covers both populations and next-generation reuse.

Mandatory production PostFader preparation and the public endpoint meter accessor were removed.
The production endpoint again requests only its bounded control queues, so the existing low-meter-cap
constructor behavior is preserved. PostFader consumers remain only in the private unit preparation
used by the final audio evidence.

The amended crate-private test seam selects `Backend::current()` for the native bank and
`Backend::Scalar` for a forced scalar endpoint while production remains pinned to
`Backend::current()`. The private unit test prepares matching ordinary-console references and
endpoints, submits identical source and addressed records (including track 8 right fader target,
ramp, and matrix), and compares PCM arrays, scalar fader/matrix state traces, and both PostFader
sample peaks bitwise. The existing builtins-compiler witness is reset/read on the render thread;
all paired factory/process/fused/fallback/member counters remain zero for both endpoint forms.

Endpoint queue construction is factored through `prepare_endpoint_queues`. After warming the
current-thread allocator, the retention test observes the actual queue allocations while owners
remain live: requested bytes equal the two independent retained layout reports, with zero
current-thread frees and reallocations. Dropping the queue owner off render reclaims exactly the
same allocation count. Largest allocation remains checked independently through each layout's
largest row; it is not inferred from allocator byte totals.

Accepted mutation evidence remains preserved for mutations 1–4. Mutation 5 was rerun against the
clean final source by clearing the sticky post-graph fault and rendering the cancellation boundary.
The clean `Err(Empty)` assertion failed with:

```
left: Ok(BuiltinBatchCompletion { ... disposition: Canceled, ... acknowledged_sample: Some(SampleTime(128)) })
right: Err(Empty)
```

The mutation was restored. Final proportional debug all-target host-core tests (14 unit tests,
14 endpoint integration tests and the complete host-core integration suite), strict Clippy,
formatting, and diff checks pass. Cargo.lock is restored before handoff. Release, rustdoc, target,
policy, and routing checks remain root-owned final gates.
