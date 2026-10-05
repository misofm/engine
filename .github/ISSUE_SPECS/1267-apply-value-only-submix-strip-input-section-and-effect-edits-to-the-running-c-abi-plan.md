# Apply value-only submix-strip input-section and effect edits to the running C ABI plan

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-6).
Code anchors verified on `main` at `6fb211594`.

Follow-up of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053) and of *Deliver value-only send and submix-strip edits
to the running C ABI plan* (#1225). It closes the submix half of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`).

## Product outcome

On the C ABI, a transaction that changes a submix strip's input `trim_db`, `polarity_invert`,
`hpf_hz` or `lpf_hz`, a live parameter of one of its effects (console entry or insert), or one of
its effects' bypass, is a live update, exactly as the same edit on a track is. Decision 14's rows
apply to every strip (its Q5 answer: buses keep trim and polarity).

## Context

- **The model.** `Submix { id, builtins, console, inserts, fader, matrix_or_pan }`
  (`crates/session/src/model.rs:688-702`): a track's strip fields, without a source mapping.
  `StripRef` (`:339-354`) views a track or a submix through the same fields, `SessionModel::strips`
  (`:419-424`) yields tracks then submixes, and `lower_strip` (`:440-449`) lowers either one's
  racks; `lower_track` (`:430-432`) delegates to it.
- **The classifier today.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) walks
  `current.tracks` only. The step-3 mask copies only track fields (`:228-256`), so every submix
  field is structural (#1053 guard G1, doc `:155-157`). `effect_records` (`:334`) takes a `&Track`
  and calls `lower_track`. After #1225, a submix strip's `fader` and `matrix_or_pan` are masked;
  after #1261 and #1262, a track's input section is.
- **The lanes.** host-core attaches one control producer per strip, submixes included, after the
  tracks (`HostLiveControlHandles::strip_controls`, `crates/host-core/src/prepare.rs:424-433`), with
  an input lane when `strip_input` is set (#1261 sets it on the C ABI). It attaches one effect
  producer per effect instance, submix effects included, with the submix ID in `track_id`
  (`:434-441`). The C ABI already selects the effect lanes (`crates/capi/src/runtime/compile.rs:18-21`).
- **The commit.** `commit_live` resolves strip producers in `strips.controls[..track_count]`
  (`crates/capi/src/runtime/control.rs:1093-1109`) and effect producers by `(track_id, address)`
  (`:1124-1128`). #1225 widens the strip resolution to submixes. #1309 moves `control.rs`
  unchanged to `crates/control-plane/src/control.rs`; capi's tests stay in capi.
- **Delivered track rules.** Effect parameters (#1264), the parametric EQ through its owner
  (#1265) and bypass with the prepared-bypass exception (#1266) are closed and live on tracks.

## Decisions frozen for this slice

- **D1. One code path.** The classifier's per-track steps run over `StripRef`s from
  `SessionModel::strips()`, paired by position in `current` and `next` (tracks, then submixes, in
  normalized order). The step-3 mask copies the same live fields for a submix as for a track:
  input `trim_db`, `polarity_invert`, `hpf_hz`, `lpf_hz`; console-entry and insert `params` and
  `bypass`. `effect_records` takes a `StripRef` and calls `lower_strip`. So every track rule,
  present (#1261, #1262, #1264, #1265, #1266) and future, applies to a submix by construction.
  Records are addressed by the submix's strip ID, never by index.
- **D2. No new lane, record, setting or ABI item.** If a rule needs a submix-specific exception,
  stop and record it rather than invent one.
- **D3. The C ABI.** `commit_live` resolves a submix's input writer and its effect producers by
  ID, as #1225 and #1264 do, and writes the submix's input values through #1346's cells and its effect values through
  #1312's, after every fallible check.
  Acked-batch question: as #1261 D5, no ack precedes a drop.
- **D4. Still structural.** A submix's `delay_samples` (decision 14, rule 1) and its ID set.
- **D5. Guard G1.** Mark G1 superseded in #1053's spec, citing this issue, if the spec is still in
  `.github/ISSUE_SPECS/`.
- **D6. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` states that every live strip value is
  live on submix strips too.

## Deliverables

1. D1 in the classifier, with its tests.
2. D3 in the C ABI control plane, with its tests.
3. D5 and D6.

## Authorized paths

- The classifier: `crates/host-core/src/live_delta.rs` and
  `crates/host-core/tests/live_delta.rs`.
- The control plane: `crates/control-plane/src/control.rs`.
- capi's live tests: `crates/capi/src/runtime/live_tests.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- `.github/ISSUE_SPECS/1053-*.md` (G1's bullet only).
- This spec.

## Non-goals

- No submix `delay_samples`. No browser change. No new live parameter on any effect.

## Objective gates

Run every command from the repository root.

1. **The classifier.** In `crates/host-core/tests/live_delta.rs`, on `with_bus` (`:123`) with the
   submix given a compressor insert and an EQ console entry where needed: for each track case of
   #1261's, #1262's, #1264's, #1265's and #1266's classifier gates, the same edit on the submix
   gives the same records (or the same `LiveRebuild` reason), addressed to the submix's ID. A
   submix `delay_samples` change gives `Structure`.
   `cargo test --locked -p host-core --all-targets --features host-core/test-support`
2. **Equal to a rebuild.** In the capi live tests, on a session where one track sends to a submix
   that has no other stateful effect (no console slots on the session, no other inserts): a trim, a polarity flip, and a compressor bypass on the
   submix strip. Each commit is live (no new plan epoch). From block
   E + ceil((longest ramp of the edit + `latency_samples`) / quantum) + 1 on, the output is
   bit-identical to a `Reference` compiled from the committed snapshot.
3. **Equal to the browser's lane.** For an HPF, an LPF, a compressor parameter and an EQ band gain
   on the submix strip: the edit through the C ABI and the same records, built by hand (filter and
   EQ targets from their preparers called directly, never from the classifier) and applied at the
   same block to a `LaneReference` plan prepared with `HostLiveLanes::ALL`, render
   bit-identically. At 48 kHz.
4. **Nothing else changes.** `cargo test --locked -p capi`,
   `cargo test --locked -p control-plane --all-targets --features control-plane/test-support`; #1261's gates 5 and 6. 4-lane (NEON)
   is CI-only.

## Test value

- Gate 1: red if a submix edit stays structural, is addressed by index or to the wrong strip, or
  takes a rule different from a track's.
- Gate 2: red if a submix input or bypass record does not end where preparation bakes it.
- Gate 3: red if a submix's prepared targets or effect records land differently from a track's.
- **Superseded in the same PR:** classifier and capi cases that assert a submix input or effect
  edit is structural (G1) are changed to the live path; `a_live_track_edit_beside_a_submix_strip_reaches_its_track`
  (`crates/capi/src/runtime/live_tests.rs:1115`) stays.

## Dependencies

- *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225)
- *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261)
- *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Hold strip input-lane values in latest-target cells* (#1346)
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)

## Standing rules for the implementer

- Work from this body and the merged code of its dependencies. Extend them; do not fork them.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
