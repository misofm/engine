# Apply value-only input trim and polarity edits to the running C ABI plan

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-4, D15-6).
Code anchors verified on `main` at `6fb211594`.

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053), and part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`), which classifies `trim_db` and
`polarity_invert` as live.

## Product outcome

A C ABI transaction that changes a track's input `trim_db` or `polarity_invert` (left, right or
both lanes), alone or together with fader, mute, pan and effect values, is a live update. It ramps
over the session's ramps and never rebuilds the plan. Every C ABI plan carries an input lane per
strip, and that lane never makes the plan's tail `TAIL_INFINITE`: the input section reports the
bounded tail of decision 15 D15-4. Today such an edit rebuilds the plan.

## Context

- **The model.** `DualMonoBuiltins { left, right }` of `ChannelBuiltins { polarity_invert,
  trim_db, hpf_hz, lpf_hz, delay_samples }` (`crates/session/src/model.rs:470-508`).
  `delay_samples` stays structural (decision 14, rule 1). `hpf_hz` and `lpf_hz` are #1262's.
- **The domain.** `trim_db` is checked by the private `checked_trim_gain`
  (`crates/builtins/src/lib.rs:4310`). Both live setters call it (`InputBuiltins::set_trim_db`,
  `:3393-3402`; `BuiltinInputBank::set_trim_db`, `:3685-3699`). Preparation spells the same
  `[-144, 24]` range itself (`prepare_sections`, `:3283-3288`). `checked_fader_gain` (`:4292`)
  is already `pub`.
- **The kernel.** Trim and polarity share one signed coefficient. A trim retarget keeps the sign,
  a polarity retarget keeps the magnitude, and both ride the same linear ramp, so a polarity flip
  passes through zero over the ramp (`InputStage::set_trim_db`, `set_polarity_invert`,
  `crates/builtins/src/lib.rs:1480-1520`).
- **The record.** `TrackInputRecord::{TrimDb, PolarityInvert, PreparedFilter}`
  (`crates/builtins-compiler/src/lib.rs:175-212`). Its `symmetry_event` (`:214-233`) keeps the
  channel-symmetry witness for a `Both` record. A `Left` or `Right` record clears the witness's
  `LIVE` term (`crates/effect-contract/src/symmetry.rs:297`), which retires the strip's mono
  collapse for the life of the plan. That costs performance, never correctness.
- **The drain.** `BuiltinBankProcessor::drain_controls` (`crates/builtins-compiler/src/lib.rs:460`)
  applies input records at block entry.
- **The classifier.** `host_core::classify_live_delta` (`crates/host-core/src/live_delta.rs:211`)
  masks only track fader, pan/matrix and effect `params`/`bypass` (`:228-264`). Every other field,
  the input section included, is structural. `LiveRamps` (`:39-58`) carries fader and mute ramps only (both 0 today).
- **The lanes.** capi selects `C_ABI_LIVE_LANES`: fader, matrix and effect lanes, no input lane
  (`crates/capi/src/runtime/compile.rs:14-21`, comment `:583-585`). `HostLiveLanes::strip_input`
  (`crates/host-core/src/prepare.rs:371-395`) attaches one `TrackControlProducer::input` per strip,
  submixes included (`crates/builtins-compiler/src/lib.rs:254-270`).
- **The tail today.** A strip with a live input lane reports `BuiltinTail::Infinite`
  (`crates/builtins-compiler/src/lib.rs:3261-3267` and `:3562-3568`). capi maps an infinite output
  tail to `TAIL_INFINITE` (`crates/capi/src/runtime/compile.rs:648-651`). Decision 15 D15-4
  replaces that rule: *State a bounded tail and an exact-rest bound for every node* (#1329) deletes
  `BuiltinTail`, and its `input_section_live_bound(rate)` gives a strip with a live input lane a
  bounded tail over the whole reachable filter domain (#1329 D5, D7). Effects keep their own
  tails until their follow-ups (for example #1372 for the parametric EQ), so a plan with such an
  effect can still report `Infinite` for that reason.
- **The commit.** `commit_live` (`crates/capi/src/runtime/control.rs:1065`) classifies, admits,
  resolves producers, checks every queue's room (step 4, `:1189`), pushes (step 6, `:1287`) and
  commits. *Extract the C ABI control plane into a portable crate both hosts call* (#1309) moves
  `control.rs` and `compile.rs` unchanged into `crates/control-plane/src/` (lib `control_plane`);
  capi's tests stay in capi. The classifier stays in host-core. #1312 replaces the live value
  queues with latest-target cells, and *Hold strip input-lane values in latest-target cells*
  (#1346) does so for the strip input lane.
- **The browser's lowering.** `COMMAND_TRIM_DB` and `COMMAND_POLARITY_INVERT`
  (`hosts/host-web/src/lib.rs:4313-4350`).

## Decisions frozen for this slice

- **D1. The lanes.** The C ABI's selection sets `strip_input: true` for every plan, at compile and
  at every structural replacement. Former owner question Q4 of #1053 ("accept an infinite tail?")
  is superseded by decision 15 D15-4: the input section reports the bounded tail #1329 defines,
  never `Infinite`.
- **D2. The classifier.** `classify_live_delta` also masks each track's `builtins.left` and
  `builtins.right` `trim_db` and `polarity_invert`. The strip's records gain an input list:
  - **TrimDb.** One per lane whose `checked_trim_gain` bits change, or one `Both` when both lanes
    change to the same `trim_db` bits. A `0.0` to `-0.0` rewrite gives the same gain bits, so it
    is live with no record.
  - **PolarityInvert.** One per lane whose bool changes, or one `Both` when both change to the
    same value.
  - **Order.** TrimDb records, then PolarityInvert records (then #1262's filter targets); left
    before right.
  - **Domain.** A trim that `checked_trim_gain` refuses gives `LiveRebuild::Domain`.
    `checked_trim_gain` becomes `pub`, documented as `checked_fader_gain` is.
  - **Ramps.** Per *Session `controlSmoothing`: configurable ramp lengths for live mute, fader
    and pan changes* (#1054) D3 and D4, a TrimDb record carries `LiveRamps::fader_samples` (the
    `fader_ms` key) and a PolarityInvert record `LiveRamps::mute_samples` (the `mute_ms` key),
    each read through `LiveRamps::resolve` with the strip's `Input` entry, so a
    `SetTrackBuiltins` that carries its own length uses it (*Carry an optional per-edit ramp
    length on live session edits*, #1394 D6).
    No input-specific ramp field exists (D15-1: every live value ramps; an explicit 0 stays
    legal).
  - `delay_samples` stays compared, so its change is `Structure`.
- **D3. The cells.** The records are written to the strip's input cells from *Hold strip
  input-lane values in latest-target cells* (#1346), built on *Hold live values in latest-target
  cells on both hosts* (#1312), after every fallible check of the commit (D15-2 condition 3), in
  #1346's canonical drain order. There is no room check: a cell has no capacity,
  and a value superseded before render drains it is counted by `live_values_superseded`. A strip
  with input records whose input writer is `None` is `CommandError::Internal`, refused before any
  write.
- **D4. The tail.** This slice computes no tail. The C ABI report carries the output tail
  host-core derives from #1329's node tails. With D1, a plan whose strips carry no effect with an
  `Infinite` tail reports `TAIL_FINITE`.
- **D5. Acked-batch question.** Can an ack ever precede a drop? No. Every check runs before the
  first cell write, a write cannot fail, and a superseded value is in the committed model, which
  is the value render converges to.
- **D6. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists trim and polarity as live, states
  that every plan carries an input lane per strip, and records the bounded tail.

## Deliverables

1. D2 in the classifier, with its tests.
2. D1 and D3 in the control-plane crate, with capi's tests.
3. `checked_trim_gain` made `pub`.
4. D6.

## Authorized paths

- The classifier: `crates/host-core/src/live_delta.rs` and `crates/host-core/tests/live_delta.rs`.
- `crates/builtins/src/lib.rs`: `checked_trim_gain`'s visibility and documentation only.
- The control plane: `crates/control-plane/src/compile.rs` (`C_ABI_LIVE_LANES`) and
  `crates/control-plane/src/control.rs` (`commit_live`).
- capi's tests: `crates/capi/src/runtime/live_tests.rs` and
  `crates/capi/tests/resource_lifecycle.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No input filters (#1262), no `delay_samples`, no submix-strip input section (#1267).
- No tail computation (#1329) and no cell layout (#1312, #1346).
- No browser change: the browser already attaches the input lane.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - each lane rule of D2: left only, right only, both equal (one `Both`), both different (two
     records);
   - trim and polarity changing together, in D2's order;
   - `trim_db` `0.0` to `-0.0` is live with no record;
   - a `delay_samples` change gives `Structure`;
   - a trim past either end of `[-144, 24]`, and a NaN trim, give `Domain`;
   - a TrimDb record carries `LiveRamps::for_session(next).fader_samples` and a PolarityInvert
     record its `mute_samples`.

   `cargo test --locked -p host-core --all-targets --features host-core/test-support`
2. **Equal to a rebuild.** New cases in the capi live tests, on `long_session` rewritten through
   `with_model` so that nothing at or after the trim keeps state: no console slots and no inserts
   (its stock form has a console EQ), and every strip's `hpf_hz` and `lpf_hz` set to `0.0` on both
   lanes, which turns the input filters off (the pattern `submix_session` uses,
   `crates/capi/src/runtime/live_tests.rs:977-981`). The parity session's filters run after the
   trim in the input stage, so with them on, their state would keep the ramp's difference forever
   and no block would equal the rebuild:
   - a trim of -6 dB on the left lane;
   - a polarity flip on both lanes.

   For 1 and 10 tracks at the four launch rates, each commit is live (no new plan epoch). From
   block E + ceil((ramp + `latency_samples`) / quantum) + 1 on (ramp = the record's), the output is bit-identical
   to a `Reference` compiled from the committed snapshot and fed the same source.
3. **Equal to the browser's lane.** The same edits through the C ABI, and hand-built
   `TrackInputRecord`s (written in the test from the edit, never taken from the classifier)
   applied at the same block to a host-core plan prepared with `HostLiveLanes::ALL`
   (`LaneReference`), render bit-identically, ramp included, for 10 tracks at 48 kHz.
4. **The tail and the resources.** At every launch rate, a fresh C ABI plan of gate 2's session
   reports `TAIL_FINITE`, and on both gate 2's session and stock `long_session` its tail equals
   the output tail of the same session prepared with
   `prepare_host_runtime_with_live_lanes(.., HostLiveLanes::ALL)`. The resource rows change only
   by the input cells. `cargo test --locked -p capi` and
   `cargo test --locked -p control-plane --all-targets --features control-plane/test-support`
5. **Render stays clean.**
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
   `./target/release/audit capi` reports 0 `allocations`, `deallocations`, `locks` and `syscalls`.
6. **Nothing else changes.**
   - `cargo test --locked --all-targets -p builtins --features builtins/test-support`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-realtime-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all -- --check`
   - 4-lane (NEON) is CI-only here.

## Test value

- Gate 1: red if a trim or polarity record goes to the wrong lane, uses `Left` and `Right` where
  `Both` was due (retiring the mono collapse for nothing), emits a record for a sign-of-zero
  rewrite, carries the wrong ramp key, or lets a delay change through.
- Gate 2: red if a live trim or polarity does not end where preparation bakes it.
- Gate 3: red if the C ABI lowers or times a value differently from the browser's lane.
- Gate 4: red if a C ABI plan lacks the input lane, or its input section still makes the tail
  `Infinite`, or the C ABI tail differs from host-core's.
- **Superseded in the same PR.** `a_sign_of_zero_trim_edit_is_structural`
  (`crates/host-core/tests/live_delta.rs:390`) is rewritten on the `gain_db` of a route into the
  output, which stays structural, so its defence (the mask compares bytes, not `PartialEq`)
  survives. Any capi case that asserts a trim or polarity edit rebuilds is changed to the live
  path.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Hold strip input-lane values in latest-target cells* (#1346)
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), for the ramp keys (its D3)
- *Carry an optional per-edit ramp length on live session edits* (#1394), for the per-edit ramp
- *State a bounded tail and an exact-rest bound for every node* (#1329), which replaces the
  live-input-lane `Infinite` rule
- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328), on which #1329's
  input-section bound rests

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
