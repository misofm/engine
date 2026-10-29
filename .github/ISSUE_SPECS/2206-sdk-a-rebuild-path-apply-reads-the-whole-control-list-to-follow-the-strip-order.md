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

## Decision (root ruling, 2026-10-29)

Approved as proposed: optional field 6 `headers_only` (BOOL) on `control_list_request`, an in-place
V1 amendment. The field is written only as `true`; a present `false` is `INVALID_FIELD` (one
spelling per value). With it, the core serves only the owner and section headers of the window,
with the list's own handles, order, epoch and cursor rules; `limit` counts header rows, and `eof`
says no header row of the window follows. A `CONTROL_STATE_GET` with it covers the same header
rows, so its page has no record. Without it, the request and the reply are byte-identical to
before; no fixture or generated SDK surface changed. Attempt 2 adds a corpus frame value for the
field and re-pins `COMPLETE_SCHEMA_HASH` (below). The C ABI serves it through the existing `CONTROL_LIST_GET` frame (no new symbol).
The SDK codec (`encodeControlListRequest`, optional `headersOnly`) sends it, and
`stripOrderSteps` uses it, so `stripOrderOf` reads the same header rows as before. Full-list
readers (`controls()`, `controlState()`) do not change. #2189's guarantees stand: the #2189 and
#2193 SDK evals stay green. No render path changed.

## Gates and evidence (attempt 1)

- Protocol gate, `crates/host-core/src/control_provider.rs`,
  `a_headers_only_request_round_trips_and_pages_exactly_the_lists_header_rows`: the request
  encodes and decodes through `ProtocolCodec` to itself, and the provider's headers-only pages, at
  limit 4 with no owner and at limit 2 with an owner filter, hold exactly the full list's header
  rows, with their handles and order, in `ceil(headers / limit)` pages. Red on three reverts: the
  provider ignores the field (control rows appear), the encoder drops it (`Truncated`), the decoder
  ignores it (the round-trip assertion).
- SDK codec gate, `sdk/test/strip-order-evals.mjs` (#2206 describe): `readControlStripOrder`
  against the shipped core gets no control row on any page, the header rows equal the full list's,
  and the order and sections equal `stripOrderOf(full list)`. Red on three reverts:
  `stripOrderSteps` without `headersOnly`, the encoder not sending field 6, and field 6 sent with
  the wrong wire type (the core refuses).
- Apply-to-resolve times (descriptive, the attempt-1 probe; four bus adds per row; "before" is the
  2cabe4121 SDK on the e65c1595 module, "after" this branch's SDK on its module; held controls
  followed every apply in both):

  | tracks | before | after | before, 3 ms per frame | after, 3 ms per frame | frames before / after | reply per apply before / after |
  | --- | --- | --- | --- | --- | --- | --- |
  | 32 | 107-137 ms | 26-41 ms | 236-258 ms | 44-62 ms | 35-36 / 6-7 | 3.8-4.0 MB / 124-128 KiB |
  | 64 | 198-210 ms | 40-43 ms | 419-436 ms | 71-77 ms | 63-64 / 9-10 | 7.0-7.2 MB / 235-239 KiB |

- Checks: `cargo fmt --all --check`; clippy `-D warnings --all-targets --all-features` on
  protocol, host-core, control-plane, host-web, capi, conformance and audit; their tests with
  `--all-features` (1,387 passed, 0 failed); `check-workspace-policy.sh`; `realtime_policy` (291
  marked regions in 46 files); `build-web-audioworklet.sh` (AudioWorklet module `bbdfd0cd...`, a
  later release re-pins it); `check-sdk-headless.sh` (706 passed); `check-sdk-types.sh`;
  `check-sdk-generated.sh`; `check-sdk-deletions.py`; `sdk-package.sh check`; Chromium
  qualification with `--sdk-root --check-matrix`, single and worker legs, both passed.

## Attempt 2 (verify of 3f97c105f: FAIL on two MAJORs, both folded)

- MAJOR 1 and MAJOR 2: the protocol gate now also asserts that a headers-only
  `CONTROL_STATE_GET` page has no record (with the list page's `last_handle` and `eof`), and that
  decoding field 6 = `0` is `INVALID_FIELD`. Red on each mutation: decoder `Some(false) => false`
  (`Ok(.. headers_only: false)` against `Err(InvalidField)`); state path without
  `&& !request.headers_only` (28 records against 0).
- MINORs: the unexercised `MockProvider` branch is removed (the mock ignores the field). A
  headers-only list page reserves at most `limit` entries and a headers-only state page reserves
  none; the full path's reservation is unchanged. The registry says field 6 is non-mandatory (a
  peer that does not know it skips it and serves the full list), and that unlike
  `include-window` a present `false` is refused.
- Re-pin: the corpus's shared control request carries `headers_only: true`, so
  `command.control_list_get` and `command.control_state_get` move; `COMPLETE_SCHEMA_HASH`
  `de7d7cf504ca191a` -> `f05688f0cd7e6f28`, 61 frames, the 59 other frames byte-equal. Updated
  with it: `crates/conformance/src/protocol_corpus.rs`, the two self-test search strings in
  `scripts/check-protocol-wasm-parity.sh`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md` and
  `fuzz/corpus/complete-schema-manifest.md`. No fuzz seed carries a control list frame, so none
  moved; there is no other wasm golden.
- Checks: fmt; clippy and tests of the touched crates (1,387 passed); workspace policy; realtime
  policy; `check-conformance-boundaries.sh`; `cargo run -p conformance`;
  `check-protocol-wasm-parity.sh` and its `--self-test`; `build-web-audioworklet.sh` (module
  `f613f286...`); `check-sdk-headless.sh` (706 passed); the strip-order eval.

## Rebase onto main dd386abf0 (after #2208)

Attempt 2 passed review. The branch carries only the #2206 commits on top of main, and two review
MINORs are folded: the re-pin is recorded beside `COMPLETE_SCHEMA_HASH`, and the `MockProvider` doc
says it ignores `headers_only`. On the rebased tree every gate is green again:
- fmt, clippy and the touched crates' tests (1,387 passed);
- `cargo run -p conformance` and the wasm parity check;
- the workspace and realtime policies;
- the AudioWorklet module build (`f613f286...`);
- the SDK gates: headless (710 passed), types, generated, deletions and package;
- the strip-order eval;
- the Chromium qualification with `--sdk-root --check-matrix`, both legs.

The probe at 32 / 64 tracks gives 26-42 / 40-43 ms, and 45-62 / 72-78 ms at 3 ms per frame.
