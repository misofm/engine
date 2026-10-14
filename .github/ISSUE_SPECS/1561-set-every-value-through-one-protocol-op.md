# Set every value through one protocol op

Slice S9 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C5).
Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The protocol has one way to set a value: `SET_CONTROL`. The per-kind value ops are removed, their
codes are refused and never reallocated, so an agent cannot pick between two ops that might
drift. Structural ops (add, remove, reorder, identity, quality, sidechain, source mapping, route
topology, VCA membership, console slot declaration) stay.

## Context

- **The opcode table** is `SessionEditOpcode` (`crates/protocol/src/model.rs:23-120`) with
  `from_raw` (`:131-182`). An unallocated code fails decode with `DecodeError::InvalidTlv`
  (`parse_edit`, `crates/protocol/src/session_wire.rs:1225-1231`). The registry test
  `opcode_registry_appends_the_console_edits_and_reallocates_nothing` (`model.rs:1864`) pins the
  count, 47 on `main`; #1553 adds `SetControl` `0x0800`, so 48.
- **The value ops** (ruling C5), with their payloads:
  - `SetControlSmoothing` `0x0008` (`control_smoothing`, `model.rs:220`);
  - `SetTrackBuiltins` `0x0203` (the whole `DualMonoBuiltins`: polarity, trim, HPF, LPF and
    `delay_samples` per lane, `:251`). Every field is a builtin control row (#1546 D3 rows 1-4 and
    11), so the op has no use that is not a value;
  - `SetEffectBypass` `0x020a`, `SetEffectLinkMode` `0x020b`, `UpsertEffectParam` `0x020d`,
    `RemoveEffectParam` `0x020e`;
  - `SetTrackFader` `0x020f` (`:376`), `SetTrackMatrixOrPan` `0x0210` (`:385`; pan, matrix and the
    window are rows: pan, `matrix_*`, `matrix_mode`, `pan_window_samples`);
  - `SetTrackConsole` `0x0211` (`:398`): one strip's whole console entry array (`bypass` and
    `params` per slot). It is also the way to give each strip its entries when `SetConsole`
    changes the slot set in the same transaction (`SetConsole`'s doc, `model.rs:200-208`);
  - `SetRouteChannelMatrix` `0x0504`, `SetRouteGainDb` `0x0505`, `SetRouteMute` `0x0506`,
    `SetRouteFollowsMute` `0x0507`;
  - `SetVcaFader` `0x0702` (`:485`).
- **Link mode is a row.** #1549 D4 adds an insert's link mode as the row `LINK_MODE_CONTROL =
  0x8000_0000` of its insert section (for an effect with more than one supported mode; effect
  parameter IDs stay below `0x8000_0000`, coordinator ruling 2026-10-13). A console slot's link mode
  is a slot declaration, set by `SetConsole`, and has no row.
- **Per-op ramps.** `record_edit_ramps` (`model.rs:615`) maps each op to `EditRampTarget` and
  `EditRampRow` (`crates/session/src/edit_ramp.rs:18-61`). #1553 gives `SetControl` its own entry.
- **Uses to migrate** (counted on `main`, Rust only, `crates`, `hosts`, `tools`): about 270 uses of
  the 14 ops in about 30 files: protocol unit and integration tests
  (`crates/protocol/src/{controller/tests.rs,session_wire/tests.rs}`, `crates/protocol/tests/*`),
  the conformance corpus (`crates/conformance/src/protocol_corpus.rs`, which pins
  `COMPLETE_SCHEMA_HASH`), `tools/audit/src/protocol.rs` and `tools/audit/src/capi.rs`, and tests
  in `crates/control-plane`, `crates/capi` and `crates/host-core`. The SDK has no transaction
  encoder on `main` (#1383 is open).

## Decisions frozen for this slice

- **D1. Retired codes.** Remove these variants from `SessionEdit` and `SessionEditOpcode`, their
  `from_raw` arms, their schema specs and their codecs: `0x0008`, `0x0203`, `0x020a`, `0x020d`,
  `0x020e`, `0x020f`, `0x0210`, `0x0504`, `0x0505`, `0x0506`, `0x0507`, `0x0702`. Each code is
  refused on decode as every unallocated code is, and the registry test lists them as retired,
  never reallocated, beside `0x0006`, `0x0102` and `0x0104`. See D2 for the count.
- **D2. `SetEffectLinkMode` (`0x020b`) is retired too.** An insert's link mode is set with
  `SET_CONTROL` on #1549's `LINK_MODE_CONTROL` row; a console slot's with `SetConsole`. Its tests
  move to those two forms. With D1, 13 codes are retired, and the allocated count becomes 35.
- **D3. `SetTrackConsole` keeps its structural use only.** It stays allocated. It is accepted only
  in a transaction that carries a `SetConsole` edit before it; otherwise the transaction is
  refused before any write with `session.edit.value_op_retired` at that edit's path, and nothing
  commits. The op's doc says: set console knob values with `SET_CONTROL`.
- **D4. Structural ops keep their bodies.** `UpsertTrack`, `UpsertSubmix`, `UpsertRoute`,
  `UpsertVca`, `PutTrackEffect`, `SetTrackRack` and `SetConsole` keep every field: their values
  are an entity's declaration, not a value edit. The classifier reads models, not ops, so a value
  that one of them changes is classified by its row exactly as a `SET_CONTROL` is.
- **D4a. `control_smoothing: default` after `SetControlSmoothing` goes** (coordinator ruling,
  2026-10-13). A transaction can no longer write the `default` arm; only a document replace can.
  Rendered behaviour is identical: `SET_CONTROL` of the three session rows (`fader_ms`, `mute_ms`,
  `pan_ms`) to the default table's values stores an `explicit` arm whose lengths, rounded at the
  session rate by `LiveRamps::for_session`, equal the default arm's, so every ramp is the same
  number of samples. Only the canonical document differs (`explicit` instead of `default`).
- **D5. Ramps.** Delete the `record_edit_ramps` arms of the retired ops, and every
  `EditRampTarget` and `EditRampRow` variant that no remaining op writes. Keep what `SetConsole`
  (row `Link`) and `SetControl` write.
- **D6. Migration, not deletion.** Every test that used a retired op to make a value edit is
  rewritten with `SET_CONTROL` and keeps its claim. A test whose only claim was the retired op's
  own codec is deleted. Add one test-support constructor in `crates/protocol`, for example
  `SessionEdit::set_control(address, value, ramp)`, so each rewrite is one line.
- **D7. Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md` marks the codes retired.
  `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md` name `SET_CONTROL` as
  the value op wherever they name a retired one.

## Deliverables

1. D1-D5 in `crates/protocol` and `crates/session/src/edit_ramp.rs`.
2. D6's migration in every listed file; the conformance corpus re-pinned once.
3. D7.

## Authorized paths

- `crates/protocol/src/**`, `crates/protocol/tests/**`.
- `crates/session/src/edit_ramp.rs`.
- `crates/conformance/src/protocol_corpus.rs`.
- `tools/audit/src/**`.
- Tests only: `crates/control-plane/src/**` (test modules and `#[cfg(test)]` helpers),
  `crates/capi/src/runtime/{live_tests.rs,live_peak_tests.rs,tests.rs}`, `crates/capi/tests/**`,
  `crates/host-core/tests/**`, `hosts/host-web/tests/control_parity.rs`.
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/C_ABI_V1_QUALIFICATION.md`,
  `crates/capi/include/miso_engine_v1.h` (prose).

## Non-goals

- No change to `SET_CONTROL`, the classifier or render.
- No SDK change (#1383 and #1385 encode `SET_CONTROL` when they land).
- No change to structural ops' payloads (D4).

## Objective gates

Commands: `cargo test --locked --workspace --all-features`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`,
`bash scripts/check-capi-abi.sh`.

1. **Retired codes refuse.** For each retired code, a `SESSION_TRANSACTION_APPLY` frame whose edit
   carries it (with any payload) is refused on decode, and the session is unchanged. The registry
   test's count and retired list pass.
2. **Console values only through `SET_CONTROL`.** A transaction with only a `SetTrackConsole` that
   changes a knob is refused `session.edit.value_op_retired`; a transaction of `SetConsole` (adding
   a slot) then `SetTrackConsole` for each strip commits.
3. **Claims kept.** Every migrated test passes. `audit capi` reports the same `pcm_digest` as the
   merge base (PR evidence).

## Test value

- Gate 1 turns red if a retired op is still accepted, so an agent could set a value by two paths.
- Gate 2 turns red if `SetTrackConsole` stays a second value path, or if its structural use breaks.
- Gate 3 turns red if a migration changed what a test proves (a value edit that now rebuilds, or a
  different render).

## Dependencies

- #1553 (`SET_CONTROL`), #1556, #1557, #1558, #1559 (every retired op's rows are reachable through it).
- #1549 (effect rows: bypass, parameters and the link-mode row).

## Supersedes and shrinks

- Shrinks *Build and encode session transactions in the SDK* (#1383): every value edit is one
  `SET_CONTROL` encoder.
- Shrinks *Encode the session, submix, output, route, automation and VCA edits in the SDK*
  (#1385): the route, VCA and `control_smoothing` value ops are gone; the structural encoders stay.

## Evidence record

Batch 4 review fold, 2026-10-14. Gate 3 (claims kept), `audit capi` on the merge base `205a8d2b4` and
on the batch 4 fold head: `{"kind":"issue022_capi_render_audit","calls":100000,"sample_rate_hz":48000,"quantum_frames":128,"pcm_digest":"76525e7ef9e78cf4","total_violations":0}`
(`cargo run --locked --release -p audit -- capi`). The two digests are equal. The fold also fixes
D3's accept rule: `SetTrackConsole` is accepted only after a `SetConsole` of the same transaction
that changes the console (the candidate console differs from the committed one); a `SetConsole` that
repeats the committed console does not open it.
