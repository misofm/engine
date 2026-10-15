# Refuse an effect-row edit whose effect changed, in the core

Successor of the batch 12b review (m4), decision 17. Code anchors verified on the batch 12b branch `codex/controls-batch-12b`. Issue number: assigned at filing (written here as #1678). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A `SET_CONTROL` edit on an effect section's row names the native effect it was written for, and
the core refuses it with a typed code when that slot or insert now runs another effect. Every
client (SDK, C ABI app) gets the same protection, and the SDK stops probing the control list
before each named-row edit.

## Context

- #1562b-K resolves named effect rows from the control list's section headers and checks each
  effect-row edit just before encoding (`effectRowResolver`, `sdk/src/core/live-controls.ts:122`),
  at the cost of a list probe per transaction; C ABI clients have no such check, so after a rebuild
  a numeric effect row can land on another effect's parameter.

## Decisions frozen for this slice

- **D1.** An effect-section `SET_CONTROL` address may carry the expected native effect ID; the
  core refuses a mismatch with a typed code before commit, nothing changes.
- **D2.** The SDK sends the expected ID from its builders and deletes the resolver hook.

## Objective gates

1. After a rebuild that changes an insert's effect, an edit written for the old effect is refused
   with the typed code on both hosts (parity row); the same edit without the expected ID keeps
   today's behaviour.
2. Wire re-pins with the repository tooling; SDK evals; protocol, control-plane, capi, host-web
   tests.

## Dependencies

- #1562b-K.

## Size

Half a day to a day.

## Decision record: #1678 (batch 13b)

**What shipped.**

- Wire: `SET_CONTROL`'s address is its own message, `SetControlAddress` (`schema::set_control_address`): the list's six address fields and an optional `7:expected-effect UTF8`. The control list's address message and a stored automation target keep `ControlAddress` (the session JSON keys pair with it, so field 7 would break that pairing). Encoder and decoder: `write_set_control_edit_address` / `decode_set_control_edit_address` in `message_wire`; the field is refused as `INVALID_FIELD` unless the section is `inserts` or `console` and the value is a stable ID.
- Model: `SessionEdit::SetControl` gains `expected_effect: Option<StableId>`; `SessionEdit::set_control` keeps its signature (no expected effect) and `with_expected_effect` adds one.
- Core check: `controls::check_expected_effect` (in `effect_rows.rs`) returns the new `ControlError::EffectChanged`, code `control.effect.changed`. It runs in `apply_session_edit` (full path, against the candidate at the edit's place in the transaction, so a transaction that replaces the effect first is judged by the new one) and in `prepare_in_place` against the committed model; the in-place path declines on a mismatch and the full path answers with the typed refusal. An address that names no insert or slot is still `control.address.unknown`. Both hosts reach this through `SessionStore`, so there is no host code.
- SDK: the builders send the expected effect (typed effect of an effect builder, the effect of the section for a named row, `expectedEffect` of a numeric `ControlWireAddress`); `effectRowResolver`, its provenance map and the control queue's `prepare` hook (its only user) are deleted, so a named-row edit no longer probes the control list. A named row follows no header any more: an edit built from the old view is sent for the old effect and the core refuses it.
- Re-pins: `COMPLETE_SCHEMA_HASH` `0xcc9a_7bcc_502c_4936` to `0x63ff_2e60_976e_0e21` (the corpus transaction appends a `SetControl` on an insert row with expected effect `miso.parametric-eq`: 36 edits; the controller test's edit-limit boundary is now 35); parity self-test strings, `fuzz/corpus/complete-schema-manifest.md`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md` history lines; `crates/protocol/tests/fixtures/set-control-frames.json` blessed (new `expectedEffect` key and an `expected_effect` command). `docs/CONTROL_PROTOCOL_REGISTRY.md` documents field 7 and the code. No C ABI vector carries the field. No rendered bit moved (audit capi `pcm_digest` `bf7dcedd1d92621f`).
- Integrated in batch 13b: one re-pin from `0xcc9a_7bcc_502c_4936` to `0x0c71_96f8_6abe_6d57` for #1635 and #1678 together.

**Departure.** The spec says the codec "gains an optional field"; the field is on a new message that shares fields 1 to 6 with the list's address, not on the shared message (reason above).

**Gates -> tests.**

1. `hosts/host-web/tests/control_parity.rs::a_stale_effect_row_edit_is_refused_alike_on_both_hosts` (rebuild replaces `t2`'s transient shaper by a soft clip; the stale edit gets one verdict, one reply, `control.effect.changed`, unchanged control rows and document, equal blocks; the same edit without the expected effect, and with the new one, commit live on both hosts, at four rates). Store level, both preparation paths: `crates/protocol/tests/expected_effect.rs` (red with the check removed). Wire: `session_wire::tests::set_control_expected_effect_is_held_to_its_section`, `crates/protocol/tests/set_control_frames.rs` fixture, `sdk/test/control-frame-evals.mjs`.
2. `sdk/test/structure-evals.mjs` ("#1678", headless and browser transport), `sdk/test/control-builders-evals.mjs` (every effect row carries `expectedEffect`).
3. Run: protocol (`test-support`), control-plane (`test-support`), controls, capi, conformance, host-core, host-web (`host-web/test-support`), parameter-metadata; clippy `-D warnings` on host-core (no features, default, all features) and the touched crates; fmt; `check-protocol-wasm-parity.sh` and `--self-test`; command kind and reason vocabulary checks; fuzz `cargo check`; protocol allocation audit; SDK types, generated, headless (557 evals), package; realtime-policy; bench-policy; chromium qualification.
