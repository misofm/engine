# Design an input filter section in render

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A1.6 and A1.7, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Root confirms README finding F1 before filing. Batch
R2.

## Product outcome

The strip's input stage can design one HPF or LPF section at a sample offset inside a block, with
the same function and the same check the control plane uses, and start the existing 64-sample
coefficient ramp there. A design that the check refuses keeps the current target and is counted;
render never aborts a block for it. The function is one: preparation, the control plane and render
call it. #1660 *Render stored input HPF and LPF automation* is its first render user, in the
same batch, so `main` never holds it with no caller. No rendered bit moves.

## Context

- **The design authority.** `SvfSection::design(rate, cutoff, high_pass)` designs one Butterworth
  section in `f64` through `math::tan`, one cast per word; `0` is the identity
  (`crates/builtins/src/lib.rs:716-760`). `prepare_input_filter_pair` validates the pair, designs
  both sections and checks each target (`crates/builtins/src/filter_control.rs:52-73`);
  `validate_prepared_input_filter_target` checks the pair order, the words and the state matrix's
  spectral norm, with two `math::sqrt` (`:75-137`). The pair rule is `validate_input_filter_pair`
  (`:35-50`). `INPUT_FILTER_RAMP_SAMPLES = 64` (`:9`).
- **The apply path.** `apply_prepared_filter` sets a 64-sample linear ramp per changed word, then
  refreshes the elision plan and the channel-symmetry words (`crates/builtins/src/lib.rs:1352-1426`;
  `refresh_filter_plan`, `:1263-1295`). A ramp that completes onto identity clears that lane's
  integrators to `+0.0` (`crates/lane/src/kernels/builtins.rs:870-883`). Sample `A` uses the current
  words and sample `A + 64` the exact target (`docs/rulings/builtins-input-liveness-d2.md:17-18`).
- **Off-render design today.** #808's amendment designs live targets off render
  (`docs/rulings/builtins-input-liveness-d2.md:6-14`) and leaves stored rendering to #1058
  (`:163-180`). README A1.6 says why stored automation designs in render (finding F1).
- **The input stage split.** #1658 D1 gives the input stage in-block operations on #1638's
  model.
- **The retarget law.** *Retarget a live input filter only through its designs and their mixtures*
  (#1407) sets the law every filter retarget follows (decision-15 record, `:589-593`).

## Decisions frozen for this slice

- **D1. One section design.** `prepare_input_filter_section(rate, hpf_hz, lpf_hz, section) ->
  Result<PreparedInputFilterTarget, BuiltinParameterError>` is factored out of
  `prepare_input_filter_pair`: `validate_input_filter_pair`, `SvfSection::design` of one section,
  `validate_prepared_input_filter_target`. `prepare_input_filter_pair` calls it twice. Render and the
  control plane call the same function.
- **D2. The in-block operation.** On #1658's model, the input stage gains, per bank lane and
  channel, "design section `k` for `(hpf_hz, lpf_hz)` at offset `o`": it calls D1's function and,
  on success, applies the target through `apply_prepared_filter` at `o`, which ramps over the fixed
  64 samples and refreshes the plan; and "set section `k` exactly at `o`", which writes the designed
  words with no ramp and refreshes the plan, keeping the integrators. An operation whose design
  equals the current target's words changes nothing.
- **D3. A refused design** keeps the current target and adds one to the input bank's
  `automation_designs_refused` count, which a readback reports.
- **D4. Work.** One operation is one `math::tan`, the design arithmetic and two `math::sqrt`, scalar
  `f64`, with no allocation (README A1.7). #1660 bounds the operations per block. For that
  bound's gate, builtins exposes the design count behind its `test-support` feature (a public
  reader of the count `FILTER_DESIGN_CALLS` keeps, `crates/builtins/src/lib.rs:723-724`), so a
  builtins-compiler test can read it.

## Deliverables

1. D1 in `crates/builtins/src/filter_control.rs`.
2. D2 and D3 in the input stage and the input bank processor.

## Authorized paths

