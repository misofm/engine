# Make the shared edge-ramp restore probe cheap enough for every pull request

Tooling follow-up to *Make every banked effect's state restore allocation-free* (#1278, closed by
PR #1299). Its delay follow-up verdict left MINOR-3 open: "The shared edge probe costs about 70 s
in debug per PR, because all four rates are probed and the delay's rings are large. A rate filter
belongs in the shared probe, which is outside this fix." Measurement shows the cost is the
delay's snapshot and restore codec, paid after every sample. This issue restores at a fixed,
reach-preserving set of samples per pull request and keeps the every-sample walk for the nightly.
No production code changes.

## Problem (verified on `main` at `d2fe0555a`)

**The probe.** `EffectDifferential::assert_edge_ramps_restore`
(`crates/conformance/src/randomized.rs:2675`) panics with every refusal that
`edge_ramp_restore_violations` (`:2681-2808`) returns. Its doc comment is `:2653-2664`. For each
quality row (`:2706`), each smoothed continuous parameter, and each domain edge (`:2731`), it:

1. finds a start a few hundred ulps inside the edge whose `f32` walk
   `current += (edge - start) / samples` leaves the domain (`overshooting_start`, `:2812-2836`);
2. prepares a scalar instance at that start and sends a `Point` to the edge;
3. renders the ramp one sample at a time (`for sample in 0..u64::from(samples)`, `:2763`), and
   after **every** sample snapshots the instance (`:2777`), prepares a fresh twin, and restores
   the snapshot into the twin. The first refusal is recorded and that edge stops (`break`,
   `:2800`).

**Callers.** Six `tests/randomized.rs` files call it, each in a test named
`the_effects_own_edge_ramp_snapshots_restore`: `crates/delay/tests/randomized.rs:24`,
`crates/compressor/tests/randomized.rs:153`, `crates/gate-expander/tests/randomized.rs:32`,
`crates/true-peak-limiter/tests/randomized.rs:35`, `crates/transient-shaper/tests/randomized.rs:32`,
`crates/multiband-compressor/tests/randomized.rs:76`. (Soft-clip's call was deleted by #1278
attempt 2, MAJOR-1: it had no unique catch. Do not re-add it.)

**Why the delay costs so much.** The delay has four quality rows (44.1, 48, 88.2 and 96 kHz,
`crates/delay/src/lib.rs:275-280`). Each lane's state is a two-second ring:
`ring_words = sample_rate * 2 + 3` (`:258`), so one lane payload is about 353 KB at 44.1 kHz and
768 KB at 96 kHz. All five parameters are probed: delay time over 128 samples
(`TRANSITION_SAMPLES`, `:65`), the other four over 64 (`RAMP_SAMPLES`, `:64`). That is
2 x 128 + 8 x 64 = 768 snapshot-and-restore pairs per rate, 3_072 in all. Each pair encodes and
decodes the whole ring word by word, which is slow in debug.

**Measured once (this host, debug, `CARGO_TARGET_DIR=target/hk-probe`, build first with
`--no-run`, then the run alone).** `cargo test --locked -p <crate> --test randomized -- --exact
the_effects_own_edge_ramp_snapshots_restore`, libtest's `finished in`:

| Crate | Debug run |
| --- | --- |
| delay | 72.45 s |
| true-peak-limiter | 0.40 s |
| multiband-compressor | 0.22 s |
| compressor | 0.14 s |
| gate-expander | 0.06 s |
| transient-shaper | 0.03 s |

The delay's time split, from a temporary timer in an exported copy of the tree (not committed):
per rate, 768 pairs; snapshot 4.1 / 4.4 / 8.2 / 8.9 s and restore 6.8 / 7.4 / 13.6 / 14.8 s at
44.1 / 48 / 88.2 / 96 kHz. Preparing the twin costs 0.26-0.54 s per rate and rendering is
negligible. So reusing the twin would not help: the cost is the number of snapshot and restore
calls, and it scales with the ring, that is, with the rate.

