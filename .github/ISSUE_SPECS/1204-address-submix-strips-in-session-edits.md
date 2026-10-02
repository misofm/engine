# Address submix strips in session edits

Slice 07 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K1.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

Every session edit that changes a track's strip also changes a submix's strip when it is given a
submix ID. That covers builtins, inserts, console entries with their bypass and parameters, the
fader, and pan or matrix.

An agent driving the C ABI, or any other `SESSION_TRANSACTION_APPLY` client, can then edit a bus
without re-upserting the whole submix. No opcode is added.

## Context (verified on `fe8ac679`; slices 02-06 gave submixes the strip fields these edits write)

- **Edits.** `SessionEdit` and `apply_session_edit` are in `crates/protocol/src/model.rs`:
  - the opcode enum is at `:20-106` (`from_raw` `:117`), and the variants at `:169-397`;
  - `apply_session_edit` is at `:480`;
  - the source assignment is at `:520`, the builtins edit at `:531`, the fader at `:663`, matrix or
    pan at `:666`, the console entries at `:672`;
  - upsert and remove submix are at `:675-680`. `UpsertSubmix` is a plain upsert into
    `session.submixes` with no namespace check: the store validates only the final candidate, so
    inside one transaction a track and a submix may briefly share an ID.
- **Resolution helpers.** Each resolves a **track** by ID and returns `SessionEditError::NotFound`
  otherwise:
  - `track_mut` (`:942-951`);
  - `rack_mut` (`:989-999`):
    - `Inserts` resolves the track's inserts;
    - `Builtins` is `NotFound`;
    - `Console` is `ConsoleSlotFixed`, before any lookup.
  - `effect_mut` (`:1004-1015`), which goes through `rack_mut`;
  - `knobs_mut` (`:1029-1053`). For `Console` it resolves the **track's entry** for the named slot;
    otherwise it goes through `effect_mut`.
- **Strip opcodes** (`crates/protocol/src/model.rs:40-77`):
  - `0203` sets builtins.
  - The structural rack and declaration edits go through `rack_mut` or `effect_mut`, so a console
    rack is refused as slot-fixed:
    - `0204` set rack;
    - `0205` put effect;
    - `0206` remove effect;
    - `0207` effect order;
    - `0208` effect identity;
    - `0209` effect quality;
    - `020b` link mode;
    - `020c` sidechain.
  - The knob edits go through `knobs_mut`, which **does** edit a console entry:
    - `020a` bypass;
    - `020d` upsert param;
    - `020e` remove param.
  - `020f` sets the fader, `0210` the matrix or pan, and `0211` the console entries.
- **Track-only opcodes.** `0200` upserts a track, `0201` removes one, and `0202` sets the source
  assignment.
- **Diagnostics.** The edit-error strings are at `crates/protocol/src/controller.rs:3509-3517`,
  for example `session.edit.not_found` and `session.edit.console_slot_fixed`.
- **IDs are unique** across tracks, submixes and outputs in a **validated** model
  (`crates/session/src/validate.rs:69-100`).
- **The all-opcode corpus** (`crates/conformance/src/protocol_corpus.rs:16`) holds exactly one edit
  per allocated opcode, and `COMPLETE_SCHEMA_HASH` covers its bytes.
- **C ABI.** Transactions arrive through `miso_engine_v1_submit_command` (`crates/capi/src/ffi.rs:569`)
  and every one is structural today (`crates/capi/src/runtime/control.rs:718`). capi applies edits
  through this same protocol model, so a C ABI test of a submix edit would catch no defect the
  protocol tests below do not (VERIFY-2 MINOR 14).

## Decisions frozen for this slice

- **D1.** Opcodes `0203`-`0211` address a **strip ID**.
  - A track ID resolves to the track's strip, and a submix ID to the submix's strip.
  - The payload field keeps its wire position and type. The registry renames it "strip ID".
  - Rust variant and field names (`SetTrackFader { track_id, .. }`) stay, as decision 12 kept
    internal names. Each gets a doc comment saying that the ID names a strip.
- **D2.** `0202` (source assignment) stays track-only. A submix ID refuses with
  `session.edit.not_found`. An **output** ID refuses with `session.edit.not_found` for every strip
  opcode.
