# #1278 attempt 3 verdict: PASS

No BLOCKER, no MAJOR, no MINOR. Three NITs, none of which blocks a close.

- Attempt 2's MAJOR-1 is fixed: the conformance differential now starts real EQ ramps, and its
  in-flight count is honest.
- The new product change, `RAMP_PATH_NORM_TOLERANCE = 1 + 2^-12`, is sound.
  - It does not change one rendered bit.
  - It can be derived, not only measured. The bound the effect's own walk needs is about
    `1 + 2^-16.9` at every launch rate, and that bound survives any chain of retargets.
  - No forged path inside `1 + 2^-12` can produce unbounded or NaN output. No such path can evolve
    into a state the effect then refuses.
- All four attempt-2 findings are closed: MAJOR-1 and NIT-1, NIT-2 and NIT-3.

**Commit reviewed:** `251113c8f`.

- Fix diff: `git diff a070cfa7d 251113c8f`, 10 files, all inside the slice's authorized paths.
- Cumulative slice: `git diff 983ac85bd 251113c8f`. `hosts/`, `fixtures/`, `scripts/` and `tools/`
  are untouched.
- The commit ends with the `Co-Authored-By` trailer.

**Export:** `/tmp/claude-1002/v1278/attempt3/`.

- Mutations ran in the export. Each was reverted, and every touched file was `cmp`-checked against
  `git show 251113c8f:<path>`.
- Logs and probes are in `scratch/`:
  - `probe_ramp_path_norm.rs`, `probe_forged_edge_path.rs`;
  - `mut-M*.log`, `probe-*.log`, `gates-summary.log`.

## Gates I re-ran (all green)

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (**79 regions in 24 files**) / ok |
| `cargo test --locked --workspace` (debug) | 309 result lines: 2,235 passed, 0 failed, 35 ignored |
| `conformance_fixtures -- --check` | ok. Nothing re-pinned |
| `cargo test --locked --release -p console-workload` | 66 passed. Console digests unchanged |
| `cargo build --release -p audit -p capi && audit capi` | 100k calls: 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `check-capi-abi.sh` | ok (shared and static) |
| `cargo build --release -p bench` + `trace-effect-contract-audit.sh target/release/bench 1000000` | ok (1M blocks) |
| `check-effect-contract.sh target/release/bench` | ok (8 production factories, 0 failed gates) |
| `check-cross-targets.sh` | PASS. The #1018 iOS `memset_pattern16` rows are identical to attempt 2's (parametric-eq 132) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all ok. "browser-correctness expected.json digests and exact rows agree with the built simd128 module" |

**ARTIFACT CHANGED: confirmed.**

- The shipped module is now `477f3f1c6c53c82835617419d878deecb9967af53dc9e1ca7ef9bece26c6d8c4`.
  That is the implementer's hash; attempt 2's was `1928ba47...`.
