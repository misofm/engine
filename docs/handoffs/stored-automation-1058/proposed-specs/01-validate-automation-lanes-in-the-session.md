# Validate stored automation lanes in the session crate and state the hold rule

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A3, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

Every session the engine accepts gives each automated lane exactly one renderable curve. Session
validation, which every path runs (both host boots, every C ABI transaction, `session-validator`
stage `typed-model`), refuses three shapes it accepts today:

- two automation entries that drive one lane (a `both` fader ride and a `left` fader ride on one
  track);
- a pan target on a strip that declares a matrix, or a matrix target on a strip that declares a
  pan;
- two discontinuities of one entry closer than 64 samples.

`docs/SESSION_SCHEMA_V1.md` states what the table means: the hold rule of A3, and these rules.
This slice reads no descriptor; unit, domain and shape rules are drafts 02 and 03a. Nothing renders
the table yet.

## Context

- **What session validation checks today.** `validate_automation`
  (`crates/session/src/validate.rs:890-1030`) checks that the target exists, that segments are
  ordered and do not overlap (`:972-1000`), the unit-local value rules (`validate_unit_value`,
  `:1032-1062`) and that exponential values are positive (`:1013-1027`). Across entries it checks
  only that automation IDs are unique (`:134-145`). Nothing compares two entries' targets, so two
  entries on one lane validate.
- **The builtin target check** (`validate_builtin_automation_target`, `:854-888`) checks the
  parameter ID against `BUILTIN_AUTOMATION_TARGETS` (`:823-840`; pan is 12, the matrix rows are
  7-10) and that a matrix row is addressed `both`. It does not look at the strip.
- **A strip is either a pan or a matrix.** `MatrixOrPan::{Pan, Matrix}`
  (`crates/session/src/model.rs:655-680`) is one tagged field of a track or a submix; both drive
  the strip's one 2x2 matrix stage.
- **Validation runs on every path, before normalization.** `compile_session` calls
  `validate_session` before it sorts anything (`crates/session/src/compile.rs:123-129`; the table
  is sorted by `id` at `:155-157`), so diagnostic indices are declared positions. The protocol
  compiles every transaction candidate (`crates/protocol/src/model.rs:944`), so the C ABI live path
  runs these rules too.
- **Session diagnostic vocabulary.** Codes are variants of `DiagnosticCode`
  (`crates/session/src/diagnostic.rs:17-76`) with a stable dotted spelling in `as_str`
  (`:78-115`); the automation codes are `automation.out_of_order`, `automation.segment_overlap`
  and `automation.invalid_range` (`:104-106`). No separate frozen list exists:
  `docs/SESSION_SCHEMA_V1.md` names each code where it states the rule (for example `vca.cycle`,
  `:74`), and the `author-session` skill keeps refusal tables by code
  (`.claude/skills/author-session/SKILL.md:129-136`, `:189-199`) and an automation paragraph
  (`:107-116`). An effect parameter absent from its instance is `reference.missing_entity` at
  `.target.parameter_id` (`validate.rs:944-956`).
- **The grid.** Render evaluates a moving curve on a fixed 64-sample grid and takes a jump at each
  discontinuity's exact sample (note A1.4). 64 is the input filters' and the EQ's fixed
  coefficient ramp (`crates/builtins/src/filter_control.rs:9`;
  `crates/parametric-eq/src/lib.rs:116-119`) and the smoothing length of 55 of the 56 launch
  `Block` parameters.
- **Checked-in documents with automation.** `fixtures/session/v1/builtins-automation.json`
  automates track `vocal`, which declares a `pan`, with a fader ride, a **`matrix_ll` (7)** ride,
  a polarity step and a trim ride. Its SDK twin `builtinsAutomationFixture`
  (`sdk/test/builder-evals.mjs:1104-1140`) is compared with it byte for byte (`:1041-1048`).
  `tools/session-validator/tests/validate.rs:269-305` mutates the fixture's target rows, the
  matrix entry among them. `fixtures/session/v1/canonical.json` and the writer corpus's
  full-surface document automate an insert; `.claude/skills/author-session/worked-session.json` a
  console EQ gain. None of them has two entries on a lane or a discontinuity pair.
- **The hold rule has no home in the schema doc.** `docs/SESSION_SCHEMA_V1.md:220-225` says only
  that the table renders nothing. The protocol's record rule ("holds its end value until
  replaced", `docs/CONTROL_PROTOCOL_REGISTRY.md:45`) leaves the registry with *Delete the
  protocol automation queue, its records and counters* (draft 21c).

## Decisions frozen for this slice