- **D3. Rack and knob semantics are unchanged; only the resolution widens.**
  - The structural edits refuse a console rack as slot-fixed, for a submix exactly as for a track.
  - The knob edits edit a submix's console entry through `knobs_mut`, exactly as for a track.
  - Builtins rack edits stay not-found.
  - The slot-set rule from *Carry every console slot on every submix strip* (#1202) stands.
- **D4. Lookup order (DESIGN P8).** A strip ID resolves **tracks first, then submixes**. In a
  validated model only one can match; inside a transaction, before final validation, a track and a
  submix may share an ID, and the track wins.

## Deliverables

1. **Strip resolution.** Add `strip_mut(session, id) -> Result<StripMut<'_>, SessionEditError>`,
   which returns mutable references to the five strip fields of a track or a submix, in D4's order.
   Route `0203` and `0204`-`0211` through it, including inside `rack_mut`, `effect_mut` and
   `knobs_mut`.
2. **Tests, not corpus rows.** Leave `complete_all_opcode_fixture` unchanged, so the hash does not
   move: no field and no opcode changes. The submix-addressed cases go in one new file in
   `crates/protocol/tests/` (gates 1-3).
3. **Docs.**
   - `docs/CONTROL_PROTOCOL_REGISTRY.md`: D1-D4.
   - `docs/C_ABI_V1_QUALIFICATION.md`: one line saying submix strips are editable through the same
     transactions. They are structural until *Deliver value-only send and submix-strip edits to the
     running C ABI plan* (#1225) lands.

## Authorized paths

- `crates/protocol/src/{model.rs,controller.rs}`
- `crates/protocol/tests/` (one new test file, for example `submix_strip_edits.rs`)
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/C_ABI_V1_QUALIFICATION.md`
- this spec

## Non-goals

- No new opcode, field or code.
- No value-only (live) classification on the C ABI (#1053 and *Deliver value-only send and
  submix-strip edits to the running C ABI plan*).
- No committed C ABI test (see Context and Evidence).

## Hazards

- **Shadowing.** IDs are unique across tracks, submixes and outputs only in a validated model. Inside
  a transaction the order of D4 decides; an output ID must still refuse with not-found.
- **Console refusal order.** `rack_mut` refuses a console rack *before* any lookup, as a property of
  the edit. Keep that order: a structural console edit naming an unknown ID still answers
  `console_slot_fixed`, for a submix exactly as for a track.

## Objective gates

1. **Every strip opcode edits a submix.**
   - A table test applies one edit per opcode in `0203`-`0211` to a submix ID: a console bypass and a
     console parameter through `020a` and `020d`, and an insert through `0205`.
   - The committed model's canonical snapshot equals the expected canonical JSON, and every other
     strip is unchanged.

   *Test value: it turns red if any strip opcode still resolves through `track_mut` and refuses a
   submix. Every existing edit test addresses a track.*
2. **Refusals are the track's.**
   - `0202` with a submix ID, and `020f` with an output ID, refuse with `session.edit.not_found`.
   - `0205` naming the console rack of a submix refuses with `session.edit.console_slot_fixed`.
   - Nothing is committed, and the revision does not advance.

   *Test value: it turns red if source assignment reaches a submix, if an output is treated as a
   strip, or if a submix's console slots become structurally editable.*
3. **Lookup order inside a transaction** (D4).
   - The base session has a track `x` that no route or automation references.
   - One transaction applies, in order: `0300` upsert submix `x` (a transparent strip), `020f` set
     fader of `x` to -6 dB, then `0201` remove track `x`. A dangling submix is a valid session, so
     the final model validates without a route.
   - It commits, and the committed submix `x`'s fader is the transparent 0 dB: the `020f` resolved
     to the track, which was then removed.
   - The mirror transaction (`0201` remove track `x` first, then `0300`, then `020f`) commits with
     submix `x` at -6 dB.

   *Test value: it turns red if `strip_mut` searches submixes first or picks an arbitrary match
   when a transaction transiently holds a track and a submix with one ID.*
4. **Policy and parity.**
   - `cargo test --locked -p protocol --features protocol/test-support`
   - the test-debug-b command (with conformance), unchanged hash
   - `bash scripts/check-protocol-wasm-parity.sh`
   - `bash scripts/check-protocol-control-policy.sh` and `bash scripts/test-protocol-control-policy.sh`
   - the test-debug-a workspace command
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- **C ABI round trip, as PR evidence and not a committed test.** Through
  `miso_engine_v1_submit_command`, a `SESSION_TRANSACTION_APPLY` with `020f` on a submix ID, with
  reliable events drained by `miso_engine_v1_dequeue_event` (`ffi.rs:652`) between commits per
  `docs/CONTROL_PROTOCOL_SEMANTICS.md`: after the replacement boundary the rendered bus level matches
  a plan compiled directly from the edited session at 48 kHz. Attach the run's output.

## Dependencies

- *Tap a submix strip at any of the seven send points* (#1203)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Transactions stay all-or-nothing: an edit that fails leaves the model, revision and replay
  untouched.
- A test that greps source or prose is refused.
- Commit on the K1 batch branch; no push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each.
