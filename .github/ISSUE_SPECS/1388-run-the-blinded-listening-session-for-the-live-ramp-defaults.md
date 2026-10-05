# Run the blinded listening session for the live ramp defaults

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The preregistered blinded listening session for the `controlSmoothing` defaults has been run by a
human listener on the defaults the engine ships, and its completed record is in the repository. If
the preregistered decision rules pick a different value for `muteMs`, `faderMs` or `panMs`, the
engine's default table changes to it, and nothing else changes. Listening evidence never replaces
the objective gates (AGENTS.md, "Realtime, quality, and research evidence").

**This issue needs a human listener.** An agent prepares the packet, checks it, and records and
applies the result. It never answers a trial, never writes a synthetic response, and never opens the
private assignment key before reveal.

## Context

- **The packet.** `docs/handoffs/control-smoothing-defaults/listening/` holds `PREREGISTRATION.md`
  (status `preregistered`, 68 trials in blocks M, F and R, exact one-sided binomial tests, positive
  controls, and mechanical decision rules), `listening.py` (prepare, run, validate, reveal,
  self-test) and `README.md` (the procedure). It follows `dsp-research/listening/TEMPLATE.md`.
- **The rules** (`PREREGISTRATION.md`, "Decision rules"): `muteMs` is 5, 10 or 20 ms by the M-block
  primaries and the R-block check; `faderMs` and `panMs` stay 20 ms unless F-30-20 is detected,
  else 35 ms or 20 ms with a host update-rate requirement. A missed positive control makes every
  non-detection inconclusive.
- **The stimuli** come from the engine's own ramps: `control_smoothing_measure stimuli`, built from
  `docs/handoffs/control-smoothing-defaults/measure/`, which links `crates/builtins` by path
  (`measure/Cargo.toml`). The packet records the engine commit (`prepare --commit`).
- **The shipped defaults.** *Session `controlSmoothing`: configurable ramp lengths for live mute,
  fader and pan changes* (#1054) ships `CONTROL_SMOOTHING_DEFAULT` in `crates/session` from #1055's
  `FINDINGS.md` section 9. The SDK sends "no length" and the engine resolves it
  (*Resolve an absent live ramp to the session default on the browser and in the SDK*, #1364), so
  the table has one home.
- **Decision 15, D15-1** (recorded resolution): a listening result changes only default values.

## Decisions frozen for this slice

- **D1. Candidate.** Prepare the packet at the `main` commit that contains #1054, so the stimuli
  are rendered by the shipped ramp kernels. Before preparation, confirm that the shipped defaults
  appear among the packet's conditions (mute 10 ms, fader and pan 20 ms in sections 1-8). If #1055
  section 9 added contrasts by a dated preregistration amendment, they run in the same session.
- **D2. Procedure.** Exactly `listening/README.md`: `prepare`, the three `run` blocks (separate
  sittings allowed), `validate`, `reveal`. A second person as facilitator is preferred. Playback
  chain, level and listener details are recorded at reveal, as the record's fields require.
- **D3. Record.** A completed copy of the preregistration, `listening/RECORD-<UTC date>.md`, with
  status `complete`, the counts, p-values, the decision `reveal` printed, the playback chain, and
  the sign-offs. `PREREGISTRATION.md` itself is not edited. Commit `public/preparation.json`,
  `responses.jsonl` and `reveal.json` next to the record. Stimulus WAVs are not committed; they
  re-render from the recorded commit.
- **D4. Applying the result.** The only product change is the value of a key in
  `CONTROL_SMOOTHING_DEFAULT`, with the expected sample counts that tests state for the default
  table, and the default table's text in the docs. A session document that says
  `control_smoothing: { "kind": "default" }` (#1054 D1) follows the new value; one that says
  `explicit` keeps its own. Where the fader rule offers 35 ms or 20 ms with a
  host update-rate requirement, take 35 ms: the engine does not add a host contract to keep a
  default. If the decision equals the shipped table, no product file changes.
- **D5. Inconclusive.** If a positive control is missed, the session is inconclusive. The record is
  committed as such, the defaults stay, and a fresh packet (new seed) may be run once under the same
  preregistration; the issue closes on the second record whatever it says.

## Deliverables

1. The completed record and its raw files (D3).
2. If D4 applies: the value change and the updated expected counts and docs.
3. A one-line pointer in `FINDINGS.md` section 1 ("Provisional on listening") to the record.

## Authorized paths

- `docs/handoffs/control-smoothing-defaults/listening/` (new record and raw files only),
  `docs/handoffs/control-smoothing-defaults/FINDINGS.md` (the pointer)
- If D4 applies: the default table in `crates/session/src/` (the `CONTROL_SMOOTHING_DEFAULT` value
  only), `docs/SESSION_SCHEMA_V1.md` (the table), and the expected default sample counts in
  `crates/session/tests/`, `crates/host-core/tests/live_delta.rs`,
  `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs` and
  `hosts/host-web/tests/control_smoothing_parity.rs`

## Non-goals

- No change to the ramp law, the rounding, the bounds, the key table (#1054 D3) or any kernel.
- No new trial, contrast or decision rule after preparation.
- No listening for effect DSP: each effect issue carries its own.

## Objective gates

1. `TMPDIR=<scratch> python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test`
   passes.
2. `python3 docs/handoffs/control-smoothing-defaults/listening/listening.py validate <packet>`
   passes, and `reveal` run again on the committed responses prints the decision the record states.
3. The record has every field of `dsp-research/listening/TEMPLATE.md`, 68 valid response rows (plus
   any amendment's), and no synthetic answer.
4. If D4 applies: `git diff --stat` against the base touches only D4's paths, and these pass:
   `cargo test --locked -p session`,
   `cargo test --locked -p host-core --features test-support --test live_delta`,
   `cargo test --locked -p capi`, `cargo test --locked -p host-web --features test-support`, the
   workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`),
   `bash scripts/check-workspace-policy.sh`, `bash scripts/check-env-vocabulary.sh`.

## Test value

- No new test. Gate 2 turns red if the recorded decision does not follow from the committed
  responses under the preregistered rules; gate 4's suites turn red if the value change leaves a
  stale expected count, or if it touched anything beyond the table.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054)
- *Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)* (#1055)
- *Resolve an absent live ramp to the session default on the browser and in the SDK* (#1364)

#1054 ships the defaults this session judges. #1055 gives section 9 and any preregistration
amendment. #1364 creates `hosts/host-web/tests/control_smoothing_parity.rs` and the host-web
resolution tests whose expected default counts D4 may update.
