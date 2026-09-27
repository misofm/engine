# Adversarial verification: limiter briefs #988-#992

Base `codex/batch-plumbing-floor-2` at `966fa624` (limiter crate identical to `6ca203f8`; every
`file:line` cited in the five briefs was re-read and is accurate). All work was in a detached
scratch worktree; nothing was pushed and no issue was edited.

## Method

I wrote a brief-conformant implementation of all five slices in the scratch tree, behind a mode
switch. Mode 0 is the unmodified kernel, token for token. The implementation is saved as
`scratchpad/verify-lim-work/verification-implementation.patch` (evidence only):

* #988: `SCREEN_FACTOR`, `taps_below`, `history_after`, a separate `SCREENED` monomorphisation;
* #989: the release arm (`target = +0.0`, literal `fma`/`max`/`flush`), a scan-earned claim and
  every listed withdrawal;
* #990: `gain_linked`, `designed_gain_agree`, the mirrored van Herk pass, `linked_frame_uniform`,
  and the right ramps advanced in the ramping dispatch;
* #991: `load_stationary`/`store_stationary`, and skipping the automation walk;
* #992: `l / max(p, l)`, the link and bypass arms (loop-invariant branches, which LLVM unswitched:
  0 `vblendvps` in the loop), and the first-product seed, in the uniform, per-lane and mono bodies.

Mutation knobs are `cfg(test)` thread-locals, or compile-time `VERIFY_MUT` for cross-crate gates.
`VERIFY_FIXED=<mode>` builds carry only one brief's path, as a shipped build would. Those builds
were used for every timing below.

## 1. Correctness: identity against the unmodified kernel (all green)

This is a differential harness (`src/verify_tests.rs`). The oracle is a mode-0 bank; the device
under test is an identical bank at the mode being tested. After every block it compares:

* every output word (NaN as "both NaN");
* every `ProcessReport`;
* every track's snapshot payload;
* the resident observation (the reported `d`).

Coverage:

* **Widths:** W8, W4 and the scalar instance.
* **Rates:** 44.1, 48, 88.2 and 96 kHz.
* **Configuration:** `Maximum` and `DualMono`, bypass on and off, symmetric and asymmetric L/R,
  and uniform and ragged cohorts.
* **Block lengths:** {1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128}.
* **Signals:**
  * quiet noise at 0.2 to 3.0 times the lane threshold (0.45, 0.48, 0.489, 0.4899, 0.495, 0.5,
    0.52 and 1.03 included);
  * samples at exactly the threshold, and at one ulp below it;
  * the attaining Annex-2 sign pattern for each phase, at the largest `f32` below the threshold;
  * `+0.0`, `-0.0` and random subnormals;
  * 2^-24 to 1 (log-uniform);
  * one loud sample in one lane of one channel;
  * loud tails that end inside a block (the history-tap case);
  * L/R-asymmetric material;
  * NaN, +inf, -inf and 1e30 injected.
* **Events:**
  * one-channel and two-channel ceiling and release retargets, same-value retargets and invalid
    `Both` spans;
  * restores (current, cross-track and stashed payloads), both reset kinds;
  * collapse runs followed by `desymmetrize`;
  * sparse loud blocks (1 in 4, 16, 32, 64 and 512) with releases of 10, 60, 500 and 2,000 ms.

| run | scenarios | engagements | result |
|---|---:|---|---|
| sweep, modes {988, 988+989, 990, 991, 992, all}, 8 seeds | 5,184 | 477k screened, 87k linked | identical |
| unity sweep, 8 seeds | 2,304 | 1.59M screened, 245k unity (207k of them with `d > 0`, so the release arm ran), 241k linked | identical |
| 1,024-frame blocks through the corpus entry (`limiter_block`), every mode, W1/W4/W8 | 144 | 300 screened | every `ChannelState` word identical |
| D90 (`corpus::run_case`), 5 cases x 3 widths x 6 modes | 90 | | identical to the pins |
| digest scenarios (native), 16 seeds x 3 families x 3 widths x 6 modes | 864 | | identical |
| **wasm32 simd128 under Node (V8)**, the same digest scenarios, W4 and scalar, 6 modes | 576 | | identical to mode 0, and W4 digests equal the **native** base digests; every per-brief `VERIFY_FIXED` guest reproduces the native base digests (12/12 each) |
| the crate's own 33 unit tests re-run with each mode as the default | 6 x 43 | | pass |
| dev profile (debug assertions and overflow checks), all lib tests | 41 | | pass |
| `console-workload` (`chain_shape` 23 tests, digest pins), `VERIFY_FIXED` = 31, 1 and 4 | | | pass |
| `tests/{determinism,mono_collapse,gain_law,observation,allocation}.rs` at `VERIFY_FIXED=31` | | | pass (render allocation-free) |

