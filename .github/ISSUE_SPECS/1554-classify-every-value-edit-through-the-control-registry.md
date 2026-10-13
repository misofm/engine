# Classify every value edit through the control registry

Slice S4b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C6,
steps 1, 2 and 4). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs. The planner's S4b is split in two: this slice (the classifier) and #1555 (live lanes on every
plan).

## Product outcome

The C ABI decides live or rebuild for every value edit from the control rows, not from a
hand-written list of fields. A value whose row is live and whose family the plan carries is a live
update on every strip, submixes included; a value whose row is rebuild-only rebuilds, for the
reason its row states. Adding a live row to an existing family needs no classifier code.

## Context

- **The classifier today.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:215-300`):
  step 1 refuses any model with a VCA (`LiveRebuild::Vca`, `:221-223`); step 2 compares the track
  IDs only (`:224-232`); step 3 masks a hand-written field list (revision, session ID, profile IDs,
  `control_smoothing`, automation, each **track's** fader, `matrix_or_pan`, console and insert
  `params` and `bypass`, `:233-263`) and compares canonical JSON bytes (`:264-270`); then the
  automation-target check (`:273-278`) and per-track records (`:281-297`): `fader_records`,
  `matrix_record` (`crates/host-core/src/prepare.rs:2560-2647`), `followed_mute_changes`
  (`:2609-2622`) and `effect_records` (`live_delta.rs:304-371`), whose
  `parameter_records` refuses a non-`Block` parameter as `LiveRebuild::Prepared` (`:404-407`) and
  whose bypass check refuses `!lowers_session_bypass` as `PreparedBypass` (`:344-354`).
  `LiveRebuild` is at `:96-124`. Submix strips are never live: any submix change is structural.
- **The commit.** `commit_live` (`crates/control-plane/src/control.rs:1358`) resolves strip
  producers among the tracks only (`strips.controls[..track_count]`, `:1387-1397`) and effect
  producers by a linear `find` (`:1417`; `HostLiveControlHandles::effect_control_mut` is the same
  scan, `crates/host-core/src/prepare.rs:501-511`). The C ABI attaches the strip fader/mute and
  matrix lanes of every strip, submixes included, and one lane per effect instance
  (`C_ABI_LIVE_LANES`, `crates/control-plane/src/compile.rs:28-31`).
- **The rows.** #1546 and #1549 give every value a row with a class and a family; `controls::list`
  yields them in a fixed order. Strip input, route and VCA rows are `RebuildOnly(RuledLiveNotBuilt)`
  until #1556, #1557 and #1558.
- **Open issues this replaces.** #1390 (submix fader, mute and pan on the C ABI), the effect half
  of #1267 (submix effects), #1305 (lookups without linear scans).

## Decisions frozen for this slice

- **D1. Step 1, the mask.** `controls::mask_values(into: &mut SessionModel, from: &SessionModel)`
  copies, for every owner present in both models with the same ID and kind, the model fields that
  hold control values from `from` into `into`: each strip's `builtins`, `fader`, `matrix_or_pan`,
  and each console entry's and insert's `params`, `bypass` and (inserts) `link_mode`; each route's
  `gain_db`, `mute`, `channel_matrix` and `follows_mute`; each VCA's `fader`; the session's
  `control_smoothing`. It copies representations, not resolved values, so an effect's `Both`/per-lane
  entry form is masked too. The classifier keeps its model-only mask (revision, session ID, the two
  profile IDs, automation; #1260, #1365) and compares canonical JSON bytes as today: a difference is
  `LiveRebuild::Structure`. The hand-written field list and the track-ID pre-check go.
- **D2. Step 2, the rows.** With the structure equal, `controls::list(current)` and
  `controls::list(next)` yield the same addresses in the same order. For each address whose value
  bits differ:
  - `RebuildOnly { reason, .. }` → `LiveRebuild::RebuildOnly(reason)` (this replaces `Prepared` and
    `PreparedBypass`);
  - family `Setting` → no record;
  - `Live` → the owner is marked changed for that family.
  `Derived` rows are skipped (#1558 adds step 3).
- **D3. One lowering per family, in host-core.** `fn lower(family, ..)` dispatches on `Family`:
  `StripLevel` → `fader_records` of the owner's before and after fader; `StripMatrix` →
  `matrix_record`; `Effect` → the instance's records (the bypass record first, then
  `parameter_records`, with the EQ's prepared targets, unchanged); `Setting` → none. `StripInput`
  and `Route` are unreachable in this slice (their rows are rebuild-only) and return
  `LiveRebuild::Structure` defensively; #1556 and #1557 add their arms. Each changed owner is lowered
  once per family; an unchanged owner is never lowered. A lowering's domain refusal stays
  `LiveRebuild::Domain`.
- **D4. Step 4, ramps.** Every record's length stays `LiveRamps::resolve(row, edit ramp)` through
  the per-owner functions (`StripEditRamps`, #1394 D6); nothing changes.
- **D5. Every strip.** The records cover tracks and submixes (`LiveDelta::strips` and
  `LiveDelta::effects` hold strips in `model.strips()` order). `followed_mute_changes` also covers a
  route whose source is a submix (`RouteSource::Submix`). The VCA early exit
  (`LiveRebuild::Vca`) and the followed-mute guard (`LiveRebuild::FollowedMute`) stay until #1558.
  The automation-target step stays.
- **D6. Producer lookup.** `commit_live` resolves a strip's producers among all of
  `strips.controls` (tracks, then submixes), and an effect producer by `partition_point` over the
  producer table, which is sorted by `(strip order, address)`; a `debug_assert!` checks the sort at
  preparation. No new retained memory.
- **D7. A counter for the test build.** Under `test-support`, host-core counts lowered owners per
  classification, so a test can show that one edit lowers one owner.
- **D8. Unchanged.** The cells, the drains, `live_values_superseded`, the watermark, the acked-batch
  order of `commit_live` (every fallible step before the first cell write) and every rendered bit
  of an existing live edit.

## Deliverables

1. `controls::mask_values` (D1) and the row diff (D2) in `crates/controls`.
2. The rewritten `classify_live_delta` and `fn lower` (D1-D5) in host-core.
3. D6 in `commit_live`; D7.
4. The tests below; existing classifier tests that named `Prepared` or `PreparedBypass` expect
   `RebuildOnly(..)` with the row's reason.

## Authorized paths

- `crates/controls/**`.
- `crates/host-core/src/{live_delta.rs,prepare.rs,lib.rs}`, `crates/host-core/tests/live_delta.rs`,
  `crates/host-core/tests/successor_swap.rs` (expected variants only).
- `crates/control-plane/src/control.rs` (`commit_live`'s resolution).
- `crates/capi/src/runtime/live_tests.rs` (new tests).

## Non-goals

- No change to which lanes a plan attaches (#1555).
- No strip input, route, VCA, follow or solo liveness (#1556, #1557, #1558, #1559).
- No change to the browser.

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi`.

1. **Live per family, on a track and on a submix.** For each live row of the strip-level,
   strip-matrix and effect families (`fader_db` per lane, `mute`, `pan`, a matrix row, `matrix_mode`,
   one effect parameter of a console slot and of an insert, one EQ parameter, an effect bypass), on a
   track and on a submix strip: a `SET_CONTROL` commits with the plan epoch unchanged and the
   revision advanced by one, and after the ramp each block is bit-identical to a plan prepared from
   the committed model.
2. **Rebuild by row.** A `rebuild_only` row (`delay_samples`, the limiter's lookahead, the delay's
   bypass, a send's `gain_db`) rebuilds, and the classifier returns `RebuildOnly` with the row's
   reason; a value edit plus a structural edit in one transaction rebuilds.
3. **Settings.** A `pan_window_samples` or `fader_ms` change alone is live with no record.
4. **Representation.** Rewriting a `Both` effect entry as equal `Left` and `Right` entries is live
   with no record.
5. **One owner.** One `fader_db` edit on a 64-track session with 4 submixes lowers exactly one
   owner (D7's counter).
6. **No allocation in render, no ack before a drop.** `audit capi` reports 0 allocations, frees,
   locks and syscalls; a batch whose second edit is out of domain writes no cell.

## Test value

- Gate 1 turns red if a family lowers to the wrong lane or cell, or a submix is left structural.
- Gate 2 turns red if a rebuild-only value is acked as live, or a value edit hides a structural one.
- Gate 3 turns red if a setting produces a record or a rebuild.
- Gate 4 turns red if the mask compares resolved values instead of representations, or the reverse.
- Gate 5 turns red if the classifier lowers every owner again.
- Gate 6 turns red if a check moves after the first cell write.

## Dependencies

- #1546, #1549 (rows and classes), #1553 (`SET_CONTROL`, used by the tests). #1547 is not needed.
- Root's ruling places this program's edits of `crates/host-core/src/live_delta.rs`, `prepare.rs`
  and `crates/control-plane/src/` before later streams (`STREAMS.md` rows 91 and 92).

## Supersedes and shrinks

- Supersedes *Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan*
  (#1390) and *Find live C ABI edit targets without linear scans* (#1305).
- Shrinks *Apply value-only submix-strip input-section and effect edits to the running C ABI plan*
  (#1267): the effect half is here; #1556 covers the input half.
- Shrinks *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the
  live console lanes* (#1053): its G1 guard is lifted for submix fader, matrix and effects here.
## Decision record (batch 2 review fold, 2026-10-13)

- **D-fold-1.** `effect_records` returns `LiveRebuild::Structure` when a changed effect
  row is neither the bypass nor a parameter (today the link mode), so a future live row without a
  lowering can never be acked and dropped. The classifier's step 4 reaches it only for a live row,
  and no link-mode row is live today, so the test calls the lowering directly (`live_delta.rs`
  unit test).
