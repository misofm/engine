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

## Attempt 1 evidence

Terra, attempt 1. Code `d3349b72`; this section and the step records are the commit after it.
Nothing pushed. Worktree `engine-1003`, branch `codex/1003-mixing-automation-row`, from
`codex/batch-plumbing-floor-2` at `1010d50c`.

### The row as built

* **Definition, once.** `tools/console-workload/src/mixing_automation.rs` holds the workload, the
  eight controls, their host lowerings, the arms, the pre-roll and the preflight. The native row,
  `tests/automation.rs` and the browser arm all read it. The browser arm reads it through
  `cargo run --example mixing_automation_controls`, so it never transcribes it.
* **Session.** `sixty_four_track_console_mono` as written, with `PlanConfig { meters: false,
  control: true, observation: Absent }`, at `Backend::current()` (`Simd8` here).
* **Controls,** resolved by `SessionRuntime::control_channel(track_id, contract id)`. Each
  parameter is checked against its descriptor name (`band-1-gain`, `threshold`, `ceiling`) and
  its 64-sample window:

  | track | effect | lowering | held base | automated values (even / odd block) |
  |---|---|---|---:|---|
  | `ch00`, `ch24`, `ch48` | EQ band-1 gain | one owner edit on `Both` | −7.5 / 1.5 / −4.5 dB | base ± 0.25 |
  | `ch08`, `ch32`, `ch56` | compressor threshold | Left then Right record | −18 / −27 / −9 dB | base ± 0.5 |
  | `ch16`, `ch40` | limiter ceiling | Left then Right record | −1.0 / −1.75 dB | 0 / base − 8 |

  * Bases are read from the session model, never written into the row (A2).
  * `automated` clamps `base ± step` into the descriptor domain.
  * One block is 3 owner edits and 10 parameter records, 13 pushes in all. The draft's
    `automation_spans_per_block: 16` no longer describes the traffic after A1, so the record
    states `owner_edits_per_block` and `parameter_records_per_block` instead.
* **Limiter step, A3.** The step was chosen before timing, from the ladder 0.25, 1, 2, 4, 8, 12,
  16, 20 dB. It is the smallest step at which the limiter-only arm moves bits in both arms.
  * `ch40` moves from step 8 (a −9.75 dB ceiling), natively and under V8.
  * `ch16` never engages natively anywhere in the ceiling's [−24, 0] dB domain, even at −24 dB.
    Its compressor (−30 dB threshold, ratio 6.75) holds it below −24 dBFS. Under V8 it engages
    only at −24 dB.
  * So `ch16`'s ride is priced but moves no bit. The premise is per effect, as A3 states it, and
    is not per track. `ch00`, for reference, moves natively from step 12.
* **Arms,** alternated per observation: `quiet`, `restated`, `automated`.
  * Every arm except `quiet` settles its controls at base, then all three render 64 untimed
    blocks.
  * Pushes happen off the clock. `timing::timed` holds `render` alone.
* **Record `console_mixing_automation`,** emitted last in each round. It carries:
  * p50, p95 and p99 per arm;
  * `paired_ramp_delta_median_ns` (automated − restated) and `paired_collapse_delta_median_ns`
    (restated − quiet);
  * each arm's `bank_collapse_counters`, and the controls with their lowerings;
  * the preflight's seven digests and seven counter pairs;
  * the usual metadata, with `descriptive_only: true`.

### In-run assertions, and their red evidence

The preflight (`mixing_automation::preflight`) runs untimed before every timed run and alone as
`bench console --preflight`. It covers seven arms over 64 pre-roll plus 64 compared blocks: the
row's three arms, `automated` restricted to each single effect, and an EQ-only `restated`. It
asserts that:

* every push is accepted;
* `quiet` collapses every cohort on every block;
* `quiet == restated`, and `restated != automated`;
* each single-effect `automated` differs from `restated` (A3);
* the EQ-only `restated` has `quiet`'s collapse counters (A6).

The timed run then re-asserts `quiet == restated`, `restated != automated`, and every push
accepted.

Red evidence. Each mutation was applied, run, and reverted.

