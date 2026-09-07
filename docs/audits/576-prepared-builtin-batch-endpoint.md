# Issue 576 attempt 1 evidence

The host-core endpoint is implemented as one preparation transaction. It calls
`prepare_host_runtime_with_console` itself with 256-record fader/matrix/input queue
capacity, keeps the returned raw track producers private to the render owner, and
adds one bounded generic typed delivery core plus one bounded application-outcome
queue. The control owner exposes only typed batch admission, cancellation, terminal
collection, source control and the existing host report.

The fixed `BuiltinBatch` carries the prepared revision, aligned absolute requested
sample, and up to 256 addressed existing fader/matrix records. Admission validates
the complete prefix before generic publication. The render owner claims at most one
ticket per block, leaves future tickets pending, injects a complete due batch before
the graph call, and publishes application metadata only after graph success. A
cancellation-only boundary services the generic cancellation barrier before any
producer or graph access. Render faults are sticky and return all render storage for
off-render teardown.

Focused evidence in `crates/host-core/tests/builtin_batch_endpoint.rs` covers on-time
application, future retention and later application, invalid track rejection with
the original batch preserved, cancellation before claim, exact terminal disposition,
requested/actual samples, and credit reuse after collection. The focused endpoint
selection passes all 3 tests; the full host-core test-support target passes with all
existing host-core suites plus the 3 endpoint tests. Strict host-core Clippy passes.

## Attempt 2

The attempt-1 review identified two correctness issues: generic collection could release a ticket
before endpoint outcome validation, and endpoint queue allocations were not projected against all
configured caps. The endpoint now stages and validates Applied outcome metadata before generic
collection. It preserves staged metadata and generic credit for missing, stale, duplicate, and
invalid collection attempts. Cancellation keeps this reconciliation active: Applied outcomes are
staged first, while a no-outcome ticket is collected only after cancellation has begun and must
resolve to Canceled.

The constructor computes generic delivery and endpoint outcome resource projections before
allocating those queues, checks aggregate retained and largest-allocation caps, and checks the
host plus endpoint composition after the existing host transaction. `BuiltinBatchResources`
reports host retention and endpoint retention separately and composes them once.

The focused suite now covers seven endpoint tests: late application after a prior empty boundary,
future FIFO and exact outcome staging, separate control/render threads with a two-ticket singleton
schedule, invalid NaN matrix and saturation atomicity, cancellation, and exact/one-below resource
caps. The private post-graph fault seam is present under `cfg(test)` and remains unreachable from
the public endpoint API. The session fixture is the existing nine-track prepared plan; this child
does not enable paired bank/scalar dispatch or change runtime source.

### Attempt 2 gate results

Focused endpoint tests pass 9/9, including the private source unit test for the post-graph sticky
fault. The allocator audit reports zero allocation and deallocation events for a repeated healthy
render and cancellation boundary. The fixture reports nonzero effect-bank scratch, while the
endpoint leaves paired dispatch disabled; the separate host observation suites continue to cover
bank and scalar owners.

Mutation probes were applied one at a time and the source was restored from the clean attempt-2
file after each probe:

| mutation | discriminating result |
| --- | --- |
| claim drains a second ticket by replacing the pending guard | focused suite failed `future_batch_stays_pending_then_applies_once_and_invalid_batches_are_atomic` at `builtin_batch_endpoint.rs:78` |
| post-claim publication is allowed to apply immediately by removing the requested-sample filter | focused suite failed `future_batch_stays_pending_then_applies_once_and_invalid_batches_are_atomic` at `builtin_batch_endpoint.rs:412` |
| partial application is reported by marking `record_count - 1` | the nine-test mutation run failed 7 tests at the render assertion `builtin_batch_endpoint.rs:78` |
| generic credit is released before Applied outcome validation | isolated outcome-staging test failed at `builtin_batch_endpoint.rs:226` with the expected collection assertion |
| post-graph fault is labeled successful/no-terminal | private source unit test failed at `builtin_batch_endpoint.rs:917`, proving sticky unknown application is required |

The credit-release mutation was run as a single deterministic test to avoid the intentional
threaded rendezvous mutation leaving a worker waiting after its assertion panic. Every mutation
source was restored before the green rerun.

The endpoint report separates retained heap bytes from checked inline control/render owner bytes; the host row remains separate and is composed exactly once. Largest actual allocation reports cover queue and host heap backings, while inline owner sizes are reported independently.

The endpoint specific PCM gate submits the fixture source, applies asymmetric fader/matrix records,
and observes nonzero output through the prepared plan. Existing host observation gates provide the
independent bank-lane and scalar-owner state/reduction oracle; the endpoint retains the Concurrent
lowering and does not select paired dispatch.

## Attempt 3 final correction record

Attempt 3 gates cancellation collection on the acknowledged endpoint cancellation join. A started
cancel alone no longer authorizes a missing outcome: `poll_cancel_boundary` records the generic
acknowledgement, Applied outcomes are still staged first, and a no-outcome generic terminal can
only be collected after that acknowledgement. The cancellation tests exercise collection before
acknowledgement, applied-plus-future frontier reconciliation, and exact acknowledgement samples.

The endpoint integration uses the workspace `bench_support::alloc::AuditedAllocator` (the crate's
real global wrapper; installing a second wrapper conflicts at link time) and calls
`assert_installed`. A positive allocation/free probe precedes repeated render and cancellation
scopes, whose audit snapshots remain at zero allocations and frees. Resource fields now separate
retained heap bytes, host composition, inline control/render ownership, and largest actual heap
allocation; the endpoint test independently sums each row.

The scoped concurrency fixture creates every channel inside `thread::scope`. It publishes the
second ticket only after the first render thread report crosses the first singleton claim, then
proves the second ticket waits for the next block. A caught panic with sender drop proves the
render receiver exits and scope joins without a timeout.

The endpoint PCM fixture submits asymmetric source planes, applies asymmetric fader and matrix
records, and asserts nonzero output through the prepared bank/scalar plan. Existing host
observation suites remain the independent state/reduction oracle, and this endpoint does not select
paired dispatch. The private cfg(test) post-graph seam directly proves a possibly applied fault
remains sticky and cannot become Canceled.

The final five source mutations were applied individually and restored from the clean attempt-3
source: second claim, post-claim publication, partial injection, premature credit, and fault
mislabel. The first, second, fourth and fifth failed their focused assertions; the partial-prefix
mutation terminated the audit binary with the expected SIGABRT after the render assertion, so its
failure is recorded candidly rather than treated as a green test result.

The endpoint PCM test now prepares a second host through the ordinary console path, submits the
same source planes, pushes the same typed fader/matrix records into its separate track producers,
and compares endpoint PCM bit-for-bit with that independently owned reference. It also compares
render frame reports and asserts nonzero output; the fixture's bank allocation and existing
observation suites provide the bank/scalar state oracle while paired dispatch remains unselected.
