# Solo a strip on the C ABI through the core's monitor overlay

Slice S7b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C3
`scope`, C6 steps 3 and 4, C7 and C8). Code anchors verified on `main` `df585e503`. Feature sprint
mode (owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or
mutation runs.

## Product outcome

A C ABI host solos a track with the same `SET_CONTROL` op it uses for every other value. The solo
lives in the core's monitor overlay: it is never saved, it never advances the revision, and the
reply says `path: monitor`. The other tracks mute and the sends that follow them close, through
the same effective function that preparation uses, and a structural edit keeps the solo. A mobile
app gets the solo the browser has today.

## Context

- **Solo exists only in the browser.** Kind `COMMAND_SOLO` (`hosts/host-web/src/lib.rs:879`) and
  host-core's `LiveControlSoloState` (`crates/host-core/src/solo.rs:129`, `set_solo` at `:273`,
  `rollback` at `:362`). The rule: `effective_mute = user_mute || vca_mute || (any_solo &&
  !solo_safe && !solo)` (`solo.rs:11-13`); tracks are soloable and every submix is solo-safe
  (`:38-43`). A submix's solo command is refused `notSoloable` (#1213 D4).
- **The core already has the rule and the row.**
  - #1546 D5 (as updated by the coordinator, 2026-10-13): strip own row 2 `solo` (`Toggle`, shared,
    scope `Monitor`, `Live`, family `StripLevel`, ramp `Solo`), listed on track strips only; a
    submix has no solo row, so a solo write to a submix is `control.address.unknown`.
    `controls::set` refuses a `Monitor` row with `control.monitor`; this slice adds the overlay
    writer.
  - #1553 D3: `protocol::apply_session_edit` applies `SET_CONTROL` through `controls::set` (the
    protocol crate depends on `controls`).
  - #1547 D1: `MonitorOverlay`, a sorted set of soloed strip `StableId`s; `controls::effective(model,
    overlay)` and `effective_into` compose the solo term. #1547 gives it no writer.
  - #1558: the classifier's step 3 diffs `controls::effective` of the two models with one overlay.
- **The ramp row exists.** `LiveRampRow::Solo`, key `mute_ms` (`crates/host-core/src/prepare.rs:2445`;
  #1546 moves it to `effect_contract::control`).
- **The reply path.** *Report each transaction's edit path in its response* (#1313) adds
  `TransactionApplied.edit_path` with `live = 1`, `model_only = 2`, `rebuild = 3` (its D1). Every
  response frame carries a `revision` (`TypedSuccessResponseFrame`,
  `crates/protocol/src/typed_frame.rs:82-86`).
- **The transaction path.** `SessionState::command` (`crates/control-plane/src/control.rs:1082`)
  prepares the protocol token, tries `commit_live` (`:1358`), else rebuilds
  (`LiveCommit::Rebuild`, `:1120`). The protocol's prepared token holds the prospective session
  and the response (`PreparedStructuralCommand`, `crates/protocol/src/controller.rs:1117`).
- **Ruling C8** and *Admit browser live edits in the Worker through the committed model* (#1382, now #1562;
  D3 and D4 as filed) define the overlay's commit rules; decision 17 moves #1382's D3 here, to
  the core, for both hosts.
- **The C ABI has no solo today** (#1053 "Out of scope": "the app composes solo as mutes").

## Decisions frozen for this slice

- **D1. Where the overlay lives.** `control_plane::SessionState` owns one `controls::MonitorOverlay`,
  empty at boot. It is the only solo state on the C ABI. `LiveControlSoloState` stays in host-core
  for the browser's own admission until #1565 deletes that admission; no C ABI code reads it.
- **D2. The row.** `SET_CONTROL` on `{owner: strip(id), section: own, control: 2, lane: both}`.
  The value is `0` or `1` (`control.domain` otherwise). `left` or `right` is `control.lane`. The
  row exists on tracks only; a solo to a submix is `control.address.unknown` (the browser's
  `notSoloable`, #1213 D4). Its `ramp_samples` is allowed (ramp row `Solo`).
- **D3. A monitor edit is not a model edit.** `protocol::apply_session_edit` sorts each
  `SET_CONTROL` by its row's `scope`, before it calls `controls::set`:
  - a `Model` row goes through `controls::set` into the prospective model, as #1553 made it;
  - a `Monitor` row is checked with `controls::check` and the address lookup (unknown, inactive,
    lane, domain, ramp) at the same point, and is recorded on the prepared token as a monitor
    edit `{strip: StableId, soloed: bool, ramp: Option<u32>}`, in transaction order. It writes
    nothing. The token exposes them as `monitor_edits() -> &[MonitorEdit]`.
  - A later monitor edit of the same strip in one transaction replaces the earlier one (edits of
    one transaction apply in order, ruling C5).
- **D4. A monitor-only transaction.** A transaction whose every edit is a monitor edit:
  - keeps the revision; the response frame carries the current committed revision;
  - replies `TransactionApplied { applied_operations, edit_path: Monitor }` with a new
    `EditPath::Monitor = 4` (wire value 4, appended to #1313's enum; `0` and values above 4 stay
    refused);
  - emits no `SESSION_COMMITTED` event and advances no watermark;
  - is recorded in the replay cache like every response, so an exact replay returns the same
    bytes and applies nothing again;
  - needs the same exact expected revision as every `SESSION_TRANSACTION_APPLY`.
- **D5. A mixed transaction.** A transaction with model edits and monitor edits advances the
  revision by 1 as usual. Its path is computed over the whole delta: `rebuild` if the model part
  needs a rebuild; else `live` if the model part or the overlay change produces a record; else
  `model_only`. `monitor` is never the path of a transaction that has a model edit.
- **D6. Records.** The commit computes `controls::effective(current_model, current_overlay)` and
  `controls::effective(next_model, next_overlay)` (#1558's step 3, with the overlay as the second
  input). Every changed effective mute lane gives a strip `Mute` record; every changed route gate
  gives a route record. A solo change produces no other record kind.
- **D7. Ramps (ruling C6 step 4, #1382 D3 as filed).** A record that only the overlay change
  produced takes `LiveRamps::resolve(Solo, ramp)` of `next`, where `ramp` is the monitor edit's
  `ramp_samples` (absent: `mute_ms`; `0`: a step). When several monitor edits of one transaction
  change records, each record takes the ramp of the last monitor edit in transaction order. A
  record that a model edit also produced takes the model edit's ramp (#1558 D7), built once.
- **D8. All or nothing (the acked-batch question).** The overlay is written only after every
  fallible step of the transaction, live or rebuild, has passed: in the live arm after the
  protocol commit predicate and the stamp, in the rebuild arm after `prepared.commit`. A refusal
  leaves model, revision, overlay, catalog and cells unchanged. Cells are latest-target cells, so
  no record is refused for room, and no ack precedes a drop.
- **D9. Structural commits keep the overlay.** The rebuild arm prepares the successor with
  `controls::effective(next_model, next_overlay)` (#1547 D4's argument), so the successor bakes the
  solo mutes, and `join_strips` diffs effective values with the overlay on both sides. At the
  same commit, an overlay entry whose strip `next_model` no longer has is dropped. A new track
  added while another strip is soloed is solo-muted.
- **D10. Pending plans.** While a replacement plan is pending, the overlay's records go to the
  newest plan's cells, as every live record does (D15-2); render converges at the block that adopts
  it.
- **D11. The control list.** The `solo` row's `value` in `CONTROL_LIST_GET` and
  `CONTROL_STATE_GET` is the overlay (ruling C4); the catalog of #1552 updates it in place on every
  overlay change. A solo changes no `structure_epoch`. `SESSION_SNAPSHOT_GET` never contains solo.
- **D12. Header prose.** `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md`
  state: solo is a `SET_CONTROL` on strip own row 2; it is host-local, never saved and never in a
  snapshot; a solo-only transaction replies `monitor` with the revision unchanged. No C symbol or
  struct changes.

## Deliverables

1. D3-D5 in `crates/protocol` (the token's monitor edits, `EditPath::Monitor`, the monitor-only
   commit with no revision change, the schema value).
2. D1 and D6-D11 in `crates/control-plane/src/control.rs` (and `compile.rs` if the preparation
   call lives there).
3. D9's overlay argument in host-core preparation and `join_strips`
   (`crates/host-core/src/prepare.rs`).
4. D2's `active` rule in `crates/controls`.
5. D12's prose; the protocol registry line for `edit-path` value 4
   (`docs/CONTROL_PROTOCOL_REGISTRY.md`); the conformance corpus re-pinned for the new enum value.

## Authorized paths

- `crates/protocol/src/{controller.rs,message_wire.rs,schema.rs,model.rs,lib.rs}`,
  `crates/protocol/src/controller/tests.rs`.
- `crates/control-plane/src/{control.rs,compile.rs}`.
- `crates/host-core/src/{prepare.rs,live_delta.rs}`.
- `crates/controls/**`.
- `crates/conformance/src/protocol_corpus.rs`, `tools/audit/src/protocol.rs` (the new enum value).
- `crates/capi/include/miso_engine_v1.h` (prose), `crates/capi/src/runtime/live_tests.rs`,
  `crates/capi/tests/resource_lifecycle.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`.

## Non-goals

- No browser change. The browser keeps `COMMAND_SOLO` until #1565; #1562 moves it onto this
  overlay.
- No saved solo, no solo in automation (the row is `Monitor`, so never automatable), no solo-safe
  row (every submix is solo-safe, as today), no VCA solo.
- No observation subscription or meter lease change (ruling C9).

## Objective gates

Commands: `cargo test --locked -p protocol -p controls`,
`cargo test --locked -p control-plane --features test-support`,
`cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi`,
`cargo test --locked -p conformance`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`.

1. **Solo renders as a fresh plan with the same overlay.** Fixture: tracks `t1`, `t2`, `t3`;
   submix `b` routed to the output; `t3` sends `pre_fader` into `b` with `follows_mute: true`.
   Through the C ABI, `SET_CONTROL` solo `t2`: the reply is `monitor`, the revision is unchanged,
   the plan epoch is unchanged. `t1` and `t3` mute, the send's both columns close, `b` stays
   audible. From the mute ramp on, every block is bit-identical to a plan prepared from the
   committed model with the same `MonitorOverlay`, fed the same PCM from frame 0. Then unsolo:
   the output returns to the unsoloed plan's bits.
2. **Structural commits keep it.** With `t2` soloed, add a track `t4` (`UpsertTrack`, path
   `rebuild`): the successor renders `t4` solo-muted and `t2` audible, bit-identical to a fresh
   plan with the overlay. Remove `t2`: its entry drops, and every track is audible again.
3. **All or nothing.** A transaction of a VCA `offset_db` `SET_CONTROL`, a solo, and a pan
   `SET_CONTROL` out of domain is refused with the pan's `control.domain`. Model, revision,
   overlay and cells are unchanged, and the next 8 blocks are bit-identical to a run without the
   call. (This is the shared-path half of #1525.)
4. **Ramps.** Solo with `ramp_samples` absent ramps over `mute_ms`; with 0 it steps at the next
   block; with 480 it reaches the target after 480 samples. A transaction that sets `t1`'s own
   `mute` with ramp 0 and solos `t2` with ramp 480 steps `t1` (the model edit's ramp).
5. **Refusals.** Solo on `b` is `control.address.unknown`; lane `left` is `control.lane`; value
   `0.5` is `control.domain`; `controls::set` called directly on the solo row is `control.monitor`.
   Each changes nothing.
6. **Replay and events.** An exact replay of a solo-only request returns the same bytes and writes
   no cell. A solo-only transaction emits no `SESSION_COMMITTED` event; a mixed one emits one.
7. **List and snapshot.** After soloing `t2`, `CONTROL_STATE_GET` reports `1` on `t2`'s solo row
   with the same `structure_epoch`; `SESSION_SNAPSHOT_GET` bytes equal those before the solo.
8. **Codec.** `edit_path` `4` round-trips; `0` and `5` are refused on decode.
9. **Realtime.** The race test (`crates/capi/tests/resource_lifecycle.rs:3364`) also races solo
   toggles over 20 runs: `allocations == 0 && frees == 0` around every render call after warm-up,
   zero `INTERNAL` results, a final block bit-identical to a fresh plan of the final committed
   model with the final overlay.

## Test value

- Gate 1 turns red if the live solo composes differently from preparation, mutes a solo-safe
  submix, leaves a following send open, or advances the revision.
- Gate 2 turns red if a rebuild drops the overlay or keeps an entry for a removed strip.
- Gate 3 turns red if the overlay commits while the model refuses.
- Gate 4 turns red if solo resolves its default from a key other than `mute_ms`, or if a record
  that a model edit and a solo both produce takes the solo's ramp.
- Gate 5 turns red if a submix becomes soloable or a malformed solo is accepted.
- Gate 6 turns red if a replayed solo re-applies or a solo-only commit tells listeners the session
  changed.
- Gate 7 turns red if solo leaks into the saved session or the list shows a stale solo value.
- Gate 8 turns red if the new path value is not on the wire or a stray value decodes.
- Gate 9 turns red if the overlay path allocates on render.

## Dependencies

- #1558 (step 3 and the effective-value records).
- #1547 (`MonitorOverlay`, the overlay argument of preparation).
- #1553 (`SET_CONTROL` in transaction apply).
- #1552 (the control catalog).
- *Report each transaction's edit path in its response* (#1313). This slice does not add the
  path field: it depends on #1313, which is not on `main`, and appends one value to #1313's enum.
  Root orders #1313 before this slice (it is stream B's, independent of decision 17).

## Supersedes and shrinks

- Shrinks *Test that a refused batch does not undo a committed browser VCA batch* (#1525): gate 3
  is its shared-path half; the browser half ends with the browser admission in #1565.
- Shrinks *Admit browser live edits in the Worker through the committed model* (#1562, formerly #1382): its D3 as filed
  (the overlay composed in the shared commit) is this slice; see #1562's amendment record.

## Implementation record (attempt 1, 2026-10-13)

Built as specified, with these facts the spec did not have, each decided here:

- **A solo's records ride a monitor publication** (#1603, root ruling of 2026-10-13, option B as
  implemented by option C). A monitor-only transaction that produces records stamps
  `stamp_monitor(committed revision)` on the newest plan's gate and publishes that key with
  `publish_key`: a new publication that carries the committed revision `r`. No revision number is
  spent: the store has no `next_gate_revision`, and the next model commit is `r + 1`. The applied
  watermark reports `r` and never a higher number. A monitor-only transaction never rebuilds (a
  rebuild verdict for one is `INTERNAL`, a defect, with nothing written), and an exhausted
  publication number (`PublicationsExhausted`, 2^63 - 1 on one plan's gate) is `INTERNAL` too.
- **Where the writes sort.** `controls::check_monitor` (new) checks a monitor row exactly as `set`
  would and returns `controls::MonitorWrite`; `SessionStore::prepare_transaction_with_monitor`
  records them on the token in order (a later write of one strip replaces the earlier). The
  one-call path (`process_command_frame_into`) has no overlay and keeps refusing with
  `control.monitor`.
- **Overlay in preparation.** `host_core::SuccessorBase` gains `monitor: MonitorBase`
  (`MonitorBase::NONE` for a host without a solo); `classify_live_delta_with_monitor` is the
  classifier with the overlays; `classify_live_delta` is it with `NONE`. `ControlCatalog::build`
  takes the overlay.
- **Ramp (D7).** `EffectiveDiff::lengths`: the strip's own mute edit's length, then a reaching VCA's,
  then `Solo` for a mute that only the solo term moved. The solo term has one spelling,
  `controls::solo_term`.
- **Corpus.** The conformance corpus's response frame is unchanged (it carries `rebuild`), so
  `COMPLETE_SCHEMA_HASH` does not move; the codec test covers 4 and refuses 0 and 5.
