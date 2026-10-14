# Commit a live-only transaction without cloning and compiling the model

Decision 17 flip batch, root ruling A of 2026-10-14. Code anchors verified on `9666949b3`.
Issue number: assigned at filing (written here as #TBD10). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review. The allocation gate below is mandatory all the same.
Builds on #TBD9 (the fast classifier, `host_core::classify_live_edit`).

## Product outcome

A transaction whose edits are all live control rows is committed in time that depends on its
edits, not on the session: no clone of the model, no compile, no canonical JSON, no whole-model
effective pass. The C ABI and the browser run the same function, so the browser's one-instance
mode (control work in the worklet's message handler, on the audio thread) stops paying 267 us
(64 tracks) to 1.11 ms (256 tracks) per fader edit, and the C ABI's control thread stops too. The
committed session after any run of such commits is what the full path would have produced for the
same transactions, byte for byte: model, canonical JSON, resource estimate, revision, replay entry,
events, watermark, control catalog. What the fast path does not cover goes through the full path.

## Context

- **The measured cost after #TBD9** (native release, x86-64, one fader edit, one source, route and
  no insert per track, 200 calls; `hosts/host-web/tests/control_command_timing.rs`): 267 us at 64
  tracks and 1.11 ms at 256 for the whole `control_command`. Of that, the protocol's
  `prepare_transaction` is 206 us and 804 us (the model clone at
  `crates/protocol/src/model.rs:1090`, then `compile_session`: validate, canonical JSON, a second
  model clone for the normalized form, sort, estimate). The rest is `live_admission` (`check_counts`
  over the prospective model) and `ControlCatalog::stage_values`
  (`crates/host-core/src/control_provider.rs:1344`: `lists_owners_of`, `controls::effective` over
  the whole model, and every strip's and route's derived rows).
- **The one path.** Both hosts reach `SessionState::command` (`crates/control-plane/src/control.rs:1177`):
  `prepare_command_frame` (protocol: decode, `plan_structural_command`, `prepare_transaction_with`,
  `prepare_structural_plan`), then `commit_live` (`:1488`), which classifies, admits, resolves the
  producers, stamps, writes the cells, commits the token (`commit_prepared_structural`, which moves
  the prospective `CompiledSession` into the store) and publishes the catalog and the revision.
- **What a `SetControl` writes.** `controls::set` (`crates/controls/src/registry.rs`) resolves one
  row by address and writes it; it reads only the row's owner (a track, a submix, a route or a VCA)
  and the session header (`console` slots, the sample rate). Nothing else of the model is read or
  written.
- **What the compile derives.** `CompiledSession` (`crates/session/src/compile.rs:50`) holds the
  normalized model, the canonical JSON, the rate, quantum, output shape, the source index and the
  `ResourceEstimate`. For a transaction that changes values only, every field but the owners'
  values, the model's `revision`, the canonical text and the estimate's three canonical-length fields
  (`canonical_bytes`, `compiled_model_bytes`, `estimated_bytes`) is unchanged: the estimate counts
  entities, effects, parameters (a `params` entry added or removed changes it), and string bytes.
- **What the classifier and the catalog read.** After #TBD9 the classifier reads the committed
  model (`current`) for old values and the prospective model (`next`) for new ones, and only for the
  owners its rows name, through `strip_at`, `routes.get`, `vcas[..]`, `LocalInputs` and
  `controls::changed_rows`. The catalog stage reads `next` through `owner_rows_with`,
  `derived_rows_with` and `controls::effective`.
- **The recorded cost of a refusal.** A refused transaction today drops its prospective compile and
  leaves the committed `CompiledSession` untouched. Nothing about that changes.

## Decisions frozen for this slice

Binding, from root (2026-10-14):

- **R1. All or nothing first.** Every edit of the transaction is validated and lowered before the
  first write to the committed model. A refusal leaves the model, the revision, the catalog and the
  replay cache byte-identical.
- **R2. The committed model stays snapshot-able.** After any sequence of fast commits the canonical
  JSON of the model equals what the full path produces for the same transactions. The differential
  gate runs on the #1560 parity script and #TBD9's seeded generator; the full path stays as the
  oracle.
- **R3. Cost does not depend on the track count.** The allocation count of a live-only commit is
  equal at 64 and 256 tracks (thread-scoped counters, as #TBD9), plus one descriptive timing line of
  the whole `control_command` at 64 and 256 tracks.
- **R4. Only the rows the edits reach.** The catalog stage reads the derived rows of the strips and
  sends the edited rows reach (VCA members, followed sends, solo); the catalog after a fast commit
  equals a full restage (test-build assert, the existing one in `stage_live`).
- **R5. Everything else is the full path.** No ack before the commit is final. The replay cache, the
  revision, the events (`SESSION_COMMITTED`), the watermark and the D15-17 completion behave exactly
  as on the full path.

Design (this slice):

- **D1. An in-place transaction form.** `PreparedSessionTransaction` carries a form: `Compiled`
  (today's), `InPlace` (new) or `Monitor` (a transaction of solo writes only: no model change, no
  compile, no clone of the committed compilation). An `InPlace` form holds: the **owners edited**,
  as a small `SessionModel` (the session header plus clones of exactly the tracks, submixes, routes
  and VCAs the edits name, in ID order, with the edits applied), the **slots** (for each, its index
  in the committed model), the next revision, the exact next `ResourceEstimate`, and a copy of the
  edits (for D8). It owns no `CompiledSession`.
- **D2. When the protocol prepares in place.** A new `SessionStore::prepare_transaction_in_place`
  (the monitor-aware entry; `prepare_transaction` and `prepare_transaction_with_monitor` are
  unchanged and stay the oracle and the fallback) does the same cheap checks in the same order
  (exact revision, conflict, empty, revision ceiling), then tries in place. It takes the form only if
  **all** hold, else it runs the full preparation and returns that, so the full path alone reports
  every refusal and its order:
  1. every edit is a `SetControl`; a monitor row (solo) goes to `monitor_edits` as today;
  2. every model row's owner is a strip, route or VCA (not the session);
  3. `controls::set` accepts each edit, in order, on the owners-edited model (the same function, the
     same checks, the same value normalization as the full path);
  4. the session's value validators (`session::owners_values_valid`, the existing
     `validate_*` helpers for finite values, ranges, units, filter pairs) accept every edited owner;
  5. the edited owners have the same counts as the committed ones (console entries, inserts,
     parameter entries per effect, members): a `params` entry added changes the estimate and the
     canonical order, so it is the full path;
  6. `record_edit_ramps` accepts the edits and no ramp is longer than one second at the rate;
  7. the next canonical length is within the estimate's conservative bound.
  A transaction of solo writes only takes the `Monitor` form.
- **D3. The model is written once, at the commit.** `SessionStore::commit_prepared` for an
  `InPlace` form swaps each edited owner into the committed model (`CompiledSession::swap_owners`,
  by index, allocation-free), sets the revision, installs the next estimate and drops the old owners.
  Nothing writes the committed model before that: classification, admission, the catalog stage, the
  producers' preflight and the stamp all read the committed model and the edited owners as two
  parts of one **view of the next model** (D5). The commit's predicate (`check_prepared_structural`)
  already proves the base revision is unchanged.
- **D4. The snapshot and the estimate stay exact.** The canonical JSON becomes lazy:
  `CompiledSession::canonical_json()` writes it from the normalized model on first use after an
  in-place commit (`OnceLock`), and a snapshot read returns the same bytes the compile wrote. The
  estimate's canonical length is kept exact by arithmetic: the next `canonical_bytes` is the old one
  plus, per edited owner, the canonical length of its new text less its old text (the existing
  canonical writer run over that owner at its array depth, `session::canonical_owner_bytes`), plus
  the change in the digits of `revision`; `compiled_model_bytes` and `estimated_bytes` move by the
  same amount. The differential compares `resource_estimate()` and `canonical_json()` with the full
  path's after every commit.
- **D5. One view of the next model.** `host_core::NextView { structure, owners }`: `structure` is
  the committed model (counts, order, IDs, header, console slots) and `owners` the edited owners; a
  strip, route or VCA is looked up by ID in `owners` first, else in `structure`. For a model that is
  whole (`classify_live_delta*`, a rebuild's carry, `controls::effective` users) it is the model
  twice. The classifier's lowering, `LevelRules`, `LocalDiff`, `LocalInputs` and the catalog stage
  read `next` through the view; none reads it as a dense `SessionModel` any more, and the lowering
  bodies stay one body for both paths. `LiveDelta` borrows from both parts of the view.
- **D6. The classifier on the view.** `host_core::classify_live_staged(current, view, ramps,
  edit_ramps, monitor, rows) -> Option<LiveDelta<'_>>` is #TBD9's fast path over the view; `None`
  when it cannot judge. #TBD9's `classify_live_edit(current, next, .., rows)` stays (it is the
  differential's subject for two whole models) and calls the same body with the view of `next`.
  `LiveDelta` gains `reach: Option<LiveReach>`: the owner indices (strips, then routes) whose derived
  rows the classification reached, set by the fast path only.
- **D7. The catalog stage and the admission.** `ControlCatalog::stage_live` takes the view. With a
  `reach` it skips `lists_owners_of` (the transaction cannot change the owner list), computes the
  effective values of the owners it stages with `controls::LocalEffective` (the existing
  `effective_strip`, `effective_follow` and `effective_gate` rules over `LocalInputs`, no table),
  and writes only the changed owners' rows and the `reach` owners' derived rows;
  `owner_rows_with` and `derived_rows_with` become thin callers of one body over the view. Without a
  `reach` the stage is today's. `live_admission` for an in-place form skips `check_counts` (no count
  can change; a debug build asserts it) and charges `compiled_model_admission` from the committed and
  the next estimate: the same figures the full path charges.
- **D8. The fast path is optimistic.** `commit_live` takes an in-place token through the fast
  classification. If `classify_live_staged` answers `None`, the token is **materialized** (the full
  preparation of its stored edits against the committed store, `ProtocolController::materialize_prepared`)
  and the rest is today's path unchanged, a rebuild included. Materializing cannot be refused (the
  edits were accepted by D2, which only accepts what the full preparation accepts); a refusal is an
  internal fault, found before any write. A rebuild never sees an in-place token.
