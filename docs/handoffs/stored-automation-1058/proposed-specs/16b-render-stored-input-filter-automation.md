# Render stored input HPF and LPF automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A1.6, A2, A3 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Root confirms README finding F1 before filing. Batch
R2.

## Product outcome

A producer's stored input HPF and LPF sweeps (rows 3 and 4) play on every host. At each event
render designs the filter section with draft 16a's operation, so the coefficients reach each
designed target exactly and the lane's HPF stays below its LPF at every sample. A filter turns off
only by a step, and the section then settles to identity and elides again. A strip with filter
automation reports the live input tail bound. A filter automation edit is a carried rebuild; a
static edit of an automated filter is `model_only`. On a lane whose HPF or LPF is automated, a live
edit of the pair's other filter is a live group-cell write on both hosts: render designs that
section from the new value and the curve, and an edit that would cross the curve at any sample is
refused with `builtin.filter.order`, with nothing changed.

## Context

- **The operation.** Draft 16a: `prepare_input_filter_section` and the input stage's in-block design
  and set operations, with the refused-design count.
- **The C ABI filter path.** *Apply value-only input HPF and LPF edits to the running C ABI plan
  through prepared targets* (#1262) designs targets in the classifier through
  `InputFilterPreparer` (`crates/host-core/src/control_preparation.rs:197-291`) and writes them to
  the strip's input cells (*Hold strip input-lane values in latest-target cells*, #1346 D1: one filter
  cell per `(section, channel)`, the pair and six coefficient words). After *Admit browser live
  edits in the Worker through the committed model* (#1382), a browser live filter edit (today
  `COMMAND_INPUT_FILTERS`, `hosts/host-web/src/lib.rs:893`) reaches the shared commit through the
  Worker's apply and takes the same path.
- **The order check.** Draft 02 D2's `input_filter_order_diagnostics(model)` checks a lane's curves
  against each other or a static value, with the `2^-22` margin, and reads only the model, so it
  can check a candidate model (draft 02 D2, "For a later live edit").
