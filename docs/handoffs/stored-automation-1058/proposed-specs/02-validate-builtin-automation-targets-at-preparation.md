# Validate builtin automation targets against their rows at preparation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A3, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A session whose builtin-target automation does not fit its row is refused, typed, on every path
that prepares a session: browser boot, C ABI boot and every C ABI rebuild, and `session-validator`
stage `prepare-builtins`. Refused today and accepted silently: a fader ride written in `linear`, a
mute that ramps, a fader ride to +30 dB, an HPF that glides out of "off", and an HPF curve that
crosses the LPF on the same lane. The order check is one function of a session model, so the slice
that classifies input-filter edits on an automated strip (slice 16b) can refuse a later static edit
that would cross a curve with the same code. Nothing renders the table yet.

## Context

- **The session crate cannot see descriptors.** It depends on `engine` and `json-syntax` only
  (`crates/session/Cargo.toml:15-17`; `scripts/check-session-policy.sh`), so its target check
  (`crates/session/src/validate.rs:854-888`) reads only the parameter ID and the channel form.
  Draft 01 adds the lane, strip-kind and discontinuity rules there.
- **The builtin descriptors.** `BUILTIN_PARAMETER_DESCRIPTORS` (`crates/builtins/src/lib.rs:453-673`);
  `builtin_parameter_unit` (`:254-269`) gives `db` for trim and fader, `hz` for the filters and
  `linear` for polarity, mute, matrix and pan. `BuiltinParameterDomain` (`:288-296`) and its
  `contains(value, sample_rate)` (`:298-316`): `BooleanExact` accepts the bits of `0.0` or `1.0`;
  `FiniteInclusive` is `[-144, 24]` dB for trim and fader and `[-1, 1]` for matrix and pan;
  `DisabledOrRateKeyedHertz` accepts `0` ("off") or `[10, max(rate)]` Hz through
  `validate_builtin_filter_cutoff` (`:336-352`) and `builtin_filter_cutoff_maximum_hz` (`:322-330`).
- **The static filter order rule.** `validate_input_filter_pair`
  (`crates/builtins/src/filter_control.rs:36-50`) and the chain's preflight
  (`crates/builtins/src/lib.rs:3290-3294`) refuse `hpf_hz > 0 && lpf_hz > 0 && hpf_hz >= lpf_hz`
  as `BuiltinParameterError::FilterOrder`, reported as `builtin.filter.order`
  (`crates/builtins-compiler/src/lib.rs:5012`) at the strip's `lpf_hz` (`filter_order_path`,
  `:5062-5071`). Nothing compares curves.
- **Builtin diagnostic vocabulary.** `BuiltinDiagnostic { code: &'static str, path: String }`
  (`crates/builtins-compiler/src/lib.rs:273-276`), built by `diag` (`:4992-4997`) with literal
  dotted codes (`parameter_diagnostic`, `:5004-5030`). No document freezes the builtin codes as a
  list; `docs/SESSION_SCHEMA_V1.md:85` names `builtin.gain.domain` where it states its rule.
- **The one builtin preparation.** Every public entry (`prepare_session_builtins`, `:3298`, and its
  four siblings, `:3316-3391`) runs `prepare_session_builtins_with_live_controls_and_policy`
  (`:3405`). Its domain preflight loops over the strips (`:3484-3494`) and returns every diagnostic
  before it allocates (`:3515-3517`). host-core calls the entries for the browser and the C ABI
  (`crates/host-core/src/prepare.rs:1503-1530`); `session-validator` runs `prepare_session_builtins`
  as stage `prepare-builtins` (`tools/session-validator/src/lib.rs:25-27`, `:87-93`).
- **The C ABI automation-only edit skips preparation.** `classify_live_delta` masks the table
  (`crates/host-core/src/live_delta.rs:234-236`), so `commit_live` commits such an edit with no
  preparation (`crates/capi/src/runtime/control.rs:1065-1080`). *Refuse automation on effect
  parameters that are not block-rate* (#1335 D4) has landed on `main` since `6ee64f484` (commit
  `0c19119d0`) and added the route that closes this for its own rule: the classifier runs
  `effect_automation_diagnostics` on `next` whenever the automation differs, and a non-empty result
  returns `LiveRebuild::AutomationTarget`, so the commit takes the rebuild path whose preparation
  refuses. The anchors in this draft stay those of `6ee64f484`. Note A10 removes the mask one row at
  a time, from slice 10 to slice 20, so until slice 20 some automation edits still skip
  preparation.