- **D9. One entry for the hosts.** The control plane calls
  `ProtocolController::prepare_command_frame_in_place`; `prepare_command_frame` is unchanged for the
  protocol's own callers and tests. A test-support switch forces the full preparation, for the
  differential. There is no browser-only code.
- **D10. Realtime.** Control thread only. No edit under `crates/engine/src/realtime/`; the cell
  writes, the stamp and the watermark are unchanged.

## Deliverables

1. `crates/session`: `OwnerSlots`, `CompiledSession::{swap_owners, finish_in_place}`, the lazy
   canonical JSON, `canonical_owner_bytes`, `owners_values_valid`, `counts_equal`; unit tests.
2. `crates/controls`: `OwnerEdits` (apply `SetControl`s to cloned owners), `LocalEffective`,
   `owner_rows`/`derived_rows` over a view-shaped source; unit tests.
3. `crates/protocol`: the `InPlace` and `Monitor` forms, `prepare_transaction_in_place`, the commit,
   `prepare_command_frame_in_place`, `materialize_prepared`; unit tests.
4. `crates/host-core`: `NextView`, the view-based lowering, `classify_live_staged`, `LiveReach`, the
   view-based catalog stage; the existing suites unchanged.
5. `crates/control-plane`: `commit_live` over the view, the materialize fallback, the admission, the
   test-support switch.