| mutation | what went red |
|---|---|
| M1: restate `base + 0.0625` | `restating the held values moved a rendered bit`, on block 0 of the test and in the preflight |
| M2: EQ lowered Left then Right | the preflight's A6 (`restating the EQ in the SDK's Both shape retired a cohort's collapse`), plus the id, lowering and tally tests |
| M3: limiter step 0.25 dB | `automating the limiter alone moved no rendered bit` |
| M4: `automated` pushes the base | `the automated arm rendered the restated arm's bits` |
| M5: control queue depth 1 | `the settling write was refused` |
| M6: `ch40` renamed to `ch99` | `bench console --preflight` exits 101 with `MissingChannel { track_id: "ch99", … }` |
| browser arm: restate `base + 0.0625` | `preflight: restating the held values moved a rendered bit` (V8) |
| browser arm: `automated` pushes the base | `preflight: the automated arm rendered the restated arm's bits` (V8) |
| browser arm: limiter ladder steps 0.25-4 | `automating the limiter alone moved no rendered bit` |

`the_eq_lowering_keeps_the_collapse_and_the_harness_only_lowering_would_not` pins F2 directly:

* a `Both` EQ restatement keeps all 8 cohorts collapsed;
* two one-channel owner edits retire exactly the 3 EQ cohorts.

### Gates

1. **Validators.** `console-benchmark-record-lib.jq` adds `mixing_automation_record_valid`. It
   pins:
   * the exact key set, and the mono session's six facts;
   * the arms and the pre-roll;
   * each control's track, slot, effect, parameter, index, lowering and step, with values that
     straddle the base;
   * `owner_edits_per_block` and `parameter_records_per_block`, which are recomputed from the
     lowerings;
   * pushes equal to observations × 13;
   * `quiet`'s counters equal to cohorts × (pre-roll + observations), and no arm collapsing more
     than `quiet`;
   * both digest rules;
   * the preflight's seven digests, the A3 inequalities, and A6's counter equality.

   The aggregate goes from 48 to 50 records. It requires both rounds of the row to agree on the
   automated digest, the preflight, and every collapse counter. `test-console-benchmark.sh`
   passes with 50 new record mutations and 5 new aggregate mutations, all red, plus one accepted
   future record (restatements that keep the collapse).

   Twelve validator rules were each weakened on a scratch copy. For each, its targeted mutation was
   then accepted, and it is refused by the intact rules. A record missing any collapse counter is
   refused by the key-set rule and by the per-key sweep.
2. **Preflight.** `bench console --preflight` is described above. It refuses a missing channel
   (M6) and is wired into `scripts/operator/preflight-console-benchmark.sh`.
3. **`tools/console-workload/tests/automation.rs`** gains five tests:
   * the eight controls resolve by id to the held bases, one per eight-lane bank, 13 pushes a
     block;
   * a `control: false` plan refuses the row;
   * restating is bit-identical on every block;
   * the preflight premises hold;
   * the EQ-lowering test above.
4. **Descriptive only.** No threshold, and one timed invocation per arm with no retry.

### Browser arm

The browser arm is `scripts/run-web-mixing-automation-benchmark.sh` (`prepare`, `preflight`,
`run`) together with `scripts/web-mixing-automation-benchmark.mjs`.

* **Module.** `prepare` builds `host_web.wasm` with the flags of `build-web-audioworklet.sh`.
  That script refuses to release an artifact whose digest does not match the pin, and this batch
  branch has not repinned. The module is digest `2309b243…` against pin `8934cdd9…`. The record
  states `module_matches_pin: false`. Two clean builds gave the same digest.
* **Boot.** The mono fixture, with its source stretched to 48,000,000 frames. The SDK-default
  command queue of 64 records and its default source ring; no meters, no observation.
* **Traffic.** Each block's 8 `channel = 2` records go as one `submitSync` through
  `prepared-control.js`, which uses the same callbacks as `sdk/src/core/boundary.ts`. The harness
  checks every base against the booted document and every track index against
  `console_track_id`. Only `miso_engine_web_v1_render` is timed.
* **Assertions.** The seven-arm preflight and the in-run digest rules, as natively. The runner
  re-checks them with `jq` before it writes a record.
* **Collapse counters** are not exported by `host_web.wasm`, so under V8 A6 is visible in the
  timings only.

### Today's numbers (descriptive, uncontrolled host)