- `crates/builtins/src/{lib.rs,filter_control.rs}`, `crates/builtins/tests/filter_liveness.rs`
- `crates/builtins-compiler/src/lib.rs` (the input bank processor's operation and the count only)

## Non-goals

- Stored filter automation, its events, group cell, tail, collapse rule and edits (#1660). EQ
  automation (#1667). Variable filter slopes (#191).

## Hazards

- **F1 is not confirmed yet.** D2 puts a filter design on the render thread. If root refuses F1,
  this slice and #1660 stop; the alternatives fail A4 or A5 (README A1.6).
- **The realtime policy region.** `SvfSection::design` counts calls through a `#[cfg(test)]`
  thread-local (`crates/builtins/src/lib.rs:724`, `:829`). Production builds have no such access;
  `check-realtime-policy.sh` and `audit capi` must accept the call path.
- **A redundant design moves bits.** `apply_prepared_filter` ramps only words that change
  (`:1381-1392`), so an equal design is a no-op.

## Objective gates

1. **Same design** (`crates/builtins/tests/filter_liveness.rs`, new). For HPF values from 20 to
   2,000 Hz under a static LPF, the words D2's operation applies equal
   `prepare_input_filter_section` called on the test thread with the same values, bit for bit, at
   44.1, 48, 88.2 and 96 kHz; `prepare_input_filter_pair` gives the same words as before the
   factoring.
2. **Off** (same file). A design for 0 Hz: 64 samples after the operation the section's words are
   the identity, both integrators are `+0.0`, and the next block takes the elided path.
3. **Refused design** (builtins unit test, new). A value fed past preparation that the check
   refuses keeps the target and adds one to `automation_designs_refused`.
4. **Partition identity** (same builtins file, new). #1658 D1's identity with a design operation
   at `o` in `{1, 63, 64, 127}` (`q = 128`) and in a block of 100, at `Scalar`, `Simd4` and `Simd8`.
5. **No rendered bit moves.** Every existing render test passes with unchanged digests;
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` gives the same
   `pcm_digest` at base and head (PR evidence) and 0 violations.
6. **Commands:**
   - `cargo test --locked -p builtins --features test-support,lane/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`
   - the DSP leg (`test-debug-b`) in `.github/workflows/qualification.yml`
   - `bash scripts/check-builtins-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-dsp-research.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule), `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: red if render designs with other code or other arithmetic than the control plane (a
  second designer, an `f32` step), or if the factoring moves a prepared bit.
- Gate 2: red if "off" leaves integrator state or never elides.
- Gate 3: red if a refused design aborts the block or changes the target.
- Gate 4: red if a design operation in a piece changes the elision plan of another piece.

## Dependencies

Batch R2. Direct dependencies:

- #1658 *Render stored input trim and polarity automation* (the input stage split, D1).
- *Retarget a live input filter only through its designs and their mixtures* (#1407).
- Root's confirmation of README finding F1.

#1660 depends on this draft.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **F1.** The README's F1 proposal (2026-10-14) recommends option A (design in render at events)
  with four conditions; this slice carries the contract amendments of condition 4, including a
  one-line note against decision 17 C6 ("strip input ... through prepared filter targets designed
  off render" stays true for live edits). It still waits for root's confirmation of F1.
- **Anchors.** `SvfSection::design` is now `crates/builtins/src/lib.rs:837-875`; the test counter
  is `FILTER_DESIGN_CALLS` (`:839`). #1407 has landed; #1602 retired the input-design bound cache.
- Realtime policy as in #1637's amendment.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Root's rulings (README "Root's rulings (2026-10-14)").

- **F1 is decided: option A.** Render designs a section at events with D1's function, the one the
  control plane and preparation call; D4's count per block is bounded (#1660 D4) and the render
  call sits in a realtime-policy region; live edits keep designing off render. The hazard "F1 is not
  confirmed yet" and the dependency "Root's confirmation of README finding F1" are met.
- **D5 (new). The contract amendment, for owner review.** This slice changes the texts that say
  targets are designed off render, for stored automation only:
  - `docs/EFFECT_CONTRACT_V1.md` (the prepared-target section: "prepares fixed targets off render"
    gains the stored-automation exception: render designs at bounded events with the owner's own
    designer and check);
  - the trait's doc sentences in `crates/effect-contract/src/prepared_target.rs` ("Render-side code
    never calls this method" gains the same exception; comments only, no code change);
  - `docs/rulings/builtins-input-liveness-d2.md` (one paragraph: stored automation designs in
    render; live edits keep #808's off-render rule);
  - one line against decision 17 C6 in `docs/rulings/one-control-abstraction-2026-10-13.md`: "strip
    input ... through prepared filter targets designed off render" holds for live edits; stored
    automation designs in render (F1 = A).
  Each changed text says "for owner review (root's application, 2026-10-14)". The amendment is
  root's application, subject to the owner's review, as decision 17's classifications are; root
  tells the owner. An owner refusal reopens this slice, #1660 and #1667. The EQ's own
  render-path comments (`crates/parametric-eq/src/lib.rs`) are #1667's (its D8).
- **Authorized paths** gain the four documents and the doc comments above.
