# Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259), which classifies `hpf_hz` and
`lpf_hz` as live through prepared targets (the #808 amendment). Anchors verified on `main` at
`54b0a1bf8`; re-verify them after #1261 lands.

## Product outcome

A C ABI transaction that changes a track's input `hpf_hz` or `lpf_hz`, enabling, moving or
disabling a filter on one lane or both, is a live update. The control thread designs the filter
targets off the render thread, and the plan applies them at a block boundary over the fixed
64-update coefficient ramp. Today such an edit rebuilds the plan.

## Context (verified at `54b0a1bf8`)

- **The model.** `ChannelBuiltins::hpf_hz` and `lpf_hz` (`crates/session/src/model.rs:489-493`).
- **The design authority.**
  - `builtins::validate_input_filter_pair` and `prepare_input_filter_pair`
    (`crates/builtins/src/filter_control.rs:36`, `:53`).
  - host-core's stateless `InputFilterPreparer::prepare(sample_rate_hz, seeds, edits, out)`
    (`crates/host-core/src/control_preparation.rs:199-290`). The seeds are four values in lane
    order: left HPF, left LPF, right HPF, right LPF. It validates every edit, designs only the
    touched sections, and merges a pair into one `Both` target when both lanes' words are equal.
- **The record.** `TrackInputRecord::PreparedFilter { target }`
  (`crates/builtins-compiler/src/lib.rs:175-211`), applied by `apply_prepared_filter`
  (`crates/builtins/src/lib.rs:3516`).
- **The browser's path.** `COMMAND_INPUT_FILTERS` (`hosts/host-web/src/lib.rs:4982-5020`) keeps a
  per-track shadow of the current pair. On the C ABI, the pre-commit model is that shadow.
- **The input lane.** #1261 attaches it on the C ABI.

## Decisions

- **D1. The classifier.** `classify_live_delta` also masks each track's `hpf_hz` and `lpf_hz`.
  For a track whose four values change:
  - call `InputFilterPreparer::prepare` with the pre-commit values as the seeds, at `next`'s sample
    rate, and with these edits, which mark only the changed sections dirty:
    - one lane's HPF only: `parameter_id` 3; its LPF only: `parameter_id` 4;
    - both of one lane's values: one atomic pair edit, `parameter_id` 0. Never two one-sided edits:
      the intermediate pair can fail validation (HPF above LPF,
      `crates/host-core/src/control_preparation.rs:180-185`);
    - the same change on both lanes: one edit with `ParameterChannel::Both`, so the preparer can
      merge the two targets into one `Both` target;
  - emit one `TrackInputRecord::PreparedFilter` per target it returns, after the trim and polarity
    records;
  - a refusal from the preparer gives `Domain`.
- **D2. capi.** These records ride the input queue that #1261 checks and pushes. The order of
  #1053 D6 holds.
- **D3. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists the filters as live.

## Authorized paths

- `crates/host-core/src/live_delta.rs` and `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/live_tests.rs`, and `control.rs` only if the input push needs it.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No change to the preparer, the record or the 64-update ramp.
- No change to the browser's shadow.

## Hazards

- **The preparer's capacity.** `INPUT_FILTER_TARGET_CAPACITY` is 4 sections per track. A track's
  delta can never need more.
- **Filters keep state.** A live filter change is not bit-identical to a fresh plan of the edited
  session, because the filter's history differs. Gate 2 compares against hand-built records.
- **A disabled section keeps its integrators.** With identity coefficients, `svf_step` leaves
  `ic1` and `ic2` at their last values (`crates/lane/src/kernels.rs:636-652`), and a section is
  elided only when they are exactly `+0.0` (`crates/lane/src/kernels/builtins.rs:1146-1163`). So
  after a live disable the section keeps running until the next rebuild, and on an exact `-0.0`
  input sample it can emit `-0.0` where a never-enabled plan emits `+0.0`. *Elide a builtin input
  filter section again after a live disable settles it to identity* (#1268) owns the fix; gate 3
  avoids the sign case.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - an HPF enabled on the left lane only;
   - an LPF moved on both lanes to the same value (one `Both` target);
   - an HPF above the LPF gives `Domain`;
   - a cutoff past the preparer's bound gives `Domain`.

   The targets equal those `InputFilterPreparer::prepare` returns for the same seeds and edits.

   *Test value: it turns red if the classifier seeds the preparer from the wrong model, so the
   targets would not match the plan's state, or lets an invalid pair through.*
2. **Equal to the browser's lane.** A live HPF edit through the C ABI, and hand-built
   `PreparedFilter` records (from `InputFilterPreparer::prepare` called directly with the test's
   own seeds and edits, not from the classifier) pushed into a host-core plan prepared with
   `HostLiveLanes::ALL` at the same block, render bit-identically. Run it on
   `generated_parity_session` for 1 and 10 tracks at the four launch rates.
   *Test value: it turns red if the C ABI designs or addresses a target differently from the
   browser's lane.*
3. **A round trip ends where a rebuild would.** Enable an HPF live, then disable it live. After the
   ramp plus `latency_samples`, the output is bit-identical to a plan compiled from the committed
   snapshot (filters off), fed the same source from sample 0, on a session with an empty console
   and no inserts. The source has no exact `±0.0` sample in the compared window (see the hazard).
   *Test value: it turns red if a disable leaves a filter section active.*
4. **Nothing else changes.** As #1257's gates 6 and 7. 4-lane (NEON) is CI-only here.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.

## Dependencies

- *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Status

Blocked: depends on #1261 (owner Q4); not started in this batch.
