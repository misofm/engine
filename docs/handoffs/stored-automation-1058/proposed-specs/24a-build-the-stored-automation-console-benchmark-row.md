# Build the stored-automation console benchmark row

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A4, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch Q.

## Product outcome

The native console benchmark gains one descriptive row that measures what stored automation costs
on the path a mobile app runs: the sixty-four-track console compiled and rendered through the C
ABI, with stored automation on every track's faders, pans, console EQ gain and compressor
threshold. Its three arms (no automation, flat automation, moving automation) alternate per
observation, and its two deltas are named for what they measure: holding automated lanes, and
moving them. This slice builds and freezes the row, its preflight and its validators, and times
nothing; draft 24b *Record the stored-automation console benchmark baseline* runs it once. The row
is descriptive: no threshold, no tuning.

## Context

- **The model row.** `console_mixing_automation` (issue #1003; module doc
  `tools/bench/src/console.rs:138-158`) rides eight live controls on the mono console: three arms
  alternated per observation, the push outside the clock and the render call alone inside it
  (`MixingAutomationMeasurement::run_for`, `:2137-2215`), an untimed preflight asserted before any
  number (`:2141-2142`; alone through `bench console --preflight`, `:268-273`, `:364-396`), in-run
  class-A assertions (`:2174-2184`), and a record (`:2263-2366`) pinned by
  `scripts/console-benchmark-record-validator.jq` and its library
  (`scripts/console-benchmark-record-lib.jq:47`). Its table and premises live in
  `tools/console-workload/src/mixing_automation.rs` so that every driver reads one definition
  (`:1-35`), tested in `tools/console-workload/tests/automation.rs`. The record's unit test is
  `the_mixing_automation_row_prints_its_controls_and_the_validator_pins_them` (`console.rs:2525`).
  `OBSERVATIONS` is 1,000 (`:217`).
- **What it measured.** Its recorded paired ramp delta is about 205-228 ns per ramping control and
  block (Simd8, 48 kHz, `q = 128`; `artifacts/steps/bus-send-base/console-benchmark.accepted.jsonl:31`,
  `:62`). It pushed edits outside the clock, so it never timed an EQ design (note "Costs").
- **The session rows do not run a host's preparation.** `console-workload` calls the four
  compilers directly and builds a `PreparedRenderPlan` (`tools/console-workload/src/lib.rs:1-35`,
  `:40-55`); it mirrors host-core but never calls it, and `console_model` empties the fixture's
  automation (`console.rs:122-124`). Stored automation compiles in host-core preparation (A1.1), so
  a session row would not time it. Owner ruling R9 keeps only real host paths
  (`docs/rulings/engine-footprint-2026-09-28.md:35`; `scripts/operator/README.md:36-44`).
- **The C ABI path in a tool.** `tools/audit/src/capi.rs` already drives
  `miso_engine_v1_compile_session`, `miso_engine_v1_source_submit_planar_f32` and
  `miso_engine_v1_render_f32_planar` through the `capi` rlib (`:1-60`, render at `:494`). `capi`
  is an `rlib` (`crates/capi/Cargo.toml:9-11`); `bench` does not depend on it today
  (`tools/bench/Cargo.toml`).
- **The standing console fixture.** `fixtures/session/v1/console-sixty-four-track-intended.json`:
  64 tracks, 48 kHz, quantum 128; console slots `eq` and `comp` (`pre_insert`) and `limiter`
  (`post_insert`); per track a fader (`left_db` -3.0, `right_db` -2.5), a pan (`left` 1.0,
  `right` 1.0), the EQ's `band-1-gain` (4) declared `left` -7.5 and `right` 6.5, and the
  compressor's `threshold` (1) declared `both` -6.0. Both parameters are `Block`, `PerLane`,
  smoothing 64 (`sdk/src/generated/catalog.ts`). The fixture carries no automation
  (`scripts/check-console-benchmark-fixture.sh:20`).
- **The runner and its validators.** `scripts/operator/run-console-benchmark.sh --step NAME`
  (header `:1-40`) refuses an existing record, a dirty tree, a host without AVX2 and a failed
  fixture check, builds `bench` in release, takes one warmup and two measured rounds, validates
  62 records with `scripts/console-benchmark-validator.jq` (count at `:12`, `:32`) and writes
  `artifacts/steps/NAME/`. `scripts/operator/preflight-console-benchmark.sh` runs everything that
  can fail without the workload (`records_required: 62`, `:86`; the row preflight at `:64`).
  `scripts/test-console-benchmark.sh` holds the validators' mutation tests (CI,
  `.github/workflows/qualification.yml:1036`).