**In CI.** Qualification run 37201085656 (PR #1299, head `e0e4e8a20`): in `test-debug-b`
("DSP crates debug tests", `.github/workflows/qualification.yml:621-651`), the delay's
`tests/randomized.rs` binary finished in 80.18 s, and libtest printed "has been running for over
60 seconds" for this test. The whole job took 6 min 8 s against a 15-minute timeout. The
`aarch64-debug` leg (`scripts/run-aarch64-tests.sh debug`, which runs every product crate)
finished the same binary in 113.30 s. In release the full probe takes 4.44 s (this host).

**The nightly already runs it.** `.github/workflows/nightly.yml:173-180` runs
`cargo test --locked --release --workspace --test randomized` with
`MISO_ENGINE_RANDOMIZED_SCALE: '100'`. That includes all six probe tests. The knob is
`dsp_reference::randomized::SCALE_VARIABLE` (`crates/dsp-reference/src/randomized.rs:25`);
`overridden()` (`:47`) is true when it or the seed variable is set. `conformance` already depends
on `dsp-reference` (`crates/conformance/src/randomized.rs:41`).

**Where refusals happen (measured once in the exported copy).** Under each recorded mutation, the
first refusal of every edge falls either at sample 0 or one sample before the probe's own model
walk first leaves the domain:

- Delay M18 (`crates/delay/tests/MUTATIONS.md:36`): 24 refusals (feedback, mix and cross feedback,
  both edges, four rates). The model's first out-of-domain step is 34 for the starts `0x3f733312`
  and `0x00000021` and 49 for `0x3f7fffa0`; the refusals are "after sample 33" and "after sample
  48".
- Gate, a moving `current` held to the strict domain (`crates/gate-expander/src/lib.rs:801`,
  `(resting && !parameter_value_valid(spec, current))` changed to
  `(!parameter_value_valid(spec, current))`): 32 refusals (threshold, ratio, range, hysteresis,
  both edges, four rates), all "after sample 33". The #1278 attempt-1 verdict found that only this
  test catches it.
- Limiter, the coefficient path budget cut from 64 to 4 ulps
  (`crates/true-peak-limiter/src/lib.rs:3962`, `let slack = 64.0 * f32::EPSILON;` changed to
  `4.0`): 4 refusals (ceiling toward -24 dB, four rates), all "after sample 0". Only this test
  catches it.

A prototype of D1 below, in the exported copy, gave byte-identical violation lists for all three
mutations, and the green delay probe took 9.65 s in debug.

## Decisions

- **D1. Per pull request, restore at a fixed position set; keep rendering every sample.** For a
  ramp of `n = smoothing_samples` samples (indices `0..n`) on quality row `r` (its index in
  `descriptor.qualities`), with `k` the first model step whose walk leaves the domain
  (`1 <= k < n`), the probe snapshots and restores after sample `s` only when `s` is in

  `P = {0, n - 2, n - 1} U {k - 2, k - 1, k} U { s : s mod 16 == (4 * r) mod 16 }`

  (members outside `0..n` are dropped). The probe still renders every sample, so the instance's
  state evolves exactly as today. Only the snapshot, the twin and the restore are skipped.
  - `0` reaches a refusal of the whole remaining path at the ramp's start (the limiter case).
  - `k - 2 ..= k` reaches the first sample whose `current` is past the edge, with one sample of
    slack on either side of the probe's own `f32` model (the delay and gate cases).
  - `n - 2` and `n - 1` reach the last moving sample and the snap to the target.
  - The stride of 16 samples in-flight positions the model does not predict (a coefficient ramp
    derived from the parameter). Its offset rotates by 4 per quality row, so the rows together
    sample every fourth position.
- **D2. `overshooting_start` returns `k` too.** Its walk already computes it. Change its return
  to `Option<(f32, u32)>` (start, first out-of-domain step). Its contract is otherwise unchanged.
- **D3. The nightly keeps the every-sample walk.** When
  `dsp_reference::randomized::overridden()` is true, `P` is every sample `0..n`, exactly as today.
  The nightly's `MISO_ENGINE_RANDOMIZED_SCALE=100` run sets it, in release, where the full delay
  probe takes about 4.4 s. No workflow file changes.
- **D4. One pure function owns `P`.** Put the rule in a private function beside
  `overshooting_start`, for example
  `fn edge_restore_positions(n: u32, k: u32, row: usize, every_sample: bool) -> Vec<u64>`
  (or an equivalent membership predicate). The probe reads
  `overridden()` once and passes it as `every_sample`, so the unit test of D5 never touches the
  process environment.
- **D5. One unit test pins the reach-critical positions.** `randomized.rs` has no test module
  today; add `#[cfg(test)] mod tests` at its end with
  `edge_restore_positions_keep_the_reach_critical_samples`: for `(n, k)` in
  `(64, 34)`, `(64, 49)`, `(128, 66)`, `(64, 1)` and `(64, 63)` and each `row` in `0..4`, the set
  contains `0`, `k - 1` (when `k >= 1`), `k`, `n - 2` and `n - 1`, every member is below `n`, and it
  has at most `n / 16 + 6` members; with `every_sample = true` it is exactly `0..n`.
  *Test value:* a later edit that narrows the window (for example to `k ..= k + 1`, or drops
  sample 0) keeps every effect test green today, because no effect is defective, but silently
  loses the M18, gate and limiter catches per pull request. This test turns red on that edit.
- **D6. Messages and the public surface are unchanged.** The violation text (quoted by
  `crates/delay/tests/MUTATIONS.md:36`), the two exported functions' signatures, and the six
  caller tests stay as they are. Rewrite the doc comment at `:2653-2664` to state D1 and D3
  instead of "after every rendered sample".

**Rejected: a rate filter (MINOR-3's suggestion).** Probing only 44.1 kHz per PR costs about
10.9 s for the delay (snapshot plus restore at that rate, measured above), no less than D1, and it
drops three quality rows. Ring sizes, damping and limiter coefficients depend on the rate, so a
rate-specific restore defect would wait for the nightly. D1 keeps every row.

**Rejected: reusing one twin.** Twin preparation is under 4% of the delay's cost (measured above).

**Rejected: a higher `opt-level` for the delay's debug tests.** A profile change is workspace
configuration and alters every debug test of the crate; it is out of this issue's scope.

## Authorized paths

- `crates/conformance/src/randomized.rs`: `edge_ramp_restore_violations`, `overshooting_start`,
  the new position function, the doc comment at `:2653-2664`, and the new test module of D5.
- This spec.

## Non-goals

- Production effect code, including the delay's payload codec and its debug speed.
- The six caller tests, `crates/delay/tests/MUTATIONS.md`, and any workflow or script.
- The seeded differentials (`run_effect_differential`) and their seed counts.
- Re-adding the probe to soft-clip.

## Hazards

- **Keep rendering every sample.** Skipping `effect.process` at an unlisted sample would change
  the state at every later position. Only the snapshot, the twin's preparation and the restore
  are skipped.
- **The model is the probe's own.** `k` comes from the probe's `f32` walk of the parameter, not
  from the effect. An effect whose carried ramp is a derived quantity can refuse at a position the
  model does not predict. Sample 0, the stride and the nightly's every-sample walk cover that.
  Gate 1 shows the three known catches are kept.
- **`overridden()` is also true while replaying one seed** (`MISO_ENGINE_RANDOMIZED_SEED`). The
  full walk is correct then too; it is only slower.
- **Timing is descriptive.** Build first with `--no-run`, then time one run. Do not tune the
  stride after measuring to pass the ceiling; if gate 2 fails at stride 16, stop and report.

## Objective gates

1. **Reach is not lost (PR evidence, not committed).** Apply each mutation below, then run the
   crate's probe twice: per-PR (no variable set) and full (`MISO_ENGINE_RANDOMIZED_SCALE=1`, which
   sets `overridden()` without adding seeds to this seedless test). Both runs are red, with the
   stated count of "its own snapshot is refused" lines, and the two violation lists are
   byte-identical. Revert and observe green.
   - Delay M18 as `crates/delay/tests/MUTATIONS.md:36` describes it (in `read_carried_ramp`,
     `crates/delay/src/lib.rs:1623`, `current_valid = parameter_value_valid(spec, read.current)`,
     and the `ramp_path_within` clause at `:1626` removed): 24 refusals.
   - Gate strict current (`crates/gate-expander/src/lib.rs:801`, as above): 32 refusals.
   - Limiter 4-ulp budget (`crates/true-peak-limiter/src/lib.rs:3962`, as above): 4 refusals.
   - D5's test is red when the window in the position function is changed to `k ..= k + 1`.
2. **Cost ceilings (measured once, recorded).**
   - Local debug, delay: `cargo test --locked -p delay --test randomized -- --exact
     the_effects_own_edge_ramp_snapshots_restore` reports `finished in` at most 15 s (the prototype
     measured 9.65 s; today 72.45 s).
   - CI, on the PR's qualification run: in `test-debug-b`, the delay's `tests/randomized.rs`
     binary reports `finished in` at most 15 s; on both debug legs (`test-debug-b`,
     `aarch64-debug`) no "has been running for over 60 seconds" line names this test.
   - The other five crates' probe tests each stay under 1 s locally in debug.
3. **Nothing else moved.**
   - `cargo test --locked -p conformance -p delay -p compressor -p gate-expander
     -p true-peak-limiter -p transient-shaper -p multiband-compressor` passes in debug.
   - `MISO_ENGINE_RANDOMIZED_SCALE=100 cargo test --locked --release -p delay -p compressor
     -p gate-expander -p true-peak-limiter -p transient-shaper -p multiband-compressor --test
     randomized` passes (the nightly's shape for these crates), and the delay's probe there takes
     the full walk (record its `finished in`).
4. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
   - `bash scripts/check-conformance-boundaries.sh`

*Test value.* One new test (D5): it names the defect above. The probe is rewritten but keeps its
catches, which gate 1 shows once against its full walk; no caller test changes.

## Evidence

- Gate 1's table: mutation, per-PR count, full count, identical or not, green after revert.
- Gate 2's numbers: local delay debug before and after; CI `test-debug-b` and `aarch64-debug`
  `finished in` for the delay's `tests/randomized.rs` binary before (80.18 s and 113.30 s, run
  37201085656) and after.
- Gate 3's release `finished in` for the delay's probe at scale 100.

## Dependencies

- None. #1278 (PR #1299) is on `main` at `d2fe0555a`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
