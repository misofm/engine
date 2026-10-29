# SDK: a rebuild-path apply reads the whole control list to follow the strip order (#2189 cost)

GitHub #2206. Found by the studio-v2 console's #15 measurement on SDK 0.10.1 (2026-10-29).

#2189 made browser live controls follow a rebuild. Before a rebuild-path `apply` (or
`replaceSession`) resolves, the SDK reads the core's whole control list once to learn the new strip
order (`followStructure`). That read scales with the session's control count. The console's
bus-add commit (from `apply` to resolve) went from 99 / 131 ms at 32 / 64 tracks on 0.10.0 to
about 233 / 409 ms on 0.10.1 (shape-cost harness, headless Chromium, descriptive single runs).

## Scope

Read only what the strip order needs: the strip headers and each strip's rack identities, which
#2189's per-strip layout check uses. Do not read every control descriptor. Or read the order lazily
at the first edit after the rebuild. Keep #2189's guarantees: held live controls follow a rebuild,
and an index on a strip whose racks changed is refused.

## Gate

The #2189 and #2193 SDK tests stay green. Report the console bus-add commit time at 32 and 64
tracks, before and after (descriptive; no timing gate).

## Findings (attempt 1, 2026-10-29, base `888bbc901`, artifact `e65c1595`): stopped before building

The brief says to stop and report if only a new engine export or a protocol change can do the
narrow read. That is the case.

- F1, cause measured. A descriptive SDK-level probe (not committed) boots the browser engine over
  a host double that runs the shipped core. The session is the console C1 shape: gate, EQ and
  compressor before the inserts, soft clip after them, seven submixes, and every track routed to
  each submix. The probe times four bus adds (`upsertSubmix` and its route to the output) from
  `apply` to resolve. The whole-list read is 35 / 63 frames and about 3.7 / 7.0 MB of reply per
  apply at 32 / 64 tracks. The list has 737 / 1,409 header rows and 7,067 / 13,179 control rows.

  | tracks | as is, no port latency | read removed | as is, 3 ms per frame | read removed, 3 ms per frame |
  | --- | --- | --- | --- | --- |
  | 32 | 122-130 ms | 23-41 ms | 222-248 ms | 29-52 ms |
  | 64 | 220-238 ms | 34-36 ms | 394-407 ms | 42-46 ms |

  The "read removed" column has `followStructure` disabled (a temporary edit, reverted), so held
  controls did not follow; it is the floor, not a candidate. The 3 ms model of a worklet port is
  close to the console's browser numbers (233 / 409 ms).
- F2, the core offers no narrow read. `ControlListRequest` (`crates/protocol/src/schema.rs`) has
  `structure_epoch`, `after_handle`, `limit` (at most `CONTROL_PAGE_ITEMS`, 256) and an optional
  owner filter. Its owner window (`crates/host-core/src/control_provider.rs`, `window`) serves the
  owner's whole run of rows, so section headers come only with every control row between them. The
  host's `sessionMap()` carries sources and the meter flag, not strips. `WasmBoundary.stripOrder()`
  and the browser's `BrowserStripOrders` both read the whole list. No strip-order export exists.
- F3, a lazy read does not keep the guarantees. `edit.strip(id)`, `insert(index, ...)` and
  `withSession()` check the order synchronously (`sdk/src/core/live-controls.ts`). If the read
  waited for the first edit after a rebuild, those calls would need the order before it arrived.
  They would then refuse a bus that the apply added, or accept an index on a strip that the
  rebuild changed. The other option is to move the checks to `submit()`, but that changes the
  public error surface.
- F4, workarounds that the existing protocol allows are not the fix. Page requests can be sent in
  parallel because the provider's handles are dense, but the protocol does not promise that.
  Parallel requests also only cut the round trips: in the probe, the 3.7 / 7.0 MB of encoding and
  decoding still takes about 95 / 190 ms without port latency. If the SDK derived the new order
  from the transaction it sent, it would break #1564 D5 (the core's list is the strip-order
  authority).

## Proposed design (for a ruling before any build)

Add an optional field to `control_list_request`, for example field 6 `headers_only` (bool). With
it, the core serves only header rows (owner and section headers) in list order, with the
catalog's own handles, so the cursor and the `control.list.stale` restart rule do not change. The
SDK's `stripOrderSteps` (`sdk/src/core/control-list.ts`) asks for headers only, and
`stripOrderOf` reads the same rows it reads now. The change covers the protocol schema and its
wire validation, the provider's `window`, the generated SDK protocol tables and fixtures, and a
codec gate for each side. A full-list reader (`controls()`, `controlState()`) does not change.
Expected effect at 64 tracks: about 6 pages of 1,409 header rows instead of 57 pages of 14,588
rows.
