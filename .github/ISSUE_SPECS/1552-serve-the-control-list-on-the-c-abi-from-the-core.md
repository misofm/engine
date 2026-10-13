# Serve the control list on the C ABI from the core

Slice S3b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C4),
second half: the service. Split from the planner's S3. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

An iOS or Android app lists every control of the running session through
`miso_engine_v1_submit_command`: strips, builtins, console slots, inserts, routes, VCAs and session
settings, with descriptors and committed values, in pages. The list and its values are built by the
core from the committed model, so they can never disagree with what `controls` validates. A client
that holds the list polls values cheaply with `CONTROL_STATE_GET`.

## Context

- **The catalog today.** `PreparedSessionControlCatalog { parameter_metadata, parameter_state }`
  (`crates/host-core/src/control_provider.rs:47-50`) is built per preparation from the prepared
  effect entries (`build_parameter_catalog`, `:475-579`: one row per effect parameter and lane,
  handles from 1 in entry order), behind the `control-provider` feature
  (`crates/host-core/src/prepare.rs:1662-1664`). `SessionControlProvider` (`:301`) pages it
  (`:306-357`) and accounts its bytes (`resource_report`, `:67`).
- **Commits.** A rebuild commit replaces the catalog (`replace_session_catalog`,
  `crates/control-plane/src/control.rs:1300-1303`; the candidate is carried in
  `PreparedRuntime.control_catalog`, `crates/control-plane/src/compile.rs:46`). A live commit keeps
  handles and writes values only: `commit_live` finds each handle by a linear scan
  (`provider.parameter_handle`, `control.rs:1521-1529`; `control_provider.rs:218`) and writes it
  after the commit (`set_parameter_value`, `control.rs:1616-1621`; `control_provider.rs:243`).
- **Automation domain checks.** `validate_automation_domains`
  (`crates/protocol/src/controller.rs:3358-3390`) reads the row by handle: automatable and not
  `None`, then the value's domain. Every commit cancels queued automation (`:1877-1879`,
  `:2005-2007`, `:2931-2933`), and a batch needs the exact revision (`:2962-2964`).
- **The wire** is #1551's; the rows are `controls::list` (#1546, #1547's `MonitorOverlay`, #1549).

## Decisions frozen for this slice

- **D1. One catalog, built from the core.** `PreparedSessionControlCatalog` is replaced by
  `ControlCatalog`: the entries of `controls::list(model, rate, &MonitorOverlay::default())` with
  #1551's header rows inserted (one owner header before each owner's rows, one section header before
  each section's rows), each with its handle from 1 in that order, and a parallel `values: Vec<f32>`.
  `build_parameter_catalog` is deleted. Preparation builds the catalog from the compiled model
  (still behind `control-provider`).
- **D2. `structure_epoch`.** The provider keeps a `u64` epoch, `1` for the first catalog. A commit
  whose new catalog's address sequence (header and control addresses, in order) differs from the
  current one advances it by one; any other commit keeps it. A live commit never changes the
  address sequence, so it never advances it.
- **D3. Values on every commit.** After every commit, live or rebuild, the provider refreshes
  `values` from the committed model through one `controls::list` pass (`controls::get` per row is
  equivalent). `parameter_handle` and `set_parameter_value` and `commit_live`'s readback loop are
  deleted. The refresh runs after the protocol commit, on the control thread; it allocates nothing
  beyond the `list` iterator's transient state.
- **D4. Paging.** `control_list` and `control_state` answer `structure_epoch = 0` from the first
  handle after `after_handle`; a non-zero epoch other than the current one is `Stale`. A page holds
  at most `limit` entries and sets `eof` on the last. An `owner_kind`/`owner_id` filter keeps that
  owner's header, section headers and rows, with their global handles. The page's `revision` is the
  committed session revision.
- **D5. Automation.** `control_row(handle)` serves `validate_automation_domains`. Until #1567, an
  `AUTOMATION_ENQUEUE` record whose handle names a row other than an effect parameter is refused
  with `INVALID_FIELD` (`automation.target.unsupported`), so the accepted set is today's.
- **D6. Capability.** The C ABI's controller advertises flag bit 7 and the two messages
  (`ProviderFeatures::ALL`, `compile.rs:870`).
- **D7. Resources.** The catalog's retained bytes (rows, strings, values) are charged where the old
  catalog's were (`resource_report`); ceilings stay, and any exact pin re-derives from the same
  function, never from a literal.

## Deliverables

1. D1-D5 in `crates/host-core/src/control_provider.rs` and the preparation call.
2. D2-D3 in `crates/control-plane/src/{compile.rs,control.rs}`; D6.
3. The tests below; `tools/audit/src/protocol.rs` sends `CONTROL_LIST_GET` in its corpus.
4. `docs/C_ABI_V1_QUALIFICATION.md:234-266` and `:529-531` describe the new catalog.

