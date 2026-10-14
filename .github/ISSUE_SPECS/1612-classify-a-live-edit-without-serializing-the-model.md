# Classify a live edit without serializing the model

Decision 17 flip batch, root ruling A of 2026-10-14. Code anchors verified on `22268aa14`.
Issue number: assigned at filing (written here as #1612). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review. The allocation gate below is mandatory all the same.

## Product outcome

A transaction whose edits are all live control rows is classified in time that depends on its
edits, not on the session. It clones no model and writes no canonical JSON. The C ABI and the
browser call the same function, so the C ABI's control-thread cost drops too, and the browser's
one-instance mode (control work in the worklet's message handler, on the audio thread) does not
ship the 1.5 ms (64 tracks) to 6.9 ms (256 tracks) per fader edit that #1562a measured. The
classification is the one the full classifier gives: the same records, ramps, lanes, edit path and
catalog inputs. What the fast path cannot judge goes to the full path, so no edit is acked as live
and then dropped.

## Context

- **The cost.** `classify_live_delta_with_monitor`
  (`crates/host-core/src/live_delta.rs:310`) clones `next` and masks it
  (`controls::mask_values`, `crates/controls/src/mask.rs:52`; clone at `live_delta.rs:319`),
  serializes `current` and the masked model to canonical JSON and compares the bytes (`:329-335`),
  lists every control of both models (`controls::changed_controls`,
  `crates/controls/src/diff.rs:37`, two `list` passes at `:42-43`), builds both models' effective
  values and VCA reach tables (`EffectiveDiff::new`, `crates/host-core/src/prepare.rs:2894`, one
  `Vec` per strip when a VCA exists), and walks every strip and every route (`:436`, `:505`).
  #1562a's carry-forward 4 measured 2.0 ms per call at 64 tracks. On this branch (native release,
  x86-64, one fader edit on a session of one source, route and no insert per track, 200 calls):
  1.56 ms at 64 tracks and 6.93 ms at 256 tracks.
- **The one caller.** `SessionState::commit_live` (`crates/control-plane/src/control.rs:1494`)
  calls the classifier for every structural command of both hosts: the C ABI's
  `SESSION_TRANSACTION_APPLY` and the browser's command entry (#1562a) reach it through
  `SessionState::command`.
- **What the caller holds.** `prepared.get().prospective_session()` is the protocol's candidate
  after the edits, compiled and normalized (`crates/protocol/src/model.rs:1088-1155`, the loop that
  applies each edit with `apply_session_edit`). The transaction keeps its per-edit ramps
  (`edit_ramps`) and its monitor writes (`monitor_edits`) but not the list of rows its edits wrote.
- **The rows.** `controls::set` (`crates/controls/src/registry.rs:~370`) resolves one row by address
  (`resolve`, `:306`) and writes that row alone; `controls::row_descriptor` and `resolve` give the
  descriptor and the committed value of one row without listing the owner. Their cost is the size of
  the row, not the model.
- **The effective building blocks.** `controls::effective_strip`, `effective_follow`,
  `effective_gate`, `solo_term` and `any_solo` (`crates/controls/src/effective.rs`) are the one
  spelling of the rules over the `EffectiveInputs` trait. `ModelInputs::new` builds the inputs of the
  whole model (VCA reach per strip, a route-source table by linear search); there is no input for
  one strip.
- **The consumers of the diff.** `EffectiveDiff` (`prepare.rs:2860`) serves the classifier and
  `join_strips` (`prepare.rs:~3250`, a successor's carry). Its records and ramp lengths
  (`level_records`, `lengths`, `gates`, `gate_length`, `source_strips`) are the rules this slice must
  not spell twice.
- **What stays whole-session in a commit.** The protocol's `prepare_transaction`
  (`crates/protocol/src/model.rs:~1100-1130`) clones the model and compiles it, canonical JSON
  included, for every transaction; `live_admission` (`control.rs:~1830`) counts the whole prospective
  model; `ControlCatalog::stage_values` (`crates/host-core/src/control_provider.rs:~1354`) computes
  `controls::effective` for the whole model and reads the derived rows of every strip and route.
  None of them is the classifier. This slice does not change them (non-goals); the evidence records
  what remains.

## Decisions frozen for this slice

Binding, from root (2026-10-14):

- **R1.** A transaction whose edits are all live rows (`SET_CONTROL` on rows of class `Live`; the
  monitor solo row too) is classified from the registry rows its edits name. It clones no model and
  builds no canonical JSON. Structural edits keep today's path.
- **R2.** One classifier. The C ABI and the browser call the same function, the control plane's
  `commit_live`. There is no browser-only classifier and no second set of rules.
- **R3.** The fast path gives exactly what the full path gives: strip, effect and route records,
  ramp lengths, lanes, the edit path (`live` or `model_only`, from `LiveDelta::is_empty`), the
  derived changes that VCAs, `follows_mute` and solo cause, and the catalog inputs
  `changed_owners` and `settings_changed`. Where the fast path cannot classify, it falls back to the
  full path. It never acks as live and then drops.
- **R4.** Gates: a differential against the full path, an allocation count that does not depend on
  the track count, and one descriptive timing line.

Design (this slice):

- **D1. One entry.** `host_core::classify_live_edit(current, next, ramps, edit_ramps, monitor,
  rows)` with `rows: Option<&[ControlAddress]>`. It returns `classify_live_delta_with_monitor`'s
  `Result<LiveDelta, LiveRebuild>`. `commit_live` calls it and nothing else. The full classifier stays
  public: it is the fallback and the differential's oracle. `classify_live_delta` and
  `classify_live_delta_with_monitor` are unchanged.
- **D2. The rows come from the protocol.** `PreparedSessionTransaction` gains
  `set_control_rows() -> Option<&[ControlAddress]>`: `Some` exactly when every edit of the
  transaction is a `SetControl`; the slice holds the address of each edit that wrote the model, in
  transaction order (a write to a monitor row is in `monitor_edits`, not here). `None` for any other
  edit. `commit_live` passes it through. The protocol applies the edits and compiles the candidate as
  today (non-goal).
- **D3. The fast path is taken, or it is not.** It returns `Some(Ok(delta))` or `None`; every
  `Err` of any step is `None`, and the caller then runs the full classifier, which alone decides the
  error. So the order in which refusals are found is the full path's. The fast path is taken only if
  `rows` is `Some` and, for every named row, `controls::changed_rows` (D4) judges it locally. It
  falls back for:
  - a transaction with any edit that is not a `SetControl` (`rows` is `None`);
  - a session row (`control_smoothing` changes every owner's descriptors and the ramp lengths of
    the same transaction) and the strip's `matrix_mode` (the switch flips which rows are `active`
    and rewrites the pan or matrix);
  - a changed row whose class is not `Live` (rebuild-only; the full path reports its reason);
  - an address that does not resolve in both models, or a derived row;
  - any lowering refusal (`LiveRebuild::Domain`, `Structure`, an effect whose `params` do not
    resolve, a designer error): the full path decides which refusal wins.
- **D4. Changed rows by address.** New `controls::changed_rows(current, next, address,
  sample_rate) -> Option<Vec<ChangedControl>>`. It resolves the row in both models, and for each
  lane the descriptor lists (the lane of the address, or both lanes for `both` on a per-lane row)
  it emits a `ChangedControl` when the value bits differ, with the descriptor of `next` and the
  per-lane address, as `changed_controls` does. `None` for the cases D3 names. The cost is the size
  of the row. Rows are visited in the control list's order (owner index, then chain position of the
  section, then control and lane), so `Work` groups strips and effect instances exactly as the list
  order does.