Native run: `scripts/run-console-benchmark.sh --step after-1003` with
`MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, pinned to CPU 31.

* The lock was held 21:26:24-21:27:44. The runner included a 60 s binary cooldown, because its
  own build step relinked the binary.
* Load average was 5.81 at the start of the hold and 6.68 at the end.
* Result: PASS, 50 records.

The browser arm ran once under the lock, 21:30:14-21:30:17, pinned to CPU 31, with load average
9.52 at the start and 9.48 at the end. It used Node 22.23.2 (V8 12.4.254.21) with `--no-liftoff`.

µs per 64-track block; the native column gives rounds 1 / 2:

| arm | native `Simd8` p50 | p95 | p99 | V8 p50 | p95 | p99 |
|---|---:|---:|---:|---:|---:|---:|
| `quiet` | 76.3 / 74.1 | 82.0 / 80.0 | 84.7 / 81.8 | 154.3 | 225.6 | 233.2 |
| `restated` | 99.6 / 99.4 | 105.5 / 105.6 | 109.3 / 108.8 | 189.2 | 277.6 | 285.3 |
| `automated` | 126.3 / 126.5 | 132.7 / 133.2 | 135.0 / 137.4 | 216.6 | 318.6 | 325.9 |
| paired collapse delta (restated − quiet) | 23.1 / 25.1 | | | 34.9 | | |
| paired ramp delta (automated − restated) | 26.7 / 27.0 | | | 27.6 | | |

* **Collapse counters (native).**
  * `quiet` holds [8512, 8]: 8 cohorts × 1064 blocks.
  * `restated` and `automated` hold [3192, 8]: only the 3 EQ cohorts stay collapsed. The
    compressor and limiter records retire the other 5 from the settling write on.
  * In the preflight, the EQ-only `restated` holds [1024, 8], equal to `quiet`.
* **Why these read below the spec's A5 figures** (V8 `restated` ≈ 246-248 µs, `automated` ≈
  287-290 µs; native `automated` ≈ 149 µs). A5 came from the diagnosis harness, which settled
  every one of the 64 tracks, and whose native EQ went Left + Right. This row writes only its
  eight controls, as the body specifies, with the EQ as `Both`.
  * Natively, that retires 5 of 8 cohorts.
  * Under V8's four-lane banks, it retires 5 of 16. The measured V8 collapse delta of 34.9 µs is
    about 5/16 of the diagnosis's full-console 99 µs.
* **`quiet` is not the `sixty_four_track_console_mono` session row** (65.7 / 65.5 µs in the same
  run). It carries the control channel and is alternated with two other plans.
* **Browser tails** (p95, p99) are wide at load 9.5. Read the p50s and the paired deltas.

### Standing rows

* No standing workload, fixture or digest moved. All 48 standing records' digests in
  `after-1003` equal `after-971`'s, and no engine code changed between the two.
* The runner's record count (48 → 50), the aggregate validator, and the operator preflight's
  `records_required` were updated because the new row adds one record per round.
* The mutation suite's aggregate index map shifted by one per round, because round two now
  starts at 25.
* `WORKLOADS`, `native_session_rows` and the wasm console guest are untouched. The guest still
  checks for `wasm32`.

### Checks run

* `cargo fmt --all -- --check`.
* `cargo clippy --locked --workspace --all-targets -- -D warnings`.
* `cargo test -p console-workload -p bench`: all pass.
* `scripts/test-console-benchmark.sh`: PASS.
* `scripts/check-env-vocabulary.sh`: ok. No new environment names; the browser runner reuses
  `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED`.
* `check-workspace-policy`, `check-bench-policy`, `check-realtime-policy`, `check-lane-policy`,
  `check-session-policy` and `check-effect-runtime-policy`: ok.
* `cargo check -p wasm-console-guest --target wasm32-unknown-unknown`: ok.

### For the verifier

* **`ch16`'s limiter ride moves no bit natively.** It is still timed. Option: move the bank-2
  limiter to a track that engages. That would change the spec's track table, so it needs an owner
  call.
* **The browser record has no hermetic mutation suite.** Its gates are the harness's in-run
  asserts and the runner's `jq` recheck. The nearest-rank percentile is re-implemented in JS; the
  bench policy's one-percentile rule scans Rust under `tools/` only.
* **The browser module is not the pinned artifact on this branch** (`module_matches_pin:
  false`). A run after the batch repins would prove the recipe equals delivery.
* **Under V8, A6 cannot be observed through the shipped ABI.**
* **Both arms ran with the host uncontrolled.**
