# Hold the control catalog in shared descriptors and small rows

Successor of #1552, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `31e6093c2` (branch codex/controls-s3). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

The C ABI's control list costs much less retained memory. Each entry no longer stores a full wire
row. Rows that share one descriptor point to one stored copy, and owner and section strings are
stored once. A client receives the same bytes as today on every page.

## Context

- **The catalog today.** `ControlCatalog { entries: Vec<ControlEntry>, values: Vec<f32> }`
  (`crates/host-core/src/control_provider.rs:57-60`). Each entry is a complete wire
  `ControlEntry` (`crates/protocol/src/message_wire.rs:535-542`): its address owns its
  `owner_id` and `slot_id` strings, and each control row (`message_wire.rs:588-640`) owns its
  `name`, `display_name`, `display_unit`, `reopen`, `ladder` and `choices`. The row's `value` is
  stored as `0.0`; the committed value is in `values`.
- **The build.** `ControlCatalog::build` (`control_provider.rs:533-693`) runs one
  `controls::list` pass and inserts owner and section headers (`openings`, `:697-705`). It converts
  each row with `control_row` (`:750-863`) and `wire_address` (`:719-746`). The source
  `controls::ControlDescriptor` holds `&'static` strings and slices
  (`crates/controls/src/descriptor.rs:89-127`), so the copies are made only for the catalog.
- **Sharing.** Rows of one table share a descriptor. In the reference session, the nine tracks'
  console EQ rows repeat one EQ descriptor per parameter, and every strip repeats the builtin and
  own-row descriptors. Two fields vary per session or per owner: `default_ramp_samples` (resolved
  from the session's `control_smoothing`, `LiveRamps::for_session`, `:561`) and `active` (a strip's
  `matrix_mode`).
- **Pages.** `window` (`:253-282`) finds the index range. `control_list` (`:338-361`) clones each
  entry and fills `value` in. `control_state` (`:363-387`) reads `values` only.
- **The epoch rule.** `replace_session_catalog` (`:237-243`) advances `structure_epoch` when
  `self.catalog.entries != catalog.entries`: an address, a header or any descriptor field differs.
  A values-only commit keeps it. This is #1552's binding decision record entry.
- **Automation.** `ControlProvider::control_row` returns `&ControlEntry`
  (`crates/protocol/src/controller.rs:584-587`; mock `:750`). `validate_automation_domains`
  (`controller.rs:3482-3510`) reads only the body: `automatable`, `kind`, `minimum`, `maximum`,
  `choices`. The provider's filter (`control_provider.rs:389-410`) reads `family` and
  `address.control`.
- **Resources.** `resource_report` (`control_provider.rs:81-179`) charges the entries, values,
  choices, ladders and strings. `crates/capi/tests/resource_lifecycle.rs` pins the reference
  session: `capi_retained_bytes` ceiling 554,240 (measured 503,819) and
  `largest_named_allocation_bytes` ceiling 278,912 (measured 253,504 = 932 entries x 272 B, the
  `entries` vector) (`:1286-1295`; history `:1165-1175`).
  `prepared_parameter_catalog_charge_covers_its_allocations` (`:1036-1050`) checks that the charge
  equals the bytes freed by dropping the catalog.

## Decisions frozen for this slice

- **D1. Three tables and two vectors.** `ControlCatalog` holds:
  - `owners`: one record per owner, in list order: owner kind, owner ID string (absent for the
    session), the header kind, name and route ends.
  - `sections`: one record per section, in list order: the owner index, the section kind, the slot
    ID string (console and inserts only), the header name and the effect identity.
  - `descriptors`: the distinct wire `ControlRow` values (with `value = 0.0`), each stored once.
  - `rows`: one fixed-size record per entry: the entry kind (owner header, section header or
    control), the section index (which gives the owner), and for a control the control ID, the
    lane and the descriptor index. No record holds a heap allocation.
  - `values: Vec<f32>`, in step with `rows`, as today.
  The exact field widths are the implementer's choice; a record of 16 bytes or less is the target.
- **D2. Deduplication is bit-exact.** Two control rows share a descriptor index only when every
  field of their wire descriptors is equal, with every `f32` compared by `to_bits`. A strip whose
  `active` differs, or a row whose `default_ramp_samples` differs, gets its own descriptor. The
  build may use a transient index (a hash map or a sorted vector) to find duplicates; it is freed
  before `build` returns and is not retained. `descriptors` is filled in first-occurrence order.
- **D3. Pages build wire entries.** `control_list` builds each `ControlEntry` of the page from the
  tables: the handle, the address from the owner and section records plus the row's control and
  lane, and the body from the header records or a clone of the descriptor with `value` from
  `values`. The page bytes are identical to #1552's for every request. `control_state` is
  unchanged in effect. `window`'s owner filter reads the owner index from `rows`, not strings.
- **D4. The epoch rule is kept exactly.** `replace_session_catalog` advances the epoch when the
  new catalog's `owners`, `sections`, `rows` or `descriptors` differ from the current ones, `f32`
  by bits. Because both builds fill `descriptors` in first-occurrence order, this comparison is
  equal exactly when every entry's address, header and descriptor are equal: the #1552 rule. A
  values-only commit keeps the epoch.