- **Tails.** *State a bounded tail and an exact-rest bound for every node* (#1329) D4 gives a fixed
  design its own tail, and D5 gives a strip with a live input lane `input_section_live_bound(rate)`,
  the bound over any history of targets and 64-sample ramps.
- **Docs.** `docs/BUILTINS_AND_METERING_V1.md:31-34` still calls the filters prepared-only.
- **Draft 15 D4** gives the input rows' collapse rule.

## Decisions frozen for this slice

- **D1. The pair is a target group, per strip lane (left, right).** At each event of the HPF cell,
  render designs section 0 from the HPF curve value and the LPF value; at each LPF event, section 1
  from the LPF curve value and the HPF value. A value comes from its curve when automated, else from
  the group cell. One draft 16a design operation per designed section. Grid period and ramp 64,
  completion `After`: an event at `τ` targets the curve value at `τ + 64`, the sample the ramp
  reaches its target. A jump ramps over the same fixed 64 samples. A seek where it reaches the
  input node uses draft 16a's set operation. Render skips the operation when the curve value's bits
  did not change.
- **D2. The group cell.** For a lane whose pair has an automated filter, this slice gives #1346's
  filter cell for the other section the meaning "group cell": the semantic pair and no
  coefficients. Every other lane keeps #1346's meaning; #1346 is not amended. Preparation seeds it
  with the session's static value. At the next block entry after it changes, render designs that
  section by D1 from the new value and the curve and applies it over the fixed 64-sample ramp, if
  its words change.
- **D3. Off.** "Off" (`0`) is reached only by a `step` (draft 02 D1). Its event designs the identity
  section, and the existing disabled completion clears the integrators; the plan then elides it.
- **D4. Bounded work.** Per section group and block, at most
  `(⌈q/64⌉ + 1) + Σ_cells(⌈q/64⌉ + 1) + 1 seek + 1 cell change` designs: the grid events of the
  group's cells share one 64-sample grid (`⌈q/64⌉ + 1`), each automated cell adds its jumps
  (`⌈q/64⌉ + 1` per cell and block, draft 01's spacing rule), and one seek and one group-cell change
  add one each. Section 0 designs only at HPF events and section 1 only at LPF events, so each
  section group has one cell, and a strip lane designs at most `2·(2⌈q/64⌉ + 4)` times per block.
- **D5. Tail.** A strip with filter automation reports `input_section_live_bound(rate)` (#1329 D5),
  as a strip with a live input lane does: its targets change on render.
- **D6. Mono collapse.** Rows 3 and 4 take draft 15 D4's rule: a `left` or `right` entry declines the
  collapse unless both lanes' entries are bit-identical; a `both` entry keeps it, and its designs
  apply to both channels.
- **D7. Edits, one rule on both hosts.**
  - The classifier's row list (draft 10 D1) gains rows 3 and 4: their automation edits are carried
    rebuilds. A static edit of an automated filter gives no record (draft 10 D3).
  - A static edit of the other filter of an automated pair first runs
    `input_filter_order_diagnostics` on `next`. A diagnostic gives `LiveRebuild::Domain`; the
    rebuild's preparation refuses with the same `builtin.filter.order` (draft 02 D3), and nothing is
    committed. Otherwise the commit writes the group cell (D2), path `live`: the group cell is the
    live slot that decision 14 rule 2 asks for. An edit whose normalized value
    (`validate_input_filter_pair`) leaves the cell's bits unchanged gives nothing.
  - A browser live edit that sets the whole pair reaches the shared commit through the Worker's
    apply and meets the two rules above: the automated filter's value commits as a fallback, and the other filter goes to
    the group cell after the order check.
  - #1262 is not amended: this slice adds the automated-pair branch to the classifier path #1262
    builds.
- **D8. Docs.** `docs/BUILTINS_AND_METERING_V1.md:31-34` states the filters' liveness and the
  stored-automation rule: render designs stored-automation targets with draft 16a's function; live
  edits of a pair with no automation stay designed off render (#808).
- **D9. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; the order check runs before any write; a group-cell write cannot fail; an edit of an
  automated filter gives no record.

## Deliverables

1. D1-D6 in the input stage's caller and the input bank processor, and preparation's seed, tail
   and witness.
2. D7 in the classifier and the commit path; D8.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the input bank processor's events and group-cell read, the
  tail and witness terms)
- `crates/host-core/src/{prepare.rs,live_delta.rs,control_preparation.rs}`,
  `crates/host-core/tests/live_delta.rs`
- `crates/control-plane/src/` (the group-cell write)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/filter_automation_realtime.rs` (new)
- `docs/BUILTINS_AND_METERING_V1.md`

## Non-goals

- The design function and the in-block operation (draft 16a). The preparer's design rules (#1262).
  EQ automation (draft 20). Variable filter slopes (#191).
- A designs-per-second cap: D4's bound is per block and independent of song length.

## Hazards

- **F1 is not confirmed yet.** If root refuses F1, this slice stops (draft 16a Hazards).
- **Draft 02's margin refuses some edits that never cross** (mixed shapes, by design). The refusal is
  the same at preparation and here, so a committed model never fails its own rebuild.

## Objective gates

1. **Flat equals static** (C ABI and browser, new). A flat HPF entry at 80 Hz renders the bits of a
   plan prepared with HPF 80 Hz.
2. **Sweep** (`crates/builtins-compiler` test, new). An exponential HPF sweep from 20 to 2,000 Hz
   under a static LPF: at every event the stage's target words equal `prepare_input_filter_section`
   of the curve value at `τ + 64`, bit for bit.
3. **Bounded designs** (builtins-compiler unit test, new). Over a block of 128 and one of 100, with a
   moving HPF and a moving LPF, the design count (`FILTER_DESIGN_CALLS`) is at most D4's bound.
4. **Tail.** A strip with HPF automation and no live lane reports `input_section_live_bound(rate)`;
   #1329's gates pass unchanged.
5. **Group cell, both hosts** (`crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
   new). HPF ride from 100 to 900 Hz on `both`, static LPF 1,000 Hz. A transaction (C ABI) and a
   browser live filter edit through the Worker's apply set LPF 2,000 Hz: path `live`, the same provider epoch; after
   its ramp the output equals a plan prepared with LPF 2,000 Hz and the same ride.
6. **Crossing refused** (same files). An edit that sets LPF 500 Hz is refused with
   `builtin.filter.order` (on the C ABI `MISO_ENGINE_V1_COMPILE_REJECTED`); the revision and the
   output are unchanged.
7. **Edits** (`crates/host-core/tests/live_delta.rs`, new). An HPF entry change gives
   `Err(LiveRebuild::Automation)`; a static change of the automated HPF gives no record; gate 5's
   edit gives one group-cell write and no `PreparedFilter` record; gate 6's gives `Domain`; an
   `lpf_hz` rewrite from `1000.0` to the same bits gives nothing.
8. **Realtime.** `hosts/host-web/tests/filter_automation_realtime.rs` (new integration binary; links
   `bench_support::alloc`, calls `assert_installed()` first): `allocations == 0 && frees == 0`
   around every render call after warm-up; `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi` reports all violation counts 0.
9. **Commands:**
   - `cargo test --locked -p builtins --features test-support,lane/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`,
     `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p control-plane --features test-support`,
     `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
     `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-dsp-research.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
10. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
    `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if preparation applies the static value or a flat curve designs anything.
- Gate 2: red if render targets the event sample instead of `τ + 64`, or reads the other filter from
  the wrong source.
- Gate 3: red if designs grow past the bound.
- Gate 4: red if an automated strip keeps a fixed-design tail shorter than its real decay.
- Gate 5: red if a live edit of the other filter is lost, overwrites the curve, rebuilds, or differs
  between the hosts.
- Gate 6: red if the edit can cross the curve, or if the refusal leaves a committed model its own
  rebuild refuses.
- Gate 7: red if the filter rows stay masked, or the classifier designs a target with a stale pair,
  or writes an unchanged cell.
- Gate 8: red if a design or the apply allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 16a *Design an input filter section in render* (it brings draft 15).
- *State a bounded tail and an exact-rest bound for every node* (#1329).
- *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262): the classifier path D7 extends.
- *Hold strip input-lane values in latest-target cells* (#1346): the cell D2 lays out.
- Root's confirmation of README finding F1.

Drafts 02, 07, 10 and 12 arrive through drafts 15 and 16a.