6. Tests: `crates/host-core/tests/in_place_commit.rs` (gate 1), `crates/capi/tests/in_place_commit.rs`
   (gate 2), `hosts/host-web/tests/control_command_timing.rs` (gate 3).
7. This spec's implementation record.

## Authorized paths

- `crates/session/src/*`, `crates/session/tests/*`.
- `crates/controls/src/*`, `crates/controls/tests/*`.
- `crates/protocol/src/{model.rs,controller.rs,lib.rs}`, `crates/protocol/src/controller/tests.rs`.
- `crates/host-core/src/{live_delta.rs,prepare.rs,control_provider.rs,lib.rs}`,
  `crates/host-core/tests/*`.
- `crates/control-plane/src/{control.rs,compile.rs,lib.rs}`.
- `crates/capi/tests/*`.
- `hosts/host-web/tests/control_command_timing.rs` (no `hosts/host-web/src`, no `sdk/`).
- This spec.

## Non-goals

- No change to a row, a family, a record, a ramp rule or the lowering's arithmetic: the core owns
  them. No change to the wire, the C ABI, `hosts/host-web/src` or `sdk/`.
- No change under `crates/engine/src/realtime/`.
- No fast path for a structural edit, a session row (`control_smoothing`), a `matrix_mode` switch,
  a `params` entry that does not exist yet, or any rebuild-only row: they take the full path.
