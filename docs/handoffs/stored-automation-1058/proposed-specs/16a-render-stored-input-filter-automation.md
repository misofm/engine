# Render stored input HPF and LPF automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A1.6, A3 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Root confirms README finding F1 before filing. Batch
R2.

## Product outcome

A producer's stored input HPF and LPF sweeps (rows 3 and 4) play on every host. At each event render
designs the filter section with the same code and the same check the control plane uses, and starts
the existing 64-sample coefficient ramp, so the coefficients reach each designed target exactly and
the lane's HPF stays below its LPF at every sample. A filter turns off only by a step, and the
section then settles to identity and elides again. A strip with filter automation reports the live
input tail bound. A filter automation edit is a carried rebuild; a static edit of an automated
filter is `model_only`. On the C ABI, a static edit of the pair's other filter is a carried rebuild.
The browser refuses a filter command on an automated pair until #1382. Live edits of the pair's
other filter are draft 16b's.

## Context

- **The design authority.** `SvfSection::design(rate, cutoff, high_pass)` designs one Butterworth
  section in `f64` through `math::tan`, one cast per word; `0` is the identity
  (`crates/builtins/src/lib.rs:716-760`). `prepare_input_filter_pair` validates the pair, designs
  both sections and checks each target (`crates/builtins/src/filter_control.rs:52-73`);
  `validate_prepared_input_filter_target` checks the pair order, the words and the state matrix's
  spectral norm, with two `math::sqrt` (`:75-137`). The pair rule is `validate_input_filter_pair`
  (`:35-50`). `INPUT_FILTER_RAMP_SAMPLES = 64` (`:9`).
- **The apply path.** `apply_prepared_filter` sets a 64-sample linear ramp per changed word, then
  refreshes the elision plan and the channel-symmetry words (`crates/builtins/src/lib.rs:1351-1426`;
  `refresh_filter_plan`, `:1263-1295`). A ramp that completes onto identity clears that lane's
  integrators to `+0.0` (`crates/lane/src/kernels/builtins.rs:870-883`). Sample `A` uses the current
  words and sample `A + 64` the exact target (`docs/rulings/builtins-input-liveness-d2.md:17-18`).
- **Off-render design today.** #808's amendment designs live targets off render
  (`docs/rulings/builtins-input-liveness-d2.md:6-14`) and leaves stored rendering to #1058
  (`:163-180`). `docs/BUILTINS_AND_METERING_V1.md:31-34` still calls the filters prepared-only.
  README A1.6 says why stored automation designs in render (finding F1).