## Authorized paths

- `crates/host-core/src/{control_provider.rs,prepare.rs,lib.rs}`, `crates/host-core/Cargo.toml`.
- `crates/control-plane/src/{compile.rs,control.rs}`.
- `crates/protocol/src/controller.rs` (D5's refusal only).
- `crates/capi/src/runtime/{tests.rs,live_tests.rs}`, `crates/capi/tests/**` (resource pins only),
  `tools/audit/src/protocol.rs`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No browser or SDK service (`engine.controls()` arrives with #1562 and #1564).
- No solo overlay (#1559 fills it); derived rows show `MonitorOverlay::default()`.
- No `SET_CONTROL` (#1553).

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi` and
`./target/release/audit protocol`.

1. **Full list.** On a capi session with two tracks, a submix, a send, a `pre_fader` and a
   `post_fader` output route, a VCA, one console slot and one insert: paging with limit 1 and with
   limit 256 returns the same entries; every address `controls::list` yields appears once, after its
   owner and section headers, in C4's order; the page `revision` equals the session revision.
2. **Live commit.** After a live fader commit, the fader row's `value` is the new value, the
   `structure_epoch` and every handle are unchanged, and `CONTROL_STATE_GET` returns the same value.
3. **Structural commit.** After adding an insert, the epoch advances by one; a request with the old
   epoch is `REVISION_CONFLICT` with `control.list.stale`; a rebuild that changes only a
   `rebuild_only` value (`delay_samples`) keeps the epoch.
4. **Automation.** An `AUTOMATION_ENQUEUE` on an effect parameter handle is admitted as before; one
   on the fader row's handle is refused `INVALID_FIELD` with `automation.target.unsupported`.

## Test value

- Gate 1 turns red if the list misses builtins, routes, VCAs or session rows, or paging drops or
  repeats an entry.
- Gate 2 turns red if a live commit leaves a stale value or moves handles.
- Gate 3 turns red if a structural change keeps old handles valid, or a value-only rebuild
  invalidates a client's list.
- Gate 4 turns red if automation is acked for a row nothing renders.

## Dependencies

- #1551 (the wire). #1546, #1547, #1549.

## Supersedes and shrinks

- None directly; this is the discovery half of decision 17.

## Decision record

- **Epoch rule (coordinator, binding; supersedes D2 and the live-commit sentence of D3).**
  `structure_epoch` advances by one on every commit, live or rebuild, after which the catalog's
  address sequence, any header, or any control row's descriptor (every field except `value`)
  differs from the previous catalog. A client therefore never holds a stale descriptor under a
  current epoch. A live commit builds the committed model's catalog before its first write and
  publishes it after the commit with the same `replace_session_catalog` a rebuild uses, so a live
  `matrix_mode` switch (which flips `active`) advances the epoch and a fader move (values only)
  does not. Tests: host-core `a_commit_refreshes_values_and_only_a_changed_address_sequence_advances_the_epoch`
  and capi `a_live_matrix_mode_switch_advances_the_epoch_and_a_fader_move_does_not`.
- **Cost.** A live commit now builds a full catalog (one `controls::list` pass plus the wire rows) and
  holds it beside the current one until the commit publishes it; that transient copy is not part of
  the double-live admission. Recorded as a successor with the compact, deduplicated row storage.
- **Page trimming.** The controller trims a page to the endpoint's response bound (`eof` false,
  cursor at the last row kept; one row that cannot fit is `LIMIT_EXCEEDED`).
  Batch 2 review fold: the page codec reports a page whose content breaks the message rules
  (unordered handles, a non-finite value, an address that disagrees with its row) as the new
  `EncodeError::InvalidField`, and only a row count above 256 or a frame bound as `LimitExceeded`;
  the controller trims on the second and answers the first as `INTERNAL`, since no page size
  repairs the provider's defect. The trim estimate multiplies in `u64`.
- **Raised C ABI reference ceilings (batch 2 review fold).** `crates/capi/tests/resource_lifecycle.rs`
  moved `capi_retained_bytes` 282,432 -> 554,240 and `largest_named_allocation_bytes` 99,840 ->
  278,912, at both lane widths (#1551 raised them to 386,112 and 161,600 for the effect-parameter
  catalog's wire rows; #1552 raised them again). The cause is structural, not tuning: the catalog now holds every
  control of the session (the nine tracks' builtins, console EQs, own rows and routes, the session
  rows, and a header before each owner and section: 932 entries of 272 bytes) where it held the
  effect parameters alone, so the one catalog allocation went 146,880 -> 253,504 bytes and C ABI
  retained 351,003 -> 503,819 (x86-64). Each ceiling is the measured value plus 10 percent, rounded
  up to 64, as every ceiling in that table is. #1599 (shared descriptors and small rows) is the
  successor that removes the per-entry copy and lowers them.