**#988 bound.** The per-phase L1 norms of the stored `f32` table are 1.4342041, 2.0228271,
2.0228271 and 1.4342041.

* `f32(0.49) * 2.0228271 * (1 + 13u) * (1 + u)` = 0.991186 < 1. The brief's 0.9915 holds.
* The attaining pattern at `nextbelow(0.49 * limit)` reaches **0.9911854 * limit** over all 769
  ceilings. The bound is tight to 7e-7.
* With 0.51, the pattern overshoots on phases 1 and 2 (1.0316).
* The largest admissible factor is 0.49436.
* NaN and inf decline through the ordered `lt`.
* The oldest history tap is never read by the detector, so including it is conservative.

## 2. Mutations (each alone; RED means a gate caught it)

| brief | mutation | result | note |
|---|---|---|---|
| 988 | M1 0.51 | RED | |
| 988 | M2 no history taps | RED | |
| 988 | M3 max of limits | RED | only with L/R ceilings far apart and material between 0.49·min and 0.49·max; the brief's gate 3 does not state a separation |
| 988 | M4 no history update | RED | |
| 988 | M5 screen during a ramp | RED | |
| 989 | **M1 claim by counter (2R+2Wb), no scan** | **green (equivalent)** | the settling argument is sound, so the scan is redundant; a counter of `Wb` without the scan is RED |
| 989 | **M2 keep claim across automation** | **green (equivalent)** | `set_target` ramps a changed value over 64 samples, so the block is unscreened and the claim is withdrawn anyway; a same-value retarget cannot touch the rings. "Keep the claim across an *unscreened* block" (early burst, quiet last 12 frames) is RED |
| 989 | M3 skip flush | RED only with a long tail | red with a 10 ms release and 512-block cycles, green with 16 to 64-block cycles; `d` needs about 45 time constants after the claim to reach `FLUSH_EPS` |
| 989 | M4 phase not advanced | RED | |
| 989 | **M5 keep claim across `restore_track`** | green with a one-track restore, **RED** with an all-track restore | a one-track restore desyncs `phase`, so `lanes_uniform` fails, the block goes ragged and unscreened, and the claim is withdrawn regardless |
| 989 | keep across collapse **and** `desymmetrize` | RED | either withdrawal alone suffices under the contract |
| 990 | M1 no mirror (backward pass) | RED in the crate harness | **green in `chain_shape`** (the fixture barely limits) |
| 990 | M2 no mirror (box) | RED | |
| 990 | M3 engage under DualMono | RED | |
| 990 | **M4 keep `gain_linked` across a collapse** | **green in `chain_shape` and in every contract-respecting run** | `desymmetrize` re-establishes it, and the contract requires that call before any dual block (`effect-contract/src/lib.rs:1887`). RED only in a mono-then-dual sequence without `desymmetrize` |
| 990 | M5 compare `current` only | RED | |
| 991 | M1 lean load while ramping | RED | |
| 991 | **M2 scatter the lean ramps** | **green on every reachable state** | `scatter` writes `current`/`step`/`remaining`, not `target`, and `step` and `remaining` are 0 at rest. RED only after restoring an at-rest ramp with `step != 0`, which `read_lane` accepts (`lib.rs:2643`) |
| 991 | M3 skip the walk when spans exist | RED | |
| 992 | M1 `limit.max(peak)` | RED (NaN peak) | |
| 992 | M3 drop the first product | RED | |

## 3. #992 proof 1 has a reachable hole

`select(p > l, l/p, 1)` and `l / max(p, l)` differ exactly when `l` is ±0, ±inf or NaN and not
`p > l`.

* **Stationary.** `l` is the validated `target`, so the two forms agree.
* **Ramping.** `read_lane` checks only `step.is_finite()` and `remaining <= 64`
  (`lib.rs:2643-2644`). A restored in-flight ramp can therefore reach `l = +0.0`
  (`step = -current`, `remaining = 2`) or `+inf` (`step = f32::MAX`).
* **Confirmed.** Both payloads are accepted. The new form writes NaN into `required_ring`, and the
  §4.4 reset zeroes the block, where base renders normally. Base itself with `l = +inf` simply
  does not limit for 63 frames; in one probe no ceiling excess was observed.
* **Fix.** Use `l / max(p, l)` only in `DISPATCH_STATIONARY` (keep the select while ramping), or
  harden `read_lane` first as a separate issue. Add `l` in {+0.0, +inf} rows to gate 1.

