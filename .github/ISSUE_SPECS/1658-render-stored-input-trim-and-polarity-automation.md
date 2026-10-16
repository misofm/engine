# Render stored input trim and polarity automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.4, A1.5 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2.

## Product outcome

A producer's stored input trim rides (row 2) and polarity flips (row 1) play on every host. At each
grid completion sample the lane's trim coefficient equals the trim of the curve value exactly; a
polarity step flips the sign at its exact sample and ramps through zero over the session mute
length. A track with a `both` entry keeps its mono collapse; a track whose input automation differs
between its lanes declines it. A trim or polarity automation edit is a carried rebuild. A static
edit of an automated row, and a browser live command on it, commit as `model_only` on both hosts
through the one shared rule of #1648.

## Context

- **One coefficient per lane and channel.** Trim and polarity share it: `set_trim_signed` retargets
  the signed coefficient over a window, one division per channel per event
  (`crates/builtins/src/lib.rs:1434-1478`). `set_trim_db` keeps the target's sign (`:1480-1500`);
  `set_polarity_invert` keeps the target's magnitude, so a flip ramps through zero (`:1502-1519`).
  The public bank converts dB through `checked_trim_gain`, `10^(dB/20)` in `f64` through `math`,
  rounded once (`:3685-3700`, `:4310-4315`, `db_gain` `:5356-5364`).
- **The kernel.** `input_chain_ramp_block` advances each channel's trim ramp per frame by its own
  additions, so it is partition-invariant (`crates/lane/src/kernels/builtins.rs:618-726`). A
  ramping block runs the unelided body, which gives the elided bits (`:620-636`). On a collapsed
  track `input_chain_ramp_block_mono` advances the left channel only, and the caller
  duplicates its state onto the right after the block (`:728-736`). Input is sanitized once per
  channel per block, and a lane whose block output is not finite has its whole block zeroed and its
  sections reset (`docs/BUILTINS_AND_METERING_V1.md:54-62`).
- **Upstream of the seam.** The input section is upstream of the mono-collapse seam
  (`TrackInputRecord`'s `SEAM`, `crates/builtins-compiler/src/lib.rs:213-216`; the bank drains in
  `begin_block`, before the collapse dispatch reads the witness, `:415-424`, `:525-530`). The
  prepare-time witness takes a `DESIGNED` term from `InputBuiltins::channel_symmetry`
  (`crates/builtins/src/lib.rs:3350`; `crates/builtins-compiler/src/lib.rs:4064-4066`); a lane pair
  with different `delay_samples` declines the collapse the same way (`track_input_delay_symmetric`,
  `:3991-3993`; `docs/SESSION_SCHEMA_V1.md:236-237`).