- **Checked-in builtin automation** (`fixtures/session/v1/builtins-automation.json`, after draft 01's
  migration): fader `both` linear `db` 0 to -3, matrix `both` linear `linear` 1 to 0.5 on a matrix
  strip, polarity `right` step 0 to 1, trim `left` linear `db` 0 to -6. Each fits D1. No checked-in
  document automates a filter.

## Decisions frozen for this slice

- **D1. Unit, domain and shape per row.**
  - One function in `builtins-compiler`, `builtin_automation_diagnostics(model: &SessionModel) ->
    Vec<BuiltinDiagnostic>`, reads each `rack = "builtins"` entry's descriptor by `parameter_id`.
    Each rule is checked per segment `k`, at `$.automation[id=<id>].segments[<k>].<field>` (the
    #1335 D2 path form):
    - **Unit.** The segment's `unit` equals `builtin_parameter_unit(row)` (session and
      effect-contract units map one to one, as `same_unit` does,
      `crates/effect-compiler/src/prepare.rs:1679-1689`). Else `builtin.automation.unit` at
      `.unit`, and no other rule is checked for that segment.
    - **Domain.** `row.domain.contains(value, model.sample_rate_hz)` for `start_value` and for
      `end_value`. Else `builtin.automation.domain` at that field.
    - **Shape.** A `BooleanExact` row (polarity, mute) takes only `step`. `exponential` is allowed
      only on a row whose unit is not `db` and whose enabled values are strictly positive: of the
      launch rows, only `hpf_hz` and `lpf_hz`. On a `DisabledOrRateKeyedHertz` row a `linear` or
      `exponential` segment has neither value equal to the disabled value, so "off" is reached only
      by a `step`. Else `builtin.automation.shape` at `.shape`.
  - Targets the session already refuses (an unknown ID, `delay_samples`, a pan or matrix target on
    the wrong strip kind) never reach preparation.
- **D2. Input filter order on every lane, at all times.**
  - A second function, `input_filter_order_diagnostics(model: &SessionModel) ->
    Vec<BuiltinDiagnostic>`, runs per strip and lane (`left`, `right`) where at least one of HPF
    (3) and LPF (4) is automated. A `both` entry drives both lanes. Each side's curve is its entry
    under the hold rule (draft 01 D4), or, when that side is not automated, the constant curve of
    its static value. A lane with neither side automated is the static rule's.
  - **Split.** The time axis `[0, ∞)` is cut at every `start_sample` and `end_sample` of either
    curve. On each sub-interval `[a, b]` each side is one monotone piece: a hold (constant), a
    `step` piece (constant), a `linear` piece or an `exponential` piece (the last sub-interval is
    a hold on both sides). A side that is a constant `0` ("off") on the sub-interval leaves it
    unchecked; D1 keeps every `linear` and `exponential` piece away from `0`.
  - **Order.** On each sub-interval where both sides are enabled, with values evaluated at `a` and
    at `b` by draft 07's `automation::value_at(&segments, t)` over a `Segment` table built from
    the entry's segments (D3 there, A1.4's law; a constant is its `f32` value):
    - both constant: `H < L` in `f32`, the static rule;
    - both `linear`, or both `exponential`: `L > H·(1 + 2^-22)` at `a` and at `b`;
    - otherwise: `min(L(a), L(b)) > max(H(a), H(b))·(1 + 2^-22)`.
  - **Why no rendered pair crosses.** Each piece is monotone, so its extremes lie at `a` and `b`.
    Two `linear` pieces have a linear difference `L - H·(1 + 2^-22)`, and two `exponential` pieces
    a monotone ratio `L/H`, so the order at both ends holds inside. For mixed pieces the extremes
    rule is sufficient, not necessary: it refuses some pairs that never cross, by design. The
    relative margin `2^-22` is four times the `f32` rounding bound `2^-24`: after render rounds
    each value once to `f32`, `f32(L) > f32(H)` still holds at every sample, so no rendered pair
    ties or crosses. Constants need no margin because render does not round them.
  - **Refusal.** `builtin.filter.order`, the static rule's code, once per lane at the first failing
    sub-interval, at `$.automation[id=<id>].segments[<k>]`: `<id>` the LPF entry when LPF is
    automated, else the HPF entry; `<k>` the segment that defines that entry's value on the
    sub-interval (the first segment for the leading hold, the held segment in a gap or after the
    last segment).
  - **For a later live edit.** The function reads only the model, so a classifier can run it on a
    candidate model to refuse a static HPF or LPF edit that would cross an automated curve. Wiring
    it into `classify_live_delta` is slice 16b's.
