# #1278 attempt 1 verdict: FAIL

One MAJOR, the test-value rule: soft-clip's new edge-probe test catches nothing that existing tests
miss, and the attempt record says so. The fix is to delete that one test, or to name and demonstrate a
defect that only it catches. The product work is correct. Every banked effect's payload calls are
allocation-free on the render thread, and a lane restored mid-ramp continues bit for bit, including
at quantum 32. I re-ran and checked both claims. Five MINORs, one of them a hostile-payload regression
in the compressor, should be fixed in the same pass.

- Commits reviewed: `git diff 983ac85bd 46ef4f263`: `c2808bb04` (attempt 1) and `46ef4f263` (its
  soft-clip amendment). `d6217a79d` (#1071 minors) is reviewed separately at the end.
- Exported to `/tmp/claude-1002/v1278/attempt1/` and checked byte-identical to `46ef4f263` after
  every mutation was reverted. Logs and probes are in `/tmp/claude-1002/v1278/attempt1/scratch/`,
  `/tmp/claude-1002/v1278/*.log`, and the source copies `/tmp/claude-1002/v1278/rtcopy`,
  `/tmp/claude-1002/v1278/sccopy`. Their `target/` directories were deleted afterwards.

## Gates I re-ran (all green)

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (72 regions, 23 files) / ok |
| `cargo test --locked --workspace` (debug) | ok, 308 test binaries, 0 failed. Includes the slice's eight crates and `effect-compiler`'s `launch_native_state_layouts_are_v1` |
| `cargo run --locked -p conformance --example conformance_fixtures -- --check` | ok, nothing re-pinned |
| `cargo build --release -p audit -p capi && audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls (100k calls) |
| `check-capi-abi.sh` | ok (shared and static) |
| `cargo build --release -p bench`, `trace-effect-contract-audit.sh target/release/bench 1000000` | ok (1M blocks) |
| `check-effect-contract.sh target/release/bench` | ok (8 production factories) |
| `cargo test --locked --release -p console-workload` | ok, console digests unchanged |
| `check-cross-targets.sh` | PASS. Only the expected #1018 iOS `memset_pattern16` rows fail |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all ok. **ARTIFACT CHANGED** confirmed: the module's sha256 is `0f508f8ca5b3849d3d43e65e058dfbd32732fbc9ec3405f64ffedfb31d685c53`, the same as the implementer's build. The pin `6c952a2c...` is untouched, as `docs/RELEASE.md` requires |

## Width coverage

- On this AVX2 host the differential binds **both** widths only for the limiter (8 Four and 8 Eight
  banks) and the multiband (12 and 12).
- The compressor, transient shaper, gate, soft-clip and EQ decline `Four` on an AVX2 build, by
  their existing design (for example, the compressor declined it 32 times). Their payload calls run
  at `Eight` here and at four lanes only in CI's `aarch64-debug`/`aarch64-release` legs
  (`run-aarch64-tests.sh`).
- The payload code is one generic body over `L: Lane`. Its 4-lane instances for those five effects
  are **not verified locally**. The worklet build compiles and gates their `f32x4` kernels.

## Does the oracle really reach mid-ramp at quantum 32?

**Yes for five of six effects; weakly for the multiband.**

- **Mid-ramp counts.** I instrumented `Continuation::take` (export only, then reverted) to decode
  each effect's ramp `remaining` words. Mid-ramp continuations per default run, with those at
  quantum 32 in brackets:
  - compressor 23 [4]
  - transient shaper 21 [7]
  - limiter 27 [3]
  - soft-clip 8 [2]
  - gate 19 [4]
  - multiband 3 [0]
- **The oracle catches the old crates.** I built `983ac85bd`'s compressor and transient-shaper crates
  against the new harness, with the harness forced to quantum 32 and banks only. Both are red in the
  `run_width` continuation:
  - compressor: "the instance restored from its snapshot rendered 0xbf51df38 where the lane,
    continuing, rendered 0xbf51df3e";
  - transient shaper: red the same way.
  - The unforced harness also catches the old compressor, through `run_scalar`'s twin.
- **The new tree passes at quantum 32.** With the harness forced to quantum 32, all seven banked
  effects are green.

## Allocation-free proof (gate 1)

- A `vec![0.0; 4]` inside the limiter's `read_lane` turns the reinstated limiter differential red
  ("restore_state_payload: ... allocations: 2, deallocations: 2"). It also turns
  `check-realtime-policy.sh` red (marked forbidden body).
- The limiter's restore now validates in place and commits from the bytes. `LaneRestore` holds only
  scalars. This is correct.

## Judgement of the implementer's deviations

1. **`state_layout_version` stays 1. Accepted.**
   - AGENTS.md outranks the spec: a genuine version's prelaunch identity is V1.
   - `launch_native_state_layouts_are_v1` pins it on purpose.
   - Nothing persists a payload (R6b; the only restore callers outside the effects are test
     doubles).
   - The exact section lengths refuse an old 22-word or 11-word section with `effect.state.length`
     (tested, for example `malformed_raw_payloads_reject_transactionally_...`).
   - Two documents now contradict this: the slice spec's D2a ("bump") and
     `docs/EFFECT_CONTRACT_V1.md:186-190` (layout change "travels with a `state_layout_version` bump
     (decision W2-D2)"). Sol should amend both. See MINOR-5.
2. **The 64-ulp path check (`ramp_path_within`).**
   - For the parameter ramps of the compressor, transient shaper, gate and multiband it is
     meaningful:
     - the target and every settled current stay strictly in the domain;
     - a moving ramp's whole remaining path stays within `64 * eps * max(|min|, |max|)`;
     - the step must be finite, and a settled ramp's step must be `+0.0`;
     - `remaining` is bounded, so the walk is bounded.
   - **Two exceptions:**
     - the compressor's new coefficient-ramp check admits unstable negative coefficients (MINOR-1);
     - the limiter does not use the path check at all (MINOR-2).
3. **The delay is not a banked effect. No scope gap.**
   - `DelayFactory::bind_homogeneous_bank` always returns `Ok(None)`, and its differential runs with
     `banks_natively: false`.
   - The spec's Context excludes it, and slice 12 moves delay instances rather than restoring them.
   - I confirmed the disclosed defect: the probe reports 24 refusals of the delay's own edge-ramp
     snapshots (feedback ±0.95, mix 0 and 1, cross feedback 0 and 1, at every rate). A successor
     should fix them before anything restores a delay.
4. **Soft-clip MINOR-2 is partly fixed. Acceptable.**
   - #1278's objective gates are met.
   - The two open cases need either an input of magnitude at least about 2.7e36 at +36 dB on the
     identity path, or a non-finite input, which D7 recovers a block later. No real signal reaches
     them.
   - Accepting `±inf` in `X` does not weaken the hostile gate:
     - every infinity it admits is one the effect produces itself from finite input;
     - `NaN` stays refused in every history;
     - `inf` stays refused in `e` and in the dry history (rejection rows `bad(12, NaN)` and
       `bad(43, inf)`);
     - a crafted `+inf`/`-inf` pair in `X` reproduces only state the effect itself reaches (open
       case 1) and is bounded by the 31-sample window.
   - The successor item and its reproducer are adequate.
5. **Realtime floors not raised.** NIT-1. The script's own rule asks for it, but the script is
   outside the slice's paths.
6. **Payload growth is within D2a.**
   - Compressor 22 to 37 words: 7 steps plus 2 coefficient ramps of 4 words.
   - Transient shaper 11 to 14 words: 3 steps.
   - The coefficient ramps are words a continuation reads, and they cannot be re-derived exactly
     (their `current` is an iterated sum).
   - The limiter restore validates in place (verified above).
7. **ARTIFACT CHANGED.** Confirmed and correctly handled; the pin is not touched.

## Findings

### MAJOR-1: `crates/soft-clip/tests/randomized.rs:244-259` (`the_effects_own_edge_ramp_snapshots_restore`) has no defect that only it catches

AGENTS.md says "there is no PASS without it", and the verifier rules say "a test with no answer is a
FAIL finding".

The attempt record itself concedes: "No mutation was found that only the probe catches for
soft-clip". I tried to find one and could not:

- **Tolerance sweep.** I scaled the line tolerance in `ramp_current_valid`
  (`crates/soft-clip/src/lib.rs:318`, `RAMP_SAMPLES` to `k`):
  - k = 16: both tests green;
  - k = 8: **only the seeded `a_restored_near_edge_ramp_continues_bit_for_bit` is red**, and the
    probe is green;
  - k = 6, 5, 4, 1: both red.
- **Strict-current and rest-guard mutations.** These are the implementer's; both tests turn red.
- **Extra quality rows.** The probe's only structural extra is its four rate rows. Soft-clip's
  restore has no rate dependence: nothing in its decode reads `sample_rate`.

So the seeded test (256 seeds, which asserts that every one of the six edges is crossed) dominates
the probe for this crate. "Uniform contract every banked effect runs" is not a defect.

**Fix.** Delete this one invocation. Keep `EffectDifferential::assert_edge_ramps_restore` and its
other five calls, each of which has a unique catch (see the test-value table). Or name a defect only
it catches and record its red run.

### MINOR-1: `crates/compressor/src/state.rs:134-143` accepts a negative attack or release coefficient, which is unstable, and the comment that justifies the slack is false

- **The check.** `ramp_path_within(ramp, (0.0, 1.0), 64.0 * f32::EPSILON, ...)` accepts coefficients
  in `[-7.6e-6, 1 + 7.6e-6]`.
- **Why that is unsafe.** The smoother is `y += c (x - y)`, whose pole is `1 - c`, so any `c < 0`
  makes it diverge. The comment at `:136-137` justifies the slack as "stable for every `c` in
  `(0, 2)`", but the accepted range includes negative values.
- **The effect never needs this slack.** The smallest coefficient it designs is
  `rate_coefficient(5000 ms, 96 kHz)`, about 2.1e-6. A walk between two such values never goes
  below zero.
- **A regression.** Before this attempt, coefficients were always designed from validated times.
  Now a payload can set any coefficient, unrelated to the attack and release parameters, and
  permanently, because a settled coefficient ramp never advances.
- **Reproduction.** The probe is `scratch/probe_negative_coefficient.rs` (copy it into
  `crates/compressor/tests/`). A settled release-coefficient ramp at `-7e-6` is accepted (`Ok(())`).
  On noise with loud and quiet halves:
  - the gain reduction runs away: -15.3, -19.8, -26.2, -35.1, -47.5, -64.9, -89.3, then -123.4 dB
    over 8 s;
  - the loud passages are crushed;
  - after 7 s the effect's own snapshot is refused (`effect.state.gain`).
  - D7 never fires, because the output stays finite.
- **Not reachable in production.** Only a crafted payload reaches it, and no production path
  restores one. I rank it MINOR, in line with how #1071's verdicts ranked hostile-only acceptance.
  It is still the one place where validation is no longer meaningful, it is a one-line fix, and it
  should go into attempt 2.

**Fix:**
- Bound coefficient ramps to `[0, 1 + slack]`, or better, to
  `[rate_coefficient(max time, rate), 1 + slack]`.
- Optionally, require a settled coefficient ramp to equal `rate_coefficient` of its settled
  parameter. This would also refuse a payload carried across sample rates, whose coefficients are
  now rate-specific.
- Add a rejection row: a settled release coefficient of `-1e-6` gives `effect.state.parameter`.
- Fix the comment.

### MINOR-2: the limiter has no path check, so its widened budget applies to targets and settled ramps too

`crates/true-peak-limiter/src/lib.rs:3960-3962` and `:4060-4080`. The orchestrator's claim 2 lists
the limiter among the effects with a "64-ulp path check". It has none: `coefficient_bounds` went from
4 to 64 ulps for `current` **and** `target`, settled or moving. The consequences:

- **The ceiling.** A crafted settled limit ramp at `1 + 7.6e-6` is now accepted, which puts the
  ceiling 6.6e-5 dB above 0 dBFS. The limiter's own settled ramps always hold exact designed values.
- **The step (pre-existing).** It is still checked only for finiteness, so a crafted step of `1e30`
  walks the coefficient anywhere for up to `RAMP_UPDATES` updates.
- **What does work.** The probe proves the widened budget is needed for the effect's own ceiling
  ramp to -24 dB: with four ulps, only the probe turns red (verified).

**Fix:** keep `target` and settled `current` strictly inside the unrelaxed bounds, and validate a
moving ramp with `ramp_path_within` and the 64-ulp slack, as the other effects do. This also bounds
the step.

### MINOR-3: D3 holds to the letter only, so the static scan does not see four effects' restore codecs

The gate (`parse_lane`/`commit_lane`, `crates/gate-expander/src/lib.rs:740-838`), the multiband
(`stage_side`/`commit_side`/`restore`, `crates/multiband-compressor/src/lib.rs:1344-1480`), the
transient shaper (`restore`/`read_lane`, `crates/transient-shaper/src/lib.rs:602-690`) and the EQ
(`restore_lane`, `crates/parametric-eq/src/lib.rs:7998`) are unmarked. Only their one-line trait
wrappers are marked.

- **Verified.** A `vec![0_u8; 4]` inserted in the transient shaper's `read_lane`, in the gate's ramp
  validation, or in the multiband's `stage_side` leaves `check-realtime-policy.sh` green. Only the
  runtime differential would catch it.
- **Contrast.** The compressor, the limiter and soft-clip mark their whole codecs.

**Fix:** extend the regions over those codec bodies so every banked effect is scanned the same way.

### MINOR-4: the coverage gate counts continuations taken after automation, not mid-ramp; the multiband barely reaches mid-ramp

`crates/conformance/src/randomized.rs:2033-2038` requires `continuations_after_automation > 0` across
the run. That does not ensure a single in-flight restore, and per effect it does not ensure quantum 32.
Measured: the multiband reaches 3 mid-ramp continuations and none at quantum 32 (12 seeds).

The multiband carries its steps, so the round-trip check catches a step re-derivation (verified: red).
Only a non-serialized word that its restore re-derives wrongly mid-ramp would slip through.

**Fix:** take continuations preferentially right after a block whose spans fall within its last 64
samples. Or make the harness count in-flight continuations through a crafting or "ramp in flight"
hook, and require one per effect.

### MINOR-5: documentation drift inside the slice's paths, plus two documents outside them

Inside the slice's paths (fix now):

- `crates/compressor/tests/ramps.rs:5-8` still describes words `1 + 3i`, `step` "deliberately
  **not** serialised", and a class-B mid-ramp restore.
- `crates/transient-shaper/tests/contract.rs:132-140` says "`step` is not persisted, it is derived",
  and names a red mutation in `read_lane` that no longer exists.
- `crates/transient-shaper/tests/MUTATIONS.md` row 4 and `:78-81` (an "eleven words" layout).
- `crates/compressor/tests/MUTATIONS.md` row 13 (`:49`) and `:83` ("the class-B mid-ramp restore").

Outside the paths (Sol follow-up):

- `docs/EFFECT_CONTRACT_V1.md:174-177`: "no engine path snapshots or restores a payload ... until a
  follow-up removes them". #1269 now makes these hooks load-bearing.
- `docs/EFFECT_CONTRACT_V1.md:186-190`: the W2-D2 bump rule (see deviation 1).
- The slice spec's D2a.

### NIT-1: realtime floors

`scripts/check-realtime-policy.sh:72-73` stays at 41 regions and 12 files. The tree now has 72 and
23. The script's own rule (`:71`) is that "raising a floor is part of the change that adds a marker".
The script is outside the slice's paths, so record a follow-up to raise the floors to 72/23.

### NIT-2: soft-clip accepts a crafted in-flight current far outside its domain (pre-existing, #1071's design)

With `step = 2^100`, `remaining = 63` and `current = target - 63 * step` (about `-8e31`, a negative
drive gain), the restore is accepted. This is verified with a scratch probe in `sccopy`. The harm is
bounded by the cubic's clamp and by D7. For consistency with the other six effects, a successor
could validate soft-clip's moving ramps with `ramp_path_within` as well.

## Test value: one sentence per new or rewritten test

| Test | The plausible defect that only it catches |
|---|---|
| conformance `Continuation` oracle (`run_width`) and `run_scalar` restored-against-continued | A restore that re-derives a word the continuation reads (the old compressor and transient shaper; both red at quantum 32, verified). It is a randomized differential, judged by what its generator reaches; reach is measured above |
| conformance audited payload calls | An allocating payload call (limiter `vec!` red, verified) |
| `assert_reached` continuation clause | A harness that stops taking continuations, or never takes one after automation |
| compressor `the_effects_own_edge_ramp_snapshots_restore` | A moving `current` held to the strict domain refuses the effect's own threshold and ratio edge snapshots (the implementer's red run) |
| gate `the_effects_own_edge_ramp_snapshots_restore` | The same, at the gate's edges: **only this test is red**, with 32 refusals (verified) |
| multiband `the_effects_own_edge_ramp_snapshots_restore` | The same, at the multiband's thresholds, ratios, attack and release (the implementer's red run) |
| limiter `the_effects_own_edge_ramp_snapshots_restore` | A 4-ulp coefficient budget refuses the effect's own ceiling ramp to -24 dB: **only this test is red** (verified) |
| transient-shaper `the_effects_own_edge_ramp_snapshots_restore` | A strict-domain moving current refuses the attack and sustain edges (the implementer's red run) |
| **soft-clip `the_effects_own_edge_ramp_snapshots_restore`** | **None; see MAJOR-1** |
| limiter `the_bank_renders_its_scalar_instances_under_random_state` (reinstated) | An allocating limiter restore. Nothing else audits the limiter's payload calls (verified red) |
| compressor `kernel::payload_restore_resumes_an_active_coefficient_ramp_exactly` | A restore that loses or rebuilds the **release** coefficient ramp (slot 1). The attack test below does not see that slot |
| compressor `payload::a_mid_ramp_restore_continues_the_ramp_exactly` | A commit that re-derives a non-rate parameter's step, compared word for word at every one of 37 samples |
| compressor `payload::an_active_attack_restore_continues_one_partition_invariant_coefficient_path` | A restore that loses or rebuilds the **attack** coefficient ramp, or makes it partition-dependent (whole against split) |
| compressor `payload::a_step_that_leaves_the_domain_before_the_snap_is_refused` | Path validation reduced to its endpoints (the implementer's red run) |
| compressor transactional `STEP` NaN row; transient-shaper version-0 and `inf`-step rows | A dropped check that a step is finite or that a settled step is `+0.0`, or `!= 1` loosened to `> 1` |
| soft-clip `a_snapshot_holding_an_overflowed_x_word_restores_and_continues_bit_for_bit` | `X` back to zero-or-normal; **only this test is red** (verified) |
| soft-clip `bad(43, inf)` row | `e` accepting infinities (the implementer's red run) |

## Other checks

Everything below is clean.

- **Realtime:**
  - no new `unsafe`;
  - every payload call is bounded by the prepared state (at most 64 path steps per ramp, rings bounded
    by the shape);
  - the audit counts no lock or syscall;
  - every restore validates both channels before it commits either.
- **Out of scope for this slice:** no ack/drop surface, no plan swap, no carry.
- **Authorized paths only.** The `effect-contract` change is doc comments only (checked line by
  line).
- **No banned tests:** no source-grepping test and no new digest pin.
- **Superseded tests are rewritten in place,** with no old copies left behind.
- **Commits** end with the `Co-Authored-By` trailer.
- **Spec record.** The attempt record is candid and agrees with what I measured, except claim 2's
  limiter wording (MINOR-2).

## Separate: `d6217a79d` (#1071 review minors)

Tests and documentation only: soft-clip `src` changes only doc comments, plus `tests/state_roundtrip.rs`
and #1071's spec.

- **`a_drive_overshoot_retargeted_inward_restores_and_continues`.** I flipped the line's sign
  (`target + remaining * step`) at `crates/soft-clip/src/lib.rs:319`. **Only this test turns red,**
  so it has a unique catch.
- **The two new rejection rows** (an at-rest output one ulp above its top, and an in-flight mix
  current of `-0.0`) close #1071 attempt 2's MINOR-2.
- **The "not closed under render" doc** is accurate. NIT-2 above is the consequence it documents.

No problems found.