- **The cell.** *Hold strip input-lane values in latest-target cells* (#1346 D1) holds one filter
  cell per `(section, channel)`: the pair and six coefficient words. README amendment rows #1262 and
  #1346: for an automated pair, that cell is the group cell and holds the semantic pair.
- **Tails.** *State a bounded tail and an exact-rest bound for every node* (#1329) D4 gives a fixed
  design its own tail, and D5 gives a strip with a live input lane `input_section_live_bound(rate)`,
  the bound over any history of targets and 64-sample ramps.
- **The retarget law.** *Retarget a live input filter only through its designs and their mixtures*
  (#1407) sets the law every filter retarget follows (decision-15 record, `:589-593`).
- **Browser kind.** `COMMAND_INPUT_FILTERS = 12` (`hosts/host-web/src/lib.rs:893`); its admission
  keeps a per-strip shadow of the current pair (`input_filter_shadows`, `:1562`).
- **The lane mask and reason** are draft 10b D1-D2 (`COMMAND_REASON_AUTOMATED`).
- **Draft 15 D1** splits the input stage in a block; draft 15 D4 gives the input rows' collapse rule.

## Decisions frozen for this slice

- **D1. One section design.** `prepare_input_filter_section(rate, hpf_hz, lpf_hz, section) ->
  Result<PreparedInputFilterTarget, BuiltinParameterError>` is factored out of
  `prepare_input_filter_pair`: `validate_input_filter_pair`, `SvfSection::design` of one section,
  `validate_prepared_input_filter_target`. `prepare_input_filter_pair` calls it twice. Render and the
  control plane call the same function.
- **D2. The pair is a target group, per strip lane (left, right).** At each event of the HPF cell,
  render designs section 0 from the HPF curve value and the LPF value; at each LPF event, section 1
  from the LPF curve value and the HPF value. A value comes from its curve when automated, else from
  the group cell, which preparation seeds with the session's static value. One
  `apply_prepared_filter` per designed section. Grid period and ramp 64, completion `After`: an event
  at `τ` targets the curve value at `τ + 64`, the sample the ramp reaches its target. A jump ramps
  over the same fixed 64 samples. A seek where it reaches the input node writes the designed words
  exactly (no ramp) and refreshes the plan; integrators are kept.
- **D3. Off.** "Off" (`0`) is reached only by a `step` (draft 02 D1). Its event designs the identity
  section, and the existing disabled completion clears the integrators; the plan then elides it.
- **D4. A refused design** (unreachable after draft 02's domain and order rules) keeps the current
  target and adds one to the input bank's `automation_designs_refused` count, which a readback
  reports. Render never aborts a block for it.
- **D5. Bounded work.** Per section group and block, at most
  `(⌈q/64⌉ + 1) + Σ_cells(⌈q/64⌉ + 1) + 1 seek + 1 cell change` designs: the grid events of the
  group's cells share one 64-sample grid (`⌈q/64⌉ + 1`), each automated cell adds its jumps
  (`⌈q/64⌉ + 1` per cell and block, draft 01's spacing rule), and one seek and one group-cell change
  (draft 16b) add one each. Section 0 designs only at HPF events and section 1 only at LPF events,
  so each section group has one cell, and a strip lane designs at most `2·(2⌈q/64⌉ + 4)` times per
  block. Each design is one `math::tan`, the design arithmetic and two `math::sqrt`, scalar `f64`,
  with no allocation (README A1.7).
- **D6. Tail.** A strip with filter automation reports `input_section_live_bound(rate)` (#1329 D5),
  as a strip with a live input lane does: its targets change on render.
- **D7. Mono collapse.** Rows 3 and 4 take draft 15 D4's rule: a `left` or `right` entry declines the
  collapse unless both lanes' entries are bit-identical; a `both` entry keeps it, and its designs
  apply to both channels.
- **D8. Edits.**
  - The classifier's row list (draft 10a D1) gains rows 3 and 4: their automation edits are carried
    rebuilds. A static edit of an automated filter gives no record (draft 10a D3).
  - A static edit of the other filter of an automated pair returns `LiveRebuild::Automation` on the
    C ABI: it is a carried rebuild. Decision 14 rule 2 makes a value live only when the plan already
    has a slot for it. The slot for this value is the group-cell write that draft 16b adds; in this
    slice the group cell holds only preparation's seed, and a designed target from the control plane
    would carry a stale pair. Draft 16b makes the edit a live group-cell write.
  - The browser refuses `COMMAND_INPUT_FILTERS` addressing a lane whose pair has an automated
    filter, with `COMMAND_REASON_AUTOMATED`, admitting nothing from the batch: the command sets the
    whole pair, the automated filter included. The lane mask (draft 10b D1) gains
    `automated_filter`. #1382 replaces this with the shared commit's group rule.
- **D9. Docs.** `docs/BUILTINS_AND_METERING_V1.md:31-34` states the filters' liveness and the
  stored-automation exception: render designs stored-automation targets with D1's function; live
  edits stay designed off render (#808).
- **D10. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; an edit of an automated pair either gives no record or rebuilds; the browser refusal
  precedes any admission.

## Deliverables

1. D1 in `crates/builtins/src/filter_control.rs`.
2. D2-D7 in the input stage and the input bank processor, and preparation's seed, tail and witness.
3. D8 in the classifier; D9.
4. D8's browser refusal and the lane mask in host-web.

## Authorized paths

- `crates/builtins/src/{lib.rs,filter_control.rs}`, `crates/builtins/tests/filter_liveness.rs`
- `crates/builtins-compiler/src/lib.rs` (the input bank processor, the tail and witness terms, and the
  lane mask)
- `crates/host-core/src/{prepare.rs,live_delta.rs}`, `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs`
- `hosts/host-web/src/lib.rs` (admission only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/filter_automation_realtime.rs` (new)
- `docs/BUILTINS_AND_METERING_V1.md`

## Non-goals

- The live edit of the pair's other filter (draft 16b). EQ automation (draft
  20). Variable filter slopes (#191).
- A designs-per-second cap: D5's bound is per block and independent of song length.

## Hazards

- **F1 is not confirmed yet.** D1-D5 put a filter design on the render thread. If root refuses F1,
  this slice stops; the alternatives fail A4 or A5 (README A1.6).
- **The realtime policy region.** `SvfSection::design` counts calls through a `#[cfg(test)]`
  thread-local (`crates/builtins/src/lib.rs:724`, `:829`). Production builds have no such access;
  `check-realtime-policy.sh` and `audit capi` must accept the call path.
- **A redundant design moves bits.** `apply_prepared_filter` ramps only words that change
  (`:1381-1392`), so an equal design is a no-op; D2 still skips the call when the curve value's bits
  did not change.

## Objective gates

1. **Same design** (`crates/builtins/tests/filter_liveness.rs`, new). An exponential HPF sweep from
   20 to 2,000 Hz under a static LPF: at every event the stage's target words equal
   `prepare_input_filter_section` called on the test thread with the same values, bit for bit, at
   44.1, 48, 88.2 and 96 kHz.
2. **Flat equals static** (C ABI and browser, new). A flat HPF entry at 80 Hz renders the bits of a
   plan prepared with HPF 80 Hz.
3. **Off by step** (same builtins file, new). A `step` from 80 Hz to 0: 64 samples after the event the
   section's words are the identity, both integrators are `+0.0`, and the next block takes the elided
   path.
4. **Bounded designs** (builtins unit test, new). Over a block of 128 and one of 100, with a moving
   HPF and a moving LPF, the design count (`FILTER_DESIGN_CALLS`) is at most D5's bound; a refused
   design (a value fed past preparation) keeps the target and adds one to
   `automation_designs_refused`.
5. **Tail.** A strip with HPF automation and no live lane reports `input_section_live_bound(rate)`;
   #1329's gates pass unchanged.
6. **Partition identity** (same builtins file, new). Draft 15 D1's identity with a filter event at
   `o` in `{1, 63, 64, 127}` (`q = 128`) and in a block of 100, at `Scalar`, `Simd4` and `Simd8`.
7. **Edits** (`crates/host-core/tests/live_delta.rs`, new). An HPF entry change gives
   `Err(LiveRebuild::Automation)`; a static change of the automated HPF gives no record; a static
   change of the LPF of that pair gives `Automation`.
8. **Browser refusal** (`hosts/host-web/src/tests.rs`, new). `COMMAND_INPUT_FILTERS` on the
   automated lane returns `RESULT_UNSUPPORTED` with `COMMAND_REASON_AUTOMATED` and admits nothing
   from its batch; the same command on a lane with no filter automation is admitted.
9. **Realtime.** `hosts/host-web/tests/filter_automation_realtime.rs` (new integration binary; links
   `bench_support::alloc`, calls `assert_installed()` first): `allocations == 0 && frees == 0`
   around every render call after warm-up; `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi` reports all violation counts 0.
10. **Commands:**
    - `cargo test --locked -p builtins --features test-support,lane/test-support`,
      `cargo test --locked -p builtins-compiler --features test-support`,
      `cargo test --locked -p host-core --features host-core/test-support`,
      `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`
    - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
      `.github/workflows/qualification.yml`
    - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
    - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-builtins-policy.sh`,
      `bash scripts/check-realtime-policy.sh`, `bash scripts/check-dsp-research.sh`,
      `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`,
      `bash scripts/run-aarch64-tests.sh debug`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
      `cargo fmt --all -- --check`
11. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
    `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if render designs with other code or other arithmetic than the control plane (a
  second designer, an `f32` step), or targets the event sample instead of `τ + 64`.
- Gate 2: red if preparation applies the static value or a flat curve designs anything.
- Gate 3: red if "off" is reached by a ramp of designs, or the disabled completion leaves integrator
  state.
- Gate 4: red if designs grow past the bound, or a refused design aborts the block or changes the
  target.
- Gate 5: red if an automated strip keeps a fixed-design tail shorter than its real decay.
- Gate 6: red if a filter event in a piece changes the elision plan of another piece.
- Gate 7: red if the filter rows stay masked, or a designed target for the other filter overwrites
  the automated section's pair.
- Gate 8: red if the browser admits a pair that the next curve event would overwrite, or refuses a
  lane with no automation.
- Gate 9: red if a design or the apply allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 15 *Render stored input trim and polarity automation* (the input stage split and the collapse
  rule).
- *State a bounded tail and an exact-rest bound for every node* (#1329).
- *Hold strip input-lane values in latest-target cells* (#1346), amended.
- *Retarget a live input filter only through its designs and their mixtures* (#1407).
- Root's confirmation of README finding F1.

Drafts 02, 07, 10a and 10b arrive through draft 15. Draft 16b depends on this draft; this draft is
complete alone.
