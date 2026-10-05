# Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-4, D15-6).
Code anchors verified on `main` at `6fb211594`.

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053), and part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`), which classifies `hpf_hz` and `lpf_hz`
as live through prepared targets (the #808 amendment).

## Product outcome

A C ABI transaction that changes a track's input `hpf_hz` or `lpf_hz` (enabling, moving or
disabling a filter, on one lane or both) is a live update. The control thread designs the filter
targets, and render applies them at a block boundary over the fixed 64-update coefficient ramp.
The input section keeps reporting the bounded tail of #1261, never `Infinite`. Today such an edit
rebuilds the plan.

## Context

- **The model.** `ChannelBuiltins::hpf_hz` and `lpf_hz` (`crates/session/src/model.rs:490-493`).
- **The design authority.**
  - `builtins::validate_input_filter_pair` and `prepare_input_filter_pair`
    (`crates/builtins/src/filter_control.rs:36`, `:53`). The ramp is
    `INPUT_FILTER_RAMP_SAMPLES = 64` (`:9`).
  - host-core's stateless `InputFilterPreparer::prepare(sample_rate_hz, seeds, edits, out)`
    (`crates/host-core/src/control_preparation.rs:197-291`). The seeds are four values in lane
    order: left HPF, left LPF, right HPF, right LPF. It validates every edit in order through
    `apply_input_filter_edit` (`:153-196`), designs only the touched sections, and merges a
    section into one `Both` target when both lanes' words are equal.
  - `InputFilterEdit { parameter_id, channel, value0, value1 }` (`:99-109`): `parameter_id` 0 is
    an atomic pair, 3 the HPF, 4 the LPF. Each edit validates the pair it leaves, so two
    one-sided edits can fail on an intermediate pair (HPF at or above LPF) that the final pair
    does not have.
  - `INPUT_FILTER_TARGET_CAPACITY` is 4 sections per track (`:29`).
- **The record.** `TrackInputRecord::PreparedFilter { target }`
  (`crates/builtins-compiler/src/lib.rs:207-211`), applied by
  `BuiltinInputBank::apply_prepared_filter` (`crates/builtins/src/lib.rs:3659`).
- **A disabled section.** When a filter ramp completes onto identity coefficients, the ramp body
  clears that lane's integrators to `+0.0` (`crates/lane/src/kernels/builtins.rs:878-883` dual,
  `:978-981` mono), and `refresh_filter_plan` (`crates/builtins/src/lib.rs:1266`) re-elides the
  section. *Elide a builtin input filter section again after a live disable settles it to
  identity* (#1268) measures this first (its gate 1); the old hazard in this spec, that a disabled
  section keeps its integrators, is the claim #1268 checks.
- **The browser's path.** `COMMAND_INPUT_FILTERS` (`hosts/host-web/src/lib.rs:4986-5028`) keeps a
  per-track shadow of the current pair. On the C ABI the committed model is that shadow
  (#1053 D9).
- **The input lane** and the bounded tail come from #1261.

## Decisions frozen for this slice

- **D1. The classifier.** `classify_live_delta` also masks each track's `hpf_hz` and `lpf_hz`.
  It compares the values each lane's pair normalizes to through `validate_input_filter_pair`
  (a `-0.0` to `0.0` rewrite is no change). For a track whose normalized values change:
  - call `InputFilterPreparer::prepare` with the pre-commit normalized values as the seeds, at
    `next`'s sample rate, with these edits, which mark only the changed sections dirty:
    - one lane's HPF only: `parameter_id` 3; its LPF only: `parameter_id` 4;
    - both of one lane's values: one atomic pair edit, `parameter_id` 0, never two one-sided
      edits;
    - the same change on both lanes: one edit with `ParameterChannel::Both`, so the preparer can
      merge the two targets into one `Both` target;
  - emit one `TrackInputRecord::PreparedFilter` per target it returns, after #1261's trim and
    polarity records, in the order the preparer returns them;
  - a preparer refusal gives `LiveRebuild::Domain`.
- **D2. The cells.** The targets are written to the strip's input cells from *Hold strip
  input-lane values in latest-target cells* (#1346), with #1261's records and after the same fallible
  checks. A section target is a level: a later target for the same lane and section supersedes an
  undrained one, and the ramp always starts from the coefficients render holds.
- **D3. The tail.** No change: #1261's plans already carry #1329's `input_section_live_bound`
  over the whole reachable filter domain (#1329 D5), because any live target can enable any
  filter.
- **D4. Acked-batch question.** As #1261 D5: every check, the design included, runs before the
  first cell write.
- **D5. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists the filters as live.

## Deliverables

1. D1 in the classifier, with its tests.
2. D2 in the C ABI control plane, with its tests.
3. D5.

## Authorized paths

- The classifier: `crates/host-core/src/live_delta.rs` and
  `crates/host-core/tests/live_delta.rs`.
- capi's live tests: `crates/capi/src/runtime/live_tests.rs`.
- `crates/control-plane/src/control.rs` (moved there by #1309), only if the input write needs it.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No change to the preparer, the record, the 64-update ramp or the browser's shadow.
- No submix strips (#1267).

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - an HPF enabled on the left lane only (one `Left` target);
   - an LPF moved on both lanes to the same value (one `Both` target);
   - both values of one lane moved past each other (HPF 200 to 9000, LPF 8000 to 12000), which
     two one-sided edits would refuse: live, with the pair's targets;
   - an `hpf_hz` `0.0` to `-0.0` rewrite is live with no record;
   - an HPF at or above the LPF gives `Domain`;
   - a cutoff past the preparer's bound gives `Domain`.

   The targets equal those `InputFilterPreparer::prepare` returns for the same seeds and edits.
   `cargo test --locked -p host-core --all-targets --features host-core/test-support`
2. **Equal to the browser's lane.** A live HPF edit through the C ABI, and hand-built
   `PreparedFilter` records (from `InputFilterPreparer::prepare` called directly with the test's
   own seeds and edits, never from the classifier) applied at the same block to a host-core plan
   prepared with `HostLiveLanes::ALL` (`LaneReference`), render bit-identically. On
   `long_session` for 1 and 10 tracks at the four launch rates.
3. **A round trip ends where a rebuild would.** Enable an HPF live, render, then disable it live.
   From block E + ceil((64 + `latency_samples`) / quantum) + 1 after the disable, the output is
   bit-identical to a `Reference` compiled from the committed snapshot (filters off) fed the same
   source, on #1261's gate-2 session (`long_session` with no console slots and no inserts) at
   48 kHz. Both runs read a variant of `source_sample`
   (`crates/capi/src/runtime/live_tests.rs:23`, which is never zero) that puts exact `-0.0`
   samples inside the compared window.
4. **The reported tail holds.** At 48 kHz, on one track with no console slots and no inserts,
   enable live the input section's worst-case pair that #1329 D5's derivation names, feed one impulse of peak 1.0 and then zeros (the session's
   source declares enough frames to cover the whole window): every output sample from the plan's
   reported `tail_samples` (plus `latency_samples`) after the impulse has a magnitude below
   `10^(-144/20)`.
5. **Nothing else changes.** `cargo test --locked -p capi`,
   `cargo test --locked -p control-plane --all-targets --features control-plane/test-support`; #1261's gates 5 and 6. 4-lane (NEON)
   is CI-only here.

## Test value

- Gate 1: red if the classifier seeds the preparer from the wrong model (targets that do not match
  the plan's state), splits a pair edit into two one-sided edits, emits a target for a
  sign-of-zero rewrite, or lets an invalid pair through.
- Gate 2: red if the C ABI designs or addresses a target differently from the browser's lane.
- Gate 3: red if a live disable leaves a filter section active or its integrators non-zero (a
  sign-of-zero difference on the `-0.0` samples).
- Gate 4: red if the C ABI reports a tail shorter than the live filter's real decay.
- **Superseded in the same PR:** the case #1261 moved to a route's `gain_db` stays; any classifier
  or capi case that asserts an `hpf_hz` or `lpf_hz` edit rebuilds is changed to the live path.

## Dependencies

- *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261)
- *Elide a builtin input filter section again after a live disable settles it to identity*
  (#1268), whose gate 1 result gate 3 relies on
- *State a bounded tail and an exact-rest bound for every node* (#1329), for gate 4's bound and
  worst-case pair
- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328)
- *Hold strip input-lane values in latest-target cells* (#1346), and *Hold live values in
  latest-target cells on both hosts* (#1312) under it

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
