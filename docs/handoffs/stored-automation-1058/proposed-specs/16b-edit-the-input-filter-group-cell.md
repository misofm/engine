# Edit an input filter group cell under stored automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A2, A3 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2, after draft 16a *Render stored input HPF
and LPF automation*.

## Product outcome

On a lane whose HPF or LPF is automated, a C ABI transaction that moves the pair's other filter is a
live update: render designs that section from the new value and the curve, and it never crosses the
automated filter. An edit that would cross the curve at any sample is refused with
`builtin.filter.order`, and nothing changes. The browser refusal of a filter command on an
automated pair is draft 16a's and stays until #1382.

## Context

- **The group** is draft 16a D2: render designs each section from the curve of an automated filter
  and the group cell for the other. Draft 16a D8 makes a static edit of the other filter a carried
  rebuild, because the plan has no live slot for it yet (decision 14 rule 2), and refuses the
  browser's filter command on such a pair.
- **The C ABI filter path.** *Apply value-only input HPF and LPF edits to the running C ABI plan
  through prepared targets* (#1262) designs targets in the classifier through
  `InputFilterPreparer` (`crates/host-core/src/control_preparation.rs:197-291`) and writes them to
  the strip's input cells (*Hold strip input-lane values in latest-target cells*, #1346 D1: one filter
  cell per `(section, channel)`, the pair and six coefficient words). README amendment rows #1262 and
  #1346: for an automated pair, a live edit of the other filter writes the group cell, not a designed
  pair, after A3's order check.
- **The order check.** Draft 02 D2's `input_filter_order_diagnostics(model)` checks a lane's curves
  against each other or a static value, with the `2^-22` margin, and reads only the model, so it
  can check a candidate model (draft 02 D2, "For a later live edit").

## Decisions frozen for this slice

- **D1. The order check first.** For a lane whose pair has an automated filter, a static edit of the
  other filter runs `input_filter_order_diagnostics` on `next`. A diagnostic gives
  `LiveRebuild::Domain`; the rebuild's preparation refuses with the same `builtin.filter.order`
  (draft 02 D3), and nothing is committed.
- **D2. The group-cell write.** Otherwise the commit writes the lane's group cell (the #1346 filter
  cell for that section and channel, holding the semantic pair and no coefficients), path `live`.
  The group cell is the live slot that decision 14 rule 2 asks for. It replaces draft 16a D8's
  rebuild for this case. An edit whose normalized value (`validate_input_filter_pair`) leaves the
  cell's bits unchanged gives nothing.
- **D3. Render.** At the next block entry after the cell changes, render designs that section by
  draft 16a D2 from the new value and the curve and applies it over the fixed 64-sample ramp, if its
  words change.
- **D4. The acked-batch question: can an ack ever precede a drop? No.** The order check runs before
  any write; the group-cell write cannot fail.

## Deliverables

1. D1 and D2 in the classifier and the commit path.
2. D3 in the input bank processor.

## Authorized paths

- `crates/host-core/src/{live_delta.rs,control_preparation.rs}`, `crates/host-core/tests/live_delta.rs`
- `crates/builtins-compiler/src/lib.rs` (the group-cell read)
- `crates/control-plane/src/` (the group-cell write), `crates/capi/src/runtime/live_tests.rs`

## Non-goals

- Rendering filter automation and the browser refusal (draft 16a). The preparer's design rules (#1262). A browser transaction
  path (#1382).

## Hazards

- **Draft 02's margin refuses some edits that never cross** (mixed shapes, by design). The refusal is
  the same at preparation and here, so a committed model never fails its own rebuild.

## Objective gates

1. **Group cell** (`crates/capi/src/runtime/live_tests.rs`, new). HPF ride from 100 to 900 Hz on
   `both`, static LPF 1,000 Hz. A transaction sets LPF 2,000 Hz: path `live`, the same provider
   epoch; after its ramp the output equals a plan prepared with LPF 2,000 Hz and the same ride.
2. **Crossing refused** (same file). A transaction that sets LPF 500 Hz returns
   `MISO_ENGINE_V1_COMPILE_REJECTED` with `builtin.filter.order`; the revision and the output are
   unchanged.
3. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). Gate 1's edit gives one group-cell
   write and no `PreparedFilter` record; gate 2's gives `Domain`; an `lpf_hz` rewrite from `1000.0`
   to the same bits gives nothing.
4. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p builtins-compiler --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` (all
     violation counts 0)
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
5. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a live edit of the other filter is lost, overwrites the curve, or still rebuilds.
- Gate 2: red if the edit can cross the curve, or if the refusal leaves a committed model its own
  rebuild refuses.
- Gate 3: red if the classifier still designs a target with a stale pair, or writes an unchanged
  cell.

## Dependencies

Batch R2. Direct dependencies:

- Draft 16a *Render stored input HPF and LPF automation*.
- *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262), amended.

Draft 02's order function, draft 10b and *Hold strip input-lane values in latest-target cells*
(#1346, amended) arrive through draft 16a.