- **D1. One entry per lane.**
  - Two entries share a lane when their `entity_id`, `rack`, `effect_id` and `parameter_id` are
    equal and their channels overlap: `both` overlaps every channel, `left` overlaps `left`,
    `right` overlaps `right`. A `left` and a `right` entry on one parameter are two lanes.
  - Of each overlapping pair, the entry whose `id` is greater (later in canonical order) is
    refused, once however many entries it overlaps, with a new
    `DiagnosticCode::AutomationDuplicateLane`, spelled `automation.duplicate_lane`, at
    `$.automation[<i>].target.channel`, `<i>` its declared position. So the refused entry does not
    depend on declared order.
  - The pass is one sort of `(entity_id, rack, effect_id, parameter_id, id)` keys, `O(n log n)`.
    It skips an entry whose target is already refused and an entry whose `id` repeats an earlier
    one (already `id.duplicate`), so no entry is reported twice.
- **D2. Pan only on a pan strip, matrix only on a matrix strip.** A `builtins` target with
  `parameter_id` 12 is valid only when its strip's `matrix_or_pan` is `Pan`; 7-10 only when it is
  `Matrix`. Else `reference.missing_entity` at `$.automation[<i>].target.parameter_id`, the code
  and path an effect parameter absent from its instance gets: the strip has no such parameter.
  With D1, a strip's 2x2 stage is driven by at most one entry per lane of one family.
- **D3. Discontinuities at least 64 samples apart.**
  - Under the hold rule (D4), sample `t` of an entry is a **discontinuity** when the value at `t`
    differs, by `f32` bits, from the value just before `t`. Only segment boundaries can be one: a
    segment's `start_sample` whose `start_value` differs from the value held just before it (the
    previous segment's `end_value`; never the first segment, whose value is held before it), and
    a `step` segment's `end_sample` when its `end_value` differs from its `start_value`. Two
    boundaries at one sample are one discontinuity.
  - Two discontinuities of one entry less than 64 samples apart are refused with a new
    `DiagnosticCode::AutomationDiscontinuitySpacing`, `automation.discontinuity_spacing`, at the
    later one's field (`$.automation[<i>].segments[<k>].start_sample` or `.end_sample`), once per
    offending pair.
  - 64 is the least grid period `G` of A1.4 (`G` is 64, or the cell's ramp length when that is
    larger), a fixed engine constant, not a session value. The rule allows at most one jump per
    lane in any 64-sample window, so no jump starts inside a fixed 64-sample coefficient ramp that
    an earlier jump of the same lane began, and the jump count of A1.7 is at most `⌈q/64⌉ + 1` per
    cell and block.