- **The C ABI input path.** *Apply value-only input trim and polarity edits to the running C ABI
  plan* (#1261) emits `TrimDb` and `PolarityInvert` records, and *Hold strip input-lane values in
  latest-target cells* (#1346) holds them in cells.
- **Jump lengths** (#1054 D3): `fader_ms` for trim, `mute_ms` for polarity (#1639's words).
- **Browser edits.** Today `COMMAND_TRIM_DB = 10` and `COMMAND_POLARITY_INVERT = 11`
  (`hosts/host-web/src/lib.rs:882`, `:891`). After *Admit browser live edits in the Worker through
  the committed model* (#1382) every browser live edit reaches the shared commit through the
  Worker's apply and meets the shared classifier.
- *Keep every trim, fader and matrix ramp inside its endpoints* (#1408) changes the trim ramp's
  per-sample law; it does not change where events go.

## Decisions frozen for this slice

- **D1. In-block operations on the input stage**, on #1638's model: per bank lane and channel, at
  an offset, "retarget the signed coefficient over `n`" and "set it exactly". Between operations
  the existing kernels run, filter plan included. Class A: a block with an operation at offset `o`
  equals the same block split at `o`, bit for bit, at `Scalar`, `Simd4` and `Simd8`, in the dual and
  the mono form. On a collapsed lane, each piece boundary duplicates the left channel's state onto
  the right, as the block end does, before the operation applies to both channels. The block's
  sanitize count is the sum of its pieces and its finiteness flags their OR; the non-finite recovery
  (zero the lane's block, reset its sections) runs once, after the last piece, on the whole block,
  as today.
- **D2. Events.**
  - **Trim** (row 2): grid period and ramp 64, completion `End` (`τ + 63`). The target is the
    signed coefficient: the trim gain of the curve value (`checked_trim_gain`'s conversion) with the
    lane's current sign. Jumps ramp over the fader length.
  - **Polarity** (row 1, `step` with 0 or 1 only, #1645 D1): jumps only. The target keeps the
    lane's remembered trim magnitude and takes the curve's sign, over the mute length.
  - **One target per offset.** A trim event and a polarity event at one offset make one signed
    target; its ramp is the polarity length when the sign changes there, else the trim's.
  - **Held events.** A trim event whose 64-sample ramp would end before a polarity ramp in flight
    on the same lane and channel is held: the magnitude is remembered, and the grid resumes at its
    first grid sample after the polarity ramp ends (README "Held events"). A live record's ramp in
    flight holds events the same way.
  - A seek sets the coefficient exactly. Only a target whose bits change retargets.
- **D3. Rows and lanes.** Trim and polarity are separate rows. A live record of the row the session
  does not automate is applied as today: `set_trim_db` keeps the sign and `set_polarity_invert`
  keeps the magnitude, so a live edit of one row never overwrites the other row's curve.
- **D4. Mono collapse** (A1.5). Preparation adds one `DESIGNED` term: a track whose rows 1 or 2 have
  a `left` or `right` entry, unless both lanes' entries of that row have bit-identical segment
  tables, is asymmetric upstream of the seam and declines its collapse. A `both` entry keeps it, and
  its events apply identically to both channels, so the input stage's own witness stays symmetric.
- **D5. Edits.** The classifier's row list (#1648 D1) gains rows 1 and 2: their automation edits
  are carried rebuilds. A static trim edit on an automated trim lane, or a static polarity edit on
  an automated polarity lane, gives no record (#1648 D3), on both hosts: a browser
  live trim or polarity edit reaches that rule through the Worker's apply (#1382) and replies
  `model_only`.
- **D6. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; an edit of an automated row gives no record or rebuilds; nothing is queued.

## Deliverables

1. D1 in `crates/builtins` (the input stage) and the input bank processor.
2. D2-D4: events, preparation's witness term.
3. D5: the classifier row.

## Authorized paths

- `crates/builtins/src/lib.rs` (the input stage),
  `crates/builtins/tests/{input_liveness.rs,input_liveness_mono.rs}`
- `crates/builtins-compiler/src/lib.rs` (the input bank processor and the witness term)
- `crates/lane/src/kernels/builtins.rs` (a range entry only, if the split needs one)
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/live_delta.rs`,
  `crates/host-core/tests/{live_delta.rs,symmetry_witness.rs}`
- `crates/capi/src/runtime/live_tests.rs`
- `hosts/host-web/src/tests.rs`, `crates/control-plane/tests/input_automation_realtime.rs` (new)
- `crates/control-plane/Cargo.toml` (a `bench-support` dev-dependency for that binary, only if an
  earlier slice has not added it), `Cargo.lock`

## Non-goals

- HPF and LPF automation (#1659 and #1660), though the split runs the filter kernels unchanged.
- `delay_samples` (row 11), which is not a target.

## Hazards

- **The filter plan between pieces.** The elided kernels are picked per call from the filter plan.
  A piece boundary with no filter event must pick the same plan; gate 5 holds it.
- **Sign of zero.** A polarity ramp passes through zero; with trim at its domain minimum the
  coefficient's sign must still follow the curve. Gate 3 checks the sign bit at completion.

## Objective gates

1. **Flat equals static** (C ABI and browser, new). Flat trim at -6 dB and flat polarity 1 render the
   bits of a plan prepared with those static values.
2. **Completion samples** (`crates/builtins/tests/input_liveness.rs`, new). A linear trim ride: at
   every grid completion sample the lane's coefficient equals `checked_trim_gain` of the curve value,
   with the lane's sign, bit for bit.
3. **Polarity step** (same file). A `step` from 0 to 1 at a sample that is not a grid sample: the
   ramp starts exactly there, passes through zero, and reaches the negated magnitude after the mute
   length; trim events inside it are held, and the grid resumes on its next grid sample.
4. **Rows do not fight** (`crates/capi/src/runtime/live_tests.rs`, new). Trim automated, polarity
   live: a live polarity flip during a trim ride flips the sign and the ride continues; after the
   flip ramp, the output equals a plan prepared with the flipped polarity and the same ride.
5. **Partition identity** (`crates/builtins/tests/input_liveness.rs` and
   `input_liveness_mono.rs`, new). D1's identity at `o` in `{1, 63, 64, 127}` (`q = 128`) and in a
   block of 100, at `Scalar`, `Simd4`, `Simd8`, dual and mono, with filters off and on, and with a
   NaN input sample in the first piece (the whole block is recovered, as an unsplit block is).
6. **Collapse** (`crates/host-core/tests/symmetry_witness.rs`, new). A mono-source track with a
   `both` trim ride keeps its collapse and renders the bits of the same session forced dual; a track
   with a `left`-only ride declines it.
7. **Edits** (`crates/host-core/tests/live_delta.rs`, new; browser in `hosts/host-web/src/tests.rs`).
   A trim entry change gives `Err(LiveRebuild::Automation)`; a static trim change on an automated
   lane gives no record; a static polarity change on that lane gives its record. In the browser,
   a live trim edit through the Worker's apply on the automated lane replies `model_only` and
   moves no bit, and a live polarity edit there replies `live`.
8. **Realtime.** `crates/control-plane/tests/input_automation_realtime.rs` (new integration binary in control-plane, the crate that
   holds the shared commit, `control_plane::SessionState`, #1309 D1-D9. A host-core binary cannot
   reach the commit without a dev-dependency cycle, since control-plane depends on host-core.
   `scripts/check-bench-policy.sh:257-280` allows a `bench-support` dev-dependency only in a
   `crates/` manifest, so no host-web binary can link it. The binary links `bench_support::alloc`
   and calls `assert_installed()` first). It drives the script through control-plane's shared
   commit and host-core's render session, the code the browser Worker (#1382) and the C ABI both
   run: `allocations == 0 && frees == 0`
   around every render call after warm-up; `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi` reports all violation counts 0.
9. **Commands:**
   - `cargo test --locked -p builtins --features test-support,lane/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`,
     `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo test --locked -p control-plane --features test-support`
   - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
     `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule), `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
10. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
    `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests; the
    mono-collapse fixtures (`crates/builtins/tests/mono_collapse.rs`) pass unchanged.

## Test value

- Gate 1: red if preparation applies the static value or a flat curve emits events.
- Gate 2: red if the conversion differs from `checked_trim_gain`, or a trim event drops the sign.
- Gate 3: red if a polarity step snaps to the grid, uses the fader length, or a trim event cuts the
  flip short.
- Gate 4: red if a live record of one row overwrites the other row's curve.
- Gate 5: red if a piece boundary changes the filter plan, the sanitize count, the mono
  duplication, or recovers only the failing piece.
- Gate 6: red if asymmetric input automation keeps the collapse (wrong audio on the right lane), or
  symmetric automation retires it.
- Gate 7: red if the rows stay masked, if a live command on an automated row fights the curve, or
  if a row the session does not automate stops being live.
- Gate 8: red if event handling allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- #1648 *Classify fader automation edits as carried rebuilds*.
- #1639 *Hold the automation jump lengths in a plan cell* (the jump lengths).
- *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261), for the C
  ABI's static-edit path.
- *Hold strip input-lane values in latest-target cells* (#1346), for the C ABI's static-edit path.

#1637's events, #1638's model (D1 follows it) and *Session `controlSmoothing`: configurable
ramp lengths for live mute, fader and pan changes* (#1054) arrive through #1648 and #1639.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **OQ1 = C: trim takes the hand offset.** When this slice adds rows 1 and 2 to
  `automation_row_renders`, #1650's write rule applies to trim: `SET_CONTROL trim_db` on an
  automated trim lane writes that lane's stored offset (live). This slice adds the trim offsets
  cell (one `f64` and a ramp word, 36 bytes per automated trim lane, charged as #1646 D4
  charges the fader's) and composes each trim event as `checked_trim_gain(clamp(f64(curve) +
  offset, -144, 24))` with the lane's current sign; a cell change jumps at the next block entry over
  `LiveRamps::resolve(InputTrim, edit ramp)`. A flat trim at `v` with offset `o` renders the bits of
  a static trim of `f32(clamp(f64(v) + f64(o)))`. D5's trim sentence is replaced by this rule.
- **Polarity: automation wins** (D5's rule stands for row 1). Its jump length is
  `LiveRamps::for_row(PolarityInvert)`, twice `mute_ms` (decision 17's ramp row), not the mute
  length of D2.
- **Browser commands** `COMMAND_TRIM_DB` and `COMMAND_POLARITY_INVERT` are deleted (#1565).
  #1261 and #1346 have landed. Realtime policy as in #1637's amendment.

## Rulings (root, batch 17, 2026-10-16)

These rulings override the filed text and every amendment above where they differ.

### R17-6: this issue owns the hand offset for the input trim

Under OQ1 = C, a hand move of an automated trim dB adds a stored dB offset. The trim offset
(its field use, its write and its gates) moved here from #1650. #1650 now delivers the fader offset
only. This issue depends on #1650, which ships the shared mechanism for the fader: the offset
field and grammar (#1650a), and the offset write, the in-place owner rule and the list flag
(#1650b).

The rule is the one of #1650, for trim (row 2, `trim_db`, builtins control 2):

- **The offset field.** The field is the one of #1650 D1: `offset { left_db, right_db }` on the
  entry that drives the lane. This issue adds `trim_db` to `takes_offset` in
  `controls::automation_diagnostics`, in the same slice that adds row 2 to
  `automation_row_renders`. The domain is `[-144, 24]` dB. Before this issue a non-zero offset on a
  trim entry is refused `control.automation.offset`. A polarity entry (row 1) takes no offset.
- **The write.** On a lane whose trim automation renders, `SET_CONTROL trim_db` writes that lane's
  offset in the entry that drives the lane, not the static field. The path is `live`. The control
  list reports the offset as the row's `value` and `automated = true`. A `both` write sets both
  lanes' offsets.
- **The in-place owner rule (R17-1).** The automation entries that drive the lane are an owner of
  the in-place edit for row 2: `OwnerEdits`, `OwnerSlots`, `ModelView`, and the row's read and write
  with model scope. A hand move reaches the offset on every path. The reply and the sound agree. An
  ack of a value that does not sound is a defect.
- **Classifier.** For an automated trim lane the lowering writes the trim offsets cell (the 2026-10-14
  amendment above) with `LiveRamps::resolve(InputTrim, the edit's ramp)`. It writes no `TrimDb`
  record. An offset-only `UpsertAutomation` is classified the same way.
- **Equivalence gate.** The same `SET_CONTROL trim_db` through the in-place path and through the full
  path gives the same reply and the same bits. It runs on both hosts: the C ABI
  (`crates/capi/src/runtime/live_tests.rs`) and the browser (`hosts/host-web/tests/control_parity.rs`).
- **Gates moved here from #1650.** A hand move of `+3` on a trim ride is heard as the ride plus
  `+3` after the ramp, bit for bit equal to a fresh plan whose entry carries that offset, and the
  other lane is unchanged. Offset `0` and `-0.0` give the bits of no offset. Re-authoring the
  segments keeps the offset. A non-zero `right_db` on a `left` trim entry is refused. The 2026-10-14
  amendment's bit rule for a flat trim stands.
- **Polarity stays "automation wins"** (D5 for row 1). No offset reaches it.
- **Paths added to this issue's list:** `crates/controls/src/{automation.rs,builtin_rows.rs,owner_edits.rs}`
  and `crates/controls/tests/**` (the `takes_offset` entry, the row 2 binding and its owner),
  `hosts/host-web/tests/control_parity.rs`, `docs/SESSION_SCHEMA_V1.md` (the automation section
  names trim as taking the offset). The batch R2 order stands: this issue follows #1650.