## 4. Timing

Clean A/B throughout: separate binaries, base against one `VERIFY_FIXED` build per brief,
interleaved, two passes, under the lock, `taskset -c 31`, p50 of 1,200 blocks x 4 rounds. The
rig is 64 tracks with the fixture parameters (`Maximum`, 5 ms, 128 frames). Units are cycles
per lane-sample and the change against base.

### Native, x86-64-v3 (Zen 3)

| row | base | 988 | 988+989 | 990 | 991 | 992 | all |
|---|---:|---:|---:|---:|---:|---:|---:|
| W8 quiet (0.3 tone) | 10.08 | -46% | -76% | -3% | +8% | +9% | -75% |
| W8 hot (+3 dBFS, limits every block) | 10.21 | **+8%** | +10% | -4% | +6% | +9% | **-12%** |
| W8 near miss (full scan, then decline) | 10.25 | **+8%** | +13% | -4% | +7% | +8% | -10% |
| W4 quiet | 18.20 | -54% | -75% | -5% | +8% | +9% | -75% |
| W4 hot | 18.13 | **+14%** | +13% | -4% | +9% | +9% | **-9%** |
| W4 near miss | 18.27 | **+14%** | +13% (one pass: +44%, interference) | -5% | +8% | +8% | -8% |

### wasm simd128 under V8 (Node 22.23), 3.70 GHz nominal

| row | base | 988 | 988+989 | 990 | 991 | 992 | all |
|---|---:|---:|---:|---:|---:|---:|---:|
| quiet | 24.92 | -51% | -76% | -13% | -2% | -2% | -76% |
| hot | 24.84 | 0% | 0% | **-12%** | 0% | -2% | **-11%** |
| near miss | 24.78 | +2% | +2% | -12% | 0% | -2% | -10% |
| sparse 1/32 | 24.84 | -51% | -76% | -12% | (one pass +27%, interference) | -2% | -76% |

### Reading the tables

* **#988 does not pass its own gate 9 in my implementation.** At W8 the unscreened stationary
  detector loops are 138 and 144 instructions against 133 (the frame loop is still 223), and an
  `#[inline(never)]` screened body did not help (+7%/+12%).
* **The regression is codegen, not the scan.** Moving the caller's stack in 256-byte steps up to
  4 KB leaves timing flat (±3%), so stack alignment is not the cause.
* **Small slices are dominated by code-generation variance** of about ±10% natively. #991 and #992
  each measured slower natively in isolation (#992 with fewer loop instructions: detector 133 to
  129, frame 223 to 218, 0 `vblendvps`). The claimed -0.1 to -0.5 cannot be confirmed natively.
  Under V8 they are -0 to -2%.
* **#989's specified release arm** measures -75% to -76%, not the prototype's -88% to -90%
  (`d == +0` arm).
* **#990 against the claims.** Native -3% to -6% is about a third of the claimed -10% to -12%;
  V8 -12% matches the low end of the claim.
* **Worst case (every track limiting).** Helped by #990 (V8 -12%) and by all five together
  (-9% to -12% native, -11% V8). #988 and #989 help only quiet material.

## 5. Other checks

* **wasm roster** (`check-web-audioworklet-callgraph.py` rules on each guest):
  * "true-peak-limiter f32x4 dual" matches exactly one function in every build, with scalar
    arithmetic 0;
  * the limiter's f32x4 kernel count is unchanged (2);
  * `process_block` grows from 13,583 to 25,567 opcodes with all five;
  * the shadow-stack frame is about 5 KB in every build.
* **Dev-profile stack.** Base `process_block` frames are 250 KB (W8) and 147 KB (W4). Each added
  `#[inline(always)]` body costs about 120 KB at opt-level 0. My 24-body build (3 MB) overflowed
  the 2 MB test-thread stack. So:
  * #992 must not monomorphise `LINK` x `BYPASS` (x4);
  * as `LimiterCore` type parameters, that would also break the roster's "exactly one function"
    rule.
* **Policy scripts.** `check-lane-policy.sh`: ok. `check-realtime-policy.sh` flagged only my
  scratch guest's `unsafe`.
* **Discrimination of the existing gates.**
  * D90 never engages the screen or unity paths: one loud 1,024-frame block, ragged lookaheads in
    cases 0, 1 and 4.
  * `chain_shape` and the console digests catch neither #990 M1 nor M4.
  * The discriminating gates are the per-brief identity harnesses.

## 6. Exact amendments and verdicts