- **D4. The schema doc states the meaning.** In `docs/SESSION_SCHEMA_V1.md`, after `:225`:
  - **The hold rule.** Before the first segment a lane holds the first `start_value`. Inside a
    segment it follows the segment: `linear` is `v0 + (v1 - v0)·x`, `exponential` is
    `v0·(v1/v0)^x`, with `x = (t - t0)/(t1 - t0)`. After a segment, and in a gap, it holds that
    segment's `end_value`. A `step` segment holds `start_value` on `[start, end)`, and the lane
    takes `end_value` at `end`.
  - D1-D3 with their codes, and a pointer to the preparation rules (unit, domain, shape and filter
    order against the target's row: drafts 02 and 03a).
  - The sentence that the table renders nothing stays; slice 09b changes it.
  - The skill's automation paragraph gains the hold rule, and a new "Automation refusals, by
    code" table, in the form of the VCA table, lists D1-D3.
- **D5. Fixture migration.** `builtins-automation.json` gains a second track, `keys`, that
  declares a `matrix`, reads the same source and routes to `main-out`. The `matrix-ll` entry moves
  to `keys`; `vocal` keeps its pan. The engine's canonical writer writes the file, and
  `builtinsAutomationFixture` gains the same track, so the SDK twin stays byte-identical. The
  validator rows at `validate.rs:269-305` keep their find strings, stages and codes.
- **D6. The acked-batch question** does not arise: every refusal is a validation diagnostic before
  anything is committed. No queue changes.

## Deliverables

1. D1-D3 in `crates/session/src/{validate,diagnostic}.rs`.
2. D4 in `docs/SESSION_SCHEMA_V1.md` and `.claude/skills/author-session/SKILL.md`.
3. D5's fixture and its SDK twin.
4. The tests below.

## Authorized paths

- `crates/session/src/validate.rs`, `crates/session/src/diagnostic.rs`, `crates/session/tests/`
- `fixtures/session/v1/builtins-automation.json`; in `sdk/test/builder-evals.mjs`,
  `builtinsAutomationFixture` only
- `tools/session-validator/tests/validate.rs`
- `docs/SESSION_SCHEMA_V1.md`, `.claude/skills/author-session/SKILL.md` (automation paragraph and
  an automation refusal table only)

## Non-goals

- Any descriptor check: builtin rows are draft 02, effect parameters draft 03a and #1335.
- The SDK builder's mirror of these rules (draft 03b).
- Rendering any automation (slices 07 to 20). The live classifier.
- New shapes, fields or per-segment ramp lengths (A3 keeps the table as it is).

## Hazards

- **The fixture migration moves session-document bytes.** Any digest of
  `builtins-automation.json`'s bytes moves with D5; list each with the reason "D5 moves the matrix
  ride to a matrix strip" and re-pin it on its own. No render digest reads this fixture's
  automation, because nothing renders it.
- **`-0.0` against `0.0`.** D3 compares bits, so a hold at `0.0` followed by a segment that starts
  at `-0.0` is a discontinuity. That is deliberate: render's change test compares bits too
  (`crates/host-core/src/solo.rs:58-67`).
- **Order of checks.** D3 reads segment order. It runs only on an entry whose segments passed the
  order and overlap checks, so it never reports on a malformed sequence.

## Objective gates

1. **Refusals with parity** (`crates/session/tests/diagnostic_parity.rs`, new rows in the case
   table, so the text and typed entry points agree):
   - the canonical fixture's automation replaced by two `builtins` fader entries (5) on its track,
     the second with the greater `id`: `both`/`both`, `both`/`left` and `left`/`left` each give one
     `automation.duplicate_lane` at `$.automation[1].target.channel`; the fixture's own insert
     entry plus a copy with a greater `id` gives the same;
   - a `matrix_ll` (7) entry on the fixture's pan track, and a pan (12) entry after its
     `matrix_or_pan` is set to a matrix, each give `reference.missing_entity` at
     `$.automation[0].target.parameter_id`;
   - a `step` segment `[0, 960)` from `0` to `1`, then a `linear` segment from `1023` that starts
     at `0.5`, give `automation.discontinuity_spacing` at
     `$.automation[0].segments[1].start_sample` (discontinuities at 960 and 1023).
2. **Acceptances and order** (`crates/session/tests/automation_lanes.rs`, new). A `left` and a
   `right` entry on one parameter compile. Two overlapping entries declared in either order refuse
   the one with the greater `id`; three give two diagnostics. Discontinuities exactly 64 samples
   apart compile; a segment that starts at the held value is not a discontinuity; a `step` whose
   two values are equal has none.
3. **Validator stage** (`tools/session-validator/tests/validate.rs`, new rows in `MUTATIONS`). In
   the migrated `builtins-automation.json`, a duplicate fader lane and a pan entry on `keys` each
   fail at stage index 1 (`typed-model`) with their codes.
4. **Every checked-in document still validates.**
   `fixtures_distinguish_schema_examples_from_launch_effects`
   (`tools/session-validator/tests/validate.rs:81`), the skill test
   (`tools/session-validator/tests/skill.rs`) and the SDK's byte comparison of the twin
   (`sdk/test/builder-evals.mjs:1041-1048`) pass.
5. **No rendered bit moves.** No render code changes. `cargo build --locked --release -p audit -p
   bench -p capi -p session-validator`, then `./target/release/audit capi`, shows the same
   `pcm_digest` at base and head (PR evidence); the browser legs of the `browser` job in
   `.github/workflows/qualification.yml` pass with unchanged digests.
6. **Commands:**
   - `cargo test --locked -p session`, `cargo test --locked -p session-validator`,
     `cargo test --locked -p protocol --features test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `bash scripts/check-session-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if overlap is not refused, if `both` is not treated as overlapping one channel,
  if a pan or matrix target is accepted on the wrong strip kind, if a step's end or a segment that
  starts away from the held value is not counted as a discontinuity, or if a code or path differs
  between the text and typed entry points. No test compares two entries' targets, a target with
  its strip kind, or two segment boundaries today.
- Gate 2 turns red if a rule is too wide: `left` plus `right` refused, a 64-sample spacing
  refused, a continuous segment start or an equal-valued step counted, or the refused entry
  depending on declared order.
- Gate 3 turns red if a rule runs anywhere other than session validation.

## Dependencies

- None.
- Batch: P1, first in it. Drafts 21a-21c (the hold rule's new home before 21c deletes the old one),
  02, 03a and 03b and draft 07 (the evaluator crate) build on it. It is a product outcome alone:
  it refuses automation lanes that no rendering could honour.