- **D5. One body for the changes.** The loop that turns changed controls into `Work` items, the
  route set, the `derived` flag, `changed_owners` and `settings_changed` (`live_delta.rs:357-411`)
  becomes one function that both paths call. The lowering (`lower`) is the one function of today. The
  per-strip loop (`:436-494`) and the route loop (`:505-538`) become functions of an iterator of
  strip or route indices: the full path passes all of them, the fast path the touched ones in index
  order. So the records and their order come from one body.
- **D6. Local effective values.** The ramp rules of the level and gate records move out of
  `EffectiveDiff` into `LevelRules` (`prepare.rs`), which both forms ask; `EffectiveDiff` (every
  strip, used by `join_strips`) keeps its behavior. The local form is `LocalDiff` (`live_delta.rs`):
  the effective values of the strips an edit reaches in both models, with their VCA reach and solo
  terms, and the gates of the sends it reaches. The lowering reads either through one private trait
  (`LevelDiff`); a strip or send the local form does not hold has unchanged effective values, and a
  value the lowering needs and the form lacks makes the accessor refuse, which D3 turns into the
  fallback. It is seeded with:
  - the strip of each changed `StripLevel` row (fader, mute);
  - the strips a changed VCA reaches (members, and members of nested VCAs): new
    `SessionModel::strips_reached_by(vca)`;
  - the strips whose solo differs between the two overlays; **all** strips when `any_solo` differs
    between them (the solo term of every strip moves);
  - the routes whose own row changed, the sources of those routes, and, only when a seeded strip's
    effective mute differs, every route that follows its source's mute and whose source is such a
    strip (one scan of the routes comparing sources, no allocation per route).