- **D3. One call site.** `prepare_session_builtins_with_live_controls_and_policy` appends both
  functions' results before its check at `:3515`, after the per-strip preflight, so no host and no
  validator stage changes its own code. Both functions allocate only off render, at preparation.
- **D4. Docs.** `docs/SESSION_SCHEMA_V1.md` (the automation section draft 01 writes) names the three
  `builtin.automation.*` codes with the rows they admit, and the curve form of the filter order
  rule. The `author-session` skill's automation refusal table gains the four rows at stage
  `prepare-builtins`.
- **D5. The live path routes a refusable edit to preparation.** #1335's route is an existing
  route on `main`: `classify_live_delta` runs `effect_automation_diagnostics` on `next` whenever
  `current.automation != next.automation`. This slice adds D1's and D2's functions to that same
  check. A non-empty result returns #1335's `LiveRebuild::AutomationTarget`, so the
  commit takes the rebuild path, and that path's preparation refuses with the D1 or D2 diagnostic.
  Nothing is pushed or committed first. The classifier only routes; preparation words the refusal.
  Once slice 20 removes the last mask row, every automation edit is a rebuild and slice 20 deletes
  this route with #1335's.
- **D6. The acked-batch question** does not arise: both refusals happen before preparation
  allocates or anything is committed. No queue changes.

## Deliverables

1. D1-D3 in `crates/builtins-compiler/src/lib.rs`: two functions, exported, and their one call.
   D5 in `crates/host-core/src/live_delta.rs`: the two calls beside #1335's existing call.
2. D4 in `docs/SESSION_SCHEMA_V1.md` and `.claude/skills/author-session/SKILL.md`.
3. The tests below.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the two functions, their exports and their call only),
  `crates/builtins-compiler/Cargo.toml` (the `automation` dependency only), `Cargo.lock`,
  `crates/builtins-compiler/tests/builtin_automation_targets.rs`
- `scripts/check-builtins-policy.sh` (only `expected_compiler`, `:17`, which pins builtins-compiler's
  exact dependency list, sorted by `gate_toml_dependencies`, `scripts/lib/gate.sh:162-196`:
  `automation` joins it first)
- `crates/host-core/src/live_delta.rs` (D5 only; stream B owns it, root sequences the merge),
  `crates/host-core/tests/live_delta.rs`, `crates/capi/src/runtime/live_tests.rs` (tests only)
- `tools/session-validator/tests/validate.rs`
- `docs/SESSION_SCHEMA_V1.md`, `.claude/skills/author-session/SKILL.md` (the automation section and
  refusal table only)

## Non-goals

- Effect-target rules: draft 03a and #1335. Session-level rules: draft 01.
- The SDK builder's mirror (draft 03b). The classifier route for static filter edits (slice 16b).
- Rendering any automation.

## Hazards

- **The C ABI automation-only path.** Without D5, a C ABI automation-only edit could commit a
  builtin segment its own rebuild refuses. Gate 7 holds it.
- **Hand-built test automation.** `crates/capi/src/runtime/live_tests.rs:1446-1467` and
  `crates/host-core/tests/live_delta.rs:509-527` build a fader ride in `db` within the domain; they
  stay valid. Any other hand-built builtin segment that now fails is a test defect to fix in its
  value, never a reason to relax D1.
- **`-0.0`.** `BooleanExact` refuses the bits of `-0.0` (`contains`, `crates/builtins/src/lib.rs:303-305`);
  a mute segment authored with `-0.0` is refused, as a static mute of `-0.0` already is.
- **Evaluation code.** D2 needs A1.4's value at two samples. It calls the evaluator crate of
  draft 07 (`automation::value_at`), which lands first, so one law exists from the start. The check
  needs only ordering, and its `2^-22` margin dwarfs any `f64` evaluation error.

## Objective gates

