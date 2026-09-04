# Issue 369: replace `MockProvider` in the production C ABI

## Scope

Close efficiency-audit row IO-4 only. The shipped `capi` controller currently instantiates
`protocol::MockProvider` with an empty enumerable catalog and one synthetic automation descriptor
for nonexistent track `capi`, effect `control`, handle `u32::MAX`. Replace it with a
`host-core::SessionControlProvider` derived from the compiled session and the live plan sample
projection. Apply IO-1 item 2 by gating `MockProvider` and `MockProviderConfig` behind
`cfg(any(test, feature = "test-support"))`.

All work is control-plane. No render arithmetic or rendered bit may move. Do not implement IO-5's
automation drain, other IO audit rows, crate renames, dependency updates, or new tooling.

## Product contract

- Parameter metadata walks normalized tracks, racks, effect declarations and the same launch
  native effect descriptors used by preparation. Handles are nonzero, strictly increasing and
  revision-scoped.
- Parameter state reflects the compiled declaration or descriptor default in handle order.
- Automation domain admission uses the corresponding real descriptor.
- Current/effective sample reads the live plan's published next absolute sample.
- Transport is endpoint-local absolute state/position with that effective sample.
- Counters expose existing protocol telemetry counters and canceled automation.
- Diagnostics expose the existing bounded C ABI render-diagnostic slots without allocating on the
  render path.
- A structural replacement fully allocates its candidate catalog before protocol commit and
  publishes the catalog only after commit succeeds.
- Retained-resource admission includes active and candidate provider catalog allocations.

## Verification gates

- `cargo test -p capi`
- `cargo test -p capi --test resource_lifecycle`
- `scripts/check-capi-abi.sh`
- `scripts/check-abi-layout-v1.py`
- refresh `docs/C_ABI_V1_QUALIFICATION.md`
- `cargo test --workspace` on this branch and `origin/main`, with equal pass counts except tests
  introduced here
- compare the worktree with `origin/main`

This is a non-render row, so no benchmark, bit-identity suite, code-generation dump, or render-row
policy gate is required by the finding.

## Decision and evidence record

The provider belongs in `host-core`: descriptor preparation is already shared there and the C ABI
continues to own protocol queues, replay, transport dispatch, render diagnostic reservations and
plan exchange. `PlanSampleSource` is a read-only shared projection; it adds no render operation and
uses the C ABI's existing release/acquire sample publication.

The retained lifecycle fixture's active CAPI row is re-derived from 149,862 bytes after the fixture
provider type replacement, plus 10,800 bytes of soft-clip catalog storage and 282 bytes of bounded
diagnostic projection storage, for 160,933 bytes. The double-live CAPI admission is 204,375 bytes.
The existing 58,804-byte canonical writer remains the largest named allocation for that fixture.

Final command outputs and commit/PR identity are recorded in the pull request.