- **D7. One strip, the same rules.** New `controls::LocalInputs` implements `EffectiveInputs` for
  one strip (and any route), with that strip's VCA reach from new
  `SessionModel::vca_reach_of(strip)`, and `controls::any_solo_in(model, overlay)` answers
  `any_solo` from the overlay's sorted IDs. `effective_strip`, `effective_follow` and
  `effective_gate` run unchanged on it, so no rule is spelled again. The differential and a unit test
  compare `vca_reach_of`, `strips_reached_by` and `any_solo_in` with `vca_reach`, the reach inverted
  and `any_solo(ModelInputs)` on random models.
- **D8. Steps of the full path the fast path does not run, and why that is exact.**
  - Step 1 (structure by canonical JSON) and step 2 (automation target): the edits are `SetControl`s
    that `controls::set` wrote, one row each, so the models differ in the named rows and in
    representation that the mask copies (an effect's `params` entries). The stored automation is not
    touched.
  - Step 4 (unresolvable `params`): run for each changed effect instance whose `params` differ in
    representation, with the full path's own check.
  - A strip lookup is a binary search over the sorted strip IDs (tracks, then submixes), not a
    scan.
- **D9. The strips and routes the loop visits.** Strips: each strip with a changed row, merged with
  each seeded strip whose effective fader or mute differs. Routes: each route in the route set or
  whose effective gate differs. Both in index order, which is the order of the full path's output.
- **D10. Realtime.** Control thread only. No change under `crates/engine/src/realtime/` and no
  render-path code.

## Deliverables

1. `crates/controls`: `changed_rows`, `LocalInputs`, `any_solo_in`, with unit tests (D4, D7).
2. `crates/session/src/vca.rs`: `vca_reach_of`, `strips_reached_by`, with a test against
   `vca_reach`.
3. `crates/protocol`: `set_control_rows` on the prepared transaction and the controller's token.
4. `crates/host-core`: `classify_live_edit`, the shared change body, the index-driven strip and route
   loops, the local `EffectiveDiff`, a test-support counter of fast classifications.
5. `crates/control-plane/src/control.rs`: `commit_live` calls `classify_live_edit`.
6. `crates/host-core/tests/live_edit_classifier.rs`: gates 1-3.
7. This spec's evidence section: before and after timing at 64 and 256 tracks, the allocation counts,
   and what remains whole-session in a commit.

## Authorized paths

- `crates/controls/src/{registry.rs,effective.rs,lib.rs}`, `crates/controls/tests/*`.
- `crates/session/src/vca.rs`, `crates/session/tests/*` (the new test only).
- `crates/protocol/src/{model.rs,controller.rs}` (the rows accessor and its tests).
- `crates/host-core/src/{live_delta.rs,prepare.rs,lib.rs}` (`EffectiveDiff` and its helpers in
  `prepare.rs` only), `crates/host-core/tests/live_edit_classifier.rs`.
- `hosts/host-web/tests/control_command_timing.rs` (new, ignored: one descriptive timing line of a
  whole command; no `hosts/host-web/src` change).
- `crates/control-plane/src/control.rs` (`commit_live` only).
- This spec.

## Non-goals

- No change to a row, a family, a record, a ramp rule or the lowering: the core owns them.
- No change to `crates/engine/src/realtime/`, to the wire, to the C ABI or to `hosts/host-web/src`
  or `sdk/`.
- No change to the protocol's `prepare_transaction` (its model clone and compile), to
  `live_admission` or to the catalog's `stage_values`. They stay whole-session; the evidence names
  their measured share and root decides the successor.
- No batch classifier for structural edits: they keep today's path.
- No benchmark framework; one descriptive timing line.

## Objective gates

Commands: `cargo fmt --check`, `cargo clippy --locked --workspace --all-targets --all-features --
-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps --all-features`,
`cargo test --locked -p controls -p session`, `cargo test --locked -p protocol --all-features`,
`cargo test --locked -p host-core --all-features`, `cargo test --locked -p control-plane --features
test-support`, `cargo test --locked -p capi`, the realtime-policy tool, and `audit capi` after a
release build (pcm `76525e7ef9e78cf4`).

1. **Differential.** For every case, `classify_live_edit` returns exactly what
   `classify_live_delta_with_monitor` returns on the same inputs: the whole `LiveDelta` (strips,
   effects, routes, `changed_owners`, `settings_changed`) or the same `LiveRebuild`. Cases: (a) the
   edits of the #1560 parity script (`hosts/host-web/tests/control_parity.rs`, `script()`) as
   `SetControl`s on the parity session, each alone and in the script's order, at 48 kHz and 96 kHz,
   with the script's solo steps as monitor overlays; (b) a seeded random set of one to four edits
   (at least 2,000 cases) drawn from every fast row, solo, the rows that must fall back (`matrix_mode`,
   `control_smoothing`, a rebuild-only effect parameter, an insert's link mode, `delay_samples`) and
   per-edit ramps, on a plain and a rich session (inserts, console slot, submix, sends, nested VCAs,
   both pan and matrix strips). The test counts fast classifications and asserts the generator reached
   fader, mute, pan, matrix, trim, polarity, filters, effect parameter, bypass, send, VCA and solo
   rows through the fast path and each fallback through the full path.
2. **Cost does not grow with the session.** For one live-only transaction of each kind (fader, mute,
   pan, trim, effect parameter, send gain and mute, VCA offset and mute, solo), the allocations and
   frees of the classification on the audited allocator's thread-scoped counters are equal at 8, 64
   and 256 tracks (rich sessions whose VCAs and sends do not grow), and the fast path was taken. The solo
   that moves `any_solo` is excluded: it moves every strip's mute, so its output grows.
3. **Timing (descriptive).** One line per size, native release, from the ignored `timing_line`
   test: the fader edit of the Context on 64 and 256 tracks, before and after. No threshold.
4. The existing suites pass unchanged: `crates/host-core/tests/live_delta.rs`,
   `live_transaction.rs`, `vca_live.rs`, `live_routes.rs`, `monitor_tick.rs`,
   `crates/capi/tests/resource_lifecycle.rs`, `hosts/host-web` control tests.

## Test value

- Gate 1 turns red if the fast path misses a derived change (a VCA, a followed send, a solo), takes
  the wrong ramp length, orders records differently, drops a changed owner from the catalog inputs,
  or answers live where the full path answers rebuild. No existing test compares the two paths on
  the same inputs, and the parity script checks the hosts against each other, which now share this
  function.
- Gate 2 turns red if the fast path clones, lists or walks the session (any allocation per strip,
  per route or per VCA of the session).
- The unit tests of `vca_reach_of`, `strips_reached_by`, `any_solo_in` and `changed_rows` turn red
  if the local rules drift from the dense ones that preparation also uses.

## Dependencies

- #1554 (the registry classifier whose rules this reuses), #1600 (`changed_owners`,
  `settings_changed`, the catalog inputs this must reproduce), #1559 (the monitor overlay and the
  solo term). #1562a (the control plane's one command entry) is on this branch.