- **AGENTS.md benchmark rules** (`AGENTS.md:182-189`): freeze the workload and the validator before
  timing; preflight arguments, schema, output persistence, exit semantics and overwrite refusal
  without the workload; one invocation, one warmup, two measured rounds; no tuning or retry; a
  post-workload tooling failure keeps the raw output and becomes a tooling issue.

## Decisions frozen for this slice

- **D1. The subject.** A new module `tools/console-workload/src/stored_automation.rs` holds the
  table, the three arms' documents and the premises, as `mixing_automation.rs` does. It derives
  each arm from the standing fixture's model (no new fixture file: the arms differ only in the
  automation table, and three near-copies of a 64-track file could drift), and writes canonical
  JSON with the engine's writer.
  - **Cells.** On every track: fader (5) `left` and `right`; pan (12) `left` and `right`; console
    `eq` `band-1-gain` (4) `left` and `right`; console `comp` `threshold` (1) `both`. Per track 8
    cells, 512 in all; one entry per declared lane, so each curve starts at that lane's own static
    value.
  - **`none`:** no entries. **`flat`:** each entry one `linear` segment over
    `[0, (PREROLL + OBSERVATIONS + 1) · 128)` from the lane's static value to the same value.
    **`moving`:** the same segments from the static value to a target: fader `-6` dB from it, pan
    `0.5`, EQ gain `-1.5` and `0.5` dB, threshold `-18` dB. Every value lies in its row's domain,
    so drafts 01, 02 and 03a admit all three documents.
  - The session's `control_smoothing` is `{ "kind": "default" }`, as in the fixture after #1054.
- **D2. The C ABI driver.** In `tools/bench/src/console.rs`, `StoredAutomationMeasurement`
  compiles each arm with `miso_engine_v1_compile_session` and the reference limits, submits the
  fixture's source PCM before every block (outside the clock), and times
  `miso_engine_v1_render_f32_planar` alone, one call per arm per observation, arms alternated.
  `bench` gains `capi` and `session-validator` as dependencies. The C ABI entry points are
  `unsafe extern "C"` (`crates/capi/src/ffi.rs:311`, `:426`, `:807`), so `tools/bench/src/console.rs`
  becomes an approved unsafe owner beside `tools/audit/src/capi.rs`, with the same
  justification: it calls the C ABI as a host does. Every arm hashes its output outside the clock.
- **D3. In-run statements.** Asserted before the record is printed: `flat`'s digest equals
  `none`'s (class A: a flat curve emits no event and renders the static value, A1.4 "change
  only"); `moving`'s differs from `flat`'s; zero allocations, locks and syscalls inside the clock
  (`engine::realtime::audit`); no render error. A failure prints no record.
- **D4. Untimed preflight.** `bench console --preflight` also runs this row's premises: the three
  documents compile through the C ABI; each family (fader, pan, EQ gain, threshold) moving alone
  changes the digest against `none`; `flat` equals `none`. The operator preflight script calls it
  as today.
- **D5. The record.** `record: "console_stored_automation"`, `workload_kind:
  "sixty_four_track_console_stored_automation"`, the arms' p50/p95/p99, and two paired medians
  per observation: `paired_hold_delta_median_ns` (`flat - none`: holding 512 automated cells) and
  `paired_motion_delta_median_ns` (`moving - flat`: grid events, conversions, EQ designs, ramps
  and effect pieces), each also per cell; the cell count, the arms' digests, the audit counts, the
  backend and the statistical-method sentence. The record and aggregate validators, their library,
  and the runner's and preflight's record count (62 to 64) learn the row before any timing.
- **D6. No timing here.** This slice runs no benchmark. The workload and the validators are frozen
  when it merges; draft 24b runs them unchanged.

## Deliverables

1. D1 in `tools/console-workload`; D2-D5 in `tools/bench`; the validator, runner-count and
   preflight-count changes.
2. The tests below.

## Authorized paths

- `tools/console-workload/src/stored_automation.rs`, `tools/console-workload/src/lib.rs` (the
  module line)
