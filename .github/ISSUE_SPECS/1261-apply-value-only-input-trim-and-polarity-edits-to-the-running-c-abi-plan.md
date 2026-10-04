# Apply value-only input trim and polarity edits to the running C ABI plan

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259), which classifies `trim_db` and
`polarity_invert` as live. Anchors verified on `main` at `54b0a1bf8`; re-verify them after #1257
lands.

## Product outcome

A C ABI transaction that changes a track's input `trim_db` or `polarity_invert` (left, right or
both lanes), alone or together with #1053's fader, mute and pan values, is a live update. Today it
rebuilds the plan.

## Context (verified at `54b0a1bf8`)

- **The model.** `DualMonoBuiltins { left, right }` of
  `ChannelBuiltins { polarity_invert, trim_db, hpf_hz, lpf_hz, delay_samples }`
  (`crates/session/src/model.rs:470-507`). `delay_samples` stays structural (decision 14, rule 1).
  `hpf_hz` and `lpf_hz` are #1262's.
- **The domain.** `trim_db` uses the private `checked_trim_gain` (`crates/builtins/src/lib.rs:4126`).
  Preparation calls it, and so does `BuiltinInputBank::set_trim_db` (`:3542-3557`).
- **What the kernel does.** A trim retarget keeps the lane's sign, and a polarity retarget keeps its
  magnitude (`:1480-1519`). Both ride one ramp on one coefficient.
- **The record.** `TrackInputRecord::TrimDb { lanes, db, smoothing_samples }` and
  `TrackInputRecord::PolarityInvert { lanes, inverted, smoothing_samples }`
  (`crates/builtins-compiler/src/lib.rs:175-211`).
  - The input stage is upstream of the fader and matrix seam.
  - A `Both` record preserves the channel-symmetry witness.
  - A `Left` or `Right` record desymmetrizes it, and retires the track's mono collapse for the life
    of the plan (`:213-232`). That costs performance, never correctness.
- **The drain.** The input drain is already bounded (`BuiltinBankProcessor::begin_block`, `:457-500`).
- **The browser's lowering.** `COMMAND_TRIM_DB` and `COMMAND_POLARITY_INVERT`
  (`hosts/host-web/src/lib.rs:4309-4345`).
- **The lanes and the tail.** The C ABI attaches no input lane before this slice (#1053 D5, #1254).
  Attaching it makes every strip's builtin tail `Infinite`
  (`crates/builtins-compiler/src/lib.rs:3435-3439`), as every browser plan with live controls
  already is. Any strip with an enabled input filter is already `Infinite` without it
  (`crates/builtins/src/lib.rs:3290-3302`).

## Decisions

- **D1. The lanes.** capi's preparation selects `strip_input: true` (#1254's `HostLiveLanes`).
  Every C ABI plan then reports `TAIL_INFINITE`. This is owner question Q4 of #1053; implement it
  only after the owner answers yes. If the owner says no, stop and record it: this slice cannot
  land without the input lane.
- **D2. The classifier.** `classify_live_delta` also masks each track's `builtins.left` and
  `builtins.right` `trim_db` and `polarity_invert`, and emits `TrackInputRecord`s on
  `LiveStripRecords`:
  - **TrimDb.** One per lane whose `checked_trim_gain` bits change, or one `Both` when both lanes
    change to the same `db` bits.
  - **PolarityInvert.** One per lane whose bool changes, or one `Both` when both change to the same
    value.
  - **Order.** TrimDb records first, then PolarityInvert records.
  - **Domain.** A trim that `checked_trim_gain` refuses gives `Domain`. `checked_trim_gain` becomes
    `pub`, as `checked_fader_gain` did in #1255.
  - **Ramps.** The records carry `LiveRamps::for_session(next).input_samples`, a new field that is
    0 until a session setting exists. #1054's table has no trim key; its coordination section
    already asks whether trim and polarity follow `faderMs` or get a key of their own.
- **D3. capi.** `commit_live` resolves each strip's `input` producer, checks its room with the
  fader and matrix queues, and pushes after them. The order of #1053 D6 holds.
- **D4. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists the new live values and records the
  tail change.

## Authorized paths

- `crates/host-core/src/live_delta.rs` and `crates/host-core/tests/live_delta.rs`.
- `crates/builtins/src/lib.rs`: `checked_trim_gain`'s visibility and documentation only.
- `crates/capi/src/runtime/compile.rs`, `control.rs` and `live_tests.rs`, and
  `crates/capi/tests/resource_lifecycle.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No input filters (#1262) and no `delay_samples`.
- No session setting for the trim ramp.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - each lane rule of D2;
   - trim and polarity changing together, in order;
   - a `delay_samples` change gives `Structure`;
   - a trim past either end of `[-144, 24]` gives `Domain`.

   *Test value: it turns red if a trim or polarity record goes to the wrong lane, uses `Left` and
   `Right` where `Both` was due (retiring the mono collapse for nothing), or lets a delay change
   through.*
2. **Equal to a rebuild.** New cases in `crates/capi/src/runtime/live_tests.rs`, on a session whose
   tracks have an empty console, no inserts and no input filter, so nothing downstream keeps state:
   - a trim of -6 dB on the left lane;
   - a polarity flip on both lanes.

   Run them for 1 and 10 tracks at the four launch rates. Each is a live update (no new epoch).
   From block E + ceil(`latency_samples` / quantum) + 1 on, the output is bit-identical to a plan
   compiled from the committed snapshot and fed the same source from sample 0.

   *Test value: it turns red if a live trim or polarity differs from what preparation bakes.*
3. **Equal to the browser's lane.** On the nine-track EQ fixture: the edit through the C ABI, and
   hand-built `TrackInputRecord`s (written in the test from the edit, never taken from the
   classifier) pushed into a host-core plan prepared with `HostLiveLanes::ALL` at the same block,
   render bit-identically.
   *Test value: it turns red if the C ABI path lowers a value differently from the browser's lane.*
4. **The tail and the resources.** The C ABI report now says `TAIL_INFINITE`. The resource oracles
   change only by the input rings: `cargo test --locked -p capi --test resource_lifecycle`.
5. **Nothing else changes.** #1257's gates 6 and 7. 4-lane (NEON) is CI-only here.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The owner's answer to Q4.
- The resource rows before and after.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257)
- *Qualify live C ABI edits against a concurrently rendering plan* (#1258); it edits the same test
  files and `commit_live`
- *Let host-core attach a strip's fader and matrix lanes without its input, effect or route lanes*
  (#1254)
- The owner's answer to #1053's Q4.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