- The cause is the EQ restore and the compressor bound, which are compiled into the module.
- The pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` still reads
  `6c952a2c...`. It is untouched, as `docs/RELEASE.md` requires.

**Rendered bits are unchanged.**

- The product diff touches only restore validation and commit. No render kernel is touched.
- The console digests, the conformance fixtures, the browser `expected.json` digests and the effect
  contract audit are all green and un-re-pinned.

**Width.**

- On this AVX2 host the EQ binds `Eight` only.
- `carry.rs` and the differential iterate `BankWidth::ALL`, and a declined width is skipped by
  design (#1112).
- The EQ's `Simd4` legs run only in CI's aarch64 jobs. I could not run them.

## Attempt-2 MAJOR-1: fixed

**1. The harness drives EQ ramps.**

- `run_width` (`crates/conformance/src/randomized.rs`, `Targets`) prepares a drawn candidate
  through `target_preparation()` off the audited scope.
- It applies every target inside `audited(...)` render scopes, to all of:
  - the scalar instance;
  - the bank lane;
  - the disengaged mono arm's lane;
  - the continued twin.
- They must agree on every verdict.
- Measured: 320 targets applied, 295 moving, 58 in-flight continuations, 26 of them at quantum 32.
  Attempt 2 measured 0 actual.
- The `if let Some(preparation) = preparation && draw.chance(1, 3)` chain short-circuits, so the
  draw streams of the six other effects are unchanged.

**2. The in-flight count is honest.**

- The new `EffectDifferential::in_flight` hook decodes `remaining`, word `19*s + 14`. That matches
  `Channel::snapshot_track`.
- A disabled band's target settles through `stationary_at`, so a nonzero `remaining` always means
  a ramp that moves.
- The raw-span proxy now credits a lane only when its report counts no invalid span. A reset clears
  every lane, and a restore that replaced the lane's state clears that lane.
- **Mutation M5:** I disabled the target drive with an environment switch. The EQ differential
  goes red in `assert_reached`: "no restored instance rendered beside its continued lane with a
  ramp in flight", with `continuations_in_flight: 0` and `moving_targets: 0`. At attempt 2, the
  EQ's clause passed even when nothing was in flight. Now it binds.

**3. Attempt 2's re-derive mutation, re-run (M1).**

- `commit_track` stores `(target - current) / remaining` as a moving band's step.
- Red:
  - `carry.rs` `a_mid_ramp_restore_continues_bit_for_bit_at_every_sample`: 342 of 390 comparisons
    diverge, on all three ramps, from sample 5 on;
  - attempt 2's `probe_eq_mid_ramp.rs`;
  - the EQ differential: "rendered 0xbeb91443 where the lane, continuing, rendered 0xbeb91440".
- Still green: the `contract.rs`, `bank.rs` and unit tests.
- Attempt 2's probe is green on `251113c8f` unmutated.

## The new product change: `RAMP_PATH_NORM_TOLERANCE = 1 + 2^-12`

The constant is at `crates/parametric-eq/src/lib.rs:812-825`. It applies only to the remaining-path
walk in `decode_track` (`:2728-2738`). Designs, prepared-target words and settled words keep
`NORM_TOLERANCE`, and a settled band must still equal its target's designed words bit for bit.

**The pre-existing defect is real.** I simulated the effect's own walk with a probe,
`scratch/probe_ramp_path_norm.rs`. It uses the walk's exact arithmetic:

- `step = (target - current) * 2^-6`;
- `current + step` once per sample;
- every one of the 64 planned points counted, because a snapshot validates the whole remaining
  path;
- random retarget chains;
- identity endpoints;
- all four launch rates, plus near-edge chains.

Results:

- **At the design limit:** 4,176 to 5,660 walk points per rate exceed it, plus about 1,000 to
  1,500 more in the near-edge chains. The EQ in `main` therefore refuses its own mid-ramp
  snapshots.
- **At the path limit:** 0 refusals over about 30.6M random points and 51.2M near-edge points.
- **Worst excess:** `3.71e-6` (`2^-18.04`) at 48 kHz, `3.63e-6` at 44.1 kHz, `2.58e-6` at 96 kHz.
  This agrees with the record's `3.7e-6`.

**The bound can be derived.**

- *Convexity and the Lipschitz bound.* `f = ‖M(w)‖₂` is convex in the words, because the matrix is
  affine in them. It is Lipschitz with `L <= 4` in the max-norm of `(c1, a2, a3)`, because
  `‖ΔM‖₂ <= ‖ΔM‖_F <= 2·2·max|Δ|`.
- *Rounding.* The step scale is exact. Each `f32` add rounds by at most `u = 2^-25`, since these
  words stay below 1. The scaled step lands within one half-ulp, about `2^-25`, of `target`.
- *Hence:* `f(w_k) - 1 <= (1 - k/64)·E + (k/64)·(τ + 4·2^-25) + 4·k·u`, where `E` is the excess
  at the walk's start and `τ = 2^-22`.
- *Induction:* whenever `E >= τ + 65·4·2^-25 ≈ 8.0e-6` (`2^-16.9`), every point of the walk stays
  at or below `1 + E`. A fresh design starts at or below `1 + τ`. So every point the effect itself
  reaches, through any chain of retargets, is at or below `1 + 8.0e-6`. That holds at every rate.

`2^-12` is 30 times that bound. The comment's other half is already derived: a forged path gains at
most `(1 + 2^-12)^64 < 1.016` before the target's exact words snap in.

**No forged path within the bound can harm.** The induction also holds with `E = 2^-12`, so a
retarget from an accepted forged start never leaves the bound. I tested this two ways.

- **Simulation** (`walk_from_forged_edge_start_stays_inside`): 42,422 starts forged to just under
  the limit, each retargeted twice. 0 walk points exceed it.
- **The real effect** (`scratch/probe_forged_edge_path.rs`): forged ramps held at norm
  `1 + 2.44e-4` for 64 samples, with integrators at ±1e3 and full-scale alternating input, on a
  1 kHz bell at Q 18 and +24 dB and on a 15 kHz notch at Q 18. In both:
  - the restore accepts the ramp;
  - the output stays finite;
  - the output decays from the forged integrators;
  - the effect's own mid-path snapshot restores;
  - a prepared retarget's mid-ramp snapshot restores too.

Every other check still runs at each path point:

- finite words, and no `-0.0`;
- `c1` in `[0, 1)`, `a2 > 0`, `a3 >= 0`;
- `|m| <= 128`.

Unbounded integrators are a separate, pre-existing case. The restore holds an integrator only to
finite, so a crafted value near `f32::MAX` overflows under any tolerance; the tolerance does not
change that. I also checked whether a `-0.0` step (which the restore refuses) can come from a
subnormal mix word. It cannot: `A² - 1` is either exactly 0 or at least about `2^-52` in `f64`, so
the mix words near zero stay normal or exactly zero.

## The rest of the fix

**EQ decode and commit (`:2672-2790`, `:3383-3413`).**

- `decode_track` validates a lane and designs each band once, carried as `RestoredBand::words`.
- `commit_track` writes the lane.
- `restore_track` decodes both channels before committing either.
- The candidate `Channel::new` builds are gone. A lane restore now runs 12 designs: 6 bands times
  2 channels.
- **Mutation M3:** commit the left channel before decoding the right. Only
  `carry.rs::a_restore_refused_on_one_channel_moves_neither` goes red ("the scalar instance moved").
  Every other EQ binary, the differential included, stays green. That closes attempt 2's NIT-3.

**Compressor (`state.rs:144-152`).**

- A target and a settled coefficient are now held to `(0, 1]`.
- **Mutation M6:** restore `[0, 1]`. Only `payload::a_coefficient_below_zero_or_above_its_design_is_refused`
  goes red ("a settled zero current"). The other 20 compressor binaries stay green.
- `rate_coefficient` designs no `0.0` for a legal time: its smallest legal coefficient is about
  2.1e-6, at 5 s and 96 kHz. That closes attempt 2's NIT-1.

**The codec region (`effect-runtime/src/state_payload.rs:185-319`).**

- **Mutation M7:** `vec![0_u8; 4]` in `ramp_path_inside`. `check-realtime-policy.sh` goes red with
  "marked realtime forbidden-body predicate".
- The script reports 79 regions in 24 files. The floors are unchanged, which is accepted as a
  follow-up because the script is outside the slice. That closes attempt 2's NIT-2.

**Realtime.**

- The prepared-target applications and every payload call run inside the differential's audited
  scopes, and the differential is green with zero allocations.
- `decode_track` returns a stack array. It contains no heap value, lock or I/O.

## Test value: one sentence per new or rewritten test

| Test | The plausible defect only it catches |
|---|---|
| `parametric-eq/tests/carry.rs` `a_mid_ramp_restore_continues_bit_for_bit_at_every_sample` | An EQ restore that refuses its own in-flight path near the contractive edge (M2: the 10 kHz swing refused at samples 0-63) or re-derives the carried step (M1). It catches both deterministically at every sample, scalar and bank lane. The differential catches M2 on only 3 of its 24 seeds (0, 6, 17) and M1 on 10 of 24, so a reshuffled draw can lose them. See NIT-2 |
| `carry.rs` `a_restore_refused_on_one_channel_moves_neither` | A restore that commits one channel before the other is validated (M3). Red only here |
| compressor `payload` rows "a settled zero current", "a zero target" | A coefficient bound that admits `0.0`, which freezes the smoother (M6). Red only here |
| conformance prepared-target drive, `in_flight` hook, `moving_targets` clause | A randomized differential, judged by reach. It now reaches accepted, moving EQ targets through scalar, bank, collapsed arm and restored twin: 295 moving targets and 58 decoded in-flight continuations, 26 at quantum 32. Disabling the drive turns the EQ's clauses red (M5) |
| proxy narrowing (refused spans credit nothing; reset and replacing-restore clear) | Harness honesty. It can only lower the proxy for the six other effects, and all of them still pass `assert_reached` |

**Mutation M4: the restore refuses `remaining == 64`.**

- Red in `carry.rs`, at sample 0 of all three ramps.
- Red in the differential, through a bypassed scenario that leaves a ramp unstarted.
- Red in the unit test `padded_banks::a_padded_lane_is_never_written_reported_or_charged`.
- So it is not unique to `carry.rs`, and I do not count it as `carry.rs`'s catch.

## Findings

### NIT-1: the path tolerance is justified by measurement on one rate; state the derivation

- **Where:** `crates/parametric-eq/src/lib.rs:812-825`.
- **What happens:** the comment rests the acceptance side on "a grid ... at 48 kHz". The bound is
  derivable, and the derivation is stronger evidence:
  - The induction above gives `E_min = τ + 65·L·u ≈ 8.0e-6` for every rate and every retarget
    chain.
  - `2^-12` clears it with margin.
  - The same induction proves that a forged start inside the bound never walks out of it.
- **Fix:** replace "measured over a grid ... at 48 kHz" with the three-line derivation. Keep the
  measured `2^-18` as a cross-check: my 4-rate simulation agrees. No code change is needed.

### NIT-2: the attempt record's test-value sentence for `carry.rs` is inaccurate

- **Where:** `.github/ISSUE_SPECS/1278-...md:523-524` says "which no other EQ test catches".
- **What happens:** the EQ's own `tests/randomized.rs` catches both M1 (seeds 2, 4, 5, 7, 8, 11,
  15, 17, 18, 22) and M2 (seeds 0, 6, 17).
- **Fix:** say what is true. `carry.rs` catches them deterministically at every sample; the
  differential catches them only on some of its seeds.

### NIT-3: an overlong merged comment line

- **Where:** `crates/compressor/src/state.rs:139`, 129 columns against `max_width = 100`.
- **What happens:** the new sentence "A moving path keeps the closed lower bound below: it lands on
  its target's exact word." was spliced into the start of the next sentence's line.
- **Fix:** reflow the paragraph.

## Open items carried forward (unchanged)

- The delay refuses its own edge-ramp snapshots.
- Soft-clip has two open non-finite history cases.
- Soft-clip validates an in-flight current by its line.
- Raise `check-realtime-policy.sh`'s floors to 79 regions and 24 files.
- The EQ's `Simd4` legs are verified only in CI aarch64.