## Size

One day for one sonnet-high implementer: deliverables 1-3 are small, 4 is the day. If it runs over,
split at deliverable 4's local `EffectiveDiff`: a first commit that takes the fast path for every
transaction with no derived change (pan, trim, effect parameter, polarity, filters), and a second for
fader, mute, VCA, send and solo.

## Implementation record

### Deviations from the brief

- **The local form is its own type, not a second form of `EffectiveDiff`.** The rules are shared
  through `LevelRules` and a private trait; `EffectiveDiff`'s fields lost their per-strip
  `StripRef` vectors (`strip_at` reads a strip in `O(1)`) and gained no variant.
- **Two small cost fixes beside the classifier, in `controls`:** `strip_index` (binary search with a
  scan fallback, the same answer as a scan on any model) now serves `owner_index`, and
  `MonitorOverlay::retain_strips_of` uses it. `commit_live` calls `MonitorOverlay::after` before the
  classifier, and that was `O(soloed x strips)`.
- **The parity script is replayed from a table in `host-core`'s test, not imported.**
  `control_parity.rs` keeps `script()` private to its binary and needs the browser host. The test
  rebuilds the parity session (`common::model_at`) and the script's `SET_CONTROL` edits, with its
  ramps, solo steps and refusals, and plays them through the real `SessionStore`.