**Order:** #990, then #992 (amended), then #991, then #988, then #989. #991 and #992 are low value
natively. #988 and #989 go ahead only if the owner accepts level-dependent paths under the
worst-case gate below.

### #990: brief with amendments

* Delete M4's claim that "the chain_shape transition tests go red". Replace M4 with: "keep
  `gain_linked` across a collapsed block, then render a dual block without `desymmetrize`; the
  crate-local identity gate goes red. Under the contract (`desymmetrize` before any dual block) M4
  is equivalent, and clearing on collapse is defensive."
* In gate 4, note that `chain_shape` and the console digests do not discriminate M1 or M4, because
  the fixture barely limits. Gate 1 is the discriminating gate.
* Add to the invariants: under `DualMono`, `gain_linked` is cleared at every dual block (or never
  set).
* Add to the invariants: the §4.4 reset re-establishes `gain_linked` iff every lane's `LaneShape`
  agrees, the same as `reset`.
* Restate the expected saving as native -3% to -6%, V8 -12%.

### #988: brief with amendments

* Make gate 9 a hard stop, with no ±2 tolerance on the detector loop. My separate-monomorphisation
  implementation failed it (138/144 against 133).
* Add a worst-case timing gate: the hot and near-miss rows no slower than base, within +1%, at
  native W8/W4 and under V8, clean A/B of separate binaries.
* Gate 3: the asymmetric-ceiling case needs L/R ceilings at least 7 dB apart under `Maximum`, with
  material between 0.49·min and 0.49·max, or M3 stays green.
* Gate 6: note that D90 never engages the screen.
* If gate 9 cannot be met, stop and rescope. Do not relax the gate.

### #989: brief with amendments

* Replace M1 with "claim after a counter of `Wb` frames, with no scan". The counter-only mutation
  at 2R+2Wb is equivalent.
* Replace M2 with "keep the claim across an unscreened block". The scenario is an early burst with
  quiet last 12 frames. Keep the automation withdrawal as defensive only.
* Scenario (e): restore every track of the bank from snapshots taken 1-4 blocks after a limiting
  block. A one-track restore desyncs `phase`, so M5 cannot go red.
* Scenario (b) needs at least 50 release time constants of screened tail for M3 (the flush) to go
  red; that is about 170 blocks at 10 ms.
* Restate the saving as -75% (release arm), not -88%.
* Drop "a bypass change" as a withdrawal point: bypass is prepared and immutable.
* The same worst-case gate as #988 applies.

### #991: brief with amendments

* Gate 2 must restore an at-rest ramp with `step != 0` (accepted by `read_lane`). Otherwise M2 is
  equivalent: `scatter` never writes `target`, and the words it does write are 0 at rest.
* Reword M2 as "scatters the lean `step`/`remaining`".
* Add the worst-case no-regression timing gate and a codegen gate (detector 133, frame 223). My
  build regressed natively by 6% to 9% in isolation.

### #992: brief with amendments

* Proof 1 holds only where `l` is the validated stationary target. Apply `l / max(p, l)` in
  `DISPATCH_STATIONARY` only; keep the select in the ramping dispatch.
* Add gate 1 rows `l` in {+0.0, +inf} showing where each form is used, plus a restored
  in-flight-ramp scenario (`step = -current`, `remaining = 2`; `step = f32::MAX`).
* Hardening `read_lane` (step consistency) belongs in a separate issue.
* Forbid `LINK`/`BYPASS` as `LimiterCore` type parameters (the roster needs exactly one
  `process_block`) and forbid x4 fn-level monomorphisation (the dev frame grows about 120 KB per
  body). Use the unswitched loop-invariant branch, which my build confirms LLVM unswitches.
* Add the worst-case timing gate: #992 measured +8% to +9% natively in isolation and -2% under V8.

## Owner question: level-dependent fast paths

* **Correctness risk: low.** The bound is exact and tight (0.99119), and 7.5k differential
  scenarios plus wasm cross-target digests were identical.
* **They do add worst-case cost:**
  * the scan on a block that just misses: +2% under V8, about 0.4 cycles per lane-sample;
  * codegen perturbation of the unscreened body: native +8% (W8) and +14% (W4) in my build, +5%
    in the prototype.
* **So "no more than today" is not met natively** unless gate 9 and the timing gate pass.
* **Capacity does not change.** A realtime budget must still cover the all-limiting case, so the
  benefit is average CPU and power (-50% to -76% on quiet material), not track count.
* **Complexity:** two extra bodies, a scan, a claim with 6-7 withdrawal points, three new test
  files and about 10 mutations.
* **Recommendation:** accept only behind the hard worst-case gate. Otherwise defer, and ship #990
  first.
