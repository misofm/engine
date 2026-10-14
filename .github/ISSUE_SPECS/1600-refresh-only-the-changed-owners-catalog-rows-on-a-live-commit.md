# Refresh only the changed owners' catalog rows on a live commit

Successor of #1552, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on the head of branch codex/controls-b2-fold (#1551-#1555 plus the batch 2 review fold). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A live fader move on a large session no longer rebuilds the whole control list. The commit
re-reads only the rows of the owners whose values changed, plus the derived values. The list a
client reads, and its `structure_epoch`, stay exactly what a full rebuild would give.

## Context

- **Today, every live commit builds a full catalog.** `commit_live`
  (`crates/control-plane/src/control.rs:1361`) calls `host_core::ControlCatalog::build` on the
  committed model before its first write (step 1b, `:1379-1385`) and publishes it after the commit
  with `replace_session_catalog` (`:1602-1604`). `build`
  (`crates/host-core/src/control_provider.rs:533`) runs one `controls::list` pass over every
  owner, allocates every wire row, and is held beside the current catalog until the publish. The
  cost is O(all controls) per fader move plus a transient full copy. That copy is not part of the
  double-live admission (#1552 decision record, "Cost").
- **The epoch rule.** `replace_session_catalog` (`control_provider.rs:237-243`, unchanged by the
  fold) advances
  `structure_epoch` when any address, header or descriptor differs; a values-only commit keeps it
  (#1552 decision record, binding). A live `matrix_mode` switch flips `active` and advances it.
- **No per-owner row pass exists.** `controls::owner_rows` is gone: #1553 replaced it with
  `resolve` (`crates/controls/src/registry.rs:179`, one row by address, used by `get` `:203` and
  `set` `:265`), and `list` (`:61-90`) computes `effective(model, overlay)` once for every owner.
  The pieces a per-owner pass needs are `strip::push_rows`, `route::push_rows`, `vca::push_rows`
  and `session_rows::push_rows`, which `list` calls with one `Effective`
  (`crates/controls/src/effective.rs:226`).
- **Derived rows cross owners.** A strip's effective fader and mute rows
  (`crates/controls/src/strip.rs:146-170`) and a route's gate rows
  (`crates/controls/src/route.rs:211-216`) are `Derived`: a VCA move, a strip mute or a follow
  change alters other owners' values.
- **Session settings change descriptors elsewhere.** A `control_smoothing` change is family
  `Setting` (live, no record) but changes `default_ramp_samples` on rows of every owner
  (`LiveRamps::for_session`, `control_provider.rs:561`; `control_row` reads it at `:764-770`).
- **The classifier.** #1554 is merged. `classify_live_delta`
  (`crates/host-core/src/live_delta.rs:199`) compares the two models' rows with
  `controls::changed_controls` (`crates/controls/src/diff.rs:31`; each `ChangedControl` is an
  address and its later descriptor, derived rows skipped), groups the changes into one work item
  per strip (`Work`, `:329`), and lowers each changed owner and family once (`lower`, `:401`).
  A test-build counter counts the lowered owners (`take_lowered_owner_count`, `:391`). The VCA
  refusal (`LiveRebuild::Vca`, `:206`) stays until #1558.
- **Raised ceilings.** The full-list catalog raised the C ABI reference ceilings to 554,240
  (`capi_retained_bytes`) and 278,912 (`largest_named_allocation_bytes`)
  (`crates/capi/tests/resource_lifecycle.rs:1279-1286`; #1552 decision record).
- **Compact rows.** #1599 (the compact catalog) gives the catalog an
  `owners` table, a `sections` table, shared `descriptors` and fixed-size `rows`, so one owner's
  rows are one contiguous run.

## Decisions frozen for this slice

- **D1. The delta names the changed owners.** `LiveDelta` gains `changed_owners`: the list-order
  indices of every owner with at least one row whose value bits differ between the two models, of
  any class and family, `Derived` rows excluded. It also gains `settings_changed: bool`, true when
  a `Setting`-family row differs. Both come from the `controls::changed_controls` result that
  `classify_live_delta` already computes (`live_delta.rs:238`); no second diff runs.
- **D2. One owner's rows from one effective pass.** `crates/controls` exports
  `owner_rows_with(model, &Effective, owner_index, sample_rate, out: &mut Vec<ControlRow>)` (name
  is the implementer's choice), which appends one owner's rows and takes the effective values
  computed once by the caller. `list` calls it for every owner, so there is one path.
- **D3. Stage before the first write.** In step 1b, `commit_live` calls
  `ControlCatalog::stage_live(&current, next, sample_rate, &delta)` (host-core). It returns one of:
  - `Values(staged)`: when `settings_changed` is false and every changed owner's new rows have
    the same addresses and descriptors (bit-exact) as its stored rows. `staged` holds, in a
    vector reserved exactly, the `(row index, value)` pairs of the changed owners' rows, followed
    by the new value of every `Derived` row of the catalog, read from the one `effective(next,
    &MonitorOverlay::default())` pass. Its cost is O(changed owners' rows + derived rows +
    strips + routes), not O(all controls).
  - `Full(catalog)`: otherwise, `ControlCatalog::build(next, rate)`, as today.
  A failure is `CommandError::Internal`, as today; nothing has been written.
- **D4. Publish after the commit, without allocation.** Where `commit_live` publishes today, the
  provider applies the staged result: `Values` writes each pair into `values` and sets the
  revision, keeping the epoch; `Full` calls `replace_session_catalog`, which keeps the #1552 rule
  (it advances the epoch exactly when a descriptor, header or address differs). Neither path
  fails or allocates.
- **D5. The epoch rule is exact.** `Values` is chosen only when the catalog after it equals, in
  every address, header and descriptor, the catalog a full build would give: unchanged owners
  have no differing value, so their descriptors cannot differ unless a setting changed, which
  forces `Full`. A `debug_assert!` under `test-support` builds the full catalog in the `Values`
  path and checks that it equals the patched one, values included.
- **D6. Rebuild commits are unchanged.** A rebuild still builds a full catalog in preparation and
  publishes it with `replace_session_catalog` (`control.rs:1290-1303`).
- **D7. A counter for the test build.** Under `test-support`, host-core counts the owners
  `stage_live` reads and the number of `Full` results, so a test can show that one fader edit reads
  one owner.

## Deliverables

1. D2 in `crates/controls/src/registry.rs` (and `lib.rs` export).
2. D1 in `crates/host-core/src/live_delta.rs`.
3. D3-D5 and D7 in `crates/host-core/src/control_provider.rs`.
4. D3-D4 in `commit_live` (`crates/control-plane/src/control.rs`).
5. The tests below; `docs/C_ABI_V1_QUALIFICATION.md:236-252` and `:543-546` describe the staged
   refresh in two sentences.

## Authorized paths

- `crates/controls/src/{registry.rs,lib.rs}`.
- `crates/host-core/src/{live_delta.rs,control_provider.rs,lib.rs}`.
- `crates/control-plane/src/control.rs` (`commit_live` only).
- `crates/capi/src/runtime/live_tests.rs` (new tests).
- `docs/C_ABI_V1_QUALIFICATION.md` (the two ranges above).

## Non-goals

- No change to #1554's own list passes in the classifier; if they show up in a measurement,
  record a successor.
- No change to the rebuild path, the wire, the page builder or the compact layout.
- No change to the double-live admission. `Full` keeps today's transient copy, which is rare (a
  `matrix_mode` switch or a settings change).
- No change to the browser.

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi`.

1. **Same list as a full rebuild.** On `control_list_session()`
   (`crates/capi/src/runtime/live_tests.rs:3937`), for each of these live commits: one track
   `fader_db` lane, one submix `mute`, one console EQ parameter, one insert bypass, a VCA fader (if
   #1558 has merged; else skip with a note), two edits on two strips in one transaction, and a
   `control_smoothing` `fader_ms` change: every page at `limit` 256 equals, entry by entry, the page
   of a provider built fresh from the committed model, and `structure_epoch` equals what the #1552
   rule gives (kept for the values, advanced for the smoothing change).
2. **Derived rows follow.** A strip `mute` edit on a track with a route into a submix that follows
   its mute: the route's gate row value is the new effective one, though the route owner is not in
   `changed_owners`.
3. **Epoch.** `a_live_matrix_mode_switch_advances_the_epoch_and_a_fader_move_does_not`
   (`live_tests.rs:4280`) stays green; the `matrix_mode` switch takes `Full` (D7's counter).
4. **One owner.** One `fader_db` edit on a 64-track session with 4 submixes reads exactly one
   owner and takes no `Full` (D7's counter).
5. **No ack before a drop.** `audit capi` reports 0 allocations, frees, locks and syscalls in
   render. A transaction whose staging fails (inject a failure in the test build) commits nothing
   and writes no cell.

## Test value

- Gate 1 turns red if the patched list differs from a full rebuild in any value or descriptor, or
  the epoch moves when it should not, or stays when it should move.
- Gate 2 turns red if derived values are refreshed only for changed owners.
- Gate 3 turns red if a descriptor change is patched as values only.
- Gate 4 turns red if the live commit still reads every owner.
- Gate 5 turns red if staging moves after the first cell write.

## Dependencies

- #1554 (merged: the registry classifier and its row diff; D1 extends it).
- #1599 (the compact catalog), which edits `control_provider.rs`
  first and gives each owner one contiguous run of rows.
- #1552 (merged in batch 2).

## Size

Half a day to one day, one sonnet-high implementer.

## Decision record

Built on batch 5's head (#1599 compact catalog, #1558 VCA add/remove live, #1559 monitor overlay,
#1561). Adaptations to the spec as written:

- **D1 as written, plus the owner-list check.** `LiveDelta` gains `changed_owners` (ascending
  list-order owner indices, from the `changed_controls` result that the classifier already
  holds, no second diff) and `settings_changed`. A VCA added or removed has no row in that diff
  (#1558), so `stage_live` compares the catalog's owner list (kind and ID, in list order) with the
  model's and takes `Full` when they differ.
- **D2: `controls::owner_rows_with(model, overlay, effective, owner, rate, out)`.** It takes the
  overlay as well as the effective values, since a strip's `solo` row reads the overlay. `list` is
  a loop over it. `controls::derived_rows_with` appends only the derived rows of a strip or a route
  (their effective fader, mute and gate), from the same effective pass.
- **D3, derived rows.** The staged writes are the changed owners' rows, then the derived rows of
  every strip and route, all from one `effective(next, overlay)` of the monitor overlay the commit leaves. Derived rows cross
  owners (a VCA move, a followed mute), so they are read for every strip and route, not only for
  the changed ones. They are found in each owner's first section by walking its few own rows after
  an O(log n) binary search of the owner's run (owner indices are monotone along the entries), so
  nothing scans the whole list.
- **D3, `Values` test.** For each changed owner the fresh rows must match its stored run: the same
  section headers, the same addresses and, row by row, the same descriptor (the row's `Shape`
  turned into a wire descriptor and compared by bits with the stored one, memoised per stored
  descriptor so an owner with 300 rows builds a handful). A `matrix_mode` switch therefore takes
  `Full` by itself (its pan and matrix rows flip `active`). A `control_smoothing` change takes
  `Full` (it moves `default_ramp_samples` of every owner), by `settings_changed`.
- **D4/D5.** `SessionControlProvider::publish_staged_catalog` writes the pairs (no allocation,
  cannot fail, keeps the epoch) or calls `replace_session_catalog`, which keeps the #1552 epoch
  rule. In a test build (`cfg(any(test, feature = "test-support"))`, debug assertions on) the
  `Values` path builds the full catalog and asserts that it has the same structure and the same
  values, bit for bit, as the patched one.
- **D7.** `host_core::take_stage_counts()` returns the changed owners read and the full catalogs
  staged since the last call; `host_core::fail_next_stages(n)` makes the next `n` stages fail, for
  gate 5 (a typed `CommandError::Internal`, before any write).
- **Admission charge (acked-batch and double-live).** The catalog is staged inside the live
  admission, before the first write, as the full build was. A `Values` stage holds one small
  vector and no table, so it is charged as no growth and the current catalog's largest allocation;
  a `Full` stage is charged exactly as before (growth and largest allocation of the new
  catalog). The published figures (`follow_catalog`, the report row) read the provider's catalog
  after the publish, so the two paths use one rule.
- **Monitor overlay (adapted onto batch 4's #1559/#1603, rebased onto `a1c143de3`).**
  `ControlCatalog::stage_live`, `stage_live_catalog` and `live_admission` take the
  `MonitorOverlay` the commit leaves (`monitor_change.overlay`, the one `ControlCatalog::build`
  and `live_admission` already read), not the default one. The derived rows, the changed owners'
  rows (a strip's `solo` row) and the test-build assertion (patched catalog equals the full build)
  all read it. A solo has no model row, so `changed_controls` never names the soloed strip:
  `classify_live_delta_with_monitor` adds every strip whose solo differs between `monitor.committed`
  and `monitor.next` to `changed_owners`. A solo-only transaction therefore reads the soloed strip
  as a changed owner, refreshes every derived row (the solo-muted strips' effective mutes, the
  sends that follow them) and takes no full catalog.
  Gate: `a_solo_only_commit_refreshes_the_solo_row_and_the_derived_rows`, which is red when the
  stage reads the default overlay or when the classifier skips the soloed strip. The one test that
  used the retired `SessionEdit::SetControlSmoothing` now writes the three session rows through
  `SET_CONTROL` (`smoothing_edits`, #1561).
- **Cost.** On a 64-track, 4-submix, 6,373-entry session, one `fader_db` edit: the full catalog
  build is 1.22 ms and `stage_values` is 54 microseconds (release, one run, descriptive; no
  transient full copy). The classifier's two `controls::list` passes remain (a non-goal; not measured here).
- **No change under `crates/engine/src/realtime/`.**
- **Gate 1 VCA row.** The VCA offset case runs through the C ABI (a VCA owner is a changed owner of
  its own and its members' derived rows are refreshed).

**Batch 6 review fold, 2026-10-14.**

- **Edits outside the slice's crates, recorded.** The slice's spec names `host-core`, `control-plane` and
  `controls` registry and list code; the commits also edit:
  - `crates/controls/src/route.rs`: `push_derived_rows`, with `push_route_rows(.., derived_only)`
    behind `push_rows` (the rows of class `Derived`, in listing order).
  - `crates/controls/src/strip.rs`: `push_own_rows(.., derived_only)` extracted from `push_rows`, and
    `push_derived_rows` on it. `push_rows` lists exactly what it did.
  - `crates/controls/src/registry.rs` and `lib.rs`: `derived_rows_with`, and, in this fold,
    `owner_index`.
  - `crates/session/src/model.rs` (commit `c71b1dcac`, #1400b): `StripRef::track` and
    `StripRef::submix` are public, so a caller finds a strip by its index in O(1) (the staged-write
    path of the provider does) rather than by `strips().nth`.
- **One owner order.** `live_delta.rs` kept its own `owner_position`, a second copy of the order that
  `owner_rows_with` walks (strips, routes, VCAs, session). `controls::owner_index(model, &Owner)` is
  now the inverse of that order, in the crate that owns it, and the classifier calls it;
  `owners_stream_one_run_each_in_list_order` checks that each owner's address names the index it was
  streamed at.