- No second classifier, no browser-only commit.
- No benchmark framework; one descriptive timing line.

## Objective gates

Commands: `cargo fmt --check`, `cargo clippy --locked --workspace --all-targets --all-features --
-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, `cargo test
--locked` for `controls`, `session`, `protocol --all-features`, `conformance`, `host-core
--all-features`, `control-plane --features test-support`, `capi`, `host-web --features test-support`,
the realtime-policy tool, `scripts/check-protocol-wasm-parity.sh`,
`scripts/run-protocol-allocation-audit.sh`, and `audit capi` and `audit protocol` after a release
build (pcm `76525e7ef9e78cf4`).

1. **Differential.** For every transaction of (a) the #1560 parity script
   (`hosts/host-web/tests/control_parity.rs`, as replayed in `live_edit_classifier.rs`) at 48 and
   96 kHz, with its solo steps, and (b) at least 2,000 seeded transactions of one to four edits drawn
   from every row class (the #TBD9 generator), the in-place chain (`prepare_transaction_in_place`,
   `classify_live_staged`, the view-based catalog stage, `commit_prepared`) equals the full chain
   (`prepare_transaction_with_monitor`, `classify_live_delta_with_monitor`, the dense stage,
   `commit_prepared`) on: the whole `LiveDelta`, the refusal, the staged and published catalog (every
   address, descriptor and value), the committed model, `canonical_json()` and
   `resource_estimate()` after each commit, the revision, and, for a refused or abandoned
   transaction, the byte-identity of model, snapshot and estimate before and after. The test counts
   in-place commits and asserts the generator reached fader, mute, pan, matrix, trim, polarity,
   filters, effect parameter, bypass, send, VCA and solo through the in-place form and each
   fallback (`matrix_mode`, `control_smoothing`, a rebuild-only parameter, a structural edit, a new
   `params` entry) through the full one. A command-level pass drives both paths through the control
   plane's `command` (the switch of D9) on the parity script and compares the response bytes, the
   drained events and the snapshot.
2. **Cost does not grow with the session.** For one live-only transaction of each kind (fader, mute,
   pan, trim, effect parameter, send gain and mute, VCA offset and mute, solo) the allocations and
   frees of the whole `SessionState::command` (through the C ABI's control entry) on the audited
   allocator's thread-scoped counters are equal at 64 and 256 tracks (rich sessions whose VCAs and
   sends do not grow), and the in-place commit was taken. The solo that moves `any_solo` is excluded.
3. **Timing (descriptive).** One line per size, native release, the whole `control_command` of a
   fader edit at 64 and 256 tracks, before and after (`control_command_timing.rs`). No threshold.
4. The existing suites pass unchanged, in particular `crates/host-core/tests/{live_edit_classifier,
   live_delta,live_transaction,vca_live,live_routes,monitor_tick}.rs`, the control-plane and capi
   fault-injection tests (a refusal at each phase leaves model, revision, canonical, estimate,
   catalog and replay cache as they were), and the host-web control tests.

## Test value

- Gate 1 turns red if the in-place chain drops a derived row (a VCA member, a followed send, a solo),
  skews the canonical length (a float of a different text length, the digits of the revision), writes
  the model before the commit is final, accepts what the full path refuses (a value the validators
  reject, a ramp too long), or serializes the snapshot differently. No existing test compares the
  two chains on the same transaction.
- Gate 2 turns red if the commit clones, compiles, lists or walks the session: one allocation per
  strip, route or VCA moves the count with the track count.
- The fault-injection suites turn red if a refusal after classification leaves a partial write.

## Dependencies

- #TBD9 (the fast classifier and `set_control_rows`), #1600 (`changed_owners`, `settings_changed`),
  #1559 (the monitor overlay), #1562a (the control plane's one command entry), D15-17 (a submit is
  synchronous only for what can fail, the watermark, the replay cache).

## Size

Two days for one sonnet-high implementer. If it runs over, split at D7: the first commit takes the
in-place form for every transaction whose catalog stage can stay dense (it still removes the
clone and the compile), the second makes the stage local.

## Implementation record

(To be written with the implementation.)