1. **Rows** (`crates/builtins-compiler/tests/builtin_automation_targets.rs`, new tests). From the
   migrated `fixtures/session/v1/builtins-automation.json`, `prepare_session_builtins` returns
   exactly one diagnostic, with the stated code and path, for each of: a fader segment in `linear`
   (`builtin.automation.unit`); a fader value of `24.001` (`builtin.automation.domain`); a matrix
   value of `1.5`; a polarity value of `0.5`; a mute `linear` segment
   (`builtin.automation.shape`); an `exponential` fader segment; an HPF `linear` segment from `0`
   to `100`; an HPF value above the 44.1 kHz maximum in a 44.1 kHz session. It accepts the fixture
   as written, an HPF `exponential` segment from `20` to `200`, and an HPF `step` from `0` to `80`.
2. **Filter order** (same file, new tests), static LPF 1,000 Hz unless stated:
   - refused: an HPF `linear` ride from 200 to 1,000 Hz (it reaches the LPF at its end); an HPF
     `step` from 100 to 1,200 Hz (refused on the hold after it); an HPF `exponential` from 100 to
     900 Hz against an LPF `linear` from 500 to 2,000 Hz over the same samples (both ends ordered;
     the mixed rule refuses, by design);
   - accepted: an HPF `linear` from 100 to 900 Hz; an HPF `linear` from 100 to 900 Hz against an
     LPF `linear` from 500 to 2,000 Hz over the same samples (both linear, ordered at both ends);
     an HPF `step` to `0` while an LPF ride passes below the HPF's former value;
   - the function, run on a model whose automation is unchanged and whose static LPF moves from
     1,000 to 500 Hz under an HPF ride to 900 Hz, returns `builtin.filter.order`.
3. **Validator stage** (`tools/session-validator/tests/validate.rs`, new rows in `MUTATIONS`). A
   fader segment in `linear` fails at stage index 3 (`prepare-builtins`) with
   `builtin.automation.unit`, and stages 0-2 pass.
4. **Every checked-in document still prepares.**
   `fixtures_distinguish_schema_examples_from_launch_effects`
   (`tools/session-validator/tests/validate.rs:81`) and the skill test pass unchanged.
5. **No rendered bit moves.** No render code changes. `cargo build --locked --release -p audit -p
   bench -p capi -p session-validator`, then `./target/release/audit capi`, shows the same
   `pcm_digest` at base and head (PR evidence); the browser legs of the `browser` job in
   `.github/workflows/qualification.yml` pass with unchanged digests.
6. **Commands:**
   - `cargo test --locked -p builtins-compiler --features test-support`,
     `cargo test --locked -p session-validator`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-builtins-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

7. **The live path** (`crates/host-core/tests/live_delta.rs` and
   `crates/capi/src/runtime/live_tests.rs`, new). A delta that only adds a fader segment in
   `linear` units gives `Err(LiveRebuild::AutomationTarget)`; a delta that adds a valid fader
   segment stays as A10's rows say. On a playing C ABI engine, a transaction that only adds the
   invalid segment is refused with the typed compile rejection; the revision does not advance, no
   plan is replaced, and the next blocks are bit-identical to an engine that never received it.

## Test value

- Gate 1 turns red for each row rule left out, for a domain check that ignores the session rate
  (the 44.1 kHz case), for a shape rule that refuses the filters' legal `exponential`, or for a
  filter rule that refuses "off" by `step`. No test reads a builtin segment's unit or values today.
- Gate 2 turns red if the check samples only segment boundaries of one curve (the hold case), uses
  the end-point rule on mixed pieces (the `exponential` against `linear` case), checks a lane whose
  HPF is off, or reads the static value of an automated side; its last case turns red if the
  function reads anything but the model it is given.
- Gate 3 turns red if D1 runs anywhere `session-validator` does not run.
- Gate 7 turns red if a C ABI automation-only edit can commit a builtin segment that preparation
  refuses: the acked-but-unrenderable gap #1335 closes for effect rates.

## Dependencies

- Draft 01 *Validate stored automation lanes in the session crate and state the hold rule* (the
  hold rule, the migrated fixture, the strip-kind rule).
- Draft 07 *Compile stored automation into per-cell events in node time*: `value_at`, the one
  evaluation law D2 calls.
- *Refuse automation on effect parameters that are not block-rate* (#1335): D5 uses its
  `LiveRebuild::AutomationTarget` route, which is on `main` at `0c19119d0`.
- Batch: R1.
