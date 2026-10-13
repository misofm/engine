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

- (empty)