- `tools/bench/src/console.rs`, `tools/bench/Cargo.toml`, `Cargo.lock`
- `scripts/console-benchmark-record-validator.jq`, `scripts/console-benchmark-validator.jq`,
  `scripts/console-benchmark-record-lib.jq`, `scripts/test-console-benchmark.sh`
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
  (the record count only)
- `scripts/check-conformance-boundaries.sh` (the pinned `tools/bench` dependency union,
  `:220-224`) and `scripts/test-conformance-boundaries.sh` (its fixture manifest, `:24-40`): `capi`
  and `session-validator` join, only
- `scripts/check-bench-policy.sh` (the approved unsafe owners under `tools/`, `:219-233`) and
  `scripts/check-realtime-policy.sh` (the unsafe ownership allowlist, `:29`):
  `tools/bench/src/console.rs` joins each, only, and their self-tests
  `scripts/test-bench-policy.sh` (the owner list of the `unsafe-owner-grep-error` case, `:499`)
  and `scripts/test-realtime-policy.sh` (its unsafe-owner fixtures), only where they list the
  owners; `docs/REALTIME_DEPENDENCY_POLICY.md` ("Unsafe-code
  ownership", the reason for that file)

## Non-goals

- A browser (V8) row for stored automation; the cross-target proof is slices 23a and 23b.
- Any change to the other rows, to the engine, or to `console-workload`'s plan path.
- The timed run and its record (draft 24b).
- Any optimisation of stored automation (weekly performance pass, after a measured miss).

## Hazards

- **C ABI overhead.** The driver times the exported render call, which includes the control
  plane's per-call checks; all three arms pay it, so the deltas exclude it. The absolute p50s are
  not comparable with the session rows', and the record says so.
- **Source feeding.** The C ABI renders silence on underrun; an arm that underruns would render
  cheaply. D2 submits before every call, and D3's digest inequality would catch a silent arm only
  for `moving`; the audit and an underrun counter of zero, read after the run, guard the rest.
- **Run length.** The moving segment must outlast the warmup round, the pre-roll and every timed
  observation; D1 sizes it from the constants, and the preflight checks that the last timed block
  is still inside it.
- **Record count.** Every runner, preflight and validator that counts records changes in this
  slice, before any timing (AGENTS.md); a stale count makes the runner refuse an honest run.

## Objective gates

1. **Premises** (`tools/bench/src/console.rs` test, new: `console-workload` depends on neither
   `capi` nor `session-validator`, and `bench` gains both). The three documents
   pass all five `session-validator` stages; every automated value equals its lane's static value
   at timeline 0; each family moving alone changes the C ABI digest.
2. **Record and validator** (`tools/bench/src/console.rs` test, new, on a short run as the mixing
   row's test does): the row prints a record the record validator accepts; edits that drop a
   field, swap two arms' digests or make `flat`'s digest differ from `none`'s are refused.
   `bash scripts/test-console-benchmark.sh` passes with new mutation cases for the row.
3. **Preflight.** `cargo build --locked --release -p bench`, then
   `scripts/operator/preflight-console-benchmark.sh --step <step>` passes with
   `records_required: 64` and launches no workload.
4. **No rendered bit moves elsewhere.** Every existing row's digest is unchanged (the run asserts
   its own pairs); no engine code changes.
5. **Commands:**
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `cargo build --locked --release -p bench`, then `./target/release/bench console --preflight`
     (untimed; it launches no timed workload)
   - `bash scripts/check-bench-preconditions.sh`, `bash scripts/check-console-benchmark-fixture.sh`,
     `bash scripts/test-console-benchmark.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a flat curve does not start at its lane's static value (the class-A
  statement would then be false by construction), or if one family's automation never reaches the
  plan.
- Gate 2 turns red if the record drops a field the validator pins, or if the validator accepts a
  record whose class-A statement fails.
- Gate 3 turns red if the runner's record count or a preflight step was not updated before timing.

## Dependencies

Batch Q. Direct dependencies:

- Draft 20 *Render stored parametric EQ automation*.

The rendering slices the row measures (09a-09b, 14a-14b, 18a-18b) land in batches R1 to R3, before
batch Q. Drafts 01, 02 and 03a (the documents must validate) and *Session `controlSmoothing`:
configurable ramp lengths for live mute, fader and pan changes* (#1054), for the document's
`control_smoothing` key, arrive through draft 20. Draft
24b *Record the stored-automation console benchmark baseline* follows it.
