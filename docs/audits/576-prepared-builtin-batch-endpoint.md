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
