# Validate effect automation units, domains and shapes at preparation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A3, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

Effect-target automation whose unit, values or shape do not fit the parameter's descriptor is
refused, typed, on every path that accepts a session: both host boots, every C ABI rebuild, a C
ABI automation-only transaction (through #1335's classifier route) and `session-validator` stage
`prepare-effects`. Today an effect ride written in `hz` on a gain, a gain ride to +30 dB, or an
`exponential` ride on a dB parameter validates and prepares. Nothing renders the table yet.

## Context

- **#1335's function.** *Refuse automation on effect parameters that are not block-rate* (#1335)
  adds `effect_compiler::effect_automation_diagnostics(model, registry)` (#1335 D2) in
  `crates/effect-compiler/src/prepare.rs`. It resolves each `inserts` or `console` target to its
  instance and descriptor, skips a third-party identity, an effect the registry lacks and a
  parameter the descriptor lacks, and refuses a non-`Block` or non-automatable parameter as
  `effect.automation.rate` at `$.automation[id=<id>].target.parameter_id`.
  `prepare_with_console_eligibility` (`:301`) appends its result before the final check
  (`:526-535`), and `classify_live_delta` routes an automation edit with a non-empty result to the
  rebuild (#1335 D4). It is not on `6ee64f484`; it has landed on `main` since (commit
  `0c19119d0`). The anchors here stay those of `6ee64f484`.
- **What the descriptor says.** `ParameterDescriptor` (`crates/effect-contract/src/lib.rs:443-466`)
  carries `unit`, `domain`, `minimum`, `maximum`, `automation_rate`. Units map one to one to the
  session's (`same_unit`, `crates/effect-compiler/src/prepare.rs:1679-1689`).
  `parameter_value_valid` (`crates/effect-contract/src/lib.rs:599-616`) is the domain test a
  static value passes (`effect.parameter.domain`, `prepare.rs:1666`); a static unit mismatch is
  `effect.parameter.unit_mismatch` (`:1627`).
- **The launch `Block` parameters** (`sdk/src/generated/catalog.ts`, `"automationRateName":
  "block"`): 56, of which 54 are continuous and 2 boolean; none is an enumeration. Units: `db`,
  `hz` (minimum 10), `linear` (minimum -1, -0.95 or 0), `milliseconds` (minimum 0.1 to 10),
  `ratio` (minimum 0.1 or 1).
- **Frozen effect codes.** `docs/EFFECT_CONTRACT_V1.md:206-229` ("Stable diagnostics"); #1335 D6
  adds `effect.automation.rate`.
- **Session and builtin rules** are drafts 01 and 02: one entry per lane, pan or matrix by strip
  kind, discontinuity spacing, and builtin unit, domain, shape and filter order.
- **Checked-in documents.** `fixtures/session/v1/canonical.json` automates an insert of the
  unregistered identity `parametric-eq` (#1335's skip applies); the skill's
  `.claude/skills/author-session/worked-session.json` rides a console EQ gain from 2.5 to 4.0 dB,
  `linear`, `db`.

## Decisions frozen for this slice

- **D1. Three rules in #1335's function.** For each segment `k` of an entry #1335 D2 resolves and
  does not refuse, at `$.automation[id=<id>].segments[<k>].<field>`:
  - **Unit.** `same_unit(segment.unit, descriptor.unit)`. Else `effect.automation.unit` at
    `.unit`, and no other rule is checked for that segment.
  - **Domain.** `parameter_value_valid(descriptor, value)` for `start_value` and `end_value`.
    Else `effect.automation.domain` at that field.
  - **Shape.** A boolean or enumeration parameter takes only `step`. `exponential` is allowed only
    on a continuous parameter whose `minimum` is strictly positive and whose unit is not `db`.
    Else `effect.automation.shape` at `.shape`.
- **D2. One fault, one diagnostic.** An entry refused as `effect.automation.rate` gets none of
  these. #1335's skips apply unchanged.
- **D3. Frozen codes.** The three codes join `docs/EFFECT_CONTRACT_V1.md`'s frozen list beside
  `effect.automation.rate`. The `author-session` skill's automation refusal table (draft 01) gains
  them at stage `prepare-effects`.
- **D4. No classifier code.** #1335 D4 routes a C ABI automation edit with any diagnostic of the
  function to the rebuild, whose preparation refuses it.
- **D5. The acked-batch question** does not arise: refusals happen at preparation, before
  anything is committed.

## Deliverables

1. D1-D2 in `crates/effect-compiler/src/prepare.rs`.
2. D3 in `docs/EFFECT_CONTRACT_V1.md` and `.claude/skills/author-session/SKILL.md`.
3. The tests below.

## Authorized paths

- `crates/effect-compiler/src/prepare.rs` (the function #1335 adds, only),
  `crates/effect-compiler/tests/native_session.rs`
- `crates/capi/src/runtime/live_tests.rs` (tests only)
- `tools/session-validator/tests/validate.rs`
- `docs/EFFECT_CONTRACT_V1.md` (the frozen list), `.claude/skills/author-session/SKILL.md` (the
  automation refusal table)

## Non-goals

- The rate rule (#1335). Session-level and builtin rules (drafts 01, 02).
- The SDK mirror (draft 03b) and submix authoring (draft 03c).
- Rendering effect automation (slices 17a to 20).

## Hazards

- **Hand-built test automation.** C ABI and host-core tests that build effect automation by hand
  must stay in the descriptor's unit and domain; a test that now fails is fixed in its value,
  never by relaxing a rule.
- **Stream ownership.** `prepare.rs` is edited by #1306, #1315 and #1345 too; root sequences the
  merge.

## Objective gates

1. **Preparation refuses** (`crates/effect-compiler/tests/native_session.rs`, new). On the
   observation-frame fixture (an EQ insert) plus one automation of the EQ's `band-1-gain` (4):
   unit `hz` gives `effect.automation.unit`; a value of `30` gives `effect.automation.domain`; an
   `exponential` segment gives `effect.automation.shape` (unit `db`); an `exponential` segment on
   `band-1-frequency` (3) from `100` to `1000` prepares. The same refusals hold for a console-slot
   target and a submix target. An entry on `band-1-enabled` gives only #1335's
   `effect.automation.rate`.
2. **Validator** (`tools/session-validator/tests/validate.rs`, new row). A unit-mismatched EQ gain
   entry fails at stage index 4 (`prepare-effects`) with `effect.automation.unit`; stages 0-3 pass.
3. **C ABI live path** (`crates/capi/src/runtime/live_tests.rs`, new). On a playing engine, a
   transaction that only adds the gate-1 domain refusal is refused with the typed compile
   rejection; the revision does not advance and the next blocks are bit-identical to an engine
   that never received it.
4. **No rendered bit moves.** No render code changes. `cargo build --locked --release -p audit -p
   bench -p capi -p session-validator`, then `./target/release/audit capi`, shows the same
   `pcm_digest` at base and head (PR evidence); the browser legs of the `browser` job in
   `.github/workflows/qualification.yml` pass with unchanged digests.
5. **Commands:**
   - `cargo test --locked -p effect-compiler --features test-support`,
     `cargo test --locked -p session-validator`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-effect-runtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red for each rule left out, for an `exponential` rule that admits `db` or refuses a
  positive-domain frequency, for a check that skips console or submix targets, or for a second
  diagnostic on a rate-refused entry. No test reads an effect segment's unit or values today.
- Gate 2 turns red if the rules run where `session-validator` does not.
- Gate 3 turns red if a C ABI automation-only edit commits a segment its own rebuild refuses.

## Dependencies

- Draft 01 *Validate stored automation lanes in the session crate and state the hold rule* (the
  skill's automation refusal table).
- Draft 02 *Validate builtin automation targets against their rows at preparation* (the builtin
  rules these effect rules complete, the shared path form and the classifier call beside which
  #1335's function runs).
- *Refuse automation on effect parameters that are not block-rate* (#1335): D1 extends its
  function and relies on its classifier route.
- Batch: R3. Draft 03b *Mirror the stored automation rules in the SDK builder* builds on it.
