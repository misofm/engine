# Make trim, polarity and the input filters live on the C ABI

Slice S5 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C6,
the strip-input family). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs. This slice is tight for one day; if it does not fit, split the input filters (D4) into a
successor and leave `hpf_hz` and `lpf_hz` `RuledLiveNotBuilt` there.

## Product outcome

On iOS and Android, as in the browser, a strip's input trim, polarity, high-pass and low-pass
change live, on tracks and submixes, with the same ramps and the same rendered bits as a plan
prepared from the edited session. The change is one lowering function of the strip-input family
and four row flips; no protocol or host code is added.

## Context

- **Rows.** `polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz` (builtin IDs 1-4) are
  `RebuildOnly(RuledLiveNotBuilt { issue: STRIP_INPUT_SLICE })`, family `StripInput` (#1546 D3); ramps
  `PolarityInvert` (twice `mute_ms`), `InputTrim` (`fader_ms`), and fixed for the filters (their
  64-update coefficient ramp, `INPUT_FILTER_RAMP_SAMPLES`, `crates/builtins/src/filter_control.rs:9`).
- **Records and cells.** `TrackInputRecord` (`crates/builtins-compiler/src/lib.rs:179`):
  `TrimDb`, `PolarityInvert`, `PreparedFilter { target }`. The strip's input cells `InputCells`
  (`:602`; `write`, `:623`, cannot fail and panics on a filter section other than 0 or 1, so the
  target is validated first) on `TrackControlProducer::input` (`:909`, `:922`), present when the plan
  attaches the input lane. A polarity flip negates the trim coefficient on the trim ramp
  (`set_polarity_invert`, `crates/builtins/src/lib.rs:1649`), so it ramps through zero.
  `checked_trim_gain` (`:4796`) is private.
- **Filter targets, designed off render.** `InputFilterPreparer::prepare`
  (`crates/host-core/src/control_preparation.rs:199-203`) designs the dirty lanes' targets from four
  seeds (left HPF, left LPF, right HPF, right LPF) and edits (`InputFilterEdit`, `:100`:
  `parameter_id` 0 pair, 3 HPF, 4 LPF), and merges equal left and right targets into `Both`. It is
  the browser's path (`hosts/host-web/src/control_targets.rs:483`). Validation:
  `validate_input_filter_pair` (`filter_control.rs:36`), `validate_prepared_input_filter_target`
  (`:76`).
- **Tail.** A strip with a live input lane reports the rate's live bound
  (`live_input_bound`, `crates/builtins-compiler/src/lib.rs:4328`), finite for every history of
  trim and filter targets (#1329 D5).
- **The issues this replaces.** #1261 (trim and polarity: its D2 record rules, ramps and gates),
  #1262 (filters: its D1 edit rules), and the input half of #1267 (submix strips). Their frozen
  decisions are this slice's D3 and D4, restated for the family.
- **The machinery.** #1554's `fn lower` (the `StripInput` arm returns `Structure` today) and
  #1555's `controls::live_lanes`, which sets `strip_input` as soon as an input row is `Live`.

## Decisions frozen for this slice

- **D1. Flip the rows.** `STRIP_INPUT_SLICE` takes this issue's number, and the four rows become
  `Live`. `controls::live_lanes` then attaches the input lane on every C ABI plan with a strip.
  `delay_samples` stays `RebuildOnly(NewMemory)`.
- **D2. The family's lowering.** `fn lower(StripInput, ..)` for one strip, given the before and
  after `DualMonoBuiltins`, returns its input records in this order: trims, then polarities, then
  filter targets, left before right.
- **D3. Trim and polarity** (#1261 D2). `TrimDb`: one per lane whose `checked_trim_gain` bits
  change, or one `Both` when both lanes change to the same `trim_db` bits (`0.0` to `-0.0` gives no
  record). `PolarityInvert`: one per lane whose value changes, or one `Both` when both change to the
  same value. Lengths: `LiveRamps::resolve(InputTrim | PolarityInvert, edit)` with the strip's
  `Input` entry. `checked_trim_gain` becomes `pub` (documented as `checked_fader_gain` is); its
  refusal is `LiveRebuild::Domain`.
- **D4. Filters** (#1262 D1). Compare the values each lane's pair normalizes to through
  `validate_input_filter_pair` (`-0.0` to `0.0` is no change). For a strip whose normalized values
  change, call `InputFilterPreparer::prepare` with the pre-commit normalized values as seeds, at
  the session rate, with: `parameter_id` 3 or 4 for a one-sided change of one lane; one pair edit
  (`0`) when both of a lane's values change; one `ParameterChannel::Both` edit for the same change
  on both lanes. Emit one `PreparedFilter` per returned target, in the preparer's order. A refusal
  is `LiveRebuild::Domain`.
- **D5. The commit.** `commit_live` writes the input records to the strip's input cells after every
  fallible check, with the other records of the transaction. A strip with input records and no
  input writer is `CommandError::Internal`, refused before any write. Every strip, tracks and
  submixes.
- **D6. Docs.** `docs/C_ABI_V1_QUALIFICATION.md` lists trim, polarity and the filters as live and
  states that every plan carries an input lane per strip, with the bounded tail.
- **D7. Acked-batch question.** The design runs before the first cell write and a write cannot
  fail; no ack precedes a drop.

## Deliverables

1. D1 (the constant and the four rows).
2. D2-D4 in host-core's lowering; D5 in `commit_live`.
3. D6, and the tests below.

## Authorized paths

- `crates/controls/**`, `crates/builtins/src/lib.rs` (the four rows' class and
  `checked_trim_gain`'s visibility and doc).
- `crates/host-core/src/{live_delta.rs,prepare.rs}`, `crates/host-core/tests/live_delta.rs`.
- `crates/control-plane/src/control.rs`.
- `crates/capi/src/runtime/live_tests.rs`, `crates/capi/tests/**` (tail and resource assertions
  that the input lane changes, re-derived from their functions).
- `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No change to the preparer, the record, the 64-update ramp, the browser's input-filter path or
  `delay_samples`.
- No route, VCA or solo work.

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi`.

1. **Live and exact.** For `trim_db` (per lane and `Both`), `polarity_invert`, `hpf_hz` and
   `lpf_hz`, on a track and on a submix strip, a `SET_CONTROL` commits with the plan epoch
   unchanged, and after the ramp every block is bit-identical to a plan prepared from the committed
   model, at 44.1 and 96 kHz.
2. **Filter enable and disable.** `hpf_hz` from `0` to `80` and back to `0`, with `-0.0` samples in
   the source, ends bit-identical to the prepared plan; a pair change (both of one lane's values)
   that two one-sided edits would refuse is live.
3. **Polarity ramps through zero.** A polarity flip with `ramp_samples` 480 on a constant source
   gives a linear ramp from `+g` to `-g` over 480 samples (no step).
4. **Record rules.** Host-core classifier tests cover each D3 and D4 case: one lane, both lanes
   equal (one `Both`), both lanes different, `0.0` to `-0.0` (no record), the order trims,
   polarities, targets. (Cross-host parity is #1560's.)
5. **Tail and realtime.** A plan with no infinite-tail effect reports `TAIL_FINITE`; `audit capi`
   reports 0 allocations, frees, locks and syscalls.

## Test value

- Gate 1 turns red if a record lands on the wrong lane, strip or rate, or a submix stays structural.
- Gate 2 turns red if a filter target is designed from the wrong seeds or a pair edit is split.
- Gate 3 turns red if a polarity flip steps instead of ramping.
- Gate 4 turns red if a `Both` merge, a no-op rewrite or the record order drifts from the rules the
  browser already follows.
- Gate 5 turns red if the input lane makes the tail infinite or adds render work that allocates.

## Dependencies

- #1554 (the family dispatch), #1555 (lanes from rows), #1553 (`SET_CONTROL` for the tests).
- *Hold strip input-lane values in latest-target cells* (#1346, closed) and *Hold live values in
  latest-target cells on both hosts* (#1312, closed).

## Supersedes and shrinks

- Supersedes *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261) and
  *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262).
- With #1554, supersedes *Apply value-only submix-strip input-section and effect edits to the
  running C ABI plan* (#1267).
