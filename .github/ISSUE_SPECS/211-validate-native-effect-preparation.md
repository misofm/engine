# Session validator CLI passes out-of-domain effect parameters (4 stages stop before effect preparation)

## Current approved scope — 2026-09-10

### 2. #211 — make CLI validation check native effect preparation

Product outcome: a session with an invalid compressor ratio cannot receive CLI PASS and subsequently fail for that same reason during engine preparation.

Current evidence: `tools/session-validator/src/lib.rs:51` lists four stages; `validate_session_document` finishes after `prepare_session_builtins`. It never calls `prepare_native_session_effects`.

Smallest scope: add `prepare-effects` using the existing effect compiler and appropriate preparation caps; translate its existing typed diagnostics into the CLI report; update stage tables, focused mutations and authoring documentation. Preserve refusal ordering, skipped later stages and canonical JSON behavior. Cover unknown effect IDs, unavailable third-party resolution, ports and quality through existing preparation semantics rather than duplicating validators.

Acceptance: the next f32 above compressor ratio 20 is rejected at the effect stage; 20 and representative valid registered effects pass. Earlier syntax/model/builtin refusals remain correctly classified. Prove parity with existing engine effect preparation for these cases; CLI exit status and dotted diagnostics are tested. Required PR/main CI and synchronized closure.

Scope trap: canonical.json contains an intentionally unregistered schema-example effect ID. Existing tests that treat every schema-valid fixture as fully preparable must distinguish schema-only fixtures from launch-registry fixtures. Do not rewrite sealed canonical fixtures to make the new stage pass. CLI PASS still does not certify graph/PDC, source availability or all host resource constraints.

Risk/size: small-to-medium, control-plane tooling; no DSP algorithm change or independent artifact repin expected. Refresh its old TOML/crate/model-assignment wording in the matching stateless spec before implementation.


Independent Astra XHIGH scoping PASS; user authorized implementation. Astra LOW implements, Astra XHIGH verifies. Five attempts maximum. Root checkpoints exact paths and pushes promptly. At most two active issues: #387 and #211; #376 queued. #211 is independent tooling, #387 owns session/parser and boundary regression source. Root owns artifact qualification/pinning. No overlapping production edits.

Implementation stops after focused green checks for root commit; independent review follows. Record actual commands, environments, source identity, exits and logs externally. No compiler-IR captures or timing campaigns. Preserve original PR history and failed evidence. Required exact-head PR qualification and corrected-main qualification precede synchronized closure and clean worktree removal.

Before implementation only: current source base14079d2c, no new code or PASS evidence. Historical instructions below are provenance; this current scope and user model/five-attempt routing supersede them.

## Historical issue body

Found during SDK #207 Phase 1 E5 (credit: the SDK implementation agent). The #179 validator CLI's four stages (toml-grammar → typed-model → compile-session → prepare-builtins) never run native effect preparation, so an out-of-domain **effect** parameter — e.g. compressor `ratio = 20.000002`, the next f32 above the declared max 20.0 — passes **all four stages**, while the real engine pipeline refuses it at prepare (`prepare_native_session_effects`, crates/miso-engine-effect-compiler/src/prepare.rs:819, surfaced through host-core prepare.rs:528 as `PrepareRejection::Effect` typed diagnostics).

Impact: the author-session skill and any CLI-validated workflow can produce documents that validate PASS but are refused by the engine — exactly the false-confidence class the validator exists to prevent. Builtins/schema/model domains are unaffected.

Fix direction: add a fifth stage (`prepare-effects`) running `prepare_native_session_effects` with the standard caps and mapping its `EffectDiagnosticSet` into the CLI's dotted-diagnostic report — or fold it into stage 4 as `prepare-session`. Ceremony: STAGE_NAMES, the validator's mutation tables gain out-of-domain effect rows (the 20.000002 probe becomes the red test), author-session skill docs, and #207's E5 note ('CLI PASS necessary but not sufficient') retires once fixed.

Standard protocol: Opus implements, Fable adversarially verifies pre-PR.