- **D5. `control_row` returns the descriptor.** The trait method becomes
  `fn control_row(&mut self, handle) -> Result<&ControlRow, ParameterProviderError>`, a borrow of
  the shared descriptor (its `value` is `0.0`; no caller reads it). The provider's effect-only
  filter is unchanged and reads `family` from the descriptor and the control ID from the row
  record. `validate_automation_domains` loses its `ControlBody::Control` match. The refusal codes
  are unchanged.
- **D6. Resources.** `resource_report` charges each table by length and each string, ladder and
  choice vector once, where it is stored. Every vector stays `try_reserve_exact`, so
  `prepared_parameter_catalog_charge_covers_its_allocations` stays exact. The reference ceilings
  move to the new measured values plus 10 %, rounded up to 64, with a doc-comment paragraph in
  `resource_lifecycle.rs` that records before and after (the file's convention).
- **D7. No other change.** `ControlCatalog::build`'s signature, `structure_epoch`, paging, the
  owner filter, page trimming in the controller, and live and rebuild commits stay as they are.
  A live commit still builds a full catalog (its successor, the live-refresh slice, removes that).

## Deliverables

1. D1-D4 and D6 in `crates/host-core/src/control_provider.rs`.
2. D5 in `crates/protocol/src/controller.rs` and the provider.
3. The new ceilings and their record in `crates/capi/tests/resource_lifecycle.rs`.
4. The tests below.
5. `docs/C_ABI_V1_QUALIFICATION.md:234-266` names the compact layout in one sentence.
6. The wire comparison and the before and after byte counts in this spec's decision record.

## Authorized paths

- `crates/host-core/src/control_provider.rs`.
- `crates/protocol/src/controller.rs` (D5 only).
- `crates/capi/tests/resource_lifecycle.rs` (the two ceilings and their record only).
- `docs/C_ABI_V1_QUALIFICATION.md` (`:234-266` only).
- This spec's decision record.

## Non-goals

- No wire change. No change to `controls`, to `ControlListPage` or to the TLV encoding.
- No change to which commits build a catalog (the live-refresh successor).
- No interning across sessions or a global descriptor table.
- No change to the browser.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p protocol`, `cargo test --locked -p control-plane --features test-support`,
`cargo test --locked -p capi`.

1. **Same wire bytes (PR evidence).** Before the change, on `31e6093c2` or the branch's parent, and
   after it, encode every response frame for these requests and compare SHA-256 digests of the
   concatenated bytes: `CONTROL_LIST_GET` and `CONTROL_STATE_GET` at `limit` 1, 7 and 256 from
   handle 0 to `eof`, and one owner filter per owner kind; on two sessions, the reference session
   of `resource_lifecycle.rs` and `control_list_session()`
   (`crates/capi/src/runtime/live_tests.rs:3892`), each also after a live `matrix_mode` switch. The
   digests are equal. The harness is a scratch test, not committed (AGENTS.md: a one-time "no bit
   moved" comparison is PR evidence). Record the digests and the base commit in the decision
   record.
2. **Distinct descriptors stay distinct.** A new host-core test builds a catalog from a session
   with two tracks of different `matrix_mode`, a submix, a route into the submix and one into the
   output, a VCA, one console EQ and two different inserts. For every pair of control entries of a
   full page, their page descriptors are equal exactly when the `control_row` conversions of the
   two `controls::list` rows are equal; and the descriptor table is shorter than the control rows.
3. **Epoch rule.** The existing tests stay green unchanged:
   `a_commit_refreshes_values_and_only_a_changed_address_sequence_advances_the_epoch`
   (`control_provider.rs:1438`), `a_structural_commit_advances_the_epoch_and_a_value_only_rebuild_keeps_it`
   (`live_tests.rs:4114`), `a_live_matrix_mode_switch_advances_the_epoch_and_a_fader_move_does_not`
   (`live_tests.rs:4235`) and `c_commands_publish_the_replacement_control_list`
   (`crates/capi/src/runtime/tests.rs:487`). A new host-core test changes only the session's
   `control_smoothing` `fader_ms` (which changes `default_ramp_samples` of every fader row) and
   asserts the epoch advances; a second catalog of the same model keeps it.
4. **Memory.** `prepared_parameter_catalog_charge_covers_its_allocations` is exact. On the
   reference session, the catalog's `catalog_retained_bytes` is at most half of its value at
   `31e6093c2`, and `largest_named_allocation_bytes` is below 253,504. Record both values before
   and after, at eight lanes (and four if an AArch64 run is available).
5. **Automation.** The existing automation tests stay green: an effect parameter handle is
   admitted, a fader handle is refused `INVALID_FIELD` with `automation.target.unsupported`.

## Test value

- Gate 1 turns red if a page differs by one byte: a wrong address string, a merged descriptor, a
  lost value, or a changed handle.
- Gate 2 turns red if deduplication merges two descriptors that differ in one field (`active`,
  `default_ramp_samples`, `reopen`), or splits nothing.
- Gate 3 turns red if the epoch comparison reads descriptor indices that a rebuild renumbers, or
  misses a descriptor change.
- Gate 4 turns red if a table is charged twice or not at all, or the compaction saves nothing.

## Dependencies

- #1552 (merged on `codex/controls-s3` at `31e6093c2`).
- Must land before #1600 (the live catalog refresh), which edits the same
  file.

## Size

Half a day to one day, one sonnet-high implementer.

## Decision record

- (empty)
