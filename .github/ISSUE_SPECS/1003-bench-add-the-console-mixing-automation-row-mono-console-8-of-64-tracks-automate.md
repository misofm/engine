# Bench: add the `console_mixing_automation` row (mono console, 8 of 64 tracks automated)

Source: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, verified in `VERIFY-AUTOMATION.md`. **The Amendments section at the end supersedes the body wherever they conflict.** Draft names map to issues: automation-5 = #1003, automation-1 = #1004, automation-2 = #1005, automation-3 = #1006, automation-4 = #1007.

**Decision recorded (root, 2026-09-27):** the browser arm is the shipped `host_web.wasm` under V8 (amendment A4), not a guest control export. The owner's standing rule is that only paths a real host reaches are benchmarked. Land this row first, so each automation fix below is measured on it.

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

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-AUTOMATION.md` (F2, F3, F6 and F7) and
`verify-automation-raw-timings.txt`. **These amendments supersede "The row, exactly" wherever they
conflict.**

As drafted, the row is **not a real host shape**, and its in-run assertion fails on the first run.

### A1. Traffic in the host's shapes

The draft pushes every control Left then Right through `push_parameter`. For the EQ that makes two
owner transactions and two one-channel targets. The product does not do that: `prepared-control.js`
→ `eq_target_prepare` sends a symmetric both-channel EQ edit as **one `Both` target**, which keeps
the mono collapse at base.

V8, mono console:

| written | row µs |
|---|---:|
| nothing | 148.8 |
| EQ only, all 64 tracks | 148.7 |
| compressor only | 248.0 |
| limiter only | 247.8 |

The row must use the host's lowering:

* **EQ:** one `push_parameter(channel, 3, ParameterChannel::Both, value)` per control per block.
  The owner accepts `(PerLane, Both)` (`effect-compiler/src/control.rs:224`) and emits one `Both`
  target, exactly as the SDK does.
* **Compressor and limiter:** Left then Right `Parameter` records with the same value. That is
  `into_effect_records`'s `(PerLane, 2)` arm.

### A2. Bases are the held values

`quiet == restated` is asserted in-run. With the drafted fixed bases (3 dB, −24 dB, −3 dB),
"restated" is a real edit, so its digest cannot equal `quiet`'s.

Each control's base is **its track's prepared value**, read from the compiled session model or
from `parameter_state`. On `sixty_four_track_console_mono`:

| track | effect | parameter | held value |
|---|---|---|---:|
| `ch00` | EQ | band-1 gain | −7.5 dB |
| `ch24` | EQ | band-1 gain | 1.5 dB |
| `ch48` | EQ | band-1 gain | −4.5 dB |
| `ch08` | compressor | threshold | −18 dB |
| `ch32` | compressor | threshold | −27 dB |
| `ch56` | compressor | threshold | −9 dB |
| `ch16` | limiter | ceiling | −1.0 dB |
| `ch40` | limiter | ceiling | −1.75 dB |

* The automated values alternate `base ± step`, clamped into the domain.
* "restated" restates exactly the held value: the #144 hoist and `LinearRamp::stationary_at`
  settle it, so no window opens.
* `quiet == restated` then states the collapse's own class-A claim, in-run.

### A3. Every automated effect must move bits (F6)

On this fixture, a limiter ceiling ridden ±0.25 dB around −3 dB never engaged in 150 blocks. Every
limiter arm had the same digest.

* **Preflight.** Run three single-effect variants of `automated`: EQ only, compressor only, limiter
  only. Assert that each digest differs from `restated`.
* If the limiter's does not, enlarge its step and record the change. On `ch00` a ceiling of −17 to
  −20 dB engages and −12 dB does not; check the chosen tracks the same way.
* A whole-mix `automated != restated` alone cannot show that the limiter's ramps ran.

### A4. The browser arm is the shipped artifact

The standing console benchmark's wasm arm is a wasmtime guest (`tools/wasm-console-guest`), not a
host path. The browser arm of this row is instead:

* `host_web.wasm`, built with the flags of `scripts/build-web-audioworklet.sh`;
* under Node's V8 (`--no-liftoff`), driven through `miso_engine_web_v1_command_submit` and
  `prepared-control.js`, exactly as the SDK does;
* timing only `miso_engine_web_v1_render`.

The diagnosis's `web_auto.mjs` (subject `mono_mix`) is the model. Fix its three hard-coded paths,
or apply `verify-automation-harness.patch`.

Do not add a control export to the guest for this row. That puts the row on a path no product
uses. Owner ruling requested (VERIFY-AUTOMATION §7.3).

### A5. What today's numbers read (replaces the draft's list)

* **V8 `host_web.wasm`:**
  * `quiet` ≈ 149 µs;
  * `restated` ≈ 246-248 µs, from the compressor and the limiter only;
  * `automated` ≈ 287-290 µs.
* **With `automation-1` (spans only):**
  * `restated` ≈ 148 µs;
  * `automated` ≈ 173 µs, and ≈ 163 µs with `automation-2` to `automation-4` as prototyped.
* **Native `Simd8`:** `quiet` ≈ 70 µs, `automated` ≈ 149 µs today.
* The C2 prototype does not reach collapsed banks, so the mono `automated` figure does not yet
  include any compressor saving.

### A6. Additional in-run assertions

* After the pre-roll, an EQ-only restated pass keeps every cohort collapsed
  (`bank_collapse_counters` equal to `quiet`'s). This pins the harness to the SDK's EQ shape; F2 is
  what happens if the harness drifts.
* Every push accepted, as drafted.
