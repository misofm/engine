# Bench: add the `console_mixing_automation` row (mono console, 8 of 64 tracks automated)

Automation follow-up, measurement only (research 2026-09-27, base `codex/batch-plumbing-floor-2` at
`49f696c7`). Evidence: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, sections 2
and 6.

## Product outcome

Mixing sessions ride faders, EQ gains and frequencies, and compressor thresholds all the time.
The standing console table cannot see what that costs:

* `console_model` clears every fixture's automation.
* The one automated row, `console_automation` (`tools/bench/src/console.rs:1624`), rides one
  compressor threshold on one track of the compressor-only row, natively.
* `console_hoist` drives bare EQ banks, outside a plan.

So three effects the diagnosis measured are invisible:

* the EQ's whole-bank ramping path (7.8x a settled bank);
* the loss of the mono collapse on the first write (+55-61 % of the mono console);
* the fact that tracks spread across banks each take a bank down.

This slice adds one row that sees all three, in the product's common shape.

## The row, exactly

* **Name.** `record: "console_mixing_automation"`,
  `workload_kind: "sixty_four_track_console_mono_mixing_automation"`.
* **Session.** `Workload::SixtyFourTrackConsoleMono`, the mono fixture as written, prepared with
  `PlanConfig { meters: false, control: true, observation: Absent }`, at the run's native backend.
* **Automated controls.** 8 of 64 tracks, one per `Simd8` cohort and every other `Simd4` cohort:

  | tracks | effect | parameter (descriptor index) | base | step |
  |---|---|---|---:|---:|
  | `ch00`, `ch24`, `ch48` | `miso.parametric-eq` | band-1 gain (3) | 3.0 dB | 0.25 dB |
  | `ch08`, `ch32`, `ch56` | `miso.compressor` | threshold (0) | −24.0 dB | 0.5 dB |
  | `ch16`, `ch40` | `miso.true-peak-limiter` | ceiling (0) | −3.0 dB | 0.25 dB |

  Channels are found with a stable `(track_id, effect_id)` lookup, never by position.
* **Traffic.** Before block `n`, each control is pushed on **both** channels through
  `SessionRuntime::push_parameter`: Left, then Right, with the same value. That is exactly what the
  web host's `channel = 2` becomes. The value is `base + step` for even `n` and `base - step` for
  odd `n`, so every block opens a 64-sample window. The push is off the clock.
* **Pre-roll.** Push every control at its base once, then render 64 untimed blocks. Every arm
  does this except `quiet`, which never writes. The difference between `quiet` and `restated` is
  the point of the row.
* **Arms**, alternated per observation, as `console_automation` does:
  * `quiet`: no write, ever;
  * `restated`: the eight controls restated at base every block. The #144 hoist settles them, so
    no window opens;
  * `automated`: the traffic above.
* **Record.**
  * p50, p95 and p99 per arm;
  * `paired_ramp_delta_median_ns` (automated − restated);
  * `paired_collapse_delta_median_ns` (restated − quiet);
  * `bank_collapse_counters` read once per arm after the run;
  * the automated `(track_id, effect_id, parameter)` list;
  * `automation_spans_per_block: 16`, `smoothing_samples: 64`;
  * the usual metadata, `descriptive_only: true`.
* **Asserted in-run:**
  * `quiet == restated` digests. Restating is a no-op, and the collapse is bit-exact.
  * `restated != automated` digests.
  * Every push accepted.
* **Where it runs.** The native bench (`Simd8`), and the wasm console guest (`simd128`).
  `miso_console_prepare` needs a row index, so the row goes after the existing ones, as
  `DRIVER_FED_WORKLOADS` did. Record the wasm arm's pushes through the guest's control export, or
  if the guest has none, add one, recorded here.

## What today's numbers will read

From the diagnosis, in µs per block at `Simd8` native / V8:

* `quiet` about 69 / 151;
* `restated` about 108 / 243. This is the collapse loss; it drops to about `quiet` once
  `automation-1` lands;
* `automated` about 148 / 285, falling to about 81 / 169 with `automation-1` to `automation-4`.

## Objective gates

1. The validator (`scripts/console-benchmark-record-lib.jq` and the aggregate check) pins the new
   record's schema, kind, arms and in-run assertions. A record missing the collapse counters is
   refused.
2. A preflight (`--preflight`, no timing) builds the three arms, runs the pre-roll and asserts the
   eight control channels resolve. It refuses on any missing channel.
3. `tools/console-workload/tests/automation.rs` gains the row's premises: restating is
   bit-identical, moving moves bits, and the eight channels resolve by id.
4. Descriptive only. No threshold, and no tuning after the first timed run (AGENTS.md benchmark
   rules).

## Non-goals

* A stereo twin of the row. The stereo console cannot collapse, so its ramping costs are already
  a subset of this row's `automated − restated`. Add one only if a regression hides there.
* Fader or pan automation. They are seam-side and gate nothing, and their cost is not this
  diagnosis's subject.

## Dependencies

None. It should land **before** `automation-1` to `automation-4`, so that their effect shows in
the record stream.
