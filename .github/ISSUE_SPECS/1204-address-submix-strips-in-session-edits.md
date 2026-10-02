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

### Attempt 1 record (Terra)

- **Sites.** `crates/protocol/src/model.rs`: private `StripMut` (`builtins`, `console`, `inserts`,
  `fader`, `matrix_or_pan`) and `strip_mut(session, id)`, which destructures `SessionModel` into
  `tracks` and `submixes`, searches tracks first, then submixes, else `NotFound` (D4). `0203`,
  `020f`, `0210` and `0211` call it directly; `rack_mut`'s `Inserts` arm and `knobs_mut`'s console
  branch call it, so `0204`-`020e` follow. `rack_mut` still refuses `Builtins` (`NotFound`) and
  `Console` (`ConsoleSlotFixed`) before any lookup. `track_mut` is left only for `0202` (D2).
  Variant/field names unchanged; every `0203`-`0211` ID field documents "Existing strip: a track ID
  or a submix ID, tracks first (#1204)"; the `SetConsole` doc names submix rewrites. No change to
  `controller.rs` was needed: the edit-error strings are unchanged. Docs: registry table rows say
  "strip ID", a *Strip addressing (#1204)* block records D1-D4, and the console-addressing bullets
  say "strip"; `C_ABI_V1_QUALIFICATION.md` has the one line (structural until #1225).
- **No hash move.** `complete_all_opcode_fixture` and `crates/conformance` are untouched;
  `check-protocol-wasm-parity.sh` passes with the existing hash.
- **Tests** (`crates/protocol/tests/submix_strip_edits.rs`, every transaction sent through the
  wire codec first; track `a` carries a strip identical to `bus`'s):
  - `every_strip_opcode_edits_a_submix` (gate 1): one edit per opcode `0203`-`0211` (asserted to
    be exactly that range in order) on submix `bus` -- `020a`/`020d` on the console entry, `0205`
    an insert, `020e` an insert parameter -- commits revision 8 whose canonical snapshot equals the
    base model with only `bus` changed. *Red if any strip opcode still resolves through the track
    lookup or lands on a track's strip; every earlier edit test addresses a track.*
  - `strip_refusals_are_the_tracks` (gate 2): after a valid `020f` on track `a`, `0202` on `bus` and
    `020f` on `main-out` answer `Edit { operation_index: 1, NotFound }` and `0205` with rack
    `console` on `bus` answers `ConsoleSlotFixed`; revision and snapshot unchanged. (The store
    error maps one-to-one to `session.edit.not_found` / `console_slot_fixed` in
    `controller.rs`.) *Red if source assignment reaches a submix, an output ID is treated as (or
    silently skipped as) a strip, or a submix's console slots become structurally editable.*
  - `a_strip_id_resolves_tracks_before_submixes` (gate 3): `0300 x`, `020f x -6`, `0201 x` commits
    submix `x` at 0 dB; the mirror order commits it at -6 dB. *Red if `strip_mut` searches submixes
    first, or only tracks.*
- **Red on revert.** With `model.rs` at `6a4d729d9`, gates 1 and 3 are red; gate 2 is green there
  by design (it pins refusals the old code also gave) and is defended by M5-M7 below.
- **Mutations** (scratch driver, one at a time, `cargo test -p protocol --test
  submix_strip_edits`, file restored; all RED):
  - M1 `strip_mut` searches submixes first -> gate 3.
  - M2 `strip_mut` resolves tracks only -> gates 1 and 3.
  - M3 `knobs_mut`'s console branch resolves tracks only -> gate 1 (`020a`).
  - M4 `rack_mut`'s `Inserts` arm resolves tracks only -> gate 1.
  - M5 `020f` skips an unresolved strip silently (`if let Ok`) -> gate 2.
  - M6 `rack_mut`'s `Console` arm returns the strip's inserts -> gate 2.
  - M7 `0202` skips an unknown track silently -> gate 2.
  - M8 `020f` resolves through `track_mut` -> gates 1 and 3.
- **Gates** (x86-64 AVX2 host, head of this commit's tree):
  - `cargo test --locked -p protocol --features protocol/test-support`: rc 0, 141 passed.
  - test-debug-b (with conformance) rc 0, 787 passed; `conformance_fixtures --check` ok; hash
    unchanged.
  - `check-protocol-wasm-parity.sh`: `issue-005 Wasm golden parity: ok (simd128)`.
  - `check-protocol-control-policy.sh`: ok; `test-protocol-control-policy.sh`: mutation tests ok.
  - test-debug-a workspace command: rc 0, 99 binaries, 1143 passed.
  - `cargo fmt --all -- --check` clean; workspace clippy `--all-targets --all-features -D warnings`
    clean.
- **C ABI round trip** (PR evidence; a scratch test appended to `capi/src/runtime/tests.rs`, run,
  then removed). Session: the nine-track fixture cut to `eq0` -> `bus` (`Submix::unity`) ->
  `main-out`, 48 kHz, 1 kHz sine (L 0.5, R -0.25). Blocks 0-1 rendered through `test_render` on the
  original plan (bit-identical to a direct unedited plan). Then:

  ```text
  miso_engine_v1_submit_command(SESSION_TRANSACTION_APPLY [020f bus -6 dB]) -> result 0, Success at revision 43
  miso_engine_v1_dequeue_event(reliable) -> SessionCommitted at revision 43
  miso_engine_v1_dequeue_event(reliable) -> result 0, 0 bytes: lane drained
  block 2 (replacement boundary): C rms 0.000000, edited direct rms 0.000000, bitwise true
  block 3: C rms 0.086838, edited direct rms 0.086838, unedited direct rms 0.173265; C==edited bitwise true (max |diff| 0e0); C vs unedited -6.000 dB
  block 4: C rms 0.091071, edited direct rms 0.091071, unedited direct rms 0.181711; C==edited bitwise true (max |diff| 0e0); C vs unedited -6.000 dB
  block 5: C rms 0.087106, edited direct rms 0.087106, unedited direct rms 0.173800; C==edited bitwise true (max |diff| 0e0); C vs unedited -6.000 dB
  block 6: C rms 0.087123, edited direct rms 0.087123, unedited direct rms 0.173833; C==edited bitwise true (max |diff| 0e0); C vs unedited -6.000 dB
  block 7: C rms 0.090479, edited direct rms 0.090479, unedited direct rms 0.180529; C==edited bitwise true (max |diff| 0e0); C vs unedited -6.000 dB
  ```

  Block 2 is silent on both sides because the frozen structural source policy resets source state
  at the replacement boundary (`ResetAtReplacementBoundary`); the host then seeks generation 2 at
  frame 384 and resubmits, and the directly compiled edited plan (pre-rolled with two silent
  blocks, the same seek and submissions) matches bit for bit.

## Dependencies

- *Tap a submix strip at any of the seven send points* (#1203)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Transactions stay all-or-nothing: an edit that fails leaves the model, revision and replay
  untouched.
- A test that greps source or prose is refused.
- Commit on the K1 batch branch; no push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each.