- **`hosts/host-web/tests/control_command_timing.rs`** is the one file outside the engine crates,
  an ignored timing test. No `hosts/host-web/src` or `sdk/` file changed.
- **`cargo doc --all-features` fails on this branch before and after this slice** (an unresolved
  link at `crates/control-plane/src/control.rs:236`). CI builds the docs with default features, which
  pass.

### What falls back

`classify_live_edit` runs the full classifier, and returns its answer, error included, for: any
transaction with an edit that is not a `SetControl` (`UpsertVca`, structural edits); a session row
(`control_smoothing`); a strip's `matrix_mode`; any changed row of class rebuild-only (an insert's
link mode, `delay_samples`, an EQ band's kind, a route into the output, a delay's bypass); an address
that does not resolve in both models or a derived row; and any lowering refusal. The random
differential reaches each of these: `matrix_mode`, `fader_ms`, `link_mode` and a structural edit
through the full path, with live, rebuild-only and no-record verdicts.

### Evidence

Native release, x86-64 (AVX2), one fader edit on a session of one source, route and no insert per
track, 200 calls, `crates/host-core/tests/live_edit_classifier.rs::timing_line`:

| tracks | full classifier (before) | `classify_live_edit` (after) |
| --- | --- | --- |
| 64 | 1.94 ms | 1.07 us |
| 256 | 7.99 ms | 1.25 us |

The whole `control_command` of the same edit (`hosts/host-web/tests/control_command_timing.rs`, the
browser host's entry, which is `SessionState::command` and so the C ABI's too), with the full
classifier forced as before and with this slice:

| tracks | before | after |
| --- | --- | --- |
| 64 | 1.80 ms | 267 us |
| 256 | 7.94 ms | 1.11 ms |

The command is still linear in the session: the protocol's own preparation of the same transaction
(`SessionStore::prepare_transaction`: a model clone and a compile with canonical JSON) takes 206 us
at 64 tracks and 804 us at 256, about three quarters of what is left; the rest is
`live_admission` (`check_counts` over the prospective model) and `ControlCatalog::stage_values`,
which builds `controls::effective` for the whole model and reads the derived rows of every strip and
send. None is the classifier. A wasm number was not taken: the pinned `-Zbuild-std` nightly build of
the worklet and its node driver are not a cheap harness; the code is the same.

Allocations of one classification (same at 8, 64 and 256 tracks; `allocations/frees`): fader
15/15; fader under a nested VCA 21/21; mute 15/15; mute of a strip a send follows 20/20; pan
10/10; trim 11/11; effect parameter 41/29; send gain 16/16; send mute 28/28; VCA offset 43/42;
VCA mute 31/31; solo with another strip soloed measured equal in the same test. The mute of a
strip a send follows scans the routes once (comparing sources); it allocates nothing per route.

Mutation checks, each red in `live_edit_classifier.rs` and restored: no VCA seeds (differential and
cost), `all_strips` off (differential), no scan for following sends (differential), and a copy of
the track IDs in the fast path (cost).

Gates run on this head: `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
--no-deps`; `cargo test --locked` for `controls`, `session`, `protocol --all-features`, `host-core
--all-features`, `control-plane --features test-support`, `capi` and `host-web --features
test-support`; `scripts/check-workspace-policy.sh`; `realtime_policy`; `audit capi` (pcm
`76525e7ef9e78cf4`, 0 violations).

### Successor (for root)

The remaining linear parts of a live commit, in order of share: the protocol's model clone and
compile in `prepare_transaction` (it needs a transaction form that edits the committed compilation
in place and re-derives only what the rows touch, which is a protocol and session change), the
catalog stage's whole-model `effective` and derived-row pass, and `check_counts`. None changes a
classification.
