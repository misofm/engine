# Charge the monitor overlays and edit ramps in the retained rows

Successor of #1310 (supersede an unadopted candidate; it adds the kept base) and #1683 (the master
row joined the overlay), decision 17 C1 (one core, one accounting on both hosts). Code anchors
verified on the batch 15a branch `codex/controls-batch-15a` at `e8f72edc5`, and read-only on the
in-progress #1310 tree `codex/controls-batch-15aw` (uncommitted; its names are marked below).
Issue number: assigned at filing (written here as #1696). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review.

## Product outcome

The retained-bytes row that bounds the control plane (`capi_retained_bytes` on the C ABI, the
control plane's `control_retained_bytes` that the browser's report reads) charges every byte the
control plane keeps for the session's monitor overlay and for the edit-ramp lengths it holds
between transactions, including the copies a pending successor keeps. An agent that raises the
limit to the byte it measured is never surprised by a heap the row did not count, and the two
hosts charge the same, because the row is computed once, in the core.

## Context

- **The row.** `capi_resources` (`crates/control-plane/src/compile.rs:210`) sums fixed allocation
  rows (the queues, the replay cache, the observation tables, the binding table), the adapter's
  rows (`ADAPTER_ALLOCATIONS`), the epoch rows (control tables, inventory, producers) and the
  catalog into `CapiResources { active_retained, epoch_retained, prepared_protocol_retained,
  largest }`. The C ABI stores it as `capi_retained_bytes` (`crates/capi/src/runtime/mod.rs:250`,
  read back at `:290`); the control plane checks it against `maximum_control_retained_bytes`
  (`compile.rs:844`, `:917`, `:1093`). The browser reuses it: `hosts/host-web/src/lib.rs:6254`
  adds `resources.control_retained_bytes` to its own bridge rows, so a charge added to the core's
  row reaches both hosts without a host edit (the implementer confirms this on the final tree and
  adds a bridge row only if a host holds a copy; none was found).
- **The overlay is uncharged.** `SessionState::monitor` (`crates/control-plane/src/control.rs:411`)
  holds a `controls::MonitorOverlay` (`crates/controls/src/effective.rs:32`): `soloed:
  Vec<StableId>` (sorted, no duplicate) and `master: Option<StableId>` (#1683). Each `StableId` is
  a `String` of at most `STABLE_ID_MAXIMUM_BYTES = 127` bytes (`crates/session/src/id.rs:7`). A
  search of `compile.rs` for `monitor` or `overlay` finds only the boot seed (`:1207`) and the
  struct's doc (`:29`): no row charges the vector, its spare capacity or the IDs' heap text. Each
  commit also builds the next overlay (`MonitorOverlay::after`, `control.rs:1624` and `:1998`),
  a transient copy that lives until the commit moves it into `self.monitor` (`:1858`, `:2351`).
- **Edit ramps are transaction metadata that #1310 makes a retained value.** `EditRamps`
  (`crates/session/src/edit_ramp.rs:100`) holds three tables (`:101`-`:106`, verified on
  `codex/controls-b15a-fold` at `804fe6939`): `entries: BTreeMap<Key, u32>` where
  `Key { owner, row }` (`:65`); `unset: BTreeSet<Key>`, the rows the latest edit wrote with no
  length; and `cleared: BTreeSet<Owner>` (`Owner`, `:56`), the targets an upsert rewrote whole
  (`clear_target`, `:142`). #1310 added the two sets for `overlay` (`:172`). A prepared
  transaction carries one (`crates/protocol/src/model.rs:931`) and it dies with the
  transaction. Rows: a strip has `Fader`, `Mute`, `Matrix`, `Input`, `Link` (five); a route has
  `RouteGain`, `RouteMute`, `RouteMatrix` (three); a VCA has `VcaFader`, `VcaMute` (two); the
  console has `Link` (one). So `entries` and `unset` each hold at most `5 x strips + 3 x routes +
  2 x VCAs + 1` keys (each key's owner ID text is at most 127 bytes; `entries` adds a `u32`), and
  `cleared` holds at most one `Owner` per strip, route, VCA and the console. The charge covers all
  three tables.
- **#1310's kept base.** `KeptBase { model, monitor, edit_ramps,
  monitor_ramp }` (`control.rs:78` on `codex/controls-b15a-fold` at `804fe6939`; the other anchors
  of this bullet are from the b15aw tree and are re-read by the implementer) is set on the epoch a pending successor succeeds
  (`:2380`), extended by every later commit (`:2355`, `:2987`), merged by `merged_ramps` (a clone
  of the kept lengths plus the new transaction's), and dropped when the epoch stops being the
  running one (`:1745`, `:2391`). Only `model` is charged: `held_models` (`:1651`) returns one
  `ResourceEstimate` per kept model, and both admissions add them (#1398 D4). The kept `monitor`
  and `edit_ramps` are held and not charged; `monitor_ramp` is a fixed `Option<Option<u32>>`.
  At most one kept base exists per pending epoch, so the count of held bases is bounded by the
  pending-provider list (`PENDING_PROVIDERS`, same file).
- **The oracle that exists.** `capi/tests/resource_lifecycle.rs` counts every allocation of the
  compile with a thread-scoped allocator and asserts the exact equality of
  `capi_retained_bytes` with its observed terms
  (`assert_capi_retained_bytes_are_complete`, `:867`; driver `:995`). It observes the compile of
  one session; it never populates a solo, a master or a pending candidate, so a term that the
  commit path adds later is outside it. The browser's ceilings are
  `hosts/host-web/tests/retained_ceilings.rs` (deleted by TA4) and `tests/browser-v1/expected.json`
  (`resourceCeilings`, `:69`).

## Decisions to take (the implementer proposes, the coordinator rules)

- **The bounds, stated as ceilings.** The overlay never holds more than one entry per strip plus
  the master: `soloed.len() <= strips` (a solo names a strip that exists; a removed strip leaves,
  `retain_strips_of`) and `master` is one ID. The ramps never hold more than the row count above.
  Both are bounded by the session's admitted caps (`maximum_tracks`, `maximum_submixes`, routes,
  VCAs in `ControlLimits`), not by the number of edits, so the charge needs no per-edit growth.
  The implementer proves each bound at the write site and adds a test that tries to exceed it.
- **Capacity versus length.** A `Vec` that grows by pushes may hold twice its length. Either the
  overlay reserves `strips` entries when a plan is prepared and the charge is exact on that
  capacity, or the charge is `2 x strips`. The proposal reserves: the charge is then a layout, and
  a solo write never allocates after the reserve. For the `BTreeMap` and the two `BTreeSet`s, the
  charge is a per-entry ceiling derived from the layouts of `Key`, `u32` and `Owner` and the leaf
  node's 11-entry layout, checked against the allocator, because a B-tree's node count is not
  exact.
- **Retained versus peak.** The retained charge is one overlay and one ramp table on the committed
  state plus, for each pending epoch, a kept overlay and a kept ramp table. The transients (the
  next overlay of `after`, the clone in `merged_ramps`) belong to the peak rows
  (`validate_replacement_peak`, `validate_live_peak`), not the retained row. The implementer
  names which row each term joins and does not charge a transient twice.
- **Where the held copies are counted.** The kept overlay and ramps join `held_models` (so both
  admissions count them with the model they belong to) or the control plane's epoch rows. The
  proposal extends `held_models` to a held-base charge, since the lifetime is the same.
- **Does the browser need its own row?** Expected no. The browser keeps the master by strip
  index in `PlanShape::master_track` and the meter header (a `u32`), a fixed field. The
  implementer states it and charges a bridge row only if a host-side copy of the overlay appears.

## Deliverables

1. The overlay and ramp charge in `capi_resources` (and `prepared_capi_resources` if the prepared
   protocol row holds a copy), as layouts over the bounds, with the reserve of the second decision
   if taken.
2. The kept base's overlay and ramps charged where its model is (after #1310 merges), in both
   admissions.
3. Re-pins of every moved row, each measured on the final tree with its reason: the capi
   `maximum_capi_retained_bytes` fixtures that sit at a measured floor, the browser
   `resourceCeilings` in `expected.json` and the `retained_ceilings.rs` budgets (deleted by TA4)
   (`scripts/check-browser-expected-resources.py` measures the browser), and the SDK or fixture
   files that carry a figure. Owner decision 4 applies: budgets stay ceilings, not exact counts,
   except the `capi_retained_bytes` equality, which stays exact.
4. The allocator oracle of gate 1.

## Authorized paths

- `crates/control-plane/src/compile.rs` and `crates/control-plane/src/control.rs` (the charge,
  the held-base term, the reserve), with their tests.
- `crates/controls/src/effective.rs` (a reserve or a capacity accessor on `MonitorOverlay`) and
  `crates/session/src/edit_ramp.rs` (a ceiling function over the row counts), with their tests.
- `crates/capi/tests/resource_lifecycle.rs`, `crates/capi/src/runtime/live_tests.rs` and
  `live_peak_tests.rs` where a floor moves; `hosts/host-web/tests/retained_ceilings.rs` (deleted by TA4),
  `hosts/host-web/tests/browser-v1/expected.json`.
- This spec file's decision record.

## Non-goals

- No new row in the C ABI report struct: the charge joins `capi_retained_bytes`.
- No change to what the overlay or the ramps hold, to their wire forms, or to the ramp merge rule.
- No release of the kept base earlier or later than #1310 decides.
- No byte-pinned test of a layout (owner decision 4); no benchmark.

## Objective gates

1. **Allocator-measured oracle.** In `crates/capi/tests/resource_lifecycle.rs`, with the
   thread-scoped allocator: populate the committed overlay to its bound (every strip soloed, a
   master set), populate the ramps to the row bound (a transaction that gives every row of every
   owner a length, a transaction that writes every row with no length so `unset` is full, and
   one that upserts every owner whole so `cleared` is full), and hold a pending candidate so a kept base holds both again (a structural
   commit followed by a live edit). Then the bytes the control plane retains for those values are
   `<=` the bytes the charge adds. The comparison names the terms observed (overlay vector
   capacity and ID text, the map's and both sets' nodes), not a byte literal.
2. The existing exact equality `capi_retained_bytes == observed terms` still holds on every
   fixture, with the new term added to the right-hand side as an observed value.
3. The same session on the browser: `WebResourceReport` reads the same core charge; the
   `check-browser-expected-resources.py` ceilings pass with the moved rows re-pinned from
   measurement, each with its reason. (`retained_ceilings.rs` was deleted by TA4: the shipped wasm32 artifact's ceilings stay in `expected.json` (checked by `check-browser-expected-resources.py`); the deleted file ceilinged only the native 8- and 4-lane builds, which do not ship.)
4. A control plane at the exact `maximum_control_retained_bytes` that the session needs without
   the overlay is refused when the overlay is populated, on both hosts, with the typed resource
   code, and accepted with the limit raised by the charge.
5. `cargo test -p control-plane --all-features`, `-p capi --all-features`, `-p controls`,
   `-p host-web --all-features`; `cargo clippy -D warnings` and `cargo fmt --check` on the touched
   crates; realtime policy check; `scripts/check-workspace-policy.sh`.

## Test value

- Gate 1: plausible defect that turns it red and no existing test catches: a populated solo set,
  a master, or a kept base's ramp table that holds heap the retained row never counted, so the
  `maximum_control_retained_bytes` limit under-reports on a large mixer. The existing oracle
  observes one compile with an empty overlay and no pending candidate, so it cannot see any of
  the three.
- Gate 4: red if the charge is added to the report but not to the admission that compares it with
  the limit (the row moves and the refusal does not).
- The re-pins are measurements, not tests; each records the base and the measured value.

## Dependencies

- #1310 (batch 15a): the kept base and its ramp merge. The overlay charge on the committed state
  does not need it, but the held-base term does.
- #1683: the master joined the overlay, so the bound is `strips + 1` IDs.

## Size

Half a day.

## Decision record: #1696

To be filled by the implementer. The bounds decision goes first.

Attempt 1 (implementer, 2026-10-24, on `main` a97b522be; stopped for a root ruling):

- D1-D4 of the scope note taken as ruled. Built: `MonitorOverlay::retained_ceiling` and a reserve
  in `after` (the solo vector is built at `next`'s strip count and a solo of a strip `next` lacks is
  skipped, so it never reallocates); `EditRamps::retained_ceiling` (std B-tree node layout, at least
  five keys per non-root node); two `epoch_rows` (`monitor.overlay`, `monitor.edit_ramps`) from
  `monitor_rows(model)` in `prepared_capi_resources`; the overlay vector joins `largest`. The boot
  seed keeps `MonitorOverlay::new()`: every commit replaces it with `after`'s reserved result, so a
  boot reserve would only allocate bytes no write uses.
- Re-anchor: #1386's `SESSION_REPLACE` keeps nothing of the document between transactions
  (`document.rs`); no new retained structure.
- **Root decision (2026-10-24, amends the non-goal "no change to what the ramps hold").**
  Attempt 1 found D1/D3 false for the kept base: its ramps are the successor's and later live
  commits', so the newest committed model bounds them, not the holding epoch's; `cleared` keeps every
  owner an upsert or removal named and `merged_ramps` unions them while a successor is pending, so
  no model bounds it; and a live VCA add moved the bound while the row followed only the catalog.
  Ruled and built:
  1. A kept base stores no `cleared` and only rows of owners the newest committed model has
     (`EditRamps::keep_within`, at both stores in `control.rs`). Nothing reads a kept base's
     `cleared` (it is only the earlier side of `overlay`), and a removed owner returns only through
     an upsert, which clears its rows in the later overlay, so no reader loses a key.
  2. The ramp row is `CapiResources::edit_ramps`, part of `active_retained` and no epoch row:
     `KEPT_BASES` (= `PENDING_PROVIDERS`, two) tables at the newest committed model's bounds. Bounded
     because a kept base sits on the epoch a pending successor succeeds, one per pending epoch. The
     replacement admission adds the candidate's row beyond the newest's.
  3. A live commit follows it (`follow_edit_ramps`, beside `follow_catalog`), and the live
     admission adds its growth (`LiveCatalogAdmission::ramp_growth_bytes`).
- `capi_retained_bytes` on the reference session: 168,818 -> 192,972 (+24,154), over 175,296;
  ceiling 212,288 (+10 %, rounded to 64).
